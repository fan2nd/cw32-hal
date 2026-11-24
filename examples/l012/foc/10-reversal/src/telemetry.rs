//! Latest outer-loop diagnostic, formatted only by the foreground.
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
pub(crate) enum Report {
    Overrun {
        entry_us: u32,
        exit_us: u32,
        current_updates: u32,
    },
    Leg {
        reached: i32,
        elapsed_us: u32,
        zero_us: u32,
        avg_iq: i32,
        bus_mv: u32,
    },
    Hold {
        target: i32,
        held_us: u32,
        mean_rpm: i32,
    },
    BusFault {
        bus_mv: u32,
    },
    TransferFault {
        error: embassy_cw32::spi::Error,
        elapsed_us: u32,
    },
    IntervalFault {
        elapsed_us: u32,
    },
    JumpFault {
        previous: u16,
        sample: u16,
        delta: i32,
        limit: i32,
        elapsed_us: u32,
    },
    MotionFault {
        target_rpm: i32,
        feedback_rpm: i32,
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
        Report::Overrun {
            entry_us,
            exit_us,
            current_updates,
        } => defmt::error!(
            "2 kHz overrun: entry_phase_us={=u32} exit_phase_us={=u32} adc_updates={=u32}; bridge off, reset to restart",
            entry_us,
            exit_us,
            current_updates
        ),
        Report::Leg {
            reached,
            elapsed_us,
            zero_us,
            avg_iq,
            bus_mv,
        } => defmt::info!(
            "reached={}rpm leg_us={} to_zero_us={} bus_peak_mV={} avg_iq={}",
            reached,
            elapsed_us,
            zero_us,
            bus_mv,
            avg_iq
        ),
        Report::Hold {
            target,
            held_us,
            mean_rpm,
        } => defmt::info!(
            "hold target={} mean_rpm={} held_us={}; reversing",
            target,
            mean_rpm,
            held_us
        ),
        Report::BusFault { bus_mv } => defmt::error!("state=fault: bus voltage={=u32}mV", bus_mv),
        Report::TransferFault { error, elapsed_us } => defmt::error!(
            "state=fault: SSI transfer {:?} dt_us={=u32}",
            error,
            elapsed_us
        ),
        Report::IntervalFault { elapsed_us } => {
            defmt::error!("state=fault: SSI sample interval dt_us={=u32}", elapsed_us)
        }
        Report::JumpFault {
            previous,
            sample,
            delta,
            limit,
            elapsed_us,
        } => defmt::error!(
            "state=fault: SSI jump prev={=u16} now={=u16} delta={=i32} limit={=i32} dt_us={=u32}",
            previous,
            sample,
            delta,
            limit,
            elapsed_us
        ),
        Report::MotionFault {
            target_rpm,
            feedback_rpm,
            iq,
        } => defmt::error!(
            "speed fault: target={=i32} fb={=i32} previous_iqref={=i16}; reset to restart",
            target_rpm,
            feedback_rpm,
            iq
        ),
    }
}
