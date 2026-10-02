//! The uploaded L012 BLDC board, with all MMIO isolated in this owner.
//!
//! This is a reviewed register port, not an electrically qualified motor drive.
//! Power outputs start as GPIO-low and require both the feature and an explicit
//! `arm_outputs` call. There is no verified external break input in the source.
//! A panic, or an EOS handler observing the next conversion already active,
//! disconnects the drive and never automatically rearms. A one-bit EOS cannot
//! count whole missed sequences; worst-case IRQ latency must be measured.
//!
//! Timing adaptation: HCLK=96 MHz, PCLK=48 MHz, ATIM=20 kHz, BTIM2/3=8 MHz.
//! VDD must be at least 1.8 V for 96 MHz (datasheet operating conditions).
//! Source 4800-count PWM values are divided by two at this hardware boundary.
//! Both ADCs run at 6 MHz. ADC1's 18+15 cycle conversions take 22 us for four
//! slots; ADC2's five 518+15 cycle slots take 444.167 us every 5 ms. These meet
//! RM v1.4 table 25-3's conservative 6 MHz / 200 ksps limits. The changed BEMF
//! acquisition time and sequential skew still require scope/analog validation.
//! ADC CR reserved bits are preserved; the uploaded SDK's conflicting SAM[9:8]
//! definition is intentionally not used. ADC2 uses coherent EOS snapshots,
//! replacing the C program's overlapping 20 kHz trigger + DMA + 5 ms restart.

use crate::board_contract::{pwm_count, validate_bridge};
use crate::control::{Actions, Bridge, TimerCommand};
use crate::frame_queue::FrameQueue;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    gpio::AnyPin,
    interrupt::{self, InterruptExt},
    pac, peripherals, rcc, Peri,
};

pub use crate::board_contract::{
    ADC1_SEQUENCE_NS, ADC2_SEQUENCE_NS, ADC_HZ, CPU_HZ, PCLK_HZ, PWM_HZ, PWM_PERIOD, STEP_TIMER_HZ,
};
const _: () = assert!(PWM_PERIOD == 2400 && ADC1_SEQUENCE_NS == 22_000);
const _: () = assert!(ADC1_SEQUENCE_NS < 1_000_000_000 / PWM_HZ);
const _: () = assert!(ADC2_SEQUENCE_NS < 5_000_000 && ADC_HZ / 33 <= 200_000);
const ADC_ICR_MASK: u32 = 0xf;
const BTIM_ICR_MASK: u32 = 0x41;
const HIGH_MASK: u32 = (1 << 5) | (1 << 6) | (1 << 7);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Owns only motor resources. Initialization returns a separate Ui owner for
/// the Embassy task; no UI pin/UART handle or forwarding method remains here.
pub struct Board {
    _motor: MotorResources,
    armed: bool,
    sampling_fault: bool,
    calibration_mv: u16,
}

struct MotorResources {
    _atim: Peri<'static, peripherals::ATIM>,
    _adc1: Peri<'static, peripherals::ADC1>,
    _adc2: Peri<'static, peripherals::ADC2>,
    _opa1: Peri<'static, peripherals::OPA1>,
    _bandgap: Peri<'static, peripherals::BGR>,
    _millisecond: Peri<'static, peripherals::BTIM1>,
    _elapsed: Peri<'static, peripherals::BTIM2>,
    _commutation: Peri<'static, peripherals::BTIM3>,
    _pins: [Peri<'static, AnyPin>; 16],
}

/// Single task-owned user interface, independent of the motor interrupt owner.
/// Normal-operation writes use GPIO SET/CLR registers; GPIO mux RMW is startup-only.
pub struct Ui {
    _uart: Peri<'static, peripherals::UART1>,
    _pins: [Peri<'static, AnyPin>; 4],
    tx: FrameQueue,
}

impl Board {
    pub fn new() -> Result<(Self, Ui), embassy_cw32::InitError> {
        let mut config = embassy_cw32::Config::default();
        config.rcc.hsi_frequency = rcc::HsiFrequency::Mhz96;
        config.rcc.pclk_divider = rcc::PclkDivider::Div2;
        let p = embassy_cw32::try_init(config)?;
        for irq in [
            interrupt::ADC1,
            interrupt::ADC2_DAC,
            interrupt::BTIM1,
            interrupt::BTIM3_HALLTIM,
        ] {
            irq.disable();
        }
        // SAFETY: global HAL initialization transferred all singletons here;
        // interrupts remain masked until the caller installs the IRQ-owned resources.
        unsafe {
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::AHBEN,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 4) | (1 << 5) | (1 << 6),
            );
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::APBEN1,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 0) | (1 << 3) | (1 << 5),
            );
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::APBEN2,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 2) | (1 << 9),
            );
            // All six gates get low latches before their output directions.
            for (port, pin) in [
                (pac::GPIOA_BASE, 15),
                (pac::GPIOB_BASE, 3),
                (pac::GPIOB_BASE, 4),
                (pac::GPIOB_BASE, 5),
                (pac::GPIOB_BASE, 6),
                (pac::GPIOB_BASE, 7),
            ] {
                configure_pin(port, pin, PinMode::OutputLow, 0);
            }
            configure_pin(pac::GPIOC_BASE, 13, PinMode::OutputHigh, 0);
            configure_pin(pac::GPIOA_BASE, 3, PinMode::InputPullUp, 0);
            for (port, pin) in [
                (pac::GPIOA_BASE, 0),
                (pac::GPIOA_BASE, 1),
                (pac::GPIOA_BASE, 2),
                (pac::GPIOA_BASE, 6),
                (pac::GPIOA_BASE, 7),
                (pac::GPIOB_BASE, 0),
                (pac::GPIOB_BASE, 2),
                (pac::GPIOA_BASE, 8),
                (pac::GPIOA_BASE, 10),
                (pac::GPIOA_BASE, 11),
            ] {
                configure_pin(port, pin, PinMode::Analog, 0);
            }
            configure_pin(pac::GPIOB_BASE, 12, PinMode::OutputHigh, 1);
            configure_pin(pac::GPIOB_BASE, 11, PinMode::InputPullUp, 1);
            initialize_pwm();
            // OPA1: external feedback, PA6 INP2, PA7 INN2, PB0 output.
            // PB0 is read by ADC1 CH8 without creating a second pin owner.
            pac::BGR
                .cr()
                .write(pac::bgr::fields::cr::BGREN.write(pac::BGR.cr().read(), true));
            pac::OPA1.cal().write(0);
            pac::OPA1.cr().write(0xe221);
            initialize_adc(pac::ADC1, 4, 0x2108, 0x4444);
            initialize_adc(pac::ADC2, 5, 0xf875b, 0xfffff);
            initialize_btim(pac::BTIM1, 47, 999, false);
            initialize_btim(pac::BTIM2, 5, 65530, false);
            initialize_btim(pac::BTIM3, 5, 65530, true);
            initialize_uart();
        }
        // >=1 ms nominal instruction delay: exceeds BGR (~30 us), OPA and ADC
        // startup requirements. It is not used as the motor timebase.
        cortex_m::asm::delay(CPU_HZ / 1_000);
        // SAFETY: documented factory calibration halfword, RM25.10/SDK
        // ADC_BGR_VOL_ADDRESS. Invalid erased calibration fails protection in
        // the core instead of substituting a fabricated calibrated voltage.
        let calibration_mv = unsafe { core::ptr::read_volatile(0x0010_07d2 as *const u16) };
        INITIALIZED.store(true, Ordering::Release);
        let board = Self {
            _motor: MotorResources {
                _atim: p.ATIM,
                _adc1: p.ADC1,
                _adc2: p.ADC2,
                _opa1: p.OPA1,
                _bandgap: p.BGR,
                _millisecond: p.BTIM1,
                _elapsed: p.BTIM2,
                _commutation: p.BTIM3,
                _pins: [
                    p.PA15.into(),
                    p.PB3.into(),
                    p.PB4.into(),
                    p.PB5.into(),
                    p.PB6.into(),
                    p.PB7.into(),
                    p.PA0.into(),
                    p.PA1.into(),
                    p.PA2.into(),
                    p.PA6.into(),
                    p.PA7.into(),
                    p.PB0.into(),
                    p.PB2.into(),
                    p.PA8.into(),
                    p.PA10.into(),
                    p.PA11.into(),
                ],
            },
            armed: false,
            sampling_fault: false,
            calibration_mv,
        };
        let ui = Ui {
            _uart: p.UART1,
            _pins: [p.PC13.into(), p.PA3.into(), p.PB11.into(), p.PB12.into()],
            tx: FrameQueue::new(),
        };
        Ok((board, ui))
    }

    /// Configure equal-priority motor IRQs and start their event sources.
    /// NVIC lines stay masked until main installs the IRQ-owned state, ends
    /// every reference to it, and calls the module-level `enable_interrupts`.
    ///
    /// # Safety
    /// ADC1, ADC2_DAC, BTIM1 and BTIM3_HALLTIM must remain NVIC-masked
    /// throughout preparation and until main has installed their state.
    pub unsafe fn prepare_interrupts(&mut self) {
        unsafe {
            pac::ADC1.icr().write(0);
            pac::ADC2.icr().write(0);
            pac::ADC1.ier().write(pac::adc::fields::ier::EOS.mask());
            pac::ADC2.ier().write(pac::adc::fields::ier::EOS.mask());
            pac::BTIM1.icr().write(BTIM_ICR_MASK & !1);
            pac::BTIM3.icr().write(BTIM_ICR_MASK & !1);
            pac::BTIM1.dier().write(1);
            pac::BTIM3.dier().write(1);
            for irq in [
                interrupt::ADC1,
                interrupt::ADC2_DAC,
                interrupt::BTIM1,
                interrupt::BTIM3_HALLTIM,
            ] {
                irq.unpend();
                irq.set_priority(interrupt::Priority::P1);
            }
            pac::ADC1
                .trigger()
                .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());
            pac::BTIM1.cr1().write(5); // EN, URS: hardware overflows only.
            pac::BTIM2.cr1().write(5);
            pac::ATIM
                .cr1()
                .write(pac::atim::fields::cr1::ARPE.mask() | 1);
        }
        self.start_adc2();
    }

    pub fn calibration_mv(&self) -> u16 {
        self.calibration_mv
    }
    pub fn sampling_fault(&self) -> bool {
        self.sampling_fault
    }
    /// Permission state, not a claim that any gate is currently energized.
    pub fn outputs_armed(&self) -> bool {
        self.armed && !self.sampling_fault
    }
    pub fn arm_outputs(&mut self) -> bool {
        self.armed = cfg!(feature = "motor-output-enable") && !self.sampling_fault;
        self.armed
    }
    pub fn disable_outputs(&mut self) {
        self.armed = false;
        unsafe { drive_off() };
    }
    pub fn step_ticks(&self) -> u16 {
        unsafe { pac::BTIM2.cnt().read() as u16 }
    }

    /// Freeze the trigger while reading an EOS tuple. If a delayed handler finds
    /// the next conversion active, fail closed rather than use a torn sequence.
    pub fn adc1_irq(&mut self) -> Option<[u16; 4]> {
        let r = pac::ADC1;
        unsafe {
            if !pac::adc::fields::isr::EOS.read(r.isr().read()) {
                return None;
            }
            r.trigger().write(0);
            if pac::adc::fields::start::START.read(r.start().read()) {
                r.start().write(0);
                r.ier().write(0);
                r.icr()
                    .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
                self.sampling_fault = true;
                self.disable_outputs();
                return None;
            }
            let result = [
                r.result0().read() as u16 & 4095,
                r.result1().read() as u16 & 4095,
                r.result2().read() as u16 & 4095,
                r.result3().read() as u16 & 4095,
            ];
            r.icr()
                .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
            r.trigger()
                .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());
            Some(result)
        }
    }
    pub fn adc2_irq(&mut self) -> Option<[u16; 5]> {
        let r = pac::ADC2;
        unsafe {
            if !pac::adc::fields::isr::EOS.read(r.isr().read())
                || pac::adc::fields::start::START.read(r.start().read())
            {
                return None;
            }
            let result = [
                r.result0().read() as u16 & 4095,
                r.result1().read() as u16 & 4095,
                r.result2().read() as u16 & 4095,
                r.result3().read() as u16 & 4095,
                r.result4().read() as u16 & 4095,
            ];
            r.icr()
                .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
            Some(result)
        }
    }
    pub fn ms_irq(&mut self) -> bool {
        take_timer_update(pac::BTIM1)
    }
    pub fn commutation_irq(&mut self) -> bool {
        if !take_timer_update(pac::BTIM3) {
            return false;
        }
        unsafe { pac::BTIM3.cr1().write(12) };
        true
    }
    pub fn start_adc2(&mut self) {
        unsafe {
            // Never restart a conversion or erase an unread coherent tuple.
            if !pac::adc::fields::start::START.read(pac::ADC2.start().read())
                && !pac::adc::fields::isr::EOS.read(pac::ADC2.isr().read())
            {
                pac::ADC2.start().write(1);
            }
        }
    }
    pub fn apply(&mut self, actions: Actions) {
        if let Some(bridge) = actions.bridge {
            self.apply_bridge(bridge);
        }
        unsafe {
            if let Some(ticks) = actions.step_timer_preset {
                pac::BTIM2.cnt().write(ticks.into());
            }
            match actions.sensorless_timer {
                TimerCommand::Unchanged => {}
                TimerCommand::Stop => pac::BTIM3.cr1().write(12),
                TimerCommand::Arm(reload) => {
                    pac::BTIM3.cr1().write(12);
                    // Preserve source ARR semantics: expires after reload+1 ticks.
                    pac::BTIM3.arr().write(reload.into());
                    pac::BTIM3.cnt().write(0);
                    pac::BTIM3.egr().write(1);
                    pac::BTIM3.icr().write(BTIM_ICR_MASK & !1);
                    pac::BTIM3.cr1().write(13); // EN, URS, ONESHOT.
                }
            }
        }
        if actions.start_adc2 {
            self.start_adc2();
        }
    }
    pub fn apply_bridge(&mut self, bridge: Bridge) {
        use pac::atim::fields::{bdtr, ccer};
        // Startup has masked IRQs; afterward only equal-priority motor IRQs
        // own this Board. Fatal exceptions remove drive and never return, so
        // no interrupted gate transaction can resume after emergency shutdown.
        unsafe {
            drive_off();
            if validate_bridge(&bridge).is_err() {
                self.armed = false;
                return;
            }
            pac::ATIM
                .ccr4()
                .write(pwm_count(bridge.sample_compare).into());
            if !self.armed || self.sampling_fault {
                return;
            }
            if break_pending() {
                self.armed = false;
                return;
            }
            pac::ATIM
                .ccr1()
                .write(pwm_count(bridge.pwm_counts[0]).into());
            pac::ATIM
                .ccr2()
                .write(pwm_count(bridge.pwm_counts[1]).into());
            pac::ATIM
                .ccr3()
                .write(pwm_count(bridge.pwm_counts[2]).into());
            for pin in [5, 6, 7] {
                set_af(pac::GPIOB_BASE, pin, 7);
            }
            // drive_off disconnected CC1E/CC2E/CC3E. Turn on the master
            // first, then recheck the asynchronous hardware break before
            // reconnecting channels so a racing break cannot be overridden.
            pac::ATIM
                .bdtr()
                .write(bdtr::MOE.write(pac::ATIM.bdtr().read(), true));
            if break_pending() || !bdtr::MOE.read(pac::ATIM.bdtr().read()) {
                drive_off();
                self.armed = false;
                return;
            }
            pac::ATIM.ccer().write(
                ccer::CC1E.mask() | ccer::CC2E.mask() | ccer::CC3E.mask() | ccer::CC4E.mask(),
            );
            if break_pending() || !bdtr::MOE.read(pac::ATIM.bdtr().read()) {
                drive_off();
                self.armed = false;
                return;
            }
            if bridge.low_sides[0] {
                pac::write(pac::GPIOA_BASE + pac::gpio::BSRR, 1 << 15);
            }
            if bridge.low_sides[1] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 3);
            }
            if bridge.low_sides[2] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 4);
            }
        }
    }
}

impl Ui {
    pub fn button_pressed(&self) -> bool {
        unsafe { pac::read(pac::GPIOA_BASE + pac::gpio::IDR) & (1 << 3) == 0 }
    }
    pub fn set_led(&mut self, on: bool) {
        // PC13 LED is active-low in global.h.
        unsafe {
            pac::write(
                pac::GPIOC_BASE + if on { pac::gpio::BRR } else { pac::gpio::BSRR },
                1 << 13,
            )
        }
    }
    /// Keep the active frame whole plus one newest pending frame. Subsequent
    /// status updates coalesce in the pending slot instead of corrupting bytes
    /// or losing the final power-off/fault status behind an active transmission.
    pub fn queue_frame(&mut self, frame: [u8; 7]) -> bool {
        self.tx.push(frame);
        true
    }
    /// Nonblocking, bounded to one transmitted byte per call.
    pub fn service_uart(&mut self) {
        if unsafe { pac::uart::fields::isr::TXE.read(pac::UART1.isr().read()) } {
            if let Some(byte) = self.tx.pop_byte() {
                unsafe { pac::UART1.tdr().write(u32::from(byte)) };
            }
        }
    }
}

impl Drop for Board {
    fn drop(&mut self) {
        self.disable_outputs();
        unsafe {
            pac::ADC1.trigger().write(0);
            pac::ADC2.trigger().write(0);
            pac::ADC1.start().write(0);
            pac::ADC2.start().write(0);
            pac::ADC1.ier().write(0);
            pac::ADC2.ier().write(0);
            for r in [pac::BTIM1, pac::BTIM2, pac::BTIM3] {
                r.dier().write(0);
                r.cr1().write(0);
            }
            pac::ATIM.cr1().write(0);
        }
    }
}

fn break_pending() -> bool {
    use pac::atim::fields::isr as f;
    unsafe { pac::ATIM.isr().read() & (f::BIF.mask() | f::B2IF.mask() | f::SBIF.mask()) != 0 }
}

/// Unmask the prepared motor IRQs without borrowing their owner.
///
/// # Safety
/// Main must have installed all motor state and the ADC1, ADC2_DAC, BTIM1
/// and BTIM3_HALLTIM handlers, and ended every reference to that state. These
/// vectors must exclusively use that state at the P1 priority set by
/// `prepare_interrupts`; no foreground or other interrupt may access it.
pub unsafe fn enable_interrupts() {
    for irq in [
        interrupt::ADC1,
        interrupt::ADC2_DAC,
        interrupt::BTIM1,
        interrupt::BTIM3_HALLTIM,
    ] {
        unsafe { irq.enable() };
    }
}

/// Fatal shutdown. It can run before `Board::new`; no MMIO is touched then.
/// The caller must never resume motor code afterward. Interrupts remain masked
/// and no Board reference is needed, even if an exception interrupted its owner.
pub fn emergency_stop() {
    cortex_m::interrupt::disable();
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe { drive_off() };
    }
}

unsafe fn drive_off() {
    unsafe {
        pac::ATIM
            .bdtr()
            .write(pac::atim::fields::bdtr::MOE.write(pac::ATIM.bdtr().read(), false));
        pac::ATIM.ccer().write(pac::atim::fields::ccer::CC4E.mask());
        pac::write(pac::GPIOA_BASE + pac::gpio::BRR, 1 << 15);
        pac::write(
            pac::GPIOB_BASE + pac::gpio::BRR,
            HIGH_MASK | (1 << 3) | (1 << 4),
        );
        for pin in [5, 6, 7] {
            set_af(pac::GPIOB_BASE, pin, 0);
        }
        pac::ATIM.ccr1().write(0);
        pac::ATIM.ccr2().write(0);
        pac::ATIM.ccr3().write(0);
    }
}
enum PinMode {
    OutputLow,
    OutputHigh,
    InputPullUp,
    Analog,
}
unsafe fn set_af(port: usize, pin: u32, af: u32) {
    let offset = if pin < 8 {
        pac::gpio::AFRL
    } else {
        pac::gpio::AFRH
    };
    let shift = (pin % 8) * 4;
    unsafe { pac::modify(port + offset, 0xf << shift, af << shift) };
}
unsafe fn configure_pin(port: usize, pin: u32, mode: PinMode, af: u32) {
    let mask = 1 << pin;
    unsafe {
        pac::modify(port + pac::gpio::DIR, 0, mask);
        set_af(port, pin, af);
        pac::modify(port + pac::gpio::RISEIE, mask, 0);
        pac::modify(port + pac::gpio::FALLIE, mask, 0);
        pac::modify(port + pac::gpio::OPENDRAIN, mask, 0);
        pac::modify(
            port + pac::gpio::PUR,
            mask,
            if matches!(mode, PinMode::InputPullUp) {
                mask
            } else {
                0
            },
        );
        pac::modify(
            port + pac::gpio::ANALOG,
            mask,
            if matches!(mode, PinMode::Analog) {
                mask
            } else {
                0
            },
        );
        if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
            pac::write(
                port + if matches!(mode, PinMode::OutputHigh) {
                    pac::gpio::BSRR
                } else {
                    pac::gpio::BRR
                },
                mask,
            );
            pac::modify(port + pac::gpio::DIR, mask, 0);
        }
    }
}
unsafe fn initialize_pwm() {
    use pac::atim::fields as f;
    unsafe {
        pac::ATIM.cr1().write(0);
        pac::ATIM.bdtr().write(0);
        pac::ATIM.dier().write(0);
        pac::ATIM.ccer().write(0);
        pac::ATIM.cr2().write(0);
        pac::ATIM.smcr().write(0);
        pac::ATIM.psc().write(0);
        pac::ATIM.arr().write(u32::from(PWM_PERIOD - 1));
        pac::ATIM.rcr().write(0);
        pac::ATIM.cnt().write(0);
        // Immediate compares while MOE is masked in apply_bridge avoid leaving
        // old preloaded phase duty active with a newly switched GPIO low side.
        pac::ATIM
            .ccmr1cmp()
            .write(f::ccmr1cmp::OC1M.write(f::ccmr1cmp::OC2M.write(0, 6), 6));
        // RM25.12.6 uses OC4REFC rising, not generic CC4 match. PWM2 rises
        // at CCR4 in edge/up mode; source PWM1 would rise at rollover instead.
        // Only sampling compare is preloaded to avoid a spurious trigger while
        // changing the desired sample point partway through a PWM period.
        pac::ATIM.ccmr2cmp().write(f::ccmr2cmp::OC4PE.write(
            f::ccmr2cmp::OC3M.write(f::ccmr2cmp::OC4M.write(0, 7), 6),
            true,
        ));
        pac::ATIM.ccr1().write(0);
        pac::ATIM.ccr2().write(0);
        pac::ATIM.ccr3().write(0);
        pac::ATIM.ccr4().write(2000); // source stopped4000 /2.
        pac::ATIM.dtr2().write(0);
        pac::ATIM.af1().write(0);
        pac::ATIM.af2().write(0);
        pac::ATIM
            .bdtr()
            .write(f::bdtr::OSSI.mask() | f::bdtr::OSSR.mask());
        pac::ATIM.ccer().write(
            f::ccer::CC1E.mask()
                | f::ccer::CC2E.mask()
                | f::ccer::CC3E.mask()
                | f::ccer::CC4E.mask(),
        );
        pac::ATIM.egr().write(1);
        pac::ATIM.icr().write(0);
    }
}
unsafe fn initialize_adc(r: pac::adc::RegisterBlock, length: u32, channels: u32, sample: u32) {
    use pac::adc::fields as f;
    unsafe {
        r.trigger().write(0);
        r.start().write(0);
        r.ier().write(0);
        let reserved = r.cr().read() & !0xff;
        r.cr().write(reserved);
        r.awdcr().write(0);
        r.sqrcfr().write(channels);
        r.sample().write(sample);
        r.icr().write(0);
        r.cr().write(f::cr::EN.write(
            f::cr::ENS.write(f::cr::CLK.write(reserved, 3), length - 1),
            true,
        ));
    }
}
unsafe fn initialize_btim(r: pac::btim::RegisterBlock, prescaler: u32, reload: u32, oneshot: bool) {
    unsafe {
        r.cr1().write(if oneshot { 12 } else { 4 });
        r.dier().write(0);
        r.cr2().write(0);
        r.smcr().write(0);
        r.psc().write(prescaler);
        r.arr().write(reload);
        r.cnt().write(0);
        r.egr().write(1);
        r.icr().write(0);
    }
}
fn take_timer_update(r: pac::btim::RegisterBlock) -> bool {
    unsafe {
        if r.isr().read() & r.dier().read() & 1 == 0 {
            return false;
        }
        r.icr().write(BTIM_ICR_MASK & !1);
    }
    true
}
unsafe fn initialize_uart() {
    unsafe {
        pac::UART1.ier().write(0);
        pac::UART1.cr1().write(0x1003); // PCLK, oversample16, 8N1, RX+TX.
        pac::UART1.cr2().write(0);
        pac::UART1.cr3().write(0);
        // 48MHz/(16*115200)=26.04166; integer26, fractional1/16.
        pac::UART1.brri().write(26);
        pac::UART1.brrf().write(1);
    }
}
