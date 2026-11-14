#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ATIM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atim {
    ptr: *mut u8,
}
unsafe impl Send for Atim {}
unsafe impl Sync for Atim {}
impl Atim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
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
    #[doc = "desc MSCR."]
    #[inline(always)]
    pub const fn mscr(self) -> crate::common::Reg<regs::Mscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc FLTR."]
    #[inline(always)]
    pub const fn fltr(self) -> crate::common::Reg<regs::Fltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc TRIG."]
    #[inline(always)]
    pub const fn trig(self) -> crate::common::Reg<regs::Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc CH1CR."]
    #[inline(always)]
    pub const fn ch1cr(self) -> crate::common::Reg<regs::Ch1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc CH2CR."]
    #[inline(always)]
    pub const fn ch2cr(self) -> crate::common::Reg<regs::Ch2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc CH3CR."]
    #[inline(always)]
    pub const fn ch3cr(self) -> crate::common::Reg<regs::Ch3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc DTR."]
    #[inline(always)]
    pub const fn dtr(self) -> crate::common::Reg<regs::Dtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc RCR."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CH1CCRA."]
    #[inline(always)]
    pub const fn ch1ccra(self) -> crate::common::Reg<regs::Ch1ccra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc CH1CCRB."]
    #[inline(always)]
    pub const fn ch1ccrb(self) -> crate::common::Reg<regs::Ch1ccrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc CH2CCRA."]
    #[inline(always)]
    pub const fn ch2ccra(self) -> crate::common::Reg<regs::Ch2ccra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc CH2CCRB."]
    #[inline(always)]
    pub const fn ch2ccrb(self) -> crate::common::Reg<regs::Ch2ccrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc CH3CCRA."]
    #[inline(always)]
    pub const fn ch3ccra(self) -> crate::common::Reg<regs::Ch3ccra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc CH3CCRB."]
    #[inline(always)]
    pub const fn ch3ccrb(self) -> crate::common::Reg<regs::Ch3ccrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc CH4CCR."]
    #[inline(always)]
    pub const fn ch4ccr(self) -> crate::common::Reg<regs::Ch4ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc CH4CR."]
    #[inline(always)]
    pub const fn ch4cr(self) -> crate::common::Reg<regs::Ch4cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "desc ARR."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ARR."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Arr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Arr {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "desc CH1CCRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1ccra(pub u32);
    impl Ch1ccra {
        #[doc = "desc CCR1A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr1a(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR1A."]
        #[inline(always)]
        pub const fn set_ccr1a(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch1ccra {
        #[inline(always)]
        fn default() -> Ch1ccra {
            Ch1ccra(0)
        }
    }
    impl core::fmt::Debug for Ch1ccra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1ccra")
                .field("ccr1a", &self.ccr1a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1ccra {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1ccra {{ ccr1a: {=u16:?} }}", self.ccr1a())
        }
    }
    #[doc = "desc CH1CCRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1ccrb(pub u32);
    impl Ch1ccrb {
        #[doc = "desc CCR1B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr1b(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR1B."]
        #[inline(always)]
        pub const fn set_ccr1b(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch1ccrb {
        #[inline(always)]
        fn default() -> Ch1ccrb {
            Ch1ccrb(0)
        }
    }
    impl core::fmt::Debug for Ch1ccrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1ccrb")
                .field("ccr1b", &self.ccr1b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1ccrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1ccrb {{ ccr1b: {=u16:?} }}", self.ccr1b())
        }
    }
    #[doc = "desc CH1CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1cr(pub u32);
    impl Ch1cr {
        #[doc = "desc BKSA."]
        #[must_use]
        #[inline(always)]
        pub const fn bksa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSA."]
        #[inline(always)]
        pub const fn set_bksa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc BKSB."]
        #[must_use]
        #[inline(always)]
        pub const fn bksb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSB."]
        #[inline(always)]
        pub const fn set_bksb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CSA."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSA."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CSB."]
        #[must_use]
        #[inline(always)]
        pub const fn csb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSB."]
        #[inline(always)]
        pub const fn set_csb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BUFEA."]
        #[must_use]
        #[inline(always)]
        pub const fn bufea(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEA."]
        #[inline(always)]
        pub const fn set_bufea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BUFEB."]
        #[must_use]
        #[inline(always)]
        pub const fn bufeb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEB."]
        #[inline(always)]
        pub const fn set_bufeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CIEA."]
        #[must_use]
        #[inline(always)]
        pub const fn ciea(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEA."]
        #[inline(always)]
        pub const fn set_ciea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CIEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cieb(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEB."]
        #[inline(always)]
        pub const fn set_cieb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CDEA."]
        #[must_use]
        #[inline(always)]
        pub const fn cdea(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEA."]
        #[inline(always)]
        pub const fn set_cdea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CDEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeb(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEB."]
        #[inline(always)]
        pub const fn set_cdeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CISB."]
        #[must_use]
        #[inline(always)]
        pub const fn cisb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CISB."]
        #[inline(always)]
        pub const fn set_cisb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CCGA."]
        #[must_use]
        #[inline(always)]
        pub const fn ccga(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGA."]
        #[inline(always)]
        pub const fn set_ccga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CCGB."]
        #[must_use]
        #[inline(always)]
        pub const fn ccgb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGB."]
        #[inline(always)]
        pub const fn set_ccgb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ch1cr {
        #[inline(always)]
        fn default() -> Ch1cr {
            Ch1cr(0)
        }
    }
    impl core::fmt::Debug for Ch1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1cr")
                .field("bksa", &self.bksa())
                .field("bksb", &self.bksb())
                .field("csa", &self.csa())
                .field("csb", &self.csb())
                .field("bufea", &self.bufea())
                .field("bufeb", &self.bufeb())
                .field("ciea", &self.ciea())
                .field("cieb", &self.cieb())
                .field("cdea", &self.cdea())
                .field("cdeb", &self.cdeb())
                .field("cisb", &self.cisb())
                .field("ccga", &self.ccga())
                .field("ccgb", &self.ccgb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1cr {{ bksa: {=u8:?}, bksb: {=u8:?}, csa: {=bool:?}, csb: {=bool:?}, bufea: {=bool:?}, bufeb: {=bool:?}, ciea: {=bool:?}, cieb: {=bool:?}, cdea: {=bool:?}, cdeb: {=bool:?}, cisb: {=u8:?}, ccga: {=bool:?}, ccgb: {=bool:?} }}",
                self.bksa(),
                self.bksb(),
                self.csa(),
                self.csb(),
                self.bufea(),
                self.bufeb(),
                self.ciea(),
                self.cieb(),
                self.cdea(),
                self.cdeb(),
                self.cisb(),
                self.ccga(),
                self.ccgb()
            )
        }
    }
    #[doc = "desc CH2CCRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2ccra(pub u32);
    impl Ch2ccra {
        #[doc = "desc CCR2A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr2a(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR2A."]
        #[inline(always)]
        pub const fn set_ccr2a(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch2ccra {
        #[inline(always)]
        fn default() -> Ch2ccra {
            Ch2ccra(0)
        }
    }
    impl core::fmt::Debug for Ch2ccra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2ccra")
                .field("ccr2a", &self.ccr2a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2ccra {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2ccra {{ ccr2a: {=u16:?} }}", self.ccr2a())
        }
    }
    #[doc = "desc CH2CCRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2ccrb(pub u32);
    impl Ch2ccrb {
        #[doc = "desc CCR2B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr2b(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR2B."]
        #[inline(always)]
        pub const fn set_ccr2b(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch2ccrb {
        #[inline(always)]
        fn default() -> Ch2ccrb {
            Ch2ccrb(0)
        }
    }
    impl core::fmt::Debug for Ch2ccrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2ccrb")
                .field("ccr2b", &self.ccr2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2ccrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2ccrb {{ ccr2b: {=u16:?} }}", self.ccr2b())
        }
    }
    #[doc = "desc CH2CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2cr(pub u32);
    impl Ch2cr {
        #[doc = "desc BKSA."]
        #[must_use]
        #[inline(always)]
        pub const fn bksa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSA."]
        #[inline(always)]
        pub const fn set_bksa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc BKSB."]
        #[must_use]
        #[inline(always)]
        pub const fn bksb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSB."]
        #[inline(always)]
        pub const fn set_bksb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CSA."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSA."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CSB."]
        #[must_use]
        #[inline(always)]
        pub const fn csb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSB."]
        #[inline(always)]
        pub const fn set_csb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BUFEA."]
        #[must_use]
        #[inline(always)]
        pub const fn bufea(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEA."]
        #[inline(always)]
        pub const fn set_bufea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BUFEB."]
        #[must_use]
        #[inline(always)]
        pub const fn bufeb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEB."]
        #[inline(always)]
        pub const fn set_bufeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CIEA."]
        #[must_use]
        #[inline(always)]
        pub const fn ciea(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEA."]
        #[inline(always)]
        pub const fn set_ciea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CIEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cieb(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEB."]
        #[inline(always)]
        pub const fn set_cieb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CDEA."]
        #[must_use]
        #[inline(always)]
        pub const fn cdea(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEA."]
        #[inline(always)]
        pub const fn set_cdea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CDEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeb(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEB."]
        #[inline(always)]
        pub const fn set_cdeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CISB."]
        #[must_use]
        #[inline(always)]
        pub const fn cisb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CISB."]
        #[inline(always)]
        pub const fn set_cisb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CCGA."]
        #[must_use]
        #[inline(always)]
        pub const fn ccga(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGA."]
        #[inline(always)]
        pub const fn set_ccga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CCGB."]
        #[must_use]
        #[inline(always)]
        pub const fn ccgb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGB."]
        #[inline(always)]
        pub const fn set_ccgb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ch2cr {
        #[inline(always)]
        fn default() -> Ch2cr {
            Ch2cr(0)
        }
    }
    impl core::fmt::Debug for Ch2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2cr")
                .field("bksa", &self.bksa())
                .field("bksb", &self.bksb())
                .field("csa", &self.csa())
                .field("csb", &self.csb())
                .field("bufea", &self.bufea())
                .field("bufeb", &self.bufeb())
                .field("ciea", &self.ciea())
                .field("cieb", &self.cieb())
                .field("cdea", &self.cdea())
                .field("cdeb", &self.cdeb())
                .field("cisb", &self.cisb())
                .field("ccga", &self.ccga())
                .field("ccgb", &self.ccgb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2cr {{ bksa: {=u8:?}, bksb: {=u8:?}, csa: {=bool:?}, csb: {=bool:?}, bufea: {=bool:?}, bufeb: {=bool:?}, ciea: {=bool:?}, cieb: {=bool:?}, cdea: {=bool:?}, cdeb: {=bool:?}, cisb: {=u8:?}, ccga: {=bool:?}, ccgb: {=bool:?} }}",
                self.bksa(),
                self.bksb(),
                self.csa(),
                self.csb(),
                self.bufea(),
                self.bufeb(),
                self.ciea(),
                self.cieb(),
                self.cdea(),
                self.cdeb(),
                self.cisb(),
                self.ccga(),
                self.ccgb()
            )
        }
    }
    #[doc = "desc CH3CCRA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3ccra(pub u32);
    impl Ch3ccra {
        #[doc = "desc CCR3A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr3a(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR3A."]
        #[inline(always)]
        pub const fn set_ccr3a(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch3ccra {
        #[inline(always)]
        fn default() -> Ch3ccra {
            Ch3ccra(0)
        }
    }
    impl core::fmt::Debug for Ch3ccra {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3ccra")
                .field("ccr3a", &self.ccr3a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3ccra {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3ccra {{ ccr3a: {=u16:?} }}", self.ccr3a())
        }
    }
    #[doc = "desc CH3CCRB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3ccrb(pub u32);
    impl Ch3ccrb {
        #[doc = "desc CCR3B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr3b(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR3B."]
        #[inline(always)]
        pub const fn set_ccr3b(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch3ccrb {
        #[inline(always)]
        fn default() -> Ch3ccrb {
            Ch3ccrb(0)
        }
    }
    impl core::fmt::Debug for Ch3ccrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3ccrb")
                .field("ccr3b", &self.ccr3b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3ccrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3ccrb {{ ccr3b: {=u16:?} }}", self.ccr3b())
        }
    }
    #[doc = "desc CH3CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3cr(pub u32);
    impl Ch3cr {
        #[doc = "desc BKSA."]
        #[must_use]
        #[inline(always)]
        pub const fn bksa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSA."]
        #[inline(always)]
        pub const fn set_bksa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc BKSB."]
        #[must_use]
        #[inline(always)]
        pub const fn bksb(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc BKSB."]
        #[inline(always)]
        pub const fn set_bksb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CSA."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSA."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CSB."]
        #[must_use]
        #[inline(always)]
        pub const fn csb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSB."]
        #[inline(always)]
        pub const fn set_csb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BUFEA."]
        #[must_use]
        #[inline(always)]
        pub const fn bufea(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEA."]
        #[inline(always)]
        pub const fn set_bufea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BUFEB."]
        #[must_use]
        #[inline(always)]
        pub const fn bufeb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFEB."]
        #[inline(always)]
        pub const fn set_bufeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CIEA."]
        #[must_use]
        #[inline(always)]
        pub const fn ciea(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEA."]
        #[inline(always)]
        pub const fn set_ciea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CIEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cieb(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEB."]
        #[inline(always)]
        pub const fn set_cieb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CDEA."]
        #[must_use]
        #[inline(always)]
        pub const fn cdea(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEA."]
        #[inline(always)]
        pub const fn set_cdea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CDEB."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeb(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDEB."]
        #[inline(always)]
        pub const fn set_cdeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CISB."]
        #[must_use]
        #[inline(always)]
        pub const fn cisb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc CISB."]
        #[inline(always)]
        pub const fn set_cisb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc CCGA."]
        #[must_use]
        #[inline(always)]
        pub const fn ccga(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGA."]
        #[inline(always)]
        pub const fn set_ccga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CCGB."]
        #[must_use]
        #[inline(always)]
        pub const fn ccgb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCGB."]
        #[inline(always)]
        pub const fn set_ccgb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ch3cr {
        #[inline(always)]
        fn default() -> Ch3cr {
            Ch3cr(0)
        }
    }
    impl core::fmt::Debug for Ch3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3cr")
                .field("bksa", &self.bksa())
                .field("bksb", &self.bksb())
                .field("csa", &self.csa())
                .field("csb", &self.csb())
                .field("bufea", &self.bufea())
                .field("bufeb", &self.bufeb())
                .field("ciea", &self.ciea())
                .field("cieb", &self.cieb())
                .field("cdea", &self.cdea())
                .field("cdeb", &self.cdeb())
                .field("cisb", &self.cisb())
                .field("ccga", &self.ccga())
                .field("ccgb", &self.ccgb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3cr {{ bksa: {=u8:?}, bksb: {=u8:?}, csa: {=bool:?}, csb: {=bool:?}, bufea: {=bool:?}, bufeb: {=bool:?}, ciea: {=bool:?}, cieb: {=bool:?}, cdea: {=bool:?}, cdeb: {=bool:?}, cisb: {=u8:?}, ccga: {=bool:?}, ccgb: {=bool:?} }}",
                self.bksa(),
                self.bksb(),
                self.csa(),
                self.csb(),
                self.bufea(),
                self.bufeb(),
                self.ciea(),
                self.cieb(),
                self.cdea(),
                self.cdeb(),
                self.cisb(),
                self.ccga(),
                self.ccgb()
            )
        }
    }
    #[doc = "desc CH4CCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4ccr(pub u32);
    impl Ch4ccr {
        #[doc = "desc CCR4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR4."]
        #[inline(always)]
        pub const fn set_ccr4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch4ccr {
        #[inline(always)]
        fn default() -> Ch4ccr {
            Ch4ccr(0)
        }
    }
    impl core::fmt::Debug for Ch4ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4ccr")
                .field("ccr4", &self.ccr4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4ccr {{ ccr4: {=u16:?} }}", self.ccr4())
        }
    }
    #[doc = "desc CH4CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4cr(pub u32);
    impl Ch4cr {
        #[doc = "desc BUFE."]
        #[must_use]
        #[inline(always)]
        pub const fn bufe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFE."]
        #[inline(always)]
        pub const fn set_bufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CIE."]
        #[must_use]
        #[inline(always)]
        pub const fn cie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIE."]
        #[inline(always)]
        pub const fn set_cie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CDE."]
        #[must_use]
        #[inline(always)]
        pub const fn cde(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CDE."]
        #[inline(always)]
        pub const fn set_cde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CIS."]
        #[must_use]
        #[inline(always)]
        pub const fn cis(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "desc CIS."]
        #[inline(always)]
        pub const fn set_cis(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "desc C4EN."]
        #[must_use]
        #[inline(always)]
        pub const fn c4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc C4EN."]
        #[inline(always)]
        pub const fn set_c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ch4cr {
        #[inline(always)]
        fn default() -> Ch4cr {
            Ch4cr(0)
        }
    }
    impl core::fmt::Debug for Ch4cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4cr")
                .field("bufe", &self.bufe())
                .field("cie", &self.cie())
                .field("cde", &self.cde())
                .field("cis", &self.cis())
                .field("c4en", &self.c4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4cr {{ bufe: {=bool:?}, cie: {=bool:?}, cde: {=bool:?}, cis: {=u8:?}, c4en: {=bool:?} }}",
                self.bufe(),
                self.cie(),
                self.cde(),
                self.cis(),
                self.c4en()
            )
        }
    }
    #[doc = "desc CNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMP."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CT."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CT."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PWM2S."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2s(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM2S."]
        #[inline(always)]
        pub const fn set_pwm2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PRS."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc BUFPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn bufpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUFPEN."]
        #[inline(always)]
        pub const fn set_bufpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UIE."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIE."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc ONESHOT."]
        #[must_use]
        #[inline(always)]
        pub const fn oneshot(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ONESHOT."]
        #[inline(always)]
        pub const fn set_oneshot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc OCCS."]
        #[must_use]
        #[inline(always)]
        pub const fn occs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OCCS."]
        #[inline(always)]
        pub const fn set_occs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc URS."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc URS."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIE."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BIE."]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIE."]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc CISA."]
        #[must_use]
        #[inline(always)]
        pub const fn cisa(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "desc CISA."]
        #[inline(always)]
        pub const fn set_cisa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "desc OCCE."]
        #[must_use]
        #[inline(always)]
        pub const fn occe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc OCCE."]
        #[inline(always)]
        pub const fn set_occe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc TG."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc TG."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc UG."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc UG."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc BG."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc BG."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIR."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc OVE."]
        #[must_use]
        #[inline(always)]
        pub const fn ove(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVE."]
        #[inline(always)]
        pub const fn set_ove(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc UNDE."]
        #[must_use]
        #[inline(always)]
        pub const fn unde(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNDE."]
        #[inline(always)]
        pub const fn set_unde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("en", &self.en())
                .field("comp", &self.comp())
                .field("ct", &self.ct())
                .field("pwm2s", &self.pwm2s())
                .field("prs", &self.prs())
                .field("bufpen", &self.bufpen())
                .field("uie", &self.uie())
                .field("mode", &self.mode())
                .field("oneshot", &self.oneshot())
                .field("occs", &self.occs())
                .field("urs", &self.urs())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("cisa", &self.cisa())
                .field("occe", &self.occe())
                .field("tg", &self.tg())
                .field("ug", &self.ug())
                .field("bg", &self.bg())
                .field("dir", &self.dir())
                .field("ove", &self.ove())
                .field("unde", &self.unde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, comp: {=bool:?}, ct: {=bool:?}, pwm2s: {=bool:?}, prs: {=u8:?}, bufpen: {=bool:?}, uie: {=bool:?}, mode: {=u8:?}, oneshot: {=bool:?}, occs: {=bool:?}, urs: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, cisa: {=u8:?}, occe: {=bool:?}, tg: {=bool:?}, ug: {=bool:?}, bg: {=bool:?}, dir: {=bool:?}, ove: {=bool:?}, unde: {=bool:?} }}",
                self.en(),
                self.comp(),
                self.ct(),
                self.pwm2s(),
                self.prs(),
                self.bufpen(),
                self.uie(),
                self.mode(),
                self.oneshot(),
                self.occs(),
                self.urs(),
                self.tie(),
                self.bie(),
                self.cisa(),
                self.occe(),
                self.tg(),
                self.ug(),
                self.bg(),
                self.dir(),
                self.ove(),
                self.unde()
            )
        }
    }
    #[doc = "desc DTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr(pub u32);
    impl Dtr {
        #[doc = "desc DTR."]
        #[must_use]
        #[inline(always)]
        pub const fn dtr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DTR."]
        #[inline(always)]
        pub const fn set_dtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc DTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTEN."]
        #[inline(always)]
        pub const fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BKE."]
        #[must_use]
        #[inline(always)]
        pub const fn bke(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKE."]
        #[inline(always)]
        pub const fn set_bke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc AOE."]
        #[must_use]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc AOE."]
        #[inline(always)]
        pub const fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MOE."]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MOE."]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SAFEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn safeen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAFEEN."]
        #[inline(always)]
        pub const fn set_safeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VCE."]
        #[must_use]
        #[inline(always)]
        pub const fn vce(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VCE."]
        #[inline(always)]
        pub const fn set_vce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Dtr {
        #[inline(always)]
        fn default() -> Dtr {
            Dtr(0)
        }
    }
    impl core::fmt::Debug for Dtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr")
                .field("dtr", &self.dtr())
                .field("dten", &self.dten())
                .field("bke", &self.bke())
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .field("safeen", &self.safeen())
                .field("vce", &self.vce())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dtr {{ dtr: {=u8:?}, dten: {=bool:?}, bke: {=bool:?}, aoe: {=bool:?}, moe: {=bool:?}, safeen: {=bool:?}, vce: {=bool:?} }}",
                self.dtr(),
                self.dten(),
                self.bke(),
                self.aoe(),
                self.moe(),
                self.safeen(),
                self.vce()
            )
        }
    }
    #[doc = "desc FLTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltr(pub u32);
    impl Fltr {
        #[doc = "desc OCM1AFLT1A."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm1aflt1a(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM1AFLT1A."]
        #[inline(always)]
        pub const fn set_ocm1aflt1a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CCP1A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp1a(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP1A."]
        #[inline(always)]
        pub const fn set_ccp1a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OCM1BFLT1B."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm1bflt1b(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM1BFLT1B."]
        #[inline(always)]
        pub const fn set_ocm1bflt1b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc CCP1B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp1b(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP1B."]
        #[inline(always)]
        pub const fn set_ccp1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc OCM2AFLT2A."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm2aflt2a(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM2AFLT2A."]
        #[inline(always)]
        pub const fn set_ocm2aflt2a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc CCP2A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp2a(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP2A."]
        #[inline(always)]
        pub const fn set_ccp2a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OCM2BFLT2B."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm2bflt2b(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM2BFLT2B."]
        #[inline(always)]
        pub const fn set_ocm2bflt2b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc CCP2B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp2b(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP2B."]
        #[inline(always)]
        pub const fn set_ccp2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OCM3AFLT3A."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm3aflt3a(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM3AFLT3A."]
        #[inline(always)]
        pub const fn set_ocm3aflt3a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc CCP3A."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp3a(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP3A."]
        #[inline(always)]
        pub const fn set_ccp3a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc OCM3BFLT3B."]
        #[must_use]
        #[inline(always)]
        pub const fn ocm3bflt3b(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCM3BFLT3B."]
        #[inline(always)]
        pub const fn set_ocm3bflt3b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc CCP3B."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp3b(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCP3B."]
        #[inline(always)]
        pub const fn set_ccp3b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FLTBK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltbk(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTBK."]
        #[inline(always)]
        pub const fn set_fltbk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc BKP."]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKP."]
        #[inline(always)]
        pub const fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FLTET."]
        #[must_use]
        #[inline(always)]
        pub const fn fltet(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTET."]
        #[inline(always)]
        pub const fn set_fltet(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "desc ETP."]
        #[must_use]
        #[inline(always)]
        pub const fn etp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETP."]
        #[inline(always)]
        pub const fn set_etp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Fltr {
        #[inline(always)]
        fn default() -> Fltr {
            Fltr(0)
        }
    }
    impl core::fmt::Debug for Fltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltr")
                .field("ocm1aflt1a", &self.ocm1aflt1a())
                .field("ccp1a", &self.ccp1a())
                .field("ocm1bflt1b", &self.ocm1bflt1b())
                .field("ccp1b", &self.ccp1b())
                .field("ocm2aflt2a", &self.ocm2aflt2a())
                .field("ccp2a", &self.ccp2a())
                .field("ocm2bflt2b", &self.ocm2bflt2b())
                .field("ccp2b", &self.ccp2b())
                .field("ocm3aflt3a", &self.ocm3aflt3a())
                .field("ccp3a", &self.ccp3a())
                .field("ocm3bflt3b", &self.ocm3bflt3b())
                .field("ccp3b", &self.ccp3b())
                .field("fltbk", &self.fltbk())
                .field("bkp", &self.bkp())
                .field("fltet", &self.fltet())
                .field("etp", &self.etp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fltr {{ ocm1aflt1a: {=u8:?}, ccp1a: {=bool:?}, ocm1bflt1b: {=u8:?}, ccp1b: {=bool:?}, ocm2aflt2a: {=u8:?}, ccp2a: {=bool:?}, ocm2bflt2b: {=u8:?}, ccp2b: {=bool:?}, ocm3aflt3a: {=u8:?}, ccp3a: {=bool:?}, ocm3bflt3b: {=u8:?}, ccp3b: {=bool:?}, fltbk: {=u8:?}, bkp: {=bool:?}, fltet: {=u8:?}, etp: {=bool:?} }}",
                self.ocm1aflt1a(),
                self.ccp1a(),
                self.ocm1bflt1b(),
                self.ccp1b(),
                self.ocm2aflt2a(),
                self.ccp2a(),
                self.ocm2bflt2b(),
                self.ccp2b(),
                self.ocm3aflt3a(),
                self.ccp3a(),
                self.ocm3bflt3b(),
                self.ccp3b(),
                self.fltbk(),
                self.bkp(),
                self.fltet(),
                self.etp()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc C1AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c1af(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1AF."]
        #[inline(always)]
        pub const fn set_c1af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc C2AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c2af(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2AF."]
        #[inline(always)]
        pub const fn set_c2af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc C3AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c3af(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3AF."]
        #[inline(always)]
        pub const fn set_c3af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc C1BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c1bf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1BF."]
        #[inline(always)]
        pub const fn set_c1bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc C2BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c2bf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2BF."]
        #[inline(always)]
        pub const fn set_c2bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc C3BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c3bf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3BF."]
        #[inline(always)]
        pub const fn set_c3bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc C1AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c1ae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1AE."]
        #[inline(always)]
        pub const fn set_c1ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc C2AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c2ae(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2AE."]
        #[inline(always)]
        pub const fn set_c2ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc C3AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c3ae(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3AE."]
        #[inline(always)]
        pub const fn set_c3ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc C1BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c1be(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1BE."]
        #[inline(always)]
        pub const fn set_c1be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc C2BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c2be(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2BE."]
        #[inline(always)]
        pub const fn set_c2be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc C3BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c3be(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3BE."]
        #[inline(always)]
        pub const fn set_c3be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BIF."]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIF."]
        #[inline(always)]
        pub const fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIF."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc UNDF."]
        #[must_use]
        #[inline(always)]
        pub const fn undf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNDF."]
        #[inline(always)]
        pub const fn set_undf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc C4AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c4af(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc C4AF."]
        #[inline(always)]
        pub const fn set_c4af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("uif", &self.uif())
                .field("c1af", &self.c1af())
                .field("c2af", &self.c2af())
                .field("c3af", &self.c3af())
                .field("c1bf", &self.c1bf())
                .field("c2bf", &self.c2bf())
                .field("c3bf", &self.c3bf())
                .field("c1ae", &self.c1ae())
                .field("c2ae", &self.c2ae())
                .field("c3ae", &self.c3ae())
                .field("c1be", &self.c1be())
                .field("c2be", &self.c2be())
                .field("c3be", &self.c3be())
                .field("bif", &self.bif())
                .field("tif", &self.tif())
                .field("ovf", &self.ovf())
                .field("undf", &self.undf())
                .field("c4af", &self.c4af())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ uif: {=bool:?}, c1af: {=bool:?}, c2af: {=bool:?}, c3af: {=bool:?}, c1bf: {=bool:?}, c2bf: {=bool:?}, c3bf: {=bool:?}, c1ae: {=bool:?}, c2ae: {=bool:?}, c3ae: {=bool:?}, c1be: {=bool:?}, c2be: {=bool:?}, c3be: {=bool:?}, bif: {=bool:?}, tif: {=bool:?}, ovf: {=bool:?}, undf: {=bool:?}, c4af: {=bool:?} }}",
                self.uif(),
                self.c1af(),
                self.c2af(),
                self.c3af(),
                self.c1bf(),
                self.c2bf(),
                self.c3bf(),
                self.c1ae(),
                self.c2ae(),
                self.c3ae(),
                self.c1be(),
                self.c2be(),
                self.c3be(),
                self.bif(),
                self.tif(),
                self.ovf(),
                self.undf(),
                self.c4af()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc C1AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c1af(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1AF."]
        #[inline(always)]
        pub const fn set_c1af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc C2AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c2af(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2AF."]
        #[inline(always)]
        pub const fn set_c2af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc C3AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c3af(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3AF."]
        #[inline(always)]
        pub const fn set_c3af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc C1BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c1bf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1BF."]
        #[inline(always)]
        pub const fn set_c1bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc C2BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c2bf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2BF."]
        #[inline(always)]
        pub const fn set_c2bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc C3BF."]
        #[must_use]
        #[inline(always)]
        pub const fn c3bf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3BF."]
        #[inline(always)]
        pub const fn set_c3bf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc C1AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c1ae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1AE."]
        #[inline(always)]
        pub const fn set_c1ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc C2AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c2ae(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2AE."]
        #[inline(always)]
        pub const fn set_c2ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc C3AE."]
        #[must_use]
        #[inline(always)]
        pub const fn c3ae(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3AE."]
        #[inline(always)]
        pub const fn set_c3ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc C1BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c1be(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1BE."]
        #[inline(always)]
        pub const fn set_c1be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc C2BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c2be(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2BE."]
        #[inline(always)]
        pub const fn set_c2be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc C3BE."]
        #[must_use]
        #[inline(always)]
        pub const fn c3be(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3BE."]
        #[inline(always)]
        pub const fn set_c3be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BIF."]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIF."]
        #[inline(always)]
        pub const fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIF."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc UNDF."]
        #[must_use]
        #[inline(always)]
        pub const fn undf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNDF."]
        #[inline(always)]
        pub const fn set_undf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc C4AF."]
        #[must_use]
        #[inline(always)]
        pub const fn c4af(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc C4AF."]
        #[inline(always)]
        pub const fn set_c4af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("uif", &self.uif())
                .field("c1af", &self.c1af())
                .field("c2af", &self.c2af())
                .field("c3af", &self.c3af())
                .field("c1bf", &self.c1bf())
                .field("c2bf", &self.c2bf())
                .field("c3bf", &self.c3bf())
                .field("c1ae", &self.c1ae())
                .field("c2ae", &self.c2ae())
                .field("c3ae", &self.c3ae())
                .field("c1be", &self.c1be())
                .field("c2be", &self.c2be())
                .field("c3be", &self.c3be())
                .field("bif", &self.bif())
                .field("tif", &self.tif())
                .field("ovf", &self.ovf())
                .field("undf", &self.undf())
                .field("c4af", &self.c4af())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ uif: {=bool:?}, c1af: {=bool:?}, c2af: {=bool:?}, c3af: {=bool:?}, c1bf: {=bool:?}, c2bf: {=bool:?}, c3bf: {=bool:?}, c1ae: {=bool:?}, c2ae: {=bool:?}, c3ae: {=bool:?}, c1be: {=bool:?}, c2be: {=bool:?}, c3be: {=bool:?}, bif: {=bool:?}, tif: {=bool:?}, ovf: {=bool:?}, undf: {=bool:?}, c4af: {=bool:?} }}",
                self.uif(),
                self.c1af(),
                self.c2af(),
                self.c3af(),
                self.c1bf(),
                self.c2bf(),
                self.c3bf(),
                self.c1ae(),
                self.c2ae(),
                self.c3ae(),
                self.c1be(),
                self.c2be(),
                self.c3be(),
                self.bif(),
                self.tif(),
                self.ovf(),
                self.undf(),
                self.c4af()
            )
        }
    }
    #[doc = "desc MSCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mscr(pub u32);
    impl Mscr {
        #[doc = "desc MMS."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc MMS."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CCDS."]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCDS."]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MSM."]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSM."]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "desc SMS."]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc SMS."]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc IA1S."]
        #[must_use]
        #[inline(always)]
        pub const fn ia1s(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IA1S."]
        #[inline(always)]
        pub const fn set_ia1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IB1S."]
        #[must_use]
        #[inline(always)]
        pub const fn ib1s(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IB1S."]
        #[inline(always)]
        pub const fn set_ib1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mscr {
        #[inline(always)]
        fn default() -> Mscr {
            Mscr(0)
        }
    }
    impl core::fmt::Debug for Mscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mscr")
                .field("mms", &self.mms())
                .field("ccds", &self.ccds())
                .field("msm", &self.msm())
                .field("ts", &self.ts())
                .field("sms", &self.sms())
                .field("ia1s", &self.ia1s())
                .field("ib1s", &self.ib1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mscr {{ mms: {=u8:?}, ccds: {=bool:?}, msm: {=bool:?}, ts: {=u8:?}, sms: {=u8:?}, ia1s: {=bool:?}, ib1s: {=bool:?} }}",
                self.mms(),
                self.ccds(),
                self.msm(),
                self.ts(),
                self.sms(),
                self.ia1s(),
                self.ib1s()
            )
        }
    }
    #[doc = "desc RCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "desc RCR."]
        #[must_use]
        #[inline(always)]
        pub const fn rcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc RCR."]
        #[inline(always)]
        pub const fn set_rcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    impl core::fmt::Debug for Rcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr")
                .field("rcr", &self.rcr())
                .field("ov", &self.ov())
                .field("ud", &self.ud())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rcr {{ rcr: {=u8:?}, ov: {=bool:?}, ud: {=bool:?} }}",
                self.rcr(),
                self.ov(),
                self.ud()
            )
        }
    }
    #[doc = "desc TRIG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig(pub u32);
    impl Trig {
        #[doc = "desc UEVE."]
        #[must_use]
        #[inline(always)]
        pub const fn ueve(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UEVE."]
        #[inline(always)]
        pub const fn set_ueve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CM1AE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm1ae(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM1AE."]
        #[inline(always)]
        pub const fn set_cm1ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CM2AE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm2ae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM2AE."]
        #[inline(always)]
        pub const fn set_cm2ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CM3AE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm3ae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM3AE."]
        #[inline(always)]
        pub const fn set_cm3ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CM1BE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm1be(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM1BE."]
        #[inline(always)]
        pub const fn set_cm1be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CM2BE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm2be(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM2BE."]
        #[inline(always)]
        pub const fn set_cm2be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CM3BE."]
        #[must_use]
        #[inline(always)]
        pub const fn cm3be(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CM3BE."]
        #[inline(always)]
        pub const fn set_cm3be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ADTE."]
        #[must_use]
        #[inline(always)]
        pub const fn adte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADTE."]
        #[inline(always)]
        pub const fn set_adte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Trig {
        #[inline(always)]
        fn default() -> Trig {
            Trig(0)
        }
    }
    impl core::fmt::Debug for Trig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig")
                .field("ueve", &self.ueve())
                .field("cm1ae", &self.cm1ae())
                .field("cm2ae", &self.cm2ae())
                .field("cm3ae", &self.cm3ae())
                .field("cm1be", &self.cm1be())
                .field("cm2be", &self.cm2be())
                .field("cm3be", &self.cm3be())
                .field("adte", &self.adte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig {{ ueve: {=bool:?}, cm1ae: {=bool:?}, cm2ae: {=bool:?}, cm3ae: {=bool:?}, cm1be: {=bool:?}, cm2be: {=bool:?}, cm3be: {=bool:?}, adte: {=bool:?} }}",
                self.ueve(),
                self.cm1ae(),
                self.cm2ae(),
                self.cm3ae(),
                self.cm1be(),
                self.cm2be(),
                self.cm3be(),
                self.adte()
            )
        }
    }
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(524287)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(524287)
    }
}
