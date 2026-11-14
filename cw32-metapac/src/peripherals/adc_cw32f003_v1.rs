#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc START."]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc SQR."]
    #[inline(always)]
    pub const fn sqr(self) -> crate::common::Reg<regs::Sqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control register2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc VTH."]
    #[inline(always)]
    pub const fn vth(self) -> crate::common::Reg<regs::Vth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc VTL."]
    #[inline(always)]
    pub const fn vtl(self) -> crate::common::Reg<regs::Vtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc TRIGGER."]
    #[inline(always)]
    pub const fn trigger(self) -> crate::common::Reg<regs::Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Ordered scan conversion results, indexed by sequence slot."]
    #[inline(always)]
    pub const fn result(self, n: usize) -> crate::common::Reg<regs::Result, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "desc RESULTACC."]
    #[inline(always)]
    pub const fn resultacc(self) -> crate::common::Reg<regs::Resultacc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
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
        #[doc = "Conversion mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Conversion mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "desc BGREN."]
        #[must_use]
        #[inline(always)]
        pub const fn bgren(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGREN."]
        #[inline(always)]
        pub const fn set_bgren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSEN."]
        #[inline(always)]
        pub const fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc REF."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc REF."]
        #[inline(always)]
        pub const fn set_ref_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc CLK."]
        #[must_use]
        #[inline(always)]
        pub const fn clk(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc CLK."]
        #[inline(always)]
        pub const fn set_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc SAM."]
        #[must_use]
        #[inline(always)]
        pub const fn sam(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "desc SAM."]
        #[inline(always)]
        pub const fn set_sam(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "desc BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn buf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUF."]
        #[inline(always)]
        pub const fn set_buf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
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
                .field("bgren", &self.bgren())
                .field("tsen", &self.tsen())
                .field("ref_", &self.ref_())
                .field("clk", &self.clk())
                .field("sam", &self.sam())
                .field("buf", &self.buf())
                .field("bias", &self.bias())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr0 {{ en: {=bool:?}, mode: {:?}, bgren: {=bool:?}, tsen: {=bool:?}, ref_: {=u8:?}, clk: {=u8:?}, sam: {=u8:?}, buf: {=bool:?}, bias: {=u8:?} }}",
                self.en(),
                self.mode(),
                self.bgren(),
                self.tsen(),
                self.ref_(),
                self.clk(),
                self.sam(),
                self.buf(),
                self.bias()
            )
        }
    }
    #[doc = "Control register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc CHMUX."]
        #[must_use]
        #[inline(always)]
        pub const fn chmux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CHMUX."]
        #[inline(always)]
        pub const fn set_chmux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DISCARD."]
        #[must_use]
        #[inline(always)]
        pub const fn discard(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DISCARD."]
        #[inline(always)]
        pub const fn set_discard(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ALIGN."]
        #[must_use]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALIGN."]
        #[inline(always)]
        pub const fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc WDTCH."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc WDTCH."]
        #[inline(always)]
        pub const fn set_wdtch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc WDTALL."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtall(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTALL."]
        #[inline(always)]
        pub const fn set_wdtall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("chmux", &self.chmux())
                .field("discard", &self.discard())
                .field("align", &self.align())
                .field("wdtch", &self.wdtch())
                .field("wdtall", &self.wdtall())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ chmux: {=u8:?}, discard: {=bool:?}, align: {=bool:?}, wdtch: {=u8:?}, wdtall: {=bool:?} }}",
                self.chmux(),
                self.discard(),
                self.align(),
                self.wdtch(),
                self.wdtall()
            )
        }
    }
    #[doc = "Control register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ACCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn accen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACCEN."]
        #[inline(always)]
        pub const fn set_accen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ACCRST."]
        #[must_use]
        #[inline(always)]
        pub const fn accrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACCRST."]
        #[inline(always)]
        pub const fn set_accrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("cnt", &self.cnt())
                .field("accen", &self.accen())
                .field("accrst", &self.accrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ cnt: {=u8:?}, accen: {=bool:?}, accrst: {=bool:?} }}",
                self.cnt(),
                self.accen(),
                self.accrst()
            )
        }
    }
    #[doc = "Interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc EOC."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOC."]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EOS."]
        #[must_use]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOS."]
        #[inline(always)]
        pub const fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EOA."]
        #[must_use]
        #[inline(always)]
        pub const fn eoa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOA."]
        #[inline(always)]
        pub const fn set_eoa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc WDTL."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTL."]
        #[inline(always)]
        pub const fn set_wdtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc WDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn wdth(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTH."]
        #[inline(always)]
        pub const fn set_wdth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WDTR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTR."]
        #[inline(always)]
        pub const fn set_wdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OVW."]
        #[must_use]
        #[inline(always)]
        pub const fn ovw(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVW."]
        #[inline(always)]
        pub const fn set_ovw(&mut self, val: bool) {
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
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("eoa", &self.eoa())
                .field("wdtl", &self.wdtl())
                .field("wdth", &self.wdth())
                .field("wdtr", &self.wdtr())
                .field("ovw", &self.ovw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ eoc: {=bool:?}, eos: {=bool:?}, eoa: {=bool:?}, wdtl: {=bool:?}, wdth: {=bool:?}, wdtr: {=bool:?}, ovw: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.eoa(),
                self.wdtl(),
                self.wdth(),
                self.wdtr(),
                self.ovw()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc EOC."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOC."]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EOS."]
        #[must_use]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOS."]
        #[inline(always)]
        pub const fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EOA."]
        #[must_use]
        #[inline(always)]
        pub const fn eoa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOA."]
        #[inline(always)]
        pub const fn set_eoa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc WDTL."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTL."]
        #[inline(always)]
        pub const fn set_wdtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc WDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn wdth(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTH."]
        #[inline(always)]
        pub const fn set_wdth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WDTR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTR."]
        #[inline(always)]
        pub const fn set_wdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OVW."]
        #[must_use]
        #[inline(always)]
        pub const fn ovw(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVW."]
        #[inline(always)]
        pub const fn set_ovw(&mut self, val: bool) {
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
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("eoa", &self.eoa())
                .field("wdtl", &self.wdtl())
                .field("wdth", &self.wdth())
                .field("wdtr", &self.wdtr())
                .field("ovw", &self.ovw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ eoc: {=bool:?}, eos: {=bool:?}, eoa: {=bool:?}, wdtl: {=bool:?}, wdth: {=bool:?}, wdtr: {=bool:?}, ovw: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.eoa(),
                self.wdtl(),
                self.wdth(),
                self.wdtr(),
                self.ovw()
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc EOC."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOC."]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EOS."]
        #[must_use]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOS."]
        #[inline(always)]
        pub const fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EOA."]
        #[must_use]
        #[inline(always)]
        pub const fn eoa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EOA."]
        #[inline(always)]
        pub const fn set_eoa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc WDTL."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTL."]
        #[inline(always)]
        pub const fn set_wdtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc WDTH."]
        #[must_use]
        #[inline(always)]
        pub const fn wdth(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTH."]
        #[inline(always)]
        pub const fn set_wdth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WDTR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTR."]
        #[inline(always)]
        pub const fn set_wdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OVW."]
        #[must_use]
        #[inline(always)]
        pub const fn ovw(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVW."]
        #[inline(always)]
        pub const fn set_ovw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc READY."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc READY."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("eoa", &self.eoa())
                .field("wdtl", &self.wdtl())
                .field("wdth", &self.wdth())
                .field("wdtr", &self.wdtr())
                .field("ovw", &self.ovw())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ eoc: {=bool:?}, eos: {=bool:?}, eoa: {=bool:?}, wdtl: {=bool:?}, wdth: {=bool:?}, wdtr: {=bool:?}, ovw: {=bool:?}, ready: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.eoa(),
                self.wdtl(),
                self.wdth(),
                self.wdtr(),
                self.ovw(),
                self.ready()
            )
        }
    }
    #[doc = "Ordered scan conversion result, right-aligned when CR1.ALIGN is zero."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result(pub u32);
    impl Result {
        #[doc = "Twelve-bit conversion result."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Twelve-bit conversion result."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Result {
        #[inline(always)]
        fn default() -> Result {
            Result(0)
        }
    }
    impl core::fmt::Debug for Result {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result")
                .field("result", &self.result())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Result {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Result {{ result: {=u16:?} }}", self.result())
        }
    }
    #[doc = "desc RESULTACC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resultacc(pub u32);
    impl Resultacc {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Resultacc {
        #[inline(always)]
        fn default() -> Resultacc {
            Resultacc(0)
        }
    }
    impl core::fmt::Debug for Resultacc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Resultacc")
                .field("result", &self.result())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Resultacc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Resultacc {{ result: {=u32:?} }}", self.result())
        }
    }
    #[doc = "desc SQR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr(pub u32);
    impl Sqr {
        #[doc = "Hardware input mux for this ordered scan slot."]
        #[must_use]
        #[inline(always)]
        pub const fn sqr(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Hardware input mux for this ordered scan slot."]
        #[inline(always)]
        pub const fn set_sqr(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Sqr {
        #[inline(always)]
        fn default() -> Sqr {
            Sqr(0)
        }
    }
    impl core::fmt::Debug for Sqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr")
                .field("sqr[0]", &self.sqr(0usize))
                .field("sqr[1]", &self.sqr(1usize))
                .field("sqr[2]", &self.sqr(2usize))
                .field("sqr[3]", &self.sqr(3usize))
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr {{ sqr[0]: {=u8:?}, sqr[1]: {=u8:?}, sqr[2]: {=u8:?}, sqr[3]: {=u8:?}, ens: {=u8:?} }}",
                self.sqr(0usize),
                self.sqr(1usize),
                self.sqr(2usize),
                self.sqr(3usize),
                self.ens()
            )
        }
    }
    #[doc = "desc START."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Start(pub u32);
    impl Start {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc AUTOSTOP."]
        #[must_use]
        #[inline(always)]
        pub const fn autostop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTOSTOP."]
        #[inline(always)]
        pub const fn set_autostop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Start {
        #[inline(always)]
        fn default() -> Start {
            Start(0)
        }
    }
    impl core::fmt::Debug for Start {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Start")
                .field("start", &self.start())
                .field("autostop", &self.autostop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Start {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Start {{ start: {=bool:?}, autostop: {=bool:?} }}",
                self.start(),
                self.autostop()
            )
        }
    }
    #[doc = "desc TRIGGER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigger(pub u32);
    impl Trigger {
        #[doc = "desc ATIM."]
        #[must_use]
        #[inline(always)]
        pub const fn atim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM."]
        #[inline(always)]
        pub const fn set_atim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc GTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM."]
        #[inline(always)]
        pub const fn set_gtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA32."]
        #[must_use]
        #[inline(always)]
        pub const fn pa32(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA32."]
        #[inline(always)]
        pub const fn set_pa32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA54."]
        #[must_use]
        #[inline(always)]
        pub const fn pa54(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA54."]
        #[inline(always)]
        pub const fn set_pa54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn btim1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM1."]
        #[inline(always)]
        pub const fn set_btim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn btim2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM2."]
        #[inline(always)]
        pub const fn set_btim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BTIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn btim3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM3."]
        #[inline(always)]
        pub const fn set_btim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA76."]
        #[must_use]
        #[inline(always)]
        pub const fn pa76(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA76."]
        #[inline(always)]
        pub const fn set_pa76(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI."]
        #[inline(always)]
        pub const fn set_spi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc I2C."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C."]
        #[inline(always)]
        pub const fn set_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB32."]
        #[must_use]
        #[inline(always)]
        pub const fn pb32(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB32."]
        #[inline(always)]
        pub const fn set_pb32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB74."]
        #[must_use]
        #[inline(always)]
        pub const fn pb74(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB74."]
        #[inline(always)]
        pub const fn set_pb74(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Trigger {
        #[inline(always)]
        fn default() -> Trigger {
            Trigger(0)
        }
    }
    impl core::fmt::Debug for Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trigger")
                .field("atim", &self.atim())
                .field("gtim", &self.gtim())
                .field("pa10", &self.pa10())
                .field("pa32", &self.pa32())
                .field("pa54", &self.pa54())
                .field("btim1", &self.btim1())
                .field("btim2", &self.btim2())
                .field("btim3", &self.btim3())
                .field("uart1", &self.uart1())
                .field("uart2", &self.uart2())
                .field("pa76", &self.pa76())
                .field("spi", &self.spi())
                .field("pb10", &self.pb10())
                .field("i2c", &self.i2c())
                .field("pb32", &self.pb32())
                .field("pb74", &self.pb74())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trigger {{ atim: {=bool:?}, gtim: {=bool:?}, pa10: {=bool:?}, pa32: {=bool:?}, pa54: {=bool:?}, btim1: {=bool:?}, btim2: {=bool:?}, btim3: {=bool:?}, uart1: {=bool:?}, uart2: {=bool:?}, pa76: {=bool:?}, spi: {=bool:?}, pb10: {=bool:?}, i2c: {=bool:?}, pb32: {=bool:?}, pb74: {=bool:?} }}",
                self.atim(),
                self.gtim(),
                self.pa10(),
                self.pa32(),
                self.pa54(),
                self.btim1(),
                self.btim2(),
                self.btim3(),
                self.uart1(),
                self.uart2(),
                self.pa76(),
                self.spi(),
                self.pb10(),
                self.i2c(),
                self.pb32(),
                self.pb74()
            )
        }
    }
    #[doc = "desc VTH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vth(pub u32);
    impl Vth {
        #[doc = "desc VTH."]
        #[must_use]
        #[inline(always)]
        pub const fn vth(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc VTH."]
        #[inline(always)]
        pub const fn set_vth(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vth {
        #[inline(always)]
        fn default() -> Vth {
            Vth(0)
        }
    }
    impl core::fmt::Debug for Vth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vth").field("vth", &self.vth()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vth {{ vth: {=u16:?} }}", self.vth())
        }
    }
    #[doc = "desc VTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtl(pub u32);
    impl Vtl {
        #[doc = "desc VTL."]
        #[must_use]
        #[inline(always)]
        pub const fn vtl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc VTL."]
        #[inline(always)]
        pub const fn set_vtl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vtl {
        #[inline(always)]
        fn default() -> Vtl {
            Vtl(0)
        }
    }
    impl core::fmt::Debug for Vtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vtl").field("vtl", &self.vtl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vtl {{ vtl: {=u16:?} }}", self.vtl())
        }
    }
}
pub mod vals {
    #[doc = "Source-qualified software-triggered modes; other conversion modes are outside this subset."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "One conversion of CR1.CHMUX, stored in RESULT0."]
        Single = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Repeated CHMUX conversions into RESULT0 until START is cleared; own-family evidence in docs/adc-classic-continuous.md."]
        Continuous = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "One pass through SQR slots, with EOS and automatic START clear after the final result."]
        Scan = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
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
