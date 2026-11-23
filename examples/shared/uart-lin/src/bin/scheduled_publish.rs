//! Test-network master/publisher. Qualify header gaps and physical bus timing.
//! Both scheduled IDs must belong exclusively to this node. No RX/echo check.
#![no_std]
#![no_main]
use core::{future::{Future, poll_fn}, pin::pin, task::Poll};
use embassy_cw32::{
    self as hal,
    usart::{self, LinBreakBits, LinChecksum, LinFrameDefinition, LinIdentifier,
        LinResponse, LinScheduleEntry, LinSchedulePlan, LinScheduleTiming, LinTransmitter},
};
use embassy_time::{Duration, Instant, Timer, with_timeout};

hal::bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});

// Check the application clock before every operation poll. This cannot preempt
// a running poll or retract a byte already queued in the UART.
async fn within_slot<F: Future>(end: Instant, operation: F) -> Result<F::Output, ()> {
    let mut operation = pin!(operation);
    let mut alarm = pin!(Timer::at(end));
    poll_fn(|cx| {
        if Instant::now() >= end || alarm.as_mut().poll(cx).is_ready() {
            return Poll::Ready(Err(()));
        }
        operation.as_mut().poll(cx).map(Ok)
    }).await
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    let p = hal::init(system).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let tx_pin = p.PA1;
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let tx_pin = p.PA2;
    let mut config = usart::Config::default();
    config.baudrate = 19_200;
    let mut tx = LinTransmitter::new(p.UART1, tx_pin, Irqs, config).unwrap();
    let responses = [
        LinResponse::new(LinIdentifier::new(0x12).unwrap(), &[0x10, 0x20], LinChecksum::Enhanced).unwrap(),
        LinResponse::new(LinIdentifier::new(0x13).unwrap(), &[1, 2, 3, 4, 5, 6, 7, 8], LinChecksum::Enhanced).unwrap(),
    ];
    let entries = [
        LinScheduleEntry {
            frame: LinFrameDefinition::new(responses[0].identifier(), 2, LinChecksum::Enhanced).unwrap(),
            time_base_ticks: 2,
        },
        LinScheduleEntry {
            frame: LinFrameDefinition::new(responses[1].identifier(), 8, LinChecksum::Enhanced).unwrap(),
            time_base_ticks: 3,
        },
    ];
    // 10 ms and 15 ms slots on a 5 ms base; 1 ms declared software start budget.
    // This declaration does not prove the actual falling-edge jitter on a board.
    let timing = LinScheduleTiming::new(19_200, 5_000, 1_000).unwrap();
    let plan = LinSchedulePlan::new(timing, &entries).unwrap();
    let Some(start_us) = Instant::now().as_micros().checked_add(10_000) else { return; };
    let mut cursor = plan.cursor(start_us);
    loop {
        Timer::at(Instant::from_micros(cursor.next_slot_us())).await;
        let slot = match cursor.start_slot(Instant::now().as_micros()) {
            Ok(Some(slot)) => slot,
            Ok(None) => continue,
            Err(error) => { core::hint::black_box(error); break; }
        };
        let response = &responses[slot.index];
        let transfer = within_slot(Instant::from_micros(slot.end_us), async {
            tx.send_break(LinBreakBits::default()).await?;
            // Hardware idle is a full frame, not a one-bit delimiter. Include
            // its duration and every software gap in board header qualification.
            tx.send_idle().await?;
            tx.write_sync_identifier(response.identifier()).await?;
            tx.write_response(response).await?;
            tx.flush().await
        }).await;
        if !matches!(transfer, Ok(Ok(()))) {
            core::hint::black_box(transfer);
            cursor.abort();
            break;
        }
        if let Err(error) = cursor.finish_slot(Instant::now().as_micros()) {
            core::hint::black_box(error);
            break;
        }
    }
    // Stop scheduling after any failure; never issue catch-up frames or retry
    // a possibly transmitted prefix. UART idle is not external bus retirement.
    let retirement = with_timeout(Duration::from_millis(10), tx.flush()).await;
    let _ = core::hint::black_box(retirement);
    core::future::pending::<()>().await;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
