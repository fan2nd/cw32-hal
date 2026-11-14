#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CORDIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cordic {
    ptr: *mut u8,
}
unsafe impl Send for Cordic {}
unsafe impl Sync for Cordic {}
impl Cordic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and State register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register X."]
    #[inline(always)]
    pub const fn x(self) -> crate::common::Reg<regs::X, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data register Y."]
    #[inline(always)]
    pub const fn y(self) -> crate::common::Reg<regs::Y, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc Z."]
    #[inline(always)]
    pub const fn z(self) -> crate::common::Reg<regs::Z, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control and State register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "desc FUNC."]
        #[must_use]
        #[inline(always)]
        pub const fn func(&self) -> super::vals::Func {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Func::from_bits(val as u8)
        }
        #[doc = "desc FUNC."]
        #[inline(always)]
        pub const fn set_func(&mut self, val: super::vals::Func) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SCALE."]
        #[must_use]
        #[inline(always)]
        pub const fn scale(&self) -> super::vals::Scale {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Scale::from_bits(val as u8)
        }
        #[doc = "desc SCALE."]
        #[inline(always)]
        pub const fn set_scale(&mut self, val: super::vals::Scale) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "desc FORMAT."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> super::vals::Format {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Format::from_bits(val as u8)
        }
        #[doc = "desc FORMAT."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: super::vals::Format) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ITER."]
        #[must_use]
        #[inline(always)]
        pub const fn iter(&self) -> super::vals::Iter {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Iter::from_bits(val as u8)
        }
        #[doc = "desc ITER."]
        #[inline(always)]
        pub const fn set_iter(&mut self, val: super::vals::Iter) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Comp {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Comp::from_bits(val as u8)
        }
        #[doc = "desc COMP."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Comp) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc DMAEOC."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaeoc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEOC."]
        #[inline(always)]
        pub const fn set_dmaeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DMAIDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaidle(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAIDLE."]
        #[inline(always)]
        pub const fn set_dmaidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EOC."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOC."]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("func", &self.func())
                .field("scale", &self.scale())
                .field("format", &self.format())
                .field("iter", &self.iter())
                .field("comp", &self.comp())
                .field("ie", &self.ie())
                .field("dmaeoc", &self.dmaeoc())
                .field("dmaidle", &self.dmaidle())
                .field("eoc", &self.eoc())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ func: {:?}, scale: {:?}, format: {:?}, iter: {:?}, comp: {:?}, ie: {=bool:?}, dmaeoc: {=bool:?}, dmaidle: {=bool:?}, eoc: {=bool:?}, busy: {=bool:?} }}",
                self.func(),
                self.scale(),
                self.format(),
                self.iter(),
                self.comp(),
                self.ie(),
                self.dmaeoc(),
                self.dmaidle(),
                self.eoc(),
                self.busy()
            )
        }
    }
    #[doc = "Data register X."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct X(pub u32);
    impl X {
        #[doc = "desc X."]
        #[must_use]
        #[inline(always)]
        pub const fn x(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc X."]
        #[inline(always)]
        pub const fn set_x(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for X {
        #[inline(always)]
        fn default() -> X {
            X(0)
        }
    }
    impl core::fmt::Debug for X {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("X").field("x", &self.x()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for X {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "X {{ x: {=u32:?} }}", self.x())
        }
    }
    #[doc = "Data register Y."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Y(pub u32);
    impl Y {
        #[doc = "desc Y."]
        #[must_use]
        #[inline(always)]
        pub const fn y(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc Y."]
        #[inline(always)]
        pub const fn set_y(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Y {
        #[inline(always)]
        fn default() -> Y {
            Y(0)
        }
    }
    impl core::fmt::Debug for Y {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Y").field("y", &self.y()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Y {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Y {{ y: {=u32:?} }}", self.y())
        }
    }
    #[doc = "desc Z."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Z(pub u32);
    impl Z {
        #[doc = "desc Z."]
        #[must_use]
        #[inline(always)]
        pub const fn z(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc Z."]
        #[inline(always)]
        pub const fn set_z(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Z {
        #[inline(always)]
        fn default() -> Z {
            Z(0)
        }
    }
    impl core::fmt::Debug for Z {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Z").field("z", &self.z()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Z {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Z {{ z: {=u32:?} }}", self.z())
        }
    }
}
pub mod vals {
    #[doc = "L012 CORDIC Comp encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Comp {
        Compensated = 0x0,
        Uncompensated = 0x01,
    }
    impl Comp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comp {
        #[inline(always)]
        fn from(val: u8) -> Comp {
            Comp::from_bits(val)
        }
    }
    impl From<Comp> for u8 {
        #[inline(always)]
        fn from(val: Comp) -> u8 {
            Comp::to_bits(val)
        }
    }
    #[doc = "L012 CORDIC Format encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Format {
        Q115 = 0x0,
        Q131 = 0x01,
    }
    impl Format {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Format {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Format {
        #[inline(always)]
        fn from(val: u8) -> Format {
            Format::from_bits(val)
        }
    }
    impl From<Format> for u8 {
        #[inline(always)]
        fn from(val: Format) -> u8 {
            Format::to_bits(val)
        }
    }
    #[doc = "L012 CORDIC Func encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Func {
        Cos = 0x0,
        Sin = 0x01,
        Atan2 = 0x02,
        Hypot = 0x03,
        Atan = 0x04,
        Cosh = 0x05,
        Sinh = 0x06,
        Atanh = 0x07,
        Ln = 0x08,
        Sqrt = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Func {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Func {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Func {
        #[inline(always)]
        fn from(val: u8) -> Func {
            Func::from_bits(val)
        }
    }
    impl From<Func> for u8 {
        #[inline(always)]
        fn from(val: Func) -> u8 {
            Func::to_bits(val)
        }
    }
    #[doc = "L012 CORDIC Iter encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iter {
        Iter2 = 0x0,
        Iter4 = 0x01,
        Iter6 = 0x02,
        Iter8 = 0x03,
        Iter10 = 0x04,
        Iter12 = 0x05,
        Iter14 = 0x06,
        Iter16 = 0x07,
        Iter18 = 0x08,
        Iter20 = 0x09,
        Iter22 = 0x0a,
        Iter24 = 0x0b,
        Iter26 = 0x0c,
        Iter28 = 0x0d,
        Iter30 = 0x0e,
        Iter32 = 0x0f,
    }
    impl Iter {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iter {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iter {
        #[inline(always)]
        fn from(val: u8) -> Iter {
            Iter::from_bits(val)
        }
    }
    impl From<Iter> for u8 {
        #[inline(always)]
        fn from(val: Iter) -> u8 {
            Iter::to_bits(val)
        }
    }
    #[doc = "L012 CORDIC Scale encodings, own CN manual Rev1.4; see docs/l012-math-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scale {
        Scale0 = 0x0,
        Scale1 = 0x01,
        Scale2 = 0x02,
        Scale3 = 0x03,
        Scale4 = 0x04,
        Scale5 = 0x05,
        Scale6 = 0x06,
        Scale7 = 0x07,
    }
    impl Scale {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scale {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scale {
        #[inline(always)]
        fn from(val: u8) -> Scale {
            Scale::from_bits(val)
        }
    }
    impl From<Scale> for u8 {
        #[inline(always)]
        fn from(val: Scale) -> u8 {
            Scale::to_bits(val)
        }
    }
}
