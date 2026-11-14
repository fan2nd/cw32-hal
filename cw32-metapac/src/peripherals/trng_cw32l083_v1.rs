#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CW32L083 TRNG, own User Manual CN Rev2.0; see docs/l083-aes-trng-evidence.md."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
unsafe impl Send for Trng {}
unsafe impl Sync for Trng {}
impl Trng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DATA0."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<regs::Data0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DATA1."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<regs::Data1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
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
        #[doc = "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::Operation {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Operation::from_bits(val as u8)
        }
        #[doc = "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: super::vals::Operation) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
                .field("en", &self.en())
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ en: {=bool:?}, start: {:?} }}",
                self.en(),
                self.start()
            )
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> super::vals::Source {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "desc SHIFT."]
        #[must_use]
        #[inline(always)]
        pub const fn shift(&self) -> super::vals::Shift {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Shift::from_bits(val as u8)
        }
        #[doc = "desc SHIFT."]
        #[inline(always)]
        pub const fn set_shift(&mut self, val: super::vals::Shift) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
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
                .field("source", &self.source())
                .field("shift", &self.shift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ source: {:?}, shift: {:?} }}",
                self.source(),
                self.shift()
            )
        }
    }
    #[doc = "desc DATA0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data0(pub u32);
    impl Data0 {
        #[doc = "Output bits31:0; valid only after a submitted operation completes. No health or error status is documented."]
        #[must_use]
        #[inline(always)]
        pub const fn data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output bits31:0; valid only after a submitted operation completes. No health or error status is documented."]
        #[inline(always)]
        pub const fn set_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data0 {
        #[inline(always)]
        fn default() -> Data0 {
            Data0(0)
        }
    }
    impl core::fmt::Debug for Data0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data0")
                .field("data0", &self.data0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data0 {{ data0: {=u32:?} }}", self.data0())
        }
    }
    #[doc = "desc DATA1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data1(pub u32);
    impl Data1 {
        #[doc = "Output bits63:32; valid only after a submitted operation completes. No health or error status is documented."]
        #[must_use]
        #[inline(always)]
        pub const fn data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output bits63:32; valid only after a submitted operation completes. No health or error status is documented."]
        #[inline(always)]
        pub const fn set_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data1 {
        #[inline(always)]
        fn default() -> Data1 {
            Data1(0)
        }
    }
    impl core::fmt::Debug for Data1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data1")
                .field("data1", &self.data1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data1 {{ data1: {=u32:?} }}", self.data1())
        }
    }
}
pub mod vals {
    #[doc = "START command/status, CW32L083 UM Rev2.0 section 26.6.1. Writing zero is not an abort."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Operation {
        #[doc = "Read: operation complete. Write-zero behavior is unspecified."]
        Idle = 0x0,
        #[doc = "Read: operation busy. Write: start an operation."]
        Active = 0x01,
    }
    impl Operation {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Operation {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Operation {
        #[inline(always)]
        fn from(val: u8) -> Operation {
            Operation::from_bits(val)
        }
    }
    impl From<Operation> for u8 {
        #[inline(always)]
        fn from(val: Operation) -> u8 {
            Operation::to_bits(val)
        }
    }
    #[doc = "LFSR shift count, UM section26.6.2. Other encodings are undocumented."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Shift {
        _RESERVED_0 = 0x0,
        Cycles8 = 0x01,
        Cycles16 = 0x02,
        Cycles32 = 0x03,
        Cycles64 = 0x04,
        Cycles128 = 0x05,
        Cycles256 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Shift {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Shift {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Shift {
        #[inline(always)]
        fn from(val: u8) -> Shift {
            Shift::from_bits(val)
        }
    }
    impl From<Shift> for u8 {
        #[inline(always)]
        fn from(val: Shift) -> u8 {
            Shift::to_bits(val)
        }
    }
    #[doc = "LFSR input, UM section26.6.2. Feedback-only is deterministic and excluded by HAL."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Source {
        Feedback = 0x0,
        Analog = 0x01,
        AnalogXorFeedback = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Source {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Source {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Source {
        #[inline(always)]
        fn from(val: u8) -> Source {
            Source::from_bits(val)
        }
    }
    impl From<Source> for u8 {
        #[inline(always)]
        fn from(val: Source) -> u8 {
            Source::to_bits(val)
        }
    }
}
