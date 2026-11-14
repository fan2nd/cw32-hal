#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WDT"]
    WDT = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - FLASHRAM"]
    FLASHRAM = 3,
    #[doc = "4 - SYSCTRL"]
    SYSCTRL = 4,
    #[doc = "5 - GPIOA"]
    GPIOA = 5,
    #[doc = "6 - GPIOB"]
    GPIOB = 6,
    #[doc = "7 - GPIOC"]
    GPIOC = 7,
    #[doc = "12 - ADC"]
    ADC = 12,
    #[doc = "13 - ATIM"]
    ATIM = 13,
    #[doc = "14 - VC1"]
    VC1 = 14,
    #[doc = "15 - VC2"]
    VC2 = 15,
    #[doc = "16 - GTIM1"]
    GTIM1 = 16,
    #[doc = "17 - GTIM2"]
    GTIM2 = 17,
    #[doc = "19 - LPTIM"]
    LPTIM = 19,
    #[doc = "20 - BTIM1"]
    BTIM1 = 20,
    #[doc = "21 - BTIM2"]
    BTIM2 = 21,
    #[doc = "22 - BTIM3"]
    BTIM3 = 22,
    #[doc = "23 - I2C"]
    I2C = 23,
    #[doc = "25 - SPI"]
    SPI = 25,
    #[doc = "27 - UART1"]
    UART1 = 27,
    #[doc = "28 - UART2"]
    UART2 = 28,
    #[doc = "29 - UART3"]
    UART3 = 29,
    #[doc = "31 - CLKFAULT"]
    CLKFAULT = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WDT();
        fn LVD();
        fn RTC();
        fn FLASHRAM();
        fn SYSCTRL();
        fn GPIOA();
        fn GPIOB();
        fn GPIOC();
        fn ADC();
        fn ATIM();
        fn VC1();
        fn VC2();
        fn GTIM1();
        fn GTIM2();
        fn LPTIM();
        fn BTIM1();
        fn BTIM2();
        fn BTIM3();
        fn I2C();
        fn SPI();
        fn UART1();
        fn UART2();
        fn UART3();
        fn CLKFAULT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector { _handler: WDT },
        Vector { _handler: LVD },
        Vector { _handler: RTC },
        Vector { _handler: FLASHRAM },
        Vector { _handler: SYSCTRL },
        Vector { _handler: GPIOA },
        Vector { _handler: GPIOB },
        Vector { _handler: GPIOC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC },
        Vector { _handler: ATIM },
        Vector { _handler: VC1 },
        Vector { _handler: VC2 },
        Vector { _handler: GTIM1 },
        Vector { _handler: GTIM2 },
        Vector { _reserved: 0 },
        Vector { _handler: LPTIM },
        Vector { _handler: BTIM1 },
        Vector { _handler: BTIM2 },
        Vector { _handler: BTIM3 },
        Vector { _handler: I2C },
        Vector { _reserved: 0 },
        Vector { _handler: SPI },
        Vector { _reserved: 0 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector { _reserved: 0 },
        Vector { _handler: CLKFAULT },
    ];
}
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_0000usize as _) };
pub const VCDIV: vcdiv::Vcdiv = unsafe { vcdiv::Vcdiv::from_ptr(0x4000_00a0usize as _) };
pub const VC1: vc::Vc = unsafe { vc::Vc::from_ptr(0x4000_00a4usize as _) };
pub const VC2: vc::Vc = unsafe { vc::Vc::from_ptr(0x4000_00b4usize as _) };
pub const LVD: lvd::Lvd = unsafe { lvd::Lvd::from_ptr(0x4000_00d0usize as _) };
pub const SPI: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_0800usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_0c00usize as _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_1000usize as _) };
pub const ATIM: atim::Atim = unsafe { atim::Atim::from_ptr(0x4000_1400usize as _) };
pub const GTIM1: gtim::Gtim = unsafe { gtim::Gtim::from_ptr(0x4000_1800usize as _) };
pub const GTIM2: gtim::Gtim = unsafe { gtim::Gtim::from_ptr(0x4000_1c00usize as _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_2000usize as _) };
pub const SYSCTRL: sysctrl::Sysctrl = unsafe { sysctrl::Sysctrl::from_ptr(0x4000_4000usize as _) };
pub const IRMOD: irmod::Irmod = unsafe { irmod::Irmod::from_ptr(0x4000_4080usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_4400usize as _) };
pub const BTIM1: btim::Btim = unsafe { btim::Btim::from_ptr(0x4000_4800usize as _) };
pub const BTIM2: btim::Btim = unsafe { btim::Btim::from_ptr(0x4000_4840usize as _) };
pub const BTIM3: btim::Btim = unsafe { btim::Btim::from_ptr(0x4000_4880usize as _) };
pub const IWDT: iwdt::Iwdt = unsafe { iwdt::Iwdt::from_ptr(0x4000_5000usize as _) };
pub const I2C: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const LPTIM: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_6000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const RAM: ram::Ram = unsafe { ram::Ram::from_ptr(0x4002_2400usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0100usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0200usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
pub use crate::peripherals::adc_cw32l011_v1 as adc;
pub use crate::peripherals::atim_cw32l010_v1 as atim;
pub use crate::peripherals::btim_cw32l010_v1 as btim;
pub use crate::peripherals::crc_cw32l010_v1 as crc;
pub use crate::peripherals::flash_cw32l011_v1 as flash;
pub use crate::peripherals::gpio_cw32l011_v1 as gpio;
pub use crate::peripherals::gtim_cw32l010_v1 as gtim;
pub use crate::peripherals::i2c_cw32l011_v1 as i2c;
pub use crate::peripherals::irmod_cw32l011_v1 as irmod;
pub use crate::peripherals::iwdt_v1 as iwdt;
pub use crate::peripherals::lptim_cw32l010_v1 as lptim;
pub use crate::peripherals::lvd_cw32l010_v1 as lvd;
pub use crate::peripherals::ram_cw32l011_v1 as ram;
pub use crate::peripherals::rtc_cw32l011_v1 as rtc;
pub use crate::peripherals::spi_cw32l010_v1 as spi;
pub use crate::peripherals::sysctrl_cw32l011_v1 as sysctrl;
pub use crate::peripherals::uart_cw32l010_v1 as uart;
pub use crate::peripherals::vc_cw32l011_v1 as vc;
pub use crate::peripherals::vcdiv_cw32l010_v1 as vcdiv;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
