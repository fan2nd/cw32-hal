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

const EN: u32 = 1;
const COMP: u32 = 1 << 1;
const SINGLE_COMPARE: u32 = 1 << 3;
const PERIOD_BUFFER: u32 = 1 << 7;
const UPDATE_IE: u32 = 1 << 10;
const BREAK_IE: u32 = 1 << 20;
const UG: u32 = 1 << 25;
// Never replay a self-clearing software trigger during a CR read-modify-write.
const CR_COMMANDS: u32 = (1 << 24) | (1 << 25) | (1 << 26);
const MOE: u32 = 1 << 12;
const AOE: u32 = 1 << 11;
const BKE: u32 = 1 << 10;
const DTEN: u32 = 1 << 9;
const VCE: u32 = 1 << 14;
const UIF: u32 = 1;
const BIF: u32 = 1 << 14;
// RM 15.7.5 documented reset: R1W0 unrelated flags stay 1, including RFU bit 1.
const ICR_PRESERVE: u32 = 0x0007_ffff;
// RM 15.7.8: global ADTE and update UEVE only. No compare-trigger selections.
const ADC_UPDATE_TRIGGER: u32 = (1 << 7) | 1;
// RM 15.7.11: A/B compare buffers; both brake output states forced low (10b).
const CHANNEL_CONFIG: u32 = (1 << 6) | (1 << 7) | (2 << 2) | 2;

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
fn control_config(c: &Config) -> u32 {
    COMP | SINGLE_COMPARE
        | PERIOD_BUFFER
        | ((c.prescaler as u32) << 4)
        | ((if c.alignment == Alignment::Center {
            3
        } else {
            2
        }) << 12)
}
fn filter_config(c: &Config) -> u32 {
    0x0066_6666 | (u32::from(c.brake_filter) << 24) | (u32::from(!c.brake_active_high) << 27)
}
fn deadtime_config(c: &Config) -> Result<u32, Error> {
    Ok(BKE
        | if c.dead_time_ticks == 0 {
            0
        } else {
            DTEN | u32::from(encode_dead_time(c.dead_time_ticks)?)
        })
}
fn control_without_commands(word: u32) -> u32 {
    word & !CR_COMMANDS
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
        // SAFETY: owned singleton; clock/reset established before any access.
        unsafe {
            pac::ATIM.dtr().write(0);
            pac::ATIM.cr().write(control);
            pac::ATIM.trig().write(0);
            pac::ATIM.arr().write(config.period.into());
            pac::ATIM.cnt().write(0);
            pac::ATIM.rcr().write(0);
            pac::ATIM.mscr().write(0);
            pac::ATIM.ch1cr().write(CHANNEL_CONFIG);
            pac::ATIM.ch2cr().write(CHANNEL_CONFIG);
            pac::ATIM.ch3cr().write(CHANNEL_CONFIG);
            pac::ATIM.ch4cr().write(0);
            pac::ATIM.ch1ccra().write(0);
            pac::ATIM.ch1ccrb().write(0);
            pac::ATIM.ch2ccra().write(0);
            pac::ATIM.ch2ccrb().write(0);
            pac::ATIM.ch3ccra().write(0);
            pac::ATIM.ch3ccrb().write(0);
            pac::ATIM.fltr().write(filter_config(&config));
            // BKE on, AOE/MOE/VCE/SAFEEN off. Comparator routing is a scoped guard.
            pac::ATIM.dtr().write(deadtime);
            pac::ATIM.cr().write(control | UG);
            pac::ATIM.icr().write(ICR_PRESERVE & !UIF);
            // ADC owns its trigger receiver. This timer emits only real updates.
            pac::ATIM.trig().write(ADC_UPDATE_TRIGGER);
        }
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
        critical_section::with(|_| unsafe {
            pac::ATIM
                .cr()
                .write(control_without_commands(pac::ATIM.cr().read()) | EN);
        });
    }
    /// Stop counting. Does not itself remove a static level from an enabled output.
    pub fn stop_counter(&mut self) {
        critical_section::with(|_| unsafe {
            pac::ATIM
                .cr()
                .write(control_without_commands(pac::ATIM.cr().read()) & !EN);
        });
    }
    pub fn fault_pending(&self) -> bool {
        unsafe { pac::ATIM.isr().read() & BIF != 0 }
    }
    pub fn outputs_enabled(&self) -> bool {
        unsafe { pac::ATIM.dtr().read() & MOE != 0 }
    }
    /// Explicit power-output arm. The caller must establish board-level safety.
    /// A pending or newly detected break prevents successful arming.
    pub fn enable_outputs(&mut self) -> Result<(), Error> {
        critical_section::with(|_| arm_sequence(&mut HardwareArm))
    }
    pub fn disable_outputs(&mut self) {
        critical_section::with(|_| unsafe {
            pac::ATIM.dtr().write(pac::ATIM.dtr().read() & !(MOE | AOE));
        });
    }
    /// Clears only BIF while keeping outputs disabled. Never auto-rearms.
    pub fn acknowledge_fault(&mut self) -> Result<(), Error> {
        self.disable_outputs();
        unsafe {
            pac::ATIM.icr().write(ICR_PRESERVE & !BIF);
        }
        if self.fault_pending() {
            Err(Error::FaultActive)
        } else {
            Ok(())
        }
    }
    /// Used only by a comparator guard that owns both borrows and checks MOE=0.
    pub(crate) fn set_comparator_brake(&mut self, enabled: bool) {
        critical_section::with(|_| unsafe {
            let old = pac::ATIM.dtr().read() & !(MOE | AOE | VCE);
            pac::ATIM.dtr().write(old | if enabled { VCE } else { 0 });
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
        unsafe { pac::ATIM.dtr().read() & MOE != 0 }
    }
    fn control(&mut self) -> u32 {
        unsafe { pac::ATIM.cr().read() }
    }
    fn trigger(&mut self) -> u32 {
        unsafe { pac::ATIM.trig().read() }
    }
    fn write_control(&mut self, value: u32) {
        unsafe {
            pac::ATIM.cr().write(value);
        }
    }
    fn write_trigger(&mut self, value: u32) {
        unsafe {
            pac::ATIM.trig().write(value);
        }
    }
    fn write_compares(&mut self, duty: [u16; 3]) {
        unsafe {
            pac::ATIM.ch1ccra().write(duty[0].into());
            pac::ATIM.ch2ccra().write(duty[1].into());
            pac::ATIM.ch3ccra().write(duty[2].into());
        }
    }
    fn clear_update(&mut self) {
        unsafe {
            pac::ATIM.icr().write(ICR_PRESERVE & !UIF);
        }
    }
}
fn program_duty(io: &mut impl DutyIo, duty: [u16; 3]) -> Result<(), Error> {
    if io.outputs_enabled() {
        return Err(Error::Busy);
    }
    let control = control_without_commands(io.control());
    let trigger = io.trigger();
    let paused = control & !(EN | UPDATE_IE);
    io.write_control(paused);
    io.write_trigger(trigger & !(1 << 7));
    io.write_compares(duty);
    io.write_control(paused | UG);
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
        unsafe {
            let old = pac::ATIM.dtr().read() & !(MOE | AOE);
            pac::ATIM.dtr().write(old | if enabled { MOE } else { 0 });
        }
    }
    fn fault(&mut self) -> bool {
        unsafe { pac::ATIM.isr().read() & BIF != 0 }
    }
    fn enabled(&mut self) -> bool {
        unsafe { pac::ATIM.dtr().read() & MOE != 0 }
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
    fn ie(self) -> u32 {
        match self {
            Self::Update => UPDATE_IE,
            Self::Break => BREAK_IE,
        }
    }
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
        unsafe { control_without_commands(pac::ATIM.cr().read()) }
    }
    fn status(&mut self) -> u32 {
        unsafe { pac::ATIM.isr().read() }
    }
    fn set_enables(&mut self, value: u32) {
        unsafe {
            pac::ATIM.cr().write(control_without_commands(value));
        }
    }
    fn clear_update(&mut self) {
        unsafe {
            pac::ATIM.icr().write(ICR_PRESERVE & !UIF);
        }
    }
}
fn set_event_enabled(io: &mut impl EventIo, event: WaitEvent, enabled: bool) {
    let old = control_without_commands(io.enables());
    io.set_enables(if enabled {
        old | event.ie()
    } else {
        old & !event.ie()
    });
}
fn prepare_event(io: &mut impl EventIo, event: WaitEvent) {
    set_event_enabled(io, event, false);
    if matches!(event, WaitEvent::Update) {
        io.clear_update();
    }
    set_event_enabled(io, event, true);
}
fn service_events(io: &mut impl EventIo) -> (u32, u32) {
    let enabled = control_without_commands(io.enables());
    let flags = io.status();
    let update = if enabled & UPDATE_IE != 0 {
        flags & UIF
    } else {
        0
    };
    let fault = if enabled & BREAK_IE != 0 {
        flags & BIF
    } else {
        0
    };
    let mask = if update != 0 { UPDATE_IE } else { 0 } | if fault != 0 { BREAK_IE } else { 0 };
    if mask != 0 {
        io.set_enables(enabled & !mask);
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
        unsafe {
            pac::ATIM.trig().write(0);
        }
        for pin in &self.pins {
            pin.disconnect();
        }
    }
}
