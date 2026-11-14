#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc I2C1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc INSEL."]
    #[inline(always)]
    pub const fn insel(self) -> crate::common::Reg<regs::Insel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Master Interrupt Flag Clear Register."]
    #[inline(always)]
    pub const fn micr(self) -> crate::common::Reg<regs::Micr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc MCR0."]
    #[inline(always)]
    pub const fn mcr0(self) -> crate::common::Reg<regs::Mcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Master Interrupt Status Register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Master Interrupt Enable Register."]
    #[inline(always)]
    pub const fn mier(self) -> crate::common::Reg<regs::Mier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Master DMA Enable Register."]
    #[inline(always)]
    pub const fn mder(self) -> crate::common::Reg<regs::Mder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Master Control Register1."]
    #[inline(always)]
    pub const fn mcr1(self) -> crate::common::Reg<regs::Mcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Master Control Register2."]
    #[inline(always)]
    pub const fn mcr2(self) -> crate::common::Reg<regs::Mcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc SCR2."]
    #[inline(always)]
    pub const fn scr2(self) -> crate::common::Reg<regs::Scr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Master Control Register3."]
    #[inline(always)]
    pub const fn mcr3(self) -> crate::common::Reg<regs::Mcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Master Control Register4."]
    #[inline(always)]
    pub const fn mcr4(self) -> crate::common::Reg<regs::Mcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Master Match Register."]
    #[inline(always)]
    pub const fn mmatch(self) -> crate::common::Reg<regs::Mmatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Master Clock Control Register."]
    #[inline(always)]
    pub const fn mccr(self) -> crate::common::Reg<regs::Mccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Master FIFO Control Register."]
    #[inline(always)]
    pub const fn mfifocr(self) -> crate::common::Reg<regs::Mfifocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Master FIFO Status Register."]
    #[inline(always)]
    pub const fn mfifosr(self) -> crate::common::Reg<regs::Mfifosr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Master Tx Data Register."]
    #[inline(always)]
    pub const fn mtdr(self) -> crate::common::Reg<regs::Mtdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Master Rx Data Register."]
    #[inline(always)]
    pub const fn mrdr(self) -> crate::common::Reg<regs::Mrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc SICR."]
    #[inline(always)]
    pub const fn sicr(self) -> crate::common::Reg<regs::Sicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Slave Control Register0."]
    #[inline(always)]
    pub const fn scr0(self) -> crate::common::Reg<regs::Scr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc SISR."]
    #[inline(always)]
    pub const fn sisr(self) -> crate::common::Reg<regs::Sisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc SIER."]
    #[inline(always)]
    pub const fn sier(self) -> crate::common::Reg<regs::Sier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "desc SDER."]
    #[inline(always)]
    pub const fn sder(self) -> crate::common::Reg<regs::Sder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "desc SCR1."]
    #[inline(always)]
    pub const fn scr1(self) -> crate::common::Reg<regs::Scr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "desc SCR3."]
    #[inline(always)]
    pub const fn scr3(self) -> crate::common::Reg<regs::Scr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "desc SAMR."]
    #[inline(always)]
    pub const fn samr(self) -> crate::common::Reg<regs::Samr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "desc SASR."]
    #[inline(always)]
    pub const fn sasr(self) -> crate::common::Reg<regs::Sasr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "desc STAR."]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc STDR."]
    #[inline(always)]
    pub const fn stdr(self) -> crate::common::Reg<regs::Stdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "desc SRDR."]
    #[inline(always)]
    pub const fn srdr(self) -> crate::common::Reg<regs::Srdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc INSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Insel(pub u32);
    impl Insel {
        #[doc = "desc SCLINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sclinsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SCLINSEL."]
        #[inline(always)]
        pub const fn set_sclinsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc SDAINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sdainsel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc SDAINSEL."]
        #[inline(always)]
        pub const fn set_sdainsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
    }
    impl Default for Insel {
        #[inline(always)]
        fn default() -> Insel {
            Insel(0)
        }
    }
    impl core::fmt::Debug for Insel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Insel")
                .field("sclinsel", &self.sclinsel())
                .field("sdainsel", &self.sdainsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Insel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Insel {{ sclinsel: {=u8:?}, sdainsel: {=u8:?} }}",
                self.sclinsel(),
                self.sdainsel()
            )
        }
    }
    #[doc = "Master Clock Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mccr(pub u32);
    impl Mccr {
        #[doc = "desc CLKLO."]
        #[must_use]
        #[inline(always)]
        pub const fn clklo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLKLO."]
        #[inline(always)]
        pub const fn set_clklo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc CLKHI."]
        #[must_use]
        #[inline(always)]
        pub const fn clkhi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLKHI."]
        #[inline(always)]
        pub const fn set_clkhi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "desc SETHOLD."]
        #[must_use]
        #[inline(always)]
        pub const fn sethold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc SETHOLD."]
        #[inline(always)]
        pub const fn set_sethold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "desc DATAVD."]
        #[must_use]
        #[inline(always)]
        pub const fn datavd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DATAVD."]
        #[inline(always)]
        pub const fn set_datavd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Mccr {
        #[inline(always)]
        fn default() -> Mccr {
            Mccr(0)
        }
    }
    impl core::fmt::Debug for Mccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mccr")
                .field("clklo", &self.clklo())
                .field("clkhi", &self.clkhi())
                .field("sethold", &self.sethold())
                .field("datavd", &self.datavd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mccr {{ clklo: {=u8:?}, clkhi: {=u8:?}, sethold: {=u8:?}, datavd: {=u8:?} }}",
                self.clklo(),
                self.clkhi(),
                self.sethold(),
                self.datavd()
            )
        }
    }
    #[doc = "desc MCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr0(pub u32);
    impl Mcr0 {
        #[doc = "desc MEN."]
        #[must_use]
        #[inline(always)]
        pub const fn men(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MEN."]
        #[inline(always)]
        pub const fn set_men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESET."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SLEEPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sleepen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SLEEPEN."]
        #[inline(always)]
        pub const fn set_sleepen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc DBGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBGEN."]
        #[inline(always)]
        pub const fn set_dbgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLKSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn clksrc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLKSRC."]
        #[inline(always)]
        pub const fn set_clksrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc TXFIFORST."]
        #[must_use]
        #[inline(always)]
        pub const fn txfiforst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFORST."]
        #[inline(always)]
        pub const fn set_txfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RXFIFORST."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfiforst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXFIFORST."]
        #[inline(always)]
        pub const fn set_rxfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mcr0 {
        #[inline(always)]
        fn default() -> Mcr0 {
            Mcr0(0)
        }
    }
    impl core::fmt::Debug for Mcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr0")
                .field("men", &self.men())
                .field("reset", &self.reset())
                .field("sleepen", &self.sleepen())
                .field("dbgen", &self.dbgen())
                .field("clksrc", &self.clksrc())
                .field("txfiforst", &self.txfiforst())
                .field("rxfiforst", &self.rxfiforst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr0 {{ men: {=bool:?}, reset: {=bool:?}, sleepen: {=bool:?}, dbgen: {=bool:?}, clksrc: {=u8:?}, txfiforst: {=bool:?}, rxfiforst: {=bool:?} }}",
                self.men(),
                self.reset(),
                self.sleepen(),
                self.dbgen(),
                self.clksrc(),
                self.txfiforst(),
                self.rxfiforst()
            )
        }
    }
    #[doc = "Master Control Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr1(pub u32);
    impl Mcr1 {
        #[doc = "desc CIRFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn cirfifo(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIRFIFO."]
        #[inline(always)]
        pub const fn set_cirfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RDMO."]
        #[must_use]
        #[inline(always)]
        pub const fn rdmo(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RDMO."]
        #[inline(always)]
        pub const fn set_rdmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mcr1 {
        #[inline(always)]
        fn default() -> Mcr1 {
            Mcr1(0)
        }
    }
    impl core::fmt::Debug for Mcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr1")
                .field("cirfifo", &self.cirfifo())
                .field("rdmo", &self.rdmo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr1 {{ cirfifo: {=bool:?}, rdmo: {=bool:?} }}",
                self.cirfifo(),
                self.rdmo()
            )
        }
    }
    #[doc = "Master Control Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr2(pub u32);
    impl Mcr2 {
        #[doc = "desc PRESCALE."]
        #[must_use]
        #[inline(always)]
        pub const fn prescale(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PRESCALE."]
        #[inline(always)]
        pub const fn set_prescale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc AUTOSTOP."]
        #[must_use]
        #[inline(always)]
        pub const fn autostop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTOSTOP."]
        #[inline(always)]
        pub const fn set_autostop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IGNACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ignack(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IGNACK."]
        #[inline(always)]
        pub const fn set_ignack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TIMECFG."]
        #[must_use]
        #[inline(always)]
        pub const fn timecfg(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIMECFG."]
        #[inline(always)]
        pub const fn set_timecfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc MATCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn matcfg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc MATCFG."]
        #[inline(always)]
        pub const fn set_matcfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc PINCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn pincfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc PINCFG."]
        #[inline(always)]
        pub const fn set_pincfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Mcr2 {
        #[inline(always)]
        fn default() -> Mcr2 {
            Mcr2(0)
        }
    }
    impl core::fmt::Debug for Mcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr2")
                .field("prescale", &self.prescale())
                .field("autostop", &self.autostop())
                .field("ignack", &self.ignack())
                .field("timecfg", &self.timecfg())
                .field("trgsel", &self.trgsel())
                .field("matcfg", &self.matcfg())
                .field("pincfg", &self.pincfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr2 {{ prescale: {=u8:?}, autostop: {=bool:?}, ignack: {=bool:?}, timecfg: {=bool:?}, trgsel: {=bool:?}, matcfg: {=u8:?}, pincfg: {=u8:?} }}",
                self.prescale(),
                self.autostop(),
                self.ignack(),
                self.timecfg(),
                self.trgsel(),
                self.matcfg(),
                self.pincfg()
            )
        }
    }
    #[doc = "Master Control Register3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr3(pub u32);
    impl Mcr3 {
        #[doc = "desc BUSIDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn busidle(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc BUSIDLE."]
        #[inline(always)]
        pub const fn set_busidle(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc FLTSCL."]
        #[must_use]
        #[inline(always)]
        pub const fn fltscl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTSCL."]
        #[inline(always)]
        pub const fn set_fltscl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc FLTSDA."]
        #[must_use]
        #[inline(always)]
        pub const fn fltsda(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTSDA."]
        #[inline(always)]
        pub const fn set_fltsda(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Mcr3 {
        #[inline(always)]
        fn default() -> Mcr3 {
            Mcr3(0)
        }
    }
    impl core::fmt::Debug for Mcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr3")
                .field("busidle", &self.busidle())
                .field("fltscl", &self.fltscl())
                .field("fltsda", &self.fltsda())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr3 {{ busidle: {=u16:?}, fltscl: {=u8:?}, fltsda: {=u8:?} }}",
                self.busidle(),
                self.fltscl(),
                self.fltsda()
            )
        }
    }
    #[doc = "Master Control Register4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr4(pub u32);
    impl Mcr4 {
        #[doc = "desc PINLOW."]
        #[must_use]
        #[inline(always)]
        pub const fn pinlow(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc PINLOW."]
        #[inline(always)]
        pub const fn set_pinlow(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
        }
    }
    impl Default for Mcr4 {
        #[inline(always)]
        fn default() -> Mcr4 {
            Mcr4(0)
        }
    }
    impl core::fmt::Debug for Mcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr4")
                .field("pinlow", &self.pinlow())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mcr4 {{ pinlow: {=u16:?} }}", self.pinlow())
        }
    }
    #[doc = "Master DMA Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mder(pub u32);
    impl Mder {
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
    }
    impl Default for Mder {
        #[inline(always)]
        fn default() -> Mder {
            Mder(0)
        }
    }
    impl core::fmt::Debug for Mder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mder")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mder {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mder {{ txe: {=bool:?}, rxne: {=bool:?} }}",
                self.txe(),
                self.rxne()
            )
        }
    }
    #[doc = "Master FIFO Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfifocr(pub u32);
    impl Mfifocr {
        #[doc = "desc TXWATER."]
        #[must_use]
        #[inline(always)]
        pub const fn txwater(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TXWATER."]
        #[inline(always)]
        pub const fn set_txwater(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc RXWATER."]
        #[must_use]
        #[inline(always)]
        pub const fn rxwater(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXWATER."]
        #[inline(always)]
        pub const fn set_rxwater(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Mfifocr {
        #[inline(always)]
        fn default() -> Mfifocr {
            Mfifocr(0)
        }
    }
    impl core::fmt::Debug for Mfifocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mfifocr")
                .field("txwater", &self.txwater())
                .field("rxwater", &self.rxwater())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mfifocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mfifocr {{ txwater: {=u8:?}, rxwater: {=u8:?} }}",
                self.txwater(),
                self.rxwater()
            )
        }
    }
    #[doc = "Master FIFO Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfifosr(pub u32);
    impl Mfifosr {
        #[doc = "desc TXCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TXCNT."]
        #[inline(always)]
        pub const fn set_txcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc RXCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc RXCNT."]
        #[inline(always)]
        pub const fn set_rxcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Mfifosr {
        #[inline(always)]
        fn default() -> Mfifosr {
            Mfifosr(0)
        }
    }
    impl core::fmt::Debug for Mfifosr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mfifosr")
                .field("txcnt", &self.txcnt())
                .field("rxcnt", &self.rxcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mfifosr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mfifosr {{ txcnt: {=u8:?}, rxcnt: {=u8:?} }}",
                self.txcnt(),
                self.rxcnt()
            )
        }
    }
    #[doc = "Master Interrupt Flag Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Micr(pub u32);
    impl Micr {
        #[doc = "desc PACKET."]
        #[must_use]
        #[inline(always)]
        pub const fn packet(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PACKET."]
        #[inline(always)]
        pub const fn set_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NACK."]
        #[must_use]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NACK."]
        #[inline(always)]
        pub const fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ARBI."]
        #[must_use]
        #[inline(always)]
        pub const fn arbi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARBI."]
        #[inline(always)]
        pub const fn set_arbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PINLOW."]
        #[must_use]
        #[inline(always)]
        pub const fn pinlow(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINLOW."]
        #[inline(always)]
        pub const fn set_pinlow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc MATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCH."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Micr {
        #[inline(always)]
        fn default() -> Micr {
            Micr(0)
        }
    }
    impl core::fmt::Debug for Micr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Micr")
                .field("packet", &self.packet())
                .field("stop", &self.stop())
                .field("nack", &self.nack())
                .field("arbi", &self.arbi())
                .field("fifo", &self.fifo())
                .field("pinlow", &self.pinlow())
                .field("match_", &self.match_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Micr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Micr {{ packet: {=bool:?}, stop: {=bool:?}, nack: {=bool:?}, arbi: {=bool:?}, fifo: {=bool:?}, pinlow: {=bool:?}, match_: {=bool:?} }}",
                self.packet(),
                self.stop(),
                self.nack(),
                self.arbi(),
                self.fifo(),
                self.pinlow(),
                self.match_()
            )
        }
    }
    #[doc = "Master Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mier(pub u32);
    impl Mier {
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
        #[doc = "desc PACKET."]
        #[must_use]
        #[inline(always)]
        pub const fn packet(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PACKET."]
        #[inline(always)]
        pub const fn set_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NACK."]
        #[must_use]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NACK."]
        #[inline(always)]
        pub const fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ARBI."]
        #[must_use]
        #[inline(always)]
        pub const fn arbi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARBI."]
        #[inline(always)]
        pub const fn set_arbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PINLOW."]
        #[must_use]
        #[inline(always)]
        pub const fn pinlow(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINLOW."]
        #[inline(always)]
        pub const fn set_pinlow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc MATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCH."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Mier {
        #[inline(always)]
        fn default() -> Mier {
            Mier(0)
        }
    }
    impl core::fmt::Debug for Mier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mier")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("packet", &self.packet())
                .field("stop", &self.stop())
                .field("nack", &self.nack())
                .field("arbi", &self.arbi())
                .field("fifo", &self.fifo())
                .field("pinlow", &self.pinlow())
                .field("match_", &self.match_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mier {{ txe: {=bool:?}, rxne: {=bool:?}, packet: {=bool:?}, stop: {=bool:?}, nack: {=bool:?}, arbi: {=bool:?}, fifo: {=bool:?}, pinlow: {=bool:?}, match_: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.packet(),
                self.stop(),
                self.nack(),
                self.arbi(),
                self.fifo(),
                self.pinlow(),
                self.match_()
            )
        }
    }
    #[doc = "Master Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
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
        #[doc = "desc PACKET."]
        #[must_use]
        #[inline(always)]
        pub const fn packet(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PACKET."]
        #[inline(always)]
        pub const fn set_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc NACK."]
        #[must_use]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc NACK."]
        #[inline(always)]
        pub const fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ARBI."]
        #[must_use]
        #[inline(always)]
        pub const fn arbi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ARBI."]
        #[inline(always)]
        pub const fn set_arbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PINLOW."]
        #[must_use]
        #[inline(always)]
        pub const fn pinlow(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINLOW."]
        #[inline(always)]
        pub const fn set_pinlow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc MATCH."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc MATCH."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc MSTBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn mstbusy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSTBUSY."]
        #[inline(always)]
        pub const fn set_mstbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc BUSBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busbusy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSBUSY."]
        #[inline(always)]
        pub const fn set_busbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("packet", &self.packet())
                .field("stop", &self.stop())
                .field("nack", &self.nack())
                .field("arbi", &self.arbi())
                .field("fifo", &self.fifo())
                .field("pinlow", &self.pinlow())
                .field("match_", &self.match_())
                .field("mstbusy", &self.mstbusy())
                .field("busbusy", &self.busbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misr {{ txe: {=bool:?}, rxne: {=bool:?}, packet: {=bool:?}, stop: {=bool:?}, nack: {=bool:?}, arbi: {=bool:?}, fifo: {=bool:?}, pinlow: {=bool:?}, match_: {=bool:?}, mstbusy: {=bool:?}, busbusy: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.packet(),
                self.stop(),
                self.nack(),
                self.arbi(),
                self.fifo(),
                self.pinlow(),
                self.match_(),
                self.mstbusy(),
                self.busbusy()
            )
        }
    }
    #[doc = "Master Match Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmatch(pub u32);
    impl Mmatch {
        #[doc = "desc MATCH0."]
        #[must_use]
        #[inline(always)]
        pub const fn match0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc MATCH0."]
        #[inline(always)]
        pub const fn set_match0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc MATCH1."]
        #[must_use]
        #[inline(always)]
        pub const fn match1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc MATCH1."]
        #[inline(always)]
        pub const fn set_match1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Mmatch {
        #[inline(always)]
        fn default() -> Mmatch {
            Mmatch(0)
        }
    }
    impl core::fmt::Debug for Mmatch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mmatch")
                .field("match0", &self.match0())
                .field("match1", &self.match1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mmatch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mmatch {{ match0: {=u8:?}, match1: {=u8:?} }}",
                self.match0(),
                self.match1()
            )
        }
    }
    #[doc = "Master Rx Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrdr(pub u32);
    impl Mrdr {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc EMPTY."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc EMPTY."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Mrdr {
        #[inline(always)]
        fn default() -> Mrdr {
            Mrdr(0)
        }
    }
    impl core::fmt::Debug for Mrdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrdr")
                .field("data", &self.data())
                .field("empty", &self.empty())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mrdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mrdr {{ data: {=u8:?}, empty: {=bool:?} }}",
                self.data(),
                self.empty()
            )
        }
    }
    #[doc = "Master Tx Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtdr(pub u32);
    impl Mtdr {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> super::vals::Command {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Command::from_bits(val as u8)
        }
        #[doc = "desc CMD."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: super::vals::Command) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Mtdr {
        #[inline(always)]
        fn default() -> Mtdr {
            Mtdr(0)
        }
    }
    impl core::fmt::Debug for Mtdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtdr")
                .field("data", &self.data())
                .field("cmd", &self.cmd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtdr {{ data: {=u8:?}, cmd: {:?} }}",
                self.data(),
                self.cmd()
            )
        }
    }
    #[doc = "desc SAMR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Samr(pub u32);
    impl Samr {
        #[doc = "desc ADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn addr0(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc ADDR0."]
        #[inline(always)]
        pub const fn set_addr0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
        }
        #[doc = "desc ADDR1."]
        #[must_use]
        #[inline(always)]
        pub const fn addr1(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc ADDR1."]
        #[inline(always)]
        pub const fn set_addr1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 17usize)) | (((val as u32) & 0x03ff) << 17usize);
        }
    }
    impl Default for Samr {
        #[inline(always)]
        fn default() -> Samr {
            Samr(0)
        }
    }
    impl core::fmt::Debug for Samr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Samr")
                .field("addr0", &self.addr0())
                .field("addr1", &self.addr1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Samr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Samr {{ addr0: {=u16:?}, addr1: {=u16:?} }}",
                self.addr0(),
                self.addr1()
            )
        }
    }
    #[doc = "desc SASR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sasr(pub u32);
    impl Sasr {
        #[doc = "desc RADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn raddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc RADDR."]
        #[inline(always)]
        pub const fn set_raddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc ANV."]
        #[must_use]
        #[inline(always)]
        pub const fn anv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ANV."]
        #[inline(always)]
        pub const fn set_anv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sasr {
        #[inline(always)]
        fn default() -> Sasr {
            Sasr(0)
        }
    }
    impl core::fmt::Debug for Sasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sasr")
                .field("raddr", &self.raddr())
                .field("anv", &self.anv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sasr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sasr {{ raddr: {=u16:?}, anv: {=bool:?} }}",
                self.raddr(),
                self.anv()
            )
        }
    }
    #[doc = "Slave Control Register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr0(pub u32);
    impl Scr0 {
        #[doc = "desc SEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEN."]
        #[inline(always)]
        pub const fn set_sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESET."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FLTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTEN."]
        #[inline(always)]
        pub const fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CLKSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn clksrc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLKSRC."]
        #[inline(always)]
        pub const fn set_clksrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc TXFIFORST."]
        #[must_use]
        #[inline(always)]
        pub const fn txfiforst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXFIFORST."]
        #[inline(always)]
        pub const fn set_txfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RXFIFORST."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfiforst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXFIFORST."]
        #[inline(always)]
        pub const fn set_rxfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Scr0 {
        #[inline(always)]
        fn default() -> Scr0 {
            Scr0(0)
        }
    }
    impl core::fmt::Debug for Scr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr0")
                .field("sen", &self.sen())
                .field("reset", &self.reset())
                .field("flten", &self.flten())
                .field("clksrc", &self.clksrc())
                .field("txfiforst", &self.txfiforst())
                .field("rxfiforst", &self.rxfiforst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scr0 {{ sen: {=bool:?}, reset: {=bool:?}, flten: {=bool:?}, clksrc: {=u8:?}, txfiforst: {=bool:?}, rxfiforst: {=bool:?} }}",
                self.sen(),
                self.reset(),
                self.flten(),
                self.clksrc(),
                self.txfiforst(),
                self.rxfiforst()
            )
        }
    }
    #[doc = "desc SCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr1(pub u32);
    impl Scr1 {
        #[doc = "desc ADRSTALL."]
        #[must_use]
        #[inline(always)]
        pub const fn adrstall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADRSTALL."]
        #[inline(always)]
        pub const fn set_adrstall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXSTALL."]
        #[must_use]
        #[inline(always)]
        pub const fn rxstall(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXSTALL."]
        #[inline(always)]
        pub const fn set_rxstall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TXSTALL."]
        #[must_use]
        #[inline(always)]
        pub const fn txstall(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXSTALL."]
        #[inline(always)]
        pub const fn set_txstall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ACKSTALL."]
        #[must_use]
        #[inline(always)]
        pub const fn ackstall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACKSTALL."]
        #[inline(always)]
        pub const fn set_ackstall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc GCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn gcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GCEN."]
        #[inline(always)]
        pub const fn set_gcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ALERTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn alerten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALERTEN."]
        #[inline(always)]
        pub const fn set_alerten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TXCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn txcfg(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXCFG."]
        #[inline(always)]
        pub const fn set_txcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RXCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcfg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXCFG."]
        #[inline(always)]
        pub const fn set_rxcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IGNACK."]
        #[must_use]
        #[inline(always)]
        pub const fn ignack(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IGNACK."]
        #[inline(always)]
        pub const fn set_ignack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc ADDRCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn addrcfg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc ADDRCFG."]
        #[inline(always)]
        pub const fn set_addrcfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Scr1 {
        #[inline(always)]
        fn default() -> Scr1 {
            Scr1(0)
        }
    }
    impl core::fmt::Debug for Scr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr1")
                .field("adrstall", &self.adrstall())
                .field("rxstall", &self.rxstall())
                .field("txstall", &self.txstall())
                .field("ackstall", &self.ackstall())
                .field("gcen", &self.gcen())
                .field("alerten", &self.alerten())
                .field("txcfg", &self.txcfg())
                .field("rxcfg", &self.rxcfg())
                .field("ignack", &self.ignack())
                .field("trgsel", &self.trgsel())
                .field("addrcfg", &self.addrcfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scr1 {{ adrstall: {=bool:?}, rxstall: {=bool:?}, txstall: {=bool:?}, ackstall: {=bool:?}, gcen: {=bool:?}, alerten: {=bool:?}, txcfg: {=bool:?}, rxcfg: {=bool:?}, ignack: {=bool:?}, trgsel: {=bool:?}, addrcfg: {=u8:?} }}",
                self.adrstall(),
                self.rxstall(),
                self.txstall(),
                self.ackstall(),
                self.gcen(),
                self.alerten(),
                self.txcfg(),
                self.rxcfg(),
                self.ignack(),
                self.trgsel(),
                self.addrcfg()
            )
        }
    }
    #[doc = "desc SCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr2(pub u32);
    impl Scr2 {
        #[doc = "desc PINCFG."]
        #[must_use]
        #[inline(always)]
        pub const fn pincfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc PINCFG."]
        #[inline(always)]
        pub const fn set_pincfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Scr2 {
        #[inline(always)]
        fn default() -> Scr2 {
            Scr2(0)
        }
    }
    impl core::fmt::Debug for Scr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr2")
                .field("pincfg", &self.pincfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Scr2 {{ pincfg: {=u8:?} }}", self.pincfg())
        }
    }
    #[doc = "desc SCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr3(pub u32);
    impl Scr3 {
        #[doc = "desc CLKHOLD."]
        #[must_use]
        #[inline(always)]
        pub const fn clkhold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CLKHOLD."]
        #[inline(always)]
        pub const fn set_clkhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DATAVD."]
        #[must_use]
        #[inline(always)]
        pub const fn datavd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DATAVD."]
        #[inline(always)]
        pub const fn set_datavd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "desc FLTSCL."]
        #[must_use]
        #[inline(always)]
        pub const fn fltscl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTSCL."]
        #[inline(always)]
        pub const fn set_fltscl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc FLTSDA."]
        #[must_use]
        #[inline(always)]
        pub const fn fltsda(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTSDA."]
        #[inline(always)]
        pub const fn set_fltsda(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Scr3 {
        #[inline(always)]
        fn default() -> Scr3 {
            Scr3(0)
        }
    }
    impl core::fmt::Debug for Scr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr3")
                .field("clkhold", &self.clkhold())
                .field("datavd", &self.datavd())
                .field("fltscl", &self.fltscl())
                .field("fltsda", &self.fltsda())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scr3 {{ clkhold: {=u8:?}, datavd: {=u8:?}, fltscl: {=u8:?}, fltsda: {=u8:?} }}",
                self.clkhold(),
                self.datavd(),
                self.fltscl(),
                self.fltsda()
            )
        }
    }
    #[doc = "desc SDER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sder(pub u32);
    impl Sder {
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
        #[doc = "desc ADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Sder {
        #[inline(always)]
        fn default() -> Sder {
            Sder(0)
        }
    }
    impl core::fmt::Debug for Sder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sder")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sder {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sder {{ txe: {=bool:?}, rxne: {=bool:?}, addr: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.addr()
            )
        }
    }
    #[doc = "desc SICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sicr(pub u32);
    impl Sicr {
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BIT."]
        #[must_use]
        #[inline(always)]
        pub const fn bit(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIT."]
        #[inline(always)]
        pub const fn set_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Sicr {
        #[inline(always)]
        fn default() -> Sicr {
            Sicr(0)
        }
    }
    impl core::fmt::Debug for Sicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sicr")
                .field("restart", &self.restart())
                .field("stop", &self.stop())
                .field("bit", &self.bit())
                .field("fifo", &self.fifo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sicr {{ restart: {=bool:?}, stop: {=bool:?}, bit: {=bool:?}, fifo: {=bool:?} }}",
                self.restart(),
                self.stop(),
                self.bit(),
                self.fifo()
            )
        }
    }
    #[doc = "desc SIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sier(pub u32);
    impl Sier {
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
        #[doc = "desc ADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TACK."]
        #[must_use]
        #[inline(always)]
        pub const fn tack(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TACK."]
        #[inline(always)]
        pub const fn set_tack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BIT."]
        #[must_use]
        #[inline(always)]
        pub const fn bit(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIT."]
        #[inline(always)]
        pub const fn set_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc AM0."]
        #[must_use]
        #[inline(always)]
        pub const fn am0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc AM0."]
        #[inline(always)]
        pub const fn set_am0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc AM1."]
        #[must_use]
        #[inline(always)]
        pub const fn am1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AM1."]
        #[inline(always)]
        pub const fn set_am1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc GC."]
        #[must_use]
        #[inline(always)]
        pub const fn gc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC."]
        #[inline(always)]
        pub const fn set_gc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc ALERT."]
        #[must_use]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALERT."]
        #[inline(always)]
        pub const fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sier {
        #[inline(always)]
        fn default() -> Sier {
            Sier(0)
        }
    }
    impl core::fmt::Debug for Sier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sier")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("addr", &self.addr())
                .field("tack", &self.tack())
                .field("restart", &self.restart())
                .field("stop", &self.stop())
                .field("bit", &self.bit())
                .field("fifo", &self.fifo())
                .field("am0", &self.am0())
                .field("am1", &self.am1())
                .field("gc", &self.gc())
                .field("alert", &self.alert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sier {{ txe: {=bool:?}, rxne: {=bool:?}, addr: {=bool:?}, tack: {=bool:?}, restart: {=bool:?}, stop: {=bool:?}, bit: {=bool:?}, fifo: {=bool:?}, am0: {=bool:?}, am1: {=bool:?}, gc: {=bool:?}, alert: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.addr(),
                self.tack(),
                self.restart(),
                self.stop(),
                self.bit(),
                self.fifo(),
                self.am0(),
                self.am1(),
                self.gc(),
                self.alert()
            )
        }
    }
    #[doc = "desc SISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sisr(pub u32);
    impl Sisr {
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
        #[doc = "desc ADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TACK."]
        #[must_use]
        #[inline(always)]
        pub const fn tack(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TACK."]
        #[inline(always)]
        pub const fn set_tack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BIT."]
        #[must_use]
        #[inline(always)]
        pub const fn bit(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIT."]
        #[inline(always)]
        pub const fn set_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc AM0."]
        #[must_use]
        #[inline(always)]
        pub const fn am0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc AM0."]
        #[inline(always)]
        pub const fn set_am0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc AM1."]
        #[must_use]
        #[inline(always)]
        pub const fn am1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AM1."]
        #[inline(always)]
        pub const fn set_am1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc GC."]
        #[must_use]
        #[inline(always)]
        pub const fn gc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC."]
        #[inline(always)]
        pub const fn set_gc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc ALERT."]
        #[must_use]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALERT."]
        #[inline(always)]
        pub const fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SLVBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn slvbusy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SLVBUSY."]
        #[inline(always)]
        pub const fn set_slvbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc BUSBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busbusy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSBUSY."]
        #[inline(always)]
        pub const fn set_busbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Sisr {
        #[inline(always)]
        fn default() -> Sisr {
            Sisr(0)
        }
    }
    impl core::fmt::Debug for Sisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sisr")
                .field("txe", &self.txe())
                .field("rxne", &self.rxne())
                .field("addr", &self.addr())
                .field("tack", &self.tack())
                .field("restart", &self.restart())
                .field("stop", &self.stop())
                .field("bit", &self.bit())
                .field("fifo", &self.fifo())
                .field("am0", &self.am0())
                .field("am1", &self.am1())
                .field("gc", &self.gc())
                .field("alert", &self.alert())
                .field("slvbusy", &self.slvbusy())
                .field("busbusy", &self.busbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sisr {{ txe: {=bool:?}, rxne: {=bool:?}, addr: {=bool:?}, tack: {=bool:?}, restart: {=bool:?}, stop: {=bool:?}, bit: {=bool:?}, fifo: {=bool:?}, am0: {=bool:?}, am1: {=bool:?}, gc: {=bool:?}, alert: {=bool:?}, slvbusy: {=bool:?}, busbusy: {=bool:?} }}",
                self.txe(),
                self.rxne(),
                self.addr(),
                self.tack(),
                self.restart(),
                self.stop(),
                self.bit(),
                self.fifo(),
                self.am0(),
                self.am1(),
                self.gc(),
                self.alert(),
                self.slvbusy(),
                self.busbusy()
            )
        }
    }
    #[doc = "desc SRDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srdr(pub u32);
    impl Srdr {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc RXEMPTY."]
        #[must_use]
        #[inline(always)]
        pub const fn rxempty(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXEMPTY."]
        #[inline(always)]
        pub const fn set_rxempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SOF."]
        #[must_use]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOF."]
        #[inline(always)]
        pub const fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Srdr {
        #[inline(always)]
        fn default() -> Srdr {
            Srdr(0)
        }
    }
    impl core::fmt::Debug for Srdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srdr")
                .field("data", &self.data())
                .field("rxempty", &self.rxempty())
                .field("sof", &self.sof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Srdr {{ data: {=u8:?}, rxempty: {=bool:?}, sof: {=bool:?} }}",
                self.data(),
                self.rxempty(),
                self.sof()
            )
        }
    }
    #[doc = "desc STAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star(pub u32);
    impl Star {
        #[doc = "desc TXNACK."]
        #[must_use]
        #[inline(always)]
        pub const fn txnack(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXNACK."]
        #[inline(always)]
        pub const fn set_txnack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Star {
        #[inline(always)]
        fn default() -> Star {
            Star(0)
        }
    }
    impl core::fmt::Debug for Star {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Star")
                .field("txnack", &self.txnack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Star {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Star {{ txnack: {=bool:?} }}", self.txnack())
        }
    }
    #[doc = "desc STDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stdr(pub u32);
    impl Stdr {
        #[doc = "desc DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Stdr {
        #[inline(always)]
        fn default() -> Stdr {
            Stdr(0)
        }
    }
    impl core::fmt::Debug for Stdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stdr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Stdr {{ data: {=u8:?} }}", self.data())
        }
    }
}
pub mod vals {
    #[doc = "MTDR commands from CW32L012 UM CN V1.4 section 23.8.11, printed page 552. Encodings 6 and 7 are not specified."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Command {
        #[doc = "Transmit DATA as one byte."]
        Transmit = 0x0,
        #[doc = "Receive DATA+1 bytes into MRDR."]
        Receive = 0x01,
        #[doc = "Generate STOP."]
        Stop = 0x02,
        #[doc = "Receive DATA+1 bytes without storing in MRDR."]
        ReceiveDiscard = 0x03,
        #[doc = "Generate START or repeated START and transmit DATA, expecting ACK."]
        StartExpectAck = 0x04,
        #[doc = "Generate START or repeated START and transmit DATA, expecting NACK."]
        StartExpectNack = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Command {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Command {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Command {
        #[inline(always)]
        fn from(val: u8) -> Command {
            Command::from_bits(val)
        }
    }
    impl From<Command> for u8 {
        #[inline(always)]
        fn from(val: Command) -> u8 {
            Command::to_bits(val)
        }
    }
}
impl regs::Micr {
    /// Source-reviewed hardware reset value; distinct from zero Default.
    #[inline(always)]
    pub const fn reset_value() -> Self {
        Self(32516)
    }
    /// No-op command preserving all events and documented reserved bits.
    /// Change only the intended command fields before writing; never RMW.
    #[inline(always)]
    pub const fn write_noop() -> Self {
        Self(32516)
    }
}
