//! 单电阻硬件时序。P0 的 ATIM/ADC1 ISR 同级且不嵌套；控制计算不进入异步任务。
use crate::{
    config::*,
    control::{Control, Fault, State},
    sampling::{self, Frame, Modulator},
    Irqs,
};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use embassy_cw32::{
    gpio::Port,
    interrupt::{
        self,
        typelevel::{self, Handler, Interrupt as _},
    },
    motor::{
        self, AdcScan, BasicTimer, ClockDivider, MotorPin, NegativeInput, PinId, PinMode,
        PositiveInput, SampleTime, ScanConfig, ScanSlot, SingleShuntPwm, TimerConfig,
    },
    peripherals, Peri,
};

/// 字段用于独占保留单例，避免其他驱动复用电机引脚和外设。
#[allow(dead_code)]
pub struct MotorResources {
    pub atim: Peri<'static, peripherals::ATIM>,
    pub adc1: Peri<'static, peripherals::ADC1>,
    pub adc2: Peri<'static, peripherals::ADC2>,
    pub opa1: Peri<'static, peripherals::OPA1>,
    pub bgr: Peri<'static, peripherals::BGR>,
    pub btim1: Peri<'static, peripherals::BTIM1>,
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
pub static RUN_REQUEST: AtomicBool = AtomicBool::new(false);
pub static MILLISECONDS: AtomicU32 = AtomicU32::new(0);
// 全部实时状态只有 P0 中断访问；线程只读独立原子状态字。
static mut MOTOR: Option<Motor> = None;
static RUN_STATUS: AtomicU32 = AtomicU32::new(0);
/// 调试器在物理断开母线并暂停 CPU 后读取；没有 RTT/UART 输出。
/// 运行中外部读取可能跨越一次更新；不允许其他 Rust 任务借用此对象。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Diagnostics {
    pub state: u32,
    pub fault: u32,
    pub bus_mv: i32,
    pub vdda_mv: i32,
    pub offset_adc: i32,
    pub raw_adc: [u16; 2],
    pub phase_ma: [i32; 3],
    pub max_control_ticks: u16,
    pub max_update_ticks: u16,
    pub armed: bool,
    pub frames: u32,
    pub current_dq_ma: [i32; 2],
    pub angle: u16,
    pub observer_angle: u16,
    pub speed_millihz: i32,
    pub bemf_mv: i32,
    pub pll_error: i32,
    pub qualified_frames: u32,
}
#[no_mangle]
pub static mut FOC_DIAGNOSTICS: Diagnostics = Diagnostics {
    state: 0,
    fault: 0,
    bus_mv: 0,
    vdda_mv: 0,
    offset_adc: 0,
    raw_adc: [0; 2],
    phase_ma: [0; 3],
    max_control_ticks: 0,
    max_update_ticks: 0,
    armed: false,
    frames: 0,
    current_dq_ma: [0; 2],
    angle: 0,
    observer_angle: 0,
    speed_millihz: 0,
    bemf_mv: 0,
    pll_error: 0,
    qualified_frames: 0,
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
            State::Align | State::OpenLoop | State::Blend | State::ClosedLoop => Mode::Foc,
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
    previous: Plan,
    previous_bus_mv: i32,
    fault_published: bool,
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
    slow_age: u16,
    slow_pending: bool,
    frames: u32,
    raw: [u16; 2],
    phase_ma: [i32; 3],
    max_control_ticks: u16,
    max_update_ticks: u16,
    armed: bool,
}
impl Motor {
    fn new(reference_mv: u16) -> Self {
        Self {
            control: Control::new(),
            modulator: Modulator::new(),
            active: Plan::off(),
            previous: Plan::off(),
            previous_bus_mv: 0,
            fault_published: false,
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
            slow_age: 0,
            slow_pending: true,
            frames: 0,
            raw: [0; 2],
            phase_ma: [0; 3],
            max_control_ticks: 0,
            max_update_ticks: 0,
            armed: false,
        }
    }
    fn trip(&mut self, fault: Fault, pwm: &mut SingleShuntPwm) {
        let first = !self.fault_published;
        self.control.trip(fault);
        self.armed = false;
        RUN_STATUS.store((self.control.fault() as u32) << 8, Ordering::Release);
        self.pending = Plan::off();
        self.pending_ready = true;
        // 故障时刻立即关断 MOE；不等待下一次 update，也不自动复位错误旗标。
        pwm.disarm();
        if first {
            self.fault_published = true;
            publish(self);
        }
    }
    fn convert_current(&self, raw: u16) -> i32 {
        (i32::from(raw) - self.offset) * self.vdda_mv * (1_000 / (SHUNT_MILLIOHM * CURRENT_GAIN))
            / 4095
    }
    fn calibrate(
        &mut self,
        raw: u16,
        adc: &mut AdcScan<peripherals::ADC1>,
        pwm: &mut SingleShuntPwm,
    ) {
        self.calibration_count += 1;
        self.calibration_sum += u32::from(raw);
        self.calibration_min = self.calibration_min.min(raw);
        self.calibration_max = self.calibration_max.max(raw);
        if self.calibration_count == 2048 {
            self.offset = (self.calibration_sum / self.calibration_count) as i32;
            if !(1800..=2300).contains(&self.offset)
                || self.calibration_max - self.calibration_min > 64
            {
                self.trip(Fault::Calibration, pwm);
            } else {
                // 阈值是本示例的实验软件保护值，不是板子额定电流。
                let delta = (i64::from(CURRENT_TRIP_MA)
                    * 4095
                    * i64::from(SHUNT_MILLIOHM)
                    * i64::from(CURRENT_GAIN)
                    / i64::from(self.vdda_mv * 1_000)) as i32;
                if self.offset - delta < 0 || self.offset + delta > 4095 {
                    self.trip(Fault::Parameters, pwm);
                    return;
                }
                adc.configure_watchdog(
                    8,
                    (self.offset - delta) as u16,
                    (self.offset + delta) as u16,
                );
                self.control.finish_calibration();
            }
        }
    }
    unsafe fn update_bus(&mut self, pwm: &mut SingleShuntPwm) {
        let mut adc = unsafe { AdcScan::<peripherals::ADC2>::acquire() };
        if self.slow_pending {
            if let Some(raw) = adc.take_sequence::<2>() {
                self.slow_pending = false;
                self.slow_age = 0;
                let reference = i32::from(raw[1]);
                if reference == 0 || !(1000..=1500).contains(&self.reference_mv) {
                    self.trip(Fault::Calibration, pwm);
                    return;
                }
                let vdda_mv = self.reference_mv * 4095 / reference;
                if !(4500..=5500).contains(&vdda_mv) {
                    self.trip(Fault::Calibration, pwm);
                    return;
                }
                self.vdda_mv = vdda_mv;
                self.bus_mv = i32::from(raw[0]) * self.vdda_mv * 11 / 4095;
                if raw[0] >= 4063 {
                    self.trip(Fault::OverVoltage, pwm);
                }
            }
        }
        self.slow_age = self.slow_age.saturating_add(1);
        if self.slow_age > 30 {
            self.trip(Fault::SampleTimeout, pwm);
        }
        if self.frames % 10 == 0 && !self.slow_pending {
            adc.start_software();
            self.slow_pending = true;
        }
    }
}

/// 仅 ISR 写入，不与任何 Rust 线程读取者共享，避免保护大快照而屏蔽高频采样。
fn publish(m: &Motor) {
    let s = m.control.snapshot();
    let value = Diagnostics {
        state: s.state as u32,
        fault: s.fault as u32,
        bus_mv: m.bus_mv,
        vdda_mv: m.vdda_mv,
        offset_adc: m.offset,
        raw_adc: m.raw,
        phase_ma: m.phase_ma,
        max_control_ticks: m.max_control_ticks,
        max_update_ticks: m.max_update_ticks,
        armed: m.armed,
        frames: m.frames,
        current_dq_ma: s.current_dq_ma,
        angle: s.angle,
        observer_angle: s.observer_angle,
        speed_millihz: s.speed_millihz,
        bemf_mv: s.bemf_mv,
        pll_error: s.pll_error,
        qualified_frames: s.qualified_frames,
    };
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(FOC_DIAGNOSTICS), value);
    }
}

/// 同一次原子读取给出解锁状态和首个锁存故障码；0 表示无故障。
pub fn running_and_fault_code() -> (bool, u8) {
    let status = RUN_STATUS.load(Ordering::Acquire);
    (status & 1 != 0, (status >> 8) as u8)
}

#[embassy_executor::task]
pub async fn motor_task(resources: MotorResources) {
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
        SingleShuntPwm::acquire().configure(
            PWM_TICKS,
            sampling::DEAD_TICKS as u8,
            initial.duty,
            initial.sample,
        );
        BasicTimer::<peripherals::BTIM1>::acquire().configure(TimerConfig {
            prescaler: 95,
            reload: 999,
        });
    }
    cortex_m::asm::delay(CPU_HZ / 1000);
    unsafe {
        core::ptr::addr_of_mut!(MOTOR).write(Some(Motor::new(motor::factory_reference_mv())));
        for pin in GATES {
            MotorPin::acquire(pin).alternate_function(7);
        }
        AdcScan::<peripherals::ADC1>::acquire().clear_events();
        AdcScan::<peripherals::ADC1>::acquire().enable_sequence_interrupt::<AdcHandler>(Irqs);
        AdcScan::<peripherals::ADC1>::acquire().trigger_from_atim_trgo2();
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
        SingleShuntPwm::acquire().start();
        BasicTimer::<peripherals::BTIM1>::acquire().start();
        typelevel::ATIM::enable();
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
    }
    // 单例归此任务永久保留；高频闭环由同步 ISR 运行，不建轮询执行器。
    core::hint::black_box(&resources);
    core::future::pending::<()>().await;
}

pub struct PwmHandler;
impl Handler<typelevel::ATIM> for PwmHandler {
    unsafe fn on_interrupt() {
        unsafe {
            let mut pwm = SingleShuntPwm::acquire();
            if !pwm.take_update() {
                return;
            }
            let m = (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap();
            let start = pwm.counter();
            if start > sampling::UPDATE_DEADLINE {
                m.trip(Fault::Timing, &mut pwm);
            }
            if !m.first_frame && (m.samples != 2 || !m.pending_ready) {
                m.trip(Fault::SampleTimeout, &mut pwm);
            }
            m.first_frame = false;
            if pwm.fault_pending() || (m.armed && !pwm.outputs_enabled()) {
                m.trip(Fault::Driver, &mut pwm);
            }
            // 硬件已经 reload：软件身份必须先晋升旧 staged，绝不把 pending 配给旧波形。
            m.previous = m.active;
            m.active = m.staged;
            m.samples = 0;
            m.frames = m.frames.wrapping_add(1);
            m.control.request_run(RUN_REQUEST.load(Ordering::Acquire));
            if matches!(
                m.control.state(),
                State::Off | State::Calibrating | State::Fault
            ) {
                m.pending = Plan::off();
                m.active.mode = Mode::Off;
                m.modulator.reset();
                pwm.disarm();
                m.armed = false;
            }
            // 只在 reload 后的保留时隙写五个预载；禁止运行时 UG/临时 UDIS。
            m.staged = m.pending;
            m.pending_ready = false;
            pwm.stage(m.staged.duty(), m.staged.frame.sample);
            if pwm.counter() > sampling::UPDATE_DEADLINE || pwm.update_pending() {
                m.trip(Fault::Timing, &mut pwm);
            }
            if m.active.mode != Mode::Off && m.control.state() != State::Fault && !m.armed {
                if pwm.arm().is_err() {
                    m.trip(Fault::Driver, &mut pwm);
                } else {
                    m.armed = true;
                }
            }
            if pwm.counter() > sampling::UPDATE_DEADLINE || pwm.update_pending() {
                m.trip(Fault::Timing, &mut pwm);
            }
            m.max_update_ticks = m.max_update_ticks.max(pwm.counter().wrapping_sub(start));
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
            let mut adc = AdcScan::<peripherals::ADC1>::acquire();
            let Some(raw) = adc.take_single() else {
                return;
            };
            let mut pwm = SingleShuntPwm::acquire();
            let start = pwm.counter();
            let m = (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap();
            if pwm.update_pending() || !m.active.frame.accepts(m.samples, start) {
                m.trip(Fault::SampleInvalid, &mut pwm);
                return;
            }
            let index = m.samples;
            m.samples += 1;
            m.raw[index] = raw;
            if m.control.state() == State::Calibrating {
                m.calibrate(raw, &mut adc, &mut pwm);
            }
            let current = m.convert_current(raw);
            if m.armed
                && (raw < 32
                    || raw > 4063
                    || current.abs() > CURRENT_TRIP_MA
                    || adc.watchdog_pending())
            {
                m.trip(Fault::OverCurrent, &mut pwm);
            }
            m.dc_ma[index] = current;
            if index == 0 {
                return;
            }
            // 慢速 ADC2 的除法放在第二样本后的控制时隙，不能占用 reload
            // 后 700 tick 的五路 CCR 预载/解锁预算。仍在本帧控制截止前检查。
            m.update_bus(&mut pwm);
            m.phase_ma = if m.active.mode == Mode::Foc {
                m.active
                    .frame
                    .reconstruct_timed(m.dc_ma, m.bus_mv, m.control.estimated_emf_mv())
            } else {
                [0; 3]
            };
            let (applied, interval_ticks) = m.active.frame.interval_voltage(
                &m.previous.frame,
                if m.previous.mode == Mode::Foc {
                    m.previous.frame.duty
                } else {
                    [0; 3]
                },
                if m.active.mode == Mode::Foc {
                    m.active.frame.duty
                } else {
                    [0; 3]
                },
                m.previous_bus_mv,
                m.bus_mv,
            );
            m.previous_bus_mv = m.bus_mv;
            let requested =
                m.control
                    .on_frame_timed(m.phase_ma, applied, m.bus_mv, true, interval_ticks);
            let frame = if matches!(
                m.control.state(),
                State::Align | State::OpenLoop | State::Blend | State::ClosedLoop
            ) {
                match m.modulator.plan(requested, m.bus_mv) {
                    Some(f) => f,
                    None => {
                        m.trip(Fault::SampleInvalid, &mut pwm);
                        Frame::initial()
                    }
                }
            } else {
                Frame::initial()
            };
            m.pending = Plan::for_state(frame, m.control.state());
            m.pending_ready = true;
            if m.frames % 500 == 0 {
                publish(m);
            }
            let end = pwm.counter();
            m.max_control_ticks = m.max_control_ticks.max(end.wrapping_sub(start));
            if end >= sampling::CONTROL_DEADLINE || end < start || pwm.update_pending() {
                m.trip(Fault::Timing, &mut pwm);
            }
            if m.control.state() == State::Fault {
                m.trip(m.control.fault(), &mut pwm);
            }
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
