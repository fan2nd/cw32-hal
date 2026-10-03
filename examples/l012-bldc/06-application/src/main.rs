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

    let mut ahben = pac::SYSCTRL.ahben().read();
    ahben.set_key(0x5a5a);
    ahben.set_dma(true);
    ahben.set_gpioa(true);
    ahben.set_gpiob(true);
    ahben.set_gpioc(true);
    pac::SYSCTRL.ahben().write_value(ahben);
    let mut apben1 = pac::SYSCTRL.apben1().read();
    apben1.set_key(0x5a5a);
    apben1.set_adc(true);
    apben1.set_atim(true);
    apben1.set_uart1(true);
    pac::SYSCTRL.apben1().write_value(apben1);
    let mut apben2 = pac::SYSCTRL.apben2().read();
    apben2.set_key(0x5a5a);
    apben2.set_btim123(true);
    apben2.set_opa(true);
    pac::SYSCTRL.apben2().write_value(apben2);

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
unsafe fn set_af(port: *mut (), pin: usize, af: u8) {
    let port: pac::gpio::Gpio = unsafe { pac::gpio::Gpio::from_ptr(port) };
    if pin < 8 {
        port.afr(0).modify(|r| r.set_afr(pin, af));
    } else {
        port.afr(1).modify(|r| r.set_afr(pin - 8, af));
    }
}
pub(crate) unsafe fn configure_pin(port: *mut (), pin: usize, mode: PinMode, af: u8) {
    let port: pac::gpio::Gpio = unsafe { pac::gpio::Gpio::from_ptr(port) };
    port.dir().modify(|r| r.set_pin(pin, true));
    unsafe { set_af(port.as_ptr(), pin, af) };
    port.riseie().modify(|r| r.set_pin(pin, false));
    port.fallie().modify(|r| r.set_pin(pin, false));
    port.opendrain().modify(|r| r.set_pin(pin, false));
    port.pur()
        .modify(|r| r.set_pin(pin, matches!(mode, PinMode::InputPullUp)));
    port.analog()
        .modify(|r| r.set_pin(pin, matches!(mode, PinMode::Analog)));
    if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
        if matches!(mode, PinMode::OutputHigh) {
            port.bsrr().write(|r| r.set_bss(pin, true));
        } else {
            port.brr().write(|r| r.set_brr(pin, true));
        }
        port.dir().modify(|r| r.set_pin(pin, false));
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
