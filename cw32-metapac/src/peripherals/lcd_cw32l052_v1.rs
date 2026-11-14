#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc LCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcd {
    ptr: *mut u8,
}
unsafe impl Send for Lcd {}
unsafe impl Sync for Lcd {}
impl Lcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PINEN1."]
    #[inline(always)]
    pub const fn pinen1(self) -> crate::common::Reg<regs::Pinen1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PINEN2."]
    #[inline(always)]
    pub const fn pinen2(self) -> crate::common::Reg<regs::Pinen2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RAM0."]
    #[inline(always)]
    pub const fn ram0(self) -> crate::common::Reg<regs::Ram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc RAM1."]
    #[inline(always)]
    pub const fn ram1(self) -> crate::common::Reg<regs::Ram1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc RAM2."]
    #[inline(always)]
    pub const fn ram2(self) -> crate::common::Reg<regs::Ram2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc RAM3."]
    #[inline(always)]
    pub const fn ram3(self) -> crate::common::Reg<regs::Ram3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc RAM4."]
    #[inline(always)]
    pub const fn ram4(self) -> crate::common::Reg<regs::Ram4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc RAM5."]
    #[inline(always)]
    pub const fn ram5(self) -> crate::common::Reg<regs::Ram5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc RAM6."]
    #[inline(always)]
    pub const fn ram6(self) -> crate::common::Reg<regs::Ram6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc RAM7."]
    #[inline(always)]
    pub const fn ram7(self) -> crate::common::Reg<regs::Ram7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc RAM8."]
    #[inline(always)]
    pub const fn ram8(self) -> crate::common::Reg<regs::Ram8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc RAM13."]
    #[inline(always)]
    pub const fn ram13(self) -> crate::common::Reg<regs::Ram13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR0."]
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
        #[doc = "desc CPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn cpclk(&self) -> super::vals::PumpFrequency {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::PumpFrequency::from_bits(val as u8)
        }
        #[doc = "desc CPCLK."]
        #[inline(always)]
        pub const fn set_cpclk(&mut self, val: super::vals::PumpFrequency) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "desc BUMP."]
        #[must_use]
        #[inline(always)]
        pub const fn bump(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUMP."]
        #[inline(always)]
        pub const fn set_bump(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> super::vals::Bias {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Bias::from_bits(val as u8)
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: super::vals::Bias) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DUTY."]
        #[must_use]
        #[inline(always)]
        pub const fn duty(&self) -> super::vals::Duty {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Duty::from_bits(val as u8)
        }
        #[doc = "desc DUTY."]
        #[inline(always)]
        pub const fn set_duty(&mut self, val: super::vals::Duty) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "desc INRS."]
        #[must_use]
        #[inline(always)]
        pub const fn inrs(&self) -> super::vals::BiasSource {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::BiasSource::from_bits(val as u8)
        }
        #[doc = "desc INRS."]
        #[inline(always)]
        pub const fn set_inrs(&mut self, val: super::vals::BiasSource) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "desc CONTRAST."]
        #[must_use]
        #[inline(always)]
        pub const fn contrast(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CONTRAST."]
        #[inline(always)]
        pub const fn set_contrast(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
                .field("cpclk", &self.cpclk())
                .field("bump", &self.bump())
                .field("bias", &self.bias())
                .field("duty", &self.duty())
                .field("inrs", &self.inrs())
                .field("contrast", &self.contrast())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, cpclk: {:?}, bump: {=bool:?}, bias: {:?}, duty: {:?}, inrs: {:?}, contrast: {=u8:?} }}",
                self.en(),
                self.cpclk(),
                self.bump(),
                self.bias(),
                self.duty(),
                self.inrs(),
                self.contrast()
            )
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc BLINKCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn blinkcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc BLINKCNT."]
        #[inline(always)]
        pub const fn set_blinkcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc BLINKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn blinken(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLINKEN."]
        #[inline(always)]
        pub const fn set_blinken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CLKCS."]
        #[must_use]
        #[inline(always)]
        pub const fn clkcs(&self) -> super::vals::ClockSource {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::ClockSource::from_bits(val as u8)
        }
        #[doc = "desc CLKCS."]
        #[inline(always)]
        pub const fn set_clkcs(&mut self, val: super::vals::ClockSource) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "desc LCDFS."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdfs(&self) -> super::vals::ScanFrequency {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ScanFrequency::from_bits(val as u8)
        }
        #[doc = "desc LCDFS."]
        #[inline(always)]
        pub const fn set_lcdfs(&mut self, val: super::vals::ScanFrequency) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "desc DMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("blinkcnt", &self.blinkcnt())
                .field("blinken", &self.blinken())
                .field("clkcs", &self.clkcs())
                .field("lcdfs", &self.lcdfs())
                .field("dmaen", &self.dmaen())
                .field("ie", &self.ie())
                .field("intf", &self.intf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ blinkcnt: {=u8:?}, blinken: {=bool:?}, clkcs: {:?}, lcdfs: {:?}, dmaen: {=bool:?}, ie: {=bool:?}, intf: {=bool:?} }}",
                self.blinkcnt(),
                self.blinken(),
                self.clkcs(),
                self.lcdfs(),
                self.dmaen(),
                self.ie(),
                self.intf()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            f.debug_struct("Icr").field("intf", &self.intf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Icr {{ intf: {=bool:?} }}", self.intf())
        }
    }
    #[doc = "desc PINEN1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pinen1(pub u32);
    impl Pinen1 {
        #[doc = "desc SEG0."]
        #[must_use]
        #[inline(always)]
        pub const fn seg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG0."]
        #[inline(always)]
        pub const fn set_seg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SEG1."]
        #[must_use]
        #[inline(always)]
        pub const fn seg1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG1."]
        #[inline(always)]
        pub const fn set_seg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SEG2."]
        #[must_use]
        #[inline(always)]
        pub const fn seg2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG2."]
        #[inline(always)]
        pub const fn set_seg2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SEG3."]
        #[must_use]
        #[inline(always)]
        pub const fn seg3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG3."]
        #[inline(always)]
        pub const fn set_seg3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SEG4."]
        #[must_use]
        #[inline(always)]
        pub const fn seg4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG4."]
        #[inline(always)]
        pub const fn set_seg4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SEG5."]
        #[must_use]
        #[inline(always)]
        pub const fn seg5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG5."]
        #[inline(always)]
        pub const fn set_seg5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SEG6."]
        #[must_use]
        #[inline(always)]
        pub const fn seg6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG6."]
        #[inline(always)]
        pub const fn set_seg6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SEG7."]
        #[must_use]
        #[inline(always)]
        pub const fn seg7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG7."]
        #[inline(always)]
        pub const fn set_seg7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SEG8."]
        #[must_use]
        #[inline(always)]
        pub const fn seg8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG8."]
        #[inline(always)]
        pub const fn set_seg8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SEG9."]
        #[must_use]
        #[inline(always)]
        pub const fn seg9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG9."]
        #[inline(always)]
        pub const fn set_seg9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc SEG10."]
        #[must_use]
        #[inline(always)]
        pub const fn seg10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG10."]
        #[inline(always)]
        pub const fn set_seg10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SEG11."]
        #[must_use]
        #[inline(always)]
        pub const fn seg11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG11."]
        #[inline(always)]
        pub const fn set_seg11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc SEG12."]
        #[must_use]
        #[inline(always)]
        pub const fn seg12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG12."]
        #[inline(always)]
        pub const fn set_seg12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SEG13."]
        #[must_use]
        #[inline(always)]
        pub const fn seg13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG13."]
        #[inline(always)]
        pub const fn set_seg13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SEG14."]
        #[must_use]
        #[inline(always)]
        pub const fn seg14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG14."]
        #[inline(always)]
        pub const fn set_seg14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SEG15."]
        #[must_use]
        #[inline(always)]
        pub const fn seg15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG15."]
        #[inline(always)]
        pub const fn set_seg15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SEG16."]
        #[must_use]
        #[inline(always)]
        pub const fn seg16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG16."]
        #[inline(always)]
        pub const fn set_seg16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SEG17."]
        #[must_use]
        #[inline(always)]
        pub const fn seg17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG17."]
        #[inline(always)]
        pub const fn set_seg17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SEG18."]
        #[must_use]
        #[inline(always)]
        pub const fn seg18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG18."]
        #[inline(always)]
        pub const fn set_seg18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SEG19."]
        #[must_use]
        #[inline(always)]
        pub const fn seg19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG19."]
        #[inline(always)]
        pub const fn set_seg19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc SEG20."]
        #[must_use]
        #[inline(always)]
        pub const fn seg20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG20."]
        #[inline(always)]
        pub const fn set_seg20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc SEG21."]
        #[must_use]
        #[inline(always)]
        pub const fn seg21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG21."]
        #[inline(always)]
        pub const fn set_seg21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc SEG22."]
        #[must_use]
        #[inline(always)]
        pub const fn seg22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG22."]
        #[inline(always)]
        pub const fn set_seg22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc SEG23."]
        #[must_use]
        #[inline(always)]
        pub const fn seg23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG23."]
        #[inline(always)]
        pub const fn set_seg23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc SEG24VLCD1."]
        #[must_use]
        #[inline(always)]
        pub const fn seg24vlcd1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG24VLCD1."]
        #[inline(always)]
        pub const fn set_seg24vlcd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc SEG25VLCD2."]
        #[must_use]
        #[inline(always)]
        pub const fn seg25vlcd2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG25VLCD2."]
        #[inline(always)]
        pub const fn set_seg25vlcd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc SEG26VLCD3."]
        #[must_use]
        #[inline(always)]
        pub const fn seg26vlcd3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG26VLCD3."]
        #[inline(always)]
        pub const fn set_seg26vlcd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SEG27VLCD4."]
        #[must_use]
        #[inline(always)]
        pub const fn seg27vlcd4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG27VLCD4."]
        #[inline(always)]
        pub const fn set_seg27vlcd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SEG28COM7."]
        #[must_use]
        #[inline(always)]
        pub const fn seg28com7(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG28COM7."]
        #[inline(always)]
        pub const fn set_seg28com7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SEG29COM6."]
        #[must_use]
        #[inline(always)]
        pub const fn seg29com6(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG29COM6."]
        #[inline(always)]
        pub const fn set_seg29com6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc SEG30COM5."]
        #[must_use]
        #[inline(always)]
        pub const fn seg30com5(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG30COM5."]
        #[inline(always)]
        pub const fn set_seg30com5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc SEG31COM4."]
        #[must_use]
        #[inline(always)]
        pub const fn seg31com4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG31COM4."]
        #[inline(always)]
        pub const fn set_seg31com4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pinen1 {
        #[inline(always)]
        fn default() -> Pinen1 {
            Pinen1(0)
        }
    }
    impl core::fmt::Debug for Pinen1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pinen1")
                .field("seg0", &self.seg0())
                .field("seg1", &self.seg1())
                .field("seg2", &self.seg2())
                .field("seg3", &self.seg3())
                .field("seg4", &self.seg4())
                .field("seg5", &self.seg5())
                .field("seg6", &self.seg6())
                .field("seg7", &self.seg7())
                .field("seg8", &self.seg8())
                .field("seg9", &self.seg9())
                .field("seg10", &self.seg10())
                .field("seg11", &self.seg11())
                .field("seg12", &self.seg12())
                .field("seg13", &self.seg13())
                .field("seg14", &self.seg14())
                .field("seg15", &self.seg15())
                .field("seg16", &self.seg16())
                .field("seg17", &self.seg17())
                .field("seg18", &self.seg18())
                .field("seg19", &self.seg19())
                .field("seg20", &self.seg20())
                .field("seg21", &self.seg21())
                .field("seg22", &self.seg22())
                .field("seg23", &self.seg23())
                .field("seg24vlcd1", &self.seg24vlcd1())
                .field("seg25vlcd2", &self.seg25vlcd2())
                .field("seg26vlcd3", &self.seg26vlcd3())
                .field("seg27vlcd4", &self.seg27vlcd4())
                .field("seg28com7", &self.seg28com7())
                .field("seg29com6", &self.seg29com6())
                .field("seg30com5", &self.seg30com5())
                .field("seg31com4", &self.seg31com4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pinen1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pinen1 {{ seg0: {=bool:?}, seg1: {=bool:?}, seg2: {=bool:?}, seg3: {=bool:?}, seg4: {=bool:?}, seg5: {=bool:?}, seg6: {=bool:?}, seg7: {=bool:?}, seg8: {=bool:?}, seg9: {=bool:?}, seg10: {=bool:?}, seg11: {=bool:?}, seg12: {=bool:?}, seg13: {=bool:?}, seg14: {=bool:?}, seg15: {=bool:?}, seg16: {=bool:?}, seg17: {=bool:?}, seg18: {=bool:?}, seg19: {=bool:?}, seg20: {=bool:?}, seg21: {=bool:?}, seg22: {=bool:?}, seg23: {=bool:?}, seg24vlcd1: {=bool:?}, seg25vlcd2: {=bool:?}, seg26vlcd3: {=bool:?}, seg27vlcd4: {=bool:?}, seg28com7: {=bool:?}, seg29com6: {=bool:?}, seg30com5: {=bool:?}, seg31com4: {=bool:?} }}",
                self.seg0(),
                self.seg1(),
                self.seg2(),
                self.seg3(),
                self.seg4(),
                self.seg5(),
                self.seg6(),
                self.seg7(),
                self.seg8(),
                self.seg9(),
                self.seg10(),
                self.seg11(),
                self.seg12(),
                self.seg13(),
                self.seg14(),
                self.seg15(),
                self.seg16(),
                self.seg17(),
                self.seg18(),
                self.seg19(),
                self.seg20(),
                self.seg21(),
                self.seg22(),
                self.seg23(),
                self.seg24vlcd1(),
                self.seg25vlcd2(),
                self.seg26vlcd3(),
                self.seg27vlcd4(),
                self.seg28com7(),
                self.seg29com6(),
                self.seg30com5(),
                self.seg31com4()
            )
        }
    }
    #[doc = "desc PINEN2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pinen2(pub u32);
    impl Pinen2 {
        #[doc = "desc SEG32."]
        #[must_use]
        #[inline(always)]
        pub const fn seg32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG32."]
        #[inline(always)]
        pub const fn set_seg32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SEG33."]
        #[must_use]
        #[inline(always)]
        pub const fn seg33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG33."]
        #[inline(always)]
        pub const fn set_seg33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SEG34."]
        #[must_use]
        #[inline(always)]
        pub const fn seg34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG34."]
        #[inline(always)]
        pub const fn set_seg34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SEG35."]
        #[must_use]
        #[inline(always)]
        pub const fn seg35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG35."]
        #[inline(always)]
        pub const fn set_seg35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SEG52."]
        #[must_use]
        #[inline(always)]
        pub const fn seg52(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG52."]
        #[inline(always)]
        pub const fn set_seg52(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc SEG53."]
        #[must_use]
        #[inline(always)]
        pub const fn seg53(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG53."]
        #[inline(always)]
        pub const fn set_seg53(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc SEG54."]
        #[must_use]
        #[inline(always)]
        pub const fn seg54(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG54."]
        #[inline(always)]
        pub const fn set_seg54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc SEG55."]
        #[must_use]
        #[inline(always)]
        pub const fn seg55(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEG55."]
        #[inline(always)]
        pub const fn set_seg55(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc COM0."]
        #[must_use]
        #[inline(always)]
        pub const fn com0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc COM0."]
        #[inline(always)]
        pub const fn set_com0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc COM1."]
        #[must_use]
        #[inline(always)]
        pub const fn com1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc COM1."]
        #[inline(always)]
        pub const fn set_com1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc COM2."]
        #[must_use]
        #[inline(always)]
        pub const fn com2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc COM2."]
        #[inline(always)]
        pub const fn set_com2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc COM3."]
        #[must_use]
        #[inline(always)]
        pub const fn com3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc COM3."]
        #[inline(always)]
        pub const fn set_com3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc MUX."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Pinen2 {
        #[inline(always)]
        fn default() -> Pinen2 {
            Pinen2(0)
        }
    }
    impl core::fmt::Debug for Pinen2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pinen2")
                .field("seg32", &self.seg32())
                .field("seg33", &self.seg33())
                .field("seg34", &self.seg34())
                .field("seg35", &self.seg35())
                .field("seg52", &self.seg52())
                .field("seg53", &self.seg53())
                .field("seg54", &self.seg54())
                .field("seg55", &self.seg55())
                .field("com0", &self.com0())
                .field("com1", &self.com1())
                .field("com2", &self.com2())
                .field("com3", &self.com3())
                .field("mux", &self.mux())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pinen2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pinen2 {{ seg32: {=bool:?}, seg33: {=bool:?}, seg34: {=bool:?}, seg35: {=bool:?}, seg52: {=bool:?}, seg53: {=bool:?}, seg54: {=bool:?}, seg55: {=bool:?}, com0: {=bool:?}, com1: {=bool:?}, com2: {=bool:?}, com3: {=bool:?}, mux: {=bool:?} }}",
                self.seg32(),
                self.seg33(),
                self.seg34(),
                self.seg35(),
                self.seg52(),
                self.seg53(),
                self.seg54(),
                self.seg55(),
                self.com0(),
                self.com1(),
                self.com2(),
                self.com3(),
                self.mux()
            )
        }
    }
    #[doc = "desc RAM0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0(pub u32);
    impl Ram0 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram0 {
        #[inline(always)]
        fn default() -> Ram0 {
            Ram0(0)
        }
    }
    impl core::fmt::Debug for Ram0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram0 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1(pub u32);
    impl Ram1 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram1 {
        #[inline(always)]
        fn default() -> Ram1 {
            Ram1(0)
        }
    }
    impl core::fmt::Debug for Ram1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram1 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram13(pub u32);
    impl Ram13 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram13 {
        #[inline(always)]
        fn default() -> Ram13 {
            Ram13(0)
        }
    }
    impl core::fmt::Debug for Ram13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram13")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram13 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2(pub u32);
    impl Ram2 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram2 {
        #[inline(always)]
        fn default() -> Ram2 {
            Ram2(0)
        }
    }
    impl core::fmt::Debug for Ram2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram2 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3(pub u32);
    impl Ram3 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram3 {
        #[inline(always)]
        fn default() -> Ram3 {
            Ram3(0)
        }
    }
    impl core::fmt::Debug for Ram3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram3 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram4(pub u32);
    impl Ram4 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram4 {
        #[inline(always)]
        fn default() -> Ram4 {
            Ram4(0)
        }
    }
    impl core::fmt::Debug for Ram4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram4")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram4 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5(pub u32);
    impl Ram5 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram5 {
        #[inline(always)]
        fn default() -> Ram5 {
            Ram5(0)
        }
    }
    impl core::fmt::Debug for Ram5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram5 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram6(pub u32);
    impl Ram6 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram6 {
        #[inline(always)]
        fn default() -> Ram6 {
            Ram6(0)
        }
    }
    impl core::fmt::Debug for Ram6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram6")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram6 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram7(pub u32);
    impl Ram7 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram7 {
        #[inline(always)]
        fn default() -> Ram7 {
            Ram7(0)
        }
    }
    impl core::fmt::Debug for Ram7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram7")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram7 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
    #[doc = "desc RAM8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram8(pub u32);
    impl Ram8 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram8 {
        #[inline(always)]
        fn default() -> Ram8 {
            Ram8(0)
        }
    }
    impl core::fmt::Debug for Ram8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram8")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ram8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ram8 {{ d0: {=bool:?}, d1: {=bool:?}, d2: {=bool:?}, d3: {=bool:?}, d4: {=bool:?}, d5: {=bool:?}, d6: {=bool:?}, d7: {=bool:?}, d8: {=bool:?}, d9: {=bool:?}, d10: {=bool:?}, d11: {=bool:?}, d12: {=bool:?}, d13: {=bool:?}, d14: {=bool:?}, d15: {=bool:?}, d16: {=bool:?}, d17: {=bool:?}, d18: {=bool:?}, d19: {=bool:?}, d20: {=bool:?}, d21: {=bool:?}, d22: {=bool:?}, d23: {=bool:?}, d24: {=bool:?}, d25: {=bool:?}, d26: {=bool:?}, d27: {=bool:?}, d28: {=bool:?}, d29: {=bool:?}, d30: {=bool:?}, d31: {=bool:?} }}",
                self.d0(),
                self.d1(),
                self.d2(),
                self.d3(),
                self.d4(),
                self.d5(),
                self.d6(),
                self.d7(),
                self.d8(),
                self.d9(),
                self.d10(),
                self.d11(),
                self.d12(),
                self.d13(),
                self.d14(),
                self.d15(),
                self.d16(),
                self.d17(),
                self.d18(),
                self.d19(),
                self.d20(),
                self.d21(),
                self.d22(),
                self.d23(),
                self.d24(),
                self.d25(),
                self.d26(),
                self.d27(),
                self.d28(),
                self.d29(),
                self.d30(),
                self.d31()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bias {
        Third = 0x0,
        Half = 0x01,
    }
    impl Bias {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bias {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bias {
        #[inline(always)]
        fn from(val: u8) -> Bias {
            Bias::from_bits(val)
        }
    }
    impl From<Bias> for u8 {
        #[inline(always)]
        fn from(val: Bias) -> u8 {
            Bias::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BiasSource {
        External = 0x0,
        InternalMedium = 0x01,
        InternalLow = 0x02,
        _RESERVED_3 = 0x03,
        InternalLowest = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        InternalHighest = 0x07,
    }
    impl BiasSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BiasSource {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BiasSource {
        #[inline(always)]
        fn from(val: u8) -> BiasSource {
            BiasSource::from_bits(val)
        }
    }
    impl From<BiasSource> for u8 {
        #[inline(always)]
        fn from(val: BiasSource) -> u8 {
            BiasSource::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ClockSource {
        Lsi = 0x0,
        Lse = 0x01,
    }
    impl ClockSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClockSource {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClockSource {
        #[inline(always)]
        fn from(val: u8) -> ClockSource {
            ClockSource::from_bits(val)
        }
    }
    impl From<ClockSource> for u8 {
        #[inline(always)]
        fn from(val: ClockSource) -> u8 {
            ClockSource::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Duty {
        Static = 0x0,
        Half = 0x01,
        Third = 0x02,
        Quarter = 0x03,
        _RESERVED_4 = 0x04,
        Sixth = 0x05,
        _RESERVED_6 = 0x06,
        Eighth = 0x07,
    }
    impl Duty {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Duty {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Duty {
        #[inline(always)]
        fn from(val: u8) -> Duty {
            Duty::from_bits(val)
        }
    }
    impl From<Duty> for u8 {
        #[inline(always)]
        fn from(val: Duty) -> u8 {
            Duty::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PumpFrequency {
        Khz2 = 0x0,
        Khz4 = 0x01,
        Khz8 = 0x02,
        Khz16 = 0x03,
    }
    impl PumpFrequency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PumpFrequency {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PumpFrequency {
        #[inline(always)]
        fn from(val: u8) -> PumpFrequency {
            PumpFrequency::from_bits(val)
        }
    }
    impl From<PumpFrequency> for u8 {
        #[inline(always)]
        fn from(val: PumpFrequency) -> u8 {
            PumpFrequency::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ScanFrequency {
        Hz128 = 0x0,
        Hz256 = 0x01,
        Hz512 = 0x02,
        Hz64 = 0x03,
    }
    impl ScanFrequency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ScanFrequency {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ScanFrequency {
        #[inline(always)]
        fn from(val: u8) -> ScanFrequency {
            ScanFrequency::from_bits(val)
        }
    }
    impl From<ScanFrequency> for u8 {
        #[inline(always)]
        fn from(val: ScanFrequency) -> u8 {
            ScanFrequency::to_bits(val)
        }
    }
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(4096)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(4096)
    }
}
