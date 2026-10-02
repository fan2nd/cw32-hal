#![no_std]
#![no_main]

pub mod control;
pub mod frame_queue;
mod motor;
pub mod protection;
pub mod protocol;
mod ui;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_frequency = embassy_cw32::rcc::HsiFrequency::Mhz96;
    config.rcc.pclk_divider = embassy_cw32::rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    // Shared clock gates are configured once before splitting task ownership.
    unsafe {
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::AHBEN,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | 1 | (1 << 4) | (1 << 5) | (1 << 6),
        );
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::APBEN1,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | (1 << 0) | (1 << 3) | (1 << 5),
        );
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::APBEN2,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | (1 << 2) | (1 << 9),
        );
    }
    // Transfer the motor's singleton tokens out of the thread executor domain.
    motor::start((
        p.DMA, p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3,
        p.PB4, p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
        p.PA11, p.UART2,
    ));
    ui::run(p.PC13, p.PA3, p.UART1, p.PB12, p.PB11).await;
}

use embassy_cw32::pac;

pub(crate) enum PinMode {
    OutputLow,
    OutputHigh,
    InputPullUp,
    Analog,
}
unsafe fn set_af(port: usize, pin: u32, af: u32) {
    let offset = if pin < 8 {
        pac::gpio::AFRL
    } else {
        pac::gpio::AFRH
    };
    let shift = (pin % 8) * 4;
    unsafe { pac::modify(port + offset, 0xf << shift, af << shift) };
}
pub(crate) unsafe fn configure_pin(port: usize, pin: u32, mode: PinMode, af: u32) {
    let mask = 1 << pin;
    unsafe {
        pac::modify(port + pac::gpio::DIR, 0, mask);
        set_af(port, pin, af);
        pac::modify(port + pac::gpio::RISEIE, mask, 0);
        pac::modify(port + pac::gpio::FALLIE, mask, 0);
        pac::modify(port + pac::gpio::OPENDRAIN, mask, 0);
        pac::modify(
            port + pac::gpio::PUR,
            mask,
            if matches!(mode, PinMode::InputPullUp) {
                mask
            } else {
                0
            },
        );
        pac::modify(
            port + pac::gpio::ANALOG,
            mask,
            if matches!(mode, PinMode::Analog) {
                mask
            } else {
                0
            },
        );
        if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
            pac::write(
                port + if matches!(mode, PinMode::OutputHigh) {
                    pac::gpio::BSRR
                } else {
                    pac::gpio::BRR
                },
                mask,
            );
            pac::modify(port + pac::gpio::DIR, mask, 0);
        }
    }
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
