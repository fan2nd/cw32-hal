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
    #[doc = "External input HEX control register."]
    #[inline(always)]
    pub const fn hex(self) -> crate::common::Reg<regs::Hex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "LSI Control Reg."]
    #[inline(always)]
    pub const fn lsi(self) -> crate::common::Reg<regs::Lsi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
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
    #[doc = "GTIM CAP Control Reg."]
    #[inline(always)]
    pub const fn gtimcap(self) -> crate::common::Reg<regs::Gtimcap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GTIM ETR Control Reg."]
    #[inline(always)]
    pub const fn gtimetr(self) -> crate::common::Reg<regs::Gtimetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "BTIMx GTIMx ATIM ITR Control Reg."]
    #[inline(always)]
    pub const fn timitr(self) -> crate::common::Reg<regs::Timitr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Master Clock Output Control Reg."]
    #[inline(always)]
    pub const fn mco(self) -> crate::common::Reg<regs::Mco, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "IR MOD Control Reg."]
    #[inline(always)]
    pub const fn irmod(self) -> crate::common::Reg<regs::Irmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahben {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahben {{ flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?} }}",
                self.flash(),
                self.crc(),
                self.gpioa(),
                self.gpiob(),
                self.gpioc()
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
        #[doc = "desc GTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM."]
        #[inline(always)]
        pub const fn set_gtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WWDT."]
        #[inline(always)]
        pub const fn set_wwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc I2C."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C."]
        #[inline(always)]
        pub const fn set_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("gtim", &self.gtim())
                .field("wwdt", &self.wwdt())
                .field("iwdt", &self.iwdt())
                .field("uart2", &self.uart2())
                .field("i2c", &self.i2c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben1 {{ gtim: {=bool:?}, wwdt: {=bool:?}, iwdt: {=bool:?}, uart2: {=bool:?}, i2c: {=bool:?} }}",
                self.gtim(),
                self.wwdt(),
                self.iwdt(),
                self.uart2(),
                self.i2c()
            )
        }
    }
    #[doc = "APB Clock Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apben2(pub u32);
    impl Apben2 {
        #[doc = "desc ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC."]
        #[inline(always)]
        pub const fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VC."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI."]
        #[inline(always)]
        pub const fn set_spi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn btim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM."]
        #[inline(always)]
        pub const fn set_btim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc AWT."]
        #[must_use]
        #[inline(always)]
        pub const fn awt(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWT."]
        #[inline(always)]
        pub const fn set_awt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("spi", &self.spi())
                .field("uart1", &self.uart1())
                .field("btim", &self.btim())
                .field("awt", &self.awt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben2 {{ adc: {=bool:?}, vc: {=bool:?}, spi: {=bool:?}, uart1: {=bool:?}, btim: {=bool:?}, awt: {=bool:?} }}",
                self.adc(),
                self.vc(),
                self.spi(),
                self.uart1(),
                self.btim(),
                self.awt()
            )
        }
    }
    #[doc = "APB Reset Control Reg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst1(pub u32);
    impl Apbrst1 {
        #[doc = "desc GTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM."]
        #[inline(always)]
        pub const fn set_gtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WWDT."]
        #[inline(always)]
        pub const fn set_wwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT."]
        #[inline(always)]
        pub const fn set_iwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc I2C."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C."]
        #[inline(always)]
        pub const fn set_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("gtim", &self.gtim())
                .field("wwdt", &self.wwdt())
                .field("iwdt", &self.iwdt())
                .field("uart2", &self.uart2())
                .field("i2c", &self.i2c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst1 {{ gtim: {=bool:?}, wwdt: {=bool:?}, iwdt: {=bool:?}, uart2: {=bool:?}, i2c: {=bool:?} }}",
                self.gtim(),
                self.wwdt(),
                self.iwdt(),
                self.uart2(),
                self.i2c()
            )
        }
    }
    #[doc = "APB Reset Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst2(pub u32);
    impl Apbrst2 {
        #[doc = "desc ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC."]
        #[inline(always)]
        pub const fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VC."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI."]
        #[inline(always)]
        pub const fn set_spi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn btim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM."]
        #[inline(always)]
        pub const fn set_btim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc AWT."]
        #[must_use]
        #[inline(always)]
        pub const fn awt(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWT."]
        #[inline(always)]
        pub const fn set_awt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("spi", &self.spi())
                .field("uart1", &self.uart1())
                .field("btim", &self.btim())
                .field("awt", &self.awt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst2 {{ adc: {=bool:?}, vc: {=bool:?}, spi: {=bool:?}, uart1: {=bool:?}, btim: {=bool:?}, awt: {=bool:?} }}",
                self.adc(),
                self.vc(),
                self.spi(),
                self.uart1(),
                self.btim(),
                self.awt()
            )
        }
    }
    #[doc = "Control Reg0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "System clock source; codes other than HSI, HEX and LSI are reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclk(&self) -> super::vals::Sysclk {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sysclk::from_bits(val as u8)
        }
        #[doc = "System clock source; codes other than HSI, HEX and LSI are reserved."]
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
        #[doc = "desc HEXEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hexen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXEN."]
        #[inline(always)]
        pub const fn set_hexen(&mut self, val: bool) {
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
                .field("hexen", &self.hexen())
                .field("lsien", &self.lsien())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ hsien: {=bool:?}, hexen: {=bool:?}, lsien: {=bool:?}, key: {=u16:?} }}",
                self.hsien(),
                self.hexen(),
                self.lsien(),
                self.key()
            )
        }
    }
    #[doc = "Control Reg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc RSTIO."]
        #[must_use]
        #[inline(always)]
        pub const fn rstio(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTIO."]
        #[inline(always)]
        pub const fn set_rstio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
                .field("rstio", &self.rstio())
                .field("swdio", &self.swdio())
                .field("lockup", &self.lockup())
                .field("wakeupclk", &self.wakeupclk())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ rstio: {=bool:?}, swdio: {=bool:?}, lockup: {=bool:?}, wakeupclk: {=bool:?}, key: {=u16:?} }}",
                self.rstio(),
                self.swdio(),
                self.lockup(),
                self.wakeupclk(),
                self.key()
            )
        }
    }
    #[doc = "Debug Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Debug(pub u32);
    impl Debug {
        #[doc = "desc GTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM."]
        #[inline(always)]
        pub const fn set_gtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "desc AWT."]
        #[must_use]
        #[inline(always)]
        pub const fn awt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWT."]
        #[inline(always)]
        pub const fn set_awt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        #[doc = "desc WWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc WWDT."]
        #[inline(always)]
        pub const fn set_wwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("gtim", &self.gtim())
                .field("btim123", &self.btim123())
                .field("awt", &self.awt())
                .field("iwdt", &self.iwdt())
                .field("wwdt", &self.wwdt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Debug {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Debug {{ gtim: {=bool:?}, btim123: {=bool:?}, awt: {=bool:?}, iwdt: {=bool:?}, wwdt: {=bool:?} }}",
                self.gtim(),
                self.btim123(),
                self.awt(),
                self.iwdt(),
                self.wwdt()
            )
        }
    }
    #[doc = "GTIM CAP Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtimcap(pub u32);
    impl Gtimcap {
        #[doc = "desc CH1."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH1."]
        #[inline(always)]
        pub const fn set_ch1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CH2."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH2."]
        #[inline(always)]
        pub const fn set_ch2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc CH3."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH3."]
        #[inline(always)]
        pub const fn set_ch3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc CH4."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH4."]
        #[inline(always)]
        pub const fn set_ch4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Gtimcap {
        #[inline(always)]
        fn default() -> Gtimcap {
            Gtimcap(0)
        }
    }
    impl core::fmt::Debug for Gtimcap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtimcap")
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtimcap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtimcap {{ ch1: {=u8:?}, ch2: {=u8:?}, ch3: {=u8:?}, ch4: {=u8:?} }}",
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4()
            )
        }
    }
    #[doc = "GTIM ETR Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtimetr(pub u32);
    impl Gtimetr {
        #[doc = "desc GTIMETR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtimetr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIMETR."]
        #[inline(always)]
        pub const fn set_gtimetr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Gtimetr {
        #[inline(always)]
        fn default() -> Gtimetr {
            Gtimetr(0)
        }
    }
    impl core::fmt::Debug for Gtimetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtimetr")
                .field("gtimetr", &self.gtimetr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtimetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gtimetr {{ gtimetr: {=u8:?} }}", self.gtimetr())
        }
    }
    #[doc = "External input HEX control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hex(pub u32);
    impl Hex {
        #[doc = "Enable the selected external clock input into HEX; configure before starting HEX."]
        #[must_use]
        #[inline(always)]
        pub const fn pinen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the selected external clock input into HEX; configure before starting HEX."]
        #[inline(always)]
        pub const fn set_pinen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External clock input pin; configure before starting HEX."]
        #[must_use]
        #[inline(always)]
        pub const fn pinmux(&self) -> super::vals::HexInput {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::HexInput::from_bits(val as u8)
        }
        #[doc = "External clock input pin; configure before starting HEX."]
        #[inline(always)]
        pub const fn set_pinmux(&mut self, val: super::vals::HexInput) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Read-only startup stability latch; clears when disabled, not on a subsequent running failure."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
    }
    impl Default for Hex {
        #[inline(always)]
        fn default() -> Hex {
            Hex(0)
        }
    }
    impl core::fmt::Debug for Hex {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hex")
                .field("pinen", &self.pinen())
                .field("pinmux", &self.pinmux())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hex {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hex {{ pinen: {=bool:?}, pinmux: {:?}, stable: {=bool:?} }}",
                self.pinen(),
                self.pinmux(),
                self.stable()
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
        #[doc = "desc HEXRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hexrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXRDY."]
        #[inline(always)]
        pub const fn set_hexrdy(&mut self, val: bool) {
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
                .field("hexrdy", &self.hexrdy())
                .field("lsirdy", &self.lsirdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ hsirdy: {=bool:?}, hexrdy: {=bool:?}, lsirdy: {=bool:?} }}",
                self.hsirdy(),
                self.hexrdy(),
                self.lsirdy()
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
        #[doc = "desc HEXRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hexrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXRDY."]
        #[inline(always)]
        pub const fn set_hexrdy(&mut self, val: bool) {
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
                .field("hexrdy", &self.hexrdy())
                .field("lsirdy", &self.lsirdy())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ hsirdy: {=bool:?}, hexrdy: {=bool:?}, lsirdy: {=bool:?}, key: {=u16:?} }}",
                self.hsirdy(),
                self.hexrdy(),
                self.lsirdy(),
                self.key()
            )
        }
    }
    #[doc = "IR MOD Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irmod(pub u32);
    impl Irmod {
        #[doc = "desc MOD."]
        #[must_use]
        #[inline(always)]
        pub const fn mod_(&self) -> super::vals::IrMode {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::IrMode::from_bits(val as u8)
        }
        #[doc = "desc MOD."]
        #[inline(always)]
        pub const fn set_mod_(&mut self, val: super::vals::IrMode) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc IRSW."]
        #[must_use]
        #[inline(always)]
        pub const fn irsw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRSW."]
        #[inline(always)]
        pub const fn set_irsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Irmod {
        #[inline(always)]
        fn default() -> Irmod {
            Irmod(0)
        }
    }
    impl core::fmt::Debug for Irmod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irmod")
                .field("mod_", &self.mod_())
                .field("irsw", &self.irsw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irmod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Irmod {{ mod_: {:?}, irsw: {=bool:?} }}",
                self.mod_(),
                self.irsw()
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
        #[doc = "desc HEXRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn hexrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXRDY."]
        #[inline(always)]
        pub const fn set_hexrdy(&mut self, val: bool) {
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
        #[doc = "desc HEXSTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn hexstable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HEXSTABLE."]
        #[inline(always)]
        pub const fn set_hexstable(&mut self, val: bool) {
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
                .field("hexrdy", &self.hexrdy())
                .field("lsirdy", &self.lsirdy())
                .field("hsistable", &self.hsistable())
                .field("hexstable", &self.hexstable())
                .field("lsistable", &self.lsistable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ hsirdy: {=bool:?}, hexrdy: {=bool:?}, lsirdy: {=bool:?}, hsistable: {=bool:?}, hexstable: {=bool:?}, lsistable: {=bool:?} }}",
                self.hsirdy(),
                self.hexrdy(),
                self.lsirdy(),
                self.hsistable(),
                self.hexstable(),
                self.lsistable()
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
        #[doc = "desc WWDT."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WWDT."]
        #[inline(always)]
        pub const fn set_wwdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("wwdt", &self.wwdt())
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
                "Resetflag {{ por: {=bool:?}, lvd: {=bool:?}, iwdt: {=bool:?}, wwdt: {=bool:?}, rstb: {=bool:?}, lockup: {=bool:?}, sysresetreq: {=bool:?} }}",
                self.por(),
                self.lvd(),
                self.iwdt(),
                self.wwdt(),
                self.rstb(),
                self.lockup(),
                self.sysresetreq()
            )
        }
    }
    #[doc = "BTIMx GTIMx ATIM ITR Control Reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timitr(pub u32);
    impl Timitr {
        #[doc = "desc GTIMITR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtimitr(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIMITR."]
        #[inline(always)]
        pub const fn set_gtimitr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc BTIM1ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn btim1itr(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "desc BTIM1ITR."]
        #[inline(always)]
        pub const fn set_btim1itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "desc BTIM2ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn btim2itr(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "desc BTIM2ITR."]
        #[inline(always)]
        pub const fn set_btim2itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "desc BTIM3ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn btim3itr(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "desc BTIM3ITR."]
        #[inline(always)]
        pub const fn set_btim3itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
    }
    impl Default for Timitr {
        #[inline(always)]
        fn default() -> Timitr {
            Timitr(0)
        }
    }
    impl core::fmt::Debug for Timitr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timitr")
                .field("gtimitr", &self.gtimitr())
                .field("btim1itr", &self.btim1itr())
                .field("btim2itr", &self.btim2itr())
                .field("btim3itr", &self.btim3itr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timitr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timitr {{ gtimitr: {=u8:?}, btim1itr: {=u8:?}, btim2itr: {=u8:?}, btim3itr: {=u8:?} }}",
                self.gtimitr(),
                self.btim1itr(),
                self.btim2itr(),
                self.btim3itr()
            )
        }
    }
}
pub mod vals {
    #[doc = "CW32F002 own RM Rev 1.4 PDF p57; selectable external input pads, neither is a crystal output."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HexInput {
        #[doc = "External clock input from PB00."]
        Pb0 = 0x0,
        #[doc = "External clock input from PB01."]
        Pb1 = 0x01,
    }
    impl HexInput {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HexInput {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HexInput {
        #[inline(always)]
        fn from(val: u8) -> HexInput {
            HexInput::from_bits(val)
        }
    }
    impl From<HexInput> for u8 {
        #[inline(always)]
        fn from(val: HexInput) -> u8 {
            HexInput::to_bits(val)
        }
    }
    #[doc = "Documented infrared modulator inputs and boolean combination; source timers/UARTs are configured independently."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMode {
        #[doc = "GTIM_CH1 & BTIM2_TOGP"]
        GtimCh1AndBtim2Togp = 0x0,
        #[doc = "GTIM_CH1 & IRSW"]
        GtimCh1AndIrsw = 0x01,
        #[doc = "GTIM_CH2 & BTIM3_TOGP"]
        GtimCh2AndBtim3Togp = 0x02,
        #[doc = "GTIM_CH2 & IRSW"]
        GtimCh2AndIrsw = 0x03,
        #[doc = "GTIM_CH1 | BTIM2_TOGP"]
        GtimCh1OrBtim2Togp = 0x04,
        #[doc = "GTIM_CH1 | IRSW"]
        GtimCh1OrIrsw = 0x05,
        #[doc = "GTIM_CH2 | BTIM3_TOGP"]
        GtimCh2OrBtim3Togp = 0x06,
        #[doc = "GTIM_CH2 | IRSW"]
        GtimCh2OrIrsw = 0x07,
        #[doc = "UART1_TXD & GTIM_CH1"]
        Uart1TxdAndGtimCh1 = 0x08,
        #[doc = "UART1_TXD | GTIM_CH1"]
        Uart1TxdOrGtimCh1 = 0x09,
        #[doc = "UART1_TXD & GTIM_CH2"]
        Uart1TxdAndGtimCh2 = 0x0a,
        #[doc = "UART1_TXD | GTIM_CH2"]
        Uart1TxdOrGtimCh2 = 0x0b,
        #[doc = "UART2_TXD & BTIM2_TOGP"]
        Uart2TxdAndBtim2Togp = 0x0c,
        #[doc = "UART2_TXD | BTIM2_TOGP"]
        Uart2TxdOrBtim2Togp = 0x0d,
        #[doc = "UART2_TXD & BTIM3_TOGP"]
        Uart2TxdAndBtim3Togp = 0x0e,
        #[doc = "UART2_TXD | BTIM3_TOGP"]
        Uart2TxdOrBtim3Togp = 0x0f,
    }
    impl IrMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMode {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMode {
        #[inline(always)]
        fn from(val: u8) -> IrMode {
            IrMode::from_bits(val)
        }
    }
    impl From<IrMode> for u8 {
        #[inline(always)]
        fn from(val: IrMode) -> u8 {
            IrMode::to_bits(val)
        }
    }
    #[doc = "CW32F002 own RM Rev 1.4 PDF p54; direct system sources, other codes reserved. See cw32-data/hex-qualified.yaml."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sysclk {
        #[doc = "Divided internal high-speed oscillator."]
        Hsi = 0x0,
        #[doc = "External clock input selected by HEX.PINMUX."]
        Hex = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Internal low-speed oscillator."]
        Lsi = 0x03,
        _RESERVED_4 = 0x04,
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
