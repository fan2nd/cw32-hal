#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SPI1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
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
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Slave slect register."]
    #[inline(always)]
    pub const fn ssi(self) -> crate::common::Reg<regs::Ssi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc CPHA."]
        #[must_use]
        #[inline(always)]
        pub const fn cpha(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPHA."]
        #[inline(always)]
        pub const fn set_cpha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn cpol(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPOL."]
        #[inline(always)]
        pub const fn set_cpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc MSTR."]
        #[must_use]
        #[inline(always)]
        pub const fn mstr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSTR."]
        #[inline(always)]
        pub const fn set_mstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc BR."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc BR."]
        #[inline(always)]
        pub const fn set_br(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
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
        #[doc = "desc LSBF."]
        #[must_use]
        #[inline(always)]
        pub const fn lsbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LSBF."]
        #[inline(always)]
        pub const fn set_lsbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SMP."]
        #[must_use]
        #[inline(always)]
        pub const fn smp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMP."]
        #[inline(always)]
        pub const fn set_smp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SSM."]
        #[must_use]
        #[inline(always)]
        pub const fn ssm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSM."]
        #[inline(always)]
        pub const fn set_ssm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc WIDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "desc WIDTH."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc DMARX."]
        #[must_use]
        #[inline(always)]
        pub const fn dmarx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMARX."]
        #[inline(always)]
        pub const fn set_dmarx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DMATX."]
        #[must_use]
        #[inline(always)]
        pub const fn dmatx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMATX."]
        #[inline(always)]
        pub const fn set_dmatx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc MISOHD."]
        #[must_use]
        #[inline(always)]
        pub const fn misohd(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc MISOHD."]
        #[inline(always)]
        pub const fn set_misohd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("mstr", &self.mstr())
                .field("br", &self.br())
                .field("en", &self.en())
                .field("lsbf", &self.lsbf())
                .field("smp", &self.smp())
                .field("ssm", &self.ssm())
                .field("width", &self.width())
                .field("mode", &self.mode())
                .field("dmarx", &self.dmarx())
                .field("dmatx", &self.dmatx())
                .field("misohd", &self.misohd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ cpha: {=bool:?}, cpol: {=bool:?}, mstr: {=bool:?}, br: {=u8:?}, en: {=bool:?}, lsbf: {=bool:?}, smp: {=bool:?}, ssm: {=bool:?}, width: {=u8:?}, mode: {=u8:?}, dmarx: {=bool:?}, dmatx: {=bool:?}, misohd: {=bool:?} }}",
                self.cpha(),
                self.cpol(),
                self.mstr(),
                self.br(),
                self.en(),
                self.lsbf(),
                self.smp(),
                self.ssm(),
                self.width(),
                self.mode(),
                self.dmarx(),
                self.dmatx(),
                self.misohd()
            )
        }
    }
    #[doc = "Control register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc HDOE."]
        #[must_use]
        #[inline(always)]
        pub const fn hdoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HDOE."]
        #[inline(always)]
        pub const fn set_hdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cr2").field("hdoe", &self.hdoe()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr2 {{ hdoe: {=bool:?} }}", self.hdoe())
        }
    }
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "desc DR."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DR."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ dr: {=u16:?} }}", self.dr())
        }
    }
    #[doc = "Interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc FLUSH."]
        #[must_use]
        #[inline(always)]
        pub const fn flush(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLUSH."]
        #[inline(always)]
        pub const fn set_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSF."]
        #[must_use]
        #[inline(always)]
        pub const fn ssf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSF."]
        #[inline(always)]
        pub const fn set_ssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ssr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSR."]
        #[inline(always)]
        pub const fn set_ssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sserr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERR."]
        #[inline(always)]
        pub const fn set_sserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODF."]
        #[must_use]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODF."]
        #[inline(always)]
        pub const fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("flush", &self.flush())
                .field("rxne", &self.rxne())
                .field("ssf", &self.ssf())
                .field("ssr", &self.ssr())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("sserr", &self.sserr())
                .field("modf", &self.modf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ flush: {=bool:?}, rxne: {=bool:?}, ssf: {=bool:?}, ssr: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, sserr: {=bool:?}, modf: {=bool:?} }}",
                self.flush(),
                self.rxne(),
                self.ssf(),
                self.ssr(),
                self.ud(),
                self.ov(),
                self.sserr(),
                self.modf()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSF."]
        #[must_use]
        #[inline(always)]
        pub const fn ssf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSF."]
        #[inline(always)]
        pub const fn set_ssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ssr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSR."]
        #[inline(always)]
        pub const fn set_ssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sserr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERR."]
        #[inline(always)]
        pub const fn set_sserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODF."]
        #[must_use]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODF."]
        #[inline(always)]
        pub const fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("ssf", &self.ssf())
                .field("ssr", &self.ssr())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("sserr", &self.sserr())
                .field("modf", &self.modf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ txe: {=bool:?}, rxne: {=bool:?}, ssf: {=bool:?}, ssr: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, sserr: {=bool:?}, modf: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.ssf(),
                self.ssr(),
                self.ud(),
                self.ov(),
                self.sserr(),
                self.modf()
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSF."]
        #[must_use]
        #[inline(always)]
        pub const fn ssf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSF."]
        #[inline(always)]
        pub const fn set_ssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSR."]
        #[must_use]
        #[inline(always)]
        pub const fn ssr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSR."]
        #[inline(always)]
        pub const fn set_ssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc UD."]
        #[must_use]
        #[inline(always)]
        pub const fn ud(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc UD."]
        #[inline(always)]
        pub const fn set_ud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OV."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OV."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sserr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSERR."]
        #[inline(always)]
        pub const fn set_sserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MODF."]
        #[must_use]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODF."]
        #[inline(always)]
        pub const fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SSLVL."]
        #[must_use]
        #[inline(always)]
        pub const fn sslvl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSLVL."]
        #[inline(always)]
        pub const fn set_sslvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("ssf", &self.ssf())
                .field("ssr", &self.ssr())
                .field("ud", &self.ud())
                .field("ov", &self.ov())
                .field("sserr", &self.sserr())
                .field("modf", &self.modf())
                .field("busy", &self.busy())
                .field("sslvl", &self.sslvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ txe: {=bool:?}, rxne: {=bool:?}, ssf: {=bool:?}, ssr: {=bool:?}, ud: {=bool:?}, ov: {=bool:?}, sserr: {=bool:?}, modf: {=bool:?}, busy: {=bool:?}, sslvl: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.ssf(),
                self.ssr(),
                self.ud(),
                self.ov(),
                self.sserr(),
                self.modf(),
                self.busy(),
                self.sslvl()
            )
        }
    }
    #[doc = "Slave slect register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssi(pub u32);
    impl Ssi {
        #[doc = "desc SSI."]
        #[must_use]
        #[inline(always)]
        pub const fn ssi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSI."]
        #[inline(always)]
        pub const fn set_ssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ssi {
        #[inline(always)]
        fn default() -> Ssi {
            Ssi(0)
        }
    }
    impl core::fmt::Debug for Ssi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssi").field("ssi", &self.ssi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ssi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ssi {{ ssi: {=bool:?} }}", self.ssi())
        }
    }
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(255)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(255)
    }
}
