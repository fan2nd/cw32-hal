#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DR."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RESULT."]
    #[inline(always)]
    pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SDK-declared initial value register; load semantics and write ordering are undocumented in the L052 V1.5 manual."]
    #[inline(always)]
    pub const fn init(self) -> crate::common::Reg<regs::Init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            f.debug_struct("Cr").field("mode", &self.mode()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ mode: {=u8:?} }}", self.mode())
        }
    }
    #[doc = "desc DR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "desc DR."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DR."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ dr: {=u8:?} }}", self.dr())
        }
    }
    #[doc = "SDK-declared initial value field; custom seed protocol is not qualified."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init(pub u32);
    impl Init {
        #[doc = "SDK-declared initial value register; load semantics and write ordering are undocumented in the L052 V1.5 manual."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "SDK-declared initial value register; load semantics and write ordering are undocumented in the L052 V1.5 manual."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Init {
        #[inline(always)]
        fn default() -> Init {
            Init(0)
        }
    }
    impl core::fmt::Debug for Init {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init").field("init", &self.init()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Init {{ init: {=u16:?} }}", self.init())
        }
    }
    #[doc = "desc RESULT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result(pub u32);
    impl Result {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Result {
        #[inline(always)]
        fn default() -> Result {
            Result(0)
        }
    }
    impl core::fmt::Debug for Result {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result")
                .field("result", &self.result())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Result {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Result {{ result: {=u16:?} }}", self.result())
        }
    }
}
