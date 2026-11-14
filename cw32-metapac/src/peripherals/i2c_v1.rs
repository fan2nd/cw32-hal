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
    #[doc = "desc BRREN."]
    #[inline(always)]
    pub const fn brren(self) -> crate::common::Reg<regs::Brren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc BRR."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Slave Addrress0."]
    #[inline(always)]
    pub const fn addr0(self) -> crate::common::Reg<regs::Addr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Slave Addrress1."]
    #[inline(always)]
    pub const fn addr1(self) -> crate::common::Reg<regs::Addr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Slave Addrress2."]
    #[inline(always)]
    pub const fn addr2(self) -> crate::common::Reg<regs::Addr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Slave Addrress match flag."]
    #[inline(always)]
    pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "Slave Addrress0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr0(pub u32);
    impl Addr0 {
        #[doc = "desc GC."]
        #[must_use]
        #[inline(always)]
        pub const fn gc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC."]
        #[inline(always)]
        pub const fn set_gc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn addr0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc ADDR0."]
        #[inline(always)]
        pub const fn set_addr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Addr0 {
        #[inline(always)]
        fn default() -> Addr0 {
            Addr0(0)
        }
    }
    impl core::fmt::Debug for Addr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr0")
                .field("gc", &self.gc())
                .field("addr0", &self.addr0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Addr0 {{ gc: {=bool:?}, addr0: {=u8:?} }}",
                self.gc(),
                self.addr0()
            )
        }
    }
    #[doc = "Slave Addrress1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr1(pub u32);
    impl Addr1 {
        #[doc = "desc ADDR1."]
        #[must_use]
        #[inline(always)]
        pub const fn addr1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc ADDR1."]
        #[inline(always)]
        pub const fn set_addr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Addr1 {
        #[inline(always)]
        fn default() -> Addr1 {
            Addr1(0)
        }
    }
    impl core::fmt::Debug for Addr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr1")
                .field("addr1", &self.addr1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addr1 {{ addr1: {=u8:?} }}", self.addr1())
        }
    }
    #[doc = "Slave Addrress2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr2(pub u32);
    impl Addr2 {
        #[doc = "desc ADDR2."]
        #[must_use]
        #[inline(always)]
        pub const fn addr2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc ADDR2."]
        #[inline(always)]
        pub const fn set_addr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Addr2 {
        #[inline(always)]
        fn default() -> Addr2 {
            Addr2(0)
        }
    }
    impl core::fmt::Debug for Addr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr2")
                .field("addr2", &self.addr2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addr2 {{ addr2: {=u8:?} }}", self.addr2())
        }
    }
    #[doc = "desc BRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "fSCL = fPCLK / 8 / (BRR+1)."]
        #[must_use]
        #[inline(always)]
        pub const fn brr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "fSCL = fPCLK / 8 / (BRR+1)."]
        #[inline(always)]
        pub const fn set_brr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr").field("brr", &self.brr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brr {{ brr: {=u8:?} }}", self.brr())
        }
    }
    #[doc = "desc BRREN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brren(pub u32);
    impl Brren {
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
    }
    impl Default for Brren {
        #[inline(always)]
        fn default() -> Brren {
            Brren(0)
        }
    }
    impl core::fmt::Debug for Brren {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brren").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brren {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brren {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc FLT."]
        #[must_use]
        #[inline(always)]
        pub const fn flt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLT."]
        #[inline(always)]
        pub const fn set_flt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc AA."]
        #[must_use]
        #[inline(always)]
        pub const fn aa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AA."]
        #[inline(always)]
        pub const fn set_aa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SI."]
        #[must_use]
        #[inline(always)]
        pub const fn si(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SI."]
        #[inline(always)]
        pub const fn set_si(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc STO."]
        #[must_use]
        #[inline(always)]
        pub const fn sto(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc STO."]
        #[inline(always)]
        pub const fn set_sto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc STA."]
        #[must_use]
        #[inline(always)]
        pub const fn sta(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc STA."]
        #[inline(always)]
        pub const fn set_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("flt", &self.flt())
                .field("aa", &self.aa())
                .field("si", &self.si())
                .field("sto", &self.sto())
                .field("sta", &self.sta())
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ flt: {=bool:?}, aa: {=bool:?}, si: {=bool:?}, sto: {=bool:?}, sta: {=bool:?}, en: {=bool:?} }}",
                self.flt(),
                self.aa(),
                self.si(),
                self.sto(),
                self.sta(),
                self.en()
            )
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
        pub const fn dr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DR."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            defmt::write!(f, "Dr {{ dr: {=u8:?} }}", self.dr())
        }
    }
    #[doc = "Slave Addrress match flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Match(pub u32);
    impl Match {
        #[doc = "desc ADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn addr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR0."]
        #[inline(always)]
        pub const fn set_addr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ADDR1."]
        #[must_use]
        #[inline(always)]
        pub const fn addr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR1."]
        #[inline(always)]
        pub const fn set_addr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ADDR2."]
        #[must_use]
        #[inline(always)]
        pub const fn addr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADDR2."]
        #[inline(always)]
        pub const fn set_addr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Match {
        #[inline(always)]
        fn default() -> Match {
            Match(0)
        }
    }
    impl core::fmt::Debug for Match {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Match")
                .field("addr0", &self.addr0())
                .field("addr1", &self.addr1())
                .field("addr2", &self.addr2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Match {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Match {{ addr0: {=bool:?}, addr1: {=bool:?}, addr2: {=bool:?} }}",
                self.addr0(),
                self.addr1(),
                self.addr2()
            )
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "desc STAT."]
        #[must_use]
        #[inline(always)]
        pub const fn stat(&self) -> super::vals::Status {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Status::from_bits(val as u8)
        }
        #[doc = "desc STAT."]
        #[inline(always)]
        pub const fn set_stat(&mut self, val: super::vals::Status) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat").field("stat", &self.stat()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Stat {{ stat: {:?} }}", self.stat())
        }
    }
}
pub mod vals {
    #[doc = "I2C STAT encodings verified in every own-family manual; see docs/i2c-typed-register-evidence.json."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Status(u8);
    impl Status {
        #[doc = "Bus error or an undefined state caused by interference."]
        pub const BusError: Self = Self(0x0);
        #[doc = "START transmitted."]
        pub const Start: Self = Self(0x08);
        #[doc = "Repeated START transmitted."]
        pub const RepeatedStart: Self = Self(0x10);
        #[doc = "SLA+W transmitted; ACK received."]
        pub const WriteAddressAck: Self = Self(0x18);
        #[doc = "SLA+W transmitted; NACK received."]
        pub const WriteAddressNack: Self = Self(0x20);
        #[doc = "Data byte transmitted; ACK received."]
        pub const WriteDataAck: Self = Self(0x28);
        #[doc = "Data byte transmitted; NACK received."]
        pub const WriteDataNack: Self = Self(0x30);
        #[doc = "Arbitration lost while transmitting an address, data, or NACK."]
        pub const ArbitrationLost: Self = Self(0x38);
        #[doc = "SLA+R transmitted; ACK received."]
        pub const ReadAddressAck: Self = Self(0x40);
        #[doc = "SLA+R transmitted; NACK received."]
        pub const ReadAddressNack: Self = Self(0x48);
        #[doc = "Data byte received; ACK returned."]
        pub const ReadDataAck: Self = Self(0x50);
        #[doc = "Data byte received; NACK returned."]
        pub const ReadDataNack: Self = Self(0x58);
        #[doc = "Own SLA+W received; ACK returned."]
        pub const SlaveWriteAddress: Self = Self(0x60);
        #[doc = "Arbitration lost; own SLA+W received; ACK returned."]
        pub const ArbitrationLostSlaveWrite: Self = Self(0x68);
        #[doc = "General-call address received; ACK returned."]
        pub const GeneralCallAddress: Self = Self(0x70);
        #[doc = "Arbitration lost; general-call address received; ACK returned."]
        pub const ArbitrationLostGeneralCall: Self = Self(0x78);
        #[doc = "Own-address data byte received; ACK returned."]
        pub const SlaveWriteDataAck: Self = Self(0x80);
        #[doc = "Own-address data byte received; NACK returned."]
        pub const SlaveWriteDataNack: Self = Self(0x88);
        #[doc = "General-call data byte received; ACK returned."]
        pub const GeneralCallDataAck: Self = Self(0x90);
        #[doc = "General-call data byte received; NACK returned."]
        pub const GeneralCallDataNack: Self = Self(0x98);
        #[doc = "STOP or repeated START received while addressed as a slave."]
        pub const SlaveStopOrRestart: Self = Self(0xa0);
        #[doc = "Own SLA+R received; ACK returned."]
        pub const SlaveReadAddress: Self = Self(0xa8);
        #[doc = "Arbitration lost; own SLA+R received; ACK returned."]
        pub const ArbitrationLostSlaveRead: Self = Self(0xb0);
        #[doc = "Slave data byte transmitted; ACK received."]
        pub const SlaveReadDataAck: Self = Self(0xb8);
        #[doc = "Slave data byte transmitted; NACK received."]
        pub const SlaveReadDataNack: Self = Self(0xc0);
        #[doc = "Last slave data byte transmitted; ACK received."]
        pub const SlaveReadLastDataAck: Self = Self(0xc8);
        #[doc = "No relevant status information; SI is clear."]
        pub const NoInformation: Self = Self(0xf8);
    }
    impl Status {
        pub const fn from_bits(val: u8) -> Status {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("BusError"),
                0x08 => f.write_str("Start"),
                0x10 => f.write_str("RepeatedStart"),
                0x18 => f.write_str("WriteAddressAck"),
                0x20 => f.write_str("WriteAddressNack"),
                0x28 => f.write_str("WriteDataAck"),
                0x30 => f.write_str("WriteDataNack"),
                0x38 => f.write_str("ArbitrationLost"),
                0x40 => f.write_str("ReadAddressAck"),
                0x48 => f.write_str("ReadAddressNack"),
                0x50 => f.write_str("ReadDataAck"),
                0x58 => f.write_str("ReadDataNack"),
                0x60 => f.write_str("SlaveWriteAddress"),
                0x68 => f.write_str("ArbitrationLostSlaveWrite"),
                0x70 => f.write_str("GeneralCallAddress"),
                0x78 => f.write_str("ArbitrationLostGeneralCall"),
                0x80 => f.write_str("SlaveWriteDataAck"),
                0x88 => f.write_str("SlaveWriteDataNack"),
                0x90 => f.write_str("GeneralCallDataAck"),
                0x98 => f.write_str("GeneralCallDataNack"),
                0xa0 => f.write_str("SlaveStopOrRestart"),
                0xa8 => f.write_str("SlaveReadAddress"),
                0xb0 => f.write_str("ArbitrationLostSlaveRead"),
                0xb8 => f.write_str("SlaveReadDataAck"),
                0xc0 => f.write_str("SlaveReadDataNack"),
                0xc8 => f.write_str("SlaveReadLastDataAck"),
                0xf8 => f.write_str("NoInformation"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "BusError"),
                0x08 => defmt::write!(f, "Start"),
                0x10 => defmt::write!(f, "RepeatedStart"),
                0x18 => defmt::write!(f, "WriteAddressAck"),
                0x20 => defmt::write!(f, "WriteAddressNack"),
                0x28 => defmt::write!(f, "WriteDataAck"),
                0x30 => defmt::write!(f, "WriteDataNack"),
                0x38 => defmt::write!(f, "ArbitrationLost"),
                0x40 => defmt::write!(f, "ReadAddressAck"),
                0x48 => defmt::write!(f, "ReadAddressNack"),
                0x50 => defmt::write!(f, "ReadDataAck"),
                0x58 => defmt::write!(f, "ReadDataNack"),
                0x60 => defmt::write!(f, "SlaveWriteAddress"),
                0x68 => defmt::write!(f, "ArbitrationLostSlaveWrite"),
                0x70 => defmt::write!(f, "GeneralCallAddress"),
                0x78 => defmt::write!(f, "ArbitrationLostGeneralCall"),
                0x80 => defmt::write!(f, "SlaveWriteDataAck"),
                0x88 => defmt::write!(f, "SlaveWriteDataNack"),
                0x90 => defmt::write!(f, "GeneralCallDataAck"),
                0x98 => defmt::write!(f, "GeneralCallDataNack"),
                0xa0 => defmt::write!(f, "SlaveStopOrRestart"),
                0xa8 => defmt::write!(f, "SlaveReadAddress"),
                0xb0 => defmt::write!(f, "ArbitrationLostSlaveRead"),
                0xb8 => defmt::write!(f, "SlaveReadDataAck"),
                0xc0 => defmt::write!(f, "SlaveReadDataNack"),
                0xc8 => defmt::write!(f, "SlaveReadLastDataAck"),
                0xf8 => defmt::write!(f, "NoInformation"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
}
