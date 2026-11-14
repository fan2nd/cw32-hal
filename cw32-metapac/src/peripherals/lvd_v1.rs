#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc LVD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd {
    ptr: *mut u8,
}
unsafe impl Send for Lvd {}
unsafe impl Sync for Lvd {}
impl Lvd {
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
    #[doc = "status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
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
        #[doc = "desc ACTION."]
        #[must_use]
        #[inline(always)]
        pub const fn action(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACTION."]
        #[inline(always)]
        pub const fn set_action(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc VTH."]
        #[must_use]
        #[inline(always)]
        pub const fn vth(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc VTH."]
        #[inline(always)]
        pub const fn set_vth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("action", &self.action())
                .field("source", &self.source())
                .field("vth", &self.vth())
                .field("ie", &self.ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, action: {=bool:?}, source: {=u8:?}, vth: {=u8:?}, ie: {=bool:?} }}",
                self.en(),
                self.action(),
                self.source(),
                self.vth(),
                self.ie()
            )
        }
    }
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc FLTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTEN."]
        #[inline(always)]
        pub const fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn flttime(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTTIME."]
        #[inline(always)]
        pub const fn set_flttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RISE."]
        #[must_use]
        #[inline(always)]
        pub const fn rise(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RISE."]
        #[inline(always)]
        pub const fn set_rise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FALL."]
        #[must_use]
        #[inline(always)]
        pub const fn fall(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FALL."]
        #[inline(always)]
        pub const fn set_fall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc LEVEL."]
        #[must_use]
        #[inline(always)]
        pub const fn level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LEVEL."]
        #[inline(always)]
        pub const fn set_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("flten", &self.flten())
                .field("flttime", &self.flttime())
                .field("fltclk", &self.fltclk())
                .field("rise", &self.rise())
                .field("fall", &self.fall())
                .field("level", &self.level())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ flten: {=bool:?}, flttime: {=u8:?}, fltclk: {=bool:?}, rise: {=bool:?}, fall: {=bool:?}, level: {=bool:?} }}",
                self.flten(),
                self.flttime(),
                self.fltclk(),
                self.rise(),
                self.fall(),
                self.level()
            )
        }
    }
    #[doc = "status register."]
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
