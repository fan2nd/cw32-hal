#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc OPA1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa {
    ptr: *mut u8,
}
unsafe impl Send for Opa {}
unsafe impl Sync for Opa {}
impl Opa {
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
    #[doc = "desc CAL."]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Mixed configuration/status/command: AZRUN read-only; SOFTTRIG write-one command. Full configuration writes must set SOFTTRIG=0 unless explicitly triggering."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cal(pub u32);
    impl Cal {
        #[doc = "desc CALEN."]
        #[must_use]
        #[inline(always)]
        pub const fn calen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CALEN."]
        #[inline(always)]
        pub const fn set_calen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CALMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn calmode(&self) -> super::vals::CalibrationMode {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::CalibrationMode::from_bits(val as u8)
        }
        #[doc = "desc CALMODE."]
        #[inline(always)]
        pub const fn set_calmode(&mut self, val: super::vals::CalibrationMode) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SOFTTRIG."]
        #[must_use]
        #[inline(always)]
        pub const fn softtrig(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTTRIG."]
        #[inline(always)]
        pub const fn set_softtrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CALPERIOD."]
        #[must_use]
        #[inline(always)]
        pub const fn calperiod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CALPERIOD."]
        #[inline(always)]
        pub const fn set_calperiod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc AZRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn azrun(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZRUN."]
        #[inline(always)]
        pub const fn set_azrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc CLKDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "desc CLKDIV."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[doc = "desc ADC1START1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1start1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC1START1."]
        #[inline(always)]
        pub const fn set_adc1start1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc ADC2START1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2start1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC2START1."]
        #[inline(always)]
        pub const fn set_adc2start1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc ATIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn atim1trgo(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM1TRGO."]
        #[inline(always)]
        pub const fn set_atim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc ATIM1OC4."]
        #[must_use]
        #[inline(always)]
        pub const fn atim1oc4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM1OC4."]
        #[inline(always)]
        pub const fn set_atim1oc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc ATIM1OC5."]
        #[must_use]
        #[inline(always)]
        pub const fn atim1oc5(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM1OC5."]
        #[inline(always)]
        pub const fn set_atim1oc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc ATIM1OC6."]
        #[must_use]
        #[inline(always)]
        pub const fn atim1oc6(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM1OC6."]
        #[inline(always)]
        pub const fn set_atim1oc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc GTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1trgo(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1TRGO."]
        #[inline(always)]
        pub const fn set_gtim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc GTIM1OC1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC1."]
        #[inline(always)]
        pub const fn set_gtim1oc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc GTIM1OC2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1oc2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1OC2."]
        #[inline(always)]
        pub const fn set_gtim1oc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc GTIM2TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2trgo(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2TRGO."]
        #[inline(always)]
        pub const fn set_gtim2trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc GTIM2OC1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC1."]
        #[inline(always)]
        pub const fn set_gtim2oc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc GTIM2OC2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim2oc2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM2OC2."]
        #[inline(always)]
        pub const fn set_gtim2oc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc GTIM3TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3trgo(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3TRGO."]
        #[inline(always)]
        pub const fn set_gtim3trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc GTIM3OC1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC1."]
        #[inline(always)]
        pub const fn set_gtim3oc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc GTIM3OC2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim3oc2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM3OC2."]
        #[inline(always)]
        pub const fn set_gtim3oc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc GTIM4TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4trgo(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4TRGO."]
        #[inline(always)]
        pub const fn set_gtim4trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc GTIM4OC1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC1."]
        #[inline(always)]
        pub const fn set_gtim4oc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc GTIM4OC2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim4oc2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM4OC2."]
        #[inline(always)]
        pub const fn set_gtim4oc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cal {
        #[inline(always)]
        fn default() -> Cal {
            Cal(0)
        }
    }
    impl core::fmt::Debug for Cal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cal")
                .field("calen", &self.calen())
                .field("calmode", &self.calmode())
                .field("start", &self.start())
                .field("softtrig", &self.softtrig())
                .field("calperiod", &self.calperiod())
                .field("azrun", &self.azrun())
                .field("clkdiv", &self.clkdiv())
                .field("adc1start1", &self.adc1start1())
                .field("adc2start1", &self.adc2start1())
                .field("atim1trgo", &self.atim1trgo())
                .field("atim1oc4", &self.atim1oc4())
                .field("atim1oc5", &self.atim1oc5())
                .field("atim1oc6", &self.atim1oc6())
                .field("gtim1trgo", &self.gtim1trgo())
                .field("gtim1oc1", &self.gtim1oc1())
                .field("gtim1oc2", &self.gtim1oc2())
                .field("gtim2trgo", &self.gtim2trgo())
                .field("gtim2oc1", &self.gtim2oc1())
                .field("gtim2oc2", &self.gtim2oc2())
                .field("gtim3trgo", &self.gtim3trgo())
                .field("gtim3oc1", &self.gtim3oc1())
                .field("gtim3oc2", &self.gtim3oc2())
                .field("gtim4trgo", &self.gtim4trgo())
                .field("gtim4oc1", &self.gtim4oc1())
                .field("gtim4oc2", &self.gtim4oc2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cal {{ calen: {=bool:?}, calmode: {:?}, start: {=bool:?}, softtrig: {=bool:?}, calperiod: {=u8:?}, azrun: {=bool:?}, clkdiv: {=u8:?}, adc1start1: {=bool:?}, adc2start1: {=bool:?}, atim1trgo: {=bool:?}, atim1oc4: {=bool:?}, atim1oc5: {=bool:?}, atim1oc6: {=bool:?}, gtim1trgo: {=bool:?}, gtim1oc1: {=bool:?}, gtim1oc2: {=bool:?}, gtim2trgo: {=bool:?}, gtim2oc1: {=bool:?}, gtim2oc2: {=bool:?}, gtim3trgo: {=bool:?}, gtim3oc1: {=bool:?}, gtim3oc2: {=bool:?}, gtim4trgo: {=bool:?}, gtim4oc1: {=bool:?}, gtim4oc2: {=bool:?} }}",
                self.calen(),
                self.calmode(),
                self.start(),
                self.softtrig(),
                self.calperiod(),
                self.azrun(),
                self.clkdiv(),
                self.adc1start1(),
                self.adc2start1(),
                self.atim1trgo(),
                self.atim1oc4(),
                self.atim1oc5(),
                self.atim1oc6(),
                self.gtim1trgo(),
                self.gtim1oc1(),
                self.gtim1oc2(),
                self.gtim2trgo(),
                self.gtim2oc1(),
                self.gtim2oc2(),
                self.gtim3trgo(),
                self.gtim3oc1(),
                self.gtim3oc2(),
                self.gtim4trgo(),
                self.gtim4oc1(),
                self.gtim4oc2()
            )
        }
    }
    #[doc = "Operational amplifier control; reset 0x0000E000 per CN manual V1.4 §29.6.1."]
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
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "desc INP1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inp1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc INP1EN."]
        #[inline(always)]
        pub const fn set_inp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INP2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inp2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INP2EN."]
        #[inline(always)]
        pub const fn set_inp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INP3EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inp3en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc INP3EN."]
        #[inline(always)]
        pub const fn set_inp3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc INP4EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inp4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc INP4EN."]
        #[inline(always)]
        pub const fn set_inp4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc INN1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inn1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc INN1EN."]
        #[inline(always)]
        pub const fn set_inn1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc INN2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn inn2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc INN2EN."]
        #[inline(always)]
        pub const fn set_inn2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc AMP."]
        #[must_use]
        #[inline(always)]
        pub const fn amp(&self) -> super::vals::Gain {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Gain::from_bits(val as u8)
        }
        #[doc = "desc AMP."]
        #[inline(always)]
        pub const fn set_amp(&mut self, val: super::vals::Gain) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> super::vals::Bias {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Bias::from_bits(val as u8)
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: super::vals::Bias) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
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
                .field("mode", &self.mode())
                .field("inp1en", &self.inp1en())
                .field("inp2en", &self.inp2en())
                .field("inp3en", &self.inp3en())
                .field("inp4en", &self.inp4en())
                .field("inn1en", &self.inn1en())
                .field("inn2en", &self.inn2en())
                .field("amp", &self.amp())
                .field("bias", &self.bias())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, mode: {:?}, inp1en: {=bool:?}, inp2en: {=bool:?}, inp3en: {=bool:?}, inp4en: {=bool:?}, inn1en: {=bool:?}, inn2en: {=bool:?}, amp: {:?}, bias: {:?} }}",
                self.en(),
                self.mode(),
                self.inp1en(),
                self.inp2en(),
                self.inp3en(),
                self.inp4en(),
                self.inn1en(),
                self.inn2en(),
                self.amp(),
                self.bias()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bias {
        #[doc = "1 microampere bias current."]
        Ua1 = 0x0,
        #[doc = "2 microampere bias current."]
        Ua2 = 0x01,
        #[doc = "3 microampere bias current."]
        Ua3 = 0x02,
        #[doc = "4 microampere bias current."]
        Ua4 = 0x03,
        #[doc = "5 microampere bias current."]
        Ua5 = 0x04,
        #[doc = "6 microampere bias current."]
        Ua6 = 0x05,
        #[doc = "7 microampere bias current."]
        Ua7 = 0x06,
        #[doc = "8 microampere bias current."]
        Ua8 = 0x07,
    }
    impl Bias {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bias {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum CalibrationMode {
        #[doc = "Software or hardware triggered calibration."]
        Triggered = 0x0,
        #[doc = "Level calibration controlled by START."]
        Level = 0x01,
    }
    impl CalibrationMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CalibrationMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CalibrationMode {
        #[inline(always)]
        fn from(val: u8) -> CalibrationMode {
            CalibrationMode::from_bits(val)
        }
    }
    impl From<CalibrationMode> for u8 {
        #[inline(always)]
        fn from(val: CalibrationMode) -> u8 {
            CalibrationMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Gain {
        #[doc = "Gain 2."]
        Mul2 = 0x0,
        #[doc = "Gain 4."]
        Mul4 = 0x01,
        #[doc = "Gain 8."]
        Mul8 = 0x02,
        #[doc = "Gain 16."]
        Mul16 = 0x03,
        #[doc = "Gain 32."]
        Mul32 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Gain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gain {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gain {
        #[inline(always)]
        fn from(val: u8) -> Gain {
            Gain::from_bits(val)
        }
    }
    impl From<Gain> for u8 {
        #[inline(always)]
        fn from(val: Gain) -> u8 {
            Gain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "External feedback network."]
        External = 0x0,
        #[doc = "Alternative documented external feedback encoding."]
        ExternalAlt = 0x01,
        #[doc = "Internal positive gain network."]
        Pga = 0x02,
        #[doc = "Internal voltage follower."]
        Follower = 0x03,
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
