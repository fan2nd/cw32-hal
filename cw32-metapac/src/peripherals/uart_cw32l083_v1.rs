#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc UART1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc BRRI."]
    #[inline(always)]
    pub const fn brri(self) -> crate::common::Reg<regs::Brri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc BRRF."]
    #[inline(always)]
    pub const fn brrf(self) -> crate::common::Reg<regs::Brrf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc RDR."]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc TDR."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc ADDR."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc MASK."]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "desc ADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDR."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addr {{ addr: {=u8:?} }}", self.addr())
        }
    }
    #[doc = "desc BRRF."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brrf(pub u32);
    impl Brrf {
        #[doc = "desc BRRF."]
        #[must_use]
        #[inline(always)]
        pub const fn brrf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc BRRF."]
        #[inline(always)]
        pub const fn set_brrf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Brrf {
        #[inline(always)]
        fn default() -> Brrf {
            Brrf(0)
        }
    }
    impl core::fmt::Debug for Brrf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brrf").field("brrf", &self.brrf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brrf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brrf {{ brrf: {=u8:?} }}", self.brrf())
        }
    }
    #[doc = "desc BRRI."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brri(pub u32);
    impl Brri {
        #[doc = "desc BRRI."]
        #[must_use]
        #[inline(always)]
        pub const fn brri(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc BRRI."]
        #[inline(always)]
        pub const fn set_brri(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Brri {
        #[inline(always)]
        fn default() -> Brri {
            Brri(0)
        }
    }
    impl core::fmt::Debug for Brri {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brri").field("brri", &self.brri()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brri {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brri {{ brri: {=u16:?} }}", self.brri())
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc TXEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEN."]
        #[inline(always)]
        pub const fn set_txen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXEN."]
        #[inline(always)]
        pub const fn set_rxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PARITY."]
        #[must_use]
        #[inline(always)]
        pub const fn parity(&self) -> super::vals::Parity {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Parity::from_bits(val as u8)
        }
        #[doc = "desc PARITY."]
        #[inline(always)]
        pub const fn set_parity(&mut self, val: super::vals::Parity) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "desc SYNC."]
        #[must_use]
        #[inline(always)]
        pub const fn sync(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNC."]
        #[inline(always)]
        pub const fn set_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc OVER."]
        #[must_use]
        #[inline(always)]
        pub const fn over(&self) -> super::vals::Over {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Over::from_bits(val as u8)
        }
        #[doc = "desc OVER."]
        #[inline(always)]
        pub const fn set_over(&mut self, val: super::vals::Over) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
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
                .field("txen", &self.txen())
                .field("rxen", &self.rxen())
                .field("parity", &self.parity())
                .field("stop", &self.stop())
                .field("sync", &self.sync())
                .field("start", &self.start())
                .field("over", &self.over())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ txen: {=bool:?}, rxen: {=bool:?}, parity: {:?}, stop: {:?}, sync: {=bool:?}, start: {=bool:?}, over: {:?} }}",
                self.txen(),
                self.rxen(),
                self.parity(),
                self.stop(),
                self.sync(),
                self.start(),
                self.over()
            )
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc ADDREN."]
        #[must_use]
        #[inline(always)]
        pub const fn addren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDREN."]
        #[inline(always)]
        pub const fn set_addren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SIGNAL."]
        #[must_use]
        #[inline(always)]
        pub const fn signal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SIGNAL."]
        #[inline(always)]
        pub const fn set_signal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CTSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTSEN."]
        #[inline(always)]
        pub const fn set_ctsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RTSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rtsen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTSEN."]
        #[inline(always)]
        pub const fn set_rtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXINV."]
        #[must_use]
        #[inline(always)]
        pub const fn rxinv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXINV."]
        #[inline(always)]
        pub const fn set_rxinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TXINV."]
        #[must_use]
        #[inline(always)]
        pub const fn txinv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXINV."]
        #[inline(always)]
        pub const fn set_txinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DMARX."]
        #[must_use]
        #[inline(always)]
        pub const fn dmarx(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMARX."]
        #[inline(always)]
        pub const fn set_dmarx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DMATX."]
        #[must_use]
        #[inline(always)]
        pub const fn dmatx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMATX."]
        #[inline(always)]
        pub const fn set_dmatx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> super::vals::Source {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
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
                .field("addren", &self.addren())
                .field("signal", &self.signal())
                .field("ctsen", &self.ctsen())
                .field("rtsen", &self.rtsen())
                .field("rxinv", &self.rxinv())
                .field("txinv", &self.txinv())
                .field("dmarx", &self.dmarx())
                .field("dmatx", &self.dmatx())
                .field("source", &self.source())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ addren: {=bool:?}, signal: {=bool:?}, ctsen: {=bool:?}, rtsen: {=bool:?}, rxinv: {=bool:?}, txinv: {=bool:?}, dmarx: {=bool:?}, dmatx: {=bool:?}, source: {:?} }}",
                self.addren(),
                self.signal(),
                self.ctsen(),
                self.rtsen(),
                self.rxinv(),
                self.txinv(),
                self.dmarx(),
                self.dmatx(),
                self.source()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
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
                .field("tc", &self.tc())
                .field("rc", &self.rc())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("cts", &self.cts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ tc: {=bool:?}, rc: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, cts: {=bool:?} }}",
                self.tc(),
                self.rc(),
                self.fe(),
                self.pe(),
                self.cts()
            )
        }
    }
    #[doc = "desc IER."]
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
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
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
                .field("txe", &self.txe())
                .field("tc", &self.tc())
                .field("rc", &self.rc())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("cts", &self.cts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ txe: {=bool:?}, tc: {=bool:?}, rc: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, cts: {=bool:?} }}",
                self.txe(),
                self.tc(),
                self.rc(),
                self.fe(),
                self.pe(),
                self.cts()
            )
        }
    }
    #[doc = "desc ISR."]
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
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RC."]
        #[must_use]
        #[inline(always)]
        pub const fn rc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC."]
        #[inline(always)]
        pub const fn set_rc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc MATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCH."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CTSLV."]
        #[must_use]
        #[inline(always)]
        pub const fn ctslv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTSLV."]
        #[inline(always)]
        pub const fn set_ctslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn txbusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBUSY."]
        #[inline(always)]
        pub const fn set_txbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("tc", &self.tc())
                .field("rc", &self.rc())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("match_", &self.match_())
                .field("cts", &self.cts())
                .field("ctslv", &self.ctslv())
                .field("txbusy", &self.txbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ txe: {=bool:?}, tc: {=bool:?}, rc: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, match_: {=bool:?}, cts: {=bool:?}, ctslv: {=bool:?}, txbusy: {=bool:?} }}",
                self.txe(),
                self.tc(),
                self.rc(),
                self.fe(),
                self.pe(),
                self.match_(),
                self.cts(),
                self.ctslv(),
                self.txbusy()
            )
        }
    }
    #[doc = "desc MASK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mask(pub u32);
    impl Mask {
        #[doc = "desc MASK."]
        #[must_use]
        #[inline(always)]
        pub const fn mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc MASK."]
        #[inline(always)]
        pub const fn set_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Mask {
        #[inline(always)]
        fn default() -> Mask {
            Mask(0)
        }
    }
    impl core::fmt::Debug for Mask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mask").field("mask", &self.mask()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mask {{ mask: {=u8:?} }}", self.mask())
        }
    }
    #[doc = "desc RDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdr(pub u32);
    impl Rdr {
        #[doc = "desc RDR."]
        #[must_use]
        #[inline(always)]
        pub const fn rdr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc RDR."]
        #[inline(always)]
        pub const fn set_rdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Rdr {
        #[inline(always)]
        fn default() -> Rdr {
            Rdr(0)
        }
    }
    impl core::fmt::Debug for Rdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rdr {{ rdr: {=u16:?} }}", self.rdr())
        }
    }
    #[doc = "desc TDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdr(pub u32);
    impl Tdr {
        #[doc = "desc TDR."]
        #[must_use]
        #[inline(always)]
        pub const fn tdr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TDR."]
        #[inline(always)]
        pub const fn set_tdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Tdr {
        #[inline(always)]
        fn default() -> Tdr {
            Tdr(0)
        }
    }
    impl core::fmt::Debug for Tdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdr").field("tdr", &self.tdr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tdr {{ tdr: {=u16:?} }}", self.tdr())
        }
    }
}
pub mod vals {
    #[doc = "UART OVER encodings; see docs/uart-typed-registers.md for own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Over {
        #[doc = "Sixteen samples per bit."]
        Over16 = 0x0,
        #[doc = "Eight samples per bit."]
        Over8 = 0x01,
        #[doc = "Four samples per bit."]
        Over4 = 0x02,
        #[doc = "Dedicated low-speed sampling."]
        Special = 0x03,
    }
    impl Over {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Over {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Over {
        #[inline(always)]
        fn from(val: u8) -> Over {
            Over::from_bits(val)
        }
    }
    impl From<Over> for u8 {
        #[inline(always)]
        fn from(val: Over) -> u8 {
            Over::to_bits(val)
        }
    }
    #[doc = "UART PARITY encodings; see docs/uart-typed-registers.md for own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Parity {
        #[doc = "No parity."]
        None = 0x0,
        #[doc = "User-defined parity."]
        Custom = 0x01,
        #[doc = "Even parity."]
        Even = 0x02,
        #[doc = "Odd parity."]
        Odd = 0x03,
    }
    impl Parity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Parity {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Parity {
        #[inline(always)]
        fn from(val: u8) -> Parity {
            Parity::from_bits(val)
        }
    }
    impl From<Parity> for u8 {
        #[inline(always)]
        fn from(val: Parity) -> u8 {
            Parity::to_bits(val)
        }
    }
    #[doc = "UART SOURCE encodings; see docs/uart-typed-registers.md for own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Source {
        #[doc = "Peripheral clock."]
        Pclk = 0x0,
        #[doc = "Peripheral clock, alternate encoding."]
        PclkAlt = 0x01,
        #[doc = "External low-speed clock."]
        Lse = 0x02,
        #[doc = "Internal low-speed clock."]
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
    #[doc = "UART STOP encodings; see docs/uart-typed-registers.md for own-manual evidence."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stop {
        #[doc = "One stop bit."]
        Stop1 = 0x0,
        #[doc = "One and a half stop bits."]
        Stop1p5 = 0x01,
        #[doc = "Two stop bits."]
        Stop2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Stop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stop {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stop {
        #[inline(always)]
        fn from(val: u8) -> Stop {
            Stop::from_bits(val)
        }
    }
    impl From<Stop> for u8 {
        #[inline(always)]
        fn from(val: Stop) -> u8 {
            Stop::to_bits(val)
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
