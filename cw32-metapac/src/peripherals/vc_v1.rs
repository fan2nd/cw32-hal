#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc VC1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vc {
    ptr: *mut u8,
}
unsafe impl Send for Vc {}
unsafe impl Sync for Vc {}
impl Vc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DIV."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register0."]
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
        #[doc = "desc RESP."]
        #[must_use]
        #[inline(always)]
        pub const fn resp(&self) -> super::vals::ResponseSpeed {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::ResponseSpeed::from_bits(val as u8)
        }
        #[doc = "desc RESP."]
        #[inline(always)]
        pub const fn set_resp(&mut self, val: super::vals::ResponseSpeed) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "desc HYS."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> super::vals::Hysteresis {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hysteresis::from_bits(val as u8)
        }
        #[doc = "desc HYS."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: super::vals::Hysteresis) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc POL."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> super::vals::Polarity {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "desc POL."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "desc WINDOW."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDOW."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc INP."]
        #[must_use]
        #[inline(always)]
        pub const fn inp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc INP."]
        #[inline(always)]
        pub const fn set_inp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc INN."]
        #[must_use]
        #[inline(always)]
        pub const fn inn(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc INN."]
        #[inline(always)]
        pub const fn set_inn(&mut self, val: u8) {
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
                .field("resp", &self.resp())
                .field("hys", &self.hys())
                .field("ie", &self.ie())
                .field("pol", &self.pol())
                .field("window", &self.window())
                .field("inp", &self.inp())
                .field("inn", &self.inn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, resp: {:?}, hys: {:?}, ie: {=bool:?}, pol: {:?}, window: {=bool:?}, inp: {=u8:?}, inn: {=u8:?} }}",
                self.en(),
                self.resp(),
                self.hys(),
                self.ie(),
                self.pol(),
                self.window(),
                self.inp(),
                self.inn()
            )
        }
    }
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc FLTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTEN."]
        #[inline(always)]
        pub const fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn flttime(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTTIME."]
        #[inline(always)]
        pub const fn set_flttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> super::vals::FilterClock {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::FilterClock::from_bits(val as u8)
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: super::vals::FilterClock) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FALLIE."]
        #[must_use]
        #[inline(always)]
        pub const fn fallie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FALLIE."]
        #[inline(always)]
        pub const fn set_fallie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RISEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn riseie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RISEIE."]
        #[inline(always)]
        pub const fn set_riseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HIGHIE."]
        #[must_use]
        #[inline(always)]
        pub const fn highie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc HIGHIE."]
        #[inline(always)]
        pub const fn set_highie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ATIMCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn atimclr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCLR."]
        #[inline(always)]
        pub const fn set_atimclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ATIMBK."]
        #[must_use]
        #[inline(always)]
        pub const fn atimbk(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMBK."]
        #[inline(always)]
        pub const fn set_atimbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BLANKCH1B."]
        #[must_use]
        #[inline(always)]
        pub const fn blankch1b(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLANKCH1B."]
        #[inline(always)]
        pub const fn set_blankch1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BLANKCH2B."]
        #[must_use]
        #[inline(always)]
        pub const fn blankch2b(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLANKCH2B."]
        #[inline(always)]
        pub const fn set_blankch2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BLANKCH3B."]
        #[must_use]
        #[inline(always)]
        pub const fn blankch3b(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLANKCH3B."]
        #[inline(always)]
        pub const fn set_blankch3b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BLANKFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn blankflt(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "desc BLANKFLT."]
        #[inline(always)]
        pub const fn set_blankflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
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
                .field("flten", &self.flten())
                .field("flttime", &self.flttime())
                .field("fltclk", &self.fltclk())
                .field("fallie", &self.fallie())
                .field("riseie", &self.riseie())
                .field("highie", &self.highie())
                .field("atimclr", &self.atimclr())
                .field("atimbk", &self.atimbk())
                .field("blankch1b", &self.blankch1b())
                .field("blankch2b", &self.blankch2b())
                .field("blankch3b", &self.blankch3b())
                .field("blankflt", &self.blankflt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ flten: {=bool:?}, flttime: {=u8:?}, fltclk: {:?}, fallie: {=bool:?}, riseie: {=bool:?}, highie: {=bool:?}, atimclr: {=bool:?}, atimbk: {=bool:?}, blankch1b: {=bool:?}, blankch2b: {=bool:?}, blankch3b: {=bool:?}, blankflt: {=u8:?} }}",
                self.flten(),
                self.flttime(),
                self.fltclk(),
                self.fallie(),
                self.riseie(),
                self.highie(),
                self.atimclr(),
                self.atimbk(),
                self.blankch1b(),
                self.blankch2b(),
                self.blankch3b(),
                self.blankflt()
            )
        }
    }
    #[doc = "desc DIV."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div(pub u32);
    impl Div {
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VIN."]
        #[must_use]
        #[inline(always)]
        pub const fn vin(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VIN."]
        #[inline(always)]
        pub const fn set_vin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Div {
        #[inline(always)]
        fn default() -> Div {
            Div(0)
        }
    }
    impl core::fmt::Debug for Div {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Div")
                .field("div", &self.div())
                .field("en", &self.en())
                .field("vin", &self.vin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Div {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Div {{ div: {=u8:?}, en: {=bool:?}, vin: {=bool:?} }}",
                self.div(),
                self.en(),
                self.vin()
            )
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTV."]
        #[must_use]
        #[inline(always)]
        pub const fn fltv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTV."]
        #[inline(always)]
        pub const fn set_fltv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc READY."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc READY."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("intf", &self.intf())
                .field("fltv", &self.fltv())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ intf: {=bool:?}, fltv: {=bool:?}, ready: {=bool:?} }}",
                self.intf(),
                self.fltv(),
                self.ready()
            )
        }
    }
}
pub mod vals {
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FilterClock {
        #[doc = "Internal RC filter clock; see the selected own-manual source."]
        InternalRc = 0x0,
        #[doc = "Peripheral bus clock."]
        Pclk = 0x01,
    }
    impl FilterClock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterClock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterClock {
        #[inline(always)]
        fn from(val: u8) -> FilterClock {
            FilterClock::from_bits(val)
        }
    }
    impl From<FilterClock> for u8 {
        #[inline(always)]
        fn from(val: FilterClock) -> u8 {
            FilterClock::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hysteresis {
        #[doc = "Hysteresis disabled."]
        None = 0x0,
        #[doc = "Approximately 10 mV hysteresis; typical only."]
        Low = 0x01,
        #[doc = "Approximately 20 mV hysteresis; typical only."]
        Medium = 0x02,
        #[doc = "Approximately 30 mV hysteresis; typical only."]
        High = 0x03,
    }
    impl Hysteresis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hysteresis {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hysteresis {
        #[inline(always)]
        fn from(val: u8) -> Hysteresis {
            Hysteresis::from_bits(val)
        }
    }
    impl From<Hysteresis> for u8 {
        #[inline(always)]
        fn from(val: Hysteresis) -> u8 {
            Hysteresis::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polarity {
        #[doc = "Non-inverted output."]
        Normal = 0x0,
        #[doc = "Inverted output."]
        Inverted = 0x01,
    }
    impl Polarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polarity {
        #[inline(always)]
        fn from(val: u8) -> Polarity {
            Polarity::from_bits(val)
        }
    }
    impl From<Polarity> for u8 {
        #[inline(always)]
        fn from(val: Polarity) -> u8 {
            Polarity::to_bits(val)
        }
    }
    #[doc = "Source-qualified comparator control; see docs/comparator.md and own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ResponseSpeed {
        #[doc = "Ultra-low speed and power."]
        UltraLow = 0x0,
        #[doc = "Low speed and power."]
        Low = 0x01,
        #[doc = "Medium speed and power."]
        Medium = 0x02,
        #[doc = "High speed and power."]
        High = 0x03,
    }
    impl ResponseSpeed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ResponseSpeed {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ResponseSpeed {
        #[inline(always)]
        fn from(val: u8) -> ResponseSpeed {
            ResponseSpeed::from_bits(val)
        }
    }
    impl From<ResponseSpeed> for u8 {
        #[inline(always)]
        fn from(val: ResponseSpeed) -> u8 {
            ResponseSpeed::to_bits(val)
        }
    }
}
impl regs::Sr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(0)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(1)
    }
}
