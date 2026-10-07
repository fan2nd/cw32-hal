//! L012 scan operations; no owned-channel or voltage witness is fabricated.
pub use crate::adc::{ClockDivider, SampleTime};
use crate::{
    adc::Instance,
    interrupt::typelevel::{Binding, Handler},
    pac,
};
use core::marker::PhantomData;

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
pub struct AdcScan<T: Instance> {
    regs: pac::adc::Adc,
    _domain: PhantomData<(*mut (), T)>,
}
impl<T: Instance> AdcScan<T> {
    /// # Safety
    /// Exclude every other accessor/configurator for the selected ADC for this
    /// handle's lifetime, including owned ADC drivers and nested interrupts.
    /// DMA may read results only under the caller's coordinated configuration;
    /// it must never write ADC registers. Channels must be bonded/routed and
    /// analog sources stable; the clock and VDDA must meet the manual limits.
    pub unsafe fn acquire() -> Self {
        Self {
            regs: T::regs(),
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
        let mut clock = <T as crate::rcc::PeripheralClock>::acquire_no_reset();
        clock.pin();
        let r = self.regs;
        r.trigger().write(|_| {});
        r.start().write(|w| w.set_start(false));
        r.ier().write(|_| {});
        let mut control = r.cr().read();
        control.set_slave(false);
        control.set_ens(0);
        control.set_clk(pac::adc::vals::CrClk::DIV1);
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
        control.set_clk(config.divider.register_value());
        control.set_ens(config.slots.len() as u8 - 1);
        control.set_en(true);
        r.cr().write_value(control);
    }
    /// Clear all ADC event flags; intended for startup and ADC1 EOS service.
    pub fn clear_events(&mut self) {
        // RM25.12.10：RFU bit2 保持复位值 1，其余实际事件位写 0 清除。
        self.regs.icr().write_value(pac::adc::regs::Icr(0x04));
    }
    /// Replace IRQ/DMA enables with EOS IRQ only.
    ///
    /// The proof binds `H` to this ADC's metadata-derived interrupt. The handler
    /// must service this ADC synchronously and check its own source on a shared
    /// vector. This changes neither NVIC state nor priority, and clears no flags.
    pub fn enable_sequence_interrupt<H: Handler<T::Interrupt>>(
        &mut self,
        _irq: impl Binding<T::Interrupt, H>,
    ) {
        self.regs.ier().write(|w| w.set_eos(true));
    }
    /// Replace IRQ/DMA enables with per-conversion DMA only. EOS DMA is off.
    pub fn enable_conversion_dma(&mut self) {
        self.regs.ier().write(|w| w.set_dmaeoc(true));
    }
    /// Select only ATIM OC4REFC rising-edge triggering. PWM1 usually rises at
    /// reload; this does not promise a sample at CNT == CCR4.
    pub fn trigger_from_pwm(&mut self) {
        self.regs.trigger().write(|w| w.set_atim_ocref(3, true));
    }
    /// 仅选择 ATIM TRGO2；单电阻场景必须配合 ENS=0 的单槽配置。
    /// 两次转换来自两个独立硬件事件，不是一次触发连续扫描两槽。
    pub fn trigger_from_atim_trgo2(&mut self) {
        self.regs.trigger().write(|w| w.set_atimtrgo2(true));
    }
    /// 断开所有外部转换触发；不终止已启动的转换，也不清除EOS/看门狗。
    /// 分频采样必须在末次EOS后调用，并在下个采样窗口前重接触发。
    pub fn disable_external_triggers(&mut self) {
        self.regs.trigger().write(|_| {});
    }
    /// 设置离散采样模拟看门狗；这不是连续、异步的硬件功率关断。
    pub fn configure_watchdog(&mut self, channel: u8, low: u16, high: u16) {
        assert!(channel < 16 && low < high && high <= 4095);
        self.regs.awdtr().write(|w| { w.set_vtl(low); w.set_vth(high); });
        self.regs.awdcr().write(|w| w.set_in(channel as usize, true));
    }
    pub fn watchdog_pending(&self) -> bool {
        let flags = self.regs.isr().read(); flags.awdl() || flags.awdh()
    }
    /// 单槽 EOS 读取，保留 AWDL/AWDH；ISR 还须验证当前帧和采样时刻。
    /// 硬件没有 OVR，不能凭一个 EOS 断言期间只发生过一次转换。
    pub fn take_single(&mut self) -> Option<u16> {
        if !self.sequence_pending() { return None; }
        let raw = self.regs.result(0).read().result();
        self.regs.icr().write_value(pac::adc::regs::Icr(0x1c));
        Some(raw)
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
