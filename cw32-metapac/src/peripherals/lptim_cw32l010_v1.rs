#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc LPTIM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim {
    ptr: *mut u8,
}
unsafe impl Send for Lptim {}
unsafe impl Sync for Lptim {}
impl Lptim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CFGR."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc CMP."]
    #[inline(always)]
    pub const fn cmp(self) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
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
    #[doc = "desc CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "desc CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKSEL."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ENCMD_CKPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn encmd_ckpol(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENCMD_CKPOL."]
        #[inline(always)]
        pub const fn set_encmd_ckpol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc CHFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn chflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "desc CHFLT."]
        #[inline(always)]
        pub const fn set_chflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "desc TRIGFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn trigflt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc TRIGFLT."]
        #[inline(always)]
        pub const fn set_trigflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> super::vals::Prescaler {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::Prescaler::from_bits(val as u8)
        }
        #[doc = "desc PRS."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: super::vals::Prescaler) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "desc TRIGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trigsel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "desc TRIGSEL."]
        #[inline(always)]
        pub const fn set_trigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "desc TRIGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn trigen(&self) -> super::vals::Trigger {
            let val = (self.0 >> 17usize) & 0x03;
            super::vals::Trigger::from_bits(val as u8)
        }
        #[doc = "desc TRIGEN."]
        #[inline(always)]
        pub const fn set_trigen(&mut self, val: super::vals::Trigger) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
        }
        #[doc = "desc TIMOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn timout(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMOUT."]
        #[inline(always)]
        pub const fn set_timout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc WAVE."]
        #[must_use]
        #[inline(always)]
        pub const fn wave(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAVE."]
        #[inline(always)]
        pub const fn set_wave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc WAVPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn wavpol(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAVPOL."]
        #[inline(always)]
        pub const fn set_wavpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc PRELOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRELOAD."]
        #[inline(always)]
        pub const fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc COUNTMD."]
        #[must_use]
        #[inline(always)]
        pub const fn countmd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc COUNTMD."]
        #[inline(always)]
        pub const fn set_countmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc ENC."]
        #[must_use]
        #[inline(always)]
        pub const fn enc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENC."]
        #[inline(always)]
        pub const fn set_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc ICLKSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn iclksrc(&self) -> super::vals::Source {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc ICLKSRC."]
        #[inline(always)]
        pub const fn set_iclksrc(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("cksel", &self.cksel())
                .field("encmd_ckpol", &self.encmd_ckpol())
                .field("chflt", &self.chflt())
                .field("trigflt", &self.trigflt())
                .field("prs", &self.prs())
                .field("trigsel", &self.trigsel())
                .field("trigen", &self.trigen())
                .field("timout", &self.timout())
                .field("wave", &self.wave())
                .field("wavpol", &self.wavpol())
                .field("preload", &self.preload())
                .field("countmd", &self.countmd())
                .field("enc", &self.enc())
                .field("iclksrc", &self.iclksrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ cksel: {=bool:?}, encmd_ckpol: {=u8:?}, chflt: {=u8:?}, trigflt: {=u8:?}, prs: {:?}, trigsel: {=u8:?}, trigen: {:?}, timout: {=bool:?}, wave: {=bool:?}, wavpol: {=bool:?}, preload: {=bool:?}, countmd: {=bool:?}, enc: {=bool:?}, iclksrc: {:?} }}",
                self.cksel(),
                self.encmd_ckpol(),
                self.chflt(),
                self.trigflt(),
                self.prs(),
                self.trigsel(),
                self.trigen(),
                self.timout(),
                self.wave(),
                self.wavpol(),
                self.preload(),
                self.countmd(),
                self.enc(),
                self.iclksrc()
            )
        }
    }
    #[doc = "desc CMP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "desc CMP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CMP."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    impl core::fmt::Debug for Cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmp").field("cmp", &self.cmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmp {{ cmp: {=u16:?} }}", self.cmp())
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
        #[doc = "desc SNGSTART."]
        #[must_use]
        #[inline(always)]
        pub const fn sngstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNGSTART."]
        #[inline(always)]
        pub const fn set_sngstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CNTSTART."]
        #[must_use]
        #[inline(always)]
        pub const fn cntstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CNTSTART."]
        #[inline(always)]
        pub const fn set_cntstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SRST."]
        #[must_use]
        #[inline(always)]
        pub const fn srst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRST."]
        #[inline(always)]
        pub const fn set_srst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ARST."]
        #[must_use]
        #[inline(always)]
        pub const fn arst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARST."]
        #[inline(always)]
        pub const fn set_arst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("sngstart", &self.sngstart())
                .field("cntstart", &self.cntstart())
                .field("srst", &self.srst())
                .field("arst", &self.arst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, sngstart: {=bool:?}, cntstart: {=bool:?}, srst: {=bool:?}, arst: {=bool:?} }}",
                self.en(),
                self.sngstart(),
                self.cntstart(),
                self.srst(),
                self.arst()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc CMPM."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPM."]
        #[inline(always)]
        pub const fn set_cmpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ARRM."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARRM."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EXTTRIG."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTTRIG."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMPOK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpok(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPOK."]
        #[inline(always)]
        pub const fn set_cmpok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ARROK."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARROK."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc UP."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc UP."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DOWN."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOWN."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("cmpm", &self.cmpm())
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("cmpok", &self.cmpok())
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ cmpm: {=bool:?}, arrm: {=bool:?}, exttrig: {=bool:?}, cmpok: {=bool:?}, arrok: {=bool:?}, up: {=bool:?}, down: {=bool:?} }}",
                self.cmpm(),
                self.arrm(),
                self.exttrig(),
                self.cmpok(),
                self.arrok(),
                self.up(),
                self.down()
            )
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc CMPM."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPM."]
        #[inline(always)]
        pub const fn set_cmpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ARRM."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARRM."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EXTTRIG."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTTRIG."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMPOK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpok(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPOK."]
        #[inline(always)]
        pub const fn set_cmpok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ARROK."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARROK."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc UP."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc UP."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DOWN."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOWN."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("cmpm", &self.cmpm())
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("cmpok", &self.cmpok())
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ cmpm: {=bool:?}, arrm: {=bool:?}, exttrig: {=bool:?}, cmpok: {=bool:?}, arrok: {=bool:?}, up: {=bool:?}, down: {=bool:?} }}",
                self.cmpm(),
                self.arrm(),
                self.exttrig(),
                self.cmpok(),
                self.arrok(),
                self.up(),
                self.down()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc CMPM."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPM."]
        #[inline(always)]
        pub const fn set_cmpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ARRM."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARRM."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EXTTRIG."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTTRIG."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMPOK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpok(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPOK."]
        #[inline(always)]
        pub const fn set_cmpok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ARROK."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARROK."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc UP."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc UP."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DOWN."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DOWN."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "desc DIR."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
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
                .field("cmpm", &self.cmpm())
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("cmpok", &self.cmpok())
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .field("dir", &self.dir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ cmpm: {=bool:?}, arrm: {=bool:?}, exttrig: {=bool:?}, cmpok: {=bool:?}, arrok: {=bool:?}, up: {=bool:?}, down: {=bool:?}, dir: {=u8:?} }}",
                self.cmpm(),
                self.arrm(),
                self.exttrig(),
                self.cmpok(),
                self.arrok(),
                self.up(),
                self.down(),
                self.dir()
            )
        }
    }
}
pub mod vals {
    #[doc = "Reviewed timer encodings; see docs/low-power-timers-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prescaler {
        #[doc = "Divide by 1."]
        Div1 = 0x0,
        #[doc = "Divide by 2."]
        Div2 = 0x01,
        #[doc = "Divide by 4."]
        Div4 = 0x02,
        #[doc = "Divide by 8."]
        Div8 = 0x03,
        #[doc = "Divide by 16."]
        Div16 = 0x04,
        #[doc = "Divide by 32."]
        Div32 = 0x05,
        #[doc = "Divide by 64."]
        Div64 = 0x06,
        #[doc = "Divide by 128."]
        Div128 = 0x07,
    }
    impl Prescaler {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prescaler {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Reviewed timer encodings; see docs/low-power-timers-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Source {
        #[doc = "Peripheral bus clock."]
        Pclk = 0x0,
        #[doc = "Peripheral bus clock, alternate encoding."]
        PclkAlt = 0x01,
        #[doc = "External low-speed oscillator."]
        Lse = 0x02,
        #[doc = "Internal low-speed oscillator."]
        Lsi = 0x03,
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
    #[doc = "Reviewed timer encodings; see docs/low-power-timers-evidence.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trigger {
        #[doc = "Software start."]
        Software = 0x0,
        #[doc = "Rising-edge trigger."]
        Rising = 0x01,
        #[doc = "Falling-edge trigger."]
        Falling = 0x02,
        #[doc = "Both-edge trigger."]
        Both = 0x03,
    }
    impl Trigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trigger {
        #[inline(always)]
        fn from(val: u8) -> Trigger {
            Trigger::from_bits(val)
        }
    }
    impl From<Trigger> for u8 {
        #[inline(always)]
        fn from(val: Trigger) -> u8 {
            Trigger::to_bits(val)
        }
    }
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(127)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(127)
    }
}
impl regs::Cr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(24)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(24)
    }
}
