#![no_std]
#![no_main]

pub mod control;
pub mod frame_queue;
mod motor;
pub mod protection;
pub mod protocol;
mod ui;

// Application wiring: each handler runs synchronously in its exact vector.
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

    // Transfer the motor's singleton tokens out of the thread executor domain.
    motor::start(
        p.DMACHANNEL2,
        (
            p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3, p.PB4,
            p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
            p.PA11, p.UART2,
        ),
        Irqs,
    );
    ui::run(p.PC13, p.PA3, p.UART1, p.PB12, p.PB11).await;
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
