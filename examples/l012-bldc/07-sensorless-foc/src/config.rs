#![deny(unsafe_code)]
//! 独立 FOC 实验参数。用户提供两电机线端间 R/L；等效模型及控制整定尚未经动态实板验证。

// 普通构建包含按键启动和有资格无感接管；上电先保持桥臂关闭并校准。
// 已采用用户线间测量值的等效模型，仍需限流、空载台架验证。

pub const CPU_HZ: u32 = 96_000_000;
pub const PWM_HZ: u32 = 8_000;
pub const CONTROL_HZ: u32 = PWM_HZ;
pub const PWM_TICKS: u16 = (CPU_HZ / PWM_HZ) as u16;
pub const ADC_NOMINAL_VDDA_MV: i32 = 5000;
pub const ADC_FULL_SCALE: i32 = 4095;
pub const SHUNT_MILLIOHM: i32 = 10;
pub const CURRENT_GAIN: i32 = 10;
pub const BUS_DIVIDER: i32 = 11;

// 仅作低电流台架起点；必须先确认电机、功率板和外部硬件保护。
pub const CURRENT_TRIP_MA: i32 = 750;
pub const CURRENT_SUM_LIMIT_MA: i32 = 200;
// 按实际电池/台架配置选择 2S 或 3S；不靠电压自动识别，也不在欠压时降档。
// 3300 mV/节沿用原 2S 的 6.6 V 实验起点，须按实际电芯和负载确认，不代替 BMS。
pub const BATTERY_SERIES_CELLS: u8 = 2;
pub const CELL_UNDERVOLTAGE_MV: i32 = 3300;
pub const BUS_MIN_MV: i32 = BATTERY_SERIES_CELLS as i32 * CELL_UNDERVOLTAGE_MV;
// 保留原 16 V 实验过压限制；电池串数不提高功率板额定值或证明驱动低压能力。
pub const BUS_MAX_MV: i32 = 16000;
pub const ALIGN_ID_MA: i32 = 150;
pub const STARTUP_CURRENT_MA: i32 = 200;
pub const RUN_IQ_MAX_MA: i32 = 250;
pub const CURRENT_SLEW_MA_PER_FRAME: i32 = 1; // 8 A/s，比原 10 A/s 略慢，保留每帧 1 mA 的低电流起步。

// 用户测得任意两根电机引线之间 6.7 Ω、静态 1.75 mH；不是独立相绕组测量。
pub const LINE_TO_LINE_R_MILLIOHM: i32 = 6700;
pub const LINE_TO_LINE_L_UH: i32 = 1750;
// 平衡三相的等效星形模型取线间值的一半，不据此断定实际绕组连接方式。
// 静态电感随转子位置、测试频率、互感及凸极性变化，必须动态校核。
pub const MODEL_PHASE_R_MILLIOHM: i32 = LINE_TO_LINE_R_MILLIOHM / 2;
pub const MODEL_PHASE_L_UH: i32 = LINE_TO_LINE_L_UH / 2;
// 50 Hz 电流环起点：Kp=L·2π50≈0.27489 Ω；Ki=R·2π50/8000≈0.13155。
// mV/mA 与 Ω 等价；Ki 已包含 nominal 1/CONTROL_HZ，实时再按实际 dt 缩放。
pub const CURRENT_KP_Q15: i32 = 9008;
pub const CURRENT_KI_Q15: i32 = 4311;
// 保留双低侧采样窗：电压矢量半径最多约 0.198 Vbus，不使用全 SVPWM 线性区。
pub const VOLTAGE_LIMIT_Q15: i32 = 6500;

pub const BOOTSTRAP_FRAMES: u32 = CONTROL_HZ / 200; // 5 ms
pub const ALIGN_FRAMES: u32 = CONTROL_HZ / 5; // 200 ms
pub const OPEN_RAMP_FRAMES: u32 = CONTROL_HZ * 5 / 2;
pub const STARTUP_TIMEOUT_FRAMES: u32 = CONTROL_HZ * 5;
pub const BLEND_FRAMES: u32 = CONTROL_HZ / 2;
pub const OPEN_START_MILLIHZ: i32 = 2000;
pub const OPEN_END_MILLIHZ: i32 = 25000;
pub const RUN_TARGET_MILLIHZ: i32 = 25000;
pub const HANDOFF_MIN_MILLIHZ: i32 = 8000;
pub const STALL_MIN_MILLIHZ: i32 = 4000;
pub const OVERSPEED_MILLIHZ: i32 = 80000;

// 8 kHz 的 alpha=10/64，近似保持原 10 kHz、alpha=1/8 的物理时间常数。
pub const BEMF_FILTER_NUMERATOR: i32 = 10;
pub const BEMF_FILTER_SHIFT: u32 = 6;
pub const BEMF_MIN_MV: i32 = 500;
// PLL 单位：一电角周为 65536；速度为每帧角度的 Q16。
pub const PLL_KP_Q16: i32 = 5120;
pub const PLL_KI_Q16: i32 = 25;
pub const PLL_MAX_MILLIHZ: i32 = 100000;
pub const PLL_ERROR_LIMIT: i32 = 1820; // 10 电角度
pub const HANDOFF_ANGLE_LIMIT: i32 = 5461; // 30 电角度
pub const HANDOFF_SPEED_ERROR_MILLIHZ: i32 = 2000;
pub const HANDOFF_GOOD_FRAMES: u32 = CONTROL_HZ / 4;
pub const OBSERVER_BAD_FRAMES: u32 = CONTROL_HZ / 20;
pub const STALL_FRAMES: u32 = CONTROL_HZ / 10;

// 100 Hz 外速度环，仅限制正向转矩，不实现主动反转或再生制动。
pub const SPEED_LOOP_DIVIDER: u32 = CONTROL_HZ / 100;
pub const SPEED_KP_Q15: i32 = 983; // mA/mHz
pub const SPEED_KI_Q15: i32 = 98; // 每次 100 Hz 更新
pub const SPEED_REF_SLEW_MILLIHZ: i32 = 100;

// ADC2 每 1 ms 发起，3 ms 未更新即故障；诊断内存每 50 ms 发布。
pub const SLOW_ADC_DIVIDER: u32 = CONTROL_HZ / 1000;
pub const SLOW_ADC_MAX_AGE: u16 = (CONTROL_HZ * 3 / 1000) as u16;
pub const DIAGNOSTIC_DIVIDER: u32 = CONTROL_HZ / 20;

/// 同时约束物理范围和定点乘法范围；参数错误不得解锁输出。
pub fn valid() -> bool {
    CONTROL_HZ == 8_000
        && PWM_HZ == CONTROL_HZ
        && CPU_HZ / PWM_HZ == PWM_TICKS as u32
        && ADC_NOMINAL_VDDA_MV == 5000
        && ADC_FULL_SCALE == 4095
        && SHUNT_MILLIOHM == 10
        && CURRENT_GAIN == 10
        && BUS_DIVIDER == 11
        && (500..=8000).contains(&CURRENT_TRIP_MA)
        && (1..=CURRENT_TRIP_MA).contains(&CURRENT_SUM_LIMIT_MA)
        && matches!(BATTERY_SERIES_CELLS, 2 | 3)
        && CELL_UNDERVOLTAGE_MV > 0
        && (1000..=50000).contains(&BUS_MIN_MV)
        && (BUS_MIN_MV + 1000..=50000).contains(&BUS_MAX_MV)
        && (1..CURRENT_TRIP_MA).contains(&ALIGN_ID_MA)
        && (1..CURRENT_TRIP_MA).contains(&STARTUP_CURRENT_MA)
        && (STARTUP_CURRENT_MA..CURRENT_TRIP_MA).contains(&RUN_IQ_MAX_MA)
        && (1..=10).contains(&CURRENT_SLEW_MA_PER_FRAME)
        && LINE_TO_LINE_R_MILLIOHM > 0
        && LINE_TO_LINE_L_UH > 0
        && LINE_TO_LINE_R_MILLIOHM % 2 == 0
        && LINE_TO_LINE_L_UH % 2 == 0
        && (1..=4000).contains(&MODEL_PHASE_R_MILLIOHM)
        && (1..=5000).contains(&MODEL_PHASE_L_UH)
        && (1..=32768).contains(&CURRENT_KP_Q15)
        && (1..=8192).contains(&CURRENT_KI_Q15)
        && (1024..=6500).contains(&VOLTAGE_LIMIT_Q15)
        && (1..=CONTROL_HZ / 10).contains(&BOOTSTRAP_FRAMES)
        && (CONTROL_HZ / 20..=CONTROL_HZ).contains(&ALIGN_FRAMES)
        && (CONTROL_HZ / 2..=CONTROL_HZ * 10).contains(&OPEN_RAMP_FRAMES)
        && (OPEN_RAMP_FRAMES + HANDOFF_GOOD_FRAMES..=CONTROL_HZ * 30)
            .contains(&STARTUP_TIMEOUT_FRAMES)
        && (CONTROL_HZ / 10..=CONTROL_HZ * 2).contains(&BLEND_FRAMES)
        && (1000..HANDOFF_MIN_MILLIHZ).contains(&OPEN_START_MILLIHZ)
        && (STALL_MIN_MILLIHZ + 1..OPEN_END_MILLIHZ).contains(&HANDOFF_MIN_MILLIHZ)
        && (OPEN_END_MILLIHZ..OVERSPEED_MILLIHZ).contains(&RUN_TARGET_MILLIHZ)
        && (1000..HANDOFF_MIN_MILLIHZ).contains(&STALL_MIN_MILLIHZ)
        && (RUN_TARGET_MILLIHZ + 1..PLL_MAX_MILLIHZ).contains(&OVERSPEED_MILLIHZ)
        && (1..=100000).contains(&PLL_MAX_MILLIHZ)
        && (OPEN_END_MILLIHZ as i64 - OPEN_START_MILLIHZ as i64) * (OPEN_RAMP_FRAMES as i64)
            <= i32::MAX as i64
        && BEMF_FILTER_SHIFT == 6
        && BEMF_FILTER_NUMERATOR == 10
        && (100..BUS_MIN_MV / 4).contains(&BEMF_MIN_MV)
        && (1..=8192).contains(&PLL_KP_Q16)
        && (1..=64).contains(&PLL_KI_Q16)
        && (100..=2730).contains(&PLL_ERROR_LIMIT)
        && (PLL_ERROR_LIMIT..=8192).contains(&HANDOFF_ANGLE_LIMIT)
        && (100..HANDOFF_MIN_MILLIHZ).contains(&HANDOFF_SPEED_ERROR_MILLIHZ)
        && (CONTROL_HZ / 10..=CONTROL_HZ).contains(&HANDOFF_GOOD_FRAMES)
        && (1..=CONTROL_HZ / 10).contains(&OBSERVER_BAD_FRAMES)
        && (1..=CONTROL_HZ / 2).contains(&STALL_FRAMES)
        && SPEED_LOOP_DIVIDER * 100 == CONTROL_HZ
        && (1..=4096).contains(&SPEED_KP_Q15)
        && (1..=1024).contains(&SPEED_KI_Q15)
        && (1..=1000).contains(&SPEED_REF_SLEW_MILLIHZ)
}
