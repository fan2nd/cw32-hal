#![deny(unsafe_code)]
//! 独立 FOC 实验参数。用户提供两电机线端间 R/L；等效模型及控制整定尚未经动态实板验证。

// 普通构建包含按键启动和有资格无感接管；上电先保持桥臂关闭并校准。
// 已采用用户线间测量值的等效模型，仍需限流、空载台架验证。

pub const CPU_HZ: u32 = 96_000_000;
pub const PWM_HZ: u32 = 2500;
pub const CONTROL_HZ: u32 = PWM_HZ;
pub const PWM_TICKS: u16 = (CPU_HZ / PWM_HZ) as u16;
pub const ADC_NOMINAL_VDDA_MV: i32 = 5000;
pub const ADC_FULL_SCALE: i32 = 4095;
pub const SHUNT_MILLIOHM: i32 = 10;
pub const CURRENT_GAIN: i32 = 10;
pub const BUS_DIVIDER: i32 = 11;

// MYH-4621F表列DC12 V/1.1 A测试工况，未区分母线/相电流；以下为台架指令配置。
pub const CURRENT_TRIP_MA: i32 = 1800;
pub const CURRENT_SUM_LIMIT_MA: i32 = 200;
// 按实际电池/台架配置选择 2S 或 3S；不靠电压自动识别，也不在欠压时降档。
// 3300 mV/节沿用原 2S 的 6.6 V 实验起点，须按实际电芯和负载确认，不代替 BMS。
pub const BATTERY_SERIES_CELLS: u8 = 3;
pub const CELL_UNDERVOLTAGE_MV: i32 = 3300;
pub const BUS_MIN_MV: i32 = BATTERY_SERIES_CELLS as i32 * CELL_UNDERVOLTAGE_MV;
// 保留原 16 V 实验过压限制；电池串数不提高功率板额定值或证明驱动低压能力。
pub const BUS_MAX_MV: i32 = 16000;
pub const ALIGN_ID_MA: i32 = 300;
// 启动/接管500 mA，闭环Iq额度1.1 A；不是厂家确认的相电流额定值。
// 1.8 A软件跳闸留出控制瞬态余量，不保证实测峰值或等价于硬件限流。
pub const STARTUP_CURRENT_MA: i32 = 500;
pub const RUN_IQ_MAX_MA: i32 = 1100;
// 台架稳态约70–76 mA；过渡Iq约48 mA时失速，保留100 mA转矩指令。
// 经3 mA/帧斜坡和启动电流圆，不在接管帧突变物理参考。
pub const BLEND_MIN_IQ_MA: i32 = 100;
pub const CURRENT_SLEW_MA_PER_FRAME: i32 = 3; // 2.5 kHz × 3 mA = 7.5 A/s。

// 用户测得任意两根电机引线之间 6.7 Ω、静态 1.75 mH；不是独立相绕组测量。
pub const LINE_TO_LINE_R_MILLIOHM: i32 = 6700;
pub const LINE_TO_LINE_L_UH: i32 = 1750;
// 平衡三相的等效星形模型取线间值的一半，不据此断定实际绕组连接方式。
// 静态电感随转子位置、测试频率、互感及凸极性变化，必须动态校核。
pub const MODEL_PHASE_R_MILLIOHM: i32 = LINE_TO_LINE_R_MILLIOHM / 2;
pub const MODEL_PHASE_L_UH: i32 = LINE_TO_LINE_L_UH / 2;
// 50 Hz 电流环起点：Kp=L·2π50≈0.27489 Ω；Ki=R·2π50/2500≈0.42097。
// mV/mA 与 Ω 等价；Ki 已包含 nominal 1/CONTROL_HZ，实时再按实际 dt 缩放。
pub const CURRENT_KP_Q15: i32 = 9008;
pub const CURRENT_KI_Q15: i32 = 13795;
// 55%占空比跨度的线性电压圆约0.55/sqrt(3)=0.318 Vbus，取0.3125。
// 2.5 kHz留出第二采样后的计算时间；窗口约束仍不允许全SVPWM调制度。
pub const VOLTAGE_LIMIT_Q15: i32 = 10240;

pub const BOOTSTRAP_FRAMES: u32 = CONTROL_HZ / 200; // 5 ms
pub const ALIGN_FRAMES: u32 = CONTROL_HZ / 5; // 200 ms
pub const OPEN_RAMP_FRAMES: u32 = CONTROL_HZ * 5 / 2;
pub const STARTUP_TIMEOUT_FRAMES: u32 = CONTROL_HZ * 5;
pub const BLEND_FRAMES: u32 = CONTROL_HZ / 2;
pub const OPEN_START_MILLIHZ: i32 = 2000;
// 7.907 V 台架原 25 Hz/250 mA 已触及电压圆；降低 I/F 平台以留接管余量。
// 18 Hz 兼顾本次 E≈1.14 V@24.641 Hz 与旧假定磁链的 500 mV 可观测门槛。
// 这是本台架起点，不是按估计反电势自适应调速，也不保证所有母线/电机均可接管。
pub const OPEN_END_MILLIHZ: i32 = 18000;
pub const RUN_TARGET_MILLIHZ: i32 = 150000;
// 表列24N22P：11对极；KV=68 RPM/V，12 V空载估算816 rpm≈149.6电气Hz。
// 六档包含停机，最高目标150 Hz用于探索，调制电压/负载可能使实际速度更低。
pub const SPEED_LEVELS_MILLIHZ: [i32; 6] = [0, 30000, 60000, 90000, 120000, RUN_TARGET_MILLIHZ];
// 8–10 Hz接管曾在Blend内失锁；12 Hz是当前台架验证的可观测速度下限。
pub const HANDOFF_MIN_MILLIHZ: i32 = 12000;
pub const STALL_MIN_MILLIHZ: i32 = 4000;
pub const OVERSPEED_MILLIHZ: i32 = 165000;

// 2.5 kHz alpha=13/64，近似保持原4 kHz alpha=8/64带宽；补偿使用实际alpha。
pub const BEMF_FILTER_NUMERATOR: i32 = 13;
pub const BEMF_FILTER_SHIFT: u32 = 6;
pub const BEMF_MIN_MV: i32 = 500;
// PLL 单位：一电角周为 65536；速度为每帧角度的 Q16。
pub const PLL_KP_Q16: i32 = 16384;
pub const PLL_KI_Q16: i32 = 256;
pub const PLL_MAX_MILLIHZ: i32 = 180000;
pub const PLL_ERROR_LIMIT: i32 = 1820; // 10 电角度
                                       // I/F 的 d 轴是电流轴；正转负载使转子磁链落后，不能把负载角当成观测误差。
                                       // 达到可观测速度即累计20 ms资格；允许加速中接管，角差走廊和质量界保持。
pub const HANDOFF_LAG_MAX: i32 = 10922; // 60°；保留到 90° 失稳边界的余量。
pub const HANDOFF_LEAD_MAX: i32 = 1820; // 10° 瞬时摆动；实际接管帧不允许负 Iq。
pub const HANDOFF_ANGLE_SPREAD: i32 = 3640; // 20° 峰峰；不是编码器精度保证。
pub const HANDOFF_PLL_PEAK_LIMIT: i32 = 3640; // 20° 硬上限，RMS 仍 <=10°。
pub const HANDOFF_PLL_SPIKE_FRAMES: u32 = CONTROL_HZ / 1000; // 连续 >=1 ms 超10°拒绝。
pub const HANDOFF_SPEED_ERROR_MILLIHZ: i32 = 2000;
pub const HANDOFF_GOOD_FRAMES: u32 = CONTROL_HZ / 50; // 20 ms，4 kHz 下80帧。
pub const HANDOFF_PLL_OUTLIER_FRAMES: u32 = HANDOFF_GOOD_FRAMES / 20; // 窗口最多5%超10°。
pub const OBSERVER_BAD_FRAMES: u32 = CONTROL_HZ / 20;
pub const STALL_FRAMES: u32 = CONTROL_HZ / 10;

// 100 Hz速度环使用2.5 kHz预滤波反馈；台架将过快的速度环降至123/1以抑制摆动。
// 仅限制正向转矩，不实现主动反转或再生制动。
pub const SPEED_LOOP_DIVIDER: u32 = CONTROL_HZ / 100;
pub const SPEED_KP_Q15: i32 = 123; // mA/mHz
pub const SPEED_KI_Q15: i32 = 1; // 每次 100 Hz 更新
pub const SPEED_REF_SLEW_MILLIHZ: i32 = 100;

// ADC2每2帧(0.8 ms)发起，7帧(2.8 ms)未更新即故障；诊断每50 ms。
pub const SLOW_ADC_DIVIDER: u32 = CONTROL_HZ / 1000;
pub const SLOW_ADC_MAX_AGE: u16 = (CONTROL_HZ * 3 / 1000) as u16;
pub const DIAGNOSTIC_DIVIDER: u32 = CONTROL_HZ / 20;

/// 同时约束物理范围和定点乘法范围；参数错误不得解锁输出。
pub fn valid() -> bool {
    CONTROL_HZ == 2_500
        && PWM_HZ == CONTROL_HZ
        && CPU_HZ / PWM_HZ == PWM_TICKS as u32
        && ADC_NOMINAL_VDDA_MV == 5000
        && ADC_FULL_SCALE == 4095
        && SHUNT_MILLIOHM == 10
        && CURRENT_GAIN == 10
        && BUS_DIVIDER == 11
        && (500..=2000).contains(&CURRENT_TRIP_MA)
        && (1..=CURRENT_TRIP_MA).contains(&CURRENT_SUM_LIMIT_MA)
        && matches!(BATTERY_SERIES_CELLS, 2 | 3)
        && CELL_UNDERVOLTAGE_MV > 0
        && (1000..=15000).contains(&BUS_MIN_MV)
        && (BUS_MIN_MV + 1000..=16000).contains(&BUS_MAX_MV)
        && (1..CURRENT_TRIP_MA).contains(&ALIGN_ID_MA)
        && (1..CURRENT_TRIP_MA).contains(&STARTUP_CURRENT_MA)
        && (STARTUP_CURRENT_MA..CURRENT_TRIP_MA).contains(&RUN_IQ_MAX_MA)
        && (1..=STARTUP_CURRENT_MA).contains(&BLEND_MIN_IQ_MA)
        && (1..=10).contains(&CURRENT_SLEW_MA_PER_FRAME)
        && LINE_TO_LINE_R_MILLIOHM > 0
        && LINE_TO_LINE_L_UH > 0
        && LINE_TO_LINE_R_MILLIOHM % 2 == 0
        && LINE_TO_LINE_L_UH % 2 == 0
        && (1000..=4000).contains(&MODEL_PHASE_R_MILLIOHM)
        && (100..=5000).contains(&MODEL_PHASE_L_UH)
        && (1..=32768).contains(&CURRENT_KP_Q15)
        && (1..=16384).contains(&CURRENT_KI_Q15)
        && (1024..=10240).contains(&VOLTAGE_LIMIT_Q15)
        // sqrt(3)向上取1.733，电压圆不得超过调制器允许的占空比跨度。
        && VOLTAGE_LIMIT_Q15 as i64 * 1733 * PWM_TICKS as i64
            <= (crate::sampling::DUTY_MAX - crate::sampling::DUTY_MIN) as i64 * 32768 * 1000
        && (1..=CONTROL_HZ / 10).contains(&BOOTSTRAP_FRAMES)
        && (CONTROL_HZ / 20..=CONTROL_HZ).contains(&ALIGN_FRAMES)
        && (CONTROL_HZ / 2..=CONTROL_HZ * 10).contains(&OPEN_RAMP_FRAMES)
        && (OPEN_RAMP_FRAMES + HANDOFF_GOOD_FRAMES..=CONTROL_HZ * 30)
            .contains(&STARTUP_TIMEOUT_FRAMES)
        && (CONTROL_HZ / 10..=CONTROL_HZ * 2).contains(&BLEND_FRAMES)
        && (1000..HANDOFF_MIN_MILLIHZ).contains(&OPEN_START_MILLIHZ)
        && (STALL_MIN_MILLIHZ + 1..OPEN_END_MILLIHZ).contains(&HANDOFF_MIN_MILLIHZ)
        && SPEED_LEVELS_MILLIHZ[0] == 0
        && SPEED_LEVELS_MILLIHZ[1..]
            .iter()
            .all(|speed| (OPEN_END_MILLIHZ..OVERSPEED_MILLIHZ).contains(speed))
        && SPEED_LEVELS_MILLIHZ
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        && SPEED_LEVELS_MILLIHZ[5] == RUN_TARGET_MILLIHZ
        && (OPEN_END_MILLIHZ..OVERSPEED_MILLIHZ).contains(&RUN_TARGET_MILLIHZ)
        && (1000..HANDOFF_MIN_MILLIHZ).contains(&STALL_MIN_MILLIHZ)
        && (RUN_TARGET_MILLIHZ + 1..PLL_MAX_MILLIHZ).contains(&OVERSPEED_MILLIHZ)
        && // 速度换算支持±180 Hz；补偿提前角还须容纳滤波配置。
        (1..=180000).contains(&PLL_MAX_MILLIHZ)
        && (OPEN_END_MILLIHZ as i64 - OPEN_START_MILLIHZ as i64) * (OPEN_RAMP_FRAMES as i64)
            <= i32::MAX as i64
        && BEMF_FILTER_SHIFT == 6
        && (4..=18).contains(&BEMF_FILTER_NUMERATOR)
        && ((PLL_MAX_MILLIHZ as i64 * (1i64 << 32) / (CONTROL_HZ as i64 * 1000))
            * (1i64 << BEMF_FILTER_SHIFT) / BEMF_FILTER_NUMERATOR as i64)
            <= i32::MAX as i64
        && (100..BUS_MIN_MV / 4).contains(&BEMF_MIN_MV)
        && (1..=16384).contains(&PLL_KP_Q16)
        && (1..=256).contains(&PLL_KI_Q16)
        && (100..=2730).contains(&PLL_ERROR_LIMIT)
        && (5461..=10922).contains(&HANDOFF_LAG_MAX)
        && (0..=PLL_ERROR_LIMIT).contains(&HANDOFF_LEAD_MAX)
        && (PLL_ERROR_LIMIT..=3640).contains(&HANDOFF_ANGLE_SPREAD)
        && (PLL_ERROR_LIMIT..=3640).contains(&HANDOFF_PLL_PEAK_LIMIT)
        && HANDOFF_PLL_SPIKE_FRAMES == CONTROL_HZ / 1000
        && HANDOFF_PLL_OUTLIER_FRAMES == HANDOFF_GOOD_FRAMES / 20
        && (100..HANDOFF_MIN_MILLIHZ).contains(&HANDOFF_SPEED_ERROR_MILLIHZ)
        && (CONTROL_HZ / 100..=CONTROL_HZ).contains(&HANDOFF_GOOD_FRAMES)
        && (1..=CONTROL_HZ / 10).contains(&OBSERVER_BAD_FRAMES)
        && (1..=CONTROL_HZ / 2).contains(&STALL_FRAMES)
        && SPEED_LOOP_DIVIDER * 100 == CONTROL_HZ
        && (1..=4096).contains(&SPEED_KP_Q15)
        && (1..=1024).contains(&SPEED_KI_Q15)
        && (1..=1000).contains(&SPEED_REF_SLEW_MILLIHZ)
}
