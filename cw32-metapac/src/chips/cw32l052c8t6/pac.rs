#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WDT"]
    WDT = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - FLASH_RAM"]
    FLASH_RAM = 3,
    #[doc = "4 - SYSCTRL"]
    SYSCTRL = 4,
    #[doc = "5 - GPIOA"]
    GPIOA = 5,
    #[doc = "6 - GPIOB"]
    GPIOB = 6,
    #[doc = "7 - GPIOC_GPIOD"]
    GPIOC_GPIOD = 7,
    #[doc = "8 - GPIOF"]
    GPIOF = 8,
    #[doc = "9 - DMACH1"]
    DMACH1 = 9,
    #[doc = "10 - DMACH23"]
    DMACH23 = 10,
    #[doc = "11 - DMACH4"]
    DMACH4 = 11,
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
    #[doc = "18 - GTIM3"]
    GTIM3 = 18,
    #[doc = "20 - BTIM1"]
    BTIM1 = 20,
    #[doc = "21 - BTIM2_LPTIM"]
    BTIM2_LPTIM = 21,
    #[doc = "22 - BTIM3"]
    BTIM3 = 22,
    #[doc = "23 - I2C1"]
    I2C1 = 23,
    #[doc = "24 - I2C2"]
    I2C2 = 24,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "26 - SPI2"]
    SPI2 = 26,
    #[doc = "27 - UART1"]
    UART1 = 27,
    #[doc = "28 - UART2"]
    UART2 = 28,
    #[doc = "29 - UART3"]
    UART3 = 29,
    #[doc = "30 - AUTOTRIM_LCD"]
    AUTOTRIM_LCD = 30,
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
        fn FLASH_RAM();
        fn SYSCTRL();
        fn GPIOA();
        fn GPIOB();
        fn GPIOC_GPIOD();
        fn GPIOF();
        fn DMACH1();
        fn DMACH23();
        fn DMACH4();
        fn ADC();
        fn ATIM();
        fn VC1();
        fn VC2();
        fn GTIM1();
        fn GTIM2();
        fn GTIM3();
        fn BTIM1();
        fn BTIM2_LPTIM();
        fn BTIM3();
        fn I2C1();
        fn I2C2();
        fn SPI1();
        fn SPI2();
        fn UART1();
        fn UART2();
        fn UART3();
        fn AUTOTRIM_LCD();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 31] = [
        Vector { _handler: WDT },
        Vector { _handler: LVD },
        Vector { _handler: RTC },
        Vector {
            _handler: FLASH_RAM,
        },
        Vector { _handler: SYSCTRL },
        Vector { _handler: GPIOA },
        Vector { _handler: GPIOB },
        Vector {
            _handler: GPIOC_GPIOD,
        },
        Vector { _handler: GPIOF },
        Vector { _handler: DMACH1 },
        Vector { _handler: DMACH23 },
        Vector { _handler: DMACH4 },
        Vector { _handler: ADC },
        Vector { _handler: ATIM },
        Vector { _handler: VC1 },
        Vector { _handler: VC2 },
        Vector { _handler: GTIM1 },
        Vector { _handler: GTIM2 },
        Vector { _handler: GTIM3 },
        Vector { _reserved: 0 },
        Vector { _handler: BTIM1 },
        Vector {
            _handler: BTIM2_LPTIM,
        },
        Vector { _handler: BTIM3 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector {
            _handler: AUTOTRIM_LCD,
        },
    ];
}
pub const GTIM1: gtim::Gtim = unsafe { gtim::Gtim::from_ptr(0x4000_0400usize as _) };
pub const GTIM2: gtim::Gtim = unsafe { gtim::Gtim::from_ptr(0x4000_1000usize as _) };
pub const LCD: lcd::Lcd = unsafe { lcd::Lcd::from_ptr(0x4000_2400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDT: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_2c00usize as _) };
pub const IWDT: iwdt::Iwdt = unsafe { iwdt::Iwdt::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_4400usize as _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0x4000_4800usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const LPTIM: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_7800usize as _) };
pub const SYSCTRL: sysctrl::Sysctrl = unsafe { sysctrl::Sysctrl::from_ptr(0x4001_0000usize as _) };
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const VC1: vc::Vc = unsafe { vc::Vc::from_ptr(0x4001_2a00usize as _) };
pub const VC2: vc::Vc = unsafe { vc::Vc::from_ptr(0x4001_2a10usize as _) };
pub const LVD: lvd::Lvd = unsafe { lvd::Lvd::from_ptr(0x4001_2a80usize as _) };
pub const ATIM: atim::Atim = unsafe { atim::Atim::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x4001_3800usize as _) };
pub const GTIM3: gtim::Gtim = unsafe { gtim::Gtim::from_ptr(0x4001_4000usize as _) };
pub const BTIM1: btim::Btim = unsafe { btim::Btim::from_ptr(0x4001_4800usize as _) };
pub const BTIM2: btim::Btim = unsafe { btim::Btim::from_ptr(0x4001_4900usize as _) };
pub const BTIM3: btim::Btim = unsafe { btim::Btim::from_ptr(0x4001_4a00usize as _) };
pub const AUTOTRIM: autotrim::Autotrim =
    unsafe { autotrim::Autotrim::from_ptr(0x4001_4c00usize as _) };
pub const DMA: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMACHANNEL1: dmachannel::Dmachannel =
    unsafe { dmachannel::Dmachannel::from_ptr(0x4002_0020usize as _) };
pub const DMACHANNEL2: dmachannel::Dmachannel =
    unsafe { dmachannel::Dmachannel::from_ptr(0x4002_0040usize as _) };
pub const DMACHANNEL3: dmachannel::Dmachannel =
    unsafe { dmachannel::Dmachannel::from_ptr(0x4002_0060usize as _) };
pub const DMACHANNEL4: dmachannel::Dmachannel =
    unsafe { dmachannel::Dmachannel::from_ptr(0x4002_0080usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const RAM: ram::Ram = unsafe { ram::Ram::from_ptr(0x4002_2400usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0c00usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
pub use crate::peripherals::adc_cw32l052_v1 as adc;
pub use crate::peripherals::atim_cw32l052_v1 as atim;
pub use crate::peripherals::autotrim_cw32l052_v1 as autotrim;
pub use crate::peripherals::btim_cw32l052_v1 as btim;
pub use crate::peripherals::crc_cw32l052_v1 as crc;
pub use crate::peripherals::dma_cw32l031_v1 as dma;
pub use crate::peripherals::dmachannel_v1 as dmachannel;
pub use crate::peripherals::flash_cw32l031_v1 as flash;
pub use crate::peripherals::gpio_cw32l052_v1 as gpio;
pub use crate::peripherals::gtim_cw32l052_v1 as gtim;
pub use crate::peripherals::i2c_cw32l031_v1 as i2c;
pub use crate::peripherals::iwdt_cw32l031_v1 as iwdt;
pub use crate::peripherals::lcd_cw32l052_v1 as lcd;
pub use crate::peripherals::lptim_cw32l010_v1 as lptim;
pub use crate::peripherals::lvd_cw32l031_v1 as lvd;
pub use crate::peripherals::ram_cw32l031_v1 as ram;
pub use crate::peripherals::rtc_cw32l052_v1 as rtc;
pub use crate::peripherals::spi_cw32l031_v1 as spi;
pub use crate::peripherals::sysctrl_cw32l052_v1 as sysctrl;
pub use crate::peripherals::uart_cw32l052_v1 as uart;
pub use crate::peripherals::vc_cw32l031_v1 as vc;
pub use crate::peripherals::wwdt_cw32l031_v1 as wwdt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
