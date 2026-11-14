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
    #[doc = "Control register1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt enable register."]
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
    #[doc = "desc TIMARR."]
    #[inline(always)]
    pub const fn timarr(self) -> crate::common::Reg<regs::Timarr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc TIMCNT."]
    #[inline(always)]
    pub const fn timcnt(self) -> crate::common::Reg<regs::Timcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Data reg for read."]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data reg for write."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Slave addr."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Slave addr mask."]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Control register3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc RXMATCH."]
    #[inline(always)]
    pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Slave addr."]
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
    #[doc = "Control register1."]
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
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Parity::from_bits(val as u8)
        }
        #[doc = "desc PARITY."]
        #[inline(always)]
        pub const fn set_parity(&mut self, val: super::vals::Parity) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PARITYEN."]
        #[must_use]
        #[inline(always)]
        pub const fn parityen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PARITYEN."]
        #[inline(always)]
        pub const fn set_parityen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "desc CHLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn chlen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHLEN."]
        #[inline(always)]
        pub const fn set_chlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MSBF."]
        #[must_use]
        #[inline(always)]
        pub const fn msbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSBF."]
        #[inline(always)]
        pub const fn set_msbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
        #[doc = "desc SIGNAL."]
        #[must_use]
        #[inline(always)]
        pub const fn signal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SIGNAL."]
        #[inline(always)]
        pub const fn set_signal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> super::vals::Source {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Source::from_bits(val as u8)
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: super::vals::Source) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "desc SYNC."]
        #[must_use]
        #[inline(always)]
        pub const fn sync(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNC."]
        #[inline(always)]
        pub const fn set_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("parityen", &self.parityen())
                .field("stop", &self.stop())
                .field("chlen", &self.chlen())
                .field("msbf", &self.msbf())
                .field("start", &self.start())
                .field("over", &self.over())
                .field("signal", &self.signal())
                .field("source", &self.source())
                .field("sync", &self.sync())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ txen: {=bool:?}, rxen: {=bool:?}, parity: {:?}, parityen: {=bool:?}, stop: {:?}, chlen: {=bool:?}, msbf: {=bool:?}, start: {=bool:?}, over: {:?}, signal: {=bool:?}, source: {:?}, sync: {=bool:?} }}",
                self.txen(),
                self.rxen(),
                self.parity(),
                self.parityen(),
                self.stop(),
                self.chlen(),
                self.msbf(),
                self.start(),
                self.over(),
                self.signal(),
                self.source(),
                self.sync()
            )
        }
    }
    #[doc = "Control register2."]
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
        #[doc = "desc RXMATCHEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmatchen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMATCHEN."]
        #[inline(always)]
        pub const fn set_rxmatchen(&mut self, val: bool) {
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
        #[doc = "desc TIMCR."]
        #[must_use]
        #[inline(always)]
        pub const fn timcr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIMCR."]
        #[inline(always)]
        pub const fn set_timcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc SWAP."]
        #[must_use]
        #[inline(always)]
        pub const fn swap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWAP."]
        #[inline(always)]
        pub const fn set_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc ADCRX."]
        #[must_use]
        #[inline(always)]
        pub const fn adcrx(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADCRX."]
        #[inline(always)]
        pub const fn set_adcrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ADCTX."]
        #[must_use]
        #[inline(always)]
        pub const fn adctx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADCTX."]
        #[inline(always)]
        pub const fn set_adctx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LOOP."]
        #[must_use]
        #[inline(always)]
        pub const fn loop_(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOOP."]
        #[inline(always)]
        pub const fn set_loop_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RXSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn rxsrc(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "desc RXSRC."]
        #[inline(always)]
        pub const fn set_rxsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
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
                .field("rxmatchen", &self.rxmatchen())
                .field("ctsen", &self.ctsen())
                .field("rtsen", &self.rtsen())
                .field("rxinv", &self.rxinv())
                .field("txinv", &self.txinv())
                .field("timcr", &self.timcr())
                .field("swap", &self.swap())
                .field("adcrx", &self.adcrx())
                .field("adctx", &self.adctx())
                .field("loop_", &self.loop_())
                .field("rxsrc", &self.rxsrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ addren: {=bool:?}, rxmatchen: {=bool:?}, ctsen: {=bool:?}, rtsen: {=bool:?}, rxinv: {=bool:?}, txinv: {=bool:?}, timcr: {=u8:?}, swap: {=bool:?}, adcrx: {=bool:?}, adctx: {=bool:?}, loop_: {=bool:?}, rxsrc: {=u8:?} }}",
                self.addren(),
                self.rxmatchen(),
                self.ctsen(),
                self.rtsen(),
                self.rxinv(),
                self.txinv(),
                self.timcr(),
                self.swap(),
                self.adcrx(),
                self.adctx(),
                self.loop_(),
                self.rxsrc()
            )
        }
    }
    #[doc = "Control register3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "desc DEM."]
        #[must_use]
        #[inline(always)]
        pub const fn dem(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEM."]
        #[inline(always)]
        pub const fn set_dem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DEP."]
        #[must_use]
        #[inline(always)]
        pub const fn dep(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEP."]
        #[inline(always)]
        pub const fn set_dep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DETIME."]
        #[must_use]
        #[inline(always)]
        pub const fn detime(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "desc DETIME."]
        #[inline(always)]
        pub const fn set_detime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "desc LIN."]
        #[must_use]
        #[inline(always)]
        pub const fn lin(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc LIN."]
        #[inline(always)]
        pub const fn set_lin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc BRKL."]
        #[must_use]
        #[inline(always)]
        pub const fn brkl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRKL."]
        #[inline(always)]
        pub const fn set_brkl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("dem", &self.dem())
                .field("dep", &self.dep())
                .field("detime", &self.detime())
                .field("lin", &self.lin())
                .field("brkl", &self.brkl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ dem: {=bool:?}, dep: {=bool:?}, detime: {=u8:?}, lin: {=bool:?}, brkl: {=bool:?} }}",
                self.dem(),
                self.dep(),
                self.detime(),
                self.lin(),
                self.brkl()
            )
        }
    }
    #[doc = "Interrupt flag clear register."]
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
        #[doc = "desc RXIDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXIDLE."]
        #[inline(always)]
        pub const fn set_rxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXBRK."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbrk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBRK."]
        #[inline(always)]
        pub const fn set_rxbrk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BAUD."]
        #[must_use]
        #[inline(always)]
        pub const fn baud(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BAUD."]
        #[inline(always)]
        pub const fn set_baud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIMOV."]
        #[must_use]
        #[inline(always)]
        pub const fn timov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMOV."]
        #[inline(always)]
        pub const fn set_timov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NE."]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NE."]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ORE."]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ORE."]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RXMATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmatch(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMATCH."]
        #[inline(always)]
        pub const fn set_rxmatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("rxidle", &self.rxidle())
                .field("rxbrk", &self.rxbrk())
                .field("baud", &self.baud())
                .field("timov", &self.timov())
                .field("cts", &self.cts())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("rxmatch", &self.rxmatch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ tc: {=bool:?}, rc: {=bool:?}, rxidle: {=bool:?}, rxbrk: {=bool:?}, baud: {=bool:?}, timov: {=bool:?}, cts: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, rxmatch: {=bool:?} }}",
                self.tc(),
                self.rc(),
                self.rxidle(),
                self.rxbrk(),
                self.baud(),
                self.timov(),
                self.cts(),
                self.fe(),
                self.pe(),
                self.ne(),
                self.ore(),
                self.rxmatch()
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
        #[doc = "desc RXIDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXIDLE."]
        #[inline(always)]
        pub const fn set_rxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXBRK."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbrk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBRK."]
        #[inline(always)]
        pub const fn set_rxbrk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BAUD."]
        #[must_use]
        #[inline(always)]
        pub const fn baud(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BAUD."]
        #[inline(always)]
        pub const fn set_baud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIMOV."]
        #[must_use]
        #[inline(always)]
        pub const fn timov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMOV."]
        #[inline(always)]
        pub const fn set_timov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NE."]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NE."]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ORE."]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ORE."]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RXMATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmatch(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMATCH."]
        #[inline(always)]
        pub const fn set_rxmatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("rxidle", &self.rxidle())
                .field("rxbrk", &self.rxbrk())
                .field("baud", &self.baud())
                .field("timov", &self.timov())
                .field("cts", &self.cts())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("rxmatch", &self.rxmatch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ txe: {=bool:?}, tc: {=bool:?}, rc: {=bool:?}, rxidle: {=bool:?}, rxbrk: {=bool:?}, baud: {=bool:?}, timov: {=bool:?}, cts: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, rxmatch: {=bool:?} }}",
                self.txe(),
                self.tc(),
                self.rc(),
                self.rxidle(),
                self.rxbrk(),
                self.baud(),
                self.timov(),
                self.cts(),
                self.fe(),
                self.pe(),
                self.ne(),
                self.ore(),
                self.rxmatch()
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
        #[doc = "desc RXIDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXIDLE."]
        #[inline(always)]
        pub const fn set_rxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXBRK."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbrk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBRK."]
        #[inline(always)]
        pub const fn set_rxbrk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BAUD."]
        #[must_use]
        #[inline(always)]
        pub const fn baud(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BAUD."]
        #[inline(always)]
        pub const fn set_baud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIMOV."]
        #[must_use]
        #[inline(always)]
        pub const fn timov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMOV."]
        #[inline(always)]
        pub const fn set_timov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CTS."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTS."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NE."]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NE."]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ORE."]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ORE."]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RXMATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmatch(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMATCH."]
        #[inline(always)]
        pub const fn set_rxmatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SLVMATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn slvmatch(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SLVMATCH."]
        #[inline(always)]
        pub const fn set_slvmatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc TXBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn txbusy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBUSY."]
        #[inline(always)]
        pub const fn set_txbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CTSLV."]
        #[must_use]
        #[inline(always)]
        pub const fn ctslv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTSLV."]
        #[inline(always)]
        pub const fn set_ctslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("rxidle", &self.rxidle())
                .field("rxbrk", &self.rxbrk())
                .field("baud", &self.baud())
                .field("timov", &self.timov())
                .field("cts", &self.cts())
                .field("fe", &self.fe())
                .field("pe", &self.pe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("rxmatch", &self.rxmatch())
                .field("slvmatch", &self.slvmatch())
                .field("txbusy", &self.txbusy())
                .field("ctslv", &self.ctslv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ txe: {=bool:?}, tc: {=bool:?}, rc: {=bool:?}, rxidle: {=bool:?}, rxbrk: {=bool:?}, baud: {=bool:?}, timov: {=bool:?}, cts: {=bool:?}, fe: {=bool:?}, pe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, rxmatch: {=bool:?}, slvmatch: {=bool:?}, txbusy: {=bool:?}, ctslv: {=bool:?} }}",
                self.txe(),
                self.tc(),
                self.rc(),
                self.rxidle(),
                self.rxbrk(),
                self.baud(),
                self.timov(),
                self.cts(),
                self.fe(),
                self.pe(),
                self.ne(),
                self.ore(),
                self.rxmatch(),
                self.slvmatch(),
                self.txbusy(),
                self.ctslv()
            )
        }
    }
    #[doc = "Slave addr mask."]
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
    #[doc = "Data reg for read."]
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
    #[doc = "desc RXMATCH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmatch(pub u32);
    impl Rxmatch {
        #[doc = "desc RXMATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmatch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc RXMATCH."]
        #[inline(always)]
        pub const fn set_rxmatch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Rxmatch {
        #[inline(always)]
        fn default() -> Rxmatch {
            Rxmatch(0)
        }
    }
    impl core::fmt::Debug for Rxmatch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxmatch")
                .field("rxmatch", &self.rxmatch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxmatch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxmatch {{ rxmatch: {=u16:?} }}", self.rxmatch())
        }
    }
    #[doc = "Data reg for write."]
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
        #[doc = "desc IDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDLE."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BREAK."]
        #[must_use]
        #[inline(always)]
        pub const fn break_(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BREAK."]
        #[inline(always)]
        pub const fn set_break_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            f.debug_struct("Tdr")
                .field("tdr", &self.tdr())
                .field("idle", &self.idle())
                .field("break_", &self.break_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tdr {{ tdr: {=u16:?}, idle: {=bool:?}, break_: {=bool:?} }}",
                self.tdr(),
                self.idle(),
                self.break_()
            )
        }
    }
    #[doc = "desc TIMARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timarr(pub u32);
    impl Timarr {
        #[doc = "desc TIMARR."]
        #[must_use]
        #[inline(always)]
        pub const fn timarr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "desc TIMARR."]
        #[inline(always)]
        pub const fn set_timarr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Timarr {
        #[inline(always)]
        fn default() -> Timarr {
            Timarr(0)
        }
    }
    impl core::fmt::Debug for Timarr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timarr")
                .field("timarr", &self.timarr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timarr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timarr {{ timarr: {=u32:?} }}", self.timarr())
        }
    }
    #[doc = "desc TIMCNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timcnt(pub u32);
    impl Timcnt {
        #[doc = "desc TIMCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn timcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "desc TIMCNT."]
        #[inline(always)]
        pub const fn set_timcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Timcnt {
        #[inline(always)]
        fn default() -> Timcnt {
            Timcnt(0)
        }
    }
    impl core::fmt::Debug for Timcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timcnt")
                .field("timcnt", &self.timcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timcnt {{ timcnt: {=u32:?} }}", self.timcnt())
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
        #[doc = "Even parity."]
        Even = 0x0,
        #[doc = "Odd parity."]
        Odd = 0x01,
    }
    impl Parity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Parity {
            unsafe { core::mem::transmute(val & 0x01) }
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
        Self(8191)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(8191)
    }
}
