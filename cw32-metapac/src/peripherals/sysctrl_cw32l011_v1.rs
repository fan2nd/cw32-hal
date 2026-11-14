#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System Ctrl."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctrl {
    ptr: *mut u8,
}
unsafe impl Send for Sysctrl {}
unsafe impl Sync for Sysctrl {}
impl Sysctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Reg0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Reg1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Reg2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interupt Enable Reg."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interupt Status Reg."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interupt Clear Reg."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "HSI Control Reg."]
    #[inline(always)]
    pub const fn hsi(self) -> crate::common::Reg<regs::Hsi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "HEX Control Reg."]
    #[inline(always)]
    pub const fn hse(self) -> crate::common::Reg<regs::Hse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "LSI Control Reg."]
    #[inline(always)]
    pub const fn lsi(self) -> crate::common::Reg<regs::Lsi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "LSE Control Reg."]
    #[inline(always)]
    pub const fn lse(self) -> crate::common::Reg<regs::Lse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Debug Control Reg."]
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "AHB Clock Control Reg."]
    #[inline(always)]
    pub const fn ahben(self) -> crate::common::Reg<regs::Ahben, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "APB Clock Control Reg2."]
    #[inline(always)]
    pub const fn apben2(self) -> crate::common::Reg<regs::Apben2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "APB Clock Control Reg1."]
    #[inline(always)]
    pub const fn apben1(self) -> crate::common::Reg<regs::Apben1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "AHB Reset Control Reg."]
    #[inline(always)]
    pub const fn ahbrst(self) -> crate::common::Reg<regs::Ahbrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "APB Reset Control Reg2."]
    #[inline(always)]
    pub const fn apbrst2(self) -> crate::common::Reg<regs::Apbrst2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "APB Reset Control Reg1."]
    #[inline(always)]
    pub const fn apbrst1(self) -> crate::common::Reg<regs::Apbrst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Reset Status Reg."]
    #[inline(always)]
    pub const fn resetflag(self) -> crate::common::Reg<regs::Resetflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Master Clock Output Control Reg."]
    #[inline(always)]
    pub const fn mco(self) -> crate::common::Reg<regs::Mco, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB Clock Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahben(pub u32);
    impl Ahben {
        #[doc = "desc FLASH."]
        #[must_use]
        #[inline(always)]
        pub const fn flash(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLASH."]
        #[inline(always)]
        pub const fn set_flash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc GPIOA."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOA."]
        #[inline(always)]
        pub const fn set_gpioa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc GPIOB."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiob(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOB."]
        #[inline(always)]
        pub const fn set_gpiob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GPIOC."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOC."]
        #[inline(always)]
        pub const fn set_gpioc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ahben {
        #[inline(always)]
        fn default() -> Ahben {
            Ahben(0)
        }
    }
    impl core::fmt::Debug for Ahben {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahben")
                .field("flash", &self.flash())
                .field("crc", &self.crc())
                .field("gpioa", &self.gpioa())
                .field("gpiob", &self.gpiob())
                .field("gpioc", &self.gpioc())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahben {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahben {{ flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?}, key: {=u16:?} }}",
                self.flash(),
                self.crc(),
                self.gpioa(),
                self.gpiob(),
                self.gpioc(),
                self.key()
            )
        }
    }
    #[doc = "AHB Reset Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrst(pub u32);
    impl Ahbrst {
        #[doc = "desc FLASH."]
        #[must_use]
        #[inline(always)]
        pub const fn flash(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLASH."]
        #[inline(always)]
        pub const fn set_flash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc GPIOA."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOA."]
        #[inline(always)]
        pub const fn set_gpioa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc GPIOB."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiob(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOB."]
        #[inline(always)]
        pub const fn set_gpiob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GPIOC."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOC."]
        #[inline(always)]
        pub const fn set_gpioc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ahbrst {
        #[inline(always)]
        fn default() -> Ahbrst {
            Ahbrst(0)
        }
    }
    impl core::fmt::Debug for Ahbrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbrst")
                .field("flash", &self.flash())
                .field("crc", &self.crc())
                .field("gpioa", &self.gpioa())
                .field("gpiob", &self.gpiob())
                .field("gpioc", &self.gpioc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahbrst {{ flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?} }}",
                self.flash(),
                self.crc(),
                self.gpioa(),
                self.gpiob(),
                self.gpioc()
            )
        }
    }
    #[doc = "APB Clock Control Reg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apben1(pub u32);
    impl Apben1 {
        #[doc = "desc ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC."]
        #[inline(always)]
        pub const fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VC."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1."]
        #[inline(always)]
        pub const fn set_gtim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc GTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2."]
        #[inline(always)]
        pub const fn set_gtim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UART3."]
        #[must_use]
        #[inline(always)]
        pub const fn uart3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART3."]
        #[inline(always)]
        pub const fn set_uart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Apben1 {
        #[inline(always)]
        fn default() -> Apben1 {
            Apben1(0)
        }
    }
    impl core::fmt::Debug for Apben1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apben1")
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("spi1", &self.spi1())
                .field("uart1", &self.uart1())
                .field("uart2", &self.uart2())
                .field("atim", &self.atim())
                .field("gtim1", &self.gtim1())
                .field("gtim2", &self.gtim2())
                .field("uart3", &self.uart3())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben1 {{ adc: {=bool:?}, vc: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, uart2: {=bool:?}, atim: {=bool:?}, gtim1: {=bool:?}, gtim2: {=bool:?}, uart3: {=bool:?}, key: {=u16:?} }}",
                self.adc(),
                self.vc(),
                self.spi1(),
                self.uart1(),
                self.uart2(),
                self.atim(),
                self.gtim1(),
                self.gtim2(),
                self.uart3(),
                self.key()
            )
        }
    }
    #[doc = "APB Clock Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apben2(pub u32);
    impl Apben2 {
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BTIM123."]
        #[must_use]
        #[inline(always)]
        pub const fn btim123(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM123."]
        #[inline(always)]
        pub const fn set_btim123(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C1."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Apben2 {
        #[inline(always)]
        fn default() -> Apben2 {
            Apben2(0)
        }
    }
    impl core::fmt::Debug for Apben2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apben2")
                .field("rtc", &self.rtc())
                .field("btim123", &self.btim123())
                .field("iwdt", &self.iwdt())
                .field("i2c1", &self.i2c1())
                .field("lptim", &self.lptim())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben2 {{ rtc: {=bool:?}, btim123: {=bool:?}, iwdt: {=bool:?}, i2c1: {=bool:?}, lptim: {=bool:?}, key: {=u16:?} }}",
                self.rtc(),
                self.btim123(),
                self.iwdt(),
                self.i2c1(),
                self.lptim(),
                self.key()
            )
        }
    }
    #[doc = "APB Reset Control Reg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst1(pub u32);
    impl Apbrst1 {
        #[doc = "desc ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC."]
        #[inline(always)]
        pub const fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VC."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc GTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1."]
        #[inline(always)]
        pub const fn set_gtim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc GTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2."]
        #[inline(always)]
        pub const fn set_gtim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UART3."]
        #[must_use]
        #[inline(always)]
        pub const fn uart3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART3."]
        #[inline(always)]
        pub const fn set_uart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Apbrst1 {
        #[inline(always)]
        fn default() -> Apbrst1 {
            Apbrst1(0)
        }
    }
    impl core::fmt::Debug for Apbrst1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apbrst1")
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("spi1", &self.spi1())
                .field("uart1", &self.uart1())
                .field("uart2", &self.uart2())
                .field("atim", &self.atim())
                .field("gtim1", &self.gtim1())
                .field("gtim2", &self.gtim2())
                .field("uart3", &self.uart3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst1 {{ adc: {=bool:?}, vc: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, uart2: {=bool:?}, atim: {=bool:?}, gtim1: {=bool:?}, gtim2: {=bool:?}, uart3: {=bool:?} }}",
                self.adc(),
                self.vc(),
                self.spi1(),
                self.uart1(),
                self.uart2(),
                self.atim(),
                self.gtim1(),
                self.gtim2(),
                self.uart3()
            )
        }
    }
    #[doc = "APB Reset Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst2(pub u32);
    impl Apbrst2 {
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BTIM123."]
        #[must_use]
        #[inline(always)]
        pub const fn btim123(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM123."]
        #[inline(always)]
        pub const fn set_btim123(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C1."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Apbrst2 {
        #[inline(always)]
        fn default() -> Apbrst2 {
            Apbrst2(0)
        }
    }
    impl core::fmt::Debug for Apbrst2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apbrst2")
                .field("rtc", &self.rtc())
                .field("btim123", &self.btim123())
                .field("iwdt", &self.iwdt())
                .field("i2c1", &self.i2c1())
                .field("lptim", &self.lptim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst2 {{ rtc: {=bool:?}, btim123: {=bool:?}, iwdt: {=bool:?}, i2c1: {=bool:?}, lptim: {=bool:?} }}",
                self.rtc(),
                self.btim123(),
                self.iwdt(),
                self.i2c1(),
                self.lptim()
            )
        }
    }
    #[doc = "Control Reg0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc SYSCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclk(&self) -> super::vals::Sysclk {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sysclk::from_bits(val as u8)
        }
        #[doc = "desc SYSCLK."]
        #[inline(always)]
        pub const fn set_sysclk(&mut self, val: super::vals::Sysclk) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "desc PCLKPRS."]
        #[must_use]
        #[inline(always)]
        pub const fn pclkprs(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "desc PCLKPRS."]
        #[inline(always)]
        pub const fn set_pclkprs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "desc HCLKPRS."]
        #[must_use]
        #[inline(always)]
        pub const fn hclkprs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "desc HCLKPRS."]
        #[inline(always)]
        pub const fn set_hclkprs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    impl core::fmt::Debug for Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0")
                .field("sysclk", &self.sysclk())
                .field("pclkprs", &self.pclkprs())
                .field("hclkprs", &self.hclkprs())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ sysclk: {:?}, pclkprs: {=u8:?}, hclkprs: {=u8:?}, key: {=u16:?} }}",
                self.sysclk(),
                self.pclkprs(),
                self.hclkprs(),
                self.key()
            )
        }
    }
    #[doc = "Control Reg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc HSIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hsien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSIEN."]
        #[inline(always)]
        pub const fn set_hsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hseen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEEN."]
        #[inline(always)]
        pub const fn set_hseen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LSIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lsien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSIEN."]
        #[inline(always)]
        pub const fn set_lsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LSEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lseen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEEN."]
        #[inline(always)]
        pub const fn set_lseen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc LSELOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn lselock(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSELOCK."]
        #[inline(always)]
        pub const fn set_lselock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc LSECCS."]
        #[must_use]
        #[inline(always)]
        pub const fn lseccs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSECCS."]
        #[inline(always)]
        pub const fn set_lseccs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HSECCS."]
        #[must_use]
        #[inline(always)]
        pub const fn hseccs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSECCS."]
        #[inline(always)]
        pub const fn set_hseccs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CLKCCS."]
        #[must_use]
        #[inline(always)]
        pub const fn clkccs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLKCCS."]
        #[inline(always)]
        pub const fn set_clkccs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("hsien", &self.hsien())
                .field("hseen", &self.hseen())
                .field("lsien", &self.lsien())
                .field("lseen", &self.lseen())
                .field("lselock", &self.lselock())
                .field("lseccs", &self.lseccs())
                .field("hseccs", &self.hseccs())
                .field("clkccs", &self.clkccs())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ hsien: {=bool:?}, hseen: {=bool:?}, lsien: {=bool:?}, lseen: {=bool:?}, lselock: {=bool:?}, lseccs: {=bool:?}, hseccs: {=bool:?}, clkccs: {=bool:?}, key: {=u16:?} }}",
                self.hsien(),
                self.hseen(),
                self.lsien(),
                self.lseen(),
                self.lselock(),
                self.lseccs(),
                self.hseccs(),
                self.clkccs(),
                self.key()
            )
        }
    }
    #[doc = "Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc SWDIO."]
        #[must_use]
        #[inline(always)]
        pub const fn swdio(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDIO."]
        #[inline(always)]
        pub const fn set_swdio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LOCKUP."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOCKUP."]
        #[inline(always)]
        pub const fn set_lockup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc WAKEUPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeupclk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAKEUPCLK."]
        #[inline(always)]
        pub const fn set_wakeupclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FLASHWAIT."]
        #[must_use]
        #[inline(always)]
        pub const fn flashwait(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLASHWAIT."]
        #[inline(always)]
        pub const fn set_flashwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc RTCLPM."]
        #[must_use]
        #[inline(always)]
        pub const fn rtclpm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCLPM."]
        #[inline(always)]
        pub const fn set_rtclpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSEBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hsebrken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEBRKEN."]
        #[inline(always)]
        pub const fn set_hsebrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LSEBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lsebrken(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEBRKEN."]
        #[inline(always)]
        pub const fn set_lsebrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CLLBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cllbrken(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLLBRKEN."]
        #[inline(always)]
        pub const fn set_cllbrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc DSBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dsbrken(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSBRKEN."]
        #[inline(always)]
        pub const fn set_dsbrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LVDBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lvdbrken(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVDBRKEN."]
        #[inline(always)]
        pub const fn set_lvdbrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RAMBRKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rambrken(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAMBRKEN."]
        #[inline(always)]
        pub const fn set_rambrken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("swdio", &self.swdio())
                .field("lockup", &self.lockup())
                .field("wakeupclk", &self.wakeupclk())
                .field("flashwait", &self.flashwait())
                .field("rtclpm", &self.rtclpm())
                .field("hsebrken", &self.hsebrken())
                .field("lsebrken", &self.lsebrken())
                .field("cllbrken", &self.cllbrken())
                .field("dsbrken", &self.dsbrken())
                .field("lvdbrken", &self.lvdbrken())
                .field("rambrken", &self.rambrken())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ swdio: {=bool:?}, lockup: {=bool:?}, wakeupclk: {=bool:?}, flashwait: {=u8:?}, rtclpm: {=bool:?}, hsebrken: {=bool:?}, lsebrken: {=bool:?}, cllbrken: {=bool:?}, dsbrken: {=bool:?}, lvdbrken: {=bool:?}, rambrken: {=bool:?}, key: {=u16:?} }}",
                self.swdio(),
                self.lockup(),
                self.wakeupclk(),
                self.flashwait(),
                self.rtclpm(),
                self.hsebrken(),
                self.lsebrken(),
                self.cllbrken(),
                self.dsbrken(),
                self.lvdbrken(),
                self.rambrken(),
                self.key()
            )
        }
    }
    #[doc = "Debug Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Debug(pub u32);
    impl Debug {
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc GTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1."]
        #[inline(always)]
        pub const fn set_gtim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc GTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2."]
        #[inline(always)]
        pub const fn set_gtim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BTIM123."]
        #[must_use]
        #[inline(always)]
        pub const fn btim123(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM123."]
        #[inline(always)]
        pub const fn set_btim123(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Debug {
        #[inline(always)]
        fn default() -> Debug {
            Debug(0)
        }
    }
    impl core::fmt::Debug for Debug {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Debug")
                .field("atim", &self.atim())
                .field("gtim1", &self.gtim1())
                .field("gtim2", &self.gtim2())
                .field("btim123", &self.btim123())
                .field("lptim", &self.lptim())
                .field("rtc", &self.rtc())
                .field("iwdt", &self.iwdt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Debug {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Debug {{ atim: {=bool:?}, gtim1: {=bool:?}, gtim2: {=bool:?}, btim123: {=bool:?}, lptim: {=bool:?}, rtc: {=bool:?}, iwdt: {=bool:?} }}",
                self.atim(),
                self.gtim1(),
                self.gtim2(),
                self.btim123(),
                self.lptim(),
                self.rtc(),
                self.iwdt()
            )
        }
    }
    #[doc = "HEX Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hse(pub u32);
    impl Hse {
        #[doc = "desc DRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn driver(&self) -> super::vals::HseDrive {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::HseDrive::from_bits(val as u8)
        }
        #[doc = "desc DRIVER."]
        #[inline(always)]
        pub const fn set_driver(&mut self, val: super::vals::HseDrive) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc WAITCYCLE."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcycle(&self) -> super::vals::HseWait {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::HseWait::from_bits(val as u8)
        }
        #[doc = "desc WAITCYCLE."]
        #[inline(always)]
        pub const fn set_waitcycle(&mut self, val: super::vals::HseWait) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HEXENPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn hexenpol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXENPOL."]
        #[inline(always)]
        pub const fn set_hexenpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc DETCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn detcnt(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc DETCNT."]
        #[inline(always)]
        pub const fn set_detcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 8usize)) | (((val as u32) & 0x07ff) << 8usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc PDRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn pdriver(&self) -> super::vals::HseDrive {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::HseDrive::from_bits(val as u8)
        }
        #[doc = "desc PDRIVER."]
        #[inline(always)]
        pub const fn set_pdriver(&mut self, val: super::vals::HseDrive) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc DIGFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn digflt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIGFLT."]
        #[inline(always)]
        pub const fn set_digflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Hse {
        #[inline(always)]
        fn default() -> Hse {
            Hse(0)
        }
    }
    impl core::fmt::Debug for Hse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hse")
                .field("driver", &self.driver())
                .field("waitcycle", &self.waitcycle())
                .field("mode", &self.mode())
                .field("hexenpol", &self.hexenpol())
                .field("detcnt", &self.detcnt())
                .field("stable", &self.stable())
                .field("pdriver", &self.pdriver())
                .field("digflt", &self.digflt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hse {{ driver: {:?}, waitcycle: {:?}, mode: {=bool:?}, hexenpol: {=bool:?}, detcnt: {=u16:?}, stable: {=bool:?}, pdriver: {:?}, digflt: {=bool:?} }}",
                self.driver(),
                self.waitcycle(),
                self.mode(),
                self.hexenpol(),
                self.detcnt(),
                self.stable(),
                self.pdriver(),
                self.digflt()
            )
        }
    }
    #[doc = "HSI Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsi(pub u32);
    impl Hsi {
        #[doc = "desc TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn trim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc TRIM."]
        #[inline(always)]
        pub const fn set_trim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
    }
    impl Default for Hsi {
        #[inline(always)]
        fn default() -> Hsi {
            Hsi(0)
        }
    }
    impl core::fmt::Debug for Hsi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hsi")
                .field("trim", &self.trim())
                .field("div", &self.div())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hsi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hsi {{ trim: {=u16:?}, div: {=u8:?}, stable: {=bool:?} }}",
                self.trim(),
                self.div(),
                self.stable()
            )
        }
    }
    #[doc = "Interupt Clear Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc HSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSIRDY."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSERDY."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSIRDY."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSERDY."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc LSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefail(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAIL."]
        #[inline(always)]
        pub const fn set_lsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefail(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAIL."]
        #[inline(always)]
        pub const fn set_hsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefault(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAULT."]
        #[inline(always)]
        pub const fn set_lsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefault(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAULT."]
        #[inline(always)]
        pub const fn set_hsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("hsirdy", &self.hsirdy())
                .field("hserdy", &self.hserdy())
                .field("lsirdy", &self.lsirdy())
                .field("lserdy", &self.lserdy())
                .field("lsefail", &self.lsefail())
                .field("hsefail", &self.hsefail())
                .field("lsefault", &self.lsefault())
                .field("hsefault", &self.hsefault())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.lsirdy(),
                self.lserdy(),
                self.lsefail(),
                self.hsefail(),
                self.lsefault(),
                self.hsefault()
            )
        }
    }
    #[doc = "Interupt Enable Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc HSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSIRDY."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSERDY."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSIRDY."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSERDY."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc LSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefail(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAIL."]
        #[inline(always)]
        pub const fn set_lsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefail(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAIL."]
        #[inline(always)]
        pub const fn set_hsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefault(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAULT."]
        #[inline(always)]
        pub const fn set_lsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefault(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAULT."]
        #[inline(always)]
        pub const fn set_hsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("hsirdy", &self.hsirdy())
                .field("hserdy", &self.hserdy())
                .field("lsirdy", &self.lsirdy())
                .field("lserdy", &self.lserdy())
                .field("lsefail", &self.lsefail())
                .field("hsefail", &self.hsefail())
                .field("lsefault", &self.lsefault())
                .field("hsefault", &self.hsefault())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?}, key: {=u16:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.lsirdy(),
                self.lserdy(),
                self.lsefail(),
                self.hsefail(),
                self.lsefault(),
                self.hsefault(),
                self.key()
            )
        }
    }
    #[doc = "Interupt Status Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc HSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSIRDY."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSERDY."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LSIRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSIRDY."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LSERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSERDY."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc LSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefail(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAIL."]
        #[inline(always)]
        pub const fn set_lsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HSEFAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefail(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAIL."]
        #[inline(always)]
        pub const fn set_hsefail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn lsefault(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSEFAULT."]
        #[inline(always)]
        pub const fn set_lsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSEFAULT."]
        #[must_use]
        #[inline(always)]
        pub const fn hsefault(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSEFAULT."]
        #[inline(always)]
        pub const fn set_hsefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HSISTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsistable(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSISTABLE."]
        #[inline(always)]
        pub const fn set_hsistable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc HSESTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsestable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSESTABLE."]
        #[inline(always)]
        pub const fn set_hsestable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LSISTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn lsistable(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSISTABLE."]
        #[inline(always)]
        pub const fn set_lsistable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc LSESTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn lsestable(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSESTABLE."]
        #[inline(always)]
        pub const fn set_lsestable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("hsirdy", &self.hsirdy())
                .field("hserdy", &self.hserdy())
                .field("lsirdy", &self.lsirdy())
                .field("lserdy", &self.lserdy())
                .field("lsefail", &self.lsefail())
                .field("hsefail", &self.hsefail())
                .field("lsefault", &self.lsefault())
                .field("hsefault", &self.hsefault())
                .field("hsistable", &self.hsistable())
                .field("hsestable", &self.hsestable())
                .field("lsistable", &self.lsistable())
                .field("lsestable", &self.lsestable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?}, hsistable: {=bool:?}, hsestable: {=bool:?}, lsistable: {=bool:?}, lsestable: {=bool:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.lsirdy(),
                self.lserdy(),
                self.lsefail(),
                self.hsefail(),
                self.lsefault(),
                self.hsefault(),
                self.hsistable(),
                self.hsestable(),
                self.lsistable(),
                self.lsestable()
            )
        }
    }
    #[doc = "LSE Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lse(pub u32);
    impl Lse {
        #[doc = "desc DRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn driver(&self) -> super::vals::LseDrive {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::LseDrive::from_bits(val as u8)
        }
        #[doc = "desc DRIVER."]
        #[inline(always)]
        pub const fn set_driver(&mut self, val: super::vals::LseDrive) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc WAITCYCLE."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcycle(&self) -> super::vals::LseWait {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::LseWait::from_bits(val as u8)
        }
        #[doc = "desc WAITCYCLE."]
        #[inline(always)]
        pub const fn set_waitcycle(&mut self, val: super::vals::LseWait) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PDRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn pdriver(&self) -> super::vals::LseDrive {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::LseDrive::from_bits(val as u8)
        }
        #[doc = "desc PDRIVER."]
        #[inline(always)]
        pub const fn set_pdriver(&mut self, val: super::vals::LseDrive) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc PINLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn pinlock(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINLOCK."]
        #[inline(always)]
        pub const fn set_pinlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
    }
    impl Default for Lse {
        #[inline(always)]
        fn default() -> Lse {
            Lse(0)
        }
    }
    impl core::fmt::Debug for Lse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lse")
                .field("driver", &self.driver())
                .field("waitcycle", &self.waitcycle())
                .field("mode", &self.mode())
                .field("pdriver", &self.pdriver())
                .field("pinlock", &self.pinlock())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lse {{ driver: {:?}, waitcycle: {:?}, mode: {=bool:?}, pdriver: {:?}, pinlock: {=bool:?}, stable: {=bool:?} }}",
                self.driver(),
                self.waitcycle(),
                self.mode(),
                self.pdriver(),
                self.pinlock(),
                self.stable()
            )
        }
    }
    #[doc = "LSI Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsi(pub u32);
    impl Lsi {
        #[doc = "desc TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn trim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc TRIM."]
        #[inline(always)]
        pub const fn set_trim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc WAITCYCLE."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcycle(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc WAITCYCLE."]
        #[inline(always)]
        pub const fn set_waitcycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
    }
    impl Default for Lsi {
        #[inline(always)]
        fn default() -> Lsi {
            Lsi(0)
        }
    }
    impl core::fmt::Debug for Lsi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lsi")
                .field("trim", &self.trim())
                .field("waitcycle", &self.waitcycle())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lsi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lsi {{ trim: {=u16:?}, waitcycle: {=u8:?}, stable: {=bool:?} }}",
                self.trim(),
                self.waitcycle(),
                self.stable()
            )
        }
    }
    #[doc = "Master Clock Output Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mco(pub u32);
    impl Mco {
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Mco {
        #[inline(always)]
        fn default() -> Mco {
            Mco(0)
        }
    }
    impl core::fmt::Debug for Mco {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mco")
                .field("source", &self.source())
                .field("div", &self.div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mco {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mco {{ source: {=u8:?}, div: {=u8:?} }}",
                self.source(),
                self.div()
            )
        }
    }
    #[doc = "Reset Status Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resetflag(pub u32);
    impl Resetflag {
        #[doc = "desc POR."]
        #[must_use]
        #[inline(always)]
        pub const fn por(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR."]
        #[inline(always)]
        pub const fn set_por(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc LVD."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD."]
        #[inline(always)]
        pub const fn set_lvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RSTB."]
        #[must_use]
        #[inline(always)]
        pub const fn rstb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTB."]
        #[inline(always)]
        pub const fn set_rstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LOCKUP."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOCKUP."]
        #[inline(always)]
        pub const fn set_lockup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SYSRESETREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn sysresetreq(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSRESETREQ."]
        #[inline(always)]
        pub const fn set_sysresetreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Resetflag {
        #[inline(always)]
        fn default() -> Resetflag {
            Resetflag(0)
        }
    }
    impl core::fmt::Debug for Resetflag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Resetflag")
                .field("por", &self.por())
                .field("lvd", &self.lvd())
                .field("iwdt", &self.iwdt())
                .field("rstb", &self.rstb())
                .field("lockup", &self.lockup())
                .field("sysresetreq", &self.sysresetreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Resetflag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Resetflag {{ por: {=bool:?}, lvd: {=bool:?}, iwdt: {=bool:?}, rstb: {=bool:?}, lockup: {=bool:?}, sysresetreq: {=bool:?} }}",
                self.por(),
                self.lvd(),
                self.iwdt(),
                self.rstb(),
                self.lockup(),
                self.sysresetreq()
            )
        }
    }
}
pub mod vals {
    #[doc = "Own CW32L011 RM CN V1.1 section 4.7.6: pre-start and run HSE drive; only codes 0..7 documented."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HseDrive {
        #[doc = "Minimum drive."]
        Level0 = 0x0,
        #[doc = "Board-qualified drive code 1."]
        Level1 = 0x01,
        #[doc = "Board-qualified drive code 2."]
        Level2 = 0x02,
        #[doc = "Board-qualified drive code 3."]
        Level3 = 0x03,
        #[doc = "Board-qualified drive code 4."]
        Level4 = 0x04,
        #[doc = "Board-qualified drive code 5."]
        Level5 = 0x05,
        #[doc = "Board-qualified drive code 6."]
        Level6 = 0x06,
        #[doc = "Maximum drive."]
        Level7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl HseDrive {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HseDrive {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HseDrive {
        #[inline(always)]
        fn from(val: u8) -> HseDrive {
            HseDrive::from_bits(val)
        }
    }
    impl From<HseDrive> for u8 {
        #[inline(always)]
        fn from(val: HseDrive) -> u8 {
            HseDrive::to_bits(val)
        }
    }
    #[doc = "Own CW32L011 RM CN V1.1 section 4.7.6: startup stable-detection HSE cycle counts."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HseWait {
        #[doc = "Wait 8192 HSE cycles."]
        Cycles8192 = 0x0,
        #[doc = "Wait 32768 HSE cycles."]
        Cycles32768 = 0x01,
        #[doc = "Wait 131072 HSE cycles."]
        Cycles131072 = 0x02,
        #[doc = "Wait 262144 HSE cycles."]
        Cycles262144 = 0x03,
    }
    impl HseWait {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HseWait {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HseWait {
        #[inline(always)]
        fn from(val: u8) -> HseWait {
            HseWait::from_bits(val)
        }
    }
    impl From<HseWait> for u8 {
        #[inline(always)]
        fn from(val: HseWait) -> u8 {
            HseWait::to_bits(val)
        }
    }
    #[doc = "Own CW32L011 RM CN Rev1.1 PDF72/printed71."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LseDrive {
        #[doc = "Board-qualified LSE drive code 0."]
        Level0 = 0x0,
        #[doc = "Board-qualified LSE drive code 1."]
        Level1 = 0x01,
        #[doc = "Board-qualified LSE drive code 2."]
        Level2 = 0x02,
        #[doc = "Board-qualified LSE drive code 3."]
        Level3 = 0x03,
        #[doc = "Board-qualified LSE drive code 4."]
        Level4 = 0x04,
        #[doc = "Board-qualified LSE drive code 5."]
        Level5 = 0x05,
        #[doc = "Board-qualified LSE drive code 6."]
        Level6 = 0x06,
        #[doc = "Board-qualified LSE drive code 7."]
        Level7 = 0x07,
        #[doc = "Board-qualified LSE drive code 8."]
        Level8 = 0x08,
        #[doc = "Board-qualified LSE drive code 9."]
        Level9 = 0x09,
        #[doc = "Board-qualified LSE drive code 10."]
        Level10 = 0x0a,
        #[doc = "Board-qualified LSE drive code 11."]
        Level11 = 0x0b,
        #[doc = "Board-qualified LSE drive code 12."]
        Level12 = 0x0c,
        #[doc = "Board-qualified LSE drive code 13."]
        Level13 = 0x0d,
        #[doc = "Board-qualified LSE drive code 14."]
        Level14 = 0x0e,
        #[doc = "Board-qualified LSE drive code 15."]
        Level15 = 0x0f,
    }
    impl LseDrive {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LseDrive {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LseDrive {
        #[inline(always)]
        fn from(val: u8) -> LseDrive {
            LseDrive::from_bits(val)
        }
    }
    impl From<LseDrive> for u8 {
        #[inline(always)]
        fn from(val: LseDrive) -> u8 {
            LseDrive::to_bits(val)
        }
    }
    #[doc = "Own CW32L011 RM CN Rev1.1 PDF72/printed71."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LseWait {
        #[doc = "Wait 256 LSE cycles."]
        Cycles256 = 0x0,
        #[doc = "Wait 1024 LSE cycles."]
        Cycles1024 = 0x01,
        #[doc = "Wait 4096 LSE cycles."]
        Cycles4096 = 0x02,
        #[doc = "Wait 16384 LSE cycles."]
        Cycles16384 = 0x03,
    }
    impl LseWait {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LseWait {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LseWait {
        #[inline(always)]
        fn from(val: u8) -> LseWait {
            LseWait::from_bits(val)
        }
    }
    impl From<LseWait> for u8 {
        #[inline(always)]
        fn from(val: LseWait) -> u8 {
            LseWait::to_bits(val)
        }
    }
    #[doc = "Own CW32L011 RM CN V1.1 section 4.7.1: system clock; codes 2 and 5..7 reserved, no documented PLL."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sysclk {
        #[doc = "HSI system clock."]
        Hsi = 0x0,
        #[doc = "HSE system clock."]
        Hse = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "LSI system clock."]
        Lsi = 0x03,
        #[doc = "LSE system clock."]
        Lse = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sysclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sysclk {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sysclk {
        #[inline(always)]
        fn from(val: u8) -> Sysclk {
            Sysclk::from_bits(val)
        }
    }
    impl From<Sysclk> for u8 {
        #[inline(always)]
        fn from(val: Sysclk) -> u8 {
            Sysclk::to_bits(val)
        }
    }
}
