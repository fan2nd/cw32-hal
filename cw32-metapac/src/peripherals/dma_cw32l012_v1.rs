#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel1 control and status register."]
    #[inline(always)]
    pub const fn csr1(self) -> crate::common::Reg<regs::Csr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel1 counter register."]
    #[inline(always)]
    pub const fn cnt1(self) -> crate::common::Reg<regs::Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn srcaddr1(self) -> crate::common::Reg<regs::Srcaddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn dstaddr1(self) -> crate::common::Reg<regs::Dstaddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn trig1(self) -> crate::common::Reg<regs::Trig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn ccnt1(self) -> crate::common::Reg<regs::Ccnt1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn csrcaddr1(self) -> crate::common::Reg<regs::Csrcaddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn cdstaddr1(self) -> crate::common::Reg<regs::Cdstaddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Channel1 control and status register."]
    #[inline(always)]
    pub const fn csr2(self) -> crate::common::Reg<regs::Csr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Channel1 counter register."]
    #[inline(always)]
    pub const fn cnt2(self) -> crate::common::Reg<regs::Cnt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn srcaddr2(self) -> crate::common::Reg<regs::Srcaddr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn dstaddr2(self) -> crate::common::Reg<regs::Dstaddr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn trig2(self) -> crate::common::Reg<regs::Trig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn ccnt2(self) -> crate::common::Reg<regs::Ccnt2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn csrcaddr2(self) -> crate::common::Reg<regs::Csrcaddr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn cdstaddr2(self) -> crate::common::Reg<regs::Cdstaddr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Channel1 control and status register."]
    #[inline(always)]
    pub const fn csr3(self) -> crate::common::Reg<regs::Csr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Channel1 counter register."]
    #[inline(always)]
    pub const fn cnt3(self) -> crate::common::Reg<regs::Cnt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn srcaddr3(self) -> crate::common::Reg<regs::Srcaddr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn dstaddr3(self) -> crate::common::Reg<regs::Dstaddr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn trig3(self) -> crate::common::Reg<regs::Trig3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn ccnt3(self) -> crate::common::Reg<regs::Ccnt3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn csrcaddr3(self) -> crate::common::Reg<regs::Csrcaddr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn cdstaddr3(self) -> crate::common::Reg<regs::Cdstaddr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Channel1 control and status register."]
    #[inline(always)]
    pub const fn csr4(self) -> crate::common::Reg<regs::Csr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Channel1 counter register."]
    #[inline(always)]
    pub const fn cnt4(self) -> crate::common::Reg<regs::Cnt4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn srcaddr4(self) -> crate::common::Reg<regs::Srcaddr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn dstaddr4(self) -> crate::common::Reg<regs::Dstaddr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn trig4(self) -> crate::common::Reg<regs::Trig4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Channel1 trigger register."]
    #[inline(always)]
    pub const fn ccnt4(self) -> crate::common::Reg<regs::Ccnt4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Channel1 source address register."]
    #[inline(always)]
    pub const fn csrcaddr4(self) -> crate::common::Reg<regs::Csrcaddr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Channel1 destination address register."]
    #[inline(always)]
    pub const fn cdstaddr4(self) -> crate::common::Reg<regs::Cdstaddr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccnt1(pub u32);
    impl Ccnt1 {
        #[doc = "desc CCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn ccnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCNT."]
        #[inline(always)]
        pub const fn set_ccnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc CREPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn crepeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CREPEAT."]
        #[inline(always)]
        pub const fn set_crepeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ccnt1 {
        #[inline(always)]
        fn default() -> Ccnt1 {
            Ccnt1(0)
        }
    }
    impl core::fmt::Debug for Ccnt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccnt1")
                .field("ccnt", &self.ccnt())
                .field("crepeat", &self.crepeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccnt1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccnt1 {{ ccnt: {=u16:?}, crepeat: {=u8:?} }}",
                self.ccnt(),
                self.crepeat()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccnt2(pub u32);
    impl Ccnt2 {
        #[doc = "desc CCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn ccnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCNT."]
        #[inline(always)]
        pub const fn set_ccnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc CREPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn crepeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CREPEAT."]
        #[inline(always)]
        pub const fn set_crepeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ccnt2 {
        #[inline(always)]
        fn default() -> Ccnt2 {
            Ccnt2(0)
        }
    }
    impl core::fmt::Debug for Ccnt2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccnt2")
                .field("ccnt", &self.ccnt())
                .field("crepeat", &self.crepeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccnt2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccnt2 {{ ccnt: {=u16:?}, crepeat: {=u8:?} }}",
                self.ccnt(),
                self.crepeat()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccnt3(pub u32);
    impl Ccnt3 {
        #[doc = "desc CCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn ccnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCNT."]
        #[inline(always)]
        pub const fn set_ccnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc CREPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn crepeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CREPEAT."]
        #[inline(always)]
        pub const fn set_crepeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ccnt3 {
        #[inline(always)]
        fn default() -> Ccnt3 {
            Ccnt3(0)
        }
    }
    impl core::fmt::Debug for Ccnt3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccnt3")
                .field("ccnt", &self.ccnt())
                .field("crepeat", &self.crepeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccnt3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccnt3 {{ ccnt: {=u16:?}, crepeat: {=u8:?} }}",
                self.ccnt(),
                self.crepeat()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccnt4(pub u32);
    impl Ccnt4 {
        #[doc = "desc CCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn ccnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCNT."]
        #[inline(always)]
        pub const fn set_ccnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc CREPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn crepeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CREPEAT."]
        #[inline(always)]
        pub const fn set_crepeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ccnt4 {
        #[inline(always)]
        fn default() -> Ccnt4 {
            Ccnt4(0)
        }
    }
    impl core::fmt::Debug for Ccnt4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccnt4")
                .field("ccnt", &self.ccnt())
                .field("crepeat", &self.crepeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccnt4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccnt4 {{ ccnt: {=u16:?}, crepeat: {=u8:?} }}",
                self.ccnt(),
                self.crepeat()
            )
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdstaddr1(pub u32);
    impl Cdstaddr1 {
        #[doc = "desc CDSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn cdstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CDSTADDR."]
        #[inline(always)]
        pub const fn set_cdstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cdstaddr1 {
        #[inline(always)]
        fn default() -> Cdstaddr1 {
            Cdstaddr1(0)
        }
    }
    impl core::fmt::Debug for Cdstaddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdstaddr1")
                .field("cdstaddr", &self.cdstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdstaddr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdstaddr1 {{ cdstaddr: {=u32:?} }}", self.cdstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdstaddr2(pub u32);
    impl Cdstaddr2 {
        #[doc = "desc CDSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn cdstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CDSTADDR."]
        #[inline(always)]
        pub const fn set_cdstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cdstaddr2 {
        #[inline(always)]
        fn default() -> Cdstaddr2 {
            Cdstaddr2(0)
        }
    }
    impl core::fmt::Debug for Cdstaddr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdstaddr2")
                .field("cdstaddr", &self.cdstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdstaddr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdstaddr2 {{ cdstaddr: {=u32:?} }}", self.cdstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdstaddr3(pub u32);
    impl Cdstaddr3 {
        #[doc = "desc CDSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn cdstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CDSTADDR."]
        #[inline(always)]
        pub const fn set_cdstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cdstaddr3 {
        #[inline(always)]
        fn default() -> Cdstaddr3 {
            Cdstaddr3(0)
        }
    }
    impl core::fmt::Debug for Cdstaddr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdstaddr3")
                .field("cdstaddr", &self.cdstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdstaddr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdstaddr3 {{ cdstaddr: {=u32:?} }}", self.cdstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdstaddr4(pub u32);
    impl Cdstaddr4 {
        #[doc = "desc CDSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn cdstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CDSTADDR."]
        #[inline(always)]
        pub const fn set_cdstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cdstaddr4 {
        #[inline(always)]
        fn default() -> Cdstaddr4 {
            Cdstaddr4(0)
        }
    }
    impl core::fmt::Debug for Cdstaddr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdstaddr4")
                .field("cdstaddr", &self.cdstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdstaddr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdstaddr4 {{ cdstaddr: {=u32:?} }}", self.cdstaddr())
        }
    }
    #[doc = "Channel1 counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt1(pub u32);
    impl Cnt1 {
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
        #[doc = "desc REPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc REPEAT."]
        #[inline(always)]
        pub const fn set_repeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cnt1 {
        #[inline(always)]
        fn default() -> Cnt1 {
            Cnt1(0)
        }
    }
    impl core::fmt::Debug for Cnt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt1")
                .field("cnt", &self.cnt())
                .field("repeat", &self.repeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt1 {{ cnt: {=u16:?}, repeat: {=u8:?} }}",
                self.cnt(),
                self.repeat()
            )
        }
    }
    #[doc = "Channel1 counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt2(pub u32);
    impl Cnt2 {
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
        #[doc = "desc REPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc REPEAT."]
        #[inline(always)]
        pub const fn set_repeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cnt2 {
        #[inline(always)]
        fn default() -> Cnt2 {
            Cnt2(0)
        }
    }
    impl core::fmt::Debug for Cnt2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt2")
                .field("cnt", &self.cnt())
                .field("repeat", &self.repeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt2 {{ cnt: {=u16:?}, repeat: {=u8:?} }}",
                self.cnt(),
                self.repeat()
            )
        }
    }
    #[doc = "Channel1 counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt3(pub u32);
    impl Cnt3 {
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
        #[doc = "desc REPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc REPEAT."]
        #[inline(always)]
        pub const fn set_repeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cnt3 {
        #[inline(always)]
        fn default() -> Cnt3 {
            Cnt3(0)
        }
    }
    impl core::fmt::Debug for Cnt3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt3")
                .field("cnt", &self.cnt())
                .field("repeat", &self.repeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt3 {{ cnt: {=u16:?}, repeat: {=u8:?} }}",
                self.cnt(),
                self.repeat()
            )
        }
    }
    #[doc = "Channel1 counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt4(pub u32);
    impl Cnt4 {
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
        #[doc = "desc REPEAT."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc REPEAT."]
        #[inline(always)]
        pub const fn set_repeat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cnt4 {
        #[inline(always)]
        fn default() -> Cnt4 {
            Cnt4(0)
        }
    }
    impl core::fmt::Debug for Cnt4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt4")
                .field("cnt", &self.cnt())
                .field("repeat", &self.repeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt4 {{ cnt: {=u16:?}, repeat: {=u8:?} }}",
                self.cnt(),
                self.repeat()
            )
        }
    }
    #[doc = "Channel1 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
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
        #[doc = "desc TCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCIE."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEIE."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TRANS."]
        #[must_use]
        #[inline(always)]
        pub const fn trans(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRANS."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRCINC."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCINC."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DSTINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTINC."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn srcload(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[inline(always)]
        pub const fn set_srcload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DSTLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dstload(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTLOAD."]
        #[inline(always)]
        pub const fn set_dstload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Csr1 {
        #[inline(always)]
        fn default() -> Csr1 {
            Csr1(0)
        }
    }
    impl core::fmt::Debug for Csr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr1")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("teie", &self.teie())
                .field("trans", &self.trans())
                .field("srcinc", &self.srcinc())
                .field("dstinc", &self.dstinc())
                .field("size", &self.size())
                .field("status", &self.status())
                .field("restart", &self.restart())
                .field("tc", &self.tc())
                .field("te", &self.te())
                .field("srcload", &self.srcload())
                .field("dstload", &self.dstload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr1 {{ en: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, trans: {=bool:?}, srcinc: {=bool:?}, dstinc: {=bool:?}, size: {=u8:?}, status: {=u8:?}, restart: {=bool:?}, tc: {=bool:?}, te: {=bool:?}, srcload: {=bool:?}, dstload: {=bool:?} }}",
                self.en(),
                self.tcie(),
                self.teie(),
                self.trans(),
                self.srcinc(),
                self.dstinc(),
                self.size(),
                self.status(),
                self.restart(),
                self.tc(),
                self.te(),
                self.srcload(),
                self.dstload()
            )
        }
    }
    #[doc = "Channel1 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr2(pub u32);
    impl Csr2 {
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
        #[doc = "desc TCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCIE."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEIE."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TRANS."]
        #[must_use]
        #[inline(always)]
        pub const fn trans(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRANS."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRCINC."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCINC."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DSTINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTINC."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn srcload(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[inline(always)]
        pub const fn set_srcload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DSTLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dstload(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTLOAD."]
        #[inline(always)]
        pub const fn set_dstload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Csr2 {
        #[inline(always)]
        fn default() -> Csr2 {
            Csr2(0)
        }
    }
    impl core::fmt::Debug for Csr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr2")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("teie", &self.teie())
                .field("trans", &self.trans())
                .field("srcinc", &self.srcinc())
                .field("dstinc", &self.dstinc())
                .field("size", &self.size())
                .field("status", &self.status())
                .field("restart", &self.restart())
                .field("tc", &self.tc())
                .field("te", &self.te())
                .field("srcload", &self.srcload())
                .field("dstload", &self.dstload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr2 {{ en: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, trans: {=bool:?}, srcinc: {=bool:?}, dstinc: {=bool:?}, size: {=u8:?}, status: {=u8:?}, restart: {=bool:?}, tc: {=bool:?}, te: {=bool:?}, srcload: {=bool:?}, dstload: {=bool:?} }}",
                self.en(),
                self.tcie(),
                self.teie(),
                self.trans(),
                self.srcinc(),
                self.dstinc(),
                self.size(),
                self.status(),
                self.restart(),
                self.tc(),
                self.te(),
                self.srcload(),
                self.dstload()
            )
        }
    }
    #[doc = "Channel1 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr3(pub u32);
    impl Csr3 {
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
        #[doc = "desc TCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCIE."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEIE."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TRANS."]
        #[must_use]
        #[inline(always)]
        pub const fn trans(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRANS."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRCINC."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCINC."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DSTINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTINC."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn srcload(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[inline(always)]
        pub const fn set_srcload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DSTLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dstload(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTLOAD."]
        #[inline(always)]
        pub const fn set_dstload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Csr3 {
        #[inline(always)]
        fn default() -> Csr3 {
            Csr3(0)
        }
    }
    impl core::fmt::Debug for Csr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr3")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("teie", &self.teie())
                .field("trans", &self.trans())
                .field("srcinc", &self.srcinc())
                .field("dstinc", &self.dstinc())
                .field("size", &self.size())
                .field("status", &self.status())
                .field("restart", &self.restart())
                .field("tc", &self.tc())
                .field("te", &self.te())
                .field("srcload", &self.srcload())
                .field("dstload", &self.dstload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr3 {{ en: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, trans: {=bool:?}, srcinc: {=bool:?}, dstinc: {=bool:?}, size: {=u8:?}, status: {=u8:?}, restart: {=bool:?}, tc: {=bool:?}, te: {=bool:?}, srcload: {=bool:?}, dstload: {=bool:?} }}",
                self.en(),
                self.tcie(),
                self.teie(),
                self.trans(),
                self.srcinc(),
                self.dstinc(),
                self.size(),
                self.status(),
                self.restart(),
                self.tc(),
                self.te(),
                self.srcload(),
                self.dstload()
            )
        }
    }
    #[doc = "Channel1 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr4(pub u32);
    impl Csr4 {
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
        #[doc = "desc TCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCIE."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEIE."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TRANS."]
        #[must_use]
        #[inline(always)]
        pub const fn trans(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRANS."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRCINC."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCINC."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DSTINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTINC."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn srcload(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRCLOAD."]
        #[inline(always)]
        pub const fn set_srcload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DSTLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn dstload(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSTLOAD."]
        #[inline(always)]
        pub const fn set_dstload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Csr4 {
        #[inline(always)]
        fn default() -> Csr4 {
            Csr4(0)
        }
    }
    impl core::fmt::Debug for Csr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr4")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("teie", &self.teie())
                .field("trans", &self.trans())
                .field("srcinc", &self.srcinc())
                .field("dstinc", &self.dstinc())
                .field("size", &self.size())
                .field("status", &self.status())
                .field("restart", &self.restart())
                .field("tc", &self.tc())
                .field("te", &self.te())
                .field("srcload", &self.srcload())
                .field("dstload", &self.dstload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr4 {{ en: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, trans: {=bool:?}, srcinc: {=bool:?}, dstinc: {=bool:?}, size: {=u8:?}, status: {=u8:?}, restart: {=bool:?}, tc: {=bool:?}, te: {=bool:?}, srcload: {=bool:?}, dstload: {=bool:?} }}",
                self.en(),
                self.tcie(),
                self.teie(),
                self.trans(),
                self.srcinc(),
                self.dstinc(),
                self.size(),
                self.status(),
                self.restart(),
                self.tc(),
                self.te(),
                self.srcload(),
                self.dstload()
            )
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csrcaddr1(pub u32);
    impl Csrcaddr1 {
        #[doc = "desc CSRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn csrcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CSRCADDR."]
        #[inline(always)]
        pub const fn set_csrcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Csrcaddr1 {
        #[inline(always)]
        fn default() -> Csrcaddr1 {
            Csrcaddr1(0)
        }
    }
    impl core::fmt::Debug for Csrcaddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csrcaddr1")
                .field("csrcaddr", &self.csrcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csrcaddr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csrcaddr1 {{ csrcaddr: {=u32:?} }}", self.csrcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csrcaddr2(pub u32);
    impl Csrcaddr2 {
        #[doc = "desc CSRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn csrcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CSRCADDR."]
        #[inline(always)]
        pub const fn set_csrcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Csrcaddr2 {
        #[inline(always)]
        fn default() -> Csrcaddr2 {
            Csrcaddr2(0)
        }
    }
    impl core::fmt::Debug for Csrcaddr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csrcaddr2")
                .field("csrcaddr", &self.csrcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csrcaddr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csrcaddr2 {{ csrcaddr: {=u32:?} }}", self.csrcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csrcaddr3(pub u32);
    impl Csrcaddr3 {
        #[doc = "desc CSRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn csrcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CSRCADDR."]
        #[inline(always)]
        pub const fn set_csrcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Csrcaddr3 {
        #[inline(always)]
        fn default() -> Csrcaddr3 {
            Csrcaddr3(0)
        }
    }
    impl core::fmt::Debug for Csrcaddr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csrcaddr3")
                .field("csrcaddr", &self.csrcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csrcaddr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csrcaddr3 {{ csrcaddr: {=u32:?} }}", self.csrcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csrcaddr4(pub u32);
    impl Csrcaddr4 {
        #[doc = "desc CSRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn csrcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CSRCADDR."]
        #[inline(always)]
        pub const fn set_csrcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Csrcaddr4 {
        #[inline(always)]
        fn default() -> Csrcaddr4 {
            Csrcaddr4(0)
        }
    }
    impl core::fmt::Debug for Csrcaddr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csrcaddr4")
                .field("csrcaddr", &self.csrcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csrcaddr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csrcaddr4 {{ csrcaddr: {=u32:?} }}", self.csrcaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstaddr1(pub u32);
    impl Dstaddr1 {
        #[doc = "desc DSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADDR."]
        #[inline(always)]
        pub const fn set_dstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstaddr1 {
        #[inline(always)]
        fn default() -> Dstaddr1 {
            Dstaddr1(0)
        }
    }
    impl core::fmt::Debug for Dstaddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstaddr1")
                .field("dstaddr", &self.dstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dstaddr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dstaddr1 {{ dstaddr: {=u32:?} }}", self.dstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstaddr2(pub u32);
    impl Dstaddr2 {
        #[doc = "desc DSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADDR."]
        #[inline(always)]
        pub const fn set_dstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstaddr2 {
        #[inline(always)]
        fn default() -> Dstaddr2 {
            Dstaddr2(0)
        }
    }
    impl core::fmt::Debug for Dstaddr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstaddr2")
                .field("dstaddr", &self.dstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dstaddr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dstaddr2 {{ dstaddr: {=u32:?} }}", self.dstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstaddr3(pub u32);
    impl Dstaddr3 {
        #[doc = "desc DSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADDR."]
        #[inline(always)]
        pub const fn set_dstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstaddr3 {
        #[inline(always)]
        fn default() -> Dstaddr3 {
            Dstaddr3(0)
        }
    }
    impl core::fmt::Debug for Dstaddr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstaddr3")
                .field("dstaddr", &self.dstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dstaddr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dstaddr3 {{ dstaddr: {=u32:?} }}", self.dstaddr())
        }
    }
    #[doc = "Channel1 destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstaddr4(pub u32);
    impl Dstaddr4 {
        #[doc = "desc DSTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn dstaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DSTADDR."]
        #[inline(always)]
        pub const fn set_dstaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dstaddr4 {
        #[inline(always)]
        fn default() -> Dstaddr4 {
            Dstaddr4(0)
        }
    }
    impl core::fmt::Debug for Dstaddr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstaddr4")
                .field("dstaddr", &self.dstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dstaddr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dstaddr4 {{ dstaddr: {=u32:?} }}", self.dstaddr())
        }
    }
    #[doc = "Interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc TC1."]
        #[must_use]
        #[inline(always)]
        pub const fn tc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC1."]
        #[inline(always)]
        pub const fn set_tc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TE1."]
        #[must_use]
        #[inline(always)]
        pub const fn te1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE1."]
        #[inline(always)]
        pub const fn set_te1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TC2."]
        #[must_use]
        #[inline(always)]
        pub const fn tc2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC2."]
        #[inline(always)]
        pub const fn set_tc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TE2."]
        #[must_use]
        #[inline(always)]
        pub const fn te2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE2."]
        #[inline(always)]
        pub const fn set_te2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TC3."]
        #[must_use]
        #[inline(always)]
        pub const fn tc3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC3."]
        #[inline(always)]
        pub const fn set_tc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TE3."]
        #[must_use]
        #[inline(always)]
        pub const fn te3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE3."]
        #[inline(always)]
        pub const fn set_te3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TC4."]
        #[must_use]
        #[inline(always)]
        pub const fn tc4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC4."]
        #[inline(always)]
        pub const fn set_tc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc TE4."]
        #[must_use]
        #[inline(always)]
        pub const fn te4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE4."]
        #[inline(always)]
        pub const fn set_te4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("tc1", &self.tc1())
                .field("te1", &self.te1())
                .field("tc2", &self.tc2())
                .field("te2", &self.te2())
                .field("tc3", &self.tc3())
                .field("te3", &self.te3())
                .field("tc4", &self.tc4())
                .field("te4", &self.te4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ tc1: {=bool:?}, te1: {=bool:?}, tc2: {=bool:?}, te2: {=bool:?}, tc3: {=bool:?}, te3: {=bool:?}, tc4: {=bool:?}, te4: {=bool:?} }}",
                self.tc1(),
                self.te1(),
                self.tc2(),
                self.te2(),
                self.tc3(),
                self.te3(),
                self.tc4(),
                self.te4()
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc TC1."]
        #[must_use]
        #[inline(always)]
        pub const fn tc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC1."]
        #[inline(always)]
        pub const fn set_tc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TE1."]
        #[must_use]
        #[inline(always)]
        pub const fn te1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE1."]
        #[inline(always)]
        pub const fn set_te1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TC2."]
        #[must_use]
        #[inline(always)]
        pub const fn tc2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC2."]
        #[inline(always)]
        pub const fn set_tc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TE2."]
        #[must_use]
        #[inline(always)]
        pub const fn te2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE2."]
        #[inline(always)]
        pub const fn set_te2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TC3."]
        #[must_use]
        #[inline(always)]
        pub const fn tc3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC3."]
        #[inline(always)]
        pub const fn set_tc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TE3."]
        #[must_use]
        #[inline(always)]
        pub const fn te3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE3."]
        #[inline(always)]
        pub const fn set_te3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TC4."]
        #[must_use]
        #[inline(always)]
        pub const fn tc4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC4."]
        #[inline(always)]
        pub const fn set_tc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc TE4."]
        #[must_use]
        #[inline(always)]
        pub const fn te4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE4."]
        #[inline(always)]
        pub const fn set_te4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("tc1", &self.tc1())
                .field("te1", &self.te1())
                .field("tc2", &self.tc2())
                .field("te2", &self.te2())
                .field("tc3", &self.tc3())
                .field("te3", &self.te3())
                .field("tc4", &self.tc4())
                .field("te4", &self.te4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ tc1: {=bool:?}, te1: {=bool:?}, tc2: {=bool:?}, te2: {=bool:?}, tc3: {=bool:?}, te3: {=bool:?}, tc4: {=bool:?}, te4: {=bool:?} }}",
                self.tc1(),
                self.te1(),
                self.tc2(),
                self.te2(),
                self.tc3(),
                self.te3(),
                self.tc4(),
                self.te4()
            )
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcaddr1(pub u32);
    impl Srcaddr1 {
        #[doc = "desc SRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADDR."]
        #[inline(always)]
        pub const fn set_srcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcaddr1 {
        #[inline(always)]
        fn default() -> Srcaddr1 {
            Srcaddr1(0)
        }
    }
    impl core::fmt::Debug for Srcaddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcaddr1")
                .field("srcaddr", &self.srcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcaddr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcaddr1 {{ srcaddr: {=u32:?} }}", self.srcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcaddr2(pub u32);
    impl Srcaddr2 {
        #[doc = "desc SRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADDR."]
        #[inline(always)]
        pub const fn set_srcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcaddr2 {
        #[inline(always)]
        fn default() -> Srcaddr2 {
            Srcaddr2(0)
        }
    }
    impl core::fmt::Debug for Srcaddr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcaddr2")
                .field("srcaddr", &self.srcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcaddr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcaddr2 {{ srcaddr: {=u32:?} }}", self.srcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcaddr3(pub u32);
    impl Srcaddr3 {
        #[doc = "desc SRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADDR."]
        #[inline(always)]
        pub const fn set_srcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcaddr3 {
        #[inline(always)]
        fn default() -> Srcaddr3 {
            Srcaddr3(0)
        }
    }
    impl core::fmt::Debug for Srcaddr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcaddr3")
                .field("srcaddr", &self.srcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcaddr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcaddr3 {{ srcaddr: {=u32:?} }}", self.srcaddr())
        }
    }
    #[doc = "Channel1 source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcaddr4(pub u32);
    impl Srcaddr4 {
        #[doc = "desc SRCADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn srcaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc SRCADDR."]
        #[inline(always)]
        pub const fn set_srcaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Srcaddr4 {
        #[inline(always)]
        fn default() -> Srcaddr4 {
            Srcaddr4(0)
        }
    }
    impl core::fmt::Debug for Srcaddr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcaddr4")
                .field("srcaddr", &self.srcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcaddr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcaddr4 {{ srcaddr: {=u32:?} }}", self.srcaddr())
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig1(pub u32);
    impl Trig1 {
        #[doc = "desc TYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TYPE."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SOFTSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn softsrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTSRC."]
        #[inline(always)]
        pub const fn set_softsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HARDSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn hardsrc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HARDSRC."]
        #[inline(always)]
        pub const fn set_hardsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
    }
    impl Default for Trig1 {
        #[inline(always)]
        fn default() -> Trig1 {
            Trig1(0)
        }
    }
    impl core::fmt::Debug for Trig1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig1")
                .field("type_", &self.type_())
                .field("softsrc", &self.softsrc())
                .field("hardsrc", &self.hardsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig1 {{ type_: {=bool:?}, softsrc: {=bool:?}, hardsrc: {=u8:?} }}",
                self.type_(),
                self.softsrc(),
                self.hardsrc()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig2(pub u32);
    impl Trig2 {
        #[doc = "desc TYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TYPE."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SOFTSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn softsrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTSRC."]
        #[inline(always)]
        pub const fn set_softsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HARDSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn hardsrc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HARDSRC."]
        #[inline(always)]
        pub const fn set_hardsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
    }
    impl Default for Trig2 {
        #[inline(always)]
        fn default() -> Trig2 {
            Trig2(0)
        }
    }
    impl core::fmt::Debug for Trig2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig2")
                .field("type_", &self.type_())
                .field("softsrc", &self.softsrc())
                .field("hardsrc", &self.hardsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig2 {{ type_: {=bool:?}, softsrc: {=bool:?}, hardsrc: {=u8:?} }}",
                self.type_(),
                self.softsrc(),
                self.hardsrc()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig3(pub u32);
    impl Trig3 {
        #[doc = "desc TYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TYPE."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SOFTSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn softsrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTSRC."]
        #[inline(always)]
        pub const fn set_softsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HARDSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn hardsrc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HARDSRC."]
        #[inline(always)]
        pub const fn set_hardsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
    }
    impl Default for Trig3 {
        #[inline(always)]
        fn default() -> Trig3 {
            Trig3(0)
        }
    }
    impl core::fmt::Debug for Trig3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig3")
                .field("type_", &self.type_())
                .field("softsrc", &self.softsrc())
                .field("hardsrc", &self.hardsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig3 {{ type_: {=bool:?}, softsrc: {=bool:?}, hardsrc: {=u8:?} }}",
                self.type_(),
                self.softsrc(),
                self.hardsrc()
            )
        }
    }
    #[doc = "Channel1 trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig4(pub u32);
    impl Trig4 {
        #[doc = "desc TYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TYPE."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SOFTSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn softsrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFTSRC."]
        #[inline(always)]
        pub const fn set_softsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HARDSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn hardsrc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "desc HARDSRC."]
        #[inline(always)]
        pub const fn set_hardsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
    }
    impl Default for Trig4 {
        #[inline(always)]
        fn default() -> Trig4 {
            Trig4(0)
        }
    }
    impl core::fmt::Debug for Trig4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig4")
                .field("type_", &self.type_())
                .field("softsrc", &self.softsrc())
                .field("hardsrc", &self.hardsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig4 {{ type_: {=bool:?}, softsrc: {=bool:?}, hardsrc: {=u8:?} }}",
                self.type_(),
                self.softsrc(),
                self.hardsrc()
            )
        }
    }
}
impl regs::Icr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(4294967295)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(4294967295)
    }
}
