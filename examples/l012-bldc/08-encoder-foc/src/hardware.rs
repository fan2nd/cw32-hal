//! 单电阻硬件时序。P0 的 ATIM/ADC1 ISR 同级且不嵌套；控制计算不进入异步任务。
use crate::{
    arithmetic::Arithmetic,
    config::*,
    control::{Control, ControlFaultTrace, Fault, State},
    sampling::{self, Frame, Modulator},
    Irqs,
};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use embassy_cw32::{
    eau::{Eau, Error as EauError},
    gpio::Port,
    interrupt::{
        self,
        typelevel::{self, Handler, Interrupt as _},
    },
    motor::{
        self, AdcScan, BasicTimer, ClockDivider, MotorPin, NegativeInput, PinId, PinMode,
        PositiveInput, SampleTime, ScanConfig, ScanSlot, SingleShuntPwm, TimerConfig,
    },
    peripherals,
    timer::{
        input_capture::Filter,
        qei::{Config as QeiConfig, Qei},
    },
    Peri,
};

/// 字段用于独占保留单例，避免其他驱动复用电机引脚和外设。
#[allow(dead_code)]
pub struct MotorResources {
    pub flash: Peri<'static, peripherals::FLASH>,
    pub eau: Peri<'static, peripherals::EAU>,
    pub atim: Peri<'static, peripherals::ATIM>,
    pub adc1: Peri<'static, peripherals::ADC1>,
    pub adc2: Peri<'static, peripherals::ADC2>,
    pub opa1: Peri<'static, peripherals::OPA1>,
    pub bgr: Peri<'static, peripherals::BGR>,
    pub btim1: Peri<'static, peripherals::BTIM1>,
    pub btim2: Peri<'static, peripherals::BTIM2>,
    pub gtim2: Peri<'static, peripherals::GTIM2>,
    pub pb14: Peri<'static, peripherals::PB14>,
    pub pb15: Peri<'static, peripherals::PB15>,
    pub pa15: Peri<'static, peripherals::PA15>,
    pub pb3: Peri<'static, peripherals::PB3>,
    pub pb4: Peri<'static, peripherals::PB4>,
    pub pb5: Peri<'static, peripherals::PB5>,
    pub pb6: Peri<'static, peripherals::PB6>,
    pub pb7: Peri<'static, peripherals::PB7>,
    pub pa6: Peri<'static, peripherals::PA6>,
    pub pa7: Peri<'static, peripherals::PA7>,
    pub pb0: Peri<'static, peripherals::PB0>,
    pub pa8: Peri<'static, peripherals::PA8>,
}
const GATES: [PinId; 6] = [
    PinId::new(Port::A, 15),
    PinId::new(Port::B, 3),
    PinId::new(Port::B, 4),
    PinId::new(Port::B, 5),
    PinId::new(Port::B, 6),
    PinId::new(Port::B, 7),
];
static INITIALIZED: AtomicBool = AtomicBool::new(false);
// 单个原子命令同时表达停机和目标电气频率，避免运行标志与档位不一致。
pub static OUTPUT_REQUEST_PERCENT: AtomicU32 = AtomicU32::new(0);
// 单生产者/消费者邮箱：线程请求(1)，ADC ISR写完发布(2)，线程读完归零。
// 只复制10个字，不在控制ISR内打印或复制完整故障诊断；Release/Acquire
// 保证线程读取期间ISR不覆盖数据，也不需要屏蔽PWM/ADC中断。
pub static LIVE_STATE: AtomicU32 = AtomicU32::new(0);
pub static LIVE_WORDS: [AtomicU32; 10] = [const { AtomicU32::new(0) }; 10];
pub static MILLISECONDS: AtomicU32 = AtomicU32::new(0);
// 全部实时状态只有 P0 中断访问；线程只读独立原子状态字。
static mut MOTOR: Option<Runtime> = None;
static RUN_STATUS: AtomicU32 = AtomicU32::new(0);
static FAULT_LOG_READY: AtomicBool = AtomicBool::new(false);
// 只在首故障已关桥、停触发后写一次；不增加运行期诊断复制成本。
#[no_mangle]
pub static mut FOC_CONTROL_FAULT: ControlFaultTrace = ControlFaultTrace::new();

// EAU 单例与实时状态共同保留；只有不嵌套的 ADC1 P0 路径借用它。
struct Runtime {
    motor: Motor,
    encoder: Qei<'static, peripherals::GTIM2>,
    eau: Eau<'static, peripherals::EAU>,
}

struct HardwareMath<'a> {
    eau: &'a mut Eau<'static, peripherals::EAU>,
    pwm: &'a mut SingleShuntPwm,
    failed: bool,
    error: u32,
}
impl HardwareMath<'_> {
    // RM1.4 §11.3：除法 2–35 HCLK、开方 17 HCLK；64 次状态读取是有限
    // 轮询预算，不是已实测 WCET。任何失败立即关桥/停触发，绝不软件重算。
    #[inline(always)]
    fn accept<T>(&mut self, result: Result<T, EauError>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                self.error = match error {
                    EauError::DivisionByZero => 1,
                    EauError::SignedOverflow => 2,
                    EauError::Busy => 3,
                    EauError::Timeout => 4,
                    EauError::ZeroPollBudget => 5,
                };
                self.pwm.disarm();
                self.pwm.stop();
                self.failed = true;
                None
            }
        }
    }
}
impl Arithmetic for HardwareMath<'_> {
    #[inline(always)]
    fn failed(&self) -> bool {
        self.failed
    }
    #[inline(always)]
    fn div(&mut self, n: i32, d: i32) -> i32 {
        self.div_rem(n, d).0
    }
    // 高频算术内联，使常量除数的检查折叠，并消除参数/返回值搬运。
    // EAU错误检查及有限轮询仍全部保留。
    #[inline(always)]
    fn div_rem(&mut self, n: i32, d: i32) -> (i32, i32) {
        if self.failed {
            return (0, 0);
        }
        let result = self.eau.divide_signed(n, d, 64);
        self.accept(result)
            .map_or((0, 0), |r| (r.quotient, r.remainder))
    }
    #[inline(always)]
    fn rem_unsigned(&mut self, n: u32, d: u32) -> u32 {
        if self.failed {
            return 0;
        }
        let result = self.eau.divide_unsigned(n, d, 64);
        self.accept(result).map_or(0, |r| r.remainder)
    }
}

/// 首个故障的采样证据；reason：0=其他，1=跨 reload，2=多余样本，3=过早，4=过晚。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SampleTrace {
    pub reason: u32,
    pub index: u32,
    pub irq_entry: u16,
    pub result_read: u16,
    pub raw: u16,
    pub expected_min: u16,
    pub expected_max: u16,
    pub previous_irq_end: u16,
    pub max_first_irq_ticks: u16,
    pub trigger: [u16; 2],
    pub received: [u16; 2],
}
impl SampleTrace {
    const fn new() -> Self {
        Self {
            reason: 0,
            index: 0,
            irq_entry: 0,
            result_read: 0,
            raw: 0,
            expected_min: 0,
            expected_max: 0,
            previous_irq_end: 0,
            max_first_irq_ticks: 0,
            trigger: [0; 2],
            received: [0; 2],
        }
    }
}
/// 13 号故障的确切检查点；只在故障时填写，不增加逐样本快照复制。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimingTrace {
    pub site: u32,
    pub state_before: u32,
    pub armed_before: bool,
    pub start: u16,
    pub observed: u16,
    pub limit: u16,
    pub samples: u16,
    pub reload_pending: bool,
    pub elapsed_min: u16,
    pub stage: u16,
}
impl TimingTrace {
    const fn new() -> Self {
        Self {
            site: 0,
            state_before: 0,
            armed_before: false,
            start: 0,
            observed: 0,
            limit: 0,
            samples: 0,
            reload_pending: false,
            elapsed_min: 0,
            stage: 0,
        }
    }
}
/// 校准失败细分；只在安全停机后打印。candidate 不是已接受的 VDDA。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalibrationTrace {
    pub reason: u32,
    pub raw_supply: [u16; 2],
    pub reference_mv: i32,
    pub candidate_vdda_mv: i32,
    pub supply_ready: bool,
    pub samples: u32,
    pub minimum: u16,
    pub maximum: u16,
}
impl CalibrationTrace {
    const fn new() -> Self {
        Self {
            reason: 0,
            raw_supply: [0; 2],
            reference_mv: 0,
            candidate_vdda_mv: 0,
            supply_ready: false,
            samples: 0,
            minimum: 4095,
            maximum: 0,
        }
    }
}
/// 调试器读取前须物理断开母线；首故障也会在高频源停止后通过 RTT 输出。
/// 运行中外部读取可能跨越一次更新；不允许其他 Rust 任务借用此对象。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Diagnostics {
    pub state: u32,
    pub fault: u32,
    pub arithmetic_error: u32,
    pub calibration_trace: CalibrationTrace,
    pub bus_mv: i32,
    pub vdda_mv: i32,
    pub offset_adc: i32,
    pub raw_adc: [u16; 2],
    pub sampled_peak_ma: i32,
    pub current_valid: bool,
    pub current_age: u32,
    pub max_control_ticks: u16,
    pub stage_ticks: [u16; 3],
    pub max_stage_ticks: [u16; 3],
    pub max_update_ticks: u16,
    pub armed: bool,
    pub frames: u32,
    pub filtered_peak_ma: i32,
    pub voltage_cap_mv: i32,
    pub voltage_dq_mv: [i32; 2],
    pub angle: u16,
    pub encoder_count: u16,
    pub encoder_delta: i32,
    pub encoder_direction: i32,
    pub output_angle: u16,
    pub speed_millihz: i32,
    pub speed_feedback_millihz: i32,
    pub sample_trace: SampleTrace,
    pub timing_trace: TimingTrace,
}
#[no_mangle]
pub static mut FOC_DIAGNOSTICS: Diagnostics = Diagnostics {
    state: 0,
    fault: 0,
    arithmetic_error: 0,
    calibration_trace: CalibrationTrace::new(),
    bus_mv: 0,
    vdda_mv: 0,
    offset_adc: 0,
    raw_adc: [0; 2],
    sampled_peak_ma: 0,
    current_valid: false,
    current_age: 0,
    max_control_ticks: 0,
    stage_ticks: [0; 3],
    max_stage_ticks: [0; 3],
    max_update_ticks: 0,
    armed: false,
    frames: 0,
    filtered_peak_ma: 0,
    voltage_cap_mv: 0,
    voltage_dq_mv: [0; 2],
    angle: 0,
    encoder_count: 0,
    encoder_delta: 0,
    encoder_direction: 0,
    output_angle: 0,
    speed_millihz: 0,
    speed_feedback_millihz: 0,
    sample_trace: SampleTrace::new(),
    timing_trace: TimingTrace::new(),
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Off,
    Bootstrap,
    Foc,
}
#[derive(Clone, Copy)]
struct Plan {
    frame: Frame,
    mode: Mode,
}
impl Plan {
    const fn off() -> Self {
        Self {
            frame: Frame::initial(),
            mode: Mode::Off,
        }
    }
    fn for_state(frame: Frame, state: State) -> Self {
        let mode = match state {
            State::Bootstrap => Mode::Bootstrap,
            State::AlignHold | State::AlignSweep | State::AlignSettle | State::EncoderRun => {
                Mode::Foc
            }
            _ => Mode::Off,
        };
        Self { frame, mode }
    }
    fn duty(&self) -> [u16; 3] {
        if self.mode == Mode::Bootstrap {
            [0; 3]
        } else {
            self.frame.duty
        }
    }
}
struct Motor {
    control: Control,
    modulator: Modulator,
    active: Plan,
    fault_published: bool,
    arithmetic_error: u32,
    staged: Plan,
    pending: Plan,
    pending_ready: bool,
    first_frame: bool,
    samples: usize,
    dc_ma: [i32; 2],
    calibration_count: u32,
    calibration_sum: u32,
    calibration_min: u16,
    calibration_max: u16,
    offset: i32,
    bus_mv: i32,
    vdda_mv: i32,
    reference_mv: i32,
    supply_ready: bool,
    raw_supply: [u16; 2],
    candidate_vdda_mv: i32,
    calibration_reason: u32,
    slow_age: u16,
    slow_pending: bool,
    frames: u32,
    frame_origin: u16,
    last_update_clock: Option<u16>,
    raw: [u16; 2],
    max_control_ticks: u16,
    stage_ticks: [u16; 3],
    max_stage_ticks: [u16; 3],
    checkpoint: u16,
    max_update_ticks: u16,
    armed: bool,
    sample_trace: SampleTrace,
    timing_trace: TimingTrace,
}
impl Motor {
    fn new(reference_mv: u16) -> Self {
        Self {
            control: Control::new(),
            modulator: Modulator::new(),
            active: Plan::off(),
            fault_published: false,
            arithmetic_error: 0,
            staged: Plan::off(),
            pending: Plan::off(),
            pending_ready: true,
            first_frame: true,
            samples: 0,
            dc_ma: [0; 2],
            calibration_count: 0,
            calibration_sum: 0,
            calibration_min: 4095,
            calibration_max: 0,
            offset: 2048,
            bus_mv: 0,
            vdda_mv: ADC_NOMINAL_VDDA_MV,
            reference_mv: i32::from(reference_mv),
            supply_ready: false,
            raw_supply: [0; 2],
            candidate_vdda_mv: 0,
            calibration_reason: 0,
            slow_age: 0,
            slow_pending: true,
            frames: 0,
            frame_origin: 0,
            last_update_clock: None,
            raw: [0; 2],
            max_control_ticks: 0,
            stage_ticks: [0; 3],
            max_stage_ticks: [0; 3],
            checkpoint: 0,
            max_update_ticks: 0,
            armed: false,
            sample_trace: SampleTrace::new(),
            timing_trace: TimingTrace::new(),
        }
    }
    /// ATIM CNT在50us载波边界回绕；BTIM2独立时间戳覆盖整个100us控制帧。
    /// frame_origin在更新IRQ先读BTIM2、后读ATIM CNT建立，读取偏差只缩短预算。
    #[inline(always)]
    fn control_tick(&self) -> u16 {
        unsafe { BasicTimer::<peripherals::BTIM2>::acquire().counter() }
            .wrapping_sub(self.frame_origin)
    }
    #[cold]
    #[inline(never)]
    fn trip_timing(
        &mut self,
        site: u32,
        start: u16,
        observed: u16,
        limit: u16,
        reload_pending: bool,
        pwm: &mut SingleShuntPwm,
    ) {
        self.timing_trace = TimingTrace {
            site,
            state_before: self.control.state() as u32,
            armed_before: self.armed,
            start,
            observed,
            limit,
            samples: self.samples as u16,
            reload_pending,
            elapsed_min: if site == 5 {
                0
            } else {
                elapsed_min(start, observed, reload_pending)
            },
            stage: self.timing_trace.stage,
        };
        self.trip(Fault::Timing, pwm);
    }
    #[inline(always)]
    fn check_arithmetic(&mut self, math: &mut HardwareMath<'_>) -> bool {
        if math.failed {
            self.arithmetic_error = math.error;
            self.trip(Fault::Arithmetic, math.pwm);
            false
        } else {
            true
        }
    }
    #[inline(always)]
    fn checkpoint_control(&mut self, pwm: &mut SingleShuntPwm, start: u16, stage: usize) -> bool {
        let end = self.control_tick();
        let reload = pwm.update_pending();
        self.timing_trace.stage = stage as u16 + 1;
        self.stage_ticks[stage] = elapsed_min(self.checkpoint, end, reload);
        self.sample_trace.previous_irq_end = end;
        if end >= sampling::CONTROL_DEADLINE || end < start || reload {
            self.trip_timing(4, start, end, sampling::CONTROL_DEADLINE, reload, pwm);
            return false;
        }
        self.max_stage_ticks[stage] = self.max_stage_ticks[stage].max(end - self.checkpoint);
        self.checkpoint = end;
        if stage == 2 {
            self.max_control_ticks = self.max_control_ticks.max(end - start);
        }
        true
    }

    fn trip(&mut self, fault: Fault, pwm: &mut SingleShuntPwm) {
        let first = !self.fault_published;
        // 先关桥，再停高频触发和中断；保留 BTIM1，普通线程才能输出首故障。
        // 不清 ADC/ATIM 故障旗标，也不自动重新开启计数器。
        pwm.disarm();
        pwm.stop();
        typelevel::ATIM::disable();
        typelevel::ADC1::disable();
        // None是正常台架结束，先冻结运行态，再转Off；不能伪记Driver故障。
        if fault != Fault::None {
            self.control.trip(fault);
        }
        if first {
            let trace = self.control.fault_trace();
            // 控制器可先锁存 Fault；使用它在覆盖状态之前保留的来源。
            self.timing_trace.state_before = trace.state_before;
            self.timing_trace.armed_before = self.armed;
            unsafe {
                core::ptr::write_volatile(core::ptr::addr_of_mut!(FOC_CONTROL_FAULT), trace);
            }
        }
        self.armed = false;
        RUN_STATUS.store((self.control.fault() as u32) << 8, Ordering::Release);
        self.pending = Plan::off();
        self.pending_ready = true;
        if first {
            self.fault_published = true;
            publish(self);
            if fault == Fault::None {
                self.control.request_output(0);
            }
            FAULT_LOG_READY.store(true, Ordering::Release);
        }
    }
    fn convert_current(&self, raw: u16, math: &mut impl Arithmetic) -> i32 {
        math.div(
            (i32::from(raw) - self.offset)
                * self.vdda_mv
                * (1_000 / (SHUNT_MILLIOHM * CURRENT_GAIN)),
            4095,
        )
    }
    fn current_over_limit(&self, raw: u16) -> bool {
        let numerator = (i32::from(raw) - self.offset)
            * self.vdda_mv
            * (1_000 / (SHUNT_MILLIOHM * CURRENT_GAIN));
        numerator.unsigned_abs() >= (CURRENT_TRIP_MA as u32 + 1) * 4095
    }
    fn calibrate(
        &mut self,
        raw: u16,
        adc: &mut AdcScan<peripherals::ADC1>,
        pwm: &mut SingleShuntPwm,
    ) -> bool {
        // 第一组实测 BGR/VDDA 合格前不使用名义 5 V 校准看门狗或允许启动。
        // ADC2 首次就绪受原有 3 ms 帧龄预算限制；等待期间仍生成完整 Off 计划。
        if !self.supply_ready {
            return true;
        }
        self.calibration_count += 1;
        self.calibration_sum += u32::from(raw);
        self.calibration_min = self.calibration_min.min(raw);
        self.calibration_max = self.calibration_max.max(raw);
        if self.calibration_count == 2048 {
            self.offset = (self.calibration_sum / self.calibration_count) as i32;
            if !(1800..=2300).contains(&self.offset) {
                self.calibration_reason = 1;
                self.trip(Fault::Calibration, pwm);
                return false;
            } else if self.calibration_max - self.calibration_min > 64 {
                self.calibration_reason = 2;
                self.trip(Fault::Calibration, pwm);
                return false;
            } else {
                // 阈值是本示例的实验软件保护值，不是板子额定电流。
                // 当前参数校验保证trip<=2000mA，分子最大819000000，i32足够。
                // 仅在偏置校准完成时计算一次；无需运行时64位除法。
                let delta =
                    CURRENT_TRIP_MA * 4095 * SHUNT_MILLIOHM * CURRENT_GAIN / (self.vdda_mv * 1_000);
                if self.offset - delta < 0 || self.offset + delta > 4095 {
                    self.trip(Fault::Parameters, pwm);
                    return false;
                }
                adc.configure_watchdog(
                    8,
                    (self.offset - delta) as u16,
                    (self.offset + delta) as u16,
                );
                self.control.finish_calibration();
                if self.control.state() == State::Fault {
                    self.trip(self.control.fault(), pwm);
                    return false;
                }
            }
        }
        true
    }
    unsafe fn update_bus(&mut self, math: &mut HardwareMath<'_>) {
        let mut adc = unsafe { AdcScan::<peripherals::ADC2>::acquire() };
        // 工厂标定值不会因等待而恢复；即使首个 EOS 未到也立即拒绝。
        if !(1000..=1500).contains(&self.reference_mv) {
            self.calibration_reason = 3;
            self.trip(Fault::Calibration, math.pwm);
            return;
        }
        if !self.supply_ready
            && (self.control.state() != State::Calibrating
                || self.armed
                || math.pwm.outputs_enabled()
                || self.active.mode != Mode::Off
                || self.staged.mode != Mode::Off
                || self.pending.mode != Mode::Off)
        {
            self.calibration_reason = 6;
            self.trip(Fault::Calibration, math.pwm);
            return;
        }
        if self.slow_pending {
            if let Some(raw) = adc.take_sequence::<2>() {
                self.slow_pending = false;
                self.raw_supply = raw;
                self.candidate_vdda_mv = 0;
                if raw[0] >= 4063 {
                    self.trip(Fault::OverVoltage, math.pwm);
                    return;
                }
                let reference = i32::from(raw[1]);
                let reason = if reference == 0 {
                    4
                } else {
                    self.candidate_vdda_mv = math.div(self.reference_mv * 4095, reference);
                    if !self.check_arithmetic(math) {
                        return;
                    }
                    if !(4500..=5500).contains(&self.candidate_vdda_mv) {
                        5
                    } else {
                        0
                    }
                };
                self.calibration_reason = reason;
                if reason == 0 {
                    self.vdda_mv = self.candidate_vdda_mv;
                    self.bus_mv = math.div(i32::from(raw[0]) * self.vdda_mv * 11, 4095);
                    if !self.check_arithmetic(math) {
                        return;
                    }
                    self.supply_ready = true;
                    self.slow_age = 0;
                } else if self.supply_ready {
                    // 已就绪之后的任何 BGR/VDDA 异常仍在本次采样立即锁存关断。
                    self.trip(Fault::Calibration, math.pwm);
                    return;
                }
                // 初次无效转换不能刷新帧龄或成为有效电压；继续原 1 ms 重采样。
            }
        }
        self.slow_age = self.slow_age.saturating_add(1);
        if self.slow_age > SLOW_ADC_MAX_AGE {
            let fault = if !self.supply_ready && self.calibration_reason != 0 {
                Fault::Calibration
            } else {
                Fault::SampleTimeout
            };
            self.trip(fault, math.pwm);
            return;
        }
        if self.frames % SLOW_ADC_DIVIDER == 0 && !self.slow_pending {
            adc.start_software();
            self.slow_pending = true;
        }
    }
}

/// 仅 ISR 写入；普通线程只在高频源停止、首故障已冻结之后读取。
fn publish(m: &Motor) {
    let s = m.control.snapshot();
    let value = Diagnostics {
        state: s.state as u32,
        fault: s.fault as u32,
        arithmetic_error: m.arithmetic_error,
        calibration_trace: CalibrationTrace {
            reason: m.calibration_reason,
            raw_supply: m.raw_supply,
            reference_mv: m.reference_mv,
            candidate_vdda_mv: m.candidate_vdda_mv,
            supply_ready: m.supply_ready,
            samples: m.calibration_count,
            minimum: m.calibration_min,
            maximum: m.calibration_max,
        },
        bus_mv: m.bus_mv,
        vdda_mv: m.vdda_mv,
        offset_adc: m.offset,
        raw_adc: m.raw,
        sampled_peak_ma: s.sampled_peak_ma,
        current_valid: s.current_valid,
        current_age: s.current_age,
        max_control_ticks: m.max_control_ticks,
        stage_ticks: m.stage_ticks,
        max_stage_ticks: m.max_stage_ticks,
        max_update_ticks: m.max_update_ticks,
        armed: m.armed,
        frames: m.frames,
        filtered_peak_ma: s.filtered_peak_ma,
        voltage_cap_mv: s.voltage_cap_mv,
        voltage_dq_mv: s.voltage_dq_mv,
        angle: s.angle,
        encoder_count: s.encoder_count,
        encoder_delta: s.encoder_delta,
        encoder_direction: s.encoder_direction,
        output_angle: s.output_angle,
        speed_millihz: s.speed_millihz,
        speed_feedback_millihz: s.speed_feedback_millihz,
        sample_trace: m.sample_trace,
        timing_trace: m.timing_trace,
    };
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(FOC_DIAGNOSTICS), value);
    }
}

/// 只在普通线程调用。Release/Acquire 发布后，高频源已停止，快照不会再改写。
/// 非阻塞 RTT 仅输出一次；运行中的每帧采样和 ISR 均不打印。
pub fn report_fault() {
    if !FAULT_LOG_READY.load(Ordering::Acquire) {
        return;
    }
    FAULT_LOG_READY.store(false, Ordering::Relaxed);
    let d = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(FOC_DIAGNOSTICS)) };
    if d.fault == Fault::Calibration as u32 || d.fault == Fault::SampleTimeout as u32 {
        let c = d.calibration_trace;
        defmt::error!(
            "CAL reason={} raw_bus={} raw_bgr={} factory_mv={} candidate_vdda={} ready={} samples={} min={} max={} source=boot_pre_accel_u16@0x001007D2",
            c.reason, c.raw_supply[0], c.raw_supply[1], c.reference_mv,
            c.candidate_vdda_mv, c.supply_ready, c.samples, c.minimum, c.maximum,
        );
        defmt::error!("CAL reason: 0=none 1=offset 2=noise 3=factory 4=zero_bgr 5=vdda_range 6=startup_not_off; pre-ready invalid sample has 3ms deadline, post-ready faults immediately");
    }
    defmt::info!("VOLTAGE current_sample_peak={}mA filtered_peak={}mA cap={}mV output_dq={:?}mV valid={} age={}frames; natural-window sample, not Iq", d.sampled_peak_ma, d.filtered_peak_ma, d.voltage_cap_mv, d.voltage_dq_mv, d.current_valid, d.current_age);
    let t = d.sample_trace;
    defmt::error!(
        "08 fault={} state={} frame={} armed={} bus={}mV vdda={}mV offset={} raw={:?}; PWM/ADC triggers stopped",
        d.fault, d.state, d.frames, d.armed, d.bus_mv, d.vdda_mv, d.offset_adc, d.raw_adc,
    );
    let result = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(FOC_CONTROL_FAULT)) };
    defmt::info!("ENCODER state_before={} age={} raw={} delta={} direction={} angle={} output_angle={} speed={}mHz filtered={}mHz alignment_travel={} voltage_dq_mv={:?}; state 3=hold 4=sweep 5=settle 6=closed", result.state_before, result.age_frames, d.encoder_count, d.encoder_delta, d.encoder_direction, d.angle, d.output_angle, d.speed_millihz, d.speed_feedback_millihz, result.alignment_travel, result.previous_voltage_dq_mv);
    defmt::error!(
        "ADC sample={} reason={} entry={} read={} raw={} expected={}..={} trigger={:?} received={:?} previous_end={} max_first={} ticks",
        t.index, t.reason, t.irq_entry, t.result_read, t.raw, t.expected_min, t.expected_max,
        t.trigger, t.received, t.previous_irq_end, t.max_first_irq_ticks,
    );
    defmt::error!(
        "ADC reason: 0=other 1=reload_pending 2=extra_sample 3=early 4=late; 96 ticks/us"
    );
    if d.arithmetic_error != 0 {
        defmt::error!("EAU error={} (1=divide_zero 2=overflow 3=busy 4=timeout 5=poll_budget); PWM stopped before return", d.arithmetic_error);
    }
    let t = d.timing_trace;
    defmt::error!(
        "TIMING site={} start={} observed={} limit={} reload={} samples={} state_before={} armed_before={} max_update={} max_control={} ticks elapsed_min={} stage={}",
        t.site, t.start, t.observed, t.limit, t.reload_pending, t.samples,
        t.state_before, t.armed_before,
        d.max_update_ticks, d.max_control_ticks, t.elapsed_min, t.stage,
    );
    defmt::error!("CONTROL stages supply/voltage_control/modulate: last={:?} max={:?} ticks; max excludes wrapping/late stage; elapsed_min is a lower bound when reload=true", d.stage_ticks, d.max_stage_ticks);
    defmt::error!(
        "TIMING site: 0=none 1=reload_entry 2=preload_end 3=arm_end 4=control_checkpoint 5=encoder_time (tick<9600 dt=4800..=14400) 6=update_period (9600+/-700); 96 ticks/us"
    );
}

/// 同一次原子读取给出解锁状态和首个锁存故障码；0 表示无故障。
pub fn running_and_fault_code() -> (bool, u8) {
    let status = RUN_STATUS.load(Ordering::Acquire);
    (status & 1 != 0, (status >> 8) as u8)
}

#[embassy_executor::task]
pub async fn motor_task(resources: MotorResources, factory_reference_mv: u16) {
    typelevel::ATIM::disable();
    typelevel::ADC1::disable();
    typelevel::BTIM1::disable();
    unsafe {
        for pin in GATES {
            MotorPin::acquire(pin).configure(PinMode::OutputLow, 0);
        }
        for pin in [
            PinId::new(Port::A, 6),
            PinId::new(Port::A, 7),
            PinId::new(Port::B, 0),
            PinId::new(Port::A, 8),
        ] {
            MotorPin::acquire(pin).configure(PinMode::Analog, 0);
        }
        AdcScan::<peripherals::ADC1>::acquire().configure(ScanConfig {
            slots: &[ScanSlot::new(8, SampleTime::Cycles70)],
            divider: ClockDivider::Div2,
        });
        AdcScan::<peripherals::ADC2>::acquire().configure(ScanConfig {
            slots: &[
                ScanSlot::new(5, SampleTime::Cycles518),
                ScanSlot::new(15, SampleTime::Cycles518),
            ],
            divider: ClockDivider::Div8,
        });
        motor::configure_current_sense(PositiveInput::Inp2, NegativeInput::Inn2);
        let initial = Frame::initial();
        SingleShuntPwm::acquire().configure_center_aligned(
            PWM_HALF_TICKS,
            sampling::DEAD_TICKS as u8,
            initial.duty,
            initial.sample,
            PWM_PER_CONTROL,
        );
        BasicTimer::<peripherals::BTIM2>::acquire().configure(TimerConfig {
            prescaler: 0,
            reload: u16::MAX,
        });
        BasicTimer::<peripherals::BTIM1>::acquire().configure(TimerConfig {
            prescaler: 95,
            reload: 999,
        });
    }
    // 启动前只读核对载波与更新分频，尚未运行高频ISR。
    assert_eq!(embassy_cw32::pac::ATIM.arr().read().arr(), PWM_HALF_TICKS);
    assert_eq!(
        embassy_cw32::pac::ATIM.rcr().read().0,
        u32::from(PWM_PER_CONTROL * 2 - 1)
    );
    defmt::info!(
        "TIMERS PWM ARR={} RCR={} control_ticks={}; center seven-segment; ADC single up-count sample; BTIM2 deadline clock",
        PWM_HALF_TICKS,
        PWM_PER_CONTROL * 2 - 1,
        CONTROL_TICKS
    );
    cortex_m::asm::delay(CPU_HZ / 1000);
    unsafe {
        core::ptr::addr_of_mut!(MOTOR).write(Some(Runtime {
            // 使用缓存启用前传入的原始半字；无效值仍走 CAL reason=3。
            motor: Motor::new(factory_reference_mv),
            eau: Eau::new(resources.eau),
            // PB14/PB15 AF6，由类型路由选择；无 Z，不开逐边沿中断。
            encoder: Qei::new(
                resources.gtim2,
                resources.pb14,
                resources.pb15,
                QeiConfig {
                    max_count: (ENCODER_CPR - 1) as u16,
                    first_filter: Filter::PclkSamples4,
                    second_filter: Filter::PclkSamples4,
                    ..Default::default()
                },
            )
            .unwrap(),
        }));
        let encoder_arr = embassy_cw32::pac::GTIM2.arr().read().arr();
        assert_eq!(encoder_arr, (ENCODER_CPR - 1) as u16);
        defmt::info!(
            "GTIM2 encoder ARR={} verified: hardware count wraps 0..4095; incremental origin, no Z",
            encoder_arr
        );
        for pin in GATES {
            MotorPin::acquire(pin).alternate_function(7);
        }
        AdcScan::<peripherals::ADC1>::acquire().clear_events();
        AdcScan::<peripherals::ADC1>::acquire().enable_sequence_interrupt::<AdcHandler>(Irqs);
        // 首次真实更新前不采样；每个更新IRQ只开放首载波上升半周的一次触发。
        AdcScan::<peripherals::ADC2>::acquire().start_software();
        SingleShuntPwm::acquire().enable_update_interrupt::<PwmHandler>(Irqs);
        BasicTimer::<peripherals::BTIM1>::acquire().enable_update_interrupt::<TickHandler>(Irqs);
        typelevel::ATIM::set_priority(interrupt::Priority::P0);
        typelevel::ADC1::set_priority(interrupt::Priority::P0);
        typelevel::BTIM1::set_priority(interrupt::Priority::P3);
        typelevel::ATIM::unpend();
        typelevel::ADC1::unpend();
        typelevel::BTIM1::unpend();
        INITIALIZED.store(true, Ordering::Release);
        BasicTimer::<peripherals::BTIM2>::acquire().start();
        SingleShuntPwm::acquire().start();
        BasicTimer::<peripherals::BTIM1>::acquire().start();
        typelevel::ATIM::enable();
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
    }
    // 单例归此任务永久保留；高频闭环由同步 ISR 运行，不建轮询执行器。
    core::hint::black_box(&resources.atim);
    core::future::pending::<()>().await;
}

// UIF 不能计数多个回绕；发生 reload 时只报告至少一个周期的下界。
fn elapsed_min(start: u16, end: u16, reload: bool) -> u16 {
    if end < start {
        CONTROL_TICKS - start + end
    } else if reload {
        // CNT 可能先在周期末读出，UIF 随后才置位；不能把旧 CNT 再加一周期。
        CONTROL_TICKS - start
    } else {
        end - start
    }
}

pub struct PwmHandler;
impl Handler<typelevel::ATIM> for PwmHandler {
    unsafe fn on_interrupt() {
        unsafe {
            let mut pwm = SingleShuntPwm::acquire();
            if !pwm.take_update() {
                return;
            }
            let runtime = (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap();
            let m = &mut runtime.motor;
            let epoch_clock = BasicTimer::<peripherals::BTIM2>::acquire().counter();
            // 检查UEV位于底部，中心CNT不能直接用作时间。
            let down = embassy_cw32::pac::ATIM.cr1().read().dir();
            let count = pwm.counter();
            let start = if down {
                PWM_TICKS.saturating_sub(count)
            } else {
                count
            };
            m.frame_origin = epoch_clock.wrapping_sub(start);
            if start > sampling::UPDATE_DEADLINE {
                m.trip_timing(
                    1,
                    start,
                    start,
                    sampling::UPDATE_DEADLINE,
                    pwm.update_pending(),
                    &mut pwm,
                );
                return;
            }
            // 用独立时钟验证RCR确实每100us更新；错误分频在校准期即锁存。
            if let Some(previous) = m.last_update_clock {
                let dt = epoch_clock.wrapping_sub(previous);
                if !(CONTROL_TICKS - sampling::UPDATE_DEADLINE
                    ..=CONTROL_TICKS + sampling::UPDATE_DEADLINE)
                    .contains(&dt)
                {
                    m.trip_timing(
                        6,
                        0,
                        dt,
                        CONTROL_TICKS + sampling::UPDATE_DEADLINE,
                        pwm.update_pending(),
                        &mut pwm,
                    );
                    return;
                }
            }
            m.last_update_clock = Some(epoch_clock);
            if !m.first_frame && (m.samples != 1 || !m.pending_ready) {
                m.trip(Fault::SampleTimeout, &mut pwm);
                return;
            }
            m.first_frame = false;
            if pwm.fault_pending() || (m.armed && !pwm.outputs_enabled()) {
                m.trip(Fault::Driver, &mut pwm);
                return;
            }
            // 硬件已经 reload：软件身份必须先晋升旧 staged，绝不把 pending 配给旧波形。
            m.active = m.staged;
            m.samples = 0;
            m.frames = m.frames.wrapping_add(1);
            let requested = OUTPUT_REQUEST_PERCENT.load(Ordering::Acquire) as i32;
            m.control.request_output(requested);
            if matches!(
                m.control.state(),
                State::Off | State::Calibrating | State::Fault
            ) {
                // 正常待机的 pending 已在 ADC 尾部准备好；只在停止过渡时覆盖。
                if m.pending.mode != Mode::Off {
                    m.pending = Plan::off();
                }
                m.active.mode = Mode::Off;
                pwm.disarm();
                m.armed = false;
            }
            // 只在 reload 后的保留时隙写五个预载；禁止运行时 UG/临时 UDIS。
            m.staged = m.pending;
            m.pending_ready = false;
            pwm.stage(m.staged.duty(), [m.staged.frame.sample, 0]);
            let staged_at = m.control_tick();
            let reload_pending = pwm.update_pending();
            if staged_at > sampling::UPDATE_DEADLINE || staged_at < start || reload_pending {
                m.trip_timing(
                    2,
                    start,
                    staged_at,
                    sampling::UPDATE_DEADLINE,
                    reload_pending,
                    &mut pwm,
                );
                return;
            }
            // 上方已将 Off/Calibrating/Fault 的 active.mode 置 Off；其后控制状态
            // 不再改变，P0 ISR 也不嵌套。无需在首次解锁路径重复读取/判断 Fault。
            if m.active.mode != Mode::Off && !m.armed {
                if pwm.arm().is_err() {
                    m.trip(Fault::Driver, &mut pwm);
                    return;
                } else {
                    m.armed = true;
                }
            }
            // RCR=3每四次半周更新；只在首载波上升半周接通一次ADC。
            AdcScan::<peripherals::ADC1>::acquire().trigger_from_atim_trgo2();
            let armed_at = m.control_tick();
            let reload_pending = pwm.update_pending();
            if armed_at > sampling::UPDATE_DEADLINE || armed_at < start || reload_pending {
                m.trip_timing(
                    3,
                    start,
                    armed_at,
                    sampling::UPDATE_DEADLINE,
                    reload_pending,
                    &mut pwm,
                );
                return;
            }
            m.max_update_ticks = m.max_update_ticks.max(armed_at.wrapping_sub(start));
            RUN_STATUS.store(
                m.armed as u32 | (m.control.fault() as u32) << 8,
                Ordering::Release,
            );
        }
    }
}

pub struct AdcHandler;
impl Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        unsafe {
            let mut pwm = SingleShuntPwm::acquire();
            let runtime = (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap();
            let m = &mut runtime.motor;
            let irq_entry = m.control_tick();
            let mut adc = AdcScan::<peripherals::ADC1>::acquire();
            let Some(raw) = adc.take_single() else {
                return;
            };
            // 仍以 RESULT0 读取/确认之后的时间判定，不用更早的 entry 放宽保护。
            let start = m.control_tick();
            let index = m.samples;
            let reason = if m.control_tick() >= PWM_TICKS {
                4 // 第二载波禁止采样；不能将CNT回绕后的样本冒认为首载波。
            } else {
                m.active
                    .frame
                    .sample_fault_reason(index, start, pwm.update_pending())
            };
            if reason != 0 {
                // 故障才填充完整证据，避免每个正常样本额外搬运大快照。
                let trace = &mut m.sample_trace;
                trace.reason = reason;
                trace.index = index as u32;
                trace.irq_entry = irq_entry;
                trace.result_read = start;
                trace.raw = raw;
                trace.trigger = [m.active.frame.sample, 0];
                trace.expected_min = if index == 0 {
                    m.active.frame.sample + sampling::CONVERSION_TICKS
                } else {
                    0
                };
                trace.expected_max = trace.expected_min + sampling::SAMPLE_IRQ_SLACK;
                m.trip(Fault::SampleInvalid, &mut pwm);
                return;
            }
            m.sample_trace.received[index] = start;
            m.samples += 1;
            m.raw[index] = raw;
            if m.control.state() == State::Calibrating && !m.calibrate(raw, &mut adc, &mut pwm) {
                return;
            }
            // 每个已解锁样本仍即时检查电源轨、过流和硬件看门狗。向零截断的
            // |numerator / 4095| > trip 等价于 |numerator| >= (trip + 1) * 4095；
            // 电流换算在原始硬过流检查后进行。
            if m.armed
                && (raw < 32 || raw > 4063 || m.current_over_limit(raw) || adc.watchdog_pending())
            {
                m.trip(Fault::OverCurrent, &mut pwm);
                return;
            }
            // 唯一EOS后立刻断开触发，后一个载波不重复采样。
            adc.disable_external_triggers();
            let start = m.control_tick();
            m.checkpoint = start;
            m.stage_ticks = [0; 3];
            // EOS后读编码器；时间戳来自BTIM2，不使用三角波CNT。
            let encoder_count = runtime.encoder.count();
            let encoder_tick = m.control_tick();
            let mut math = HardwareMath {
                eau: &mut runtime.eau,
                pwm: &mut pwm,
                failed: false,
                error: 0,
            };
            // 只换算自然窗口有效电流；零矢量兜底不作为绕组电流。
            m.dc_ma = if m.armed {
                [
                    if m.active.frame.current_valid {
                        m.convert_current(m.raw[0], &mut math)
                    } else {
                        0
                    },
                    0, // 保留诊断布局，本版没有第二次采样。
                ]
            } else {
                [0; 2]
            };
            // 慢速 ADC2 的除法放在唯一EOS后的控制时隙，不能占用 reload
            // 后 700 tick 的五路 CCR 预载/解锁预算。仍在本帧控制截止前检查。
            if !m.check_arithmetic(&mut math) {
                return;
            }
            m.update_bus(&mut math);
            if !m.check_arithmetic(&mut math) {
                return;
            }
            if m.control.state() == State::Fault {
                return;
            }
            // 电压控制只接受已验证的供电范围；生成电压请求前拦截异常母线。
            // Off/Calibrating 仍允许未接母线，启动/运行沿用原欠压和过压门限。
            if matches!(
                m.control.state(),
                State::Bootstrap
                    | State::AlignHold
                    | State::AlignSweep
                    | State::AlignSettle
                    | State::EncoderRun
            ) {
                if m.bus_mv > BUS_MAX_MV {
                    m.trip(Fault::OverVoltage, math.pwm);
                    return;
                }
                if m.bus_mv < BUS_MIN_MV {
                    m.trip(Fault::UnderVoltage, math.pwm);
                    return;
                }
            }
            if !m.checkpoint_control(math.pwm, start, 0) {
                return;
            }
            // 首自然窗口提供母线瞬时电流，带显式有效性传给粗限流。
            // 不合成三相电流，不做跨采样时刻RL运输，也不做Id/Iq PI。
            let sampled_peak_ma = m.dc_ma[0].abs();
            let requested = m.control.on_frame_timed(
                m.active.frame.current_valid.then_some(sampled_peak_ma),
                m.bus_mv,
                (encoder_count, encoder_tick),
                &mut math,
            );
            if !m.check_arithmetic(&mut math) {
                return;
            }
            if m.control.state() == State::Fault {
                if m.control.fault() == Fault::Timing {
                    m.trip_timing(
                        5,
                        0,
                        encoder_tick,
                        CONTROL_TICKS,
                        math.pwm.update_pending(),
                        math.pwm,
                    );
                } else {
                    m.trip(m.control.fault(), math.pwm);
                }
                return;
            }
            if !m.checkpoint_control(math.pwm, start, 1) {
                return;
            }
            let frame = if matches!(
                m.control.state(),
                State::AlignHold | State::AlignSweep | State::AlignSettle | State::EncoderRun
            ) {
                match m.modulator.plan(requested, m.bus_mv, &mut math) {
                    Some(f) => f,
                    None => {
                        if !m.check_arithmetic(&mut math) {
                            return;
                        }
                        m.trip(Fault::SampleInvalid, math.pwm);
                        return;
                    }
                }
            } else {
                Frame::initial()
            };
            if !m.check_arithmetic(&mut math) {
                return;
            }
            m.pending = Plan::for_state(frame, m.control.state());
            m.pending_ready = true;
            if LIVE_STATE.load(Ordering::Acquire) == 1 {
                let s = m.control.snapshot();
                let words = [
                    s.state as u32,
                    s.speed_millihz as u32,
                    s.speed_feedback_millihz as u32,
                    s.voltage_dq_mv[0] as u32,
                    s.voltage_dq_mv[1] as u32,
                    s.voltage_cap_mv as u32,
                    s.filtered_peak_ma as u32,
                    s.current_valid as u32,
                    s.current_age,
                    OUTPUT_REQUEST_PERCENT.load(Ordering::Relaxed),
                ];
                for (dst, value) in LIVE_WORDS.iter().zip(words) {
                    dst.store(value, Ordering::Relaxed);
                }
                LIVE_STATE.store(2, Ordering::Release);
            }
            // 完整诊断只在trip关桥后发布；运行帧仅更新Motor内的峰值。
            // 没有运行态读者，周期搬运整份故障快照会制造50ms耗时尖峰。
            m.checkpoint_control(math.pwm, start, 2);
        }
    }
}

pub struct TickHandler;
impl Handler<typelevel::BTIM1> for TickHandler {
    unsafe fn on_interrupt() {
        if !unsafe { BasicTimer::<peripherals::BTIM1>::acquire() }.take_update() {
            return;
        }
        MILLISECONDS.store(
            MILLISECONDS.load(Ordering::Relaxed).wrapping_add(1),
            Ordering::Release,
        );
        crate::io::wake_tasks();
    }
}

pub fn fatal() -> ! {
    cortex_m::interrupt::disable();
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe {
            SingleShuntPwm::acquire().disarm();
            for pin in GATES {
                MotorPin::acquire(pin).configure(PinMode::OutputLow, 0);
            }
        }
    }
    loop {
        cortex_m::asm::wfi();
    }
}
