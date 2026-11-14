#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GTIM1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtim {
    ptr: *mut u8,
}
unsafe impl Send for Gtim {}
unsafe impl Sync for Gtim {}
impl Gtim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "desc CMMR."]
    #[inline(always)]
    pub const fn cmmr(self) -> crate::common::Reg<regs::Cmmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "desc ETR."]
    #[inline(always)]
    pub const fn etr(self) -> crate::common::Reg<regs::Etr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "desc CCR1."]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "desc CCR2."]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::common::Reg<regs::Ccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "desc CCR3."]
    #[inline(always)]
    pub const fn ccr3(self) -> crate::common::Reg<regs::Ccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "desc CCR4."]
    #[inline(always)]
    pub const fn ccr4(self) -> crate::common::Reg<regs::Ccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "desc PSC."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "desc DMA."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<regs::Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "desc ARR."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ARR."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Arr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Arr {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "desc CCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr1(pub u32);
    impl Ccr1 {
        #[doc = "desc CCR."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr1 {
        #[inline(always)]
        fn default() -> Ccr1 {
            Ccr1(0)
        }
    }
    impl core::fmt::Debug for Ccr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr1").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr1 {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "desc CCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr2(pub u32);
    impl Ccr2 {
        #[doc = "desc CCR."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr2 {
        #[inline(always)]
        fn default() -> Ccr2 {
            Ccr2(0)
        }
    }
    impl core::fmt::Debug for Ccr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr2").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr2 {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "desc CCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr3(pub u32);
    impl Ccr3 {
        #[doc = "desc CCR."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr3 {
        #[inline(always)]
        fn default() -> Ccr3 {
            Ccr3(0)
        }
    }
    impl core::fmt::Debug for Ccr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr3").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr3 {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "desc CCR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr4(pub u32);
    impl Ccr4 {
        #[doc = "desc CCR."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCR."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr4 {
        #[inline(always)]
        fn default() -> Ccr4 {
            Ccr4(0)
        }
    }
    impl core::fmt::Debug for Ccr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr4").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr4 {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "desc CMMR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmmr(pub u32);
    impl Cmmr {
        #[doc = "desc CC1M."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1m(&self) -> super::vals::CcMode {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::CcMode::from_bits(val as u8)
        }
        #[doc = "desc CC1M."]
        #[inline(always)]
        pub const fn set_cc1m(&mut self, val: super::vals::CcMode) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc CC2M."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2m(&self) -> super::vals::CcMode {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::CcMode::from_bits(val as u8)
        }
        #[doc = "desc CC2M."]
        #[inline(always)]
        pub const fn set_cc2m(&mut self, val: super::vals::CcMode) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc CC3M."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3m(&self) -> super::vals::CcMode {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::CcMode::from_bits(val as u8)
        }
        #[doc = "desc CC3M."]
        #[inline(always)]
        pub const fn set_cc3m(&mut self, val: super::vals::CcMode) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc CC4M."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4m(&self) -> super::vals::CcMode {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::CcMode::from_bits(val as u8)
        }
        #[doc = "desc CC4M."]
        #[inline(always)]
        pub const fn set_cc4m(&mut self, val: super::vals::CcMode) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Cmmr {
        #[inline(always)]
        fn default() -> Cmmr {
            Cmmr(0)
        }
    }
    impl core::fmt::Debug for Cmmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmmr")
                .field("cc1m", &self.cc1m())
                .field("cc2m", &self.cc2m())
                .field("cc3m", &self.cc3m())
                .field("cc4m", &self.cc4m())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmmr {{ cc1m: {:?}, cc2m: {:?}, cc3m: {:?}, cc4m: {:?} }}",
                self.cc1m(),
                self.cc2m(),
                self.cc3m(),
                self.cc4m()
            )
        }
    }
    #[doc = "desc CNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
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
        #[doc = "desc MD."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MD."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "desc TRS."]
        #[must_use]
        #[inline(always)]
        pub const fn trs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRS."]
        #[inline(always)]
        pub const fn set_trs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc POLARITY."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc POLARITY."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ONESHOT."]
        #[must_use]
        #[inline(always)]
        pub const fn oneshot(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ONESHOT."]
        #[inline(always)]
        pub const fn set_oneshot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TOGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn togen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOGEN."]
        #[inline(always)]
        pub const fn set_togen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ENCMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn encmode(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENCMODE."]
        #[inline(always)]
        pub const fn set_encmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "desc ENCRESET."]
        #[must_use]
        #[inline(always)]
        pub const fn encreset(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENCRESET."]
        #[inline(always)]
        pub const fn set_encreset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "desc ENCRELOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn encreload(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENCRELOAD."]
        #[inline(always)]
        pub const fn set_encreload(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
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
                .field("mode", &self.mode())
                .field("trs", &self.trs())
                .field("pol", &self.pol())
                .field("oneshot", &self.oneshot())
                .field("togen", &self.togen())
                .field("encmode", &self.encmode())
                .field("encreset", &self.encreset())
                .field("encreload", &self.encreload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, mode: {:?}, trs: {=bool:?}, pol: {=bool:?}, oneshot: {=bool:?}, togen: {=bool:?}, encmode: {=u8:?}, encreset: {=u8:?}, encreload: {=u8:?} }}",
                self.en(),
                self.mode(),
                self.trs(),
                self.pol(),
                self.oneshot(),
                self.togen(),
                self.encmode(),
                self.encreset(),
                self.encreload()
            )
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc CH1FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1flt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH1FLT."]
        #[inline(always)]
        pub const fn set_ch1flt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc CH1POL."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1pol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH1POL."]
        #[inline(always)]
        pub const fn set_ch1pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CH2FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2flt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH2FLT."]
        #[inline(always)]
        pub const fn set_ch2flt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc CH2POL."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2pol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH2POL."]
        #[inline(always)]
        pub const fn set_ch2pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CH3FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3flt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH3FLT."]
        #[inline(always)]
        pub const fn set_ch3flt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc CH3POL."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3pol(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH3POL."]
        #[inline(always)]
        pub const fn set_ch3pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CH4FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4flt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc CH4FLT."]
        #[inline(always)]
        pub const fn set_ch4flt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc CH4POL."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4pol(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH4POL."]
        #[inline(always)]
        pub const fn set_ch4pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("ch1flt", &self.ch1flt())
                .field("ch1pol", &self.ch1pol())
                .field("ch2flt", &self.ch2flt())
                .field("ch2pol", &self.ch2pol())
                .field("ch3flt", &self.ch3flt())
                .field("ch3pol", &self.ch3pol())
                .field("ch4flt", &self.ch4flt())
                .field("ch4pol", &self.ch4pol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ ch1flt: {=u8:?}, ch1pol: {=bool:?}, ch2flt: {=u8:?}, ch2pol: {=bool:?}, ch3flt: {=u8:?}, ch3pol: {=bool:?}, ch4flt: {=u8:?}, ch4pol: {=bool:?} }}",
                self.ch1flt(),
                self.ch1pol(),
                self.ch2flt(),
                self.ch2pol(),
                self.ch3flt(),
                self.ch3pol(),
                self.ch4flt(),
                self.ch4pol()
            )
        }
    }
    #[doc = "desc DMA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma(pub u32);
    impl Dma {
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TRS."]
        #[must_use]
        #[inline(always)]
        pub const fn trs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRS."]
        #[inline(always)]
        pub const fn set_trs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CC1."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC2."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2."]
        #[inline(always)]
        pub const fn set_cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC3."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3."]
        #[inline(always)]
        pub const fn set_cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CC4."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4."]
        #[inline(always)]
        pub const fn set_cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Dma {
        #[inline(always)]
        fn default() -> Dma {
            Dma(0)
        }
    }
    impl core::fmt::Debug for Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma")
                .field("ov", &self.ov())
                .field("trs", &self.trs())
                .field("cc1", &self.cc1())
                .field("cc2", &self.cc2())
                .field("cc3", &self.cc3())
                .field("cc4", &self.cc4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dma {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dma {{ ov: {=bool:?}, trs: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, cc4: {=bool:?} }}",
                self.ov(),
                self.trs(),
                self.cc1(),
                self.cc2(),
                self.cc3(),
                self.cc4()
            )
        }
    }
    #[doc = "desc ETR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etr(pub u32);
    impl Etr {
        #[doc = "desc ETRFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn etrflt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc ETRFLT."]
        #[inline(always)]
        pub const fn set_etrflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Etr {
        #[inline(always)]
        fn default() -> Etr {
            Etr(0)
        }
    }
    impl core::fmt::Debug for Etr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Etr")
                .field("etrflt", &self.etrflt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Etr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Etr {{ etrflt: {=u8:?} }}", self.etrflt())
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC1."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC2."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2."]
        #[inline(always)]
        pub const fn set_cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CC3."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3."]
        #[inline(always)]
        pub const fn set_cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CC4."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4."]
        #[inline(always)]
        pub const fn set_cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DIRCHANGE."]
        #[must_use]
        #[inline(always)]
        pub const fn dirchange(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRCHANGE."]
        #[inline(always)]
        pub const fn set_dirchange(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            f.debug_struct("Icr")
                .field("ov", &self.ov())
                .field("ti", &self.ti())
                .field("ud", &self.ud())
                .field("cc1", &self.cc1())
                .field("cc2", &self.cc2())
                .field("cc3", &self.cc3())
                .field("cc4", &self.cc4())
                .field("dirchange", &self.dirchange())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ ov: {=bool:?}, ti: {=bool:?}, ud: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, cc4: {=bool:?}, dirchange: {=bool:?} }}",
                self.ov(),
                self.ti(),
                self.ud(),
                self.cc1(),
                self.cc2(),
                self.cc3(),
                self.cc4(),
                self.dirchange()
            )
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC1."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC2."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2."]
        #[inline(always)]
        pub const fn set_cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CC3."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3."]
        #[inline(always)]
        pub const fn set_cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CC4."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4."]
        #[inline(always)]
        pub const fn set_cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DIRCHANGE."]
        #[must_use]
        #[inline(always)]
        pub const fn dirchange(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRCHANGE."]
        #[inline(always)]
        pub const fn set_dirchange(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("ov", &self.ov())
                .field("ti", &self.ti())
                .field("ud", &self.ud())
                .field("cc1", &self.cc1())
                .field("cc2", &self.cc2())
                .field("cc3", &self.cc3())
                .field("cc4", &self.cc4())
                .field("dirchange", &self.dirchange())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ ov: {=bool:?}, ti: {=bool:?}, ud: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, cc4: {=bool:?}, dirchange: {=bool:?} }}",
                self.ov(),
                self.ti(),
                self.ud(),
                self.cc1(),
                self.cc2(),
                self.cc3(),
                self.cc4(),
                self.dirchange()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TI."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TI."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CC1."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC1."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CC2."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC2."]
        #[inline(always)]
        pub const fn set_cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CC3."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC3."]
        #[inline(always)]
        pub const fn set_cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CC4."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CC4."]
        #[inline(always)]
        pub const fn set_cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DIRCHANGE."]
        #[must_use]
        #[inline(always)]
        pub const fn dirchange(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRCHANGE."]
        #[inline(always)]
        pub const fn set_dirchange(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIR."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("ov", &self.ov())
                .field("ti", &self.ti())
                .field("ud", &self.ud())
                .field("cc1", &self.cc1())
                .field("cc2", &self.cc2())
                .field("cc3", &self.cc3())
                .field("cc4", &self.cc4())
                .field("dirchange", &self.dirchange())
                .field("dir", &self.dir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ ov: {=bool:?}, ti: {=bool:?}, ud: {=bool:?}, cc1: {=bool:?}, cc2: {=bool:?}, cc3: {=bool:?}, cc4: {=bool:?}, dirchange: {=bool:?}, dir: {=bool:?} }}",
                self.ov(),
                self.ti(),
                self.ud(),
                self.cc1(),
                self.cc2(),
                self.cc3(),
                self.cc4(),
                self.dirchange(),
                self.dir()
            )
        }
    }
    #[doc = "desc PSC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psc(pub u32);
    impl Psc {
        #[doc = "desc PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PSC."]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Psc {
        #[inline(always)]
        fn default() -> Psc {
            Psc(0)
        }
    }
    impl core::fmt::Debug for Psc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psc").field("psc", &self.psc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Psc {{ psc: {=u16:?} }}", self.psc())
        }
    }
}
pub mod vals {
    #[doc = "Classic capture/compare modes; own-manual evidence in docs/classic-gtim-modes-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcMode {
        #[doc = "No capture or compare function."]
        Disabled = 0x0,
        #[doc = "Capture on rising edges."]
        CaptureRising = 0x01,
        #[doc = "Capture on falling edges."]
        CaptureFalling = 0x02,
        #[doc = "Capture on both rising and falling edges."]
        CaptureBoth = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Force the output low."]
        ForceLow = 0x08,
        #[doc = "Force the output high."]
        ForceHigh = 0x09,
        #[doc = "Drive the output low on a compare match."]
        CompareLow = 0x0a,
        #[doc = "Drive the output high on a compare match."]
        CompareHigh = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        #[doc = "PWM output high when CNT is greater than or equal to CCR."]
        PwmHighAtOrAbove = 0x0e,
        #[doc = "PWM output high when CNT is less than CCR."]
        PwmHighBelow = 0x0f,
    }
    impl CcMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcMode {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcMode {
        #[inline(always)]
        fn from(val: u8) -> CcMode {
            CcMode::from_bits(val)
        }
    }
    impl From<CcMode> for u8 {
        #[inline(always)]
        fn from(val: CcMode) -> u8 {
            CcMode::to_bits(val)
        }
    }
    #[doc = "Basic timer mode when ENCMODE is zero; own-manual evidence in docs/classic-gtim-modes-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Count PCLK ticks."]
        Timer = 0x0,
        #[doc = "Count TRS input events. PSC must be greater than zero in counter mode."]
        Counter = 0x01,
        #[doc = "Count PCLK ticks; a valid TRS event can start the timer."]
        TriggerStart = 0x02,
        #[doc = "Count PCLK ticks while enabled and ETR is at the active level selected by POL."]
        Gated = 0x03,
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
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(1023)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(1023)
    }
}
