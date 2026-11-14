#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Automatic trim and independent timer. Safe HAL exposes only MD=3/AUTO=0; see docs/autotrim-counter-evidence.md."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autotrim {
    ptr: *mut u8,
}
unsafe impl Send for Autotrim {}
unsafe impl Sync for Autotrim {}
impl Autotrim {
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
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc FCAP."]
    #[inline(always)]
    pub const fn fcap(self) -> crate::common::Reg<regs::Fcap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc TVAL."]
    #[inline(always)]
    pub const fn tval(self) -> crate::common::Reg<regs::Tval, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc FLIM."]
    #[inline(always)]
    pub const fn flim(self) -> crate::common::Reg<regs::Flim, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
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
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
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
        pub const fn md(&self) -> super::vals::Mode {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MD."]
        #[inline(always)]
        pub const fn set_md(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "desc AUTO."]
        #[must_use]
        #[inline(always)]
        pub const fn auto(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTO."]
        #[inline(always)]
        pub const fn set_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> super::vals::Prescaler {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Prescaler::from_bits(val as u8)
        }
        #[doc = "desc PRS."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: super::vals::Prescaler) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc SRC."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> super::vals::Source {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc SRC."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "desc OST."]
        #[must_use]
        #[inline(always)]
        pub const fn ost(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OST."]
        #[inline(always)]
        pub const fn set_ost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
            f.debug_struct("Cr")
                .field("en", &self.en())
                .field("md", &self.md())
                .field("auto", &self.auto())
                .field("prs", &self.prs())
                .field("src", &self.src())
                .field("ost", &self.ost())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, md: {:?}, auto: {=bool:?}, prs: {:?}, src: {:?}, ost: {=bool:?} }}",
                self.en(),
                self.md(),
                self.auto(),
                self.prs(),
                self.src(),
                self.ost()
            )
        }
    }
    #[doc = "desc FCAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcap(pub u32);
    impl Fcap {
        #[doc = "desc FCAP."]
        #[must_use]
        #[inline(always)]
        pub const fn fcap(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FCAP."]
        #[inline(always)]
        pub const fn set_fcap(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Fcap {
        #[inline(always)]
        fn default() -> Fcap {
            Fcap(0)
        }
    }
    impl core::fmt::Debug for Fcap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcap").field("fcap", &self.fcap()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fcap {{ fcap: {=u16:?} }}", self.fcap())
        }
    }
    #[doc = "desc FLIM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flim(pub u32);
    impl Flim {
        #[doc = "desc FLIM."]
        #[must_use]
        #[inline(always)]
        pub const fn flim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FLIM."]
        #[inline(always)]
        pub const fn set_flim(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Flim {
        #[inline(always)]
        fn default() -> Flim {
            Flim(0)
        }
    }
    impl core::fmt::Debug for Flim {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flim").field("flim", &self.flim()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flim {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Flim {{ flim: {=u16:?} }}", self.flim())
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Calibration ended, not proof of success."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration ended, not proof of success."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[must_use]
        #[inline(always)]
        pub const fn ok(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[inline(always)]
        pub const fn set_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Calibration interval count was missed."]
        #[must_use]
        #[inline(always)]
        pub const fn miss(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration interval count was missed."]
        #[inline(always)]
        pub const fn set_miss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("end", &self.end())
                .field("ok", &self.ok())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("miss", &self.miss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ end: {=bool:?}, ok: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, miss: {=bool:?} }}",
                self.end(),
                self.ok(),
                self.ud(),
                self.ov(),
                self.miss()
            )
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Calibration ended, not proof of success."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration ended, not proof of success."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[must_use]
        #[inline(always)]
        pub const fn ok(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[inline(always)]
        pub const fn set_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Calibration interval count was missed."]
        #[must_use]
        #[inline(always)]
        pub const fn miss(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration interval count was missed."]
        #[inline(always)]
        pub const fn set_miss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("end", &self.end())
                .field("ok", &self.ok())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("miss", &self.miss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ end: {=bool:?}, ok: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, miss: {=bool:?} }}",
                self.end(),
                self.ok(),
                self.ud(),
                self.ov(),
                self.miss()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Calibration ended, not proof of success."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration ended, not proof of success."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[must_use]
        #[inline(always)]
        pub const fn ok(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration reached its frequency-error target."]
        #[inline(always)]
        pub const fn set_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer underflow event; multiple periods coalesce."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration trim code reached 0 or 0x1ff; not counter overflow."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Calibration interval count was missed."]
        #[must_use]
        #[inline(always)]
        pub const fn miss(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration interval count was missed."]
        #[inline(always)]
        pub const fn set_miss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("end", &self.end())
                .field("ok", &self.ok())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("miss", &self.miss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ end: {=bool:?}, ok: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, miss: {=bool:?} }}",
                self.end(),
                self.ok(),
                self.ud(),
                self.ov(),
                self.miss()
            )
        }
    }
    #[doc = "desc TVAL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tval(pub u32);
    impl Tval {
        #[doc = "desc TVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn tval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TVAL."]
        #[inline(always)]
        pub const fn set_tval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tval {
        #[inline(always)]
        fn default() -> Tval {
            Tval(0)
        }
    }
    impl core::fmt::Debug for Tval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tval").field("tval", &self.tval()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tval {{ tval: {=u16:?} }}", self.tval())
        }
    }
}
pub mod vals {
    #[doc = "Own-family CN UM section 11; see docs/autotrim-counter-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "HSIOSC calibration; unsafe oscillator ownership is outside the safe HAL."]
        HsiCalibration = 0x0,
        #[doc = "LSI calibration; unsafe oscillator ownership is outside the safe HAL."]
        LsiCalibration = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Periodic down-counter, with automatic trim disabled."]
        Timer = 0x03,
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
    #[doc = "Own-family CN UM section 11; see docs/autotrim-counter-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prescaler {
        _RESERVED_0 = 0x0,
        #[doc = "Divide source by 2."]
        Div2 = 0x01,
        #[doc = "Divide source by 4."]
        Div4 = 0x02,
        #[doc = "Divide source by 8."]
        Div8 = 0x03,
        #[doc = "Divide source by 16."]
        Div16 = 0x04,
        #[doc = "Divide source by 32."]
        Div32 = 0x05,
        #[doc = "Divide source by 64."]
        Div64 = 0x06,
        #[doc = "Divide source by 128."]
        Div128 = 0x07,
        #[doc = "Divide source by 256."]
        Div256 = 0x08,
        #[doc = "Divide source by 512."]
        Div512 = 0x09,
        #[doc = "Divide source by 1024."]
        Div1024 = 0x0a,
        #[doc = "Divide source by 2048."]
        Div2048 = 0x0b,
        #[doc = "Divide source by 4096."]
        Div4096 = 0x0c,
        #[doc = "Divide source by 8192."]
        Div8192 = 0x0d,
        #[doc = "Divide source by 16384."]
        Div16384 = 0x0e,
        #[doc = "Divide source by 32768."]
        Div32768 = 0x0f,
    }
    impl Prescaler {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prescaler {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prescaler {
        #[inline(always)]
        fn from(val: u8) -> Prescaler {
            Prescaler::from_bits(val)
        }
    }
    impl From<Prescaler> for u8 {
        #[inline(always)]
        fn from(val: Prescaler) -> u8 {
            Prescaler::to_bits(val)
        }
    }
    #[doc = "Own-family CN UM section 11; see docs/autotrim-counter-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Source {
        #[doc = "Undivided nominal 48 MHz HSIOSC."]
        HsiOsc = 0x0,
        #[doc = "Internal low-speed oscillator."]
        Lsi = 0x01,
        #[doc = "External high-speed oscillator."]
        Hse = 0x02,
        #[doc = "External low-speed oscillator."]
        Lse = 0x03,
        #[doc = "External input."]
        Etr = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Source {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Source {
            unsafe { core::mem::transmute(val & 0x07) }
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
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(63)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(63)
    }
}
