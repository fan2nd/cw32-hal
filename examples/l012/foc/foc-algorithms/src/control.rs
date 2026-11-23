//! Small state and command helpers kept separate from the hardware loop.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotorState {
    WaitingForZero,
    Aligning,
    ProbingDirection,
    Running,
    Fault,
}

impl MotorState {
    pub const fn name(self) -> &'static str {
        match self {
            Self::WaitingForZero => "wait-zero",
            Self::Aligning => "align",
            Self::ProbingDirection => "direction-probe",
            Self::Running => "run",
            Self::Fault => "fault",
        }
    }
}

pub struct ArmingGate {
    state: MotorState,
    zero_samples: u32,
    required_zero_samples: u32,
}

impl ArmingGate {
    pub const fn new(required_zero_samples: u32) -> Self {
        Self {
            state: MotorState::WaitingForZero,
            zero_samples: 0,
            required_zero_samples,
        }
    }

    pub fn state(&self) -> MotorState {
        self.state
    }

    pub fn observe(&mut self, pot: u16, zero_limit: u16) -> bool {
        if self.state != MotorState::WaitingForZero {
            return true;
        }
        if pot <= zero_limit {
            self.zero_samples = self.zero_samples.saturating_add(1);
            if self.zero_samples >= self.required_zero_samples {
                self.state = MotorState::Aligning;
                return true;
            }
        } else {
            self.zero_samples = 0;
        }
        false
    }

    pub fn set_state(&mut self, state: MotorState) {
        self.state = state;
    }
}

/// Complete a requested stop before accepting a fresh potentiometer start.
#[derive(Default)]
pub struct StopGate {
    stopping: bool,
    start_us: u32,
}

impl StopGate {
    pub fn command(&mut self, request: i32, active: bool, at_zero: bool, elapsed_us: u32) -> i32 {
        if active && request == 0 {
            self.stopping = true;
        }
        if self.stopping {
            if !active && at_zero {
                self.stopping = false;
            }
            self.start_us = 0;
            return 0;
        }
        if active || request == 0 {
            self.start_us = 0;
            return request;
        }
        self.start_us = self.start_us.saturating_add(elapsed_us);
        if self.start_us >= 20_000 { request } else { 0 }
    }
}

pub fn target_rpm_from_pot(
    pot: u16,
    zero_limit: u16,
    min_counts: u16,
    max_counts: u16,
    min_rpm: i32,
    max_rpm: i32,
) -> i32 {
    if pot <= zero_limit {
        0
    } else if pot <= min_counts {
        0
    } else {
        let span = i32::from(max_counts - min_counts);
        let command = i32::from(pot.min(max_counts) - min_counts);
        // Power-shaped curve with a small linear blend. The quadratic part
        // gives useful resolution at the low end; the blend coefficient keeps
        // the first nonzero command from jumping while preserving the exact
        // 0..max endpoint. All intermediates fit in i32 for the 12-bit ADC.
        const LINEAR_BLEND_Q10: i32 = 256;
        let quadratic = command.saturating_mul(command) / span;
        let shaped = quadratic
            .saturating_mul(1_024 - LINEAR_BLEND_Q10)
            .saturating_add(command.saturating_mul(LINEAR_BLEND_Q10))
            / 1_024;
        min_rpm + shaped * (max_rpm - min_rpm) / span
    }
}

/// A smooth q-axis bias for static friction and load torque.
///
/// The speed PI supplies only the dynamic correction. This bias is largest at
/// the bottom of the command range and fades continuously to zero at the top,
/// so there is no separate low-speed control mode or threshold.
pub fn holding_torque_reference(target_rpm: i32, max_rpm: i32) -> i16 {
    if target_rpm <= 0 || max_rpm <= 0 {
        return 0;
    }
    let target = target_rpm.min(max_rpm);
    let remaining = max_rpm - target;
    // The squared fade keeps the added holding torque small once the motor is
    // already moving quickly. All values fit comfortably in i32.
    const HOLDING_TORQUE_MAX_Q15: i32 = 300;
    let denominator = max_rpm.saturating_mul(max_rpm);
    if denominator == 0 {
        return 0;
    }
    (HOLDING_TORQUE_MAX_Q15
        .saturating_mul(remaining)
        .saturating_mul(remaining)
        / denominator)
        .clamp(0, i32::from(i16::MAX)) as i16
}

/// Combine the static holding torque with the speed-loop correction.
///
/// Holding torque is only a forward feed-forward term while the rotor is at
/// or below the requested speed. Once the measured speed is above the target,
/// the speed PI must be allowed to apply a signed braking torque without a
/// positive bias fighting it.
pub fn combined_torque_reference(
    speed_iq: i16,
    holding_iq: i16,
    target_rpm: i32,
    feedback_rpm: i32,
) -> i16 {
    // Holding torque is feed-forward for static friction. Scale it by the
    // remaining speed error so it helps a stalled rotor, then fades out as
    // the measured speed reaches the target. It is removed on overspeed so a
    // negative speed-loop result reaches the current loop.
    let holding_nominal = i32::from(holding_iq).max(0);
    let holding = if target_rpm > 0 && feedback_rpm <= target_rpm {
        let remaining = target_rpm.saturating_sub(feedback_rpm).clamp(0, target_rpm);
        // Across the bottom 16 rpm, taper breakaway assistance instead of
        // demanding the full bias at standstill for even a 1 rpm command.
        let span = target_rpm.max(16);
        let denominator = span.saturating_mul(span);
        if denominator == 0 {
            0
        } else {
            holding_nominal
                .saturating_mul(remaining)
                .saturating_mul(remaining)
                / denominator
        }
    } else {
        0
    };
    i32::from(speed_iq)
        .saturating_add(holding)
        .clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16
}

#[cfg(test)]
mod stop_tests {
    use super::StopGate;

    #[test]
    fn stop_requires_bridge_off_zero_and_a_sustained_new_command() {
        let mut gate = StopGate::default();
        assert_eq!(gate.command(0, true, true, 500), 0);
        assert_eq!(gate.command(123, true, false, 500), 0);
        assert_eq!(gate.command(123, false, false, 30_000), 0);
        assert_eq!(gate.command(0, false, true, 500), 0);
        assert_eq!(gate.command(1, false, false, 500), 0);
        assert_eq!(gate.command(0, false, true, 500), 0);
        for _ in 0..39 {
            assert_eq!(gate.command(1, false, false, 500), 0);
        }
        assert_eq!(gate.command(1, false, false, 500), 1);
        assert_eq!(gate.command(123, true, false, 500), 123);
    }
}
