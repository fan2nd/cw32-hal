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
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc START."]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc AWDTR."]
    #[inline(always)]
    pub const fn awdtr(self) -> crate::common::Reg<regs::Awdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc TRIGGER."]
    #[inline(always)]
    pub const fn trigger(self) -> crate::common::Reg<regs::Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc AWDCR."]
    #[inline(always)]
    pub const fn awdcr(self) -> crate::common::Reg<regs::Awdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc SAMPLE."]
    #[inline(always)]
    pub const fn sample(self) -> crate::common::Reg<regs::Sample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc SQRCFR."]
    #[inline(always)]
    pub const fn sqrcfr(self) -> crate::common::Reg<regs::Sqrcfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Right-aligned conversion result for sequence slot n."]
    #[inline(always)]
    pub const fn result(self, n: usize) -> crate::common::Reg<regs::Result, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc AWDCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdcr(pub u32);
    impl Awdcr {
        #[doc = "desc IN0."]
        #[must_use]
        #[inline(always)]
        pub const fn in0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN0."]
        #[inline(always)]
        pub const fn set_in0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IN1."]
        #[must_use]
        #[inline(always)]
        pub const fn in1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN1."]
        #[inline(always)]
        pub const fn set_in1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc IN2."]
        #[must_use]
        #[inline(always)]
        pub const fn in2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN2."]
        #[inline(always)]
        pub const fn set_in2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IN3."]
        #[must_use]
        #[inline(always)]
        pub const fn in3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN3."]
        #[inline(always)]
        pub const fn set_in3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IN4."]
        #[must_use]
        #[inline(always)]
        pub const fn in4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN4."]
        #[inline(always)]
        pub const fn set_in4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IN5."]
        #[must_use]
        #[inline(always)]
        pub const fn in5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN5."]
        #[inline(always)]
        pub const fn set_in5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc IN6."]
        #[must_use]
        #[inline(always)]
        pub const fn in6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN6."]
        #[inline(always)]
        pub const fn set_in6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc IN7."]
        #[must_use]
        #[inline(always)]
        pub const fn in7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN7."]
        #[inline(always)]
        pub const fn set_in7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc IN8."]
        #[must_use]
        #[inline(always)]
        pub const fn in8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN8."]
        #[inline(always)]
        pub const fn set_in8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IN9."]
        #[must_use]
        #[inline(always)]
        pub const fn in9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN9."]
        #[inline(always)]
        pub const fn set_in9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc IN10."]
        #[must_use]
        #[inline(always)]
        pub const fn in10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN10."]
        #[inline(always)]
        pub const fn set_in10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc IN11."]
        #[must_use]
        #[inline(always)]
        pub const fn in11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN11."]
        #[inline(always)]
        pub const fn set_in11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IN12."]
        #[must_use]
        #[inline(always)]
        pub const fn in12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN12."]
        #[inline(always)]
        pub const fn set_in12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IN13."]
        #[must_use]
        #[inline(always)]
        pub const fn in13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN13."]
        #[inline(always)]
        pub const fn set_in13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc IN14."]
        #[must_use]
        #[inline(always)]
        pub const fn in14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN14."]
        #[inline(always)]
        pub const fn set_in14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IN15."]
        #[must_use]
        #[inline(always)]
        pub const fn in15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IN15."]
        #[inline(always)]
        pub const fn set_in15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Awdcr {
        #[inline(always)]
        fn default() -> Awdcr {
            Awdcr(0)
        }
    }
    impl core::fmt::Debug for Awdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdcr")
                .field("in0", &self.in0())
                .field("in1", &self.in1())
                .field("in2", &self.in2())
                .field("in3", &self.in3())
                .field("in4", &self.in4())
                .field("in5", &self.in5())
                .field("in6", &self.in6())
                .field("in7", &self.in7())
                .field("in8", &self.in8())
                .field("in9", &self.in9())
                .field("in10", &self.in10())
                .field("in11", &self.in11())
                .field("in12", &self.in12())
                .field("in13", &self.in13())
                .field("in14", &self.in14())
                .field("in15", &self.in15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awdcr {{ in0: {=bool:?}, in1: {=bool:?}, in2: {=bool:?}, in3: {=bool:?}, in4: {=bool:?}, in5: {=bool:?}, in6: {=bool:?}, in7: {=bool:?}, in8: {=bool:?}, in9: {=bool:?}, in10: {=bool:?}, in11: {=bool:?}, in12: {=bool:?}, in13: {=bool:?}, in14: {=bool:?}, in15: {=bool:?} }}",
                self.in0(),
                self.in1(),
                self.in2(),
                self.in3(),
                self.in4(),
                self.in5(),
                self.in6(),
                self.in7(),
                self.in8(),
                self.in9(),
                self.in10(),
                self.in11(),
                self.in12(),
                self.in13(),
                self.in14(),
                self.in15()
            )
        }
    }
    #[doc = "desc AWDTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awdtr(pub u32);
    impl Awdtr {
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
        #[doc = "desc VTH."]
        #[must_use]
        #[inline(always)]
        pub const fn vth(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc VTH."]
        #[inline(always)]
        pub const fn set_vth(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awdtr {
        #[inline(always)]
        fn default() -> Awdtr {
            Awdtr(0)
        }
    }
    impl core::fmt::Debug for Awdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awdtr")
                .field("vtl", &self.vtl())
                .field("vth", &self.vth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awdtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awdtr {{ vtl: {=u16:?}, vth: {=u16:?} }}",
                self.vtl(),
                self.vth()
            )
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
        #[doc = "desc BGREN."]
        #[must_use]
        #[inline(always)]
        pub const fn bgren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGREN."]
        #[inline(always)]
        pub const fn set_bgren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSEN."]
        #[inline(always)]
        pub const fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CONT."]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CONT."]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLKSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn clk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLKSRC."]
        #[inline(always)]
        pub const fn set_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
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
                .field("bgren", &self.bgren())
                .field("tsen", &self.tsen())
                .field("cont", &self.cont())
                .field("clk", &self.clk())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, bgren: {=bool:?}, tsen: {=bool:?}, cont: {=bool:?}, clk: {=u8:?}, ens: {=u8:?} }}",
                self.en(),
                self.bgren(),
                self.tsen(),
                self.cont(),
                self.clk(),
                self.ens()
            )
        }
    }
    #[doc = "desc ICR."]
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
        #[doc = "desc AWDL."]
        #[must_use]
        #[inline(always)]
        pub const fn awdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDL."]
        #[inline(always)]
        pub const fn set_awdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc AWDH."]
        #[must_use]
        #[inline(always)]
        pub const fn awdh(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDH."]
        #[inline(always)]
        pub const fn set_awdh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("awdl", &self.awdl())
                .field("awdh", &self.awdh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ eoc: {=bool:?}, eos: {=bool:?}, awdl: {=bool:?}, awdh: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.awdl(),
                self.awdh()
            )
        }
    }
    #[doc = "desc IER."]
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
        #[doc = "desc AWDL."]
        #[must_use]
        #[inline(always)]
        pub const fn awdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDL."]
        #[inline(always)]
        pub const fn set_awdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc AWDH."]
        #[must_use]
        #[inline(always)]
        pub const fn awdh(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDH."]
        #[inline(always)]
        pub const fn set_awdh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("awdl", &self.awdl())
                .field("awdh", &self.awdh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ eoc: {=bool:?}, eos: {=bool:?}, awdl: {=bool:?}, awdh: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.awdl(),
                self.awdh()
            )
        }
    }
    #[doc = "desc ISR."]
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
        #[doc = "desc AWDL."]
        #[must_use]
        #[inline(always)]
        pub const fn awdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDL."]
        #[inline(always)]
        pub const fn set_awdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc AWDH."]
        #[must_use]
        #[inline(always)]
        pub const fn awdh(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AWDH."]
        #[inline(always)]
        pub const fn set_awdh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("awdl", &self.awdl())
                .field("awdh", &self.awdh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ eoc: {=bool:?}, eos: {=bool:?}, awdl: {=bool:?}, awdh: {=bool:?} }}",
                self.eoc(),
                self.eos(),
                self.awdl(),
                self.awdh()
            )
        }
    }
    #[doc = "Right-aligned 12-bit sample in a 16-bit result field."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result(pub u32);
    impl Result {
        #[doc = "Conversion result for the corresponding sequence slot."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Conversion result for the corresponding sequence slot."]
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
    #[doc = "desc SAMPLE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sample(pub u32);
    impl Sample {
        #[doc = "Per-slot acquisition time encoding."]
        #[must_use]
        #[inline(always)]
        pub const fn sqrch(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Per-slot acquisition time encoding."]
        #[inline(always)]
        pub const fn set_sqrch(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Sample {
        #[inline(always)]
        fn default() -> Sample {
            Sample(0)
        }
    }
    impl core::fmt::Debug for Sample {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sample")
                .field("sqrch[0]", &self.sqrch(0usize))
                .field("sqrch[1]", &self.sqrch(1usize))
                .field("sqrch[2]", &self.sqrch(2usize))
                .field("sqrch[3]", &self.sqrch(3usize))
                .field("sqrch[4]", &self.sqrch(4usize))
                .field("sqrch[5]", &self.sqrch(5usize))
                .field("sqrch[6]", &self.sqrch(6usize))
                .field("sqrch[7]", &self.sqrch(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sample {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sample {{ sqrch[0]: {=u8:?}, sqrch[1]: {=u8:?}, sqrch[2]: {=u8:?}, sqrch[3]: {=u8:?}, sqrch[4]: {=u8:?}, sqrch[5]: {=u8:?}, sqrch[6]: {=u8:?}, sqrch[7]: {=u8:?} }}",
                self.sqrch(0usize),
                self.sqrch(1usize),
                self.sqrch(2usize),
                self.sqrch(3usize),
                self.sqrch(4usize),
                self.sqrch(5usize),
                self.sqrch(6usize),
                self.sqrch(7usize)
            )
        }
    }
    #[doc = "desc SQRCFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrcfr(pub u32);
    impl Sqrcfr {
        #[doc = "Per-slot input mux; slots execute in ascending slot order."]
        #[must_use]
        #[inline(always)]
        pub const fn sqrch(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Per-slot input mux; slots execute in ascending slot order."]
        #[inline(always)]
        pub const fn set_sqrch(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Sqrcfr {
        #[inline(always)]
        fn default() -> Sqrcfr {
            Sqrcfr(0)
        }
    }
    impl core::fmt::Debug for Sqrcfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrcfr")
                .field("sqrch[0]", &self.sqrch(0usize))
                .field("sqrch[1]", &self.sqrch(1usize))
                .field("sqrch[2]", &self.sqrch(2usize))
                .field("sqrch[3]", &self.sqrch(3usize))
                .field("sqrch[4]", &self.sqrch(4usize))
                .field("sqrch[5]", &self.sqrch(5usize))
                .field("sqrch[6]", &self.sqrch(6usize))
                .field("sqrch[7]", &self.sqrch(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqrcfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqrcfr {{ sqrch[0]: {=u8:?}, sqrch[1]: {=u8:?}, sqrch[2]: {=u8:?}, sqrch[3]: {=u8:?}, sqrch[4]: {=u8:?}, sqrch[5]: {=u8:?}, sqrch[6]: {=u8:?}, sqrch[7]: {=u8:?} }}",
                self.sqrch(0usize),
                self.sqrch(1usize),
                self.sqrch(2usize),
                self.sqrch(3usize),
                self.sqrch(4usize),
                self.sqrch(5usize),
                self.sqrch(6usize),
                self.sqrch(7usize)
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Start {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Start {{ start: {=bool:?} }}", self.start())
        }
    }
    #[doc = "desc TRIGGER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigger(pub u32);
    impl Trigger {
        #[doc = "desc ATIMTRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn atimtrgo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMTRGO."]
        #[inline(always)]
        pub const fn set_atimtrgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ATIMTRGO2."]
        #[must_use]
        #[inline(always)]
        pub const fn atimtrgo2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMTRGO2."]
        #[inline(always)]
        pub const fn set_atimtrgo2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ATIMCC1."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC1."]
        #[inline(always)]
        pub const fn set_atimcc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ATIMCC2."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC2."]
        #[inline(always)]
        pub const fn set_atimcc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc ATIMCC3."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC3."]
        #[inline(always)]
        pub const fn set_atimcc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIMCC4."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc4(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC4."]
        #[inline(always)]
        pub const fn set_atimcc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ATIMCC5."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc5(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC5."]
        #[inline(always)]
        pub const fn set_atimcc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ATIMCC6."]
        #[must_use]
        #[inline(always)]
        pub const fn atimcc6(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMCC6."]
        #[inline(always)]
        pub const fn set_atimcc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc GTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1trgo(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1TRGO."]
        #[inline(always)]
        pub const fn set_gtim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc GTIM1CC1."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1cc1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1CC1."]
        #[inline(always)]
        pub const fn set_gtim1cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc GTIM1CC2."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1cc2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1CC2."]
        #[inline(always)]
        pub const fn set_gtim1cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc GTIM1CC3."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1cc3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1CC3."]
        #[inline(always)]
        pub const fn set_gtim1cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc GTIM1CC4."]
        #[must_use]
        #[inline(always)]
        pub const fn gtim1cc4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc GTIM1CC4."]
        #[inline(always)]
        pub const fn set_gtim1cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BTIM1TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim1trgo(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM1TRGO."]
        #[inline(always)]
        pub const fn set_btim1trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BTIM2TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim2trgo(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM2TRGO."]
        #[inline(always)]
        pub const fn set_btim2trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc BTIM3TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn btim3trgo(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc BTIM3TRGO."]
        #[inline(always)]
        pub const fn set_btim3trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("atimtrgo", &self.atimtrgo())
                .field("atimtrgo2", &self.atimtrgo2())
                .field("atimcc1", &self.atimcc1())
                .field("atimcc2", &self.atimcc2())
                .field("atimcc3", &self.atimcc3())
                .field("atimcc4", &self.atimcc4())
                .field("atimcc5", &self.atimcc5())
                .field("atimcc6", &self.atimcc6())
                .field("gtim1trgo", &self.gtim1trgo())
                .field("gtim1cc1", &self.gtim1cc1())
                .field("gtim1cc2", &self.gtim1cc2())
                .field("gtim1cc3", &self.gtim1cc3())
                .field("gtim1cc4", &self.gtim1cc4())
                .field("btim1trgo", &self.btim1trgo())
                .field("btim2trgo", &self.btim2trgo())
                .field("btim3trgo", &self.btim3trgo())
                .field("spi1", &self.spi1())
                .field("uart1", &self.uart1())
                .field("uart2", &self.uart2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trigger {{ atimtrgo: {=bool:?}, atimtrgo2: {=bool:?}, atimcc1: {=bool:?}, atimcc2: {=bool:?}, atimcc3: {=bool:?}, atimcc4: {=bool:?}, atimcc5: {=bool:?}, atimcc6: {=bool:?}, gtim1trgo: {=bool:?}, gtim1cc1: {=bool:?}, gtim1cc2: {=bool:?}, gtim1cc3: {=bool:?}, gtim1cc4: {=bool:?}, btim1trgo: {=bool:?}, btim2trgo: {=bool:?}, btim3trgo: {=bool:?}, spi1: {=bool:?}, uart1: {=bool:?}, uart2: {=bool:?} }}",
                self.atimtrgo(),
                self.atimtrgo2(),
                self.atimcc1(),
                self.atimcc2(),
                self.atimcc3(),
                self.atimcc4(),
                self.atimcc5(),
                self.atimcc6(),
                self.gtim1trgo(),
                self.gtim1cc1(),
                self.gtim1cc2(),
                self.gtim1cc3(),
                self.gtim1cc4(),
                self.btim1trgo(),
                self.btim2trgo(),
                self.btim3trgo(),
                self.spi1(),
                self.uart1(),
                self.uart2()
            )
        }
    }
}
