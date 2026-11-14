#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc RTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc KEY."]
    #[inline(always)]
    pub const fn key(self) -> crate::common::Reg<regs::Key, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc COMPEN."]
    #[inline(always)]
    pub const fn compen(self) -> crate::common::Reg<regs::Compen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc DATE."]
    #[inline(always)]
    pub const fn date(self) -> crate::common::Reg<regs::Date, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc TIME."]
    #[inline(always)]
    pub const fn time(self) -> crate::common::Reg<regs::Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc ALARMA."]
    #[inline(always)]
    pub const fn alarma(self) -> crate::common::Reg<regs::Alarma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc ALARMB."]
    #[inline(always)]
    pub const fn alarmb(self) -> crate::common::Reg<regs::Alarmb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TAMPDATE."]
    #[inline(always)]
    pub const fn tampdate(self) -> crate::common::Reg<regs::Tampdate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc TAMPTIME."]
    #[inline(always)]
    pub const fn tamptime(self) -> crate::common::Reg<regs::Tamptime, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc AWTARR."]
    #[inline(always)]
    pub const fn awtarr(self) -> crate::common::Reg<regs::Awtarr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ALARMA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarma(pub u32);
    impl Alarma {
        #[doc = "desc SECOND."]
        #[must_use]
        #[inline(always)]
        pub const fn second(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SECOND."]
        #[inline(always)]
        pub const fn set_second(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc SECONDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn seconden(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SECONDEN."]
        #[inline(always)]
        pub const fn set_seconden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MINUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn minute(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc MINUTE."]
        #[inline(always)]
        pub const fn set_minute(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc MINUTEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn minuteen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc MINUTEEN."]
        #[inline(always)]
        pub const fn set_minuteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HOUR."]
        #[must_use]
        #[inline(always)]
        pub const fn hour(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HOUR."]
        #[inline(always)]
        pub const fn set_hour(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "desc HOUREN."]
        #[must_use]
        #[inline(always)]
        pub const fn houren(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HOUREN."]
        #[inline(always)]
        pub const fn set_houren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc WEEKMASK."]
        #[must_use]
        #[inline(always)]
        pub const fn weekmask(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WEEKMASK."]
        #[inline(always)]
        pub const fn set_weekmask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Alarma {
        #[inline(always)]
        fn default() -> Alarma {
            Alarma(0)
        }
    }
    impl core::fmt::Debug for Alarma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarma")
                .field("second", &self.second())
                .field("seconden", &self.seconden())
                .field("minute", &self.minute())
                .field("minuteen", &self.minuteen())
                .field("hour", &self.hour())
                .field("houren", &self.houren())
                .field("weekmask", &self.weekmask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarma {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alarma {{ second: {=u8:?}, seconden: {=bool:?}, minute: {=u8:?}, minuteen: {=bool:?}, hour: {=u8:?}, houren: {=bool:?}, weekmask: {=u8:?} }}",
                self.second(),
                self.seconden(),
                self.minute(),
                self.minuteen(),
                self.hour(),
                self.houren(),
                self.weekmask()
            )
        }
    }
    #[doc = "desc ALARMB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarmb(pub u32);
    impl Alarmb {
        #[doc = "desc SECOND."]
        #[must_use]
        #[inline(always)]
        pub const fn second(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SECOND."]
        #[inline(always)]
        pub const fn set_second(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc SECONDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn seconden(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SECONDEN."]
        #[inline(always)]
        pub const fn set_seconden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MINUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn minute(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc MINUTE."]
        #[inline(always)]
        pub const fn set_minute(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc MINUTEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn minuteen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc MINUTEEN."]
        #[inline(always)]
        pub const fn set_minuteen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HOUR."]
        #[must_use]
        #[inline(always)]
        pub const fn hour(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HOUR."]
        #[inline(always)]
        pub const fn set_hour(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "desc HOUREN."]
        #[must_use]
        #[inline(always)]
        pub const fn houren(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc HOUREN."]
        #[inline(always)]
        pub const fn set_houren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc WEEKMASK."]
        #[must_use]
        #[inline(always)]
        pub const fn weekmask(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WEEKMASK."]
        #[inline(always)]
        pub const fn set_weekmask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Alarmb {
        #[inline(always)]
        fn default() -> Alarmb {
            Alarmb(0)
        }
    }
    impl core::fmt::Debug for Alarmb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarmb")
                .field("second", &self.second())
                .field("seconden", &self.seconden())
                .field("minute", &self.minute())
                .field("minuteen", &self.minuteen())
                .field("hour", &self.hour())
                .field("houren", &self.houren())
                .field("weekmask", &self.weekmask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarmb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alarmb {{ second: {=u8:?}, seconden: {=bool:?}, minute: {=u8:?}, minuteen: {=bool:?}, hour: {=u8:?}, houren: {=bool:?}, weekmask: {=u8:?} }}",
                self.second(),
                self.seconden(),
                self.minute(),
                self.minuteen(),
                self.hour(),
                self.houren(),
                self.weekmask()
            )
        }
    }
    #[doc = "desc AWTARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awtarr(pub u32);
    impl Awtarr {
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
    impl Default for Awtarr {
        #[inline(always)]
        fn default() -> Awtarr {
            Awtarr(0)
        }
    }
    impl core::fmt::Debug for Awtarr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awtarr").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awtarr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Awtarr {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "desc COMPEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Compen(pub u32);
    impl Compen {
        #[doc = "desc COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc COMP."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc STEP."]
        #[must_use]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc STEP."]
        #[inline(always)]
        pub const fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc SIGN."]
        #[must_use]
        #[inline(always)]
        pub const fn sign(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SIGN."]
        #[inline(always)]
        pub const fn set_sign(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Compen {
        #[inline(always)]
        fn default() -> Compen {
            Compen(0)
        }
    }
    impl core::fmt::Debug for Compen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Compen")
                .field("comp", &self.comp())
                .field("step", &self.step())
                .field("sign", &self.sign())
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Compen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Compen {{ comp: {=u16:?}, step: {=u8:?}, sign: {=bool:?}, en: {=bool:?} }}",
                self.comp(),
                self.step(),
                self.sign(),
                self.en()
            )
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc INTERVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn interval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc INTERVAL."]
        #[inline(always)]
        pub const fn set_interval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc AMPM."]
        #[must_use]
        #[inline(always)]
        pub const fn h24(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AMPM."]
        #[inline(always)]
        pub const fn set_h24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RTC1HZ."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc1hz(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc RTC1HZ."]
        #[inline(always)]
        pub const fn set_rtc1hz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("interval", &self.interval())
                .field("h24", &self.h24())
                .field("rtc1hz", &self.rtc1hz())
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ interval: {=u8:?}, h24: {=bool:?}, rtc1hz: {=u8:?}, start: {=bool:?} }}",
                self.interval(),
                self.h24(),
                self.rtc1hz(),
                self.start()
            )
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc ACCESS."]
        #[must_use]
        #[inline(always)]
        pub const fn access(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACCESS."]
        #[inline(always)]
        pub const fn set_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WINDOW."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDOW."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> super::vals::Source {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .field("access", &self.access())
                .field("window", &self.window())
                .field("source", &self.source())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ access: {=bool:?}, window: {=bool:?}, source: {:?} }}",
                self.access(),
                self.window(),
                self.source()
            )
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc AWTSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn awtsrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc AWTSRC."]
        #[inline(always)]
        pub const fn set_awtsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc TAMPEDGE."]
        #[must_use]
        #[inline(always)]
        pub const fn tampedge(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMPEDGE."]
        #[inline(always)]
        pub const fn set_tampedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RTCOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcout(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc RTCOUT."]
        #[inline(always)]
        pub const fn set_rtcout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc TAMPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tampen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMPEN."]
        #[inline(always)]
        pub const fn set_tampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc AWTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn awten(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWTEN."]
        #[inline(always)]
        pub const fn set_awten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ALARMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn alarmaen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMAEN."]
        #[inline(always)]
        pub const fn set_alarmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ALARMBEN."]
        #[must_use]
        #[inline(always)]
        pub const fn alarmben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMBEN."]
        #[inline(always)]
        pub const fn set_alarmben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("awtsrc", &self.awtsrc())
                .field("tampedge", &self.tampedge())
                .field("rtcout", &self.rtcout())
                .field("tampen", &self.tampen())
                .field("awten", &self.awten())
                .field("alarmaen", &self.alarmaen())
                .field("alarmben", &self.alarmben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ awtsrc: {=u8:?}, tampedge: {=bool:?}, rtcout: {=u8:?}, tampen: {=bool:?}, awten: {=bool:?}, alarmaen: {=bool:?}, alarmben: {=bool:?} }}",
                self.awtsrc(),
                self.tampedge(),
                self.rtcout(),
                self.tampen(),
                self.awten(),
                self.alarmaen(),
                self.alarmben()
            )
        }
    }
    #[doc = "desc DATE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Date(pub u32);
    impl Date {
        #[doc = "desc DAY."]
        #[must_use]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DAY."]
        #[inline(always)]
        pub const fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc MONTH."]
        #[must_use]
        #[inline(always)]
        pub const fn month(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc MONTH."]
        #[inline(always)]
        pub const fn set_month(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc YEAR."]
        #[must_use]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc YEAR."]
        #[inline(always)]
        pub const fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Date {
        #[inline(always)]
        fn default() -> Date {
            Date(0)
        }
    }
    impl core::fmt::Debug for Date {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Date")
                .field("day", &self.day())
                .field("month", &self.month())
                .field("year", &self.year())
                .field("week", &self.week())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Date {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Date {{ day: {=u8:?}, month: {=u8:?}, year: {=u8:?}, week: {=u8:?} }}",
                self.day(),
                self.month(),
                self.year(),
                self.week()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc ALARMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alarma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMA."]
        #[inline(always)]
        pub const fn set_alarma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALARMB."]
        #[must_use]
        #[inline(always)]
        pub const fn alarmb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMB."]
        #[inline(always)]
        pub const fn set_alarmb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AWTIMER."]
        #[must_use]
        #[inline(always)]
        pub const fn awtimer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWTIMER."]
        #[inline(always)]
        pub const fn set_awtimer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TAMP."]
        #[must_use]
        #[inline(always)]
        pub const fn tamp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMP."]
        #[inline(always)]
        pub const fn set_tamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TAMPOV."]
        #[must_use]
        #[inline(always)]
        pub const fn tampov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMPOV."]
        #[inline(always)]
        pub const fn set_tampov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INTERVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn interval(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTERVAL."]
        #[inline(always)]
        pub const fn set_interval(&mut self, val: bool) {
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
                .field("alarma", &self.alarma())
                .field("alarmb", &self.alarmb())
                .field("awtimer", &self.awtimer())
                .field("tamp", &self.tamp())
                .field("tampov", &self.tampov())
                .field("interval", &self.interval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ alarma: {=bool:?}, alarmb: {=bool:?}, awtimer: {=bool:?}, tamp: {=bool:?}, tampov: {=bool:?}, interval: {=bool:?} }}",
                self.alarma(),
                self.alarmb(),
                self.awtimer(),
                self.tamp(),
                self.tampov(),
                self.interval()
            )
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc ALARMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alarma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMA."]
        #[inline(always)]
        pub const fn set_alarma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALARMB."]
        #[must_use]
        #[inline(always)]
        pub const fn alarmb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMB."]
        #[inline(always)]
        pub const fn set_alarmb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AWTIMER."]
        #[must_use]
        #[inline(always)]
        pub const fn awtimer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWTIMER."]
        #[inline(always)]
        pub const fn set_awtimer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TAMP."]
        #[must_use]
        #[inline(always)]
        pub const fn tamp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMP."]
        #[inline(always)]
        pub const fn set_tamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TAMPOV."]
        #[must_use]
        #[inline(always)]
        pub const fn tampov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMPOV."]
        #[inline(always)]
        pub const fn set_tampov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INTERVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn interval(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTERVAL."]
        #[inline(always)]
        pub const fn set_interval(&mut self, val: bool) {
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
                .field("alarma", &self.alarma())
                .field("alarmb", &self.alarmb())
                .field("awtimer", &self.awtimer())
                .field("tamp", &self.tamp())
                .field("tampov", &self.tampov())
                .field("interval", &self.interval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ alarma: {=bool:?}, alarmb: {=bool:?}, awtimer: {=bool:?}, tamp: {=bool:?}, tampov: {=bool:?}, interval: {=bool:?} }}",
                self.alarma(),
                self.alarmb(),
                self.awtimer(),
                self.tamp(),
                self.tampov(),
                self.interval()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc ALARMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alarma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMA."]
        #[inline(always)]
        pub const fn set_alarma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALARMB."]
        #[must_use]
        #[inline(always)]
        pub const fn alarmb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALARMB."]
        #[inline(always)]
        pub const fn set_alarmb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AWTIMER."]
        #[must_use]
        #[inline(always)]
        pub const fn awtimer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWTIMER."]
        #[inline(always)]
        pub const fn set_awtimer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TAMP."]
        #[must_use]
        #[inline(always)]
        pub const fn tamp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMP."]
        #[inline(always)]
        pub const fn set_tamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TAMPOV."]
        #[must_use]
        #[inline(always)]
        pub const fn tampov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TAMPOV."]
        #[inline(always)]
        pub const fn set_tampov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INTERVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn interval(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTERVAL."]
        #[inline(always)]
        pub const fn set_interval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("alarma", &self.alarma())
                .field("alarmb", &self.alarmb())
                .field("awtimer", &self.awtimer())
                .field("tamp", &self.tamp())
                .field("tampov", &self.tampov())
                .field("interval", &self.interval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ alarma: {=bool:?}, alarmb: {=bool:?}, awtimer: {=bool:?}, tamp: {=bool:?}, tampov: {=bool:?}, interval: {=bool:?} }}",
                self.alarma(),
                self.alarmb(),
                self.awtimer(),
                self.tamp(),
                self.tampov(),
                self.interval()
            )
        }
    }
    #[doc = "desc KEY."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key(pub u32);
    impl Key {
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Key {
        #[inline(always)]
        fn default() -> Key {
            Key(0)
        }
    }
    impl core::fmt::Debug for Key {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key {{ key: {=u8:?} }}", self.key())
        }
    }
    #[doc = "desc TAMPDATE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tampdate(pub u32);
    impl Tampdate {
        #[doc = "desc DAY."]
        #[must_use]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DAY."]
        #[inline(always)]
        pub const fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc MONTH."]
        #[must_use]
        #[inline(always)]
        pub const fn month(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MONTH."]
        #[inline(always)]
        pub const fn set_month(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Tampdate {
        #[inline(always)]
        fn default() -> Tampdate {
            Tampdate(0)
        }
    }
    impl core::fmt::Debug for Tampdate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tampdate")
                .field("day", &self.day())
                .field("month", &self.month())
                .field("week", &self.week())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tampdate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tampdate {{ day: {=u8:?}, month: {=u8:?}, week: {=u8:?} }}",
                self.day(),
                self.month(),
                self.week()
            )
        }
    }
    #[doc = "desc TAMPTIME."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tamptime(pub u32);
    impl Tamptime {
        #[doc = "desc SECOND."]
        #[must_use]
        #[inline(always)]
        pub const fn second(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SECOND."]
        #[inline(always)]
        pub const fn set_second(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc MINUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn minute(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc MINUTE."]
        #[inline(always)]
        pub const fn set_minute(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc HOUR."]
        #[must_use]
        #[inline(always)]
        pub const fn hour(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HOUR."]
        #[inline(always)]
        pub const fn set_hour(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Tamptime {
        #[inline(always)]
        fn default() -> Tamptime {
            Tamptime(0)
        }
    }
    impl core::fmt::Debug for Tamptime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tamptime")
                .field("second", &self.second())
                .field("minute", &self.minute())
                .field("hour", &self.hour())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tamptime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tamptime {{ second: {=u8:?}, minute: {=u8:?}, hour: {=u8:?} }}",
                self.second(),
                self.minute(),
                self.hour()
            )
        }
    }
    #[doc = "desc TIME."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Time(pub u32);
    impl Time {
        #[doc = "desc SECOND."]
        #[must_use]
        #[inline(always)]
        pub const fn second(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SECOND."]
        #[inline(always)]
        pub const fn set_second(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc MINUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn minute(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc MINUTE."]
        #[inline(always)]
        pub const fn set_minute(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc HOUR."]
        #[must_use]
        #[inline(always)]
        pub const fn hour(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HOUR."]
        #[inline(always)]
        pub const fn set_hour(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Time {
        #[inline(always)]
        fn default() -> Time {
            Time(0)
        }
    }
    impl core::fmt::Debug for Time {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Time")
                .field("second", &self.second())
                .field("minute", &self.minute())
                .field("hour", &self.hour())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Time {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Time {{ second: {=u8:?}, minute: {=u8:?}, hour: {=u8:?} }}",
                self.second(),
                self.minute(),
                self.hour()
            )
        }
    }
}
pub mod vals {
    #[doc = "Own-manual RTC source encodings; divider applies to HSE. Encodings 1 and 3 remain reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Source {
        #[doc = "External low-speed clock."]
        Lse = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Internal low-speed clock."]
        Lsi = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HSE divided by 128."]
        HseDiv128 = 0x04,
        #[doc = "HSE divided by 256."]
        HseDiv256 = 0x05,
        #[doc = "HSE divided by 512."]
        HseDiv512 = 0x06,
        #[doc = "HSE divided by 1024."]
        HseDiv1024 = 0x07,
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
        Self(127)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(127)
    }
}
