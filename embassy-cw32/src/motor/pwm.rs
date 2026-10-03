//! L012 ATIM PWM1 and source-ordered three-high/three-GPIO-low commutation.
use crate::pac;
use core::marker::PhantomData;
#[derive(Clone, Copy, Debug)]
pub struct PwmConfig {
    pub period: u16,
    pub sample_compare: u16,
    /// False configures CH4 only, leaving all phase channels disabled.
    pub phase_outputs: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BridgeUpdate {
    Commutate,
    PwmOnly,
}
#[derive(Clone, Copy, Debug)]
pub struct PhaseDrive {
    /// Whole source compare words. Values are deliberately not clamped or
    /// divided; the caller must validate the motor algorithm/electrical range.
    pub pwm_counts: [u32; 3],
    pub low_sides: [bool; 3],
    pub sample_compare: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FaultActive;
/// Exclusive ATIM lease; pins are passed as separate caller-owned leases.
pub struct PwmBridge {
    _domain: PhantomData<*mut ()>,
}
impl PwmBridge {
    /// # Safety
    /// Caller must exclude every other ATIM user, including the owned PWM HAL,
    /// trigger reconfiguration, other handles and nested interrupts throughout
    /// this lease. Clock/pin routing and bridge electrical safety are external.
    pub unsafe fn acquire() -> Self {
        Self {
            _domain: PhantomData,
        }
    }
    /// Configure edge-aligned PWM1/preload and OC4REFC, with MOE disabled.
    /// No update pulse is inserted; this preserves original preload timing.
    /// External brake routes are disabled until explicitly configured by the
    /// board; this setup alone is not hardware overcurrent protection.
    ///
    /// # Safety
    /// Boot-only setup with the power stage disconnected and counter stopped.
    /// This clears event flags; it must not be used to clear a running fault.
    pub unsafe fn configure(&mut self, config: PwmConfig) {
        assert!(config.period > 0);
        #[cfg(sysctrl_l012)]
        critical_section::with(|_| {
            let mut gate = pac::SYSCTRL.apben1().read();
            gate.set_key(0x5a5a);
            gate.set_atim(true);
            pac::SYSCTRL.apben1().write_value(gate);
        });
        let r = pac::ATIM;
        r.cr1().write(|w| w.set_arpe(true));
        r.bdtr().write_value(pac::atim::regs::Bdtr(0));
        r.dier().write(|_| {});
        r.ccer().write(|_| {});
        r.cr2().write(|_| {});
        r.smcr().write(|_| {});
        r.psc().write(|_| {});
        r.arr().write(|w| w.set_arr(config.period - 1));
        r.rcr().write(|_| {});
        r.cnt().write(|_| {});
        r.ccmr_cmp(0).write(|w| {
            if config.phase_outputs {
                w.set_ocm(0, 6);
                w.set_ocpe(0, true);
                w.set_ocm(1, 6);
                w.set_ocpe(1, true);
            }
        });
        r.ccmr_cmp(1).write(|w| {
            if config.phase_outputs {
                w.set_ocm(0, 6);
                w.set_ocpe(0, true);
            }
            w.set_ocm(1, 6);
            w.set_ocpe(1, true);
        });
        for i in 0..3 {
            r.ccr(i).write_value(pac::atim::regs::Ccr(0));
        }
        r.ccr(3).write(|w| w.set_ccr(config.sample_compare));
        r.dtr2().write(|_| {});
        r.af1().write(|w| w.set_bkine(false));
        r.af2().write(|w| w.set_bk2ine(false));
        if config.phase_outputs {
            r.bdtr().write_value(pac::atim::regs::Bdtr(0));
        }
        r.ccer().write(|w| {
            w.set_cc1e(config.phase_outputs);
            w.set_cc2e(config.phase_outputs);
            w.set_cc3e(config.phase_outputs);
            w.set_cc4e(true);
        });
        r.icr().write_value(pac::atim::regs::Icr(0));
    }
    pub fn start(&mut self) {
        pac::ATIM.cr1().write(|w| {
            w.set_arpe(true);
            w.set_cen(true);
        });
    }
    /// A read-only view of latched hardware break sources. Never acknowledges.
    pub fn fault_pending(&mut self) -> bool {
        let flags = pac::ATIM.isr().read();
        flags.bif() || flags.b2if() || flags.sbif()
    }
    /// Explicit boot arming. AOE remains clear and hardware faults stay latched.
    /// Every refusal leaves AOE, MOE and phase output enables off, including
    /// when this lease took over an externally configured active timer.
    ///
    /// # Safety
    /// Caller has explicitly authorized physical output, checked the gate pin
    /// routing, inactive levels, supply and external protection, and completed
    /// initialization. Never call this as an automatic fault recovery action.
    pub unsafe fn arm_outputs(&mut self) -> Result<(), FaultActive> {
        if self.fault_pending() {
            self.disable_outputs();
            return Err(FaultActive);
        }
        let mut control = pac::ATIM.bdtr().read();
        control.set_aoe(false);
        control.set_moe(true);
        pac::ATIM.bdtr().write_value(control);
        if self.fault_pending() || !pac::ATIM.bdtr().read().moe() {
            self.disable_outputs();
            return Err(FaultActive);
        }
        Ok(())
    }
    /// Disable automatic rearming, MOE and power channels while retaining CH4
    /// sampling. Fault flags are untouched, even on a preconfigured AOE=1 timer.
    pub fn disable_outputs(&mut self) {
        let mut control = pac::ATIM.bdtr().read();
        control.set_aoe(false);
        control.set_moe(false);
        pac::ATIM.bdtr().write_value(control);
        pac::ATIM.ccer().write(|w| w.set_cc4e(true));
    }
    #[cfg(gpio_l012)]
    /// Apply a three-high PWM/three-GPIO-low bridge state in source order.
    /// Inactive low sides -> inactive compare words -> active compare words ->
    /// selected low sides -> sample compare. PWM-only never changes GPIOs.
    /// It does not remux pins, toggle MOE, clear faults or insert a new update.
    pub fn apply(
        &mut self,
        lows: &mut [super::MotorPin; 3],
        drive: PhaseDrive,
        update: BridgeUpdate,
    ) {
        let commutate = update == BridgeUpdate::Commutate;
        if commutate && drive.low_sides == [false; 3] {
            for i in 0..3 {
                pac::ATIM.ccr(i).write_value(pac::atim::regs::Ccr(0));
            }
            for pin in lows {
                pin.set_high(false);
            }
            pac::ATIM.ccr(3).write(|w| w.set_ccr(drive.sample_compare));
            return;
        }
        if commutate {
            for (i, pin) in lows.iter_mut().enumerate() {
                if !drive.low_sides[i] {
                    pin.set_high(false);
                }
            }
        }
        for i in 0..3 {
            if drive.pwm_counts[i] == 0 {
                pac::ATIM.ccr(i).write_value(pac::atim::regs::Ccr(0));
            }
        }
        for i in 0..3 {
            if drive.pwm_counts[i] != 0 {
                pac::ATIM
                    .ccr(i)
                    .write_value(pac::atim::regs::Ccr(drive.pwm_counts[i]));
            }
        }
        if commutate {
            for (i, pin) in lows.iter_mut().enumerate() {
                if drive.low_sides[i] {
                    pin.set_high(true);
                }
            }
        }
        pac::ATIM.ccr(3).write(|w| w.set_ccr(drive.sample_compare));
    }
    /// Clear the phase compare words only. Intended after disconnecting pins.
    pub fn zero_phase_compares(&mut self) {
        for i in 0..3 {
            pac::ATIM.ccr(i).write_value(pac::atim::regs::Ccr(0));
        }
    }
}
/// Immediately disable automatic rearming and disconnect ATIM phase outputs,
/// leaving hardware faults latched.
///
/// # Safety
/// Only for a non-returning fatal path: mask maskable interrupts, permanently
/// abandon any interrupted motor operation, then disconnect board gate GPIOs
/// and muxes. No safe/unsafe owner may resume afterward. This bypasses a live
/// lease without taking a Rust reference to it, and never reenables outputs.
pub unsafe fn emergency_disable_outputs() {
    let mut control = pac::ATIM.bdtr().read();
    control.set_aoe(false);
    control.set_moe(false);
    pac::ATIM.bdtr().write_value(control);
    pac::ATIM.ccer().write(|w| w.set_cc4e(true));
}

#[cfg(gpio_l012)]
/// Fatal-path disconnect: MOE, phase enables, gate latches, high-side muxes,
/// then phase compares. Adjacent pins on one port use one atomic latch write.
/// Does not alter any fault flag or restart timer/ADC state.
///
/// # Safety
/// Same non-returning, permanently-abandoned-domain requirements as
/// [`emergency_disable_outputs`]. All six descriptors must identify this power
/// stage's output pins. GPIO clocks/configuration must already be valid.
pub unsafe fn emergency_disconnect(lows: [super::PinId; 3], highs: [super::PinId; 3]) {
    unsafe { emergency_disable_outputs() };
    let pins = [lows[0], lows[1], lows[2], highs[0], highs[1], highs[2]];
    let mut i = 0;
    while i < pins.len() {
        let port = pins[i].port;
        let mut mask = 0u16;
        while i < pins.len() && pins[i].port == port {
            mask |= 1 << pins[i].number;
            i += 1;
        }
        // SAFETY: generated register address; caller permanently owns fatal path.
        let gpio: pac::gpio::Gpio = unsafe { pac::gpio::Gpio::from_ptr(port.base() as *mut ()) };
        gpio.brr().write(|w| {
            for n in 0..16 {
                if mask & (1 << n) != 0 {
                    w.set_brr(n, true);
                }
            }
        });
    }
    for pin in highs {
        let gpio: pac::gpio::Gpio =
            unsafe { pac::gpio::Gpio::from_ptr(pin.port.base() as *mut ()) };
        let n = pin.number as usize;
        gpio.afr(n / 8).modify(|w| w.set_afr(n % 8, 0));
    }
    for i in 0..3 {
        pac::ATIM.ccr(i).write_value(pac::atim::regs::Ccr(0));
    }
}
