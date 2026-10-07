#![deny(unsafe_code)]
//! 08：MYH-4621F、AB 增量编码器、单电阻电流采样。均为待实板整定起点。
pub const CPU_HZ: u32 = 96_000_000;
pub const PWM_HZ: u32 = 20_000;
pub const CONTROL_HZ: u32 = 10_000;
pub const PWM_PER_CONTROL: u8 = (PWM_HZ / CONTROL_HZ) as u8;
pub const CONTROL_TICKS: u16 = (CPU_HZ / CONTROL_HZ) as u16;
pub const PWM_TICKS: u16 = (CPU_HZ / PWM_HZ) as u16;
pub const PWM_HALF_TICKS: u16 = PWM_TICKS / 2;
pub const ADC_NOMINAL_VDDA_MV: i32 = 5000;
pub const ADC_FULL_SCALE: i32 = 4095;
pub const SHUNT_MILLIOHM: i32 = 10;
pub const CURRENT_GAIN: i32 = 10;
pub const BUS_DIVIDER: i32 = 11;
pub const CURRENT_TRIP_MA: i32 = 1800;
pub const BATTERY_SERIES_CELLS: u8 = 3;
pub const CELL_UNDERVOLTAGE_MV: i32 = 3300;
pub const BUS_MIN_MV: i32 = BATTERY_SERIES_CELLS as i32 * CELL_UNDERVOLTAGE_MV;
pub const BUS_MAX_MV: i32 = 16000;
// 电压控制的粗限流阈值，不是Iq设定；硬过流仍独立检查原始样本。
// 带载实测1.6V扫角仅37计数（应约93）；提高对齐力度，仍渐升且检查位移。
pub const ALIGN_VOLTAGE_MV: i32 = 2800;
pub const ALIGN_CURRENT_LIMIT_MA: i32 = 900;
pub const RUN_CURRENT_LIMIT_MA: i32 = 1100;
// 无有效采样时只允许低电压恢复观测；不将盲区样本当作零电流。
// 半周矢量窗口减半，全角度可测需约0.26Vbus；允许低压恢复到0.275Vbus。
pub const UNOBSERVED_VOLTAGE_Q15: i32 = 9000;
pub const CURRENT_STALE_FRAMES: u32 = CONTROL_HZ / 10;
pub const CURRENT_HYSTERESIS_MA: i32 = 100;
pub const CURRENT_BACKOFF_MV: i32 = 10;
pub const CURRENT_RECOVER_MV: i32 = 1;
pub const VOLTAGE_SLEW_MV_PER_FRAME: i32 = 2; // 10kHz时仍20 V/s。
                                              // 12V时Uq可到6.19V，匹配中心对齐5%..95%的90%跨度，留在线性调制区。
pub const VOLTAGE_LIMIT_Q15: i32 = 16900;

pub const POLE_PAIRS: i32 = 11;
// MT6701 ABZ 1024 PPR，经硬件 Mode3 四倍频。不是 1024 计数/圈。
pub const ENCODER_CPR: i32 = 4096;
pub const ENCODER_MAX_DELTA: i32 = 16; // 单次控制读取，明显跳变即故障。
                                       // 快速测速下单计数回弹已约3.36电气Hz；反转改按真实反向位移判断。
pub const REVERSE_TRAVEL_COUNTS: i32 = 8;
// 编译期宽算术避免中间溢出；实时路径只加载最终32位比例常量。
pub const ENCODER_SPEED_SCALE: i32 =
    (CPU_HZ as i64 * 1000 * POLE_PAIRS as i64 / ENCODER_CPR as i64) as i32;
pub const BOOTSTRAP_FRAMES: u32 = CONTROL_HZ / 200;
pub const ALIGN_HOLD_FRAMES: u32 = CONTROL_HZ / 2;
pub const ALIGN_SWEEP_FRAMES: u32 = CONTROL_HZ / 2;
pub const ALIGN_SETTLE_FRAMES: u32 = CONTROL_HZ / 2;
pub const ALIGN_STILL_FRAMES: u32 = CONTROL_HZ / 10;
pub const ALIGN_STILL_COUNTS: i32 = 2;
// 先固定 90° 电角，缓慢退到 0°；由这段约 93 计数的运动判断 A/B 方向。
pub const ALIGN_TRAVEL_COUNTS: i32 = ENCODER_CPR / (4 * POLE_PAIRS);
// 对齐06：档位是输出百分比，非rpm目标；每30ms增加1%，满量程3s。
pub const OUTPUT_LEVELS_PERCENT: [i32; 6] = [0, 20, 40, 60, 80, 100];
pub const OUTPUT_RAMP_STEP_FRAMES: u32 = CONTROL_HZ * 30 / 1000;
pub const OVERSPEED_MILLIHZ: i32 = 165000;
// 每控制帧更新8点滑动窗口；0.8ms平滑，不再10ms分批输出。
pub const SPEED_WINDOW_FRAMES: usize = 8;
pub const SPEED_LOG_FILTER_DIVIDER: u32 = CONTROL_HZ / 100;
pub const STALL_GRACE_FRAMES: u32 = CONTROL_HZ * 2;
pub const STALL_FRAMES: u32 = CONTROL_HZ / 2;
pub const STALL_MIN_MILLIHZ: i32 = 2000;
pub const SLOW_ADC_DIVIDER: u32 = CONTROL_HZ / 1000;
pub const SLOW_ADC_MAX_AGE: u16 = (CONTROL_HZ * 3 / 1000) as u16;

pub fn valid() -> bool {
    CONTROL_HZ == 10_000
        && PWM_HZ == 20_000 && PWM_PER_CONTROL == 2 && CONTROL_TICKS == 9600
        && ADC_FULL_SCALE == 4095
        && BUS_DIVIDER == 11
        && CPU_HZ == 96_000_000
        && PWM_TICKS == 4800
        && ENCODER_CPR == 4096
        && POLE_PAIRS == 11
        && ENCODER_MAX_DELTA < ENCODER_CPR / 2
        && CURRENT_TRIP_MA <= 2000
        && CURRENT_TRIP_MA > RUN_CURRENT_LIMIT_MA
        && ALIGN_VOLTAGE_MV > 0
        && ALIGN_VOLTAGE_MV <= (BUS_MIN_MV * VOLTAGE_LIMIT_Q15 >> 15)
        && ALIGN_CURRENT_LIMIT_MA > CURRENT_HYSTERESIS_MA
        && ALIGN_CURRENT_LIMIT_MA < RUN_CURRENT_LIMIT_MA
        && RUN_CURRENT_LIMIT_MA <= 1100
        && CURRENT_BACKOFF_MV > CURRENT_RECOVER_MV && CURRENT_RECOVER_MV > 0
        && VOLTAGE_SLEW_MV_PER_FRAME > 0
        && BUS_MIN_MV > 0
        && BUS_MIN_MV < BUS_MAX_MV
        && BUS_MAX_MV <= 16000
        && VOLTAGE_LIMIT_Q15 > 0
        && VOLTAGE_LIMIT_Q15 <= 16900
        && VOLTAGE_LIMIT_Q15 as i64 * 1733 * PWM_HALF_TICKS as i64
            <= (crate::sampling::DUTY_MAX - crate::sampling::DUTY_MIN) as i64 * 32768 * 1000
        && ALIGN_STILL_FRAMES < ALIGN_SETTLE_FRAMES
        && ALIGN_SWEEP_FRAMES > 0
        && ALIGN_HOLD_FRAMES > ALIGN_STILL_FRAMES
        && ALIGN_TRAVEL_COUNTS >= 8
        && SPEED_LOG_FILTER_DIVIDER * 100 == CONTROL_HZ
        && SPEED_WINDOW_FRAMES == 8
        // 32 位商余数测速依赖整比例与有界的计数×余数乘积。
        && (CPU_HZ as i64 * 1000 * POLE_PAIRS as i64) % ENCODER_CPR as i64 == 0
        && CPU_HZ as i64 * 1000 * POLE_PAIRS as i64 / ENCODER_CPR as i64 <= i32::MAX as i64
        && ENCODER_MAX_DELTA as i64 * SPEED_WINDOW_FRAMES as i64
            * SPEED_WINDOW_FRAMES as i64 * (CONTROL_TICKS as i64 * 3 / 2) < i32::MAX as i64
        && OUTPUT_LEVELS_PERCENT[0] == 0
        && OUTPUT_LEVELS_PERCENT.windows(2).all(|v| v[0] < v[1])
        && OUTPUT_LEVELS_PERCENT[5] == 100
        && OUTPUT_RAMP_STEP_FRAMES == 300
        && OVERSPEED_MILLIHZ <= 180000
}
