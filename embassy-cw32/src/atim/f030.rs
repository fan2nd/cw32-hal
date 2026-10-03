//! CW32F030 ATIM: three complementary A/B pairs and real update/brake IRQs.
//!
//! Audited against CW32x030 RM 2.5, chapter 15 and SDK 2.2. F030 has CR,
//! CHxCR/CHxCCRA/B, FLTR, TRIG and DTR; it is not the L012 ATIM register map.
//! Construction leaves EN and MOE clear, AOE stays clear, and both channel
//! brake states are forced low. External gate-driver interlocks and pull
//! resistors remain necessary; this is not a board-level safety guarantee.
//!
//! The hardware DOES support running buffered compare updates: RM 2.5 p268
//! specifies BUFEy=1 preloads CHxCCRy and UEV transfers those values to their
//! active shadows. BUFEy=0 instead makes an individual write immediately affect
//! its waveform. Neither facility alone makes three separate software writes
//! atomic if a UEV falls between them. CPU critical sections do not stop UEV.
//!
//! This initial strict three-phase API rejects duty changes while outputs are
//! enabled because an uninterrupted all-three-channel commit protocol has not
//! yet been verified. It does not imply a hardware prohibition on individual
//! buffered updates. RM pp254,262,268 and 306-307 document UEV, RCR and BUFEy,
//! but do not establish a global shadow-load lock or RCR=0 live-update barrier.
//! Disabled-output changes use an explicit UG with ADC triggering gated, reset
//! PWM phase, and disturb the trigger cadence. This backend therefore does not
//! promise uninterrupted real-time closed-loop FOC modulation.
use crate::{
    async_support::EventState,
    gpio::{AnyPin, Pin},
    interrupt::{self, InterruptExt},
    pac, peripherals,
    rcc::PeripheralClock,
    Peri,
};
use pac::atim::regs;

mod sealed {
    pub trait Sealed {}
}
/// Audited bonded signal. `B=false` selects CHxA, `B=true` selects CHxB.
/// The driver explicitly sets CR.COMP to make those physical A/B outputs complementary.
pub trait OutputPin<const CHANNEL: u8, const B: bool>: sealed::Sealed + Pin {
    const AF: u8;
}
pub trait BrakePin: sealed::Sealed + Pin {
    const AF: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_atim_pins.rs"));

const UIF: u32 = 1;
const BIF: u32 = 1 << 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    PeriodTooSmall,
    DeadTimeTooLarge,
    InvalidFilter,
    DutyOutOfRange,
    FaultActive,
    Busy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Edge,
    Center,
}
/// F030 CR.PRS choices; this is not an arbitrary divisor-minus-one register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    Div1 = 0,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div256,
}
impl Prescaler {
    pub const fn divisor(self) -> u16 {
        match self {
            Self::Div1 => 1,
            Self::Div2 => 2,
            Self::Div4 => 4,
            Self::Div8 => 8,
            Self::Div16 => 16,
            Self::Div32 => 32,
            Self::Div64 => 64,
            Self::Div256 => 256,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub prescaler: Prescaler,
    /// Counter ARR, at least 2. Center alignment produces an update at each
    /// overflow/underflow because this driver uses RCR=0 without masking either.
    pub period: u16,
    pub alignment: Alignment,
    /// Minimum common dead time, in TTCLK ticks (the prescaled timer clock).
    /// Zero disables DTEN; positive values round up to an encoding (2..1010).
    /// Both edges and all three complementary pairs share this setting.
    pub dead_time_ticks: u16,
    pub brake_active_high: bool,
    /// FLTBK encoding: 0 = unfiltered; 4/5/6/7 = three consecutive samples at
    /// PCLK /1,/4,/16,/64. Other encodings are rejected rather than aliased.
    pub brake_filter: u8,
}
/// F030's four-segment DTR encoding includes an additional TWO TTCLK ticks.
/// Code zero therefore means two ticks when DTEN is enabled.
pub const fn dead_time_ticks(code: u8) -> u16 {
    match code {
        0..=127 => code as u16 + 2,
        128..=191 => (64 + (code & 63) as u16) * 2 + 2,
        192..=223 => (32 + (code & 31) as u16) * 8 + 2,
        _ => (32 + (code & 31) as u16) * 16 + 2,
    }
}
/// Smallest code whose enabled dead time is no shorter than requested.
/// To disable insertion, set Config.dead_time_ticks=0; code 0 itself is 2 ticks.
pub fn encode_dead_time(ticks: u16) -> Result<u8, Error> {
    (0..=255u16)
        .find(|&v| dead_time_ticks(v as u8) >= ticks)
        .map(|v| v as u8)
        .ok_or(Error::DeadTimeTooLarge)
}
impl Config {
    pub fn validate(&self) -> Result<(), Error> {
        if self.period < 2 {
            return Err(Error::PeriodTooSmall);
        }
        encode_dead_time(self.dead_time_ticks)?;
        if !matches!(self.brake_filter, 0 | 4..=7) {
            return Err(Error::InvalidFilter);
        }
        Ok(())
    }
}
fn control_config(c: &Config) -> regs::Cr {
    // The driver intentionally clears the reset CISA selection (0b11).
    let mut value = regs::Cr(0);
    value.set_comp(true);
    value.set_pwm2s(true);
    value.set_bufpen(true);
    value.set_prs(c.prescaler as u8);
    value.set_mode(if c.alignment == Alignment::Center {
        3
    } else {
        2
    });
    value
}
fn filter_config(c: &Config) -> regs::Fltr {
    let mut value = regs::Fltr::default();
    value.set_ocm1aflt1a(6);
    value.set_ocm1bflt1b(6);
    value.set_ocm2aflt2a(6);
    value.set_ocm2bflt2b(6);
    value.set_ocm3aflt3a(6);
    value.set_ocm3bflt3b(6);
    value.set_fltbk(c.brake_filter);
    value.set_bkp(!c.brake_active_high);
    value
}
fn deadtime_config(c: &Config) -> Result<regs::Dtr, Error> {
    let mut value = regs::Dtr::default();
    value.set_bke(true);
    if c.dead_time_ticks != 0 {
        value.set_dten(true);
        value.set_dtr(encode_dead_time(c.dead_time_ticks)?);
    }
    Ok(value)
}
fn control_without_commands(word: u32) -> u32 {
    // Never replay a self-clearing software trigger during a CR RMW.
    let mut value = regs::Cr(word);
    value.set_bg(false);
    value.set_ug(false);
    value.set_tg(false);
    value.0
}

/// Owns the timer, six real A/B pins and an external BK input.
/// Polling construction does not claim an NVIC vector.
pub struct ThreePhasePwm<'d> {
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
        let control = control_config(&config);
        let deadtime = deadtime_config(&config)?;
        <peripherals::ATIM as PeripheralClock>::enable_and_reset();
        // Owned singleton; clock/reset established before any access.
        pac::ATIM.dtr().write_value(regs::Dtr(0));
        pac::ATIM.cr().write_value(control);
        pac::ATIM.trig().write_value(regs::Trig(0));
        pac::ATIM.arr().write(|v| v.set_arr(config.period));
        pac::ATIM.cnt().write_value(regs::Cnt(0));
        pac::ATIM.rcr().write_value(regs::Rcr(0));
        pac::ATIM.mscr().write_value(regs::Mscr(0));
        // A/B buffers enabled and both brake states forced low (10b).
        // Explicit zero seeds also clear the reset CISB selection (0b11).
        for n in 0..3 {
            pac::ATIM.chcr(n).write(|v| {
                *v = regs::Chcr(0);
                v.set_bufea(true);
                v.set_bufeb(true);
                v.set_bksa(2);
                v.set_bksb(2);
            });
        }
        pac::ATIM.ch4cr().write_value(regs::Ch4cr(0));
        for n in 0..3 {
            pac::ATIM.ccra(n).write_value(regs::Ccra(0));
            pac::ATIM.ccrb(n).write_value(regs::Ccrb(0));
        }
        pac::ATIM.fltr().write_value(filter_config(&config));
        // BKE on, AOE/MOE/VCE/SAFEEN off. Comparator routing is a scoped guard.
        pac::ATIM.dtr().write_value(deadtime);
        let mut update = control;
        update.set_ug(true);
        pac::ATIM.cr().write_value(update);
        // R1W0: start at the full documented reset, preserving RFU bit 1.
        pac::ATIM.icr().write(|v| v.set_uif(false));
        // ADC owns its trigger receiver. This timer emits only real updates.
        pac::ATIM.trig().write(|v| {
            v.set_adte(true);
            v.set_ueve(true);
        });
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
            _instance: instance,
            pins,
            period: config.period,
        })
    }
    /// Set all three A comparison values. B outputs are hardware complements.
    /// Returns Busy before writing anything if MOE is enabled. With outputs
    /// disabled, pauses the counter, gates ADC triggering, and issues UG to
    /// load all buffers together. This RESETS PWM phase and clears the update
    /// indication; it is not an uninterrupted runtime modulation operation.
    pub fn set_duty(&mut self, duty: [u16; 3]) -> Result<(), Error> {
        if duty.iter().any(|&v| v > self.period) {
            return Err(Error::DutyOutOfRange);
        }
        critical_section::with(|_| program_duty(&mut HardwareDuty, duty))
    }
    /// Start counting/ADC triggers without enabling the phase outputs.
    pub fn start_counter(&mut self) {
        critical_section::with(|_| {
            let mut value = regs::Cr(control_without_commands(pac::ATIM.cr().read().0));
            value.set_en(true);
            pac::ATIM.cr().write_value(value);
        });
    }
    /// Stop counting. Does not itself remove a static level from an enabled output.
    pub fn stop_counter(&mut self) {
        critical_section::with(|_| {
            let mut value = regs::Cr(control_without_commands(pac::ATIM.cr().read().0));
            value.set_en(false);
            pac::ATIM.cr().write_value(value);
        });
    }
    pub fn fault_pending(&self) -> bool {
        pac::ATIM.isr().read().bif()
    }
    pub fn outputs_enabled(&self) -> bool {
        pac::ATIM.dtr().read().moe()
    }
    /// Explicit power-output arm. The caller must establish board-level safety.
    /// A pending or newly detected break prevents successful arming.
    pub fn enable_outputs(&mut self) -> Result<(), Error> {
        critical_section::with(|_| arm_sequence(&mut HardwareArm))
    }
    pub fn disable_outputs(&mut self) {
        critical_section::with(|_| {
            pac::ATIM.dtr().modify(|v| {
                v.set_moe(false);
                v.set_aoe(false);
            });
        });
    }
    /// Clears only BIF while keeping outputs disabled. Never auto-rearms.
    pub fn acknowledge_fault(&mut self) -> Result<(), Error> {
        self.disable_outputs();
        pac::ATIM.icr().write(|v| v.set_bif(false));
        if self.fault_pending() {
            Err(Error::FaultActive)
        } else {
            Ok(())
        }
    }
    /// Used only by a comparator guard that owns both borrows and checks MOE=0.
    pub(crate) fn set_comparator_brake(&mut self, enabled: bool) {
        critical_section::with(|_| {
            pac::ATIM.dtr().modify(|v| {
                v.set_moe(false);
                v.set_aoe(false);
                v.set_vce(enabled);
            });
        });
    }
}

trait DutyIo {
    fn outputs_enabled(&mut self) -> bool;
    fn control(&mut self) -> u32;
    fn trigger(&mut self) -> u32;
    fn write_control(&mut self, value: u32);
    fn write_trigger(&mut self, value: u32);
    fn write_compares(&mut self, duty: [u16; 3]);
    fn clear_update(&mut self);
}
struct HardwareDuty;
impl DutyIo for HardwareDuty {
    fn outputs_enabled(&mut self) -> bool {
        pac::ATIM.dtr().read().moe()
    }
    fn control(&mut self) -> u32 {
        pac::ATIM.cr().read().0
    }
    fn trigger(&mut self) -> u32 {
        pac::ATIM.trig().read().0
    }
    fn write_control(&mut self, value: u32) {
        pac::ATIM.cr().write_value(regs::Cr(value));
    }
    fn write_trigger(&mut self, value: u32) {
        pac::ATIM.trig().write_value(regs::Trig(value));
    }
    fn write_compares(&mut self, duty: [u16; 3]) {
        for (n, value) in duty.into_iter().enumerate() {
            pac::ATIM.ccra(n).write(|v| v.set_ccr(value));
        }
    }
    fn clear_update(&mut self) {
        pac::ATIM.icr().write(|v| v.set_uif(false));
    }
}
fn program_duty(io: &mut impl DutyIo, duty: [u16; 3]) -> Result<(), Error> {
    if io.outputs_enabled() {
        return Err(Error::Busy);
    }
    let control = control_without_commands(io.control());
    let trigger = io.trigger();
    let mut paused = regs::Cr(control);
    paused.set_en(false);
    paused.set_uie(false);
    io.write_control(paused.0);
    let mut gated = regs::Trig(trigger);
    gated.set_adte(false);
    io.write_trigger(gated.0);
    io.write_compares(duty);
    paused.set_ug(true);
    io.write_control(paused.0);
    io.clear_update();
    io.write_trigger(trigger);
    io.write_control(control);
    Ok(())
}

trait ArmIo {
    fn master(&mut self, enabled: bool);
    fn fault(&mut self) -> bool;
    fn enabled(&mut self) -> bool;
}
struct HardwareArm;
impl ArmIo for HardwareArm {
    fn master(&mut self, enabled: bool) {
        pac::ATIM.dtr().modify(|v| {
            v.set_moe(enabled);
            v.set_aoe(false);
        });
    }
    fn fault(&mut self) -> bool {
        pac::ATIM.isr().read().bif()
    }
    fn enabled(&mut self) -> bool {
        pac::ATIM.dtr().read().moe()
    }
}
fn arm_sequence(io: &mut impl ArmIo) -> Result<(), Error> {
    io.master(false);
    if io.fault() {
        return Err(Error::FaultActive);
    }
    io.master(true);
    // No second enable write may override a brake received during arming.
    if io.fault() || !io.enabled() {
        io.master(false);
        return Err(Error::FaultActive);
    }
    Ok(())
}

static UPDATE_STATE: EventState = EventState::new();
static BREAK_STATE: EventState = EventState::new();
/// F030 exposes one combined BIF. The originating external/VC/safety source
/// cannot be inferred from it, so no fabricated per-source flags are offered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakFlags(u32);
impl BreakFlags {
    pub fn brake(self) -> bool {
        self.0 & BIF != 0
    }
}
#[derive(Clone, Copy)]
enum WaitEvent {
    Update,
    Break,
}
impl WaitEvent {
    fn flags(self) -> u32 {
        match self {
            Self::Update => UIF,
            Self::Break => BIF,
        }
    }
    fn state(self) -> &'static EventState {
        match self {
            Self::Update => &UPDATE_STATE,
            Self::Break => &BREAK_STATE,
        }
    }
}
trait EventIo {
    fn enables(&mut self) -> u32;
    fn status(&mut self) -> u32;
    fn set_enables(&mut self, value: u32);
    fn clear_update(&mut self);
}
struct HardwareEvents;
impl EventIo for HardwareEvents {
    fn enables(&mut self) -> u32 {
        control_without_commands(pac::ATIM.cr().read().0)
    }
    fn status(&mut self) -> u32 {
        pac::ATIM.isr().read().0
    }
    fn set_enables(&mut self, value: u32) {
        pac::ATIM
            .cr()
            .write_value(regs::Cr(control_without_commands(value)));
    }
    fn clear_update(&mut self) {
        pac::ATIM.icr().write(|v| v.set_uif(false));
    }
}
fn set_event_enabled(io: &mut impl EventIo, event: WaitEvent, enabled: bool) {
    let mut value = regs::Cr(control_without_commands(io.enables()));
    match event {
        WaitEvent::Update => value.set_uie(enabled),
        WaitEvent::Break => value.set_bie(enabled),
    }
    io.set_enables(value.0);
}
fn prepare_event(io: &mut impl EventIo, event: WaitEvent) {
    set_event_enabled(io, event, false);
    if matches!(event, WaitEvent::Update) {
        io.clear_update();
    }
    set_event_enabled(io, event, true);
}
fn service_events(io: &mut impl EventIo) -> (u32, u32) {
    let mut enabled = regs::Cr(control_without_commands(io.enables()));
    let flags = io.status();
    let update = if enabled.uie() { flags & UIF } else { 0 };
    let fault = if enabled.bie() { flags & BIF } else { 0 };
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
    // Do not clear BIF: acknowledge_fault() is an explicit separate operation.
    (update, fault)
}
/// Bind to the actual ATIM IRQ with bind_interrupts!. No SysTick or software timer.
pub struct InterruptHandler;
impl interrupt::typelevel::Handler<interrupt::typelevel::ATIM> for InterruptHandler {
    unsafe fn on_interrupt() {
        let (update, fault) = critical_section::with(|_| {
            let (u, b) = service_events(&mut HardwareEvents);
            (
                if u != 0 { UPDATE_STATE.latch(u) } else { None },
                if b != 0 { BREAK_STATE.latch(b) } else { None },
            )
        });
        if let Some(waker) = update {
            waker.wake();
        }
        if let Some(waker) = fault {
            waker.wake();
        }
    }
}
pub struct AsyncThreePhasePwm<'d> {
    inner: ThreePhasePwm<'d>,
}
impl<'d> ThreePhasePwm<'d> {
    /// Bind IRQ ownership without changing counter state, outputs or BIF.
    pub fn into_async(
        self,
        _irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ATIM, InterruptHandler>,
    ) -> AsyncThreePhasePwm<'d> {
        critical_section::with(|_| {
            UPDATE_STATE.reset();
            BREAK_STATE.reset();
            set_event_enabled(&mut HardwareEvents, WaitEvent::Update, false);
            set_event_enabled(&mut HardwareEvents, WaitEvent::Break, false);
            unsafe {
                interrupt::ATIM.enable();
            }
        });
        AsyncThreePhasePwm { inner: self }
    }
}
impl<'d> core::ops::Deref for AsyncThreePhasePwm<'d> {
    type Target = ThreePhasePwm<'d>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AsyncThreePhasePwm<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
struct WaitGuard(WaitEvent);
impl WaitGuard {
    fn new(event: WaitEvent) -> Self {
        critical_section::with(|_| {
            event.state().reset();
            prepare_event(&mut HardwareEvents, event);
        });
        Self(event)
    }
}
impl Drop for WaitGuard {
    fn drop(&mut self) {
        critical_section::with(|_| {
            set_event_enabled(&mut HardwareEvents, self.0, false);
            self.0.state().reset();
        });
    }
}
impl AsyncThreePhasePwm<'_> {
    async fn wait_event(&mut self, event: WaitEvent) -> u32 {
        let _guard = WaitGuard::new(event);
        core::future::poll_fn(|cx| {
            event.state().register(cx.waker());
            let latched = event.state().take();
            let current = critical_section::with(|_| HardwareEvents.status()) & event.flags();
            let result = latched | current;
            if result != 0 {
                core::task::Poll::Ready(result)
            } else {
                core::task::Poll::Pending
            }
        })
        .await
    }
    /// Wait for an update after the first poll. Does not start the timer.
    /// Events coalesce. Cancellation masks only UIE and never changes power outputs.
    pub async fn wait_update(&mut self) {
        self.wait_event(WaitEvent::Update).await;
    }
    /// Historical BIF returns immediately and remains latched. Cancellation
    /// masks only BIE; external/VC brake protection is never disabled by a wait.
    pub async fn wait_break(&mut self) -> BreakFlags {
        BreakFlags(self.wait_event(WaitEvent::Break).await)
    }
}
impl Drop for AsyncThreePhasePwm<'_> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            set_event_enabled(&mut HardwareEvents, WaitEvent::Update, false);
            set_event_enabled(&mut HardwareEvents, WaitEvent::Break, false);
            UPDATE_STATE.reset();
            BREAK_STATE.reset();
        });
    }
}
impl Drop for ThreePhasePwm<'_> {
    fn drop(&mut self) {
        self.disable_outputs();
        self.stop_counter();
        pac::ATIM.trig().write_value(regs::Trig(0));
        for pin in &self.pins {
            pin.disconnect();
        }
    }
}
