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
        // Integer quadratic curve: zero stays at zero, full scale reaches the
        // rated speed, and the low end has finer adjustment than the C linear
        // command. All intermediates fit in i32 for the 12-bit ADC range.
        let shaped = command.saturating_mul(command) / span;
        min_rpm + shaped * (max_rpm - min_rpm) / span
    }
}
