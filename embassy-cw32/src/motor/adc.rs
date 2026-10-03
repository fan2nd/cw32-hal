//! L012 scan operations; no owned-channel or voltage witness is fabricated.
pub use crate::adc::{ClockDivider, SampleTime};
use crate::pac;
use core::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub enum AdcUnit {
    Adc1,
    Adc2,
}
#[derive(Clone, Copy, Debug)]
pub struct ScanSlot {
    pub channel: u8,
    pub sample: SampleTime,
}
impl ScanSlot {
    pub const fn new(channel: u8, sample: SampleTime) -> Self {
        assert!(channel < 16);
        Self { channel, sample }
    }
}
/// Fixed scan configuration; unused slots are explicitly zeroed.
#[derive(Clone, Copy, Debug)]
pub struct ScanConfig<'a> {
    pub slots: &'a [ScanSlot],
    pub divider: ClockDivider,
}

/// Exclusive lease for ADC registers, trigger routes, IRQ enables and results.
/// Deliberately neither Copy nor Send/Sync; contains no owner token.
pub struct AdcScan {
    regs: pac::adc::Adc,
    _domain: PhantomData<*mut ()>,
}
impl AdcScan {
    /// # Safety
    /// Exclude every other accessor/configurator for the selected ADC for this
    /// handle's lifetime, including owned ADC drivers and nested interrupts.
    /// DMA may read results only under the caller's coordinated configuration;
    /// it must never write ADC registers. Channels must be bonded/routed and
    /// analog sources stable; the clock and VDDA must meet the manual limits.
    pub unsafe fn acquire(unit: AdcUnit) -> Self {
        Self {
            regs: match unit {
                AdcUnit::Adc1 => pac::ADC1,
                AdcUnit::Adc2 => pac::ADC2,
            },
            _domain: PhantomData,
        }
    }
    /// Stop triggering and configure a one-to-eight-slot, non-continuous scan.
    /// Preserves *the same* reserved CR bits through disable and enable writes,
    /// including reserved bit 8. No startup delay or software start is implicit.
    ///
    /// # Safety
    /// Call only during quiescent setup with no active conversion/DMA request.
    /// The selected channel/sample/clock combinations must be electrically valid
    /// at the actual VDDA (48 MHz is not valid for every supply voltage).
    pub unsafe fn configure(&mut self, config: ScanConfig<'_>) {
        assert!(!config.slots.is_empty() && config.slots.len() <= 8);
        assert!(config.slots.iter().all(|s| s.channel < 16));
        #[cfg(sysctrl_l012)]
        critical_section::with(|_| {
            let mut gate = pac::SYSCTRL.apben1().read();
            gate.set_key(0x5a5a);
            gate.set_adc(true);
            pac::SYSCTRL.apben1().write_value(gate);
        });
        let r = self.regs;
        r.trigger().write(|_| {});
        r.start().write(|w| w.set_start(false));
        r.ier().write(|_| {});
        let mut control = r.cr().read();
        control.set_slave(false);
        control.set_ens(0);
        control.set_clk(0);
        control.set_cont(false);
        control.set_en(false);
        r.cr().write_value(control);
        r.awdcr().write(|_| {});
        r.sqrcfr().write(|w| {
            for i in 0..8 {
                w.set_sqrch(i, config.slots.get(i).map_or(0, |s| s.channel));
            }
        });
        r.sample().write(|w| {
            for i in 0..8 {
                w.set_sqrch(i, config.slots.get(i).map_or(0, |s| s.sample as u8));
            }
        });
        self.clear_events();
        control.set_clk(config.divider as u8);
        control.set_ens(config.slots.len() as u8 - 1);
        control.set_en(true);
        r.cr().write_value(control);
    }
    /// Clear all ADC event flags; intended for startup and ADC1 EOS service.
    pub fn clear_events(&mut self) {
        self.regs.icr().write_value(pac::adc::regs::Icr(0));
    }
    /// Replace IRQ/DMA enables with EOS IRQ only.
    pub fn enable_sequence_interrupt(&mut self) {
        self.regs.ier().write(|w| w.set_eos(true));
    }
    /// Replace IRQ/DMA enables with per-conversion DMA only. EOS DMA is off.
    pub fn enable_conversion_dma(&mut self) {
        self.regs.ier().write(|w| w.set_dmaeoc(true));
    }
    /// Select only ATIM OC4REFC rising-edge triggering. PWM1 usually rises at
    /// reload; this does not promise a sample at CNT == CCR4.
    pub fn trigger_from_pwm(&mut self) {
        self.regs.trigger().write(|w| w.set_atimoc4refc(true));
    }
    /// Start once without clearing flags or changing the hardware trigger route.
    /// A busy conversion may coalesce/ignore the request, as on the peripheral.
    pub fn start_software(&mut self) {
        self.regs.start().write(|w| w.set_start(true));
    }
    pub fn sequence_pending(&mut self) -> bool {
        self.regs.isr().read().eos()
    }
    /// Acknowledge EOS only; preserve other event flags with W0C ones.
    pub fn acknowledge_sequence(&mut self) {
        self.regs.icr().write_value(pac::adc::regs::Icr(0x1d));
    }
    /// Check EOS, clear all flags first, then read N result slots in order.
    /// Hardware can begin another scan; this is not a frozen DMA snapshot.
    pub fn take_sequence<const N: usize>(&mut self) -> Option<[u16; N]> {
        assert!(N > 0 && N <= 8);
        if !self.sequence_pending() {
            return None;
        }
        self.clear_events();
        Some(core::array::from_fn(|i| {
            self.regs.result(i).read().result()
        }))
    }
    /// Start address for a caller-audited 32-bit incrementing DMA scan.
    /// The pointer confers no lifetime or buffer/transfer ownership.
    pub fn result_address(&mut self) -> *const u32 {
        self.regs.result(0).as_ptr().cast()
    }
}
/// Factory bandgap calibration millivolts, unmodified even when invalid.
/// Board protection code decides how to handle invalid calibration inputs.
pub fn factory_reference_mv() -> u16 {
    // SAFETY: read-only documented L012 factory calibration halfword.
    unsafe { core::ptr::read_volatile(0x0010_07d2 as *const u16) }
}
