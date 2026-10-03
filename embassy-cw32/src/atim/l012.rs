//! Three-phase complementary PWM on the CW32 ATIM, RM 17.
//! Construction leaves MOE and CEN clear. No automatic restart after a break.
//! This is peripheral control, not a board-level motor safety certification.
use crate::{
    async_support::EventState,
    gpio::{AnyPin, Pin},
    interrupt::{self, InterruptExt},
    pac, peripherals,
    rcc::PeripheralClock,
    Peri,
};
use pac::atim::fields as f;

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
fn disabled_control_words(config: &Config, rise: u8, fall: u8) -> (u32, u32, u32) {
    let cr1 = f::cr1::ARPE.write(
        f::cr1::CMS.write(
            0,
            if config.alignment == Alignment::Center {
                3
            } else {
                0
            },
        ),
        true,
    );
    let dt = f::dtr2::DTAE.write(f::dtr2::DTGF.write(0, fall.into()), rise != fall);
    let bdtr = f::bdtr::DTG.write(0, rise.into());
    let bdtr = f::bdtr::BKE.write(
        f::bdtr::BKP.write(
            f::bdtr::BKF.write(bdtr, config.brake_filter.into()),
            config.brake_active_high,
        ),
        true,
    );
    (
        cr1,
        f::bdtr::OSSI.write(f::bdtr::OSSR.write(bdtr, true), true),
        dt,
    )
}
/// Owns ATIM, all six phase pins and one external break pin until dropped.
/// Does not claim or enable the NVIC ATIM interrupt; polling does not steal its vector.
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
        let rise = encode_dead_time(config.rising_dead_time)?;
        let fall = encode_dead_time(config.falling_dead_time)?;
        let (cr1, bdtr, dt) = disabled_control_words(&config, rise, fall);
        <peripherals::ATIM as PeripheralClock>::enable_and_reset();
        // SAFETY: singleton ownership and enabled clock; reset removes stale lock/mode state.
        unsafe {
            pac::ATIM.bdtr().write_value(0);
            pac::ATIM.cr1().write_value(0);
            pac::ATIM.dier().write_value(0);
            pac::ATIM.ccer().write_value(0);
            pac::ATIM.cr1().write_value(cr1);
            pac::ATIM.psc().write_value(config.prescaler.into());
            pac::ATIM.arr().write_value(config.period.into());
            pac::ATIM.rcr().write_value(0);
            pac::ATIM.cnt().write_value(0);
            let mode1 = f::ccmr1cmp::OC1M.write(f::ccmr1cmp::OC2M.write(0, 6), 6);
            pac::ATIM
                .ccmr1cmp()
                .write_value(f::ccmr1cmp::OC1PE.write(f::ccmr1cmp::OC2PE.write(mode1, true), true));
            pac::ATIM
                .ccmr2cmp()
                .write_value(f::ccmr2cmp::OC3PE.write(f::ccmr2cmp::OC3M.write(0, 6), true));
            pac::ATIM.ccr1().write_value(0);
            pac::ATIM.ccr2().write_value(0);
            pac::ATIM.ccr3().write_value(0);
            pac::ATIM.dtr2().write_value(dt);
            // LOCK=0; AOE=0 deliberately. OIS defaults low. CCER=0 leaves
            // pins high-Z until explicitly armed (RM table 17-13); external
            // gate-driver disable/pull resistors must establish a safe level.
            pac::ATIM.bdtr().write_value(bdtr);
            pac::ATIM.af1().write_value(f::af1::BKINE.write(0, true));
            pac::ATIM.af2().write_value(0);
            // TRGO=update, usable by ADC even while phase outputs remain disabled.
            pac::ATIM.cr2().write_value(f::cr2::MMS.write(0, 2));
            pac::ATIM.egr().write_value(f::egr::UG.write(0, true));
            pac::ATIM
                .icr()
                .write_value(ATIM_ICR_MASK & !f::icr::UIF.mask());
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
    /// All three values latch on the same next update event. CCR cannot exceed ARR.
    /// UDIS temporarily suppresses update events, including TRGO: a coinciding ADC
    /// trigger may be skipped. This API does not guarantee a jitter-free sampling schedule.
    pub fn set_duty(&mut self, duty: [u16; 3]) -> Result<(), Error> {
        if duty.iter().any(|&d| d > self.period) {
            return Err(Error::DutyOutOfRange);
        }
        critical_section::with(|_| unsafe {
            let old = pac::ATIM.cr1().read();
            pac::ATIM.cr1().write_value(f::cr1::UDIS.write(old, true));
            pac::ATIM.ccr1().write_value(duty[0].into());
            pac::ATIM.ccr2().write_value(duty[1].into());
            pac::ATIM.ccr3().write_value(duty[2].into());
            pac::ATIM.cr1().write_value(old);
        });
        Ok(())
    }
    /// Start the counter/ADC trigger while retaining disabled power outputs.
    pub fn start_counter(&mut self) {
        unsafe {
            pac::ATIM
                .cr1()
                .write_value(f::cr1::CEN.write(pac::ATIM.cr1().read(), true));
        }
    }
    pub fn fault_pending(&self) -> bool {
        unsafe { pac::ATIM.isr().read() & fault_mask() != 0 }
    }
    /// Explicitly arm phase outputs. The caller must establish board-specific power-stage safety.
    /// A pending break blocks arming. Hardware can break asynchronously after this return.
    pub fn enable_outputs(&mut self) -> Result<(), Error> {
        arm_sequence(&mut HardwareArm)
    }
    pub fn outputs_enabled(&self) -> bool {
        unsafe { f::bdtr::MOE.read(pac::ATIM.bdtr().read()) }
    }
    pub fn disable_outputs(&mut self) {
        unsafe {
            pac::ATIM
                .bdtr()
                .write_value(f::bdtr::MOE.write(pac::ATIM.bdtr().read(), false));
        }
    }
    /// Clear latched break flags with R1W0 semantics. Leaves outputs disabled; never auto-rearms.
    pub fn acknowledge_fault(&mut self) -> Result<(), Error> {
        self.disable_outputs();
        unsafe {
            pac::ATIM.icr().write_value(ATIM_ICR_MASK & !fault_mask());
        }
        if self.fault_pending() {
            Err(Error::FaultActive)
        } else {
            Ok(())
        }
    }
}
// RM 17.10.6: documented reset value; R1W0 flags preserve unrelated
// latches at one and reserved bits at zero. Never read-modify-write ICR.
const ATIM_ICR_MASK: u32 = 0x00ff_3fff;
const UPDATE_IE: u32 = 1;
const BREAK_IE: u32 = 1 << 7;
static UPDATE_STATE: EventState = EventState::new();
static BREAK_STATE: EventState = EventState::new();

/// A latched hardware break notification. Reading this value does not
/// acknowledge the fault or restore power outputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakFlags(u32);
impl BreakFlags {
    pub fn external_break(self) -> bool {
        self.0 & f::isr::BIF.mask() != 0
    }
    pub fn second_break(self) -> bool {
        self.0 & f::isr::B2IF.mask() != 0
    }
    pub fn system_break(self) -> bool {
        self.0 & f::isr::SBIF.mask() != 0
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
            Self::Update => f::isr::UIF.mask(),
            Self::Break => fault_mask(),
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
        unsafe { pac::ATIM.dier().read() }
    }
    fn status(&mut self) -> u32 {
        unsafe { pac::ATIM.isr().read() }
    }
    fn set_enables(&mut self, value: u32) {
        unsafe { pac::ATIM.dier().write_value(value) }
    }
    fn clear_update(&mut self) {
        unsafe {
            pac::ATIM
                .icr()
                .write_value(ATIM_ICR_MASK & !f::icr::UIF.mask())
        }
    }
}
// Callers serialize all DIER RMW with a critical section. These helpers cannot
// access BDTR/MOE, CEN, CCER, EGR, or any break-acknowledgment operation.
fn set_event_enabled(io: &mut impl EventIo, event: WaitEvent, enabled: bool) {
    let old = io.enables();
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
    let enabled = io.enables();
    let flags = io.status();
    let update = if enabled & UPDATE_IE != 0 {
        flags & f::isr::UIF.mask()
    } else {
        0
    };
    let fault = if enabled & BREAK_IE != 0 {
        flags & fault_mask()
    } else {
        0
    };
    let disable = if update != 0 { UPDATE_IE } else { 0 } | if fault != 0 { BREAK_IE } else { 0 };
    if disable != 0 {
        io.set_enables(enabled & !disable);
    }
    if update != 0 {
        io.clear_update();
    }
    // Break flags deliberately remain latched until acknowledge_fault().
    (update, fault)
}

/// Bind this handler to the real ATIM interrupt with `bind_interrupts!`.
/// Only enabled update/break subscriptions are serviced. Other ATIM sources
/// remain available to additional handlers in the same binding.
pub struct InterruptHandler;
impl interrupt::typelevel::Handler<interrupt::typelevel::ATIM> for InterruptHandler {
    unsafe fn on_interrupt() {
        let (update, fault) = critical_section::with(|_| {
            let (update, fault) = service_events(&mut HardwareEvents);
            let update = if update != 0 {
                UPDATE_STATE.latch(update)
            } else {
                None
            };
            let fault = if fault != 0 {
                BREAK_STATE.latch(fault)
            } else {
                None
            };
            (update, fault)
        });
        // Hardware service and latch publication are one cancellation-atomic
        // transaction; only owning Wakers leave the CS, never unlatchable bits.
        if let Some(waker) = update {
            waker.wake();
        }
        if let Some(waker) = fault {
            waker.wake();
        }
    }
}
/// IRQ-driven event waits plus the original explicit PWM controls.
/// The wrapper owns the complete ATIM, not just the two subscriptions.
pub struct AsyncThreePhasePwm<'d> {
    inner: ThreePhasePwm<'d>,
}
impl<'d> ThreePhasePwm<'d> {
    /// Install a proved IRQ binding without changing the counter or power state.
    /// Construction and waits never enable phase outputs or acknowledge faults.
    pub fn into_async(
        self,
        _irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ATIM, InterruptHandler>,
    ) -> AsyncThreePhasePwm<'d> {
        critical_section::with(|_| {
            UPDATE_STATE.reset();
            BREAK_STATE.reset();
            set_event_enabled(&mut HardwareEvents, WaitEvent::Update, false);
            set_event_enabled(&mut HardwareEvents, WaitEvent::Break, false);
            // SAFETY: caller supplied the type-level proof for this real vector.
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
        // Do not disable/unpend the shared vector, clear a fault, or touch MOE.
    }
}
impl AsyncThreePhasePwm<'_> {
    async fn wait_event(&mut self, event: WaitEvent) -> u32 {
        let _guard = WaitGuard::new(event);
        core::future::poll_fn(|cx| {
            event.state().register(cx.waker());
            let latched = event.state().take();
            // Covers hardware completion before IRQ entry (including historical
            // break latches). ISR clears UIF only after copying into EventState.
            let flags = critical_section::with(|_| HardwareEvents.status()) & event.flags();
            let result = latched | flags;
            if result != 0 {
                core::task::Poll::Ready(result)
            } else {
                core::task::Poll::Pending
            }
        })
        .await
    }
    /// Wait for the next update after first polling. Does not start the counter;
    /// call start_counter() explicitly. Updates coalesce, this is not a count.
    /// Cancellation masks only UIE, leaving the counter and outputs unchanged.
    pub async fn wait_update(&mut self) {
        self.wait_event(WaitEvent::Update).await;
    }
    /// Wait for a break, or return an already latched break immediately. Leaves
    /// all break latches and MOE untouched. Only acknowledge_fault() may clear
    /// latches; enable_outputs() remains a separate explicit operation.
    /// Cancellation masks only BIE, never hardware break protection BKE/BK2E.
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
        // inner Drop safely disables power outputs and counter as before.
    }
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
        unsafe {
            pac::ATIM
                .bdtr()
                .write_value(f::bdtr::MOE.write(pac::ATIM.bdtr().read(), on));
        }
    }
    fn fault(&mut self) -> bool {
        unsafe { pac::ATIM.isr().read() & fault_mask() != 0 }
    }
    fn enabled(&mut self) -> bool {
        unsafe { f::bdtr::MOE.read(pac::ATIM.bdtr().read()) }
    }
    fn channels(&mut self, on: bool) {
        unsafe {
            let mut cc = 0;
            cc = f::ccer::CC1E.write(cc, on);
            cc = f::ccer::CC1NE.write(cc, on);
            cc = f::ccer::CC2E.write(cc, on);
            cc = f::ccer::CC2NE.write(cc, on);
            cc = f::ccer::CC3E.write(cc, on);
            cc = f::ccer::CC3NE.write(cc, on);
            pac::ATIM.ccer().write_value(cc);
        }
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
    f::icr::BIF.mask() | f::icr::B2IF.mask() | f::icr::SBIF.mask()
}
impl Drop for ThreePhasePwm<'_> {
    fn drop(&mut self) {
        self.disable_outputs();
        unsafe {
            pac::ATIM
                .cr1()
                .write_value(f::cr1::CEN.write(pac::ATIM.cr1().read(), false));
            pac::ATIM.ccer().write_value(0);
        }
        for pin in &self.pins {
            pin.disconnect();
        }
    }
}
