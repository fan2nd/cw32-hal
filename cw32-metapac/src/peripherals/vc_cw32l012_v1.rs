#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc VC1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vc {
    ptr: *mut u8,
}
unsafe impl Send for Vc {}
unsafe impl Sync for Vc {}
impl Vc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
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
        #[doc = "desc RESP."]
        #[must_use]
        #[inline(always)]
        pub const fn resp(&self) -> super::vals::ResponseSpeed {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::ResponseSpeed::from_bits(val as u8)
        }
        #[doc = "desc RESP."]
        #[inline(always)]
        pub const fn set_resp(&mut self, val: super::vals::ResponseSpeed) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HYS."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> super::vals::Hysteresis {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Hysteresis::from_bits(val as u8)
        }
        #[doc = "desc HYS."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: super::vals::Hysteresis) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POL."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> super::vals::Polarity {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "desc POL."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "desc WINDOW."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDOW."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc INN."]
        #[must_use]
        #[inline(always)]
        pub const fn inn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc INN."]
        #[inline(always)]
        pub const fn set_inn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc INP."]
        #[must_use]
        #[inline(always)]
        pub const fn inp(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc INP."]
        #[inline(always)]
        pub const fn set_inp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
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
                .field("en", &self.en())
                .field("resp", &self.resp())
                .field("hys", &self.hys())
                .field("ie", &self.ie())
                .field("pol", &self.pol())
                .field("window", &self.window())
                .field("inn", &self.inn())
                .field("inp", &self.inp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, resp: {:?}, hys: {:?}, ie: {=bool:?}, pol: {:?}, window: {=bool:?}, inn: {=u8:?}, inp: {=u8:?} }}",
                self.en(),
                self.resp(),
                self.hys(),
                self.ie(),
                self.pol(),
                self.window(),
                self.inn(),
                self.inp()
            )
        }
    }
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc FLTTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn flttime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTTIME."]
        #[inline(always)]
        pub const fn set_flttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> super::vals::FilterClock {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::FilterClock::from_bits(val as u8)
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: super::vals::FilterClock) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FALLIE."]
        #[must_use]
        #[inline(always)]
        pub const fn fallie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FALLIE."]
        #[inline(always)]
        pub const fn set_fallie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RISEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn riseie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RISEIE."]
        #[inline(always)]
        pub const fn set_riseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HIGHIE."]
        #[must_use]
        #[inline(always)]
        pub const fn highie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HIGHIE."]
        #[inline(always)]
        pub const fn set_highie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BLANKTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn blanktime(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc BLANKTIME."]
        #[inline(always)]
        pub const fn set_blanktime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc BLANKLVL."]
        #[must_use]
        #[inline(always)]
        pub const fn blanklvl(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLANKLVL."]
        #[inline(always)]
        pub const fn set_blanklvl(&mut self, val: bool) {
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
                .field("flttime", &self.flttime())
                .field("fltclk", &self.fltclk())
                .field("fallie", &self.fallie())
                .field("riseie", &self.riseie())
                .field("highie", &self.highie())
                .field("blanktime", &self.blanktime())
                .field("blanklvl", &self.blanklvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ flttime: {=u8:?}, fltclk: {:?}, fallie: {=bool:?}, riseie: {=bool:?}, highie: {=bool:?}, blanktime: {=u8:?}, blanklvl: {=bool:?} }}",
                self.flttime(),
                self.fltclk(),
                self.fallie(),
                self.riseie(),
                self.highie(),
                self.blanktime(),
                self.blanklvl()
            )
        }
    }
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc ATIMTRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn atimtrgo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMTRGO."]
        #[inline(always)]
        pub const fn set_atimtrgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ATIMTRGO2."]
        #[must_use]
        #[inline(always)]
        pub const fn atimtrgo2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMTRGO2."]
        #[inline(always)]
        pub const fn set_atimtrgo2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ATIMOC1REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc1refc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC1REFC."]
        #[inline(always)]
        pub const fn set_atimoc1refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ATIMOC2REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc2refc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC2REFC."]
        #[inline(always)]
        pub const fn set_atimoc2refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ATIMOC3REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc3refc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC3REFC."]
        #[inline(always)]
        pub const fn set_atimoc3refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIMOC4REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc4refc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC4REFC."]
        #[inline(always)]
        pub const fn set_atimoc4refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ATIMOC5REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc5refc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC5REFC."]
        #[inline(always)]
        pub const fn set_atimoc5refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ATIMOC6REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn atimoc6refc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMOC6REFC."]
        #[inline(always)]
        pub const fn set_atimoc6refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc GTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1trgo(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1TRGO."]
        #[inline(always)]
        pub const fn set_gtim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc GTIM1OC1REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc1refc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC1REFC."]
        #[inline(always)]
        pub const fn set_gtim1oc1refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc GTIM1OC2REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc2refc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC2REFC."]
        #[inline(always)]
        pub const fn set_gtim1oc2refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc GTIM1OC3REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc3refc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC3REFC."]
        #[inline(always)]
        pub const fn set_gtim1oc3refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc GTIM1OC4REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc4refc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC4REFC."]
        #[inline(always)]
        pub const fn set_gtim1oc4refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc GTIM2TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2trgo(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2TRGO."]
        #[inline(always)]
        pub const fn set_gtim2trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc GTIM2OC1REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc1refc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC1REFC."]
        #[inline(always)]
        pub const fn set_gtim2oc1refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc GTIM2OC2REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc2refc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC2REFC."]
        #[inline(always)]
        pub const fn set_gtim2oc2refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc GTIM2OC3REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc3refc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC3REFC."]
        #[inline(always)]
        pub const fn set_gtim2oc3refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc GTIM2OC4REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc4refc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC4REFC."]
        #[inline(always)]
        pub const fn set_gtim2oc4refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc GTIM3TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3trgo(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3TRGO."]
        #[inline(always)]
        pub const fn set_gtim3trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc GTIM3OC1REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc1refc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC1REFC."]
        #[inline(always)]
        pub const fn set_gtim3oc1refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc GTIM3OC2REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc2refc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC2REFC."]
        #[inline(always)]
        pub const fn set_gtim3oc2refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc GTIM3OC3REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc3refc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC3REFC."]
        #[inline(always)]
        pub const fn set_gtim3oc3refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc GTIM3OC4REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc4refc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC4REFC."]
        #[inline(always)]
        pub const fn set_gtim3oc4refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc GTIM4TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4trgo(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4TRGO."]
        #[inline(always)]
        pub const fn set_gtim4trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc GTIM4OC1REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc1refc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC1REFC."]
        #[inline(always)]
        pub const fn set_gtim4oc1refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc GTIM4OC2REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc2refc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC2REFC."]
        #[inline(always)]
        pub const fn set_gtim4oc2refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc GTIM4OC3REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc3refc(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC3REFC."]
        #[inline(always)]
        pub const fn set_gtim4oc3refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc GTIM4OC4REFC."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc4refc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC4REFC."]
        #[inline(always)]
        pub const fn set_gtim4oc4refc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc BTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim1trgo(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM1TRGO."]
        #[inline(always)]
        pub const fn set_btim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc BTIM2TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim2trgo(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM2TRGO."]
        #[inline(always)]
        pub const fn set_btim2trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc BTIM3TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim3trgo(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM3TRGO."]
        #[inline(always)]
        pub const fn set_btim3trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc HALLTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn halltimtrgo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc HALLTIM1TRGO."]
        #[inline(always)]
        pub const fn set_halltimtrgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("atimtrgo", &self.atimtrgo())
                .field("atimtrgo2", &self.atimtrgo2())
                .field("atimoc1refc", &self.atimoc1refc())
                .field("atimoc2refc", &self.atimoc2refc())
                .field("atimoc3refc", &self.atimoc3refc())
                .field("atimoc4refc", &self.atimoc4refc())
                .field("atimoc5refc", &self.atimoc5refc())
                .field("atimoc6refc", &self.atimoc6refc())
                .field("gtim1trgo", &self.gtim1trgo())
                .field("gtim1oc1refc", &self.gtim1oc1refc())
                .field("gtim1oc2refc", &self.gtim1oc2refc())
                .field("gtim1oc3refc", &self.gtim1oc3refc())
                .field("gtim1oc4refc", &self.gtim1oc4refc())
                .field("gtim2trgo", &self.gtim2trgo())
                .field("gtim2oc1refc", &self.gtim2oc1refc())
                .field("gtim2oc2refc", &self.gtim2oc2refc())
                .field("gtim2oc3refc", &self.gtim2oc3refc())
                .field("gtim2oc4refc", &self.gtim2oc4refc())
                .field("gtim3trgo", &self.gtim3trgo())
                .field("gtim3oc1refc", &self.gtim3oc1refc())
                .field("gtim3oc2refc", &self.gtim3oc2refc())
                .field("gtim3oc3refc", &self.gtim3oc3refc())
                .field("gtim3oc4refc", &self.gtim3oc4refc())
                .field("gtim4trgo", &self.gtim4trgo())
                .field("gtim4oc1refc", &self.gtim4oc1refc())
                .field("gtim4oc2refc", &self.gtim4oc2refc())
                .field("gtim4oc3refc", &self.gtim4oc3refc())
                .field("gtim4oc4refc", &self.gtim4oc4refc())
                .field("btim1trgo", &self.btim1trgo())
                .field("btim2trgo", &self.btim2trgo())
                .field("btim3trgo", &self.btim3trgo())
                .field("halltimtrgo", &self.halltimtrgo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ atimtrgo: {=bool:?}, atimtrgo2: {=bool:?}, atimoc1refc: {=bool:?}, atimoc2refc: {=bool:?}, atimoc3refc: {=bool:?}, atimoc4refc: {=bool:?}, atimoc5refc: {=bool:?}, atimoc6refc: {=bool:?}, gtim1trgo: {=bool:?}, gtim1oc1refc: {=bool:?}, gtim1oc2refc: {=bool:?}, gtim1oc3refc: {=bool:?}, gtim1oc4refc: {=bool:?}, gtim2trgo: {=bool:?}, gtim2oc1refc: {=bool:?}, gtim2oc2refc: {=bool:?}, gtim2oc3refc: {=bool:?}, gtim2oc4refc: {=bool:?}, gtim3trgo: {=bool:?}, gtim3oc1refc: {=bool:?}, gtim3oc2refc: {=bool:?}, gtim3oc3refc: {=bool:?}, gtim3oc4refc: {=bool:?}, gtim4trgo: {=bool:?}, gtim4oc1refc: {=bool:?}, gtim4oc2refc: {=bool:?}, gtim4oc3refc: {=bool:?}, gtim4oc4refc: {=bool:?}, btim1trgo: {=bool:?}, btim2trgo: {=bool:?}, btim3trgo: {=bool:?}, halltimtrgo: {=bool:?} }}",
                self.atimtrgo(),
                self.atimtrgo2(),
                self.atimoc1refc(),
                self.atimoc2refc(),
                self.atimoc3refc(),
                self.atimoc4refc(),
                self.atimoc5refc(),
                self.atimoc6refc(),
                self.gtim1trgo(),
                self.gtim1oc1refc(),
                self.gtim1oc2refc(),
                self.gtim1oc3refc(),
                self.gtim1oc4refc(),
                self.gtim2trgo(),
                self.gtim2oc1refc(),
                self.gtim2oc2refc(),
                self.gtim2oc3refc(),
                self.gtim2oc4refc(),
                self.gtim3trgo(),
                self.gtim3oc1refc(),
                self.gtim3oc2refc(),
                self.gtim3oc3refc(),
                self.gtim3oc4refc(),
                self.gtim4trgo(),
                self.gtim4oc1refc(),
                self.gtim4oc2refc(),
                self.gtim4oc3refc(),
                self.gtim4oc4refc(),
                self.btim1trgo(),
                self.btim2trgo(),
                self.btim3trgo(),
                self.halltimtrgo()
            )
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTV."]
        #[must_use]
        #[inline(always)]
        pub const fn fltv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTV."]
        #[inline(always)]
        pub const fn set_fltv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("intf", &self.intf())
                .field("fltv", &self.fltv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ intf: {=bool:?}, fltv: {=bool:?} }}",
                self.intf(),
                self.fltv()
            )
        }
    }
}
pub mod vals {
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FilterClock {
        #[doc = "Internal RC filter clock; see the selected own-manual source."]
        InternalRc = 0x0,
        #[doc = "Peripheral bus clock."]
        Pclk = 0x01,
    }
    impl FilterClock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterClock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterClock {
        #[inline(always)]
        fn from(val: u8) -> FilterClock {
            FilterClock::from_bits(val)
        }
    }
    impl From<FilterClock> for u8 {
        #[inline(always)]
        fn from(val: FilterClock) -> u8 {
            FilterClock::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hysteresis {
        #[doc = "Hysteresis disabled."]
        None = 0x0,
        #[doc = "Hysteresis enabled; own datasheet typical value only."]
        Enabled = 0x01,
    }
    impl Hysteresis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hysteresis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hysteresis {
        #[inline(always)]
        fn from(val: u8) -> Hysteresis {
            Hysteresis::from_bits(val)
        }
    }
    impl From<Hysteresis> for u8 {
        #[inline(always)]
        fn from(val: Hysteresis) -> u8 {
            Hysteresis::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polarity {
        #[doc = "Non-inverted output."]
        Normal = 0x0,
        #[doc = "Inverted output."]
        Inverted = 0x01,
    }
    impl Polarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polarity {
        #[inline(always)]
        fn from(val: u8) -> Polarity {
            Polarity::from_bits(val)
        }
    }
    impl From<Polarity> for u8 {
        #[inline(always)]
        fn from(val: Polarity) -> u8 {
            Polarity::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ResponseSpeed {
        #[doc = "Low speed and power."]
        Low = 0x0,
        #[doc = "High speed and power."]
        High = 0x01,
    }
    impl ResponseSpeed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ResponseSpeed {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ResponseSpeed {
        #[inline(always)]
        fn from(val: u8) -> ResponseSpeed {
            ResponseSpeed::from_bits(val)
        }
    }
    impl From<ResponseSpeed> for u8 {
        #[inline(always)]
        fn from(val: ResponseSpeed) -> u8 {
            ResponseSpeed::to_bits(val)
        }
    }
}
impl regs::Sr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(0)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(1)
    }
}
