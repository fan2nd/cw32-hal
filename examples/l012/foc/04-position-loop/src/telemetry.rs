//! Latest outer-loop diagnostic, formatted only by the foreground.
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
pub(crate) enum Report {
    Overrun,
    Fault,
    Running {
        target: i32,
        measured: i32,
        error: i32,
        reference: i32,
        settled: bool,
        rpm: i32,
        iqref: i16,
        id: i16,
        iq: i16,
    },
}
static LAST: Mutex<Cell<Option<Report>>> = Mutex::new(Cell::new(None));
pub(crate) fn publish(report: Report) {
    interrupt::free(|cs| LAST.borrow(cs).set(Some(report)));
}
pub fn report() {
    let Some(event) = interrupt::free(|cs| LAST.borrow(cs).take()) else {
        return;
    };
    match event {
        Report::Overrun => defmt::error!("1 kHz control overrun: bridge off, return pot to zero"),
        Report::Fault => defmt::error!("control fault: return pot to zero"),
        Report::Running {
            target,
            measured,
            error,
            reference,
            settled,
            rpm,
            iqref,
            id,
            iq,
        } => defmt::info!(
            "position target={=i32} measured={=i32} error={=i32} speed_ref_x8={=i32} settled={=bool} rpm={=i32} iq_ref={=i16} id={=i16} iq={=i16}",
            target,
            measured,
            error,
            reference,
            settled,
            rpm,
            iqref,
            id,
            iq
        ),
    }
}
