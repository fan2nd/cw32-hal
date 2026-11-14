#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SYSCTRL."]
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
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc HSI."]
    #[inline(always)]
    pub const fn hsi(self) -> crate::common::Reg<regs::Hsi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc HSE."]
    #[inline(always)]
    pub const fn hse(self) -> crate::common::Reg<regs::Hse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc LSI."]
    #[inline(always)]
    pub const fn lsi(self) -> crate::common::Reg<regs::Lsi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc LSE."]
    #[inline(always)]
    pub const fn lse(self) -> crate::common::Reg<regs::Lse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc PLL."]
    #[inline(always)]
    pub const fn pll(self) -> crate::common::Reg<regs::Pll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc DEBUG."]
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc AHBEN."]
    #[inline(always)]
    pub const fn ahben(self) -> crate::common::Reg<regs::Ahben, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc APBEN2."]
    #[inline(always)]
    pub const fn apben2(self) -> crate::common::Reg<regs::Apben2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc APBEN1."]
    #[inline(always)]
    pub const fn apben1(self) -> crate::common::Reg<regs::Apben1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc AHBRST."]
    #[inline(always)]
    pub const fn ahbrst(self) -> crate::common::Reg<regs::Ahbrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc APBRST2."]
    #[inline(always)]
    pub const fn apbrst2(self) -> crate::common::Reg<regs::Apbrst2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc APBRST1."]
    #[inline(always)]
    pub const fn apbrst1(self) -> crate::common::Reg<regs::Apbrst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc RESETFLAG."]
    #[inline(always)]
    pub const fn resetflag(self) -> crate::common::Reg<regs::Resetflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc GTIM1CAP."]
    #[inline(always)]
    pub const fn gtim1cap(self) -> crate::common::Reg<regs::Gtim1cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc GTIM2CAP."]
    #[inline(always)]
    pub const fn gtim2cap(self) -> crate::common::Reg<regs::Gtim2cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc GTIM3CAP."]
    #[inline(always)]
    pub const fn gtim3cap(self) -> crate::common::Reg<regs::Gtim3cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc GTIM4CAP."]
    #[inline(always)]
    pub const fn gtim4cap(self) -> crate::common::Reg<regs::Gtim4cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc ATIMETR."]
    #[inline(always)]
    pub const fn atimetr(self) -> crate::common::Reg<regs::Atimetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc GTIMETR."]
    #[inline(always)]
    pub const fn gtimetr(self) -> crate::common::Reg<regs::Gtimetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc TIMITR."]
    #[inline(always)]
    pub const fn timitr(self) -> crate::common::Reg<regs::Timitr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc MCO."]
    #[inline(always)]
    pub const fn mco(self) -> crate::common::Reg<regs::Mco, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc IRMOD."]
    #[inline(always)]
    pub const fn irmod(self) -> crate::common::Reg<regs::Irmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc AHBEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahben(pub u32);
    impl Ahben {
        #[doc = "desc DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
        #[doc = "desc GPIOD."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOD."]
        #[inline(always)]
        pub const fn set_gpiod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc GPIOE."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOE."]
        #[inline(always)]
        pub const fn set_gpioe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc GPIOF."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiof(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOF."]
        #[inline(always)]
        pub const fn set_gpiof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TRNG."]
        #[must_use]
        #[inline(always)]
        pub const fn trng(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG."]
        #[inline(always)]
        pub const fn set_trng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc AES."]
        #[must_use]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc AES."]
        #[inline(always)]
        pub const fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("dma", &self.dma())
                .field("flash", &self.flash())
                .field("crc", &self.crc())
                .field("gpioa", &self.gpioa())
                .field("gpiob", &self.gpiob())
                .field("gpioc", &self.gpioc())
                .field("gpiod", &self.gpiod())
                .field("gpioe", &self.gpioe())
                .field("gpiof", &self.gpiof())
                .field("trng", &self.trng())
                .field("aes", &self.aes())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahben {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahben {{ dma: {=bool:?}, flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?}, gpiod: {=bool:?}, gpioe: {=bool:?}, gpiof: {=bool:?}, trng: {=bool:?}, aes: {=bool:?} }}",
                self.dma(),
                self.flash(),
                self.crc(),
                self.gpioa(),
                self.gpiob(),
                self.gpioc(),
                self.gpiod(),
                self.gpioe(),
                self.gpiof(),
                self.trng(),
                self.aes()
            )
        }
    }
    #[doc = "desc AHBRST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrst(pub u32);
    impl Ahbrst {
        #[doc = "desc DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
        #[doc = "desc GPIOD."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOD."]
        #[inline(always)]
        pub const fn set_gpiod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc GPIOE."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOE."]
        #[inline(always)]
        pub const fn set_gpioe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc GPIOF."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiof(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIOF."]
        #[inline(always)]
        pub const fn set_gpiof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TRNG."]
        #[must_use]
        #[inline(always)]
        pub const fn trng(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG."]
        #[inline(always)]
        pub const fn set_trng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc AES."]
        #[must_use]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc AES."]
        #[inline(always)]
        pub const fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("dma", &self.dma())
                .field("flash", &self.flash())
                .field("crc", &self.crc())
                .field("gpioa", &self.gpioa())
                .field("gpiob", &self.gpiob())
                .field("gpioc", &self.gpioc())
                .field("gpiod", &self.gpiod())
                .field("gpioe", &self.gpioe())
                .field("gpiof", &self.gpiof())
                .field("trng", &self.trng())
                .field("aes", &self.aes())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahbrst {{ dma: {=bool:?}, flash: {=bool:?}, crc: {=bool:?}, gpioa: {=bool:?}, gpiob: {=bool:?}, gpioc: {=bool:?}, gpiod: {=bool:?}, gpioe: {=bool:?}, gpiof: {=bool:?}, trng: {=bool:?}, aes: {=bool:?} }}",
                self.dma(),
                self.flash(),
                self.crc(),
                self.gpioa(),
                self.gpiob(),
                self.gpioc(),
                self.gpiod(),
                self.gpioe(),
                self.gpiof(),
                self.trng(),
                self.aes()
            )
        }
    }
    #[doc = "desc APBEN1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apben1(pub u32);
    impl Apben1 {
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
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "desc SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        #[doc = "desc UART4."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART4."]
        #[inline(always)]
        pub const fn set_uart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc UART5."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART5."]
        #[inline(always)]
        pub const fn set_uart5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C1."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc I2C2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C2."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LCD."]
        #[must_use]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc LCD."]
        #[inline(always)]
        pub const fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("gtim1", &self.gtim1())
                .field("gtim2", &self.gtim2())
                .field("rtc", &self.rtc())
                .field("wwdt", &self.wwdt())
                .field("iwdt", &self.iwdt())
                .field("spi2", &self.spi2())
                .field("uart2", &self.uart2())
                .field("uart3", &self.uart3())
                .field("uart4", &self.uart4())
                .field("uart5", &self.uart5())
                .field("i2c1", &self.i2c1())
                .field("i2c2", &self.i2c2())
                .field("lcd", &self.lcd())
                .field("lptim", &self.lptim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben1 {{ gtim1: {=bool:?}, gtim2: {=bool:?}, rtc: {=bool:?}, wwdt: {=bool:?}, iwdt: {=bool:?}, spi2: {=bool:?}, uart2: {=bool:?}, uart3: {=bool:?}, uart4: {=bool:?}, uart5: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, lcd: {=bool:?}, lptim: {=bool:?} }}",
                self.gtim1(),
                self.gtim2(),
                self.rtc(),
                self.wwdt(),
                self.iwdt(),
                self.spi2(),
                self.uart2(),
                self.uart3(),
                self.uart4(),
                self.uart5(),
                self.i2c1(),
                self.i2c2(),
                self.lcd(),
                self.lptim()
            )
        }
    }
    #[doc = "desc APBEN2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apben2(pub u32);
    impl Apben2 {
        #[doc = "desc UART6."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART6."]
        #[inline(always)]
        pub const fn set_uart6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
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
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
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
        #[doc = "desc GTIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3."]
        #[inline(always)]
        pub const fn set_gtim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc GTIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4."]
        #[inline(always)]
        pub const fn set_gtim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "desc AUTOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn autotrim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTOTRIM."]
        #[inline(always)]
        pub const fn set_autotrim(&mut self, val: bool) {
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
                .field("uart6", &self.uart6())
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("atim", &self.atim())
                .field("spi1", &self.spi1())
                .field("uart1", &self.uart1())
                .field("gtim3", &self.gtim3())
                .field("gtim4", &self.gtim4())
                .field("btim", &self.btim())
                .field("autotrim", &self.autotrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apben2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apben2 {{ uart6: {=bool:?}, adc: {=bool:?}, vc: {=bool:?}, atim: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, gtim3: {=bool:?}, gtim4: {=bool:?}, btim: {=bool:?}, autotrim: {=bool:?} }}",
                self.uart6(),
                self.adc(),
                self.vc(),
                self.atim(),
                self.spi1(),
                self.uart1(),
                self.gtim3(),
                self.gtim4(),
                self.btim(),
                self.autotrim()
            )
        }
    }
    #[doc = "desc APBRST1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst1(pub u32);
    impl Apbrst1 {
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
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "desc SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        #[doc = "desc UART4."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART4."]
        #[inline(always)]
        pub const fn set_uart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc UART5."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART5."]
        #[inline(always)]
        pub const fn set_uart5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C1."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc I2C2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C2."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LCD."]
        #[must_use]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc LCD."]
        #[inline(always)]
        pub const fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("gtim1", &self.gtim1())
                .field("gtim2", &self.gtim2())
                .field("rtc", &self.rtc())
                .field("wwdt", &self.wwdt())
                .field("iwdt", &self.iwdt())
                .field("spi2", &self.spi2())
                .field("uart2", &self.uart2())
                .field("uart3", &self.uart3())
                .field("uart4", &self.uart4())
                .field("uart5", &self.uart5())
                .field("i2c1", &self.i2c1())
                .field("i2c2", &self.i2c2())
                .field("lcd", &self.lcd())
                .field("lptim", &self.lptim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst1 {{ gtim1: {=bool:?}, gtim2: {=bool:?}, rtc: {=bool:?}, wwdt: {=bool:?}, iwdt: {=bool:?}, spi2: {=bool:?}, uart2: {=bool:?}, uart3: {=bool:?}, uart4: {=bool:?}, uart5: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, lcd: {=bool:?}, lptim: {=bool:?} }}",
                self.gtim1(),
                self.gtim2(),
                self.rtc(),
                self.wwdt(),
                self.iwdt(),
                self.spi2(),
                self.uart2(),
                self.uart3(),
                self.uart4(),
                self.uart5(),
                self.i2c1(),
                self.i2c2(),
                self.lcd(),
                self.lptim()
            )
        }
    }
    #[doc = "desc APBRST2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbrst2(pub u32);
    impl Apbrst2 {
        #[doc = "desc UART6."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART6."]
        #[inline(always)]
        pub const fn set_uart6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
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
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
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
        #[doc = "desc GTIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3."]
        #[inline(always)]
        pub const fn set_gtim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc GTIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4."]
        #[inline(always)]
        pub const fn set_gtim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "desc AUTOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn autotrim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTOTRIM."]
        #[inline(always)]
        pub const fn set_autotrim(&mut self, val: bool) {
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
                .field("uart6", &self.uart6())
                .field("adc", &self.adc())
                .field("vc", &self.vc())
                .field("atim", &self.atim())
                .field("spi1", &self.spi1())
                .field("uart1", &self.uart1())
                .field("gtim3", &self.gtim3())
                .field("gtim4", &self.gtim4())
                .field("btim", &self.btim())
                .field("autotrim", &self.autotrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbrst2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbrst2 {{ uart6: {=bool:?}, adc: {=bool:?}, vc: {=bool:?}, atim: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, gtim3: {=bool:?}, gtim4: {=bool:?}, btim: {=bool:?}, autotrim: {=bool:?} }}",
                self.uart6(),
                self.adc(),
                self.vc(),
                self.atim(),
                self.spi1(),
                self.uart1(),
                self.gtim3(),
                self.gtim4(),
                self.btim(),
                self.autotrim()
            )
        }
    }
    #[doc = "desc ATIMETR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atimetr(pub u32);
    impl Atimetr {
        #[doc = "desc ATIMETR."]
        #[must_use]
        #[inline(always)]
        pub const fn atimetr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc ATIMETR."]
        #[inline(always)]
        pub const fn set_atimetr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Atimetr {
        #[inline(always)]
        fn default() -> Atimetr {
            Atimetr(0)
        }
    }
    impl core::fmt::Debug for Atimetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atimetr")
                .field("atimetr", &self.atimetr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atimetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Atimetr {{ atimetr: {=u8:?} }}", self.atimetr())
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "System-clock source; own RM section 4.7.1 PDF75."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclk(&self) -> super::vals::Sysclk {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sysclk::from_bits(val as u8)
        }
        #[doc = "System-clock source; own RM section 4.7.1 PDF75."]
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
    #[doc = "desc CR1."]
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
        #[doc = "desc PLLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pllen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLEN."]
        #[inline(always)]
        pub const fn set_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("pllen", &self.pllen())
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
                "Cr1 {{ hsien: {=bool:?}, hseen: {=bool:?}, pllen: {=bool:?}, lsien: {=bool:?}, lseen: {=bool:?}, lselock: {=bool:?}, lseccs: {=bool:?}, hseccs: {=bool:?}, clkccs: {=bool:?}, key: {=u16:?} }}",
                self.hsien(),
                self.hseen(),
                self.pllen(),
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
    #[doc = "desc CR2."]
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
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ swdio: {=bool:?}, lockup: {=bool:?}, wakeupclk: {=bool:?}, flashwait: {=u8:?}, key: {=u16:?} }}",
                self.swdio(),
                self.lockup(),
                self.wakeupclk(),
                self.flashwait(),
                self.key()
            )
        }
    }
    #[doc = "desc DEBUG."]
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
        #[doc = "desc GTIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3."]
        #[inline(always)]
        pub const fn set_gtim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc GTIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4."]
        #[inline(always)]
        pub const fn set_gtim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        #[doc = "desc AUTOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn autotrim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTOTRIM."]
        #[inline(always)]
        pub const fn set_autotrim(&mut self, val: bool) {
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
        #[doc = "desc LPTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPTIM."]
        #[inline(always)]
        pub const fn set_lptim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("gtim3", &self.gtim3())
                .field("gtim4", &self.gtim4())
                .field("btim123", &self.btim123())
                .field("autotrim", &self.autotrim())
                .field("rtc", &self.rtc())
                .field("iwdt", &self.iwdt())
                .field("wwdt", &self.wwdt())
                .field("lptim", &self.lptim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Debug {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Debug {{ atim: {=bool:?}, gtim1: {=bool:?}, gtim2: {=bool:?}, gtim3: {=bool:?}, gtim4: {=bool:?}, btim123: {=bool:?}, autotrim: {=bool:?}, rtc: {=bool:?}, iwdt: {=bool:?}, wwdt: {=bool:?}, lptim: {=bool:?} }}",
                self.atim(),
                self.gtim1(),
                self.gtim2(),
                self.gtim3(),
                self.gtim4(),
                self.btim123(),
                self.autotrim(),
                self.rtc(),
                self.iwdt(),
                self.wwdt(),
                self.lptim()
            )
        }
    }
    #[doc = "desc GTIM1CAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtim1cap(pub u32);
    impl Gtim1cap {
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
    impl Default for Gtim1cap {
        #[inline(always)]
        fn default() -> Gtim1cap {
            Gtim1cap(0)
        }
    }
    impl core::fmt::Debug for Gtim1cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtim1cap")
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtim1cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtim1cap {{ ch1: {=u8:?}, ch2: {=u8:?}, ch3: {=u8:?}, ch4: {=u8:?} }}",
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4()
            )
        }
    }
    #[doc = "desc GTIM2CAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtim2cap(pub u32);
    impl Gtim2cap {
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
    impl Default for Gtim2cap {
        #[inline(always)]
        fn default() -> Gtim2cap {
            Gtim2cap(0)
        }
    }
    impl core::fmt::Debug for Gtim2cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtim2cap")
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtim2cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtim2cap {{ ch1: {=u8:?}, ch2: {=u8:?}, ch3: {=u8:?}, ch4: {=u8:?} }}",
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4()
            )
        }
    }
    #[doc = "desc GTIM3CAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtim3cap(pub u32);
    impl Gtim3cap {
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
    impl Default for Gtim3cap {
        #[inline(always)]
        fn default() -> Gtim3cap {
            Gtim3cap(0)
        }
    }
    impl core::fmt::Debug for Gtim3cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtim3cap")
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtim3cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtim3cap {{ ch1: {=u8:?}, ch2: {=u8:?}, ch3: {=u8:?}, ch4: {=u8:?} }}",
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4()
            )
        }
    }
    #[doc = "desc GTIM4CAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtim4cap(pub u32);
    impl Gtim4cap {
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
    impl Default for Gtim4cap {
        #[inline(always)]
        fn default() -> Gtim4cap {
            Gtim4cap(0)
        }
    }
    impl core::fmt::Debug for Gtim4cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtim4cap")
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtim4cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtim4cap {{ ch1: {=u8:?}, ch2: {=u8:?}, ch3: {=u8:?}, ch4: {=u8:?} }}",
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4()
            )
        }
    }
    #[doc = "desc GTIMETR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtimetr(pub u32);
    impl Gtimetr {
        #[doc = "desc GTIM1ETR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1etr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM1ETR."]
        #[inline(always)]
        pub const fn set_gtim1etr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc GTIM2ETR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2etr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM2ETR."]
        #[inline(always)]
        pub const fn set_gtim2etr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc GTIM3ETR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3etr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM3ETR."]
        #[inline(always)]
        pub const fn set_gtim3etr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc GTIM4ETR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4etr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM4ETR."]
        #[inline(always)]
        pub const fn set_gtim4etr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
                .field("gtim1etr", &self.gtim1etr())
                .field("gtim2etr", &self.gtim2etr())
                .field("gtim3etr", &self.gtim3etr())
                .field("gtim4etr", &self.gtim4etr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtimetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gtimetr {{ gtim1etr: {=u8:?}, gtim2etr: {=u8:?}, gtim3etr: {=u8:?}, gtim4etr: {=u8:?} }}",
                self.gtim1etr(),
                self.gtim2etr(),
                self.gtim3etr(),
                self.gtim4etr()
            )
        }
    }
    #[doc = "HSE control; own RM section 4.7.6 PDF80. Preserve reserved bits31:20."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hse(pub u32);
    impl Hse {
        #[doc = "desc DRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn driver(&self) -> super::vals::HseDrive {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::HseDrive::from_bits(val as u8)
        }
        #[doc = "desc DRIVER."]
        #[inline(always)]
        pub const fn set_driver(&mut self, val: super::vals::HseDrive) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "desc FREQRANGE."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> super::vals::HseRange {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::HseRange::from_bits(val as u8)
        }
        #[doc = "desc FREQRANGE."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: super::vals::HseRange) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
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
        #[doc = "desc FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn flt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLT."]
        #[inline(always)]
        pub const fn set_flt(&mut self, val: bool) {
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
        #[doc = "Read-only startup latch; disabling HSE clears it, later source failure does not."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
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
                .field("freqrange", &self.freqrange())
                .field("waitcycle", &self.waitcycle())
                .field("mode", &self.mode())
                .field("flt", &self.flt())
                .field("detcnt", &self.detcnt())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hse {{ driver: {:?}, freqrange: {:?}, waitcycle: {:?}, mode: {=bool:?}, flt: {=bool:?}, detcnt: {=u16:?}, stable: {=bool:?} }}",
                self.driver(),
                self.freqrange(),
                self.waitcycle(),
                self.mode(),
                self.flt(),
                self.detcnt(),
                self.stable()
            )
        }
    }
    #[doc = "desc HSI."]
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
    #[doc = "desc ICR."]
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
        #[doc = "desc PLLRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLRDY."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("pllrdy", &self.pllrdy())
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
                "Icr {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, pllrdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.pllrdy(),
                self.lsirdy(),
                self.lserdy(),
                self.lsefail(),
                self.hsefail(),
                self.lsefault(),
                self.hsefault()
            )
        }
    }
    #[doc = "desc IER."]
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
        #[doc = "desc PLLRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLRDY."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("pllrdy", &self.pllrdy())
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
                "Ier {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, pllrdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?}, key: {=u16:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.pllrdy(),
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
    #[doc = "desc IRMOD."]
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
    }
    impl Default for Irmod {
        #[inline(always)]
        fn default() -> Irmod {
            Irmod(0)
        }
    }
    impl core::fmt::Debug for Irmod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irmod").field("mod_", &self.mod_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irmod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Irmod {{ mod_: {:?} }}", self.mod_())
        }
    }
    #[doc = "desc ISR."]
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
        #[doc = "desc PLLRDY."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLRDY."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PLLSTABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn pllstable(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLSTABLE."]
        #[inline(always)]
        pub const fn set_pllstable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("pllrdy", &self.pllrdy())
                .field("lsirdy", &self.lsirdy())
                .field("lserdy", &self.lserdy())
                .field("lsefail", &self.lsefail())
                .field("hsefail", &self.hsefail())
                .field("lsefault", &self.lsefault())
                .field("hsefault", &self.hsefault())
                .field("hsistable", &self.hsistable())
                .field("hsestable", &self.hsestable())
                .field("pllstable", &self.pllstable())
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
                "Isr {{ hsirdy: {=bool:?}, hserdy: {=bool:?}, pllrdy: {=bool:?}, lsirdy: {=bool:?}, lserdy: {=bool:?}, lsefail: {=bool:?}, hsefail: {=bool:?}, lsefault: {=bool:?}, hsefault: {=bool:?}, hsistable: {=bool:?}, hsestable: {=bool:?}, pllstable: {=bool:?}, lsistable: {=bool:?}, lsestable: {=bool:?} }}",
                self.hsirdy(),
                self.hserdy(),
                self.pllrdy(),
                self.lsirdy(),
                self.lserdy(),
                self.lsefail(),
                self.hsefail(),
                self.lsefault(),
                self.hsefault(),
                self.hsistable(),
                self.hsestable(),
                self.pllstable(),
                self.lsistable(),
                self.lsestable()
            )
        }
    }
    #[doc = "desc LSE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lse(pub u32);
    impl Lse {
        #[doc = "desc DRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn driver(&self) -> super::vals::LseDrive {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::LseDrive::from_bits(val as u8)
        }
        #[doc = "desc DRIVER."]
        #[inline(always)]
        pub const fn set_driver(&mut self, val: super::vals::LseDrive) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "desc AMP."]
        #[must_use]
        #[inline(always)]
        pub const fn amp(&self) -> super::vals::LseAmplitude {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::LseAmplitude::from_bits(val as u8)
        }
        #[doc = "desc AMP."]
        #[inline(always)]
        pub const fn set_amp(&mut self, val: super::vals::LseAmplitude) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
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
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
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
                .field("amp", &self.amp())
                .field("waitcycle", &self.waitcycle())
                .field("mode", &self.mode())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lse {{ driver: {:?}, amp: {:?}, waitcycle: {:?}, mode: {=bool:?}, stable: {=bool:?} }}",
                self.driver(),
                self.amp(),
                self.waitcycle(),
                self.mode(),
                self.stable()
            )
        }
    }
    #[doc = "desc LSI."]
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
    #[doc = "desc MCO."]
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
    #[doc = "L083 PLL control. Preserve reserved debug default0x5 at19:16. UM Rev2.0 section4.7.8 PDF82."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll(pub u32);
    impl Pll {
        #[doc = "L083 PLL SOURCE; UM section4.7.8."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> super::vals::PllSource {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::PllSource::from_bits(val as u8)
        }
        #[doc = "L083 PLL SOURCE; UM section4.7.8."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: super::vals::PllSource) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "L083 PLL FREQIN; UM section4.7.8."]
        #[must_use]
        #[inline(always)]
        pub const fn freqin(&self) -> super::vals::PllInputRange {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::PllInputRange::from_bits(val as u8)
        }
        #[doc = "L083 PLL FREQIN; UM section4.7.8."]
        #[inline(always)]
        pub const fn set_freqin(&mut self, val: super::vals::PllInputRange) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "L083 PLL MUL; UM section4.7.8."]
        #[must_use]
        #[inline(always)]
        pub const fn mul(&self) -> super::vals::PllMul {
            let val = (self.0 >> 4usize) & 0x1f;
            super::vals::PllMul::from_bits(val as u8)
        }
        #[doc = "L083 PLL MUL; UM section4.7.8."]
        #[inline(always)]
        pub const fn set_mul(&mut self, val: super::vals::PllMul) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
        }
        #[doc = "L083 PLL FREQOUT; UM section4.7.8."]
        #[must_use]
        #[inline(always)]
        pub const fn freqout(&self) -> super::vals::PllOutputRange {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::PllOutputRange::from_bits(val as u8)
        }
        #[doc = "L083 PLL FREQOUT; UM section4.7.8."]
        #[inline(always)]
        pub const fn set_freqout(&mut self, val: super::vals::PllOutputRange) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "L083 PLL WAITCYCLE; UM section4.7.8."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcycle(&self) -> super::vals::PllWait {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::PllWait::from_bits(val as u8)
        }
        #[doc = "L083 PLL WAITCYCLE; UM section4.7.8."]
        #[inline(always)]
        pub const fn set_waitcycle(&mut self, val: super::vals::PllWait) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "Startup stable latch; not continuous lock-loss detection."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Documented reserved debug value; keep default."]
        #[must_use]
        #[inline(always)]
        pub const fn reserved_debug(&self) -> super::vals::PllDebug {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::PllDebug::from_bits(val as u8)
        }
        #[doc = "Documented reserved debug value; keep default."]
        #[inline(always)]
        pub const fn set_reserved_debug(&mut self, val: super::vals::PllDebug) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Pll {
        #[inline(always)]
        fn default() -> Pll {
            Pll(0)
        }
    }
    impl core::fmt::Debug for Pll {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll")
                .field("source", &self.source())
                .field("freqin", &self.freqin())
                .field("mul", &self.mul())
                .field("freqout", &self.freqout())
                .field("waitcycle", &self.waitcycle())
                .field("stable", &self.stable())
                .field("reserved_debug", &self.reserved_debug())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll {{ source: {:?}, freqin: {:?}, mul: {:?}, freqout: {:?}, waitcycle: {:?}, stable: {=bool:?}, reserved_debug: {:?} }}",
                self.source(),
                self.freqin(),
                self.mul(),
                self.freqout(),
                self.waitcycle(),
                self.stable(),
                self.reserved_debug()
            )
        }
    }
    #[doc = "desc RESETFLAG."]
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
    #[doc = "desc TIMITR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timitr(pub u32);
    impl Timitr {
        #[doc = "desc ATIMITR."]
        #[must_use]
        #[inline(always)]
        pub const fn atimitr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc ATIMITR."]
        #[inline(always)]
        pub const fn set_atimitr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc GTIM1ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1itr(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM1ITR."]
        #[inline(always)]
        pub const fn set_gtim1itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc GTIM2ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2itr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM2ITR."]
        #[inline(always)]
        pub const fn set_gtim2itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc GTIM3ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3itr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM3ITR."]
        #[inline(always)]
        pub const fn set_gtim3itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc GTIM4ITR."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4itr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc GTIM4ITR."]
        #[inline(always)]
        pub const fn set_gtim4itr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
                .field("atimitr", &self.atimitr())
                .field("gtim1itr", &self.gtim1itr())
                .field("gtim2itr", &self.gtim2itr())
                .field("gtim3itr", &self.gtim3itr())
                .field("gtim4itr", &self.gtim4itr())
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
                "Timitr {{ atimitr: {=u8:?}, gtim1itr: {=u8:?}, gtim2itr: {=u8:?}, gtim3itr: {=u8:?}, gtim4itr: {=u8:?}, btim1itr: {=u8:?}, btim2itr: {=u8:?}, btim3itr: {=u8:?} }}",
                self.atimitr(),
                self.gtim1itr(),
                self.gtim2itr(),
                self.gtim3itr(),
                self.gtim4itr(),
                self.btim1itr(),
                self.btim2itr(),
                self.btim3itr()
            )
        }
    }
}
pub mod vals {
    #[doc = "HSE crystal drive strengths; select for the board and resonator."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HseDrive {
        #[doc = "Weakest drive."]
        Level0 = 0x0,
        #[doc = "Weak drive."]
        Level1 = 0x01,
        #[doc = "Default recommended drive."]
        Level2 = 0x02,
        #[doc = "Strongest drive."]
        Level3 = 0x03,
    }
    impl HseDrive {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HseDrive {
            unsafe { core::mem::transmute(val & 0x03) }
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
    #[doc = "Own-manual HSE crystal nominal frequency ranges."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HseRange {
        #[doc = "4 to 8 MHz."]
        Mhz4To8 = 0x0,
        #[doc = "8 to 16 MHz."]
        Mhz8To16 = 0x01,
        #[doc = "16 to 24 MHz."]
        Mhz16To24 = 0x02,
        #[doc = "24 to 32 MHz."]
        Mhz24To32 = 0x03,
    }
    impl HseRange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HseRange {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HseRange {
        #[inline(always)]
        fn from(val: u8) -> HseRange {
            HseRange::from_bits(val)
        }
    }
    impl From<HseRange> for u8 {
        #[inline(always)]
        fn from(val: HseRange) -> u8 {
            HseRange::to_bits(val)
        }
    }
    #[doc = "HSE stable/startup failure detection cycle counts."]
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
    #[doc = "Documented infrared modulator inputs and boolean combination; source timers/UARTs are configured independently."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMode {
        #[doc = "GTIM1_CH1 & GTIM2_CH1"]
        Gtim1Ch1AndGtim2Ch1 = 0x0,
        #[doc = "GTIM1_CH1 & GTIM2_CH2"]
        Gtim1Ch1AndGtim2Ch2 = 0x01,
        #[doc = "GTIM1_CH2 & GTIM2_CH1"]
        Gtim1Ch2AndGtim2Ch1 = 0x02,
        #[doc = "GTIM1_CH2 & GTIM2_CH2"]
        Gtim1Ch2AndGtim2Ch2 = 0x03,
        #[doc = "GTIM1_CH1 | GTIM2_CH1"]
        Gtim1Ch1OrGtim2Ch1 = 0x04,
        #[doc = "GTIM1_CH1 | GTIM2_CH2"]
        Gtim1Ch1OrGtim2Ch2 = 0x05,
        #[doc = "GTIM1_CH2 | GTIM2_CH1"]
        Gtim1Ch2OrGtim2Ch1 = 0x06,
        #[doc = "GTIM1_CH2 | GTIM2_CH2"]
        Gtim1Ch2OrGtim2Ch2 = 0x07,
        #[doc = "UART1_TXD & GTIM1_CH1"]
        Uart1TxdAndGtim1Ch1 = 0x08,
        #[doc = "UART1_TXD | GTIM1_CH1"]
        Uart1TxdOrGtim1Ch1 = 0x09,
        #[doc = "UART1_TXD & GTIM1_CH2"]
        Uart1TxdAndGtim1Ch2 = 0x0a,
        #[doc = "UART1_TXD | GTIM1_CH2"]
        Uart1TxdOrGtim1Ch2 = 0x0b,
        #[doc = "UART2_TXD & GTIM2_CH1"]
        Uart2TxdAndGtim2Ch1 = 0x0c,
        #[doc = "UART2_TXD | GTIM2_CH1"]
        Uart2TxdOrGtim2Ch1 = 0x0d,
        #[doc = "UART2_TXD & GTIM2_CH2"]
        Uart2TxdAndGtim2Ch2 = 0x0e,
        #[doc = "UART2_TXD | GTIM2_CH2"]
        Uart2TxdOrGtim2Ch2 = 0x0f,
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
    #[doc = "Own CW32L083 CN UM Rev2.0 PDF81 / printed80; docs/lse-active-l083.json. One analog bank only."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LseAmplitude {
        Least = 0x0,
        Small = 0x01,
        Normal = 0x02,
        Large = 0x03,
    }
    impl LseAmplitude {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LseAmplitude {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LseAmplitude {
        #[inline(always)]
        fn from(val: u8) -> LseAmplitude {
            LseAmplitude::from_bits(val)
        }
    }
    impl From<LseAmplitude> for u8 {
        #[inline(always)]
        fn from(val: LseAmplitude) -> u8 {
            LseAmplitude::to_bits(val)
        }
    }
    #[doc = "Own CW32L083 CN UM Rev2.0 PDF81 / printed80; docs/lse-active-l083.json. One analog bank only."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LseDrive {
        Least = 0x0,
        Small = 0x01,
        Normal = 0x02,
        Strong = 0x03,
    }
    impl LseDrive {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LseDrive {
            unsafe { core::mem::transmute(val & 0x03) }
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
    #[doc = "Own CW32L083 CN UM Rev2.0 PDF81 / printed80; docs/lse-active-l083.json. One analog bank only."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LseWait {
        Cycles256 = 0x0,
        Cycles1024 = 0x01,
        Cycles4096 = 0x02,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllDebug {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "Documented default; do not alter reserved debug controls."]
        Default = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl PllDebug {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllDebug {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllDebug {
        #[inline(always)]
        fn from(val: u8) -> PllDebug {
            PllDebug::from_bits(val)
        }
    }
    impl From<PllDebug> for u8 {
        #[inline(always)]
        fn from(val: PllDebug) -> u8 {
            PllDebug::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllInputRange {
        #[doc = "Input analog bin 4 to 6 MHz."]
        Mhz4to6 = 0x0,
        #[doc = "Input analog bin 6 to 12 MHz."]
        Mhz6to12 = 0x01,
        #[doc = "Input analog bin 12 to 20 MHz."]
        Mhz12to20 = 0x02,
        #[doc = "Input analog bin 20 to 24 MHz."]
        Mhz20to24 = 0x03,
    }
    impl PllInputRange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllInputRange {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllInputRange {
        #[inline(always)]
        fn from(val: u8) -> PllInputRange {
            PllInputRange::from_bits(val)
        }
    }
    impl From<PllInputRange> for u8 {
        #[inline(always)]
        fn from(val: PllInputRange) -> u8 {
            PllInputRange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllMul {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Multiply PLL input by 2."]
        Mul2 = 0x02,
        #[doc = "Multiply PLL input by 3."]
        Mul3 = 0x03,
        #[doc = "Multiply PLL input by 4."]
        Mul4 = 0x04,
        #[doc = "Multiply PLL input by 5."]
        Mul5 = 0x05,
        #[doc = "Multiply PLL input by 6."]
        Mul6 = 0x06,
        #[doc = "Multiply PLL input by 7."]
        Mul7 = 0x07,
        #[doc = "Multiply PLL input by 8."]
        Mul8 = 0x08,
        #[doc = "Multiply PLL input by 9."]
        Mul9 = 0x09,
        #[doc = "Multiply PLL input by 10."]
        Mul10 = 0x0a,
        #[doc = "Multiply PLL input by 11."]
        Mul11 = 0x0b,
        #[doc = "Multiply PLL input by 12."]
        Mul12 = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl PllMul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllMul {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllMul {
        #[inline(always)]
        fn from(val: u8) -> PllMul {
            PllMul::from_bits(val)
        }
    }
    impl From<PllMul> for u8 {
        #[inline(always)]
        fn from(val: PllMul) -> u8 {
            PllMul::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllOutputRange {
        #[doc = "Output analog bin 12 to 18 MHz; independent datasheet cap still applies."]
        Mhz12to18 = 0x0,
        #[doc = "Output analog bin 18 to 24 MHz; independent datasheet cap still applies."]
        Mhz18to24 = 0x01,
        #[doc = "Output analog bin 24 to 36 MHz; independent datasheet cap still applies."]
        Mhz24to36 = 0x02,
        #[doc = "Output analog bin 36 to 48 MHz; independent datasheet cap still applies."]
        Mhz36to48 = 0x03,
        #[doc = "Output analog bin 48 to 72 MHz; independent datasheet cap still applies."]
        Mhz48to72 = 0x04,
        #[doc = "Documented 1xx alias for the 48 to 72 MHz analog bin; HAL uses canonical code4."]
        Mhz48to72alt5 = 0x05,
        #[doc = "Documented 1xx alias for the 48 to 72 MHz analog bin; HAL uses canonical code4."]
        Mhz48to72alt6 = 0x06,
        #[doc = "Documented 1xx alias for the 48 to 72 MHz analog bin; HAL uses canonical code4."]
        Mhz48to72alt7 = 0x07,
    }
    impl PllOutputRange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllOutputRange {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllOutputRange {
        #[inline(always)]
        fn from(val: u8) -> PllOutputRange {
            PllOutputRange::from_bits(val)
        }
    }
    impl From<PllOutputRange> for u8 {
        #[inline(always)]
        fn from(val: PllOutputRange) -> u8 {
            PllOutputRange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllSource {
        #[doc = "HSE oscillator clock."]
        HseCrystal = 0x0,
        #[doc = "External HSE pin clock input."]
        HseBypass = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "HSI after its existing divider."]
        Hsi = 0x03,
    }
    impl PllSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllSource {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllSource {
        #[inline(always)]
        fn from(val: u8) -> PllSource {
            PllSource::from_bits(val)
        }
    }
    impl From<PllSource> for u8 {
        #[inline(always)]
        fn from(val: PllSource) -> u8 {
            PllSource::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PllWait {
        #[doc = "Wait 128 PLL cycles."]
        Cycles128 = 0x0,
        #[doc = "Wait 256 PLL cycles."]
        Cycles256 = 0x01,
        #[doc = "Wait 512 PLL cycles."]
        Cycles512 = 0x02,
        #[doc = "Wait 1024 PLL cycles."]
        Cycles1024 = 0x03,
        #[doc = "Wait 2048 PLL cycles."]
        Cycles2048 = 0x04,
        #[doc = "Wait 4096 PLL cycles."]
        Cycles4096 = 0x05,
        #[doc = "Wait 8192 PLL cycles."]
        Cycles8192 = 0x06,
        #[doc = "Wait 16384 PLL cycles."]
        Cycles16384 = 0x07,
    }
    impl PllWait {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PllWait {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PllWait {
        #[inline(always)]
        fn from(val: u8) -> PllWait {
            PllWait::from_bits(val)
        }
    }
    impl From<PllWait> for u8 {
        #[inline(always)]
        fn from(val: PllWait) -> u8 {
            PllWait::to_bits(val)
        }
    }
    #[doc = "Own L083 SYSCLK selector; PLL is 2 and values 5 through 7 are reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sysclk {
        #[doc = "HSI system clock."]
        Hsi = 0x0,
        #[doc = "HSE system clock."]
        Hse = 0x01,
        #[doc = "PLL system clock; encoding only, not a HAL PLL configuration API."]
        Pll = 0x02,
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
