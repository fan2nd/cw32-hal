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
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn dr16(self) -> crate::common::Reg<regs::Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn dr32(self) -> crate::common::Reg<regs::Dr32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn dr8(self) -> crate::common::Reg<regs::Dr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Result register."]
    #[inline(always)]
    pub const fn result16(self) -> crate::common::Reg<regs::Result16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Result register."]
    #[inline(always)]
    pub const fn result32(self) -> crate::common::Reg<regs::Result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register."]
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
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr16(pub u16);
    impl Dr16 {
        #[doc = "desc DR16."]
        #[must_use]
        #[inline(always)]
        pub const fn dr16(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DR16."]
        #[inline(always)]
        pub const fn set_dr16(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr16 {
        #[inline(always)]
        fn default() -> Dr16 {
            Dr16(0)
        }
    }
    impl core::fmt::Debug for Dr16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr16").field("dr16", &self.dr16()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr16 {{ dr16: {=u16:?} }}", self.dr16())
        }
    }
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr32(pub u32);
    impl Dr32 {
        #[doc = "desc DR32."]
        #[must_use]
        #[inline(always)]
        pub const fn dr32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DR32."]
        #[inline(always)]
        pub const fn set_dr32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dr32 {
        #[inline(always)]
        fn default() -> Dr32 {
            Dr32(0)
        }
    }
    impl core::fmt::Debug for Dr32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr32").field("dr32", &self.dr32()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr32 {{ dr32: {=u32:?} }}", self.dr32())
        }
    }
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr8(pub u8);
    impl Dr8 {
        #[doc = "desc DR8."]
        #[must_use]
        #[inline(always)]
        pub const fn dr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DR8."]
        #[inline(always)]
        pub const fn set_dr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Dr8 {
        #[inline(always)]
        fn default() -> Dr8 {
            Dr8(0)
        }
    }
    impl core::fmt::Debug for Dr8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr8").field("dr8", &self.dr8()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr8 {{ dr8: {=u8:?} }}", self.dr8())
        }
    }
    #[doc = "Result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result16(pub u16);
    impl Result16 {
        #[doc = "desc RESULT16."]
        #[must_use]
        #[inline(always)]
        pub const fn result16(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RESULT16."]
        #[inline(always)]
        pub const fn set_result16(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Result16 {
        #[inline(always)]
        fn default() -> Result16 {
            Result16(0)
        }
    }
    impl core::fmt::Debug for Result16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result16")
                .field("result16", &self.result16())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Result16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Result16 {{ result16: {=u16:?} }}", self.result16())
        }
    }
    #[doc = "Result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result32(pub u32);
    impl Result32 {
        #[doc = "desc RESULT32."]
        #[must_use]
        #[inline(always)]
        pub const fn result32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc RESULT32."]
        #[inline(always)]
        pub const fn set_result32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Result32 {
        #[inline(always)]
        fn default() -> Result32 {
            Result32(0)
        }
    }
    impl core::fmt::Debug for Result32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result32")
                .field("result32", &self.result32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Result32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Result32 {{ result32: {=u32:?} }}", self.result32())
        }
    }
}
