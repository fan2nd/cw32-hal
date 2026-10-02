#![deny(unsafe_code)]
//! Port of `MOTOR.C`, `sensorless.c`, `control.c`, and the BTIM1/main loop.
//!
//! Timing units matter: BTIM2/3 count at **8 MHz**, PWM compares use a 4800-count
//! period, and [`MotorController::tick_1ms`] runs at 1 kHz. `StepTime >> 3` is
//! preserved from the source (one eighth of the previous commutation interval),
//! rather than silently replacing it with a textbook 30-degree delay.
//!
//! Deliberate safety fixes: duty is bounded to the PWM period; calibration and
//! zero bus voltage are checked; a failed start cannot overwrite the fault
//! state; cancelling a delayed start cannot fall through into startup; faults
//! shut the bridge off in the same event; stale timer events cannot restart a
//! stopped motor; and the key hold counter cannot wrap into another press.
//! ADC1/ADC2 freshness deadlines (2ms/20ms) also fail closed while enabled.
//! Busy waits are represented by states so protection and the key stay live.

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
pub const ADC1_STALE_MS: u16 = 2;
pub const ADC2_STALE_MS: u16 = 20;

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
    pub pwm_counts: [u16; 3],
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

    /// The source's five-millisecond, all-low bootstrap charge. Applying and
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
    pub const fn commutation(sector: u8, duty: u16) -> Self {
        let duty = if duty > PWM_PERIOD { PWM_PERIOD } else { duty };
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
    pub const fn alignment(duty: u16) -> Self {
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
    target_percent: u8,
    duty: u16,
    bridge: Bridge,
    sector: u8,
    step_time: u16,
    crossing_samples: u8,
    required_samples: u8,
    good_crossings: u16,
    startup_iterations: u16,
    startup_locked: bool,
    motor_enabled: bool,
    key_hold_ms: u16,
    adc2_ms: u8,
    state_ms: u16,
    ramp_ms: u16,
    measurement_ms: u16,
    idle_ms: u16,
    commutations_in_window: u32,
    reported_speed: u32,
    stall_windows: u8,
    led_on: bool,
    fault_blink_ms: u16,
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
            step_time: 0,
            crossing_samples: 0,
            required_samples: 2,
            good_crossings: 0,
            startup_iterations: 0,
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
            fault_blink_ms: 0,
        }
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
    pub const fn duty(&self) -> u16 {
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
    pub const fn good_crossings(&self) -> u16 {
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

    fn disable(&mut self, actions: &mut Actions) {
        self.motor_enabled = false;
        self.startup = StartupState::Inactive;
        self.sensorless = SensorlessState::Idle;
        self.startup_locked = false;
        self.crossing_samples = 0;
        self.duty = 0;
        self.target_percent = 0;
        actions.sensorless_timer = TimerCommand::Stop;
        self.set_bridge(Bridge::off(), actions);
    }

    fn trip(&mut self, fault: Fault, actions: &mut Actions) {
        if self.fault.is_none() {
            self.fault = Some(fault);
            self.fault_blink_ms = 0;
            self.state_ms = 0;
        }
        self.state = MotorState::ErrorOver;
        self.disable(actions);
        self.set_led(false, actions);
    }

    /// External fault input (for an adapter with a real hardware break source).
    /// The C's HardFAULTAD/NUMtimes constants are unused, so ADC1 current is not
    /// silently treated as a calibrated instantaneous current trip here.
    pub fn report_fault(&mut self, fault: Fault) -> Actions {
        let mut actions = Actions::default();
        self.trip(fault, &mut actions);
        actions
    }

    fn key_pressed(&mut self, actions: &mut Actions) {
        self.idle_ms = 0;
        // A stale-sampling fault is not acknowledged by a key press until both
        // streams have recovered. Freshness recovery itself never clears it.
        if self.fault == Some(Fault::AdcStale) && !self.adc_streams_fresh() {
            return;
        }
        if self.powered_off {
            self.powered_off = false;
            self.fault = None;
            self.protection.reset();
            self.set_speed = 0;
            self.speed_level = 0;
            self.state = MotorState::StartCheck;
            self.state_ms = 0;
            self.measurement_ms = 0;
            self.commutations_in_window = 0;
            self.stall_windows = 0;
            self.set_led(true, actions);
        } else if self.fault.is_some() {
            self.fault = None;
            self.protection.reset();
            self.set_speed = 0;
            self.speed_level = 0;
            self.state = MotorState::WaitStart;
            self.state_ms = 0;
            self.stall_windows = 0;
            self.set_led(true, actions);
        } else {
            self.speed_level = (self.speed_level + 1) % 6;
            self.set_speed = self.speed_level * 20;
        }
    }

    /// Equivalent of BTIM1 interrupt plus one nonblocking foreground iteration.
    /// `step_ticks` is a snapshot of BTIM2 CNT, not a microsecond value.
    pub fn tick_1ms(&mut self, key_pressed: bool, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        self.adc1_age_ms = self.adc1_age_ms.saturating_add(1);
        self.adc2_age_ms = self.adc2_age_ms.saturating_add(1);
        if key_pressed {
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

        if self.motor_enabled && !self.adc_streams_fresh() {
            self.trip(Fault::AdcStale, &mut actions);
        }

        self.state_ms = self.state_ms.saturating_add(1);
        self.ramp_ms = self.ramp_ms.saturating_add(1);
        self.measurement_ms += 1;
        // Like the C, the idle counter accumulates only while stopped/faulted
        // and is reset by a key press, rather than by positive speed commands.
        if self.set_speed == 0 || self.fault.is_some() {
            self.idle_ms = self.idle_ms.saturating_add(1);
        }

        if self.measurement_ms >= MEASUREMENT_INTERVAL_MS {
            self.measurement_ms = 0;
            self.reported_speed = self.commutations_in_window.saturating_mul(100);
            self.commutations_in_window = 0;
            if self.adc2_ready {
                if let Err(fault) = self.protection.sample(self.adc2) {
                    if self.fault.is_none() {
                        self.trip(fault, &mut actions);
                    }
                }
            }
            if self.reported_speed < 30 && self.state == MotorState::RunOpen {
                self.stall_windows = self.stall_windows.saturating_add(1);
                if self.stall_windows >= 50 {
                    self.stall_windows = 0;
                    self.trip(Fault::Stall, &mut actions);
                }
            } else {
                self.stall_windows = 0;
            }
        }

        if self.idle_ms >= AUTO_POWER_OFF_MS {
            self.disable(&mut actions);
            self.powered_off = true;
            self.set_led(false, &mut actions);
            return actions;
        }

        if self.fault.is_some() {
            self.blink_fault(&mut actions);
        } else {
            self.advance_state(step_ticks, &mut actions);
        }
        actions
    }

    fn blink_fault(&mut self, actions: &mut Actions) {
        // Initial 200ms off, N pulses (200ms on/200ms off), then 500ms off.
        let pulse_ms = self.fault.unwrap() as u16 * 400;
        let cycle_ms = 200 + pulse_ms + 500;
        self.fault_blink_ms = (self.fault_blink_ms + 1) % cycle_ms;
        let at = self.fault_blink_ms;
        let on = at >= 200 && at < 200 + pulse_ms && (at - 200) % 400 < 200;
        self.set_led(on, actions);
    }

    fn advance_state(&mut self, step_ticks: u16, actions: &mut Actions) {
        match self.state {
            MotorState::StartCheck => {
                if self.set_speed > 0 {
                    self.state = MotorState::StartDelay;
                    self.state_ms = 0;
                    self.target_percent = 0;
                }
            }
            MotorState::StartDelay => {
                if self.set_speed == 0 {
                    self.begin_stop(actions);
                } else if self.state_ms >= START_DELAY_MS {
                    if !self.adc2_ready {
                        self.trip(Fault::InvalidCalibration, actions);
                        return;
                    }
                    self.protection.set_current_offset(self.adc2.current);
                    self.begin_alignment(actions);
                }
            }
            MotorState::StartOpen => {
                if self.set_speed == 0 {
                    self.begin_stop(actions);
                    return;
                }
                match self.startup {
                    StartupState::Aligning if self.state_ms >= ALIGNMENT_MS => {
                        self.startup = StartupState::Forced;
                        self.good_crossings = 0;
                        self.required_samples = 3;
                        self.startup_iterations = 0;
                        // The C writes CNT=30000, sets sector=1, then increments
                        // before commutation: its first forced sector is 2.
                        self.sector = 2;
                        self.state_ms = 0;
                        self.commutate(30_000, actions);
                    }
                    StartupState::Forced if self.state_ms >= FORCED_STEP_TIMEOUT_MS => {
                        self.finish_forced_step(false, step_ticks, actions);
                    }
                    _ => {}
                }
            }
            MotorState::RunOpen => {
                if self.set_speed == 0 {
                    self.begin_stop(actions);
                } else if self.ramp_ms >= RAMP_FULL_SCALE_MS / 100 {
                    self.ramp_ms = 0;
                    if self.target_percent < self.set_speed {
                        self.target_percent += 1;
                    } else if self.target_percent > self.set_speed {
                        self.target_percent = self.set_speed;
                    } else {
                        return;
                    }
                    self.duty = self.target_percent as u16 * ONE_PERCENT_PWM;
                    self.set_bridge(Bridge::commutation(self.sector, self.duty), actions);
                }
            }
            MotorState::Stop => {
                // RealS1 is never updated in the attached C; the source's stop
                // path therefore always takes this 500ms settling delay.
                if self.state_ms >= STOP_SETTLE_MS {
                    self.state = MotorState::StartCheck;
                    self.state_ms = 0;
                }
            }
            MotorState::WaitStart => {
                if self.set_speed == 0 {
                    self.state = MotorState::StartCheck;
                    self.state_ms = 0;
                }
            }
            MotorState::Error | MotorState::ErrorOver => {}
        }
    }

    fn begin_alignment(&mut self, actions: &mut Actions) {
        if !self.adc_streams_fresh() {
            self.trip(Fault::AdcStale, actions);
            return;
        }
        let voltage = self.measurements().bus_decivolts;
        if voltage == 0 || self.adc2.reference == 0 {
            self.trip(Fault::InvalidCalibration, actions);
            return;
        }
        // QDPwm = 15*105/CanshuV; then PWM_PERIOD*QDPwm/100.
        let percent = (15 * 105 / voltage).min(100);
        self.duty = (u32::from(PWM_PERIOD) * percent / 100) as u16;
        self.state = MotorState::StartOpen;
        self.startup = StartupState::Aligning;
        self.state_ms = 0;
        self.startup_locked = false;
        self.sector = 0;
        self.motor_enabled = true;
        self.commutate(10_000, actions);
        self.set_bridge(Bridge::alignment(self.duty), actions);
    }

    fn begin_stop(&mut self, actions: &mut Actions) {
        self.disable(actions);
        self.state = MotorState::Stop;
        self.state_ms = 0;
    }

    fn adc_streams_fresh(&self) -> bool {
        self.adc1_age_ms < ADC1_STALE_MS && self.adc2_age_ms < ADC2_STALE_MS
    }

    fn commutate(&mut self, measured_ticks: u16, actions: &mut Actions) {
        if !self.motor_enabled || self.fault.is_some() || self.powered_off {
            self.disable(actions);
            return;
        }
        if measured_ticks < 100 {
            self.trip(Fault::TooFast, actions);
            return;
        }
        self.step_time = measured_ticks;
        self.sensorless = SensorlessState::Demagnetizing;
        self.crossing_samples = 0;
        self.set_bridge(Bridge::commutation(self.sector, self.duty), actions);
        actions.step_timer_preset = Some(0);
        actions.sensorless_timer = TimerCommand::Arm(measured_ticks >> 3);
        self.commutations_in_window = self.commutations_in_window.saturating_add(1);
    }

    fn finish_forced_step(&mut self, crossed: bool, step_ticks: u16, actions: &mut Actions) {
        self.startup_iterations += 1;
        self.duty = self.duty.saturating_add(5).min(PWM_PERIOD);
        if !crossed {
            self.good_crossings = 0;
        }
        if self.startup_locked {
            self.required_samples = 2;
            self.startup = StartupState::Inactive;
            self.state = MotorState::RunOpen;
            self.target_percent = (self.duty / ONE_PERCENT_PWM) as u8;
            self.measurement_ms = 0;
            self.state_ms = 0;
            return;
        }
        if self.startup_iterations >= MAX_STARTUP_STEPS {
            self.trip(Fault::StartupFailed, actions);
            return;
        }
        self.sector = (self.sector + 1) % 6;
        self.state_ms = 0;
        self.commutate(step_ticks, actions);
    }

    /// BTIM3 update event: first end demagnetization, then (after a accepted
    /// low-speed crossing) perform the delayed next commutation.
    pub fn on_sensorless_timer(&mut self, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        if !self.motor_enabled || self.fault.is_some() || self.powered_off {
            actions.sensorless_timer = TimerCommand::Stop;
            return actions;
        }
        match self.sensorless {
            SensorlessState::Demagnetizing => {
                self.sensorless = SensorlessState::Detecting;
                actions.sensorless_timer = TimerCommand::Stop;
            }
            SensorlessState::WaitingCommutation if self.startup_locked => {
                self.sector = (self.sector + 1) % 6;
                self.commutate(step_ticks, &mut actions);
            }
            _ => {
                actions.sensorless_timer = TimerCommand::Stop;
            }
        }
        actions
    }

    /// ADC1 EOS event. Qualification must consist of consecutive samples
    /// strictly on the expected side of ADC2 bus/2; equality breaks a run.
    pub fn on_adc1(&mut self, sample: Adc1Sample, step_ticks: u16) -> Actions {
        let mut actions = Actions::default();
        self.adc1 = sample;
        self.adc1_age_ms = 0;
        if !self.motor_enabled
            || self.fault.is_some()
            || self.powered_off
            || self.sensorless != SensorlessState::Detecting
            || self.startup == StartupState::Aligning
        {
            return actions;
        }
        let valid = crossing_side_matches(self.sector, sample, self.adc2.bus_voltage);
        if !valid {
            self.crossing_samples = 0;
            return actions;
        }
        self.crossing_samples += 1;
        if self.crossing_samples < self.required_samples {
            return actions;
        }
        self.crossing_samples = 0;
        self.sensorless = SensorlessState::WaitingCommutation;
        self.good_crossings = self.good_crossings.saturating_add(1);
        if self.good_crossings >= STARTUP_CROSSINGS {
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
        if self.startup == StartupState::Forced && self.fault.is_none() {
            self.finish_forced_step(true, step_ticks, &mut actions);
        }
        actions
    }
}
