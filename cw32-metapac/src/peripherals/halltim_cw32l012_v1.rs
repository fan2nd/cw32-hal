#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CW32L012 24-bit Hall sensor timer; CN manual V1.4 section 18."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halltim {
    ptr: *mut u8,
}
unsafe impl Send for Halltim {}
unsafe impl Sync for Halltim {}
impl Halltim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA and Interrupt register2."]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "counter register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Auto Reload Register."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Width Register."]
    #[inline(always)]
    pub const fn width(self) -> crate::common::Reg<regs::Width, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Compare control Register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt states register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt clean register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "State register."]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Auto Reload Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "24-bit ARR value."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "24-bit ARR value."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
            f.debug_struct("Arr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Arr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Arr {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Compare control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "24-bit CCR value."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "24-bit CCR value."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "24-bit counter. Write zero only; any filtered input edge captures and clears it."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "24-bit counter. Write zero only; any filtered input edge captures and clears it."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "24-bit counter. Write zero only; any filtered input edge captures and clears it."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
            f.debug_struct("Cnt").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Control register. SOFTCAP is a write-one command that reads zero."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Hall timer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Hall timer enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable first-stage 5/7 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable first-stage 5/7 filter."]
        #[inline(always)]
        pub const fn set_flt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PCLK divisor; CN V1.4 table 18-1 and section 18.8.1."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> super::vals::Prescaler {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Prescaler::from_bits(val as u8)
        }
        #[doc = "PCLK divisor; CN V1.4 table 18-1 and section 18.8.1."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: super::vals::Prescaler) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Local HALLTIM_TRGO waveform only; downstream routing is not implied."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::MasterMode {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::MasterMode::from_bits(val as u8)
        }
        #[doc = "Local HALLTIM_TRGO waveform only; downstream routing is not implied."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::MasterMode) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Filter rejects pulses shorter than FLT2LEN ticks; pulses at least FLT2LEN+2 ticks pass."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2len(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x7fff;
            val as u16
        }
        #[doc = "Filter rejects pulses shorter than FLT2LEN ticks; pulses at least FLT2LEN+2 ticks pass."]
        #[inline(always)]
        pub const fn set_flt2len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 8usize)) | (((val as u32) & 0x7fff) << 8usize);
        }
        #[doc = "Write one generates a software trigger; reads zero. HAL does not infer that it captures WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn softcap(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Write one generates a software trigger; reads zero. HAL does not infer that it captures WIDTH."]
        #[inline(always)]
        pub const fn set_softcap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("flt1en", &self.flt1en())
                .field("div", &self.div())
                .field("mms", &self.mms())
                .field("flt2len", &self.flt2len())
                .field("softcap", &self.softcap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, flt1en: {=bool:?}, div: {:?}, mms: {:?}, flt2len: {=u16:?}, softcap: {=bool:?} }}",
                self.en(),
                self.flt1en(),
                self.div(),
                self.mms(),
                self.flt2len(),
                self.softcap()
            )
        }
    }
    #[doc = "DMA and Interrupt register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier(pub u32);
    impl Dier {
        #[doc = "desc CAPIE."]
        #[must_use]
        #[inline(always)]
        pub const fn capie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPIE."]
        #[inline(always)]
        pub const fn set_capie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OVIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ovie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVIE."]
        #[inline(always)]
        pub const fn set_ovie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc MATCHIE."]
        #[must_use]
        #[inline(always)]
        pub const fn matchie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCHIE."]
        #[inline(always)]
        pub const fn set_matchie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CAPDMA."]
        #[must_use]
        #[inline(always)]
        pub const fn capdma(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPDMA."]
        #[inline(always)]
        pub const fn set_capdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OVDMA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovdma(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVDMA."]
        #[inline(always)]
        pub const fn set_ovdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc MATCHDMA."]
        #[must_use]
        #[inline(always)]
        pub const fn matchdma(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCHDMA."]
        #[inline(always)]
        pub const fn set_matchdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SOFTDMA."]
        #[must_use]
        #[inline(always)]
        pub const fn softdma(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTDMA."]
        #[inline(always)]
        pub const fn set_softdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Dier {
        #[inline(always)]
        fn default() -> Dier {
            Dier(0)
        }
    }
    impl core::fmt::Debug for Dier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier")
                .field("capie", &self.capie())
                .field("ovie", &self.ovie())
                .field("matchie", &self.matchie())
                .field("capdma", &self.capdma())
                .field("ovdma", &self.ovdma())
                .field("matchdma", &self.matchdma())
                .field("softdma", &self.softdma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier {{ capie: {=bool:?}, ovie: {=bool:?}, matchie: {=bool:?}, capdma: {=bool:?}, ovdma: {=bool:?}, matchdma: {=bool:?}, softdma: {=bool:?} }}",
                self.capie(),
                self.ovie(),
                self.matchie(),
                self.capdma(),
                self.ovdma(),
                self.matchdma(),
                self.softdma()
            )
        }
    }
    #[doc = "R1W0 flags. Start from write_noop(), clear intended fields, and write once."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Input capture flag; write zero to clear, one leaves unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn capf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input capture flag; write zero to clear, one leaves unchanged."]
        #[inline(always)]
        pub const fn set_capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter overflow flag; write zero to clear, one leaves unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter overflow flag; write zero to clear, one leaves unchanged."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counter compare match flag; write zero to clear, one leaves unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn matchf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Counter compare match flag; write zero to clear, one leaves unchanged."]
        #[inline(always)]
        pub const fn set_matchf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("capf", &self.capf())
                .field("ovf", &self.ovf())
                .field("matchf", &self.matchf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ capf: {=bool:?}, ovf: {=bool:?}, matchf: {=bool:?} }}",
                self.capf(),
                self.ovf(),
                self.matchf()
            )
        }
    }
    #[doc = "Read-only, coalescing capture, overflow and match flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Input capture flag; read only."]
        #[must_use]
        #[inline(always)]
        pub const fn capf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input capture flag; read only."]
        #[inline(always)]
        pub const fn set_capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter overflow flag; read only."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter overflow flag; read only."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counter compare match flag; read only."]
        #[must_use]
        #[inline(always)]
        pub const fn matchf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Counter compare match flag; read only."]
        #[inline(always)]
        pub const fn set_matchf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("capf", &self.capf())
                .field("ovf", &self.ovf())
                .field("matchf", &self.matchf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ capf: {=bool:?}, ovf: {=bool:?}, matchf: {=bool:?} }}",
                self.capf(),
                self.ovf(),
                self.matchf()
            )
        }
    }
    #[doc = "Live raw and filtered inputs, not a capture-latched state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State(pub u32);
    impl State {
        #[doc = "desc CH1F."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH1F."]
        #[inline(always)]
        pub const fn set_ch1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CH2F."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH2F."]
        #[inline(always)]
        pub const fn set_ch2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CH3F."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH3F."]
        #[inline(always)]
        pub const fn set_ch3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CH1S."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1s(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH1S."]
        #[inline(always)]
        pub const fn set_ch1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CH2S."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2s(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH2S."]
        #[inline(always)]
        pub const fn set_ch2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CH3S."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3s(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CH3S."]
        #[inline(always)]
        pub const fn set_ch3s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for State {
        #[inline(always)]
        fn default() -> State {
            State(0)
        }
    }
    impl core::fmt::Debug for State {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("State")
                .field("ch1f", &self.ch1f())
                .field("ch2f", &self.ch2f())
                .field("ch3f", &self.ch3f())
                .field("ch1s", &self.ch1s())
                .field("ch2s", &self.ch2s())
                .field("ch3s", &self.ch3s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for State {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "State {{ ch1f: {=bool:?}, ch2f: {=bool:?}, ch3f: {=bool:?}, ch1s: {=bool:?}, ch2s: {=bool:?}, ch3s: {=bool:?} }}",
                self.ch1f(),
                self.ch2f(),
                self.ch3f(),
                self.ch1s(),
                self.ch2s(),
                self.ch3s()
            )
        }
    }
    #[doc = "Latest captured counter value; overwritten on every filtered input transition. No FIFO or overcapture flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Width(pub u32);
    impl Width {
        #[doc = "Latest captured counter value; overwritten on every filtered input transition. No FIFO or overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Latest captured counter value; overwritten on every filtered input transition. No FIFO or overcapture flag."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Width {
        #[inline(always)]
        fn default() -> Width {
            Width(0)
        }
    }
    impl core::fmt::Debug for Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Width").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Width {{ data: {=u32:?} }}", self.data())
        }
    }
}
pub mod vals {
    #[doc = "Local TRGO output selection; own L012 CN V1.4 pp418,421. No consumer route implied."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MasterMode {
        #[doc = "Overflow pulse."]
        Overflow = 0x0,
        #[doc = "PWM signal."]
        Pwm = 0x01,
        #[doc = "Compare match pulse."]
        Match = 0x02,
        #[doc = "Filtered Hall input change pulse."]
        Capture = 0x03,
        #[doc = "Filtered channel 3."]
        Ch3 = 0x04,
        #[doc = "Filtered channel 2."]
        Ch2 = 0x05,
        #[doc = "Filtered channel 1."]
        Ch1 = 0x06,
        #[doc = "XOR of the three filtered inputs."]
        Xor = 0x07,
    }
    impl MasterMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MasterMode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MasterMode {
        #[inline(always)]
        fn from(val: u8) -> MasterMode {
            MasterMode::from_bits(val)
        }
    }
    impl From<MasterMode> for u8 {
        #[inline(always)]
        fn from(val: MasterMode) -> u8 {
            MasterMode::to_bits(val)
        }
    }
    #[doc = "PCLK divider; own L012 CN V1.4 pp415,421."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prescaler {
        #[doc = "PCLK divided by 1."]
        Div1 = 0x0,
        #[doc = "PCLK divided by 2."]
        Div2 = 0x01,
        #[doc = "PCLK divided by 4."]
        Div4 = 0x02,
        #[doc = "PCLK divided by 8."]
        Div8 = 0x03,
    }
    impl Prescaler {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prescaler {
            unsafe { core::mem::transmute(val & 0x03) }
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
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(65535)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(65535)
    }
}
