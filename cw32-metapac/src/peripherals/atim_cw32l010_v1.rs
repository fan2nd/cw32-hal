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
    #[doc = "Control Register1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Slave Mode Control Register."]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Event generation register."]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture compare mode register 1."]
    #[inline(always)]
    pub const fn ccmr1cap(self) -> crate::common::Reg<regs::Ccmr1cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture compare mode register 1."]
    #[inline(always)]
    pub const fn ccmr1cmp(self) -> crate::common::Reg<regs::Ccmr1cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture compare mode register 2."]
    #[inline(always)]
    pub const fn ccmr2cap(self) -> crate::common::Reg<regs::Ccmr2cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture compare mode register 2."]
    #[inline(always)]
    pub const fn ccmr2cmp(self) -> crate::common::Reg<regs::Ccmr2cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler Register."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Auto reload Register."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Repetition Counter Register."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture compare register1."]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "capture compare register2."]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::common::Reg<regs::Ccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "capture compare register3."]
    #[inline(always)]
    pub const fn ccr3(self) -> crate::common::Reg<regs::Ccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "capture compare register4."]
    #[inline(always)]
    pub const fn ccr4(self) -> crate::common::Reg<regs::Ccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "break and dead-time register."]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "capture compare register5."]
    #[inline(always)]
    pub const fn ccr5(self) -> crate::common::Reg<regs::Ccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "capture compare register6."]
    #[inline(always)]
    pub const fn ccr6(self) -> crate::common::Reg<regs::Ccr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "capture compare mode register 3."]
    #[inline(always)]
    pub const fn ccmr3cap(self) -> crate::common::Reg<regs::Ccmr3cap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "capture compare mode register 3."]
    #[inline(always)]
    pub const fn ccmr3cmp(self) -> crate::common::Reg<regs::Ccmr3cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "dead-time register2."]
    #[inline(always)]
    pub const fn dtr2(self) -> crate::common::Reg<regs::Dtr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Encoder control Register."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Timer Input Select Register1."]
    #[inline(always)]
    pub const fn tisel1(self) -> crate::common::Reg<regs::Tisel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Alternate function Register1."]
    #[inline(always)]
    pub const fn af1(self) -> crate::common::Reg<regs::Af1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Alternate function Register2."]
    #[inline(always)]
    pub const fn af2(self) -> crate::common::Reg<regs::Af2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Timer Input Select Register2."]
    #[inline(always)]
    pub const fn tisel2(self) -> crate::common::Reg<regs::Tisel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Interrupt clean register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
}
pub mod regs {
    #[doc = "Alternate function Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af1(pub u32);
    impl Af1 {
        #[doc = "desc BKINE."]
        #[must_use]
        #[inline(always)]
        pub const fn bkine(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKINE."]
        #[inline(always)]
        pub const fn set_bkine(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BKVC1E."]
        #[must_use]
        #[inline(always)]
        pub const fn bkvc1e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKVC1E."]
        #[inline(always)]
        pub const fn set_bkvc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BKVC2E."]
        #[must_use]
        #[inline(always)]
        pub const fn bkvc2e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKVC2E."]
        #[inline(always)]
        pub const fn set_bkvc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BKINP."]
        #[must_use]
        #[inline(always)]
        pub const fn bkinp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKINP."]
        #[inline(always)]
        pub const fn set_bkinp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BKVC1P."]
        #[must_use]
        #[inline(always)]
        pub const fn bkvc1p(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKVC1P."]
        #[inline(always)]
        pub const fn set_bkvc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BKVC2P."]
        #[must_use]
        #[inline(always)]
        pub const fn bkvc2p(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKVC2P."]
        #[inline(always)]
        pub const fn set_bkvc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc ETRSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn etrsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "desc ETRSEL."]
        #[inline(always)]
        pub const fn set_etrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
    }
    impl Default for Af1 {
        #[inline(always)]
        fn default() -> Af1 {
            Af1(0)
        }
    }
    impl core::fmt::Debug for Af1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af1")
                .field("bkine", &self.bkine())
                .field("bkvc1e", &self.bkvc1e())
                .field("bkvc2e", &self.bkvc2e())
                .field("bkinp", &self.bkinp())
                .field("bkvc1p", &self.bkvc1p())
                .field("bkvc2p", &self.bkvc2p())
                .field("etrsel", &self.etrsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Af1 {{ bkine: {=bool:?}, bkvc1e: {=bool:?}, bkvc2e: {=bool:?}, bkinp: {=bool:?}, bkvc1p: {=bool:?}, bkvc2p: {=bool:?}, etrsel: {=u8:?} }}",
                self.bkine(),
                self.bkvc1e(),
                self.bkvc2e(),
                self.bkinp(),
                self.bkvc1p(),
                self.bkvc2p(),
                self.etrsel()
            )
        }
    }
    #[doc = "Alternate function Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Af2(pub u32);
    impl Af2 {
        #[doc = "desc BK2INE."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2ine(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2INE."]
        #[inline(always)]
        pub const fn set_bk2ine(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BK2VC1E."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2vc1e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2VC1E."]
        #[inline(always)]
        pub const fn set_bk2vc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BK2VC2E."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2vc2e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2VC2E."]
        #[inline(always)]
        pub const fn set_bk2vc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BK2INP."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2inp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2INP."]
        #[inline(always)]
        pub const fn set_bk2inp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BK2VC1P."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2vc1p(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2VC1P."]
        #[inline(always)]
        pub const fn set_bk2vc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BK2VC2P."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2vc2p(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2VC2P."]
        #[inline(always)]
        pub const fn set_bk2vc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OCRSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ocrsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc OCRSEL."]
        #[inline(always)]
        pub const fn set_ocrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Af2 {
        #[inline(always)]
        fn default() -> Af2 {
            Af2(0)
        }
    }
    impl core::fmt::Debug for Af2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Af2")
                .field("bk2ine", &self.bk2ine())
                .field("bk2vc1e", &self.bk2vc1e())
                .field("bk2vc2e", &self.bk2vc2e())
                .field("bk2inp", &self.bk2inp())
                .field("bk2vc1p", &self.bk2vc1p())
                .field("bk2vc2p", &self.bk2vc2p())
                .field("ocrsel", &self.ocrsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Af2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Af2 {{ bk2ine: {=bool:?}, bk2vc1e: {=bool:?}, bk2vc2e: {=bool:?}, bk2inp: {=bool:?}, bk2vc1p: {=bool:?}, bk2vc2p: {=bool:?}, ocrsel: {=u8:?} }}",
                self.bk2ine(),
                self.bk2vc1e(),
                self.bk2vc2e(),
                self.bk2inp(),
                self.bk2vc1p(),
                self.bk2vc2p(),
                self.ocrsel()
            )
        }
    }
    #[doc = "Auto reload Register."]
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
    #[doc = "break and dead-time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr(pub u32);
    impl Bdtr {
        #[doc = "desc DTG."]
        #[must_use]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DTG."]
        #[inline(always)]
        pub const fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc LOCK."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc OSSI."]
        #[must_use]
        #[inline(always)]
        pub const fn ossi(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSSI."]
        #[inline(always)]
        pub const fn set_ossi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OSSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ossr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSSR."]
        #[inline(always)]
        pub const fn set_ossr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BKE."]
        #[must_use]
        #[inline(always)]
        pub const fn bke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKE."]
        #[inline(always)]
        pub const fn set_bke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BKP."]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKP."]
        #[inline(always)]
        pub const fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc AOE."]
        #[must_use]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc AOE."]
        #[inline(always)]
        pub const fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc MOE."]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc MOE."]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc BKF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BKF."]
        #[inline(always)]
        pub const fn set_bkf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc BK2F."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2f(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BK2F."]
        #[inline(always)]
        pub const fn set_bk2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc BK2E."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2e(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2E."]
        #[inline(always)]
        pub const fn set_bk2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc BK2P."]
        #[must_use]
        #[inline(always)]
        pub const fn bk2p(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc BK2P."]
        #[inline(always)]
        pub const fn set_bk2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Bdtr {
        #[inline(always)]
        fn default() -> Bdtr {
            Bdtr(0)
        }
    }
    impl core::fmt::Debug for Bdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdtr")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke", &self.bke())
                .field("bkp", &self.bkp())
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .field("bkf", &self.bkf())
                .field("bk2f", &self.bk2f())
                .field("bk2e", &self.bk2e())
                .field("bk2p", &self.bk2p())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdtr {{ dtg: {=u8:?}, lock: {=u8:?}, ossi: {=bool:?}, ossr: {=bool:?}, bke: {=bool:?}, bkp: {=bool:?}, aoe: {=bool:?}, moe: {=bool:?}, bkf: {=u8:?}, bk2f: {=u8:?}, bk2e: {=bool:?}, bk2p: {=bool:?} }}",
                self.dtg(),
                self.lock(),
                self.ossi(),
                self.ossr(),
                self.bke(),
                self.bkp(),
                self.aoe(),
                self.moe(),
                self.bkf(),
                self.bk2f(),
                self.bk2e(),
                self.bk2p()
            )
        }
    }
    #[doc = "capture compare enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer(pub u32);
    impl Ccer {
        #[doc = "desc CC1E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1E."]
        #[inline(always)]
        pub const fn set_cc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CC1P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1P."]
        #[inline(always)]
        pub const fn set_cc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC1NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1NE."]
        #[inline(always)]
        pub const fn set_cc1ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC1NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1np(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1NP."]
        #[inline(always)]
        pub const fn set_cc1np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC2E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2E."]
        #[inline(always)]
        pub const fn set_cc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CC2P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2p(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2P."]
        #[inline(always)]
        pub const fn set_cc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CC2NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2NE."]
        #[inline(always)]
        pub const fn set_cc2ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CC2NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2np(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2NP."]
        #[inline(always)]
        pub const fn set_cc2np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CC3E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3E."]
        #[inline(always)]
        pub const fn set_cc3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CC3P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3P."]
        #[inline(always)]
        pub const fn set_cc3p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CC3NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3NE."]
        #[inline(always)]
        pub const fn set_cc3ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CC3NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3np(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3NP."]
        #[inline(always)]
        pub const fn set_cc3np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CC4E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4E."]
        #[inline(always)]
        pub const fn set_cc4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc CC4P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4p(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4P."]
        #[inline(always)]
        pub const fn set_cc4p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc CC4NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4ne(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4NE."]
        #[inline(always)]
        pub const fn set_cc4ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CC4NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4np(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4NP."]
        #[inline(always)]
        pub const fn set_cc4np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc CC5E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5e(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5E."]
        #[inline(always)]
        pub const fn set_cc5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CC5P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5p(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5P."]
        #[inline(always)]
        pub const fn set_cc5p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CC5NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5ne(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5NE."]
        #[inline(always)]
        pub const fn set_cc5ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CC5NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5np(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5NP."]
        #[inline(always)]
        pub const fn set_cc5np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc CC6E."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6e(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6E."]
        #[inline(always)]
        pub const fn set_cc6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc CC6P."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6p(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6P."]
        #[inline(always)]
        pub const fn set_cc6p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc CC6NE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6ne(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6NE."]
        #[inline(always)]
        pub const fn set_cc6ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc CC6NP."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6np(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6NP."]
        #[inline(always)]
        pub const fn set_cc6np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ccer {
        #[inline(always)]
        fn default() -> Ccer {
            Ccer(0)
        }
    }
    impl core::fmt::Debug for Ccer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer")
                .field("cc1e", &self.cc1e())
                .field("cc1p", &self.cc1p())
                .field("cc1ne", &self.cc1ne())
                .field("cc1np", &self.cc1np())
                .field("cc2e", &self.cc2e())
                .field("cc2p", &self.cc2p())
                .field("cc2ne", &self.cc2ne())
                .field("cc2np", &self.cc2np())
                .field("cc3e", &self.cc3e())
                .field("cc3p", &self.cc3p())
                .field("cc3ne", &self.cc3ne())
                .field("cc3np", &self.cc3np())
                .field("cc4e", &self.cc4e())
                .field("cc4p", &self.cc4p())
                .field("cc4ne", &self.cc4ne())
                .field("cc4np", &self.cc4np())
                .field("cc5e", &self.cc5e())
                .field("cc5p", &self.cc5p())
                .field("cc5ne", &self.cc5ne())
                .field("cc5np", &self.cc5np())
                .field("cc6e", &self.cc6e())
                .field("cc6p", &self.cc6p())
                .field("cc6ne", &self.cc6ne())
                .field("cc6np", &self.cc6np())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer {{ cc1e: {=bool:?}, cc1p: {=bool:?}, cc1ne: {=bool:?}, cc1np: {=bool:?}, cc2e: {=bool:?}, cc2p: {=bool:?}, cc2ne: {=bool:?}, cc2np: {=bool:?}, cc3e: {=bool:?}, cc3p: {=bool:?}, cc3ne: {=bool:?}, cc3np: {=bool:?}, cc4e: {=bool:?}, cc4p: {=bool:?}, cc4ne: {=bool:?}, cc4np: {=bool:?}, cc5e: {=bool:?}, cc5p: {=bool:?}, cc5ne: {=bool:?}, cc5np: {=bool:?}, cc6e: {=bool:?}, cc6p: {=bool:?}, cc6ne: {=bool:?}, cc6np: {=bool:?} }}",
                self.cc1e(),
                self.cc1p(),
                self.cc1ne(),
                self.cc1np(),
                self.cc2e(),
                self.cc2p(),
                self.cc2ne(),
                self.cc2np(),
                self.cc3e(),
                self.cc3p(),
                self.cc3ne(),
                self.cc3np(),
                self.cc4e(),
                self.cc4p(),
                self.cc4ne(),
                self.cc4np(),
                self.cc5e(),
                self.cc5p(),
                self.cc5ne(),
                self.cc5np(),
                self.cc6e(),
                self.cc6p(),
                self.cc6ne(),
                self.cc6np()
            )
        }
    }
    #[doc = "capture compare mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr1cap(pub u32);
    impl Ccmr1cap {
        #[doc = "desc CC1S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC1S."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc IC1PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC1PSC."]
        #[inline(always)]
        pub const fn set_ic1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc IC1F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC1F."]
        #[inline(always)]
        pub const fn set_ic1f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc CC2S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC2S."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc IC2PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC2PSC."]
        #[inline(always)]
        pub const fn set_ic2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc IC2F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC2F."]
        #[inline(always)]
        pub const fn set_ic2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ccmr1cap {
        #[inline(always)]
        fn default() -> Ccmr1cap {
            Ccmr1cap(0)
        }
    }
    impl core::fmt::Debug for Ccmr1cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr1cap")
                .field("cc1s", &self.cc1s())
                .field("ic1psc", &self.ic1psc())
                .field("ic1f", &self.ic1f())
                .field("cc2s", &self.cc2s())
                .field("ic2psc", &self.ic2psc())
                .field("ic2f", &self.ic2f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr1cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr1cap {{ cc1s: {=u8:?}, ic1psc: {=u8:?}, ic1f: {=u8:?}, cc2s: {=u8:?}, ic2psc: {=u8:?}, ic2f: {=u8:?} }}",
                self.cc1s(),
                self.ic1psc(),
                self.ic1f(),
                self.cc2s(),
                self.ic2psc(),
                self.ic2f()
            )
        }
    }
    #[doc = "capture compare mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr1cmp(pub u32);
    impl Ccmr1cmp {
        #[doc = "desc CC1S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC1S."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc OC1FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC1FE."]
        #[inline(always)]
        pub const fn set_oc1fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OC1PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1pe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC1PE."]
        #[inline(always)]
        pub const fn set_oc1pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OC1M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1m(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC1M."]
        #[inline(always)]
        pub const fn set_oc1m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc OC1CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1ce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC1CE."]
        #[inline(always)]
        pub const fn set_oc1ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CC2S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC2S."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc OC2FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2fe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC2FE."]
        #[inline(always)]
        pub const fn set_oc2fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OC2PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2pe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC2PE."]
        #[inline(always)]
        pub const fn set_oc2pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OC2M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2m(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC2M."]
        #[inline(always)]
        pub const fn set_oc2m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc OC2CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2ce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC2CE."]
        #[inline(always)]
        pub const fn set_oc2ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OC1MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1mh(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC1MH."]
        #[inline(always)]
        pub const fn set_oc1mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OC2MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2mh(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC2MH."]
        #[inline(always)]
        pub const fn set_oc2mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccmr1cmp {
        #[inline(always)]
        fn default() -> Ccmr1cmp {
            Ccmr1cmp(0)
        }
    }
    impl core::fmt::Debug for Ccmr1cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr1cmp")
                .field("cc1s", &self.cc1s())
                .field("oc1fe", &self.oc1fe())
                .field("oc1pe", &self.oc1pe())
                .field("oc1m", &self.oc1m())
                .field("oc1ce", &self.oc1ce())
                .field("cc2s", &self.cc2s())
                .field("oc2fe", &self.oc2fe())
                .field("oc2pe", &self.oc2pe())
                .field("oc2m", &self.oc2m())
                .field("oc2ce", &self.oc2ce())
                .field("oc1mh", &self.oc1mh())
                .field("oc2mh", &self.oc2mh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr1cmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr1cmp {{ cc1s: {=u8:?}, oc1fe: {=bool:?}, oc1pe: {=bool:?}, oc1m: {=u8:?}, oc1ce: {=bool:?}, cc2s: {=u8:?}, oc2fe: {=bool:?}, oc2pe: {=bool:?}, oc2m: {=u8:?}, oc2ce: {=bool:?}, oc1mh: {=bool:?}, oc2mh: {=bool:?} }}",
                self.cc1s(),
                self.oc1fe(),
                self.oc1pe(),
                self.oc1m(),
                self.oc1ce(),
                self.cc2s(),
                self.oc2fe(),
                self.oc2pe(),
                self.oc2m(),
                self.oc2ce(),
                self.oc1mh(),
                self.oc2mh()
            )
        }
    }
    #[doc = "capture compare mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr2cap(pub u32);
    impl Ccmr2cap {
        #[doc = "desc CC3S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC3S."]
        #[inline(always)]
        pub const fn set_cc3s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc IC3PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic3psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC3PSC."]
        #[inline(always)]
        pub const fn set_ic3psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc IC3F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic3f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC3F."]
        #[inline(always)]
        pub const fn set_ic3f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc CC4S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC4S."]
        #[inline(always)]
        pub const fn set_cc4s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc IC4PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic4psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC4PSC."]
        #[inline(always)]
        pub const fn set_ic4psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc IC4F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic4f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC4F."]
        #[inline(always)]
        pub const fn set_ic4f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ccmr2cap {
        #[inline(always)]
        fn default() -> Ccmr2cap {
            Ccmr2cap(0)
        }
    }
    impl core::fmt::Debug for Ccmr2cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr2cap")
                .field("cc3s", &self.cc3s())
                .field("ic3psc", &self.ic3psc())
                .field("ic3f", &self.ic3f())
                .field("cc4s", &self.cc4s())
                .field("ic4psc", &self.ic4psc())
                .field("ic4f", &self.ic4f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr2cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr2cap {{ cc3s: {=u8:?}, ic3psc: {=u8:?}, ic3f: {=u8:?}, cc4s: {=u8:?}, ic4psc: {=u8:?}, ic4f: {=u8:?} }}",
                self.cc3s(),
                self.ic3psc(),
                self.ic3f(),
                self.cc4s(),
                self.ic4psc(),
                self.ic4f()
            )
        }
    }
    #[doc = "capture compare mode register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr2cmp(pub u32);
    impl Ccmr2cmp {
        #[doc = "desc CC3S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC3S."]
        #[inline(always)]
        pub const fn set_cc3s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc OC3FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC3FE."]
        #[inline(always)]
        pub const fn set_oc3fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OC3PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3pe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC3PE."]
        #[inline(always)]
        pub const fn set_oc3pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OC3M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3m(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC3M."]
        #[inline(always)]
        pub const fn set_oc3m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc OC3CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3ce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC3CE."]
        #[inline(always)]
        pub const fn set_oc3ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CC4S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC4S."]
        #[inline(always)]
        pub const fn set_cc4s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc OC4FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4fe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC4FE."]
        #[inline(always)]
        pub const fn set_oc4fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OC4PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4pe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC4PE."]
        #[inline(always)]
        pub const fn set_oc4pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OC4M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4m(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC4M."]
        #[inline(always)]
        pub const fn set_oc4m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc OC4CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4ce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC4CE."]
        #[inline(always)]
        pub const fn set_oc4ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OC3MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3mh(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC3MH."]
        #[inline(always)]
        pub const fn set_oc3mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OC4MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4mh(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC4MH."]
        #[inline(always)]
        pub const fn set_oc4mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccmr2cmp {
        #[inline(always)]
        fn default() -> Ccmr2cmp {
            Ccmr2cmp(0)
        }
    }
    impl core::fmt::Debug for Ccmr2cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr2cmp")
                .field("cc3s", &self.cc3s())
                .field("oc3fe", &self.oc3fe())
                .field("oc3pe", &self.oc3pe())
                .field("oc3m", &self.oc3m())
                .field("oc3ce", &self.oc3ce())
                .field("cc4s", &self.cc4s())
                .field("oc4fe", &self.oc4fe())
                .field("oc4pe", &self.oc4pe())
                .field("oc4m", &self.oc4m())
                .field("oc4ce", &self.oc4ce())
                .field("oc3mh", &self.oc3mh())
                .field("oc4mh", &self.oc4mh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr2cmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr2cmp {{ cc3s: {=u8:?}, oc3fe: {=bool:?}, oc3pe: {=bool:?}, oc3m: {=u8:?}, oc3ce: {=bool:?}, cc4s: {=u8:?}, oc4fe: {=bool:?}, oc4pe: {=bool:?}, oc4m: {=u8:?}, oc4ce: {=bool:?}, oc3mh: {=bool:?}, oc4mh: {=bool:?} }}",
                self.cc3s(),
                self.oc3fe(),
                self.oc3pe(),
                self.oc3m(),
                self.oc3ce(),
                self.cc4s(),
                self.oc4fe(),
                self.oc4pe(),
                self.oc4m(),
                self.oc4ce(),
                self.oc3mh(),
                self.oc4mh()
            )
        }
    }
    #[doc = "capture compare mode register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr3cap(pub u32);
    impl Ccmr3cap {
        #[doc = "desc CC5S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC5S."]
        #[inline(always)]
        pub const fn set_cc5s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc IC5PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic5psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC5PSC."]
        #[inline(always)]
        pub const fn set_ic5psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc IC5F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic5f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC5F."]
        #[inline(always)]
        pub const fn set_ic5f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc CC6S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC6S."]
        #[inline(always)]
        pub const fn set_cc6s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc IC6PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn ic6psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc IC6PSC."]
        #[inline(always)]
        pub const fn set_ic6psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc IC6F."]
        #[must_use]
        #[inline(always)]
        pub const fn ic6f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc IC6F."]
        #[inline(always)]
        pub const fn set_ic6f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ccmr3cap {
        #[inline(always)]
        fn default() -> Ccmr3cap {
            Ccmr3cap(0)
        }
    }
    impl core::fmt::Debug for Ccmr3cap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr3cap")
                .field("cc5s", &self.cc5s())
                .field("ic5psc", &self.ic5psc())
                .field("ic5f", &self.ic5f())
                .field("cc6s", &self.cc6s())
                .field("ic6psc", &self.ic6psc())
                .field("ic6f", &self.ic6f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr3cap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr3cap {{ cc5s: {=u8:?}, ic5psc: {=u8:?}, ic5f: {=u8:?}, cc6s: {=u8:?}, ic6psc: {=u8:?}, ic6f: {=u8:?} }}",
                self.cc5s(),
                self.ic5psc(),
                self.ic5f(),
                self.cc6s(),
                self.ic6psc(),
                self.ic6f()
            )
        }
    }
    #[doc = "capture compare mode register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr3cmp(pub u32);
    impl Ccmr3cmp {
        #[doc = "desc CC5S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC5S."]
        #[inline(always)]
        pub const fn set_cc5s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc OC5FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc5fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC5FE."]
        #[inline(always)]
        pub const fn set_oc5fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OC5PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc5pe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC5PE."]
        #[inline(always)]
        pub const fn set_oc5pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OC5M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc5m(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC5M."]
        #[inline(always)]
        pub const fn set_oc5m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc OC5CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc5ce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC5CE."]
        #[inline(always)]
        pub const fn set_oc5ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CC6S."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CC6S."]
        #[inline(always)]
        pub const fn set_cc6s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc OC6FE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc6fe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC6FE."]
        #[inline(always)]
        pub const fn set_oc6fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OC6PE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc6pe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC6PE."]
        #[inline(always)]
        pub const fn set_oc6pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OC6M."]
        #[must_use]
        #[inline(always)]
        pub const fn oc6m(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc OC6M."]
        #[inline(always)]
        pub const fn set_oc6m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc OC6CE."]
        #[must_use]
        #[inline(always)]
        pub const fn oc6ce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC6CE."]
        #[inline(always)]
        pub const fn set_oc6ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OC5MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc5mh(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC5MH."]
        #[inline(always)]
        pub const fn set_oc5mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OC6MH."]
        #[must_use]
        #[inline(always)]
        pub const fn oc6mh(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc OC6MH."]
        #[inline(always)]
        pub const fn set_oc6mh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccmr3cmp {
        #[inline(always)]
        fn default() -> Ccmr3cmp {
            Ccmr3cmp(0)
        }
    }
    impl core::fmt::Debug for Ccmr3cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr3cmp")
                .field("cc5s", &self.cc5s())
                .field("oc5fe", &self.oc5fe())
                .field("oc5pe", &self.oc5pe())
                .field("oc5m", &self.oc5m())
                .field("oc5ce", &self.oc5ce())
                .field("cc6s", &self.cc6s())
                .field("oc6fe", &self.oc6fe())
                .field("oc6pe", &self.oc6pe())
                .field("oc6m", &self.oc6m())
                .field("oc6ce", &self.oc6ce())
                .field("oc5mh", &self.oc5mh())
                .field("oc6mh", &self.oc6mh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr3cmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr3cmp {{ cc5s: {=u8:?}, oc5fe: {=bool:?}, oc5pe: {=bool:?}, oc5m: {=u8:?}, oc5ce: {=bool:?}, cc6s: {=u8:?}, oc6fe: {=bool:?}, oc6pe: {=bool:?}, oc6m: {=u8:?}, oc6ce: {=bool:?}, oc5mh: {=bool:?}, oc6mh: {=bool:?} }}",
                self.cc5s(),
                self.oc5fe(),
                self.oc5pe(),
                self.oc5m(),
                self.oc5ce(),
                self.cc6s(),
                self.oc6fe(),
                self.oc6pe(),
                self.oc6m(),
                self.oc6ce(),
                self.oc5mh(),
                self.oc6mh()
            )
        }
    }
    #[doc = "capture compare register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr1(pub u32);
    impl Ccr1 {
        #[doc = "desc CCR1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR1."]
        #[inline(always)]
        pub const fn set_ccr1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr1 {
        #[inline(always)]
        fn default() -> Ccr1 {
            Ccr1(0)
        }
    }
    impl core::fmt::Debug for Ccr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr1").field("ccr1", &self.ccr1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr1 {{ ccr1: {=u16:?} }}", self.ccr1())
        }
    }
    #[doc = "capture compare register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr2(pub u32);
    impl Ccr2 {
        #[doc = "desc CCR2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR2."]
        #[inline(always)]
        pub const fn set_ccr2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr2 {
        #[inline(always)]
        fn default() -> Ccr2 {
            Ccr2(0)
        }
    }
    impl core::fmt::Debug for Ccr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr2").field("ccr2", &self.ccr2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr2 {{ ccr2: {=u16:?} }}", self.ccr2())
        }
    }
    #[doc = "capture compare register3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr3(pub u32);
    impl Ccr3 {
        #[doc = "desc CCR3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR3."]
        #[inline(always)]
        pub const fn set_ccr3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr3 {
        #[inline(always)]
        fn default() -> Ccr3 {
            Ccr3(0)
        }
    }
    impl core::fmt::Debug for Ccr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr3").field("ccr3", &self.ccr3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr3 {{ ccr3: {=u16:?} }}", self.ccr3())
        }
    }
    #[doc = "capture compare register4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr4(pub u32);
    impl Ccr4 {
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
    impl Default for Ccr4 {
        #[inline(always)]
        fn default() -> Ccr4 {
            Ccr4(0)
        }
    }
    impl core::fmt::Debug for Ccr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr4").field("ccr4", &self.ccr4()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr4 {{ ccr4: {=u16:?} }}", self.ccr4())
        }
    }
    #[doc = "capture compare register5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr5(pub u32);
    impl Ccr5 {
        #[doc = "desc CCR5."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR5."]
        #[inline(always)]
        pub const fn set_ccr5(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc GC5C1."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C1."]
        #[inline(always)]
        pub const fn set_gc5c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc GC5C2."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c2(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C2."]
        #[inline(always)]
        pub const fn set_gc5c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc GC5C3."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c3(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C3."]
        #[inline(always)]
        pub const fn set_gc5c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc GC5C4."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c4(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C4."]
        #[inline(always)]
        pub const fn set_gc5c4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc GC5C5."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c5(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C5."]
        #[inline(always)]
        pub const fn set_gc5c5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc GC5C6."]
        #[must_use]
        #[inline(always)]
        pub const fn gc5c6(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC5C6."]
        #[inline(always)]
        pub const fn set_gc5c6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ccr5 {
        #[inline(always)]
        fn default() -> Ccr5 {
            Ccr5(0)
        }
    }
    impl core::fmt::Debug for Ccr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr5")
                .field("ccr5", &self.ccr5())
                .field("gc5c1", &self.gc5c1())
                .field("gc5c2", &self.gc5c2())
                .field("gc5c3", &self.gc5c3())
                .field("gc5c4", &self.gc5c4())
                .field("gc5c5", &self.gc5c5())
                .field("gc5c6", &self.gc5c6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr5 {{ ccr5: {=u16:?}, gc5c1: {=bool:?}, gc5c2: {=bool:?}, gc5c3: {=bool:?}, gc5c4: {=bool:?}, gc5c5: {=bool:?}, gc5c6: {=bool:?} }}",
                self.ccr5(),
                self.gc5c1(),
                self.gc5c2(),
                self.gc5c3(),
                self.gc5c4(),
                self.gc5c5(),
                self.gc5c6()
            )
        }
    }
    #[doc = "capture compare register6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr6(pub u32);
    impl Ccr6 {
        #[doc = "desc CCR6."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr6(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR6."]
        #[inline(always)]
        pub const fn set_ccr6(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc GC6C1."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C1."]
        #[inline(always)]
        pub const fn set_gc6c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc GC6C2."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c2(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C2."]
        #[inline(always)]
        pub const fn set_gc6c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc GC6C3."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c3(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C3."]
        #[inline(always)]
        pub const fn set_gc6c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc GC6C4."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c4(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C4."]
        #[inline(always)]
        pub const fn set_gc6c4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc GC6C5."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c5(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C5."]
        #[inline(always)]
        pub const fn set_gc6c5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc GC6C6."]
        #[must_use]
        #[inline(always)]
        pub const fn gc6c6(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC6C6."]
        #[inline(always)]
        pub const fn set_gc6c6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ccr6 {
        #[inline(always)]
        fn default() -> Ccr6 {
            Ccr6(0)
        }
    }
    impl core::fmt::Debug for Ccr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr6")
                .field("ccr6", &self.ccr6())
                .field("gc6c1", &self.gc6c1())
                .field("gc6c2", &self.gc6c2())
                .field("gc6c3", &self.gc6c3())
                .field("gc6c4", &self.gc6c4())
                .field("gc6c5", &self.gc6c5())
                .field("gc6c6", &self.gc6c6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr6 {{ ccr6: {=u16:?}, gc6c1: {=bool:?}, gc6c2: {=bool:?}, gc6c3: {=bool:?}, gc6c4: {=bool:?}, gc6c5: {=bool:?}, gc6c6: {=bool:?} }}",
                self.ccr6(),
                self.gc6c1(),
                self.gc6c2(),
                self.gc6c3(),
                self.gc6c4(),
                self.gc6c5(),
                self.gc6c6()
            )
        }
    }
    #[doc = "Counter Register."]
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
        #[doc = "desc UIFCPY."]
        #[must_use]
        #[inline(always)]
        pub const fn uifcpy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIFCPY."]
        #[inline(always)]
        pub const fn set_uifcpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            f.debug_struct("Cnt")
                .field("cnt", &self.cnt())
                .field("uifcpy", &self.uifcpy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt {{ cnt: {=u16:?}, uifcpy: {=bool:?} }}",
                self.cnt(),
                self.uifcpy()
            )
        }
    }
    #[doc = "Control Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc CEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CEN."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc UDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDIS."]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc URS."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc URS."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OPM."]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPM."]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIR."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CMS."]
        #[must_use]
        #[inline(always)]
        pub const fn cms(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMS."]
        #[inline(always)]
        pub const fn set_cms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc ARPE."]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARPE."]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CKD."]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CKD."]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc UIFREMAP."]
        #[must_use]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIFREMAP."]
        #[inline(always)]
        pub const fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("uifremap", &self.uifremap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {=bool:?}, opm: {=bool:?}, dir: {=bool:?}, cms: {=u8:?}, arpe: {=bool:?}, ckd: {=u8:?}, uifremap: {=bool:?} }}",
                self.cen(),
                self.udis(),
                self.urs(),
                self.opm(),
                self.dir(),
                self.cms(),
                self.arpe(),
                self.ckd(),
                self.uifremap()
            )
        }
    }
    #[doc = "Control Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc CCPC."]
        #[must_use]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCPC."]
        #[inline(always)]
        pub const fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CCUS."]
        #[must_use]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCUS."]
        #[inline(always)]
        pub const fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MMS."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc MMS."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc TI1S."]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI1S."]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc OIS1."]
        #[must_use]
        #[inline(always)]
        pub const fn ois1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS1."]
        #[inline(always)]
        pub const fn set_ois1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc OIS1N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois1n(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS1N."]
        #[inline(always)]
        pub const fn set_ois1n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc OIS2."]
        #[must_use]
        #[inline(always)]
        pub const fn ois2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS2."]
        #[inline(always)]
        pub const fn set_ois2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OIS2N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois2n(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS2N."]
        #[inline(always)]
        pub const fn set_ois2n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc OIS3."]
        #[must_use]
        #[inline(always)]
        pub const fn ois3(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS3."]
        #[inline(always)]
        pub const fn set_ois3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc OIS3N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois3n(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS3N."]
        #[inline(always)]
        pub const fn set_ois3n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc OIS4."]
        #[must_use]
        #[inline(always)]
        pub const fn ois4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS4."]
        #[inline(always)]
        pub const fn set_ois4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc OIS4N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois4n(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS4N."]
        #[inline(always)]
        pub const fn set_ois4n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc OIS5."]
        #[must_use]
        #[inline(always)]
        pub const fn ois5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS5."]
        #[inline(always)]
        pub const fn set_ois5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OIS5N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois5n(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS5N."]
        #[inline(always)]
        pub const fn set_ois5n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc OIS6."]
        #[must_use]
        #[inline(always)]
        pub const fn ois6(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS6."]
        #[inline(always)]
        pub const fn set_ois6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc OIS6N."]
        #[must_use]
        #[inline(always)]
        pub const fn ois6n(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc OIS6N."]
        #[inline(always)]
        pub const fn set_ois6n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc MMS2."]
        #[must_use]
        #[inline(always)]
        pub const fn mms2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MMS2."]
        #[inline(always)]
        pub const fn set_mms2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "desc MMSH."]
        #[must_use]
        #[inline(always)]
        pub const fn mmsh(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "desc MMSH."]
        #[inline(always)]
        pub const fn set_mmsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
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
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois1", &self.ois1())
                .field("ois1n", &self.ois1n())
                .field("ois2", &self.ois2())
                .field("ois2n", &self.ois2n())
                .field("ois3", &self.ois3())
                .field("ois3n", &self.ois3n())
                .field("ois4", &self.ois4())
                .field("ois4n", &self.ois4n())
                .field("ois5", &self.ois5())
                .field("ois5n", &self.ois5n())
                .field("ois6", &self.ois6())
                .field("ois6n", &self.ois6n())
                .field("mms2", &self.mms2())
                .field("mmsh", &self.mmsh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ ccpc: {=bool:?}, ccus: {=bool:?}, mms: {=u8:?}, ti1s: {=bool:?}, ois1: {=bool:?}, ois1n: {=bool:?}, ois2: {=bool:?}, ois2n: {=bool:?}, ois3: {=bool:?}, ois3n: {=bool:?}, ois4: {=bool:?}, ois4n: {=bool:?}, ois5: {=bool:?}, ois5n: {=bool:?}, ois6: {=bool:?}, ois6n: {=bool:?}, mms2: {=u8:?}, mmsh: {=u8:?} }}",
                self.ccpc(),
                self.ccus(),
                self.mms(),
                self.ti1s(),
                self.ois1(),
                self.ois1n(),
                self.ois2(),
                self.ois2n(),
                self.ois3(),
                self.ois3n(),
                self.ois4(),
                self.ois4n(),
                self.ois5(),
                self.ois5n(),
                self.ois6(),
                self.ois6n(),
                self.mms2(),
                self.mmsh()
            )
        }
    }
    #[doc = "dead-time register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr2(pub u32);
    impl Dtr2 {
        #[doc = "desc DTGF."]
        #[must_use]
        #[inline(always)]
        pub const fn dtgf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DTGF."]
        #[inline(always)]
        pub const fn set_dtgf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc DTAE."]
        #[must_use]
        #[inline(always)]
        pub const fn dtae(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTAE."]
        #[inline(always)]
        pub const fn set_dtae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DTPE."]
        #[must_use]
        #[inline(always)]
        pub const fn dtpe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTPE."]
        #[inline(always)]
        pub const fn set_dtpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Dtr2 {
        #[inline(always)]
        fn default() -> Dtr2 {
            Dtr2(0)
        }
    }
    impl core::fmt::Debug for Dtr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr2")
                .field("dtgf", &self.dtgf())
                .field("dtae", &self.dtae())
                .field("dtpe", &self.dtpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dtr2 {{ dtgf: {=u8:?}, dtae: {=bool:?}, dtpe: {=bool:?} }}",
                self.dtgf(),
                self.dtae(),
                self.dtpe()
            )
        }
    }
    #[doc = "Encoder control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IDIR."]
        #[must_use]
        #[inline(always)]
        pub const fn idir(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc IDIR."]
        #[inline(always)]
        pub const fn set_idir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc FIDX."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIDX."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc IPOS."]
        #[must_use]
        #[inline(always)]
        pub const fn ipos(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc IPOS."]
        #[inline(always)]
        pub const fn set_ipos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    impl core::fmt::Debug for Ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecr")
                .field("ie", &self.ie())
                .field("idir", &self.idir())
                .field("fidx", &self.fidx())
                .field("ipos", &self.ipos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ecr {{ ie: {=bool:?}, idir: {=u8:?}, fidx: {=bool:?}, ipos: {=u8:?} }}",
                self.ie(),
                self.idir(),
                self.fidx(),
                self.ipos()
            )
        }
    }
    #[doc = "Event generation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr(pub u32);
    impl Egr {
        #[doc = "desc UG."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UG."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CC1G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1g(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1G."]
        #[inline(always)]
        pub const fn set_cc1g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC2G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2g(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2G."]
        #[inline(always)]
        pub const fn set_cc2g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC3G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3g(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3G."]
        #[inline(always)]
        pub const fn set_cc3g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC4G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4g(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4G."]
        #[inline(always)]
        pub const fn set_cc4g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COMG."]
        #[must_use]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMG."]
        #[inline(always)]
        pub const fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TG."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TG."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BG."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BG."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc B2G."]
        #[must_use]
        #[inline(always)]
        pub const fn b2g(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2G."]
        #[inline(always)]
        pub const fn set_b2g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CC5G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5g(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5G."]
        #[inline(always)]
        pub const fn set_cc5g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CC6G."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6g(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6G."]
        #[inline(always)]
        pub const fn set_cc6g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Egr {
        #[inline(always)]
        fn default() -> Egr {
            Egr(0)
        }
    }
    impl core::fmt::Debug for Egr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr")
                .field("ug", &self.ug())
                .field("cc1g", &self.cc1g())
                .field("cc2g", &self.cc2g())
                .field("cc3g", &self.cc3g())
                .field("cc4g", &self.cc4g())
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg", &self.bg())
                .field("b2g", &self.b2g())
                .field("cc5g", &self.cc5g())
                .field("cc6g", &self.cc6g())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr {{ ug: {=bool:?}, cc1g: {=bool:?}, cc2g: {=bool:?}, cc3g: {=bool:?}, cc4g: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg: {=bool:?}, b2g: {=bool:?}, cc5g: {=bool:?}, cc6g: {=bool:?} }}",
                self.ug(),
                self.cc1g(),
                self.cc2g(),
                self.cc3g(),
                self.cc4g(),
                self.comg(),
                self.tg(),
                self.bg(),
                self.b2g(),
                self.cc5g(),
                self.cc6g()
            )
        }
    }
    #[doc = "Interrupt clean register."]
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
        #[doc = "desc CC1IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1IF."]
        #[inline(always)]
        pub const fn set_cc1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC2IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2IF."]
        #[inline(always)]
        pub const fn set_cc2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC3IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3IF."]
        #[inline(always)]
        pub const fn set_cc3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC4IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4IF."]
        #[inline(always)]
        pub const fn set_cc4if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COMIF."]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMIF."]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIF."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BIF."]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIF."]
        #[inline(always)]
        pub const fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc B2IF."]
        #[must_use]
        #[inline(always)]
        pub const fn b2if(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2IF."]
        #[inline(always)]
        pub const fn set_b2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CC1OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1of(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1OF."]
        #[inline(always)]
        pub const fn set_cc1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CC2OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2of(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2OF."]
        #[inline(always)]
        pub const fn set_cc2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CC3OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3of(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3OF."]
        #[inline(always)]
        pub const fn set_cc3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CC4OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4of(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4OF."]
        #[inline(always)]
        pub const fn set_cc4of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SBIF."]
        #[must_use]
        #[inline(always)]
        pub const fn sbif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SBIF."]
        #[inline(always)]
        pub const fn set_sbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc CC5IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5if(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5IF."]
        #[inline(always)]
        pub const fn set_cc5if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CC6IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6if(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6IF."]
        #[inline(always)]
        pub const fn set_cc6if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CC5OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5of(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5OF."]
        #[inline(always)]
        pub const fn set_cc5of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CC6OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6of(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6OF."]
        #[inline(always)]
        pub const fn set_cc6of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IDXF."]
        #[must_use]
        #[inline(always)]
        pub const fn idxf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDXF."]
        #[inline(always)]
        pub const fn set_idxf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc DIRF."]
        #[must_use]
        #[inline(always)]
        pub const fn dirf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRF."]
        #[inline(always)]
        pub const fn set_dirf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc IERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn ierrf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc IERRF."]
        #[inline(always)]
        pub const fn set_ierrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn terrf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TERRF."]
        #[inline(always)]
        pub const fn set_terrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("cc1if", &self.cc1if())
                .field("cc2if", &self.cc2if())
                .field("cc3if", &self.cc3if())
                .field("cc4if", &self.cc4if())
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif", &self.bif())
                .field("b2if", &self.b2if())
                .field("cc1of", &self.cc1of())
                .field("cc2of", &self.cc2of())
                .field("cc3of", &self.cc3of())
                .field("cc4of", &self.cc4of())
                .field("sbif", &self.sbif())
                .field("cc5if", &self.cc5if())
                .field("cc6if", &self.cc6if())
                .field("cc5of", &self.cc5of())
                .field("cc6of", &self.cc6of())
                .field("idxf", &self.idxf())
                .field("dirf", &self.dirf())
                .field("ierrf", &self.ierrf())
                .field("terrf", &self.terrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ uif: {=bool:?}, cc1if: {=bool:?}, cc2if: {=bool:?}, cc3if: {=bool:?}, cc4if: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif: {=bool:?}, b2if: {=bool:?}, cc1of: {=bool:?}, cc2of: {=bool:?}, cc3of: {=bool:?}, cc4of: {=bool:?}, sbif: {=bool:?}, cc5if: {=bool:?}, cc6if: {=bool:?}, cc5of: {=bool:?}, cc6of: {=bool:?}, idxf: {=bool:?}, dirf: {=bool:?}, ierrf: {=bool:?}, terrf: {=bool:?} }}",
                self.uif(),
                self.cc1if(),
                self.cc2if(),
                self.cc3if(),
                self.cc4if(),
                self.comif(),
                self.tif(),
                self.bif(),
                self.b2if(),
                self.cc1of(),
                self.cc2of(),
                self.cc3of(),
                self.cc4of(),
                self.sbif(),
                self.cc5if(),
                self.cc6if(),
                self.cc5of(),
                self.cc6of(),
                self.idxf(),
                self.dirf(),
                self.ierrf(),
                self.terrf()
            )
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc UIE."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIE."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CC1IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1IE."]
        #[inline(always)]
        pub const fn set_cc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC2IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2IE."]
        #[inline(always)]
        pub const fn set_cc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC3IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3IE."]
        #[inline(always)]
        pub const fn set_cc3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC4IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4IE."]
        #[inline(always)]
        pub const fn set_cc4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COMIE."]
        #[must_use]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMIE."]
        #[inline(always)]
        pub const fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIE."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BIE."]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIE."]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CC5IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5IE."]
        #[inline(always)]
        pub const fn set_cc5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CC6IE."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6IE."]
        #[inline(always)]
        pub const fn set_cc6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc IDXIE."]
        #[must_use]
        #[inline(always)]
        pub const fn idxie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDXIE."]
        #[inline(always)]
        pub const fn set_idxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc DIRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn dirie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRIE."]
        #[inline(always)]
        pub const fn set_dirie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc IERRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ierrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc IERRIE."]
        #[inline(always)]
        pub const fn set_ierrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TERRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TERRIE."]
        #[inline(always)]
        pub const fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("uie", &self.uie())
                .field("cc1ie", &self.cc1ie())
                .field("cc2ie", &self.cc2ie())
                .field("cc3ie", &self.cc3ie())
                .field("cc4ie", &self.cc4ie())
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("cc5ie", &self.cc5ie())
                .field("cc6ie", &self.cc6ie())
                .field("idxie", &self.idxie())
                .field("dirie", &self.dirie())
                .field("ierrie", &self.ierrie())
                .field("terrie", &self.terrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ uie: {=bool:?}, cc1ie: {=bool:?}, cc2ie: {=bool:?}, cc3ie: {=bool:?}, cc4ie: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, cc5ie: {=bool:?}, cc6ie: {=bool:?}, idxie: {=bool:?}, dirie: {=bool:?}, ierrie: {=bool:?}, terrie: {=bool:?} }}",
                self.uie(),
                self.cc1ie(),
                self.cc2ie(),
                self.cc3ie(),
                self.cc4ie(),
                self.comie(),
                self.tie(),
                self.bie(),
                self.cc5ie(),
                self.cc6ie(),
                self.idxie(),
                self.dirie(),
                self.ierrie(),
                self.terrie()
            )
        }
    }
    #[doc = "Interrupt status register."]
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
        #[doc = "desc CC1IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1IF."]
        #[inline(always)]
        pub const fn set_cc1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC2IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2IF."]
        #[inline(always)]
        pub const fn set_cc2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC3IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3IF."]
        #[inline(always)]
        pub const fn set_cc3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC4IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4IF."]
        #[inline(always)]
        pub const fn set_cc4if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COMIF."]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMIF."]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIF."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BIF."]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIF."]
        #[inline(always)]
        pub const fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc B2IF."]
        #[must_use]
        #[inline(always)]
        pub const fn b2if(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc B2IF."]
        #[inline(always)]
        pub const fn set_b2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CC1OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1of(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1OF."]
        #[inline(always)]
        pub const fn set_cc1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CC2OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2of(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2OF."]
        #[inline(always)]
        pub const fn set_cc2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CC3OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3of(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3OF."]
        #[inline(always)]
        pub const fn set_cc3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CC4OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4of(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4OF."]
        #[inline(always)]
        pub const fn set_cc4of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SBIF."]
        #[must_use]
        #[inline(always)]
        pub const fn sbif(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SBIF."]
        #[inline(always)]
        pub const fn set_sbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc CC5IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5if(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5IF."]
        #[inline(always)]
        pub const fn set_cc5if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CC6IF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6if(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6IF."]
        #[inline(always)]
        pub const fn set_cc6if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CC5OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc5of(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC5OF."]
        #[inline(always)]
        pub const fn set_cc5of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CC6OF."]
        #[must_use]
        #[inline(always)]
        pub const fn cc6of(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC6OF."]
        #[inline(always)]
        pub const fn set_cc6of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IDXF."]
        #[must_use]
        #[inline(always)]
        pub const fn idxf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDXF."]
        #[inline(always)]
        pub const fn set_idxf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc DIRF."]
        #[must_use]
        #[inline(always)]
        pub const fn dirf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRF."]
        #[inline(always)]
        pub const fn set_dirf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc IERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn ierrf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc IERRF."]
        #[inline(always)]
        pub const fn set_ierrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn terrf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TERRF."]
        #[inline(always)]
        pub const fn set_terrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("cc1if", &self.cc1if())
                .field("cc2if", &self.cc2if())
                .field("cc3if", &self.cc3if())
                .field("cc4if", &self.cc4if())
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif", &self.bif())
                .field("b2if", &self.b2if())
                .field("cc1of", &self.cc1of())
                .field("cc2of", &self.cc2of())
                .field("cc3of", &self.cc3of())
                .field("cc4of", &self.cc4of())
                .field("sbif", &self.sbif())
                .field("cc5if", &self.cc5if())
                .field("cc6if", &self.cc6if())
                .field("cc5of", &self.cc5of())
                .field("cc6of", &self.cc6of())
                .field("idxf", &self.idxf())
                .field("dirf", &self.dirf())
                .field("ierrf", &self.ierrf())
                .field("terrf", &self.terrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ uif: {=bool:?}, cc1if: {=bool:?}, cc2if: {=bool:?}, cc3if: {=bool:?}, cc4if: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif: {=bool:?}, b2if: {=bool:?}, cc1of: {=bool:?}, cc2of: {=bool:?}, cc3of: {=bool:?}, cc4of: {=bool:?}, sbif: {=bool:?}, cc5if: {=bool:?}, cc6if: {=bool:?}, cc5of: {=bool:?}, cc6of: {=bool:?}, idxf: {=bool:?}, dirf: {=bool:?}, ierrf: {=bool:?}, terrf: {=bool:?} }}",
                self.uif(),
                self.cc1if(),
                self.cc2if(),
                self.cc3if(),
                self.cc4if(),
                self.comif(),
                self.tif(),
                self.bif(),
                self.b2if(),
                self.cc1of(),
                self.cc2of(),
                self.cc3of(),
                self.cc4of(),
                self.sbif(),
                self.cc5if(),
                self.cc6if(),
                self.cc5of(),
                self.cc6of(),
                self.idxf(),
                self.dirf(),
                self.ierrf(),
                self.terrf()
            )
        }
    }
    #[doc = "prescaler Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psc(pub u32);
    impl Psc {
        #[doc = "desc PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PSC."]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Psc {
        #[inline(always)]
        fn default() -> Psc {
            Psc(0)
        }
    }
    impl core::fmt::Debug for Psc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psc").field("psc", &self.psc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Psc {{ psc: {=u16:?} }}", self.psc())
        }
    }
    #[doc = "Repetition Counter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "desc REP."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc REP."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            f.debug_struct("Rcr").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rcr {{ rep: {=u16:?} }}", self.rep())
        }
    }
    #[doc = "Slave Mode Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr(pub u32);
    impl Smcr {
        #[doc = "desc SMS."]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SMS."]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc OCCS."]
        #[must_use]
        #[inline(always)]
        pub const fn occs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OCCS."]
        #[inline(always)]
        pub const fn set_occs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc MSM."]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSM."]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ETF."]
        #[must_use]
        #[inline(always)]
        pub const fn etf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc ETF."]
        #[inline(always)]
        pub const fn set_etf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc ETPS."]
        #[must_use]
        #[inline(always)]
        pub const fn etps(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc ETPS."]
        #[inline(always)]
        pub const fn set_etps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc ECE."]
        #[must_use]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECE."]
        #[inline(always)]
        pub const fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc ETP."]
        #[must_use]
        #[inline(always)]
        pub const fn etp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETP."]
        #[inline(always)]
        pub const fn set_etp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SMSH."]
        #[must_use]
        #[inline(always)]
        pub const fn smsh(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMSH."]
        #[inline(always)]
        pub const fn set_smsh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TSH."]
        #[must_use]
        #[inline(always)]
        pub const fn tsh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc TSH."]
        #[inline(always)]
        pub const fn set_tsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc SMSPE."]
        #[must_use]
        #[inline(always)]
        pub const fn smspe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMSPE."]
        #[inline(always)]
        pub const fn set_smspe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc SMSPS."]
        #[must_use]
        #[inline(always)]
        pub const fn smsps(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMSPS."]
        #[inline(always)]
        pub const fn set_smsps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Smcr {
        #[inline(always)]
        fn default() -> Smcr {
            Smcr(0)
        }
    }
    impl core::fmt::Debug for Smcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr")
                .field("sms", &self.sms())
                .field("occs", &self.occs())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("etf", &self.etf())
                .field("etps", &self.etps())
                .field("ece", &self.ece())
                .field("etp", &self.etp())
                .field("smsh", &self.smsh())
                .field("tsh", &self.tsh())
                .field("smspe", &self.smspe())
                .field("smsps", &self.smsps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smcr {{ sms: {=u8:?}, occs: {=bool:?}, ts: {=u8:?}, msm: {=bool:?}, etf: {=u8:?}, etps: {=u8:?}, ece: {=bool:?}, etp: {=bool:?}, smsh: {=bool:?}, tsh: {=u8:?}, smspe: {=bool:?}, smsps: {=bool:?} }}",
                self.sms(),
                self.occs(),
                self.ts(),
                self.msm(),
                self.etf(),
                self.etps(),
                self.ece(),
                self.etp(),
                self.smsh(),
                self.tsh(),
                self.smspe(),
                self.smsps()
            )
        }
    }
    #[doc = "Timer Input Select Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tisel1(pub u32);
    impl Tisel1 {
        #[doc = "desc TI1SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti1sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI1SEL."]
        #[inline(always)]
        pub const fn set_ti1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc TI2SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti2sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI2SEL."]
        #[inline(always)]
        pub const fn set_ti2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc TI3SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti3sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI3SEL."]
        #[inline(always)]
        pub const fn set_ti3sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc TI4SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti4sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI4SEL."]
        #[inline(always)]
        pub const fn set_ti4sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Tisel1 {
        #[inline(always)]
        fn default() -> Tisel1 {
            Tisel1(0)
        }
    }
    impl core::fmt::Debug for Tisel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tisel1")
                .field("ti1sel", &self.ti1sel())
                .field("ti2sel", &self.ti2sel())
                .field("ti3sel", &self.ti3sel())
                .field("ti4sel", &self.ti4sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tisel1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tisel1 {{ ti1sel: {=u8:?}, ti2sel: {=u8:?}, ti3sel: {=u8:?}, ti4sel: {=u8:?} }}",
                self.ti1sel(),
                self.ti2sel(),
                self.ti3sel(),
                self.ti4sel()
            )
        }
    }
    #[doc = "Timer Input Select Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tisel2(pub u32);
    impl Tisel2 {
        #[doc = "desc TI5SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti5sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI5SEL."]
        #[inline(always)]
        pub const fn set_ti5sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc TI6SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ti6sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TI6SEL."]
        #[inline(always)]
        pub const fn set_ti6sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Tisel2 {
        #[inline(always)]
        fn default() -> Tisel2 {
            Tisel2(0)
        }
    }
    impl core::fmt::Debug for Tisel2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tisel2")
                .field("ti5sel", &self.ti5sel())
                .field("ti6sel", &self.ti6sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tisel2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tisel2 {{ ti5sel: {=u8:?}, ti6sel: {=u8:?} }}",
                self.ti5sel(),
                self.ti6sel()
            )
        }
    }
}
