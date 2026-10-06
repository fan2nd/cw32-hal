#![no_std]
#![no_main]

use defmt_rtt as _;

mod arithmetic;
mod config;
mod control;
mod hardware;
mod io;
mod sampling;

embassy_cw32::bind_interrupts!(struct Irqs {
    ATIM => hardware::PwmHandler;
    ADC1 => hardware::AdcHandler;
    BTIM1 => hardware::TickHandler;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_divider = embassy_cw32::rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = embassy_cw32::rcc::PclkDivider::Div1;
    let mut p = embassy_cw32::init(config);
    let clocks = embassy_cw32::rcc::clocks();
    assert_eq!(clocks.hclk_hz(), config::CPU_HZ);
    assert_eq!(clocks.pclk_hz(), config::CPU_HZ);
    // RM1.4 §25.8：0x001007D2 是以 mV 保存的工厂半字。先读入 RAM，
    // 再启用预取/缓存；电机任务不能在缓存开启后重新读取 BootLoader 区。
    let factory_reference_mv = embassy_cw32::motor::factory_reference_mv();
    // 96 MHz 的三等待周期仍由 RCC 保持；07 不做 Flash 擦写，单例永久留给电机任务。
    let flash = embassy_cw32::flash::enable_read_acceleration(&mut p.FLASH).unwrap();
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    assert_eq!(flash.wait_states, 3);
    // 此读数只用于定位缓存影响，绝不替换启动前保存的工厂值。
    let factory_after_accel_mv = embassy_cw32::motor::factory_reference_mv();
    // 此时电机定时器尚未启动；实时采样期间不输出日志。
    defmt::info!(
        "07-sensorless-foc: boot HCLK={}Hz PCLK={}Hz RTT=nonblocking",
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    defmt::info!(
        "Flash wait={} prefetch={}->true cache={}->true (readback verified)",
        flash.wait_states,
        flash.prefetch_was_enabled,
        flash.cache_was_enabled
    );
    defmt::info!(
        "BGR factory @0x001007D2 u16 mV: before_accel={} after_accel={} selected=before_accel",
        factory_reference_mv,
        factory_after_accel_mv
    );
    // 直接复位入口应为关闭状态；若已启用，无法证明上面的值来自无缓存读取。
    // 不允许使用来源不明的值启动，也不静默回退到标称 1.2 V。
    assert!(!flash.prefetch_was_enabled && !flash.cache_was_enabled);
    defmt::info!(
        "PWM={}Hz ADC1={}Hz ADC2={}Hz; outputs off, calibrate then wait for PA3 key",
        config::PWM_HZ,
        clocks.pclk_hz() / 2,
        clocks.pclk_hz() / 8
    );
    defmt::info!(
        "Battery={}S cell_uv={}mV bus_uv={}mV bus_ov={}mV (configured, no auto-detect)",
        config::BATTERY_SERIES_CELLS,
        config::CELL_UNDERVOLTAGE_MV,
        config::BUS_MIN_MV,
        config::BUS_MAX_MV
    );
    spawner.spawn(
        hardware::motor_task(
            hardware::MotorResources {
                flash: p.FLASH,
                atim: p.ATIM,
                eau: p.EAU,
                adc1: p.ADC1,
                adc2: p.ADC2,
                opa1: p.OPA1,
                bgr: p.BGR,
                btim1: p.BTIM1,
                pa15: p.PA15,
                pb3: p.PB3,
                pb4: p.PB4,
                pb5: p.PB5,
                pb6: p.PB6,
                pb7: p.PB7,
                pa6: p.PA6,
                pa7: p.PA7,
                pb0: p.PB0,
                pa8: p.PA8,
            },
            factory_reference_mv,
        )
        .unwrap(),
    );
    spawner.spawn(io::io_task(p.PC13, p.PA3).unwrap());
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    hardware::fatal()
}
#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    hardware::fatal()
}
#[cortex_m_rt::exception]
unsafe fn NonMaskableInt() -> ! {
    hardware::fatal()
}
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) {
    hardware::fatal()
}
