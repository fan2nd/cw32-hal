//! Latest outer-loop diagnostic, formatted only by the foreground.
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
pub(crate) enum Report {
    Overrun {
        entry_us: u32,
        exit_us: u32,
        current_updates: u32,
    },
    Stopped,
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
    Deadband {
        pot: u16,
    },
    MotionFault {
        target_rpm: i32,
        feedback_rpm: i32,
        iq: i16,
    },
    Started {
        pot: u16,
        target_rpm: i32,
    },
    Running {
        pot: u16,
        target_rpm: i32,
        feedback_rpm: i32,
        iqref: i16,
        id: i16,
        iq: i16,
        mechanical_angle: u16,
        electrical_angle: u16,
        applied_angle: u16,
        adc_a: u16,
        adc_b: u16,
        offset_a: u16,
        offset_b: u16,
        bus_mv: u32,
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
            "2 kHz overrun: entry_phase_us={=u32} exit_phase_us={=u32} adc_updates={=u32}; bridge off, return pot to zero",
            entry_us,
            exit_us,
            current_updates
        ),
        Report::Stopped => defmt::info!("run: potentiometer zero, output and faults cleared"),
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
        Report::Deadband { pot } => {
            defmt::info!("run: command deadband, bridge off pot={=u16}", pot)
        }
        Report::MotionFault {
            target_rpm,
            feedback_rpm,
            iq,
        } => defmt::error!(
            "speed fault: target={=i32} fb={=i32} previous_iqref={=i16}; return pot to zero",
            target_rpm,
            feedback_rpm,
            iq
        ),
        Report::Started { pot, target_rpm } => defmt::info!(
            "run: command active, bridge start requested pot={=u16} target={=i32}",
            pot,
            target_rpm
        ),
        Report::Running {
            pot,
            target_rpm,
            feedback_rpm,
            iqref,
            id,
            iq,
            mechanical_angle,
            electrical_angle,
            applied_angle,
            adc_a,
            adc_b,
            offset_a,
            offset_b,
            bus_mv,
        } => defmt::info!(
            "run: pot={=u16} target={=i32} fb={=i32} iqref={=i16} idq={=i16}/{=i16} mech14={=u16} elec16={=u16} adc_angle16={=u16} adc={=u16}/{=u16} off={=u16}/{=u16} bus={=u32}",
            pot,
            target_rpm,
            feedback_rpm,
            iqref,
            id,
            iq,
            mechanical_angle,
            electrical_angle,
            applied_angle,
            adc_a,
            adc_b,
            offset_a,
            offset_b,
            bus_mv
        ),
    }
}
