#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Shared comparator reference divider."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vc2ref {
    ptr: *mut u8,
}
unsafe impl Send for Vc2ref {}
unsafe impl Sync for Vc2ref {}
impl Vc2ref {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "VCREF Control register."]
    #[inline(always)]
    pub const fn ref_(self) -> crate::common::Reg<regs::Ref, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Shared comparator reference control; configure only while no consumer selects it."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ref(pub u32);
    impl Ref {
        #[doc = "Ratio code 0..7 selects (1 + DIV) / 8. Values 8..15 are unqualified on L012."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> super::vals::DividerRatio {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::DividerRatio::from_bits(val as u8)
        }
        #[doc = "Ratio code 0..7 selects (1 + DIV) / 8. Values 8..15 are unqualified on L012."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: super::vals::DividerRatio) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Enable the shared resistor divider."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the shared resistor divider."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog supply (VDD on L010, VDDA on L011/L012) or nominal 1.6 V core rail."]
        #[must_use]
        #[inline(always)]
        pub const fn vin(&self) -> super::vals::InputSource {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::InputSource::from_bits(val as u8)
        }
        #[doc = "Analog supply (VDD on L010, VDDA on L011/L012) or nominal 1.6 V core rail."]
        #[inline(always)]
        pub const fn set_vin(&mut self, val: super::vals::InputSource) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ref {
        #[inline(always)]
        fn default() -> Ref {
            Ref(0)
        }
    }
    impl core::fmt::Debug for Ref {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ref")
                .field("div", &self.div())
                .field("en", &self.en())
                .field("vin", &self.vin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ref {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ref {{ div: {:?}, en: {=bool:?}, vin: {:?} }}",
                self.div(),
                self.en(),
                self.vin()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DividerRatio {
        #[doc = "1/8 of the selected input; subject to analog limits."]
        OneEighth = 0x0,
        #[doc = "2/8 of the selected input; subject to analog limits."]
        TwoEighths = 0x01,
        #[doc = "3/8 of the selected input; subject to analog limits."]
        ThreeEighths = 0x02,
        #[doc = "4/8 of the selected input; subject to analog limits."]
        FourEighths = 0x03,
        #[doc = "5/8 of the selected input; subject to analog limits."]
        FiveEighths = 0x04,
        #[doc = "6/8 of the selected input; subject to analog limits."]
        SixEighths = 0x05,
        #[doc = "7/8 of the selected input; subject to analog limits."]
        SevenEighths = 0x06,
        #[doc = "8/8 of the selected input; subject to analog limits."]
        EightEighths = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl DividerRatio {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DividerRatio {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DividerRatio {
        #[inline(always)]
        fn from(val: u8) -> DividerRatio {
            DividerRatio::from_bits(val)
        }
    }
    impl From<DividerRatio> for u8 {
        #[inline(always)]
        fn from(val: DividerRatio) -> u8 {
            DividerRatio::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum InputSource {
        #[doc = "VDD on L010; VDDA on L011/L012."]
        Supply = 0x0,
        #[doc = "Internal Vcore, nominally 1.6 V; not a calibrated reference."]
        Core = 0x01,
    }
    impl InputSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> InputSource {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for InputSource {
        #[inline(always)]
        fn from(val: u8) -> InputSource {
            InputSource::from_bits(val)
        }
    }
    impl From<InputSource> for u8 {
        #[inline(always)]
        fn from(val: InputSource) -> u8 {
            InputSource::to_bits(val)
        }
    }
}
