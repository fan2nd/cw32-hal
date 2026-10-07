#![deny(unsafe_code)]
//! 编码器角度 + 电压档位直接调 Uq；单母线样本只作粗限流，无 Id/Iq 电流环。
use crate::{arithmetic::Arithmetic, config::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum State {
    Calibrating,
    Off,
    Bootstrap,
    AlignHold,
    AlignSweep,
    AlignSettle,
    EncoderRun,
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
    Encoder,
    Alignment,
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
    pub sampled_peak_ma: i32,
    pub current_valid: bool,
    pub current_age: u32,
    pub filtered_peak_ma: i32,
    pub voltage_cap_mv: i32,
    pub voltage_dq_mv: [i32; 2],
    pub angle: u16,
    pub output_angle: u16,
    pub encoder_count: u16,
    pub encoder_delta: i32,
    pub encoder_direction: i32,
    pub speed_millihz: i32,
    pub speed_feedback_millihz: i32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControlFaultTrace {
    pub state_before: u32,
    pub age_frames: u32,
    pub alignment_travel: i32,
    pub previous_voltage_dq_mv: [i32; 2],
}
impl ControlFaultTrace {
    pub const fn new() -> Self {
        Self {
            state_before: 0,
            age_frames: 0,
            alignment_travel: 0,
            previous_voltage_dq_mv: [0; 2],
        }
    }
}

/// 硬件ARR=4095；按4096周期计算最短有符号差分，跨零点不产生假跳变。
struct Encoder {
    aligned: bool,
    initialized: bool,
    raw: u16,
    tick: u16,
    position: i32,
    zero: i32,
    delta: i32,
    direction: i32,
    window_counts: i32,
    window_ticks: u32,
    window_index: usize,
    count_history: [i32; SPEED_WINDOW_FRAMES],
    tick_history: [u16; SPEED_WINDOW_FRAMES],
    speed: i32,
}
impl Encoder {
    const fn new() -> Self {
        Self {
            aligned: false,
            initialized: false,
            raw: 0,
            tick: 0,
            position: 0,
            zero: 0,
            delta: 0,
            direction: 1,
            window_counts: 0,
            window_ticks: 0,
            window_index: 0,
            count_history: [0; SPEED_WINDOW_FRAMES],
            tick_history: [0; SPEED_WINDOW_FRAMES],
            speed: 0,
        }
    }

    fn update(
        &mut self,
        raw: u16,
        tick: u16,
        active: bool,
        math: &mut impl Arithmetic,
    ) -> Result<(), Fault> {
        if tick >= CONTROL_TICKS {
            return Err(Fault::Timing);
        }
        if i32::from(raw) >= ENCODER_CPR {
            return Err(Fault::Encoder);
        }
        if !self.initialized {
            self.initialized = true;
            self.raw = raw;
            self.tick = tick;
            return Ok(());
        }
        self.delta = ((i32::from(raw) - i32::from(self.raw) + ENCODER_CPR / 2) & (ENCODER_CPR - 1))
            - ENCODER_CPR / 2;
        let dt = CONTROL_TICKS as i32 + tick as i32 - self.tick as i32;
        self.raw = raw;
        self.tick = tick;
        if active && self.delta.abs() > ENCODER_MAX_DELTA {
            return Err(Fault::Encoder);
        }
        if !(CONTROL_TICKS as i32 / 2..=CONTROL_TICKS as i32 * 3 / 2).contains(&dt) {
            return Err(Fault::Timing);
        }
        // CPR 为固定的 2 次幂，包含负向运动；不将 16 位计数直接当成机械位置。
        self.position = (self.position + self.delta) & (ENCODER_CPR - 1);
        // 停机手转不受 active 的每帧位移约束，也不参与闭环速度反馈。
        // 清空窗口，避免把停机的大位移带进下一次启动的有限宽度运算。
        if !active {
            self.window_counts = 0;
            self.window_ticks = 0;
            self.window_index = 0;
            self.count_history = [0; SPEED_WINDOW_FRAMES];
            self.tick_history = [0; SPEED_WINDOW_FRAMES];
            self.speed = 0;
            return Ok(());
        }
        // 每100us替换最旧样本并重新测速；8帧滑动窗口抑制单计数噪声。
        // 同时累加真实时间，不能对不等间隔的瞬时速度做无权平均。
        self.window_counts += self.delta - self.count_history[self.window_index];
        self.window_ticks += dt as u32;
        self.window_ticks -= u32::from(self.tick_history[self.window_index]);
        self.count_history[self.window_index] = self.delta;
        self.tick_history[self.window_index] = dt as u16;
        self.window_index = (self.window_index + 1) % SPEED_WINDOW_FRAMES;
        // 商余数分解避免64位软除法；总计数<=8*16，余数<8*14400。
        let scale = ENCODER_SPEED_SCALE;
        let ticks = self.window_ticks as i32;
        let (quotient, remainder) = math.div_rem(scale, ticks);
        self.speed = (self.window_counts * quotient
            + math.div(self.window_counts * remainder, ticks))
            * self.direction;
        Ok(())
    }
    fn angle(&self) -> u16 {
        let relative = (self.position - self.zero) * self.direction;
        (((relative * POLE_PAIRS) & (ENCODER_CPR - 1)) * (65536 / ENCODER_CPR)) as u16
    }
}

pub struct Control {
    state: State,
    fault: Fault,
    fault_trace: ControlFaultTrace,
    requested_percent: i32,
    reset_pending: bool,
    age: u32,
    encoder: Encoder,
    travel: i32,
    still_min: i32,
    still_max: i32,
    still_position: i32,
    stall: u32,
    reverse_travel: i32,
    output_percent: i32,
    speed_feedback: i32,
    voltage_mv: i32,
    voltage_cap_mv: i32,
    current_q8: i32,
    snapshot: Snapshot,
}
impl Control {
    pub const fn new() -> Self {
        Self {
            state: State::Calibrating,
            fault: Fault::None,
            fault_trace: ControlFaultTrace::new(),
            requested_percent: 0,
            reset_pending: false,
            age: 0,
            encoder: Encoder::new(),
            travel: 0,
            still_min: 0,
            still_max: 0,
            still_position: 0,
            stall: 0,
            reverse_travel: 0,
            output_percent: 0,
            speed_feedback: 0,
            voltage_mv: 0,
            voltage_cap_mv: 0,
            current_q8: 0,
            snapshot: Snapshot {
                state: State::Calibrating,
                fault: Fault::None,
                sampled_peak_ma: 0,
                current_valid: false,
                current_age: 0,
                filtered_peak_ma: 0,
                voltage_cap_mv: 0,
                voltage_dq_mv: [0; 2],
                angle: 0,
                output_angle: 0,
                encoder_count: 0,
                encoder_delta: 0,
                encoder_direction: 0,
                speed_millihz: 0,
                speed_feedback_millihz: 0,
            },
        }
    }
    pub fn finish_calibration(&mut self) {
        if self.state == State::Calibrating {
            if valid() {
                self.enter(State::Off);
            } else {
                self.trip(Fault::Parameters);
            }
        }
    }
    pub fn request_output(&mut self, target: i32) {
        let rising = target > 0 && self.requested_percent == 0;
        self.requested_percent = target.clamp(0, 100);
        if target <= 0 && self.energized() {
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
    pub fn fault(&self) -> Fault {
        self.fault
    }
    pub fn fault_trace(&self) -> ControlFaultTrace {
        self.fault_trace
    }
    pub fn snapshot(&self) -> Snapshot {
        let mut s = self.snapshot;
        s.state = self.state;
        s.fault = self.fault;
        s.encoder_count = self.encoder.raw;
        s.encoder_delta = self.encoder.delta;
        s.encoder_direction = if self.encoder.aligned {
            self.encoder.direction
        } else {
            0
        };
        s.speed_millihz = self.encoder.speed;
        s.speed_feedback_millihz = self.speed_feedback;
        s
    }
    pub fn trip(&mut self, fault: Fault) {
        if self.state != State::Fault {
            // 启动请求尚未消费时也可能故障；禁止下一帧的延迟复位清除首故障。
            self.reset_pending = false;
            self.fault_trace = ControlFaultTrace {
                state_before: self.state as u32,
                age_frames: self.age,
                alignment_travel: self.travel,
                previous_voltage_dq_mv: self.snapshot.voltage_dq_mv,
            };
            self.fault = if fault == Fault::None {
                Fault::Driver
            } else {
                fault
            };
            self.enter(State::Fault);
            self.snapshot.voltage_dq_mv = [0; 2];
        }
    }
    fn energized(&self) -> bool {
        matches!(
            self.state,
            State::Bootstrap
                | State::AlignHold
                | State::AlignSweep
                | State::AlignSettle
                | State::EncoderRun
        )
    }
    fn enter(&mut self, state: State) {
        self.state = state;
        self.age = 0;
    }

    pub fn on_frame_timed(
        &mut self,
        current_sample: Option<i32>,
        bus_mv: i32,
        encoder_sample: (u16, u16),
        math: &mut impl Arithmetic,
    ) -> [i32; 2] {
        if self.state == State::Fault {
            return [0; 2];
        }
        if self.reset_pending {
            // 放在ADC EOS后的计算时隙，禁止在 reload 的 CCR 紧时隙清大对象。
            let state = self.state;
            let requested = self.requested_percent;
            *self = Self::new();
            self.state = state;
            self.requested_percent = requested;
        }
        let (raw, tick) = encoder_sample;
        if let Err(f) = self.encoder.update(raw, tick, self.energized(), math) {
            self.trip(f);
            return [0; 2];
        }
        self.snapshot.current_valid = current_sample.is_some();
        self.snapshot.current_age = if current_sample.is_some() {
            0
        } else {
            self.snapshot.current_age.saturating_add(1)
        };
        let sampled_peak_ma = current_sample.unwrap_or(self.snapshot.sampled_peak_ma);
        if current_sample.is_some() {
            self.snapshot.sampled_peak_ma = sampled_peak_ma;
        }
        if !self.energized() {
            return [0; 2];
        }
        let fault = if sampled_peak_ma < 0 {
            Fault::SampleInvalid
        } else if sampled_peak_ma > CURRENT_TRIP_MA {
            Fault::OverCurrent
        } else if bus_mv > BUS_MAX_MV {
            Fault::OverVoltage
        } else if bus_mv < BUS_MIN_MV {
            Fault::UnderVoltage
        } else {
            Fault::None
        };
        if fault != Fault::None {
            self.trip(fault);
            return [0; 2];
        }
        self.age = self.age.saturating_add(1);
        if self.state == State::Bootstrap {
            if self.age < BOOTSTRAP_FRAMES {
                return [0; 2];
            }
            self.enter(State::AlignHold);
        }
        // 静止资格使用末 100 ms 的位置包络，不用净位移掩盖来回振荡。
        if matches!(self.state, State::AlignHold | State::AlignSettle) {
            let duration = if self.state == State::AlignHold {
                ALIGN_HOLD_FRAMES
            } else {
                ALIGN_SETTLE_FRAMES
            };
            if self.age == duration - ALIGN_STILL_FRAMES {
                self.still_position = 0;
                self.still_min = 0;
                self.still_max = 0;
            }
            if self.age > duration - ALIGN_STILL_FRAMES {
                self.still_position += self.encoder.delta;
                self.still_min = self.still_min.min(self.still_position);
                self.still_max = self.still_max.max(self.still_position);
            }
        }
        if matches!(self.state, State::AlignSweep | State::AlignSettle) {
            self.travel += self.encoder.delta;
        }
        if self.state == State::AlignHold && self.age >= ALIGN_HOLD_FRAMES {
            if self.still_max - self.still_min > ALIGN_STILL_COUNTS {
                self.trip(Fault::Alignment);
                return [0; 2];
            }
            self.travel = 0;
            self.enter(State::AlignSweep);
        }
        if self.state == State::AlignSweep && self.age >= ALIGN_SWEEP_FRAMES {
            self.enter(State::AlignSettle);
        }
        if self.state == State::AlignSettle && self.age >= ALIGN_SETTLE_FRAMES {
            // 双相真实运动必须接近 90 电角对应的计数；静止/断线不能通过对齐。
            if !(ALIGN_TRAVEL_COUNTS * 3 / 4..=ALIGN_TRAVEL_COUNTS * 5 / 4)
                .contains(&self.travel.abs())
                || self.still_max - self.still_min > ALIGN_STILL_COUNTS
            {
                self.trip(Fault::Alignment);
                return [0; 2];
            }
            self.encoder.direction = if self.travel < 0 { 1 } else { -1 };
            self.encoder.zero = self.encoder.position;
            self.encoder.aligned = true;
            self.encoder.window_counts = 0;
            self.encoder.window_ticks = 0;
            self.encoder.window_index = 0;
            self.encoder.count_history = [0; SPEED_WINDOW_FRAMES];
            self.encoder.tick_history = [0; SPEED_WINDOW_FRAMES];
            self.encoder.speed = 0;
            self.output_percent = 0;
            self.speed_feedback = 0;
            self.voltage_mv = 0; // 对齐Ud退出后，Uq从零斜坡，避免电压轴突变。
            self.enter(State::EncoderRun);
        }
        // 仅对有效自然窗口的母线样本取绝对值，不声称它是Iq或三相峰值。
        // 1/32低通约3.2ms；未滤波峰值也能立即压低电压上限。
        if current_sample.is_some() {
            self.current_q8 += ((sampled_peak_ma << 8) - self.current_q8) >> 5;
        }
        // 高电流后若失去观测，不能靠假设电流归零自行恢复；超时锁存。
        if self.snapshot.current_age > CURRENT_STALE_FRAMES
            && (self.current_q8 >> 8) > RUN_CURRENT_LIMIT_MA - CURRENT_HYSTERESIS_MA
        {
            self.trip(Fault::SampleTimeout);
            return [0; 2];
        }
        let filtered = self.current_q8 >> 8;
        let current_limit = if self.state == State::EncoderRun {
            RUN_CURRENT_LIMIT_MA
        } else {
            ALIGN_CURRENT_LIMIT_MA
        };
        let bus_limit = bus_mv * VOLTAGE_LIMIT_Q15 >> 15;
        let nominal_cap = if self.state == State::EncoderRun {
            bus_limit
        } else {
            ALIGN_VOLTAGE_MV.min(bus_limit)
        };
        if sampled_peak_ma.max(filtered) > current_limit {
            // 从实际已用电压回退，不能先耗尽未使用的电压余量才起作用。
            self.voltage_cap_mv =
                (self.voltage_cap_mv.min(self.voltage_mv) - CURRENT_BACKOFF_MV).max(0);
        } else if filtered < current_limit - CURRENT_HYSTERESIS_MA {
            self.voltage_cap_mv = (self.voltage_cap_mv + CURRENT_RECOVER_MV).min(nominal_cap);
        }
        self.voltage_cap_mv = self.voltage_cap_mv.min(nominal_cap);
        if current_sample.is_none() {
            self.voltage_cap_mv = self
                .voltage_cap_mv
                .min(bus_mv * UNOBSERVED_VOLTAGE_Q15 >> 15);
        }
        self.snapshot.filtered_peak_ma = filtered;
        self.snapshot.voltage_cap_mv = self.voltage_cap_mv;
        let (angle, target) = match self.state {
            State::AlignHold => (16384, ALIGN_VOLTAGE_MV),
            State::AlignSweep => (
                (16384 - math.div(16384 * self.age as i32, ALIGN_SWEEP_FRAMES as i32)) as u16,
                ALIGN_VOLTAGE_MV,
            ),
            State::AlignSettle => (0, ALIGN_VOLTAGE_MV),
            State::EncoderRun => {
                if self.encoder.speed.abs() > OVERSPEED_MILLIHZ {
                    self.trip(Fault::Overspeed);
                    return [0; 2];
                }
                // 累积从正向最远位置回退的计数，正向运动抵消回退。
                // 不以8帧测速的单计数脉冲触发反转；零电压斜坡阶段仍检查
                // 实际回退量，持续反转不会因速度窗口间歇归零而漏报。
                self.reverse_travel =
                    (self.reverse_travel - self.encoder.delta * self.encoder.direction).max(0);
                if self.reverse_travel >= REVERSE_TRAVEL_COUNTS {
                    self.trip(Fault::Encoder);
                    return [0; 2];
                }
                self.stall = if self.age > STALL_GRACE_FRAMES
                    && self.encoder.speed < STALL_MIN_MILLIHZ
                    && self.output_percent > 0
                {
                    self.stall + 1
                } else {
                    0
                };
                if self.stall >= STALL_FRAMES {
                    self.trip(Fault::Stall);
                    return [0; 2];
                }
                if math.rem_unsigned(self.age, SPEED_LOG_FILTER_DIVIDER) == 0 {
                    self.speed_feedback += (self.encoder.speed - self.speed_feedback) / 4;
                }
                // 与06一致：每30ms升1%，降低档位立即降低百分比请求。
                self.output_percent = self.output_percent.min(self.requested_percent);
                if self.age > 0 && math.rem_unsigned(self.age, OUTPUT_RAMP_STEP_FRAMES) == 0 {
                    self.output_percent = (self.output_percent + 1).min(self.requested_percent);
                }
                // 档位按母线电压可用范围定义，不受测得转速驱动，也无PI积分。
                (
                    self.encoder.angle(),
                    math.div(bus_limit * self.output_percent, 100),
                )
            }
            _ => return [0; 2],
        };
        self.snapshot.angle = angle;
        // 电压斜坡用于平滑起步；电流限幅可立即下压，不受斜坡阻碍。
        self.voltage_mv = if self.state == State::EncoderRun {
            target.min(self.voltage_cap_mv)
        } else {
            slew(
                self.voltage_mv,
                target.min(self.voltage_cap_mv),
                VOLTAGE_SLEW_MV_PER_FRAME,
            )
            .min(self.voltage_cap_mv)
        };
        let voltage_dq = if self.state == State::EncoderRun {
            [0, self.voltage_mv]
        } else {
            [self.voltage_mv, 0]
        };
        self.snapshot.voltage_dq_mv = voltage_dq;
        // N 帧计算，N+1 reload 写预载，N+2 生效；预测到该帧中点。
        // 编码器角只用于电压矢量，不再对电流做 Park 变换。
        let output_angle = if self.state == State::EncoderRun {
            let us = (CONTROL_TICKS as i32 * 5 / 2 - tick as i32) / (CPU_HZ as i32 / 1_000_000);
            let scaled = math.div(self.encoder.speed * us, 125);
            let advance = math.div(scaled * 128, 15625);
            angle.wrapping_add(advance as u16)
        } else {
            angle
        };
        self.snapshot.output_angle = output_angle;
        let (sin, cos) = sin_cos(output_angle);
        if math.failed() {
            self.trip(Fault::Arithmetic);
            return [0; 2];
        }
        inverse_park(voltage_dq, sin, cos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetic::Software;

    fn frame(c: &mut Control, raw: u16) -> [i32; 2] {
        c.on_frame_timed(Some(0), 12000, (raw, 2000), &mut Software)
    }

    // 合成理想跟随编码器，仅验证软件状态机，不模拟电机电流/力矩。
    fn aligned(direction: i32) -> (Control, u16) {
        let mut c = Control::new();
        c.finish_calibration();
        c.request_output(20);
        let mut raw = 1000u16;
        for _ in 0..CONTROL_HZ * 3 {
            if c.state == State::AlignSweep {
                let progress = (c.age + 1).min(ALIGN_SWEEP_FRAMES) as i32;
                raw = (1000
                    - direction * ALIGN_TRAVEL_COUNTS * progress / ALIGN_SWEEP_FRAMES as i32)
                    as u16;
            }
            frame(&mut c, raw);
            assert_eq!(c.fault, Fault::None);
            if c.state == State::EncoderRun {
                return (c, raw);
            }
        }
        panic!("alignment did not finish");
    }

    #[test]
    fn alignment_voltage_ramps_without_current_pi() {
        let mut c = Control::new();
        c.finish_calibration();
        c.request_output(20);
        while c.state != State::AlignHold {
            frame(&mut c, 0);
        }
        let before = c.snapshot.voltage_dq_mv[0];
        for _ in 0..100 {
            frame(&mut c, 0);
        }
        assert_eq!(
            c.snapshot.voltage_dq_mv[0] - before,
            100 * CURRENT_RECOVER_MV.min(VOLTAGE_SLEW_MV_PER_FRAME)
        );
        assert_eq!(c.snapshot.voltage_dq_mv[1], 0);
        // 有观测时可渐升至增强后的对齐电压；超过对齐粗限流立即回退。
        for _ in 0..ALIGN_VOLTAGE_MV {
            frame(&mut c, 0);
        }
        assert_eq!(c.snapshot.voltage_dq_mv[0], ALIGN_VOLTAGE_MV);
        c.on_frame_timed(
            Some(ALIGN_CURRENT_LIMIT_MA + 1),
            12000,
            (0, 2000),
            &mut Software,
        );
        assert_eq!(
            c.snapshot.voltage_dq_mv[0],
            ALIGN_VOLTAGE_MV - CURRENT_BACKOFF_MV
        );
        // 提高有观测电压不能扩大采样盲区的电压上限。
        c.on_frame_timed(None, 12000, (0, 2000), &mut Software);
        assert!(c.snapshot.voltage_dq_mv[0] <= 12000 * UNOBSERVED_VOLTAGE_Q15 >> 15);
    }

    #[test]
    fn natural_modulation_never_injects_voltage_for_zero_request() {
        use crate::sampling::*;
        let mut m = Modulator::new();
        for _ in 0..100 {
            let f = m.plan([0, 0], 12000, &mut Software).unwrap();
            assert_eq!(f.duty, [PWM_HALF_TICKS / 2; 3]);
            assert!(!f.current_valid);
            assert!(f.valid());
        }
        // 实际可测窗口必须来自自然占空比，不得靠扩展脉冲制造。
        for angle in (0..65536u32).step_by(257) {
            let (sin, cos) = sin_cos(angle as u16);
            let f = m
                .plan(inverse_park([1600, 0], sin, cos), 12000, &mut Software)
                .unwrap();
            assert!(f.valid());
            assert!(CONTROL_DEADLINE > f.sample + CONVERSION_TICKS + SAMPLE_IRQ_SLACK);
        }
    }

    #[test]
    fn boot_is_off_until_manual_start_and_fault_latches() {
        assert!(valid());
        let mut c = Control::new();
        c.finish_calibration();
        for _ in 0..100 {
            assert_eq!(frame(&mut c, 0), [0; 2]);
        }
        assert_eq!(c.state, State::Off);
        c.request_output(20);
        assert_eq!(c.state, State::Bootstrap);
        c.trip(Fault::Encoder);
        c.request_output(0);
        c.request_output(20);
        assert_eq!(frame(&mut c, 0), [0; 2]);
        assert_eq!(c.state, State::Fault);
        assert_eq!(c.fault, Fault::Encoder);
    }

    #[test]
    fn expanded_voltage_range_preserves_sampling_and_line_voltage() {
        use crate::sampling::*;
        let mut modulator = Modulator::new();
        for bus in [BUS_MIN_MV, 12000, BUS_MAX_MV] {
            let cap = bus * VOLTAGE_LIMIT_Q15 >> 15;
            let observable = bus * UNOBSERVED_VOLTAGE_Q15 >> 15;
            for voltage in [0, 500, 1600, ALIGN_VOLTAGE_MV, observable, cap] {
                for angle in (0..65536u32).step_by(31) {
                    let (sin, cos) = sin_cos(angle as u16);
                    let ab = inverse_park([0, voltage], sin, cos);
                    let f = modulator.plan(ab, bus, &mut Software).unwrap();
                    assert!(f.valid());
                    assert!(
                        (f.duty[f.order[0]] + f.duty[f.order[2]]).abs_diff(PWM_HALF_TICKS) <= 1
                    );
                    if voltage >= observable {
                        assert!(f.current_valid);
                    }
                    // 在新的上限内不发生跨度压缩；平均线电压只允许整数舍入误差。
                    let phase_b = (-ab[0] + ab[1] + ((ab[1] * 23988) >> 15)) / 2;
                    let expected = ab[0] - phase_b;
                    let actual =
                        (f.duty[0] as i32 - f.duty[1] as i32) * bus / PWM_HALF_TICKS as i32;
                    assert!((actual - expected).abs() <= 2 * bus / PWM_HALF_TICKS as i32 + 2);
                }
            }
        }
    }

    #[test]
    fn center_pwm_has_mirrored_seven_segments_and_equal_zero_times() {
        use crate::sampling::Modulator;
        let mut modulator = Modulator::new();
        for sector in 0..6u32 {
            let angle = ((sector * 65536 / 6 + 1700) & 65535) as u16;
            let (sin, cos) = sin_cos(angle);
            let f = modulator
                .plan(inverse_park([0, 4000], sin, cos), 12000, &mut Software)
                .unwrap();
            let [a, b, c] = f.order;
            let edges = [
                0,
                PWM_HALF_TICKS - f.duty[c],
                PWM_HALF_TICKS - f.duty[b],
                PWM_HALF_TICKS - f.duty[a],
                PWM_HALF_TICKS + f.duty[a],
                PWM_HALF_TICKS + f.duty[b],
                PWM_HALF_TICKS + f.duty[c],
                PWM_TICKS,
            ];
            let mut masks = [0u8; 7];
            for i in 0..7 {
                assert!(edges[i + 1] > edges[i]);
                let t = (edges[i] + edges[i + 1]) / 2;
                let count = t.abs_diff(PWM_HALF_TICKS);
                for phase in 0..3 {
                    if count < f.duty[phase] {
                        masks[i] |= 1 << phase;
                    }
                }
            }
            assert_eq!((masks[0], masks[3], masks[6]), (0, 7, 0));
            for i in 0..6 {
                assert_eq!((masks[i] ^ masks[i + 1]).count_ones(), 1);
            }
            for i in 0..7 {
                assert_eq!(masks[i], masks[6 - i]);
            }
            let zero_000 = edges[1] + PWM_TICKS - edges[6];
            let zero_111 = edges[4] - edges[3];
            assert!(zero_000.abs_diff(zero_111) <= 2);
        }
    }

    #[test]
    fn wraps_hardware_count_in_both_directions() {
        assert_eq!(
            Encoder::new().update(4096, 2000, true, &mut Software),
            Err(Fault::Encoder)
        );
        let mut e = Encoder::new();
        e.update(4094, 2000, true, &mut Software).unwrap();
        e.update(1, 2000, true, &mut Software).unwrap();
        assert_eq!(e.delta, 3);
        assert_eq!(e.position, 3);
        e.update(4093, 2000, true, &mut Software).unwrap();
        assert_eq!(e.delta, -4);
        assert_eq!(e.position, 4095);
        assert!(e.update(1000, 2000, true, &mut Software).is_err());
    }

    #[test]
    fn exactly_eleven_electrical_turns_per_mechanical_turn() {
        for direction in [-1, 1] {
            let mut e = Encoder::new();
            e.direction = direction;
            e.zero = 0;
            let mut accumulated = 0i32;
            let mut last = 0u16;
            for count in 1..=ENCODER_CPR {
                e.position = count & (ENCODER_CPR - 1);
                let angle = e.angle();
                accumulated += angle.wrapping_sub(last) as i16 as i32;
                last = angle;
            }
            assert_eq!(accumulated, direction * POLE_PAIRS * 65536);
            assert_eq!(last, 0);
        }
    }

    #[test]
    fn speed_uses_actual_read_timestamps() {
        let mut e = Encoder::new();
        let hz = 60000i64;
        for n in 0..=300 {
            let tick = if n % 2 == 0 {
                CONTROL_TICKS / 8
            } else {
                CONTROL_TICKS * 3 / 8
            };
            let time = n as i64 * CONTROL_TICKS as i64 + tick as i64;
            let raw = (time * hz * ENCODER_CPR as i64 / (CPU_HZ as i64 * 1000 * POLE_PAIRS as i64))
                as u16;
            e.update(raw & 4095, tick, true, &mut Software).unwrap();
            if n >= SPEED_LOG_FILTER_DIVIDER && n % SPEED_LOG_FILTER_DIVIDER == 0 {
                assert!((e.speed - 60000).abs() < 4000, "{}", e.speed);
            }
        }
    }

    #[test]
    fn hardware_speed_arithmetic_matches_wide_reference() {
        for direction in [-1, 1] {
            let mut e = Encoder::new();
            e.direction = direction;
            let mut raw = 0u16;
            let mut previous_tick = 2000;
            let mut counts = [0i32; SPEED_WINDOW_FRAMES];
            let mut ticks = [0u32; SPEED_WINDOW_FRAMES];
            e.update(raw, previous_tick, true, &mut Software).unwrap();
            for n in 0..1000 {
                let delta = n as i32 % 33 - 16;
                let tick = if n % 2 == 0 { 1000 } else { 3000 };
                raw = ((i32::from(raw) + delta) & 4095) as u16;
                counts[n % SPEED_WINDOW_FRAMES] = delta;
                ticks[n % SPEED_WINDOW_FRAMES] =
                    (CONTROL_TICKS as i32 + tick as i32 - previous_tick as i32) as u32;
                previous_tick = tick;
                e.update(raw, tick, true, &mut Software).unwrap();
                let expected = counts.iter().sum::<i32>() as i64
                    * direction as i64
                    * POLE_PAIRS as i64
                    * CPU_HZ as i64
                    * 1000
                    / (ENCODER_CPR as i64 * ticks.iter().sum::<u32>() as i64);
                assert_eq!(e.speed, expected as i32);
            }
            e.update(1234, 2000, false, &mut Software).unwrap();
            assert_eq!((e.window_counts, e.window_ticks, e.speed), (0, 0, 0));
            assert_eq!(e.count_history, [0; SPEED_WINDOW_FRAMES]);
            assert_eq!(e.tick_history, [0; SPEED_WINDOW_FRAMES]);
        }
    }

    #[test]
    fn speed_changes_every_frame_and_replaces_old_motion_in_eight_frames() {
        let mut e = Encoder::new();
        let mut raw = 4090u16;
        e.update(raw, 2000, true, &mut Software).unwrap();
        for _ in 0..SPEED_WINDOW_FRAMES {
            raw = (raw + 2) & 4095;
            e.update(raw, 2000, true, &mut Software).unwrap();
        }
        let original = e.speed;
        for _ in 0..SPEED_WINDOW_FRAMES {
            let previous = e.speed;
            raw = (raw + 4) & 4095;
            e.update(raw, 2000, true, &mut Software).unwrap();
            assert!(e.speed > previous);
        }
        assert!((e.speed - 2 * original).abs() <= 1);
        for _ in 0..SPEED_WINDOW_FRAMES {
            let previous = e.speed;
            e.update(raw, 2000, true, &mut Software).unwrap();
            assert!(e.speed < previous);
        }
        assert_eq!(e.speed, 0);
    }

    #[test]
    fn alignment_learns_either_ab_order_and_zero() {
        for direction in [-1, 1] {
            let (mut c, raw) = aligned(direction);
            assert_eq!(c.encoder.direction, direction);
            assert_eq!(c.encoder.angle(), 0);
            frame(&mut c, raw.wrapping_add(direction as u16));
            assert_eq!(c.encoder.angle(), 176); // 11 * 65536 / 4096
            c.request_output(40);
            assert_eq!(c.state, State::EncoderRun);
        }
    }

    #[test]
    fn disconnected_encoder_never_enters_closed_loop() {
        let mut c = Control::new();
        c.finish_calibration();
        c.request_output(20);
        for _ in 0..CONTROL_HZ * 2 {
            frame(&mut c, 1234);
            assert_ne!(c.state, State::EncoderRun);
            if c.state == State::Fault {
                break;
            }
        }
        assert_eq!(c.fault, Fault::Alignment);
    }

    #[test]
    fn wrong_resolution_is_rejected() {
        let mut c = Control::new();
        c.finish_calibration();
        c.request_output(20);
        let mut raw = 1000;
        for _ in 0..CONTROL_HZ * 2 {
            if c.state == State::AlignSweep {
                raw = (1000
                    - ALIGN_TRAVEL_COUNTS * (c.age + 1).min(ALIGN_SWEEP_FRAMES) as i32
                        / (4 * ALIGN_SWEEP_FRAMES as i32)) as u16;
            }
            frame(&mut c, raw);
            assert_ne!(c.state, State::EncoderRun);
            if c.state == State::Fault {
                break;
            }
        }
        assert_eq!(c.fault, Fault::Alignment);
    }

    #[test]
    fn loaded_short_alignment_travel_is_rejected() {
        let mut c = Control::new();
        c.finish_calibration();
        c.request_output(20);
        let mut raw = 1000;
        // 回归台架37计数的失败轨迹，增强电压不应绕过对齐资格。
        for _ in 0..CONTROL_HZ * 2 {
            if c.state == State::AlignSweep {
                raw = (1000 - 37 * (c.age + 1).min(ALIGN_SWEEP_FRAMES) / ALIGN_SWEEP_FRAMES) as u16;
            }
            frame(&mut c, raw);
            assert_ne!(c.state, State::EncoderRun);
            if c.state == State::Fault {
                break;
            }
        }
        assert_eq!(c.travel.abs(), 37);
        assert_eq!(c.fault, Fault::Alignment);
    }

    #[test]
    fn stop_discards_alignment_and_restart_realigns() {
        let (mut c, raw) = aligned(1);
        c.request_output(0);
        assert_eq!(frame(&mut c, raw), [0; 2]);
        assert_eq!(c.state, State::Off);
        frame(&mut c, raw.wrapping_add(100));
        c.request_output(20);
        frame(&mut c, raw);
        assert_eq!(c.state, State::Bootstrap);
    }

    #[test]
    fn overspeed_reverse_and_stall_stop_output() {
        for (delta, fault) in [(7i32, Fault::Overspeed)] {
            let (mut c, raw) = aligned(1);
            assert_eq!(frame(&mut c, ((raw as i32 + delta) & 4095) as u16), [0; 2]);
            assert_eq!(c.fault, fault);
        }
        let (mut c, raw) = aligned(1);
        for _ in 0..STALL_GRACE_FRAMES + STALL_FRAMES + 10 {
            frame(&mut c, raw);
        }
        assert_eq!(c.fault, Fault::Stall);
    }

    #[test]
    fn startup_rebound_is_not_reverse_but_sustained_reverse_still_trips() {
        let (mut c, mut raw) = aligned(1);
        // 实板故障发生在首个30ms斜坡台阶之前；一个反向计数不能误停。
        for _ in 0..219 {
            frame(&mut c, raw);
        }
        raw = raw.wrapping_sub(1) & 4095;
        frame(&mut c, raw);
        assert_eq!(c.fault, Fault::None);
        assert!(c.encoder.speed < -STALL_MIN_MILLIHZ);
        assert_eq!(c.reverse_travel, 1);
        raw = (raw + 1) & 4095;
        frame(&mut c, raw);
        assert_eq!(c.reverse_travel, 0);
        // 每次后退之间停20帧，速度会归零，但持续反向位移仍须被发现。
        for n in 1..=REVERSE_TRAVEL_COUNTS {
            raw = raw.wrapping_sub(1) & 4095;
            frame(&mut c, raw);
            if n < REVERSE_TRAVEL_COUNTS {
                for _ in 0..20 {
                    frame(&mut c, raw);
                }
                assert_eq!(c.fault, Fault::None);
            }
        }
        assert_eq!(c.fault, Fault::Encoder);
        assert_eq!(c.snapshot.voltage_dq_mv, [0; 2]);
    }

    #[test]
    fn voltage_prediction_uses_pipeline_delay_not_current_angle() {
        let (mut c, raw) = aligned(1);
        frame(&mut c, (raw + 5) & 4095);
        let us = (CONTROL_TICKS as i32 * 5 / 2 - 2000) / (CPU_HZ as i32 / 1_000_000);
        let expected = c.encoder.speed as i64 * us as i64 * 65536 / 1_000_000_000;
        assert!(
            (c.snapshot.output_angle.wrapping_sub(c.snapshot.angle) as i64 - expected).abs() <= 1
        );
        assert_eq!(c.snapshot.angle, 5 * 176);
    }

    #[test]
    fn current_and_supply_faults_return_zero_voltage() {
        for (peak, bus, fault) in [
            (1801, 12000, Fault::OverCurrent),
            (0, BUS_MIN_MV - 1, Fault::UnderVoltage),
            (0, BUS_MAX_MV + 1, Fault::OverVoltage),
        ] {
            let (mut c, raw) = aligned(1);
            let out = c.on_frame_timed(Some(peak), bus, (raw, 2000), &mut Software);
            assert_eq!(out, [0; 2]);
            assert_eq!(c.fault, fault);
        }
    }

    #[test]
    fn coarse_limit_cuts_voltage_immediately_and_recovers_slowly() {
        let (mut c, raw) = aligned(1);
        c.voltage_mv = 2000;
        c.output_percent = 100;
        c.requested_percent = 100;
        c.voltage_cap_mv = 3750;
        // 单帧尖峰不等低通抬升；从实际2V回退，而不是从空余3.75V回退。
        c.on_frame_timed(Some(1200), 12000, (raw, 2000), &mut Software);
        assert_eq!(c.fault, Fault::None);
        assert_eq!(c.voltage_mv, 2000 - CURRENT_BACKOFF_MV);
        let cap = c.voltage_cap_mv;
        c.on_frame_timed(Some(0), 12000, (raw, 2000), &mut Software);
        assert_eq!(c.voltage_cap_mv, cap + CURRENT_RECOVER_MV);
        assert_eq!(c.snapshot.voltage_dq_mv[0], 0);
        // 持续软过流退到零；不冒充精确Iq，也不代替更高一级硬关断。
        for _ in 0..200 {
            c.on_frame_timed(Some(1200), 12000, (raw, 2000), &mut Software);
        }
        assert_eq!(c.voltage_mv, 0);
        assert_eq!(c.fault, Fault::None);
    }

    #[test]
    fn missing_current_is_not_zero_and_limits_voltage() {
        let (mut c, raw) = aligned(1);
        c.voltage_mv = 3000;
        c.output_percent = 100;
        c.requested_percent = 100;
        c.voltage_cap_mv = 3750;
        c.snapshot.sampled_peak_ma = 700;
        c.current_q8 = 700 << 8;
        c.on_frame_timed(None, 12000, (raw, 2000), &mut Software);
        assert_eq!(c.snapshot.sampled_peak_ma, 700);
        assert_eq!(c.snapshot.filtered_peak_ma, 700);
        assert!(!c.snapshot.current_valid);
        assert!(c.voltage_mv <= 12000 * UNOBSERVED_VOLTAGE_Q15 >> 15);
        // 曾观察到高电流后失去观测，不能假定电流为零并自行恢复。
        c.current_q8 = 1200 << 8;
        c.snapshot.sampled_peak_ma = 1200;
        for _ in 0..CURRENT_STALE_FRAMES + 1 {
            c.on_frame_timed(None, 12000, (raw, 2000), &mut Software);
        }
        assert_eq!(c.fault, Fault::SampleTimeout);
        assert_eq!(c.snapshot.voltage_dq_mv, [0; 2]);
    }

    #[test]
    fn voltage_output_and_modulator_respect_bus_and_current_caps() {
        use crate::sampling::Modulator;
        let (mut c, mut raw) = aligned(1);
        c.request_output(100);
        let mut modulator = Modulator::new();
        for i in 0..CONTROL_HZ * 2 {
            raw = raw.wrapping_add((50_000 / CONTROL_HZ) as u16) & 4095;
            let peak = if i % 100 < 30 { 1200 } else { 600 };
            let out = c.on_frame_timed(Some(peak), BUS_MIN_MV, (raw, 2000), &mut Software);
            assert_eq!(c.fault, Fault::None);
            assert!(c.voltage_mv <= c.voltage_cap_mv);
            assert!(c.voltage_cap_mv <= BUS_MIN_MV * VOLTAGE_LIMIT_Q15 >> 15);
            assert!(modulator
                .plan(out, BUS_MIN_MV, &mut Software)
                .unwrap()
                .valid());
        }
    }

    #[test]
    fn output_percent_matches_06_ramp_and_is_independent_of_speed() {
        let (mut c, mut raw) = aligned(1);
        c.request_output(100);
        let full = 12000 * VOLTAGE_LIMIT_Q15 >> 15;
        for n in 1..=CONTROL_HZ * 3 {
            raw = raw.wrapping_add(1) & 4095; // 保持正向运动，测试斜坡而非堵转判据。
            frame(&mut c, raw);
            assert_eq!(c.fault, Fault::None);
            assert_eq!(c.output_percent, (n / OUTPUT_RAMP_STEP_FRAMES) as i32);
            assert_eq!(c.voltage_mv, full * c.output_percent / 100);
        }
        assert_eq!(c.output_percent, 100);
        c.request_output(40);
        frame(&mut c, raw);
        assert_eq!(c.output_percent, 40); // 降档不等待慢升斜坡。
        assert_eq!(c.voltage_mv, full * 40 / 100);
        c.encoder.speed = 100000;
        frame(&mut c, raw);
        assert_eq!(c.voltage_mv, full * 40 / 100); // 编码器速度不参与电压调节。
        c.request_output(0);
        assert_eq!(frame(&mut c, raw), [0; 2]);
        assert_eq!(c.state, State::Off);
    }
}

fn inverse_park(v: [i32; 2], sin: i32, cos: i32) -> [i32; 2] {
    [
        (v[0] * cos - v[1] * sin) >> 15,
        (v[0] * sin + v[1] * cos) >> 15,
    ]
}

fn slew(value: i32, target: i32, amount: i32) -> i32 {
    value + (target - value).clamp(-amount, amount)
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

// Q15 sine table, one quarter turn; interpolation preserves the 07 math convention.
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
