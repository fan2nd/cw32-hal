use crate::{
    board_resources::ServoUart,
    global::{CURRENT_FAULT, FIRST_STOP, SYSTICK_OVERRUNS, StopReason, record_stop},
    protocol::*,
    timing::Timing,
};
use core::{cell::Cell, sync::atomic::Ordering};
use cortex_m::interrupt::{self, Mutex};
use embassy_cw32::{
    bind_interrupts, dma,
    interrupt::{
        Priority,
        typelevel::{DMACH12, Interrupt, UART1},
    },
    mode::Async,
    peripherals,
    usart::{self, Uart},
};
use embassy_futures::select::{Either, select};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<peripherals::UART1>;
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH1>, dma::InterruptHandler<peripherals::DMA_CH2>;
});
pub fn open(p: ServoUart) -> Uart<'static, Async> {
    let mut config = usart::Config::default();
    config.baudrate = 1_000_000;
    config.dma_rx_checked_frames = true;
    UART1::set_priority(Priority::P2);
    DMACH12::set_priority(Priority::P2);
    let tx =
        cortex_m::singleton!(: [u8;crate::wave::PWM_FRAME]=[0;crate::wave::PWM_FRAME]).unwrap();
    let rx = cortex_m::singleton!(: [u8;64]=[0;64]).unwrap();
    Uart::new_with_dma(p.uart, p.tx, p.rx, p.tx_dma, p.rx_dma, Irqs, tx, rx, config).unwrap_or_else(
        |error| {
            defmt::error!("UART1 DMA init: {:?}", error);
            crate::halt()
        },
    )
}
static STATE: Mutex<Cell<State>> = Mutex::new(Cell::new(State {
    command: Command::DEFAULT,
    enabled: false,
    heartbeat: None,
    timed_out: false,
}));
#[derive(Clone, Copy, Default)]
pub struct Feedback {
    pub angle: i32,
    pub rpm: i16,
    pub iq: i16,
    pub moe: bool,
}
static FEEDBACK: Mutex<Cell<Feedback>> = Mutex::new(Cell::new(Feedback {
    angle: 0,
    rpm: 0,
    iq: 0,
    moe: false,
}));
pub fn publish(f: Feedback) {
    interrupt::free(|cs| FEEDBACK.borrow(cs).set(f));
}
pub fn demand(now: u32) -> Option<Command> {
    interrupt::free(|cs| {
        let cell = STATE.borrow(cs);
        let mut s = cell.get();
        let active = s.enabled;
        s.expire(now);
        if active && !s.enabled {
            record_stop(StopReason::Heartbeat);
        }
        if s.heartbeat.is_none() && crate::wave::enabled() {
            crate::wave::subscribe(0);
        }
        cell.set(s);
        s.enabled.then_some(s.command)
    })
}
fn reply(frame: &[u8], out: &mut [u8; 48]) {
    let candidate = Command::decode(&frame[4..27]);
    let (state, f, status, fault) = interrupt::free(|cs| {
        let cell = STATE.borrow(cs);
        let mut s = cell.get();
        let now = Timing::micros();
        let active = s.enabled;
        s.expire(now);
        if active && !s.enabled {
            record_stop(StopReason::Heartbeat);
        }
        let active = s.enabled;
        let ok = match frame[2] {
            5 | 7 if s.heartbeat.is_some() => {
                crate::wave::subscribe(frame[2]);
                true
            }
            8 | 9 => true,
            10 if s.heartbeat.is_some() => {
                let word = |n: usize| u16::from_le_bytes([frame[n], frame[n + 1]]);
                let speed_kp = word(12);
                let speed_ki = word(14);
                let position_kp = word(16);
                if speed_kp <= 512 && speed_ki <= 128 && (1..=64).contains(&position_kp) {
                    crate::global::CURRENT_CONTROLLER
                        .borrow(cs)
                        .borrow_mut()
                        .set_current_gains([word(4), word(6), word(8), word(10)]);
                    s.command.speed_kp = speed_kp as i16;
                    s.command.speed_ki = speed_ki as i16;
                    s.command.pos_gain = position_kp as i16;
                    true
                } else {
                    false
                }
            }
            6 => {
                crate::wave::subscribe(0);
                true
            }
            op => s.accept(
                op,
                candidate,
                now,
                CURRENT_FAULT.load(Ordering::Acquire) != 0,
            ),
        };
        if ok && frame[2] == APPLY && !active {
            FIRST_STOP.store(0, Ordering::Relaxed);
        }
        if active && matches!(frame[2], RELEASE | CONNECT) {
            record_stop(if frame[2] == RELEASE {
                StopReason::Release
            } else {
                StopReason::Connect
            });
        }
        if frame[2] == CONNECT {
            crate::wave::subscribe(0);
        }
        if matches!(frame[2], CONNECT | RELEASE) {
            crate::servo::off();
        }
        if frame[2] == RELEASE {
            CURRENT_FAULT.store(0, Ordering::Release);
        }
        let status = u8::from(!ok);
        cell.set(s);
        (
            s,
            FEEDBACK.borrow(cs).get(),
            status,
            CURRENT_FAULT.load(Ordering::Acquire),
        )
    });
    out.fill(0);
    out[..2].copy_from_slice(b"SR");
    out[2] = status;
    out[3] = frame[3];
    state.command.encode(&mut out[4..27]);
    out[27] = u8::from(state.enabled)
        | (u8::from(f.moe && state.enabled && fault == 0) << 1)
        | (u8::from(fault != 0) << 2)
        | (u8::from(state.timed_out) << 3)
        | (u8::from(crate::wave::enabled()) << 4);
    out[28..32].copy_from_slice(&f.angle.to_le_bytes());
    out[32..34].copy_from_slice(&f.rpm.to_le_bytes());
    out[34..36].copy_from_slice(&f.iq.to_le_bytes());
    out[36..40].copy_from_slice(&SYSTICK_OVERRUNS.load(Ordering::Relaxed).to_le_bytes());
    out[40] = FIRST_STOP.load(Ordering::Relaxed) as u8;
    if matches!(frame[2], 9 | 10) {
        let gains = interrupt::free(|cs| {
            crate::global::CURRENT_CONTROLLER
                .borrow(cs)
                .borrow()
                .current_gains()
        });
        for (dst, gain) in out[4..12].chunks_exact_mut(2).zip(gains) {
            dst.copy_from_slice(&(gain as u16).to_le_bytes());
        }
        out[12..14].copy_from_slice(&state.command.speed_kp.to_le_bytes());
        out[14..16].copy_from_slice(&state.command.speed_ki.to_le_bytes());
        out[16..18].copy_from_slice(&state.command.pos_gain.to_le_bytes());
    }
    if frame[2] == 8 {
        for (dst, counter) in out[4..16].chunks_exact_mut(4).zip([
            &crate::wave::GENERATED,
            &crate::wave::OVERWRITTEN,
            &crate::wave::SENT,
        ]) {
            dst.copy_from_slice(&counter.load(Ordering::Relaxed).to_le_bytes());
        }
    }
    let check = crc(&out[..46]);
    out[46..].copy_from_slice(&check.to_le_bytes());
}
static RESPONSE: Signal<CriticalSectionRawMutex, [u8; 48]> = Signal::new();

pub async fn serve(uart: Uart<'static, Async>) {
    let (mut tx, mut rx) = uart.split();
    let send = async {
        let mut buffer = [0; crate::wave::PWM_FRAME];
        loop {
            let result = match select(RESPONSE.wait(), crate::wave::READY.receive()).await {
                Either::First(reply) => tx.write(&reply).await,
                Either::Second(batch) => {
                    let result = tx.write(batch.encode(&mut buffer)).await;
                    if result.is_ok() {
                        let sent = &crate::wave::SENT;
                        sent.store(
                            sent.load(Ordering::Relaxed).wrapping_add(1),
                            Ordering::Relaxed,
                        );
                    }
                    result
                }
            };
            if let Err(error) = result {
                defmt::error!("UART1 TX: {:?}", error);
                return;
            }
        }
    };
    let receive = async {
        let mut chunk = [0; 64];
        let mut window = [0; FRAME - 1 + 64];
        let mut response = [0; 48];
        loop {
            if let Err(error) = rx.read(&mut chunk).await {
                defmt::warn!("UART1 RX: {:?}", error);
                window.fill(0);
                if matches!(
                    error,
                    usart::Error::Overrun
                        | usart::Error::Noise
                        | usart::Error::Framing
                        | usart::Error::Parity
                ) {
                    continue;
                }
                // A quarantined DMA lease requires peripheral reinitialization.
                // Stop here; the independent SysTick watchdog releases the bridge.
                break;
            }
            window[FRAME - 1..].copy_from_slice(&chunk);
            for frame in window.windows(FRAME) {
                if frame.starts_with(b"SC")
                    && crc(&frame[..30]) == u16::from_le_bytes([frame[30], frame[31]])
                {
                    reply(frame, &mut response);
                    RESPONSE.signal(response);
                }
            }
            window.copy_within(64.., 0);
        }
    };
    let _ = select(receive, send).await;
}
