#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA Channel 1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmachannel {
    ptr: *mut u8,
}
unsafe impl Send for Dmachannel {}
unsafe impl Sync for Dmachannel {}
impl Dmachannel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel.y control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel.y counter register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel.y source address register."]
    #[inline(always)]
    pub const fn srcaddr(self) -> crate::common::Reg<regs::Srcaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel.y destination address register."]
    #[inline(always)]
    pub const fn dstaddr(self) -> crate::common::Reg<regs::Dstaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel.y trigger register."]
    #[inline(always)]
    pub const fn trig(self) -> crate::common::Reg<regs::Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel.y counter register."]
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
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt")
                .field("cnt", &self.cnt())
                .field("repeat", &self.repeat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnt {{ cnt: {=u16:?}, repeat: {=u8:?} }}",
                self.cnt(),
                self.repeat()
            )
        }
    }
    #[doc = "Channel.y control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
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
        pub const fn trans(&self) -> super::vals::TransferMode {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::TransferMode::from_bits(val as u8)
        }
        #[doc = "desc TRANS."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: super::vals::TransferMode) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
        pub const fn size(&self) -> super::vals::Width {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> super::vals::Status {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Status::from_bits(val as u8)
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: super::vals::Status) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("teie", &self.teie())
                .field("trans", &self.trans())
                .field("srcinc", &self.srcinc())
                .field("dstinc", &self.dstinc())
                .field("size", &self.size())
                .field("status", &self.status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ en: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, trans: {:?}, srcinc: {=bool:?}, dstinc: {=bool:?}, size: {:?}, status: {:?} }}",
                self.en(),
                self.tcie(),
                self.teie(),
                self.trans(),
                self.srcinc(),
                self.dstinc(),
                self.size(),
                self.status()
            )
        }
    }
    #[doc = "Channel.y destination address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dstaddr(pub u32);
    impl Dstaddr {
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
    impl Default for Dstaddr {
        #[inline(always)]
        fn default() -> Dstaddr {
            Dstaddr(0)
        }
    }
    impl core::fmt::Debug for Dstaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dstaddr")
                .field("dstaddr", &self.dstaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dstaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dstaddr {{ dstaddr: {=u32:?} }}", self.dstaddr())
        }
    }
    #[doc = "Channel.y source address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcaddr(pub u32);
    impl Srcaddr {
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
    impl Default for Srcaddr {
        #[inline(always)]
        fn default() -> Srcaddr {
            Srcaddr(0)
        }
    }
    impl core::fmt::Debug for Srcaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srcaddr")
                .field("srcaddr", &self.srcaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srcaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Srcaddr {{ srcaddr: {=u32:?} }}", self.srcaddr())
        }
    }
    #[doc = "Channel.y trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig(pub u32);
    impl Trig {
        #[doc = "desc TYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> super::vals::Trigger {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Trigger::from_bits(val as u8)
        }
        #[doc = "desc TYPE."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: super::vals::Trigger) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
    impl Default for Trig {
        #[inline(always)]
        fn default() -> Trig {
            Trig(0)
        }
    }
    impl core::fmt::Debug for Trig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig")
                .field("type_", &self.type_())
                .field("softsrc", &self.softsrc())
                .field("hardsrc", &self.hardsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trig {{ type_: {:?}, softsrc: {=bool:?}, hardsrc: {=u8:?} }}",
                self.type_(),
                self.softsrc(),
                self.hardsrc()
            )
        }
    }
}
pub mod vals {
    #[doc = "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Status {
        Initial = 0x0,
        AddressOutOfRange = 0x01,
        Stopped = 0x02,
        SourceError = 0x03,
        DestinationError = 0x04,
        Complete = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Status {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Status {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Status {
        #[inline(always)]
        fn from(val: u8) -> Status {
            Status::from_bits(val)
        }
    }
    impl From<Status> for u8 {
        #[inline(always)]
        fn from(val: Status) -> u8 {
            Status::to_bits(val)
        }
    }
    #[doc = "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TransferMode {
        Bulk = 0x0,
        Block = 0x01,
    }
    impl TransferMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TransferMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TransferMode {
        #[inline(always)]
        fn from(val: u8) -> TransferMode {
            TransferMode::from_bits(val)
        }
    }
    impl From<TransferMode> for u8 {
        #[inline(always)]
        fn from(val: TransferMode) -> u8 {
            TransferMode::to_bits(val)
        }
    }
    #[doc = "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trigger {
        Software = 0x0,
        Hardware = 0x01,
    }
    impl Trigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigger {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[doc = "Own-family manuals sections 8.8.3-8.8.4 agree across all canonical users; see docs/dma-owned-copy-evidence.md."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Width {
        Byte = 0x0,
        HalfWord = 0x01,
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Width {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Width {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Width {
        #[inline(always)]
        fn from(val: u8) -> Width {
            Width::from_bits(val)
        }
    }
    impl From<Width> for u8 {
        #[inline(always)]
        fn from(val: Width) -> u8 {
            Width::to_bits(val)
        }
    }
}
