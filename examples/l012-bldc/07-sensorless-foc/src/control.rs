#![deny(unsafe_code)]
//! 8 kHz 定点无感 FOC：三相电流 → Clarke/Park → Id/Iq PI → 电压矢量。
//! 输入电压是两个电流保持时刻间按实际 PWM 开关状态积分的平均 αβ 电压。
//! 板级将首样本运输到第二保持时刻，并传入真实 dt，不能回填未限幅指令。
//! R/L 电压模型假定两次电流为同一时刻的相电流；错位采样及 PWM 纹波是误差源。
//! CCR 重建不包含死区、管压降或 ADC 延迟误差，观测器不能代替实板标定。

use crate::{arithmetic::Arithmetic, config::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum State {
    Calibrating,
    Off,
    Bootstrap,
    Align,
    OpenLoop,
    Blend,
    ClosedLoop,
    Fault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Fault {
    None,
    Parameters,
    Calibration,
    SampleInvalid,
    SampleTimeout,
    OverCurrent,
    UnderVoltage,
    OverVoltage,
    ObserverLost,
    StartupTimeout,
    Stall,
    Overspeed,
    Driver,
    Timing,
    Arithmetic,
}

#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub state: State,
    pub fault: Fault,
    pub phase_ma: [i32; 3],
    pub current_dq_ma: [i32; 2],
    pub target_dq_ma: [i32; 2],
    pub voltage_dq_mv: [i32; 2],
    pub bus_mv: i32,
    pub angle: u16,
    pub observer_angle: u16,
    pub speed_millihz: i32,
    pub bemf_mv: i32,
    pub pll_error: i32,
    pub qualified_frames: u32,
}

pub struct Control {
    state: State,
    fault: Fault,
    requested: bool,
    reset_pending: bool,
    age: u32,
    good: u32,
    bad: u32,
    stall: u32,
    open_phase: u32,
    blend_offset: i32,
    blend_d_reference: i32,
    speed_reference: i32,
    iq_reference: i32,
    d_pi: Pi,
    q_pi: Pi,
    speed_pi: Pi,
    observer: Observer,
    snapshot: Snapshot,
}

impl Control {
    pub const fn new() -> Self {
        Self {
            state: State::Calibrating,
            fault: Fault::None,
            requested: false,
            reset_pending: false,
            age: 0,
            good: 0,
            bad: 0,
            stall: 0,
            open_phase: 0,
            blend_offset: 0,
            blend_d_reference: 0,
            speed_reference: 0,
            iq_reference: 0,
            d_pi: Pi::new(CURRENT_KP_Q15, CURRENT_KI_Q15),
            q_pi: Pi::new(CURRENT_KP_Q15, CURRENT_KI_Q15),
            speed_pi: Pi::new(SPEED_KP_Q15, SPEED_KI_Q15),
            observer: Observer::new(),
            snapshot: Snapshot {
                state: State::Calibrating,
                fault: Fault::None,
                phase_ma: [0; 3],
                current_dq_ma: [0; 2],
                target_dq_ma: [0; 2],
                voltage_dq_mv: [0; 2],
                bus_mv: 0,
                angle: 0,
                observer_angle: 0,
                speed_millihz: 0,
                bemf_mv: 0,
                pll_error: 0,
                qualified_frames: 0,
            },
        }
    }

    /// 仅在板级零点及噪声/范围校准通过后调用；校准结束不会自动启动。
    pub fn finish_calibration(&mut self) {
        if self.state == State::Calibrating {
            if valid() {
                self.enter(State::Off);
            } else {
                self.trip(Fault::Parameters);
            }
        }
    }

    /// 停止优先；故障锁存到复位，持续为 true 也不会重新启动。
    /// reload 只切换状态并标记复位；PI/观测器批量清零留给第二样本后的控制时隙。
    pub fn request_run(&mut self, run: bool) {
        let rising = run && !self.requested;
        self.requested = run;
        if !run && self.energized() {
            self.reset_pending = true;
            self.enter(State::Off);
        } else if rising && self.state == State::Off {
            self.reset_pending = true;
            self.enter(State::Bootstrap);
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn estimated_emf_mv(&self) -> [i32; 2] {
        [self.observer.emf_q8[0] >> 8, self.observer.emf_q8[1] >> 8]
    }

    pub fn fault(&self) -> Fault {
        self.fault
    }

    pub fn snapshot(&self) -> Snapshot {
        let mut s = self.snapshot;
        s.state = self.state;
        s.fault = self.fault;
        // 命令边沿之后、下一帧计算之前也报告复位后的命令/估计量；
        // 诊断按需合成，不把整块清零搬回 reload 的紧时隙。
        if self.reset_pending {
            s.target_dq_ma = [0; 2];
            s.voltage_dq_mv = [0; 2];
            s.observer_angle = 0;
            s.speed_millihz = 0;
            s.bemf_mv = 0;
            s.pll_error = 0;
            s.qualified_frames = 0;
        } else {
            s.observer_angle = self.observer.angle();
            s.speed_millihz = speed_millihz(self.observer.speed);
            s.bemf_mv = self.observer.magnitude;
            s.pll_error = self.observer.error;
            s.qualified_frames = self.good;
        }
        s
    }

    pub fn trip(&mut self, fault: Fault) {
        if self.state != State::Fault {
            self.fault = if fault == Fault::None {
                Fault::Driver
            } else {
                fault
            };
            self.enter(State::Fault);
            self.d_pi.integral = 0;
            self.q_pi.integral = 0;
            self.snapshot.target_dq_ma = [0; 2];
            self.snapshot.voltage_dq_mv = [0; 2];
        }
    }

    /// 每个完整采样帧恰好一次；丢帧/超时由板级另行 trip，不得补算虚构帧。
    pub fn on_frame_timed(
        &mut self,
        phase_ma: [i32; 3],
        actual_voltage_mv: [i32; 2],
        bus_mv: i32,
        sample_valid: bool,
        dt_ticks: u16,
        math: &mut impl Arithmetic,
    ) -> [i32; 2] {
        // 必须先复位再消费任何控制输入；启动时首个物理帧仍是 Off，
        // 新 Bootstrap 计划仅在本次计算之后产生，下一次真实 reload 才能预载。
        if self.reset_pending {
            self.clear_dynamic();
            self.reset_pending = false;
        }
        self.snapshot.bus_mv = bus_mv;
        self.snapshot.phase_ma = phase_ma;
        if !self.energized() {
            return [0; 2];
        }
        let fault = if !sample_valid {
            Fault::SampleInvalid
        } else if !(PWM_TICKS / 2..=PWM_TICKS * 3 / 2).contains(&dt_ticks) {
            Fault::Timing
        } else if phase_ma
            .iter()
            .any(|&i| !(-CURRENT_TRIP_MA..=CURRENT_TRIP_MA).contains(&i))
        {
            Fault::OverCurrent
        } else if bus_mv > BUS_MAX_MV {
            Fault::OverVoltage
        } else if bus_mv < BUS_MIN_MV {
            Fault::UnderVoltage
        } else if (phase_ma[0] + phase_ma[1] + phase_ma[2]).abs() > CURRENT_SUM_LIMIT_MA
            || actual_voltage_mv
                .iter()
                .any(|&v| !(-BUS_MAX_MV..=BUS_MAX_MV).contains(&v))
        {
            Fault::SampleInvalid
        } else {
            Fault::None
        };
        if fault != Fault::None {
            self.trip(fault);
            return [0; 2];
        }
        let current = clarke(phase_ma, math);
        self.age = self.age.wrapping_add(1);
        if self.state == State::Bootstrap {
            if self.age < BOOTSTRAP_FRAMES {
                return [0; 2];
            }
            self.enter(State::Align);
        }
        if self.state == State::Align && self.age >= ALIGN_FRAMES {
            self.enter(State::OpenLoop);
            self.observer.reset(current);
        }
        if matches!(
            self.state,
            State::OpenLoop | State::Blend | State::ClosedLoop
        ) {
            self.observer
                .update(current, actual_voltage_mv, dt_ticks, math);
        }

        if math.failed() {
            self.trip(Fault::Arithmetic);
            return [0; 2];
        }

        // 开环 I/F 使用旋转 d 轴电流；轻载转子跟随此轴，而不是相差约 90° 的 q 轴。
        let mut target = [STARTUP_CURRENT_MA, 0];
        let angle = match self.state {
            State::Align => {
                target = [ALIGN_ID_MA, 0];
                0
            }
            State::OpenLoop => {
                let ramp = self.age.min(OPEN_RAMP_FRAMES) as i32;
                let hz = OPEN_START_MILLIHZ
                    + math.div(
                        (OPEN_END_MILLIHZ - OPEN_START_MILLIHZ) * ramp,
                        OPEN_RAMP_FRAMES as i32,
                    );
                self.open_phase =
                    self.open_phase
                        .wrapping_add(scale_dt(speed_step(hz, math), dt_ticks, math) as u32);
                if math.failed() {
                    self.trip(Fault::Arithmetic);
                    return [0; 2];
                }
                let angle = (self.open_phase >> 16) as u16;
                let speed_error = (speed_millihz(self.observer.speed) - hz).abs();
                let good = self.observer.qualified()
                    && speed_millihz(self.observer.speed) >= HANDOFF_MIN_MILLIHZ
                    && angle_error(self.observer.angle(), angle).abs() <= HANDOFF_ANGLE_LIMIT
                    && speed_error <= (hz / 4).max(HANDOFF_SPEED_ERROR_MILLIHZ);
                self.good = if good { self.good.saturating_add(1) } else { 0 };
                if self.good >= HANDOFF_GOOD_FRAMES {
                    self.blend_offset = angle_error(angle, self.observer.angle());
                    let (sin, cos) = sin_cos(self.blend_offset as u16);
                    let observed_reference = inverse_park(self.snapshot.target_dq_ma, sin, cos);
                    self.blend_d_reference = observed_reference[0];
                    self.iq_reference = observed_reference[1].clamp(0, RUN_IQ_MAX_MA);
                    self.speed_pi.integral = self.iq_reference << 15;
                    self.speed_reference = speed_millihz(self.observer.speed)
                        .clamp(HANDOFF_MIN_MILLIHZ, RUN_TARGET_MILLIHZ);
                    self.enter(State::Blend);
                } else if self.age >= STARTUP_TIMEOUT_FRAMES {
                    self.trip(Fault::StartupTimeout);
                    return [0; 2];
                }
                angle
            }
            State::Blend | State::ClosedLoop => {
                let speed = speed_millihz(self.observer.speed);
                self.bad = if self.observer.qualified() {
                    0
                } else {
                    self.bad.saturating_add(1)
                };
                self.stall = if speed < STALL_MIN_MILLIHZ {
                    self.stall.saturating_add(1)
                } else {
                    0
                };
                let fault = if speed > OVERSPEED_MILLIHZ {
                    Fault::Overspeed
                } else if self.stall >= STALL_FRAMES {
                    Fault::Stall
                } else if self.bad >= OBSERVER_BAD_FRAMES {
                    Fault::ObserverLost
                } else {
                    Fault::None
                };
                if fault != Fault::None {
                    self.trip(fault);
                    return [0; 2];
                }
                // Blend 立即接入限流速度环，不能在半秒过渡中固定注入满启动转矩。
                if math.rem_unsigned(self.age, SPEED_LOOP_DIVIDER) == 0 {
                    self.speed_reference = slew(
                        self.speed_reference,
                        RUN_TARGET_MILLIHZ,
                        SPEED_REF_SLEW_MILLIHZ,
                    );
                    self.iq_reference =
                        self.speed_pi
                            .scalar(self.speed_reference - speed, 0, RUN_IQ_MAX_MA, math);
                }
                if self.state == State::Blend {
                    let remaining = BLEND_FRAMES.saturating_sub(self.age);
                    let offset =
                        math.div(self.blend_offset * remaining as i32, BLEND_FRAMES as i32);
                    let angle = self.observer.angle().wrapping_add(offset as u16);
                    let observer_target = [
                        math.div(
                            self.blend_d_reference * remaining as i32,
                            BLEND_FRAMES as i32,
                        ),
                        self.iq_reference,
                    ];
                    let (sin, cos) = sin_cos(offset as u16);
                    // 先在真实转子参考系淡出 Id，再转回当前混合参考系，避免电流指令跳变。
                    target = park(observer_target, sin, cos);
                    if remaining == 0 {
                        self.enter(State::ClosedLoop);
                    }
                    angle
                } else {
                    target = [0, self.iq_reference];
                    self.observer.angle()
                }
            }
            _ => return [0; 2],
        };
        self.snapshot.angle = angle;
        let (sin, cos) = sin_cos(angle);
        let dq = park(current, sin, cos);
        self.snapshot.current_dq_ma = dq;
        for (command, desired) in self.snapshot.target_dq_ma.iter_mut().zip(target) {
            *command = slew(*command, desired, CURRENT_SLEW_MA_PER_FRAME);
        }
        let errors = [
            self.snapshot.target_dq_ma[0] - dq[0],
            self.snapshot.target_dq_ma[1] - dq[1],
        ];
        let limit = bus_mv * VOLTAGE_LIMIT_Q15 >> 15;
        let (vd, id) = self.d_pi.propose(errors[0], -limit, limit, dt_ticks, math);
        let (vq, iq) = self.q_pi.propose(errors[1], -limit, limit, dt_ticks, math);
        let bounded = limit_vector([vd, vq], limit, math);
        self.d_pi.commit(errors[0], vd, bounded[0], id);
        self.q_pi.commit(errors[1], vq, bounded[1], iq);
        self.snapshot.voltage_dq_mv = bounded;
        inverse_park(bounded, sin, cos)
    }

    fn energized(&self) -> bool {
        matches!(
            self.state,
            State::Bootstrap | State::Align | State::OpenLoop | State::Blend | State::ClosedLoop
        )
    }

    fn enter(&mut self, state: State) {
        self.state = state;
        self.age = 0;
    }

    fn clear_dynamic(&mut self) {
        self.good = 0;
        self.bad = 0;
        self.stall = 0;
        self.open_phase = 0;
        self.blend_offset = 0;
        self.blend_d_reference = 0;
        self.speed_reference = 0;
        self.iq_reference = 0;
        self.d_pi.integral = 0;
        self.q_pi.integral = 0;
        self.speed_pi.integral = 0;
        self.observer = Observer::new();
        self.snapshot.target_dq_ma = [0; 2];
        self.snapshot.voltage_dq_mv = [0; 2];
    }
}

struct Pi {
    kp: i32,
    ki: i32,
    integral: i32,
}

impl Pi {
    const fn new(kp: i32, ki: i32) -> Self {
        Self {
            kp,
            ki,
            integral: 0,
        }
    }

    fn propose(
        &self,
        error: i32,
        low: i32,
        high: i32,
        dt_ticks: u16,
        math: &mut impl Arithmetic,
    ) -> (i32, i32) {
        let ki = math.div(
            self.ki * i32::from(dt_ticks) + i32::from(PWM_TICKS) / 2,
            i32::from(PWM_TICKS),
        );
        let integral = self
            .integral
            .saturating_add(error * ki)
            .clamp(low << 15, high << 15);
        ((error * self.kp >> 15) + (integral >> 15), integral)
    }

    // 已达到电压圆/电流边界时，只允许积分沿退出饱和的方向变化。
    fn commit(&mut self, error: i32, raw: i32, applied: i32, integral: i32) {
        if raw == applied || (raw > applied && error < 0) || (raw < applied && error > 0) {
            self.integral = integral;
        }
    }

    fn scalar(&mut self, error: i32, low: i32, high: i32, math: &mut impl Arithmetic) -> i32 {
        let (raw, integral) = self.propose(error, low, high, PWM_TICKS, math);
        let applied = raw.clamp(low, high);
        self.commit(error, raw, applied, integral);
        applied
    }
}

struct Observer {
    previous_current: [i32; 2],
    corrected_angle: u16,
    emf_q8: [i32; 2],
    phase: u32,
    speed: i32,
    magnitude: i32,
    error: i32,
    tracking: bool,
}

impl Observer {
    const fn new() -> Self {
        Self {
            previous_current: [0; 2],
            corrected_angle: 0,
            emf_q8: [0; 2],
            phase: 0,
            speed: 0,
            magnitude: 0,
            error: 0,
            tracking: false,
        }
    }

    fn reset(&mut self, current: [i32; 2]) {
        *self = Self::new();
        self.previous_current = current;
    }

    fn update(
        &mut self,
        current: [i32; 2],
        applied: [i32; 2],
        dt_ticks: u16,
        math: &mut impl Arithmetic,
    ) {
        for axis in 0..2 {
            let average = (current[axis] + self.previous_current[axis]) / 2;
            let resistive = math.div(MODEL_PHASE_R_MILLIOHM * average, 1000);
            // μH·ΔmA·96 / ATIMticks = mV；商/余数分解避免 M0+ 的 64 位除法。
            let inductive = mul_div(
                MODEL_PHASE_L_UH * (current[axis] - self.previous_current[axis]),
                (CPU_HZ / 1_000_000) as i32,
                i32::from(dt_ticks),
                math,
            );
            let emf = (applied[axis] - resistive - inductive).clamp(-BUS_MAX_MV, BUS_MAX_MV);
            self.emf_q8[axis] +=
                (((emf << 8) - self.emf_q8[axis]) * BEMF_FILTER_NUMERATOR) >> BEMF_FILTER_SHIFT;
        }
        self.previous_current = current;
        let emf = [self.emf_q8[0] >> 8, self.emf_q8[1] >> 8];
        // 先缩小一位使最大配置下平方和仍在 u32 范围内。
        self.magnitude = (math.sqrt(((emf[0] / 2).pow(2) + (emf[1] / 2).pow(2)) as u32) * 2) as i32;
        let increment = scale_dt(self.speed, dt_ticks, math);
        if self.magnitude < BEMF_MIN_MV / 4 {
            self.phase = self.phase.wrapping_add(increment as u32);
            self.error = 32767;
            self.correct_angle(dt_ticks, math);
            return;
        }
        // 正转时 eα=-E sinθ，eβ=E cosθ；仅测得的反电势可修正 PLL。
        let measured = atan2_angle(-emf[0], emf[1], math);
        if !self.tracking {
            self.phase = (measured as u32) << 16;
            self.tracking = true;
        }
        let predicted = self.phase.wrapping_add(increment as u32);
        self.error = angle_error(measured, (predicted >> 16) as u16);
        self.speed = (self.speed + scale_dt(self.error * PLL_KI_Q16, dt_ticks, math)).clamp(
            -const { speed_step_const(PLL_MAX_MILLIHZ) },
            const { speed_step_const(PLL_MAX_MILLIHZ) },
        );
        self.phase = predicted.wrapping_add((self.error * PLL_KP_Q16) as u32);
        self.correct_angle(dt_ticks, math);
    }

    fn correct_angle(&mut self, dt_ticks: u16, math: &mut impl Arithmetic) {
        // 滤波低频群延迟 + 当前差分区间的半个 dt；校正值只计算一次。
        let advance = mul_div(
            self.speed,
            (1 << BEMF_FILTER_SHIFT) - BEMF_FILTER_NUMERATOR,
            BEMF_FILTER_NUMERATOR,
            math,
        ) + scale_dt(self.speed, dt_ticks, math) / 2;
        self.corrected_angle = (self.phase.wrapping_add(advance as u32) >> 16) as u16;
    }

    fn angle(&self) -> u16 {
        self.corrected_angle
    }

    fn qualified(&self) -> bool {
        self.tracking
            && self.magnitude >= BEMF_MIN_MV
            && self.error.abs() <= PLL_ERROR_LIMIT
            && self.speed > 0
            && self.speed < const { speed_step_const(OVERSPEED_MILLIHZ) }
    }
}

fn clarke(i: [i32; 3], math: &mut impl Arithmetic) -> [i32; 2] {
    [
        math.div(2 * i[0] - i[1] - i[2], 3),
        (i[1] - i[2]) * 18919 >> 15,
    ]
}

fn park(v: [i32; 2], sin: i32, cos: i32) -> [i32; 2] {
    [
        (v[0] * cos + v[1] * sin) >> 15,
        (-v[0] * sin + v[1] * cos) >> 15,
    ]
}

fn inverse_park(v: [i32; 2], sin: i32, cos: i32) -> [i32; 2] {
    [
        (v[0] * cos - v[1] * sin) >> 15,
        (v[0] * sin + v[1] * cos) >> 15,
    ]
}

fn limit_vector(mut v: [i32; 2], limit: i32, math: &mut impl Arithmetic) -> [i32; 2] {
    let maximum = v[0].abs().max(v[1].abs());
    if maximum > limit {
        // 先限制分量，避免平方溢出；仍按同一个系数缩放整个矢量。
        v = [
            math.div(v[0] * limit, maximum),
            math.div(v[1] * limit, maximum),
        ];
    }
    let squared = (v[0] * v[0] + v[1] * v[1]) as u32;
    if squared > (limit * limit) as u32 {
        let magnitude = math.sqrt(squared) as i32 + 1; // 向上取整，限幅不能超圆。
        v = [
            math.div(v[0] * limit, magnitude),
            math.div(v[1] * limit, magnitude),
        ];
    }
    v
}

fn angle_error(a: u16, b: u16) -> i32 {
    a.wrapping_sub(b) as i16 as i32
}

fn slew(value: i32, target: i32, amount: i32) -> i32 {
    value + (target - value).clamp(-amount, amount)
}

// 定点整数运算；没有浮点或运行时三角函数。
fn scale_dt(value: i32, dt_ticks: u16, math: &mut impl Arithmetic) -> i32 {
    mul_div(value, i32::from(dt_ticks), i32::from(PWM_TICKS), math)
}

// divisor > 0；有效参数保证两个小乘积不溢出，正负输入均等于向零截断。
fn mul_div(value: i32, multiplier: i32, divisor: i32, math: &mut impl Arithmetic) -> i32 {
    let (quotient, remainder) = math.div_rem(value, divisor);
    quotient * multiplier + math.div(remainder * multiplier, divisor)
}

const fn speed_step_const(millihz: i32) -> i32 {
    // 8 kHz：2^32/8e6 = 536 + 13608/15625；|mHz| <= 100000 时
    // 最大分数乘积 1,360,800,000，避免 M0+ 的 64 位除法并保持精确截断。
    let magnitude = millihz.unsigned_abs();
    let step = magnitude * 536 + magnitude * 13_608 / 15_625;
    if millihz < 0 {
        -(step as i32)
    } else {
        step as i32
    }
}

fn speed_millihz(step: i32) -> i32 {
    ((step as i64 * (CONTROL_HZ as i64 * 1000)) >> 32) as i32
}

fn speed_step(millihz: i32, math: &mut impl Arithmetic) -> i32 {
    let magnitude = millihz.abs();
    let step = magnitude * 536 + math.div(magnitude * 13_608, 15_625);
    if millihz < 0 {
        -step
    } else {
        step
    }
}

fn sin_cos(angle: u16) -> (i32, i32) {
    (sin_q15(angle), sin_q15(angle.wrapping_add(16384)))
}

fn sin_q15(angle: u16) -> i32 {
    let quadrant = angle >> 14;
    let offset = (angle & 16383) as usize;
    let x = if quadrant & 1 == 0 {
        offset
    } else {
        16384 - offset
    };
    let index = x >> 6;
    let mut value = SIN_QUARTER[index] as i32;
    if index < 256 {
        value += (SIN_QUARTER[index + 1] as i32 - value) * (x as i32 & 63) >> 6;
    }
    if quadrant >= 2 {
        -value
    } else {
        value
    }
}

fn atan2_angle(y: i32, x: i32, math: &mut impl Arithmetic) -> u16 {
    let ax = x.abs();
    let ay = y.abs();
    let maximum = ax.max(ay);
    if maximum == 0 {
        return 0;
    }
    let ratio = math.div(ax.min(ay) * 32768, maximum);
    let index = (ratio >> 8) as usize;
    let mut angle = ATAN_OCTANT[index] as i32;
    if index < 128 {
        angle += (ATAN_OCTANT[index + 1] as i32 - angle) * (ratio & 255) >> 8;
    }
    if ay > ax {
        angle = 16384 - angle;
    }
    if x < 0 {
        angle = 32768 - angle;
    }
    if y < 0 {
        angle = -angle;
    }
    angle as u16
}

// sin(πi/512)·32767 与 atan(i/128)·65536/(2π)，ROM 查表线性插值。
const SIN_QUARTER: [i16; 257] = [
    0, 201, 402, 603, 804, 1005, 1206, 1407, 1608, 1809, 2009, 2210, 2410, 2611, 2811, 3012, 3212,
    3412, 3612, 3811, 4011, 4210, 4410, 4609, 4808, 5007, 5205, 5404, 5602, 5800, 5998, 6195, 6393,
    6590, 6786, 6983, 7179, 7375, 7571, 7767, 7962, 8157, 8351, 8545, 8739, 8933, 9126, 9319, 9512,
    9704, 9896, 10087, 10278, 10469, 10659, 10849, 11039, 11228, 11417, 11605, 11793, 11980, 12167,
    12353, 12539, 12725, 12910, 13094, 13279, 13462, 13645, 13828, 14010, 14191, 14372, 14553,
    14732, 14912, 15090, 15269, 15446, 15623, 15800, 15976, 16151, 16325, 16499, 16673, 16846,
    17018, 17189, 17360, 17530, 17700, 17869, 18037, 18204, 18371, 18537, 18703, 18868, 19032,
    19195, 19357, 19519, 19680, 19841, 20000, 20159, 20317, 20475, 20631, 20787, 20942, 21096,
    21250, 21403, 21554, 21705, 21856, 22005, 22154, 22301, 22448, 22594, 22739, 22884, 23027,
    23170, 23311, 23452, 23592, 23731, 23870, 24007, 24143, 24279, 24413, 24547, 24680, 24811,
    24942, 25072, 25201, 25329, 25456, 25582, 25708, 25832, 25955, 26077, 26198, 26319, 26438,
    26556, 26674, 26790, 26905, 27019, 27133, 27245, 27356, 27466, 27575, 27683, 27790, 27896,
    28001, 28105, 28208, 28310, 28411, 28510, 28609, 28706, 28803, 28898, 28992, 29085, 29177,
    29268, 29358, 29447, 29534, 29621, 29706, 29791, 29874, 29956, 30037, 30117, 30195, 30273,
    30349, 30424, 30498, 30571, 30643, 30714, 30783, 30852, 30919, 30985, 31050, 31113, 31176,
    31237, 31297, 31356, 31414, 31470, 31526, 31580, 31633, 31685, 31736, 31785, 31833, 31880,
    31926, 31971, 32014, 32057, 32098, 32137, 32176, 32213, 32250, 32285, 32318, 32351, 32382,
    32412, 32441, 32469, 32495, 32521, 32545, 32567, 32589, 32609, 32628, 32646, 32663, 32678,
    32692, 32705, 32717, 32728, 32737, 32745, 32752, 32757, 32761, 32765, 32766, 32767,
];
const ATAN_OCTANT: [i16; 129] = [
    0, 81, 163, 244, 326, 407, 489, 570, 651, 732, 813, 894, 975, 1056, 1136, 1217, 1297, 1377,
    1457, 1537, 1617, 1696, 1775, 1854, 1933, 2012, 2090, 2168, 2246, 2324, 2401, 2478, 2555, 2632,
    2708, 2784, 2860, 2935, 3010, 3085, 3159, 3233, 3307, 3380, 3453, 3526, 3599, 3670, 3742, 3813,
    3884, 3955, 4025, 4095, 4164, 4233, 4302, 4370, 4438, 4505, 4572, 4639, 4705, 4771, 4836, 4901,
    4966, 5030, 5094, 5157, 5220, 5282, 5344, 5406, 5467, 5528, 5589, 5649, 5708, 5768, 5826, 5885,
    5943, 6000, 6058, 6114, 6171, 6227, 6282, 6337, 6392, 6446, 6500, 6554, 6607, 6660, 6712, 6764,
    6815, 6867, 6917, 6968, 7018, 7068, 7117, 7166, 7214, 7262, 7310, 7358, 7405, 7451, 7498, 7544,
    7589, 7635, 7679, 7724, 7768, 7812, 7856, 7899, 7942, 7984, 8026, 8068, 8110, 8151, 8192,
];
