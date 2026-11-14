#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn swtrgr(self) -> crate::common::Reg<regs::Swtrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DHR12R1."]
    #[inline(always)]
    pub const fn dhr12r1(self) -> crate::common::Reg<regs::Dhr12r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DHR12L1."]
    #[inline(always)]
    pub const fn dhr12l1(self) -> crate::common::Reg<regs::Dhr12l1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DHR8R1."]
    #[inline(always)]
    pub const fn dhr8r1(self) -> crate::common::Reg<regs::Dhr8r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc DHR12R2."]
    #[inline(always)]
    pub const fn dhr12r2(self) -> crate::common::Reg<regs::Dhr12r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc DHR12L2."]
    #[inline(always)]
    pub const fn dhr12l2(self) -> crate::common::Reg<regs::Dhr12l2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc DHR8R2."]
    #[inline(always)]
    pub const fn dhr8r2(self) -> crate::common::Reg<regs::Dhr8r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc DHR12RD."]
    #[inline(always)]
    pub const fn dhr12rd(self) -> crate::common::Reg<regs::Dhr12rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc DHR12LD."]
    #[inline(always)]
    pub const fn dhr12ld(self) -> crate::common::Reg<regs::Dhr12ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc DHR8RD."]
    #[inline(always)]
    pub const fn dhr8rd(self) -> crate::common::Reg<regs::Dhr8rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc DOR1."]
    #[inline(always)]
    pub const fn dor1(self) -> crate::common::Reg<regs::Dor1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc DOR2."]
    #[inline(always)]
    pub const fn dor2(self) -> crate::common::Reg<regs::Dor2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Result register."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc EN1."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN1."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn ten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEN1."]
        #[inline(always)]
        pub const fn set_ten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn tsel1(&self) -> super::vals::Trigger {
            let val = (self.0 >> 2usize) & 0x0f;
            super::vals::Trigger::from_bits(val as u8)
        }
        #[doc = "desc TSEL1."]
        #[inline(always)]
        pub const fn set_tsel1(&mut self, val: super::vals::Trigger) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
        }
        #[doc = "desc WAVE1."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1(&self) -> super::vals::Wave {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Wave::from_bits(val as u8)
        }
        #[doc = "desc WAVE1."]
        #[inline(always)]
        pub const fn set_wave1(&mut self, val: super::vals::Wave) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "desc MAMP1."]
        #[must_use]
        #[inline(always)]
        pub const fn mamp1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MAMP1."]
        #[inline(always)]
        pub const fn set_mamp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc DMAEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN1."]
        #[inline(always)]
        pub const fn set_dmaen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc DMAUDRIE1."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudrie1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAUDRIE1."]
        #[inline(always)]
        pub const fn set_dmaudrie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc EN2."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN2."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn ten2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEN2."]
        #[inline(always)]
        pub const fn set_ten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn tsel2(&self) -> super::vals::Trigger {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Trigger::from_bits(val as u8)
        }
        #[doc = "desc TSEL2."]
        #[inline(always)]
        pub const fn set_tsel2(&mut self, val: super::vals::Trigger) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "desc WAVE2."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2(&self) -> super::vals::Wave {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Wave::from_bits(val as u8)
        }
        #[doc = "desc WAVE2."]
        #[inline(always)]
        pub const fn set_wave2(&mut self, val: super::vals::Wave) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "desc MAMP2."]
        #[must_use]
        #[inline(always)]
        pub const fn mamp2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MAMP2."]
        #[inline(always)]
        pub const fn set_mamp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc DMAEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN2."]
        #[inline(always)]
        pub const fn set_dmaen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc DMAUDRIE2."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudrie2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAUDRIE2."]
        #[inline(always)]
        pub const fn set_dmaudrie2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("en1", &self.en1())
                .field("ten1", &self.ten1())
                .field("tsel1", &self.tsel1())
                .field("wave1", &self.wave1())
                .field("mamp1", &self.mamp1())
                .field("dmaen1", &self.dmaen1())
                .field("dmaudrie1", &self.dmaudrie1())
                .field("en2", &self.en2())
                .field("ten2", &self.ten2())
                .field("tsel2", &self.tsel2())
                .field("wave2", &self.wave2())
                .field("mamp2", &self.mamp2())
                .field("dmaen2", &self.dmaen2())
                .field("dmaudrie2", &self.dmaudrie2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en1: {=bool:?}, ten1: {=bool:?}, tsel1: {:?}, wave1: {:?}, mamp1: {=u8:?}, dmaen1: {=bool:?}, dmaudrie1: {=bool:?}, en2: {=bool:?}, ten2: {=bool:?}, tsel2: {:?}, wave2: {:?}, mamp2: {=u8:?}, dmaen2: {=bool:?}, dmaudrie2: {=bool:?} }}",
                self.en1(),
                self.ten1(),
                self.tsel1(),
                self.wave1(),
                self.mamp1(),
                self.dmaen1(),
                self.dmaudrie1(),
                self.en2(),
                self.ten2(),
                self.tsel2(),
                self.wave2(),
                self.mamp2(),
                self.dmaen2(),
                self.dmaudrie2()
            )
        }
    }
    #[doc = "Result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc C1OUT."]
        #[must_use]
        #[inline(always)]
        pub const fn c1out(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1OUT."]
        #[inline(always)]
        pub const fn set_c1out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc C2OUT."]
        #[must_use]
        #[inline(always)]
        pub const fn c2out(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2OUT."]
        #[inline(always)]
        pub const fn set_c2out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("c1out", &self.c1out())
                .field("c2out", &self.c2out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ c1out: {=bool:?}, c2out: {=bool:?} }}",
                self.c1out(),
                self.c2out()
            )
        }
    }
    #[doc = "desc DHR12L1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12l1(pub u32);
    impl Dhr12l1 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for Dhr12l1 {
        #[inline(always)]
        fn default() -> Dhr12l1 {
            Dhr12l1(0)
        }
    }
    impl core::fmt::Debug for Dhr12l1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12l1")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12l1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12l1 {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "desc DHR12L2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12l2(pub u32);
    impl Dhr12l2 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for Dhr12l2 {
        #[inline(always)]
        fn default() -> Dhr12l2 {
            Dhr12l2(0)
        }
    }
    impl core::fmt::Debug for Dhr12l2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12l2")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12l2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12l2 {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "desc DHR12LD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12ld(pub u32);
    impl Dhr12ld {
        #[doc = "desc C1DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c1data(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc C1DATA."]
        #[inline(always)]
        pub const fn set_c1data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "desc C2DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c2data(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc C2DATA."]
        #[inline(always)]
        pub const fn set_c2data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dhr12ld {
        #[inline(always)]
        fn default() -> Dhr12ld {
            Dhr12ld(0)
        }
    }
    impl core::fmt::Debug for Dhr12ld {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12ld")
                .field("c1data", &self.c1data())
                .field("c2data", &self.c2data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12ld {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr12ld {{ c1data: {=u16:?}, c2data: {=u16:?} }}",
                self.c1data(),
                self.c2data()
            )
        }
    }
    #[doc = "desc DHR12R1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12r1(pub u32);
    impl Dhr12r1 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dhr12r1 {
        #[inline(always)]
        fn default() -> Dhr12r1 {
            Dhr12r1(0)
        }
    }
    impl core::fmt::Debug for Dhr12r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12r1")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12r1 {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "desc DHR12R2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12r2(pub u32);
    impl Dhr12r2 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dhr12r2 {
        #[inline(always)]
        fn default() -> Dhr12r2 {
            Dhr12r2(0)
        }
    }
    impl core::fmt::Debug for Dhr12r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12r2")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr12r2 {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "desc DHR12RD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12rd(pub u32);
    impl Dhr12rd {
        #[doc = "desc C1DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c1data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc C1DATA."]
        #[inline(always)]
        pub const fn set_c1data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc C2DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c2data(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc C2DATA."]
        #[inline(always)]
        pub const fn set_c2data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dhr12rd {
        #[inline(always)]
        fn default() -> Dhr12rd {
            Dhr12rd(0)
        }
    }
    impl core::fmt::Debug for Dhr12rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12rd")
                .field("c1data", &self.c1data())
                .field("c2data", &self.c2data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr12rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr12rd {{ c1data: {=u16:?}, c2data: {=u16:?} }}",
                self.c1data(),
                self.c2data()
            )
        }
    }
    #[doc = "desc DHR8R1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8r1(pub u32);
    impl Dhr8r1 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dhr8r1 {
        #[inline(always)]
        fn default() -> Dhr8r1 {
            Dhr8r1(0)
        }
    }
    impl core::fmt::Debug for Dhr8r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8r1")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr8r1 {{ data: {=u8:?} }}", self.data())
        }
    }
    #[doc = "desc DHR8R2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8r2(pub u32);
    impl Dhr8r2 {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dhr8r2 {
        #[inline(always)]
        fn default() -> Dhr8r2 {
            Dhr8r2(0)
        }
    }
    impl core::fmt::Debug for Dhr8r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8r2")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhr8r2 {{ data: {=u8:?} }}", self.data())
        }
    }
    #[doc = "desc DHR8RD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8rd(pub u32);
    impl Dhr8rd {
        #[doc = "desc C1DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c1data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc C1DATA."]
        #[inline(always)]
        pub const fn set_c1data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc C2DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn c2data(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc C2DATA."]
        #[inline(always)]
        pub const fn set_c2data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Dhr8rd {
        #[inline(always)]
        fn default() -> Dhr8rd {
            Dhr8rd(0)
        }
    }
    impl core::fmt::Debug for Dhr8rd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8rd")
                .field("c1data", &self.c1data())
                .field("c2data", &self.c2data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhr8rd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dhr8rd {{ c1data: {=u8:?}, c2data: {=u8:?} }}",
                self.c1data(),
                self.c2data()
            )
        }
    }
    #[doc = "Output code is read-only; DMAUDR is write-zero-to-clear. Never use read-modify-write to clear flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor1(pub u32);
    impl Dor1 {
        #[doc = "Read-only output code."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Read-only output code."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "DMA underrun: write 0 clears, write 1 leaves unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMA underrun: write 0 clears, write 1 leaves unchanged."]
        #[inline(always)]
        pub const fn set_dmaudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dor1 {
        #[inline(always)]
        fn default() -> Dor1 {
            Dor1(0)
        }
    }
    impl core::fmt::Debug for Dor1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dor1")
                .field("data", &self.data())
                .field("dmaudr", &self.dmaudr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dor1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dor1 {{ data: {=u16:?}, dmaudr: {=bool:?} }}",
                self.data(),
                self.dmaudr()
            )
        }
    }
    #[doc = "Output code is read-only; DMAUDR is write-zero-to-clear. Never use read-modify-write to clear flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor2(pub u32);
    impl Dor2 {
        #[doc = "Read-only output code."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Read-only output code."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "DMA underrun: write 0 clears, write 1 leaves unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMA underrun: write 0 clears, write 1 leaves unchanged."]
        #[inline(always)]
        pub const fn set_dmaudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dor2 {
        #[inline(always)]
        fn default() -> Dor2 {
            Dor2(0)
        }
    }
    impl core::fmt::Debug for Dor2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dor2")
                .field("data", &self.data())
                .field("dmaudr", &self.dmaudr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dor2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dor2 {{ data: {=u16:?}, dmaudr: {=bool:?} }}",
                self.data(),
                self.dmaudr()
            )
        }
    }
    #[doc = "Software trigger command. W1 triggers, W0 no action; bits auto-clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swtrgr(pub u32);
    impl Swtrgr {
        #[doc = "desc SWTRIG1."]
        #[must_use]
        #[inline(always)]
        pub const fn swtrig1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWTRIG1."]
        #[inline(always)]
        pub const fn set_swtrig1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWTRIG2."]
        #[must_use]
        #[inline(always)]
        pub const fn swtrig2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWTRIG2."]
        #[inline(always)]
        pub const fn set_swtrig2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Swtrgr {
        #[inline(always)]
        fn default() -> Swtrgr {
            Swtrgr(0)
        }
    }
    impl core::fmt::Debug for Swtrgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swtrgr")
                .field("swtrig1", &self.swtrig1())
                .field("swtrig2", &self.swtrig2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swtrgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swtrgr {{ swtrig1: {=bool:?}, swtrig2: {=bool:?} }}",
                self.swtrig1(),
                self.swtrig2()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trigger {
        #[doc = "Software trigger."]
        Software = 0x0,
        #[doc = "Btim1Trgo trigger."]
        Btim1trgo = 0x01,
        #[doc = "Btim2Trgo trigger."]
        Btim2trgo = 0x02,
        #[doc = "Btim3Trgo trigger."]
        Btim3trgo = 0x03,
        #[doc = "Gtim1Trgo trigger."]
        Gtim1trgo = 0x04,
        #[doc = "Gtim2Trgo trigger."]
        Gtim2trgo = 0x05,
        #[doc = "Gtim3Trgo trigger."]
        Gtim3trgo = 0x06,
        #[doc = "Gtim4Trgo trigger."]
        Gtim4trgo = 0x07,
        #[doc = "AtimTrgo trigger."]
        AtimTrgo = 0x08,
        #[doc = "LptimOverflow trigger."]
        LptimOverflow = 0x09,
        #[doc = "Btim1Etr trigger."]
        Btim1etr = 0x0a,
        #[doc = "Btim2Etr trigger."]
        Btim2etr = 0x0b,
        #[doc = "Btim3Etr trigger."]
        Btim3etr = 0x0c,
        #[doc = "Gtim1Etr trigger."]
        Gtim1etr = 0x0d,
        #[doc = "Gtim2Etr trigger."]
        Gtim2etr = 0x0e,
        #[doc = "Gtim3Etr trigger."]
        Gtim3etr = 0x0f,
    }
    impl Trigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigger {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trigger {
        #[inline(always)]
        fn from(val: u8) -> Trigger {
            Trigger::from_bits(val)
        }
    }
    impl From<Trigger> for u8 {
        #[inline(always)]
        fn from(val: Trigger) -> u8 {
            Trigger::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wave {
        #[doc = "Data holding register only."]
        Disabled = 0x0,
        #[doc = "Noise generator."]
        Noise = 0x01,
        #[doc = "Triangle generator."]
        Triangle = 0x02,
        #[doc = "Alternative documented triangle encoding."]
        TriangleAlt = 0x03,
    }
    impl Wave {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wave {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wave {
        #[inline(always)]
        fn from(val: u8) -> Wave {
            Wave::from_bits(val)
        }
    }
    impl From<Wave> for u8 {
        #[inline(always)]
        fn from(val: Wave) -> u8 {
            Wave::to_bits(val)
        }
    }
}
impl regs::Dor1 {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(0)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(32768)
    }
}
impl regs::Dor2 {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(0)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(32768)
    }
}
