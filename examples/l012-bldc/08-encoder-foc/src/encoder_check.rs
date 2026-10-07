#![no_std]
#![no_main]
//! 独立的无功率编码器检查固件。不开 ATIM/ADC，避免 RTT 临界区干扰采样截止。
use defmt_rtt as _;
use embassy_cw32::{
    gpio::{Level, Output},
    timer::{
        input_capture::Filter,
        qei::{Config, Qei},
    },
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_divider = embassy_cw32::rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = embassy_cw32::rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    // 所有桥臂命令持续拉低，不创建功率 PWM，不响应 PA3。
    let gates = [
        Output::new(p.PA15, Level::Low),
        Output::new(p.PB3, Level::Low),
        Output::new(p.PB4, Level::Low),
        Output::new(p.PB5, Level::Low),
        Output::new(p.PB6, Level::Low),
        Output::new(p.PB7, Level::Low),
    ];
    let encoder = Qei::new(
        p.GTIM2,
        p.PB14,
        p.PB15,
        Config {
            max_count: 4095,
            first_filter: Filter::PclkSamples4,
            second_filter: Filter::PclkSamples4,
            ..Default::default()
        },
    )
    .unwrap();
    let mut previous = encoder.count();
    let mut position = 0i64;
    let mut levels = encoder.input_levels();
    // 只读诊断：同一线程拥有 QEI，无并发写配置；不通过 PAC 更改引脚/计数器。
    let gpio = embassy_cw32::pac::GPIOB;
    let timer = embassy_cw32::pac::GTIM2;
    defmt::info!(
        "ENCODER_REG GPIOB DIR={=u32:x} ANALOG={=u32:x} AFRH={=u32:x} IDR={=u32:x}",
        gpio.dir().read().0,
        gpio.analog().read().0,
        gpio.afr(1).read().0,
        gpio.idr().read().0
    );
    defmt::info!("ENCODER_REG GTIM2 CR1={=u32:x} SMCR={=u32:x} CCMR1={=u32:x} CCER={=u32:x} TISEL={=u32:x} PSC={} ARR={}",
        timer.cr1().read().0, timer.smcr().read().0, timer.ccmr_cap(0).read().0,
        timer.ccer().read().0, timer.tisel().read().0, timer.psc().read().0, timer.arr().read().0);
    defmt::info!("08 ENCODER_CHECK: gates LOW, no PWM/ADC; PB14=A PB15=B GTIM2 x4; turn shaft slowly; 4096 counts/rev expected");
    loop {
        let mut edges = [0u32; 2];
        let mut seen = 0u8;
        let mut movement = 0u32;
        // 毫秒级轮询仅用于慢速手转诊断；不能据此统计高速编码器真实边沿数。
        for _ in 0..500 {
            cortex_m::asm::delay(embassy_cw32::rcc::clocks().hclk_hz() / 1000);
            let now = encoder.input_levels();
            for axis in 0..2 {
                edges[axis] += u32::from(now[axis] != levels[axis]);
            }
            levels = now;
            seen |= 1 << ((now[0] as u8) * 2 + now[1] as u8);
            let count = encoder.count();
            let delta = ((i32::from(count) - i32::from(previous) + 2048) & 4095) as i16 - 2048;
            previous = count;
            position += i64::from(delta);
            movement += u32::from(delta.unsigned_abs());
        }
        let count = encoder.count();
        let delta = ((i32::from(count) - i32::from(previous) + 2048) & 4095) as i16 - 2048;
        previous = count;
        position += i64::from(delta);
        let [a, b] = encoder.input_levels();
        defmt::info!(
            "ENCODER_CHECK raw={} tail_delta={} total={} A={} B={} running={} observed_edges={:?} seen_states={=u8:x} movement={}",
            count,
            delta,
            position,
            a,
            b,
            encoder.is_running(), edges, seen, movement
        );
        core::hint::black_box(&gates);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // 此固件始终未开启功率 PWM；停在原有 GPIO 低电平状态。
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
