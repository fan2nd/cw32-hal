//! Three-phase complementary PWM on the CW32 ATIM, RM 17.
//! Construction leaves MOE and CEN clear. No automatic restart after a break.
//! This is peripheral control, not a board-level motor safety certification.
use crate::{
    gpio::{AnyPin, Pin},
    pac, peripherals,
    rcc::PeripheralClock,
    Peri,
};
use pac::atim::regs;

use super::events::{EventIo, WaitEvent};

mod sealed {
    pub trait Sealed {}
}
/// Audited package-bonded ATIM signal. Implementations are generated from metadata.
pub trait OutputPin<const CHANNEL: u8, const COMPLEMENTARY: bool>: sealed::Sealed + Pin {
    const AF: u8;
}
pub trait BrakePin: sealed::Sealed + Pin {
    const AF: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_atim_pins.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    PeriodTooSmall,
    DeadTimeTooLarge,
    InvalidFilter,
    DutyOutOfRange,
    FaultActive,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Edge,
    Center,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Counter divider is prescaler + 1. CKD stays /1; dead time uses PCLK ticks, not divided counter ticks.
    pub prescaler: u16,
    /// ARR. Edge period = (ARR+1)*(PSC+1), center period = 2*ARR*(PSC+1).
    pub period: u16,
    pub alignment: Alignment,
    /// Requested minimum dead time in PCLK ticks, rounded up to a hardware encoding (0..1008).
    pub rising_dead_time: u16,
    pub falling_dead_time: u16,
    pub brake_active_high: bool,
    /// Hardware BKF filter encoding 0..15, RM 17.10.18. Zero is asynchronous.
    /// Nonzero requires a working filter clock; this HAL does not configure
    /// the separate fault-safe clock protection needed for clock-loss safety.
    pub brake_filter: u8,
}
/// Decode the non-linear hardware dead-time encoding (RM table 17-12).
pub const fn dead_time_ticks(code: u8) -> u16 {
    match code {
        0..=127 => code as u16,
        128..=191 => (64 + (code & 63) as u16) * 2,
        192..=223 => (32 + (code & 31) as u16) * 8,
        _ => (32 + (code & 31) as u16) * 16,
    }
}
/// Rounds upward, never shortening the requested dead time.
pub fn encode_dead_time(ticks: u16) -> Result<u8, Error> {
    (0..=255u16)
        .find(|&c| dead_time_ticks(c as u8) >= ticks)
        .map(|c| c as u8)
        .ok_or(Error::DeadTimeTooLarge)
}
impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if self.period < 2 {
            return Err(Error::PeriodTooSmall);
        }
        encode_dead_time(self.rising_dead_time)?;
        encode_dead_time(self.falling_dead_time)?;
        if self.brake_filter > 15 {
            return Err(Error::InvalidFilter);
        }
        Ok(())
    }
}
fn disabled_control_words(
    config: &Config,
    rise: u8,
    fall: u8,
) -> (regs::Cr1, regs::Bdtr, regs::Dtr2) {
    let mut cr1 = regs::Cr1::default();
    cr1.set_cms(if config.alignment == Alignment::Center {
        pac::atim::vals::Cr1Cms::CENTER_BOTH
    } else {
        pac::atim::vals::Cr1Cms::EDGE_ALIGNED
    });
    cr1.set_arpe(true);
    let mut dt = regs::Dtr2::default();
    dt.set_dtgf(fall);
    dt.set_dtae(rise != fall);
    let mut bdtr = regs::Bdtr::default();
    bdtr.set_dtg(rise);
    bdtr.set_bkf(config.brake_filter);
    bdtr.set_bkp(config.brake_active_high);
    bdtr.set_bke(true);
    bdtr.set_ossr(true);
    bdtr.set_ossi(true);
    (cr1, bdtr, dt)
}
/// Owns ATIM, all six phase pins and one external break pin until dropped.
/// Does not claim or enable the NVIC ATIM interrupt; polling does not steal its vector.
pub struct ThreePhasePwm<'d> {
    _clock: crate::rcc::ClockGuard,
    _instance: Peri<'d, peripherals::ATIM>,
    pins: [Peri<'d, AnyPin>; 7],
    period: u16,
}
impl<'d> ThreePhasePwm<'d> {
    #[allow(clippy::too_many_arguments)]
    pub fn new<
        A: OutputPin<1, false>,
        B: OutputPin<1, true>,
        C: OutputPin<2, false>,
        D: OutputPin<2, true>,
        E: OutputPin<3, false>,
        F: OutputPin<3, true>,
        K: BrakePin,
    >(
        instance: Peri<'d, peripherals::ATIM>,
        a: Peri<'d, A>,
        b: Peri<'d, B>,
        c: Peri<'d, C>,
        d: Peri<'d, D>,
        e: Peri<'d, E>,
        f: Peri<'d, F>,
        brake: Peri<'d, K>,
        config: Config,
    ) -> Result<Self, Error> {
        config.validate()?;
        let rise = encode_dead_time(config.rising_dead_time)?;
        let fall = encode_dead_time(config.falling_dead_time)?;
        let (cr1, bdtr, dt) = disabled_control_words(&config, rise, fall);
        let clock = <peripherals::ATIM as PeripheralClock>::acquire();
        // Singleton ownership and enabled clock; reset removes stale lock/mode state.
        pac::ATIM.bdtr().write_value(regs::Bdtr(0));
        pac::ATIM.cr1().write_value(regs::Cr1(0));
        pac::ATIM.dier().write_value(regs::Dier(0));
        pac::ATIM.ccer().write_value(regs::Ccer(0));
        pac::ATIM.cr1().write_value(cr1);
        pac::ATIM.psc().write(|v| v.set_psc(config.prescaler));
        pac::ATIM.arr().write(|v| v.set_arr(config.period));
        pac::ATIM.rcr().write_value(regs::Rcr(0));
        pac::ATIM.cnt().write_value(regs::Cnt(0));
        pac::ATIM.ccmr_cmp(0).write(|v| {
            v.set_ocm(0, pac::atim::vals::CcmrCmpOcm::PWM1);
            v.set_ocm(1, pac::atim::vals::CcmrCmpOcm::PWM1);
            v.set_ocpe(0, true);
            v.set_ocpe(1, true);
        });
        pac::ATIM.ccmr_cmp(1).write(|v| {
            v.set_ocm(0, pac::atim::vals::CcmrCmpOcm::PWM1);
            v.set_ocpe(0, true);
        });
        for n in 0..3 {
            pac::ATIM.ccr(n).write_value(regs::Ccr(0));
        }
        pac::ATIM.dtr2().write_value(dt);
        // LOCK=0; AOE=0 deliberately. OIS defaults low. CCER=0 leaves
        // pins high-Z until explicitly armed (RM table 17-13); external
        // gate-driver disable/pull resistors must establish a safe level.
        pac::ATIM.bdtr().write_value(bdtr);
        pac::ATIM.af1().write(|v| v.set_bkine(true));
        // Deliberately disable BK2INE, which is set in the AF2 reset word.
        pac::ATIM.af2().write_value(regs::Af2(0));
        // Both MMS=reset and MMS=update propagate UG. Gate TRGO/TRGO2 to
        // stopped CNT_EN while loading preloads, before connecting ADC triggers.
        pac::ATIM.cr2().write(|v| {
            v.set_mms(1);
            v.set_mms2(1);
        });
        pac::ATIM.egr().write(|v| v.set_ug(true));
        // R1W0: preserve every unrelated flag with the documented reset word.
        pac::ATIM.icr().write(|v| v.set_uif(false));
        // Only real updates emit TRGO. No further software UG is used by this driver.
        pac::ATIM.cr2().write(|v| v.set_mms(2));
        let pins = [
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            brake.into(),
        ];
        for (pin, af) in pins[..6]
            .iter()
            .zip([A::AF, B::AF, C::AF, D::AF, E::AF, F::AF])
        {
            pin.configure_alternate(af, true);
        }
        pins[6].configure_alternate(K::AF, false);
        Ok(Self {
            _clock: clock,
            _instance: instance,
            pins,
            period: config.period,
        })
    }
    /// All three values latch on the same next update event. CCR cannot exceed ARR.
    /// UDIS temporarily suppresses update events, including TRGO: a coinciding ADC
    /// trigger may be skipped. This API does not guarantee a jitter-free sampling schedule.
    pub fn set_duty(&mut self, duty: [u16; 3]) -> Result<(), Error> {
        if duty.iter().any(|&d| d > self.period) {
            return Err(Error::DutyOutOfRange);
        }
        critical_section::with(|_| {
            let old = pac::ATIM.cr1().read();
            let mut paused = old;
            paused.set_udis(true);
            pac::ATIM.cr1().write_value(paused);
            for (n, value) in duty.into_iter().enumerate() {
                pac::ATIM.ccr(n).write(|v| v.set_ccr(value));
            }
            pac::ATIM.cr1().write_value(old);
        });
        Ok(())
    }
    /// Start the counter/ADC trigger while retaining disabled power outputs.
    pub fn start_counter(&mut self) {
        pac::ATIM.cr1().modify(|v| v.set_cen(true));
    }
    pub fn fault_pending(&self) -> bool {
        pac::ATIM.isr().read().0 & fault_mask() != 0
    }
    /// Explicitly arm phase outputs. The caller must establish board-specific power-stage safety.
    /// A pending break blocks arming. Hardware can break asynchronously after this return.
    pub fn enable_outputs(&mut self) -> Result<(), Error> {
        arm_sequence(&mut HardwareArm)
    }
    pub fn outputs_enabled(&self) -> bool {
        pac::ATIM.bdtr().read().moe()
    }
    pub fn disable_outputs(&mut self) {
        // BDTR also contains write-once LOCK; keep its explicit read/write sequence.
        let mut value = pac::ATIM.bdtr().read();
        value.set_moe(false);
        pac::ATIM.bdtr().write_value(value);
    }
    /// Clear latched break flags with R1W0 semantics. Leaves outputs disabled; never auto-rearms.
    pub fn acknowledge_fault(&mut self) -> Result<(), Error> {
        self.disable_outputs();
        pac::ATIM.icr().write(|v| {
            v.set_bif(false);
            v.set_b2if(false);
            v.set_sbif(false);
        });
        if self.fault_pending() {
            Err(Error::FaultActive)
        } else {
            Ok(())
        }
    }
}
const UIF: u32 = 1;

/// A latched hardware break notification. Reading this value does not
/// acknowledge the fault or restore power outputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakFlags(pub(super) u32);
impl BreakFlags {
    pub fn external_break(self) -> bool {
        regs::Isr(self.0).bif()
    }
    pub fn second_break(self) -> bool {
        regs::Isr(self.0).b2if()
    }
    pub fn system_break(self) -> bool {
        regs::Isr(self.0).sbif()
    }
}
impl WaitEvent {
    pub(super) fn flags(self) -> u32 {
        match self {
            Self::Update => UIF,
            Self::Break => fault_mask(),
        }
    }
}
pub(super) struct HardwareEvents;
impl EventIo for HardwareEvents {
    fn enables(&mut self) -> u32 {
        pac::ATIM.dier().read().0
    }
    fn status(&mut self) -> u32 {
        pac::ATIM.isr().read().0
    }
    fn set_enables(&mut self, value: u32) {
        pac::ATIM.dier().write_value(regs::Dier(value));
    }
    fn clear_update(&mut self) {
        pac::ATIM.icr().write(|v| v.set_uif(false));
    }
}
// Callers serialize all DIER RMW with a critical section. These helpers cannot
// access BDTR/MOE, CEN, CCER, EGR, or any break-acknowledgment operation.
pub(super) fn set_event_enabled(io: &mut impl EventIo, event: WaitEvent, enabled: bool) {
    let mut value = regs::Dier(io.enables());
    match event {
        WaitEvent::Update => value.set_uie(enabled),
        WaitEvent::Break => value.set_bie(enabled),
    }
    io.set_enables(value.0);
}
pub(super) fn service_events(io: &mut impl EventIo) -> (u32, u32) {
    let mut enabled = regs::Dier(io.enables());
    let flags = io.status();
    let update = if enabled.uie() { flags & UIF } else { 0 };
    let fault = if enabled.bie() {
        flags & fault_mask()
    } else {
        0
    };
    if update != 0 || fault != 0 {
        if update != 0 {
            enabled.set_uie(false);
        }
        if fault != 0 {
            enabled.set_bie(false);
        }
        io.set_enables(enabled.0);
    }
    if update != 0 {
        io.clear_update();
    }
    // Break flags deliberately remain latched until acknowledge_fault().
    (update, fault)
}

trait ArmIo {
    fn master(&mut self, on: bool);
    fn channels(&mut self, on: bool);
    fn fault(&mut self) -> bool;
    fn enabled(&mut self) -> bool;
}
struct HardwareArm;
impl ArmIo for HardwareArm {
    fn master(&mut self, on: bool) {
        let mut value = pac::ATIM.bdtr().read();
        value.set_moe(on);
        pac::ATIM.bdtr().write_value(value);
    }
    fn fault(&mut self) -> bool {
        pac::ATIM.isr().read().0 & fault_mask() != 0
    }
    fn enabled(&mut self) -> bool {
        pac::ATIM.bdtr().read().moe()
    }
    fn channels(&mut self, on: bool) {
        pac::ATIM.ccer().write(|v| {
            v.set_cce(0, on);
            v.set_ccne(0, on);
            v.set_cce(1, on);
            v.set_ccne(1, on);
            v.set_cce(2, on);
            v.set_ccne(2, on);
        });
    }
}
fn arm_sequence(io: &mut impl ArmIo) -> Result<(), Error> {
    io.master(false);
    io.channels(false);
    if io.fault() {
        return Err(Error::FaultActive);
    }
    // Set MOE only while channels are disconnected, then check the fault latch.
    // Once channels connect there is no later MOE=1 write to override a break.
    io.master(true);
    if io.fault() || !io.enabled() {
        io.master(false);
        return Err(Error::FaultActive);
    }
    io.channels(true);
    if io.fault() || !io.enabled() {
        io.master(false);
        return Err(Error::FaultActive);
    }
    Ok(())
}
fn fault_mask() -> u32 {
    let mut value = regs::Icr(0);
    value.set_bif(true);
    value.set_b2if(true);
    value.set_sbif(true);
    value.0
}
impl Drop for ThreePhasePwm<'_> {
    fn drop(&mut self) {
        self.disable_outputs();
        pac::ATIM.cr1().modify(|v| v.set_cen(false));
        pac::ATIM.ccer().write_value(regs::Ccer(0));
        for pin in &self.pins {
            pin.disconnect();
        }
    }
}
