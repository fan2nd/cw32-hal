#![deny(unsafe_code)]
//! Port of `MOTOR.C`, `sensorless.c`, `control.c`, and the BTIM1/main loop.
//!
//! Timing units matter: BTIM2/3 count at **8 MHz**, PWM compares use a 4800-count
//! period, and [`MotorController::tick_1ms`] runs at 1 kHz. `StepTime >> 3` is
//! preserved from the source (one eighth of the previous commutation interval),
//! rather than silently replacing it with a textbook 30-degree delay.
//!
//! Timer ISRs only update the source counters and key state. The motor
//! task calls foreground_step on real hardware events, draining ready continuations. Explicit continuations
//! preserve the original blocking waits without holding a Rust borrow across
//! an interrupt. Protection is paused during startup, settling and error loops.
//! Original state-order edge cases are deliberately retained.

use crate::protection::{Adc2Sample, Fault, Measurements, Protection};

pub const PWM_PERIOD: u16 = 4800;
pub const ONE_PERCENT_PWM: u16 = PWM_PERIOD / 100;
pub const SENSORLESS_TIMER_HZ: u32 = 8_000_000;
pub const START_DELAY_MS: u16 = 400;
pub const ALIGNMENT_MS: u16 = 150;
pub const FORCED_STEP_TIMEOUT_MS: u16 = 10;
pub const MAX_STARTUP_STEPS: u16 = 200;
pub const STARTUP_CROSSINGS: u16 = 15;
pub const RAMP_FULL_SCALE_MS: u16 = 3000;
pub const KEY_DEBOUNCE_MS: u16 = 60;
pub const AUTO_POWER_OFF_MS: u16 = 10_000;
pub const MEASUREMENT_INTERVAL_MS: u16 = 100;
pub const STOP_SETTLE_MS: u16 = 500;

/// Source state numbers are retained for diagnostics and source comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MotorState {
    StartCheck = 0,
    Stop = 3,
    Error = 5,
    ErrorOver = 6,
    WaitStart = 7,
    StartDelay = 8,
    StartOpen = 9,
    RunOpen = 10,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupState {
    Inactive,
    Aligning,
    Forced,
}

/// `Sta` values from sensorless.c, made explicit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SensorlessState {
    Idle = 0,
    Demagnetizing = 1,
    Detecting = 2,
    WaitingCommutation = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Edge {
    Rising,
    Falling,
}

/// The C uses `TAB_RFling[1]` and ADC channels `{3,2,1,3,2,1}`.
pub const EXPECTED_EDGE: [Edge; 6] = [
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
];
pub const BEMF_CHANNEL: [usize; 6] = [3, 2, 1, 3, 2, 1];

/// ADC1 sequence is current, phase A, phase B, phase C. Only the floating
/// phase enters the source zero-crossing detector; bus threshold is ADC2/2.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Adc1Sample {
    pub current: u16,
    pub phase_a: u16,
    pub phase_b: u16,
    pub phase_c: u16,
}

impl From<[u16; 4]> for Adc1Sample {
    fn from(value: [u16; 4]) -> Self {
        Self {
            current: value[0],
            phase_a: value[1],
            phase_b: value[2],
            phase_c: value[3],
        }
    }
}

fn crossing_side_matches(sector: u8, sample: Adc1Sample, bus_adc: u16) -> bool {
    let phases = [
        sample.current,
        sample.phase_a,
        sample.phase_b,
        sample.phase_c,
    ];
    let floating = phases[BEMF_CHANNEL[sector as usize]];
    let threshold = bus_adc >> 1;
    match EXPECTED_EDGE[sector as usize] {
        Edge::Rising => floating > threshold,
        Edge::Falling => floating < threshold,
    }
}

/// Complete desired bridge image. The board must perform break-before-make
/// when applying it: remove old low-side/high-side drive before enabling new
/// drive. A compare value is not a low-side gate signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bridge {
    pub pwm_counts: [u32; 3],
    pub low_sides: [bool; 3],
    pub sample_compare: u16,
}

impl Bridge {
    pub const fn off() -> Self {
        Self {
            pwm_counts: [0; 3],
            low_sides: [false; 3],
            sample_compare: PWM_PERIOD - 800,
        }
    }

    /// The source's six-tick (while TimeCountTemp <= 5) bootstrap charge. Applying and
    /// ending this is an explicit board initialization step, not implicit in
    /// `MotorController::new`.
    pub const fn bootstrap() -> Self {
        Self {
            pwm_counts: [0; 3],
            low_sides: [true; 3],
            sample_compare: PWM_PERIOD - 800,
        }
    }

    /// Six-step table, sectors 0..5: A+B-, A+C-, B+C-, B+A-, C+A-, C+B-.
    /// Invalid sectors fail closed instead of retaining a previous bridge.
    pub const fn commutation(sector: u8, duty: u32) -> Self {
        let (high, low) = match sector {
            0 => (0, 1),
            1 => (0, 2),
            2 => (1, 2),
            3 => (1, 0),
            4 => (2, 0),
            5 => (2, 1),
            _ => return Self::off(),
        };
        let mut result = Self {
            pwm_counts: [0; 3],
            low_sides: [false; 3],
            sample_compare: 300,
        };
        result.pwm_counts[high] = duty;
        result.low_sides[low] = true;
        result
    }

    /// A+ with both B- and C- for initial rotor alignment.
    pub const fn alignment(duty: u32) -> Self {
        let mut result = Self::commutation(0, duty);
        result.low_sides[2] = true;
        result
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimerCommand {
    #[default]
    Unchanged,
    Stop,
    /// BTIM3 auto-reload value, in 8 MHz ticks, exactly as in the source.
    Arm(u16),
}

/// Effects for the hardware adapter to apply once, in event order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[must_use = "apply returned bridge/timer/ADC/LED actions to the board"]
pub struct Actions {
    pub bridge: Option<Bridge>,
    /// UPPWM updates compares without changing any low-side GPIO.
    pub pwm_only: bool,
    /// Source alignment calls UPPWM on STEP_last before Commutation(0).
    pub pre_alignment_pwm: Option<(u8, u32)>,
    pub sensorless_timer: TimerCommand,
    /// Set the free-running BTIM2 counter. Commutation resets it to zero.
    pub step_timer_preset: Option<u16>,
    pub start_adc2: bool,
    pub led_on: Option<bool>,
}

/// Single-owner, allocation-free state machine. Serialize calls from IRQs or
/// run them in a foreground event loop; shared interrupt globals are not needed.
pub struct MotorController {
    state: MotorState,
    startup: StartupState,
    sensorless: SensorlessState,
    protection: Protection,
    adc2: Adc2Sample,
    adc1: Adc1Sample,
    adc2_ready: bool,
    adc1_age_ms: u16,
    adc2_age_ms: u16,
    fault: Option<Fault>,
    powered_off: bool,
    speed_level: u8,
    set_speed: u8,
    target_percent: u32,
    duty: u32,
    bridge: Bridge,
    sector: u8,
    last_sector: u8,
    step_time: u16,
    crossing_samples: u8,
    required_samples: u8,
    good_crossings: u32,
    startup_iterations: u16,
    crossing_flag: bool,
    stop_waiting: bool,
    initialized_measurements: bool,
    speed_ms: u32,
    captured_commutations: u32,
    startup_locked: bool,
    motor_enabled: bool,
    key_hold_ms: u16,
    adc2_ms: u8,
    state_ms: u32,
    ramp_ms: u32,
    measurement_ms: u32,
    idle_ms: u32,
    commutations_in_window: u32,
    reported_speed: u32,
    stall_windows: u8,
    led_on: bool,
}

impl MotorController {
    pub fn new(calibration_mv: u16) -> Self {
        Self {
            state: MotorState::StartCheck,
            startup: StartupState::Inactive,
            sensorless: SensorlessState::Idle,
            protection: Protection::new(calibration_mv),
            adc2: Adc2Sample {
                current: 0,
                bus_voltage: 0,
                potentiometer: 0,
                temperature: 0,
                reference: 0,
            },
            adc1: Adc1Sample::default(),
            adc2_ready: false,
            adc1_age_ms: u16::MAX,
            adc2_age_ms: u16::MAX,
            fault: None,
            powered_off: true,
            speed_level: 0,
            set_speed: 0,
            target_percent: 0,
            duty: 0,
            bridge: Bridge::off(),
            sector: 0,
            last_sector: 0,
            step_time: 0,
            crossing_samples: 0,
            required_samples: 2,
            good_crossings: 0,
            startup_iterations: 0,
            crossing_flag: false,
            stop_waiting: false,
            initialized_measurements: false,
            speed_ms: 0,
            captured_commutations: 0,
            startup_locked: false,
            motor_enabled: false,
            key_hold_ms: 0,
            adc2_ms: 0,
            state_ms: 0,
            ramp_ms: 0,
            measurement_ms: 0,
            idle_ms: 0,
            commutations_in_window: 0,
            reported_speed: 0,
            stall_windows: 0,
            led_on: false,
        }
    }

    /// Before User/main.c sets closeflag=1, BTIM1 counts through bootstrap.
    pub fn begin_bootstrap(&mut self) {
        self.powered_off = false;
    }

    /// The source sets closeflag=1 and timecountforClose=1000 after bootstrap.
    /// The output feature gates hardware only, not these logical timer ticks.
    pub fn finish_bootstrap(&mut self) {
        self.powered_off = true;
        self.idle_ms = 1000;
    }

    pub const fn state(&self) -> MotorState {
        self.state
    }
    pub const fn startup_state(&self) -> StartupState {
        self.startup
    }
    pub const fn sensorless_state(&self) -> SensorlessState {
        self.sensorless
    }
    pub const fn fault(&self) -> Option<Fault> {
        self.fault
    }
    pub const fn powered_off(&self) -> bool {
        self.powered_off
    }
    pub const fn speed_level(&self) -> u8 {
        self.speed_level
    }
    pub const fn speed_percent(&self) -> u8 {
        self.set_speed
    }
    pub const fn duty(&self) -> u32 {
        self.duty
    }
    pub const fn bridge(&self) -> Bridge {
        self.bridge
    }
    pub const fn sector(&self) -> u8 {
        self.sector
    }
    pub const fn step_time(&self) -> u16 {
        self.step_time
    }
    pub const fn good_crossings(&self) -> u32 {
        self.good_crossings
    }
    pub const fn required_crossing_samples(&self) -> u8 {
        self.required_samples
    }
    pub const fn motor_enabled(&self) -> bool {
        self.motor_enabled
    }
    pub const fn adc1_age_ms(&self) -> u16 {
        self.adc1_age_ms
    }
    pub const fn adc2_age_ms(&self) -> u16 {
        self.adc2_age_ms
    }
    /// Source `RealS = commutations_in_100ms * 100`; not mechanical RPM without
    /// the motor pole-pair and commutations-per-revolution conversion.
    pub const fn reported_speed(&self) -> u32 {
        self.reported_speed
    }
    pub fn measurements(&self) -> Measurements {
        self.protection.measurements()
    }

    /// Test/board-level speed command; the original key chooses 0/20/40/60/80/100.
    /// This does not power on or clear a fault. Values above 100 are bounded.
    pub fn set_speed_percent(&mut self, percent: u8) {
        self.set_speed = percent.min(100);
        self.speed_level = self.set_speed / 20;
    }

    /// Called on completed ADC2 sequence; conversion is requested every 5ms.
    pub fn update_adc2(&mut self, sample: Adc2Sample) {
        if !self.adc2_ready {
            self.protection.set_current_offset(sample.current);
        }
        self.adc2 = sample;
        self.adc2_ready = true;
        self.adc2_age_ms = 0;
    }

    fn set_led(&mut self, on: bool, actions: &mut Actions) {
        if self.led_on != on {
            self.led_on = on;
            actions.led_on = Some(on);
        }
    }

    fn set_bridge(&mut self, bridge: Bridge, actions: &mut Actions) {
        self.bridge = bridge;
        actions.bridge = Some(bridge);
    }

    // MOTOR_STOP0 does not clear the C duty, filter or BTIM3 state.
    fn motor_stop(&mut self, actions: &mut Actions) {
        self.motor_enabled = false;
        self.set_bridge(Bridge::off(), actions);
    }

    /// Assign the source ErrorCode. The foreground observes it on its next
    /// outer-loop iteration; the ISR does not invent immediate shutdown.
    pub fn report_fault(&mut self, fault: Fault) -> Actions {
        self.fault = Some(fault);
        Actions::default()
    }

    fn key_pressed(&mut self, actions: &mut Actions) {
        self.idle_ms = 0;
        if self.powered_off {
            self.powered_off = false;
            self.fault = None;
            self.protection.reset();
            self.set_speed = 0;
            self.speed_level = 0;
            self.state = MotorState::StartCheck;
            self.set_led(true, actions);
        } else if self.fault.is_some() {
            self.fault = None;
            self.protection.reset();
            self.set_speed = 0;
            self.speed_level = 0;
            // The C key ISR clears ErrorCode only. MotorErrorOver resumes
            // into WAITSTART in the foreground, rather than here.
        } else {
            self.speed_level = (self.speed_level + 1) % 6;
            self.set_speed = self.speed_level * 20;
        }
    }

    /// BTIM1 bookkeeping only. The motor foreground is not a 1 kHz task.
    pub fn tick_1ms(&mut self, key_pressed: bool, _step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        self.adc1_age_ms = self.adc1_age_ms.saturating_add(1);
        self.adc2_age_ms = self.adc2_age_ms.saturating_add(1);
        if key_pressed {
            // UI choice: saturating debounce avoids repeat presses on wrap.
            self.key_hold_ms = self.key_hold_ms.saturating_add(1);
            if self.key_hold_ms == KEY_DEBOUNCE_MS {
                self.key_pressed(&mut actions);
            }
        } else {
            self.key_hold_ms = 0;
        }
        self.adc2_ms += 1;
        if self.adc2_ms >= 5 {
            self.adc2_ms = 0;
            actions.start_adc2 = true;
        }
        if self.powered_off {
            return actions;
        }
        if self.set_speed == 0 || self.fault.is_some() {
            self.idle_ms = self.idle_ms.wrapping_add(1);
        }
        self.state_ms = self.state_ms.wrapping_add(1);
        self.ramp_ms = self.ramp_ms.wrapping_add(1);
        self.measurement_ms = self.measurement_ms.wrapping_add(1);
        self.speed_ms = self.speed_ms.wrapping_add(1);
        if self.speed_ms >= 100 {
            self.captured_commutations = self.commutations_in_window;
            self.commutations_in_window = 0;
            self.speed_ms = 0;
        }
        actions
    }

    /// Whether another source foreground continuation can advance immediately.
    /// Call after one foreground_step for a real event. This is not a timer or
    /// a replacement for interrupt processing: ADC filtering and timer state
    /// transitions have already happened in their hardware handlers.
    pub fn foreground_ready(&self) -> bool {
        if !self.initialized_measurements {
            return self.adc2_ready;
        }
        if self.powered_off {
            return false;
        }
        match self.startup {
            StartupState::Aligning => return self.state_ms >= u32::from(ALIGNMENT_MS),
            StartupState::Forced => {
                // A fault alone does not escape the original inner busy wait.
                return self.state_ms >= u32::from(FORCED_STEP_TIMEOUT_MS)
                    || self.crossing_flag
                    || self.startup_locked;
            }
            StartupState::Inactive => {}
        }
        if self.stop_waiting {
            return self.state_ms >= u32::from(STOP_SETTLE_MS);
        }
        if self.state == MotorState::ErrorOver {
            // Blink is presentation work done once per real event, not an
            // endlessly-ready continuation after the initial 200 ms pause.
            return self.state_ms >= 200
                && (self.fault.is_none() || self.idle_ms >= u32::from(AUTO_POWER_OFF_MS));
        }
        if self.measurement_ms >= u32::from(MEASUREMENT_INTERVAL_MS)
            || (self.fault.is_some() && self.state != MotorState::Error)
            || self.idle_ms >= u32::from(AUTO_POWER_OFF_MS)
        {
            return true;
        }
        match self.state {
            MotorState::StartCheck => self.set_speed > 0,
            MotorState::StartDelay => {
                self.set_speed == 0 || self.state_ms >= u32::from(START_DELAY_MS)
            }
            MotorState::StartOpen | MotorState::Stop | MotorState::Error => true,
            MotorState::RunOpen => {
                self.set_speed == 0 || self.ramp_ms >= u32::from(RAMP_FULL_SCALE_MS / 100)
            }
            MotorState::WaitStart => self.set_speed == 0,
            MotorState::ErrorOver => false,
        }
    }

    /// One original main-loop/continuation iteration. Drain foreground_ready
    /// before awaiting a new hardware event; do not impose a fixed 1 ms poll.
    /// No controller reference may survive await or return from a handler.
    pub fn foreground_step(&mut self, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        if !self.initialized_measurements {
            if !self.adc2_ready {
                return actions;
            }
            self.protection.set_current_offset(self.adc2.current);
            if let Err(fault) = self.protection.sample_vi(self.adc2, self.fault) {
                self.fault = Some(fault);
            }
            self.initialized_measurements = true;
        }
        if self.powered_off {
            return actions;
        }

        // These are continuations of blocking C functions. In particular,
        // they bypass SampleVI/SampleT and the outer ErrorCode dispatch.
        if self.startup != StartupState::Inactive {
            self.advance_startup(step_ticks, &mut actions);
            return actions;
        }
        if self.stop_waiting {
            if self.state_ms >= u32::from(STOP_SETTLE_MS) {
                self.stop_waiting = false;
                self.sensorless = SensorlessState::Idle;
                self.state = MotorState::StartCheck;
            }
            return actions;
        }
        if self.state == MotorState::ErrorOver {
            if self.state_ms < 200 {
                return actions;
            }
            if self.fault.is_none() {
                self.set_led(true, &mut actions);
                self.state = MotorState::WaitStart;
            } else {
                self.blink_fault(&mut actions);
                self.auto_power_off(&mut actions);
            }
            return actions;
        }

        if self.measurement_ms >= u32::from(MEASUREMENT_INTERVAL_MS) {
            self.measurement_ms = 0;
            if let Err(fault) = self.protection.sample(self.adc2, self.fault) {
                self.fault = Some(fault);
            }
            self.reported_speed = self.captured_commutations.wrapping_mul(100);
            if self.reported_speed < 30 && self.state == MotorState::RunOpen {
                self.stall_windows = self.stall_windows.wrapping_add(1);
                if self.stall_windows >= 50 {
                    self.stall_windows = 0;
                    // The source does not gate this assignment on ErrorCode.
                    self.fault = Some(Fault::Stall);
                }
            } else {
                self.stall_windows = 0;
            }
        }
        if self.fault.is_some()
            && self.state != MotorState::Error
            && self.state != MotorState::ErrorOver
        {
            self.state = MotorState::Error;
        }
        match self.state {
            MotorState::StartCheck => {
                if self.set_speed > 0 {
                    self.state = MotorState::StartDelay;
                    self.target_percent = 0;
                    self.state_ms = 0;
                }
            }
            MotorState::StartDelay => {
                // Deliberately independent conditions: the C can overwrite
                // STOP with STARTOPEN when cancellation coincides with expiry.
                if self.set_speed == 0 {
                    self.state = MotorState::Stop;
                }
                if self.state_ms >= u32::from(START_DELAY_MS) {
                    self.state_ms = 0;
                    self.state = MotorState::StartOpen;
                    self.protection.set_current_offset(self.adc2.current);
                }
            }
            MotorState::StartOpen => self.begin_alignment(&mut actions),
            MotorState::RunOpen => {
                if self.set_speed == 0 {
                    self.state = MotorState::Stop;
                }
                if self.ramp_ms >= u32::from(RAMP_FULL_SCALE_MS / 100) {
                    self.ramp_ms = 0;
                    if self.target_percent < u32::from(self.set_speed) {
                        self.target_percent += 1;
                        self.target_percent = self.target_percent.min(u32::from(self.set_speed));
                        self.update_pwm(&mut actions);
                    } else if self.target_percent > u32::from(self.set_speed) {
                        self.target_percent = u32::from(self.set_speed);
                        self.update_pwm(&mut actions);
                    }
                }
            }
            MotorState::Stop => {
                self.motor_stop(&mut actions);
                // RealS1 is initialized to zero and never assigned nonzero
                // in the supplied project: this branch always waits 500 ms.
                if self.adc1.phase_a < 180 && self.adc1.phase_b < 180 && self.adc1.phase_c < 180 {
                    self.sensorless = SensorlessState::Idle;
                }
                self.state_ms = 0;
                self.stop_waiting = true;
            }
            MotorState::Error => {
                self.motor_stop(&mut actions);
                self.state = MotorState::ErrorOver;
                self.state_ms = 0;
                self.set_led(false, &mut actions);
            }
            MotorState::WaitStart => {
                if self.set_speed == 0 {
                    self.state = MotorState::StartCheck;
                }
            }
            MotorState::ErrorOver => {}
        }
        self.auto_power_off(&mut actions);
        actions
    }

    fn auto_power_off(&mut self, actions: &mut Actions) {
        if self.idle_ms >= u32::from(AUTO_POWER_OFF_MS) {
            self.powered_off = true;
            self.set_led(false, actions);
        }
    }

    fn blink_fault(&mut self, actions: &mut Actions) {
        // LED presentation is permitted to differ; it does not mutate motor,
        // fault counters, timer commands or the original ErrorCode.
        let pulses = self.fault.unwrap() as u32 * 400;
        let at = self.state_ms % (200 + pulses + 500);
        self.set_led(
            at >= 200 && at < 200 + pulses && (at - 200) % 400 < 200,
            actions,
        );
    }

    fn update_pwm(&mut self, actions: &mut Actions) {
        self.duty = self.target_percent.wrapping_mul(u32::from(ONE_PERCENT_PWM));
        // UPPWM changes CCR1..4 only; it does not change the low-side GPIOs.
        let mut bridge = Bridge::commutation(self.last_sector, self.duty);
        bridge.low_sides = self.bridge.low_sides;
        actions.pwm_only = true;
        self.set_bridge(bridge, actions);
    }

    fn begin_alignment(&mut self, actions: &mut Actions) {
        let voltage = self.measurements().bus_decivolts;
        if voltage == 0 {
            // C division by zero is undefined. This explicit boundary is not
            // claimed as a matching source behavior.
            self.fault = Some(Fault::InvalidCalibration);
            return;
        }
        let percent = 15 * 105 / voltage;
        self.duty = u32::from(PWM_PERIOD).wrapping_mul(percent) / 100;
        self.startup = StartupState::Aligning;
        self.state_ms = 0;
        self.startup_locked = false; // C StOk=2 during alignment, not success.
        self.sensorless = SensorlessState::Idle;
        self.motor_enabled = true;
        self.sector = 0;
        actions.pre_alignment_pwm = Some((self.last_sector, self.duty));
        self.commutate(10_000, actions);
        let mut bridge = self.bridge;
        bridge.low_sides[1] = true;
        bridge.low_sides[2] = true;
        self.set_bridge(bridge, actions);
    }

    fn advance_startup(&mut self, step_ticks: u16, actions: &mut Actions) {
        match self.startup {
            StartupState::Aligning if self.state_ms >= u32::from(ALIGNMENT_MS) => {
                self.startup = StartupState::Forced;
                self.good_crossings = 0;
                self.crossing_flag = false;
                self.required_samples = 3;
                self.startup_locked = false;
                self.startup_iterations = 0;
                self.sector = 2; // source sets 1, then pre-increments in do loop.
                self.commutate(30_000, actions);
                self.state_ms = 0;
            }
            StartupState::Forced => {
                if self.state_ms < u32::from(FORCED_STEP_TIMEOUT_MS)
                    && !self.crossing_flag
                    && !self.startup_locked
                {
                    return;
                }
                self.startup_iterations += 1;
                self.duty = self.duty.wrapping_add(5);
                if self.startup_locked
                    || self.startup_iterations >= MAX_STARTUP_STEPS
                    || self.fault.is_some()
                {
                    if !self.startup_locked {
                        self.motor_stop(actions);
                        // Sensorless_START returned 0. The caller increments
                        // coun and sets ErrorCode=3 only if no earlier error.
                        if self.fault.is_none() {
                            self.fault = Some(Fault::StartupFailed);
                            self.state = MotorState::Error;
                        }
                    } else {
                        self.required_samples = 2;
                        self.measurement_ms = 0;
                    }
                    self.startup = StartupState::Inactive;
                    // Preserve MotorStartOPEN's unconditional assignment,
                    // even after its callee wrote STATEERROR.
                    self.state = MotorState::RunOpen;
                    self.target_percent = self.duty / u32::from(ONE_PERCENT_PWM);
                    return;
                }
                self.sector = (self.sector + 1) % 6;
                if !self.crossing_flag {
                    self.good_crossings = 0;
                }
                self.crossing_flag = false;
                self.commutate(step_ticks, actions);
                self.state_ms = 0;
            }
            _ => {}
        }
    }

    fn commutate(&mut self, measured_ticks: u16, actions: &mut Actions) {
        if !self.motor_enabled
            || self.state == MotorState::Error
            || self.state == MotorState::ErrorOver
        {
            self.set_bridge(Bridge::off(), actions);
            return;
        }
        self.set_bridge(Bridge::commutation(self.sector, self.duty), actions);
        self.last_sector = self.sector;
        self.step_time = measured_ticks;
        if measured_ticks < 100 {
            self.fault = Some(Fault::TooFast);
        }
        actions.step_timer_preset = Some(0);
        actions.sensorless_timer = TimerCommand::Arm(measured_ticks >> 3);
        self.sensorless = SensorlessState::Demagnetizing;
        // ADCS_chuli's static cou is NOT reset by C Commutation.
        self.commutations_in_window = self.commutations_in_window.wrapping_add(1);
    }

    pub fn on_sensorless_timer(&mut self, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        match self.sensorless {
            SensorlessState::Demagnetizing => {
                self.sensorless = SensorlessState::Detecting;
                actions.sensorless_timer = TimerCommand::Stop;
            }
            SensorlessState::WaitingCommutation if self.startup_locked => {
                actions.sensorless_timer = TimerCommand::Stop;
                self.sector = (self.sector + 1) % 6;
                self.commutate(step_ticks, &mut actions);
            }
            _ => {} // source leaves the periodic timer running in this case.
        }
        actions
    }

    pub fn on_adc1(&mut self, sample: Adc1Sample, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        self.adc1 = sample;
        self.adc1_age_ms = 0;
        if !self.motor_enabled
            || self.fault.is_some()
            || self.sensorless != SensorlessState::Detecting
        {
            return actions;
        }
        if !crossing_side_matches(self.sector, sample, self.adc2.bus_voltage) {
            self.crossing_samples = 0;
            return actions;
        }
        self.crossing_samples = self.crossing_samples.wrapping_add(1);
        if self.crossing_samples < self.required_samples {
            return actions;
        }
        self.crossing_samples = 0;
        self.sensorless = SensorlessState::WaitingCommutation;
        self.good_crossings = self.good_crossings.wrapping_add(1);
        self.crossing_flag = true;
        // C StOk=2 prevents success during alignment, but does not suppress
        // ADCS_chuli or its static filter while aligning.
        if self.good_crossings >= u32::from(STARTUP_CROSSINGS)
            && self.startup != StartupState::Aligning
        {
            self.startup_locked = true;
        }
        if self.startup_locked {
            if self.step_time > 2000 {
                actions.sensorless_timer = TimerCommand::Arm(self.step_time >> 3);
            } else {
                self.required_samples = 1;
                self.sector = (self.sector + 1) % 6;
                self.commutate(step_ticks, &mut actions);
            }
        }
        // The original foreground consumes FFlag; the ADC ISR does not run
        // the forced-start loop or increment OutPwmValue itself.
        actions
    }
}
