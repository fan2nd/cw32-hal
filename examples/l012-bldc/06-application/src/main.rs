#![no_std]
#![no_main]

mod logging;
use defmt_rtt as _;

pub mod control;
pub mod frame_queue;
mod io;
mod motor;
pub mod protection;
pub mod protocol;

// 应用中断绑定：每个处理函数均在对应的中断向量中同步执行。
embassy_cw32::bind_interrupts!(
    struct Irqs {
        ADC1 => motor::AdcHandler;
        BTIM1 => motor::TickHandler;
        BTIM3_HALLTIM => motor::CommutationHandler;
        UART2 => motor::MotorExecutorHandler;
    }
);

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_divider = embassy_cw32::rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = embassy_cw32::rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    let clocks = embassy_cw32::rcc::clocks();
    defmt::info!(
        "06-application: boot HCLK={}Hz PCLK={}Hz RTT=nonblocking",
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    defmt::info!(
        "PWM=20000Hz ADC1=48000000Hz ADC2=12000000Hz; bootstrap=6ms, delay=400ms, align=150ms"
    );
    defmt::info!("power outputs: 6ms low-side bootstrap, then off until key start");

    // 将电机单例 token 的所有权移出普通线程执行器域。
    motor::start(
        motor::MotorResources {
            atim: p.ATIM,
            adc1: p.ADC1,
            adc2: p.ADC2,
            adc2_dma: p.DMA_CH2,
            opa1: p.OPA1,
            bgr: p.BGR,
            btim1: p.BTIM1,
            btim2: p.BTIM2,
            btim3: p.BTIM3,
            pa15: p.PA15,
            pb3: p.PB3,
            pb4: p.PB4,
            pb5: p.PB5,
            pb6: p.PB6,
            pb7: p.PB7,
            pa0: p.PA0,
            pa1: p.PA1,
            pa2: p.PA2,
            pa6: p.PA6,
            pa7: p.PA7,
            pb0: p.PB0,
            pb2: p.PB2,
            pa8: p.PA8,
            pa10: p.PA10,
            pa11: p.PA11,
            uart2: p.UART2,
        },
        Irqs,
    );
    io::io_task(p.PC13, p.PA3, p.UART1, p.PB12, p.PB11).await;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    motor::fatal()
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    motor::fatal()
}

#[cortex_m_rt::exception]
unsafe fn NonMaskableInt() -> ! {
    motor::fatal()
}

#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) {
    motor::fatal()
}
