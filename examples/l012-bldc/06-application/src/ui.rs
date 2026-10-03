//! Bounded messages between the motor interrupt domain and the UI task.
//! No peripheral handle, register pointer or controller reference crosses here.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiFeedback {
    pub led_on: Option<bool>,
    pub telemetry: Option<[u8; 7]>,
}

impl UiFeedback {
    /// Latest complete message wins while a consumer is delayed. A frame is
    /// copied as seven bytes, never interleaved with an older partial frame.
    pub fn merge(&mut self, newer: Self) {
        if newer.led_on.is_some() {
            self.led_on = newer.led_on;
        }
        if newer.telemetry.is_some() {
            self.telemetry = newer.telemetry;
        }
    }
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiCommand {
    key_pressed: bool,
    age_ms: u16,
}

impl UiCommand {
    pub const fn default_const() -> Self {
        Self {
            key_pressed: false,
            age_ms: 0,
        }
    }
    pub fn update(&mut self, key_pressed: bool) {
        self.key_pressed = key_pressed;
        self.age_ms = 0;
    }
    pub fn tick_1ms(&mut self) {
        self.age_ms = self.age_ms.saturating_add(1);
    }
    // Debounce requires a stream of fresh observations. One pressed sample
    // followed by a stalled task must never masquerade as a 60 ms key hold.
    pub const fn key_pressed(&self) -> bool {
        self.key_pressed && self.age_ms < 2
    }
}

use crate::frame_queue::FrameQueue;
use core::cell::RefCell;
use critical_section::Mutex;
use embassy_cw32::pac;

pub async fn run(
    led: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PC13>,
    key: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA3>,
    uart: embassy_cw32::Peri<'static, embassy_cw32::peripherals::UART1>,
    tx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB12>,
    rx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB11>,
) {
    // UI owns these pins and UART exclusively; configuration finishes before
    // yielding to the thread executor. The boot-only critical section prevents
    // a motor-init GPIO register RMW from interleaving with UI pin setup.
    critical_section::with(|_| unsafe {
        crate::configure_pin(pac::GPIOC_BASE, 13, crate::PinMode::OutputHigh, 0);
        crate::configure_pin(pac::GPIOA_BASE, 3, crate::PinMode::InputPullUp, 0);
        crate::configure_pin(pac::GPIOB_BASE, 12, crate::PinMode::OutputHigh, 1);
        crate::configure_pin(pac::GPIOB_BASE, 11, crate::PinMode::InputPullUp, 1);
        pac::UART1.ier().write_value(0);
        pac::UART1.cr1().write_value(0x1003);
        pac::UART1.cr2().write_value(0);
        pac::UART1.cr3().write_value(0);
        pac::UART1.brri().write_value(52);
        pac::UART1.brrf().write_value(1);
    });
    let mut tx = FrameQueue::new();
    loop {
        let feedback = next_ui_tick().await;
        let pressed = unsafe { pac::read(pac::GPIOA_BASE + pac::gpio::IDR) & (1 << 3) == 0 };
        critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().command.update(pressed));
        if let Some(on) = feedback.led_on {
            // PC13 LED is active-low; SET/CLR does not race motor GPIO mux RMW.
            unsafe {
                pac::write(
                    pac::GPIOC_BASE + if on { pac::gpio::BRR } else { pac::gpio::BSRR },
                    1 << 13,
                )
            };
        }
        if let Some(frame) = feedback.telemetry {
            tx.push(frame);
        }
        // Nonblocking, at most one UART byte per task wake.
        if unsafe { pac::uart::fields::isr::TXE.read(pac::UART1.isr().read()) } {
            if let Some(byte) = tx.pop_byte() {
                unsafe { pac::UART1.tdr().write_value(u32::from(byte)) };
            }
        }
        core::hint::black_box((&led, &key, &uart, &tx_pin, &rx_pin));
    }
}

struct UiLink {
    command: UiCommand,
    feedback: UiFeedback,
    pending_tick: bool,
    waker: Option<core::task::Waker>,
}
static UI_LINK: Mutex<RefCell<UiLink>> = Mutex::new(RefCell::new(UiLink {
    command: UiCommand::default_const(),
    feedback: UiFeedback {
        led_on: None,
        telemetry: None,
    },
    pending_tick: false,
    waker: None,
}));

pub fn publish_ui_tick() {
    let waker = critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.pending_tick = true;
        link.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

async fn next_ui_tick() -> UiFeedback {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut link = UI_LINK.borrow(cs).borrow_mut();
            if link.pending_tick {
                link.pending_tick = false;
                core::task::Poll::Ready(link.feedback.take())
            } else {
                if !link
                    .waker
                    .as_ref()
                    .is_some_and(|old| old.will_wake(cx.waker()))
                {
                    link.waker = Some(cx.waker().clone());
                }
                core::task::Poll::Pending
            }
        })
    })
    .await
}

pub fn key_sample() -> bool {
    critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.command.tick_1ms();
        link.command.key_pressed()
    })
}

pub fn publish_status(feedback: UiFeedback) {
    critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().feedback.merge(feedback));
}
