#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EAU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eau {
    ptr: *mut u8,
}
unsafe impl Send for Eau {}
unsafe impl Sync for Eau {}
impl Eau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CSR."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DIVIDEND."]
    #[inline(always)]
    pub const fn dividend(self) -> crate::common::Reg<regs::Dividend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DIVISOR."]
    #[inline(always)]
    pub const fn divisor(self) -> crate::common::Reg<regs::Divisor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc QUOTIENT."]
    #[inline(always)]
    pub const fn quotient(self) -> crate::common::Reg<regs::Quotient, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc REMAINDER."]
    #[inline(always)]
    pub const fn remainder(self) -> crate::common::Reg<regs::Remainder, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "desc OVR."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVR."]
        #[inline(always)]
        pub const fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ZERO."]
        #[must_use]
        #[inline(always)]
        pub const fn zero(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ZERO."]
        #[inline(always)]
        pub const fn set_zero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("mode", &self.mode())
                .field("ovr", &self.ovr())
                .field("zero", &self.zero())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ mode: {:?}, ovr: {=bool:?}, zero: {=bool:?}, busy: {=bool:?} }}",
                self.mode(),
                self.ovr(),
                self.zero(),
                self.busy()
            )
        }
    }
    #[doc = "desc DIVIDEND."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dividend(pub u32);
    impl Dividend {
        #[doc = "desc DIVIDEND."]
        #[must_use]
        #[inline(always)]
        pub const fn dividend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DIVIDEND."]
        #[inline(always)]
        pub const fn set_dividend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dividend {
        #[inline(always)]
        fn default() -> Dividend {
            Dividend(0)
        }
    }
    impl core::fmt::Debug for Dividend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dividend")
                .field("dividend", &self.dividend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dividend {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dividend {{ dividend: {=u32:?} }}", self.dividend())
        }
    }
    #[doc = "desc DIVISOR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divisor(pub u32);
    impl Divisor {
        #[doc = "desc DIVISOR."]
        #[must_use]
        #[inline(always)]
        pub const fn divisor(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DIVISOR."]
        #[inline(always)]
        pub const fn set_divisor(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Divisor {
        #[inline(always)]
        fn default() -> Divisor {
            Divisor(0)
        }
    }
    impl core::fmt::Debug for Divisor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divisor")
                .field("divisor", &self.divisor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Divisor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Divisor {{ divisor: {=u32:?} }}", self.divisor())
        }
    }
    #[doc = "desc QUOTIENT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Quotient(pub u32);
    impl Quotient {
        #[doc = "desc QUOTIENT."]
        #[must_use]
        #[inline(always)]
        pub const fn quotient(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc QUOTIENT."]
        #[inline(always)]
        pub const fn set_quotient(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Quotient {
        #[inline(always)]
        fn default() -> Quotient {
            Quotient(0)
        }
    }
    impl core::fmt::Debug for Quotient {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Quotient")
                .field("quotient", &self.quotient())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Quotient {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Quotient {{ quotient: {=u32:?} }}", self.quotient())
        }
    }
    #[doc = "desc REMAINDER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Remainder(pub u32);
    impl Remainder {
        #[doc = "desc REMAINDER."]
        #[must_use]
        #[inline(always)]
        pub const fn remainder(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc REMAINDER."]
        #[inline(always)]
        pub const fn set_remainder(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Remainder {
        #[inline(always)]
        fn default() -> Remainder {
            Remainder(0)
        }
    }
    impl core::fmt::Debug for Remainder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Remainder")
                .field("remainder", &self.remainder())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Remainder {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Remainder {{ remainder: {=u32:?} }}", self.remainder())
        }
    }
}
pub mod vals {
    #[doc = "L012 EAU Mode encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        UnsignedDivision = 0x0,
        SignedDivision = 0x01,
        SquareRoot = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
