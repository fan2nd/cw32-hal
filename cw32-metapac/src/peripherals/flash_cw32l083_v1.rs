#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc FLASH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
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
    #[doc = "desc PAGELOCK1."]
    #[inline(always)]
    pub const fn pagelock1(self) -> crate::common::Reg<regs::Pagelock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PAGELOCK2."]
    #[inline(always)]
    pub const fn pagelock2(self) -> crate::common::Reg<regs::Pagelock2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PAGELOCK3."]
    #[inline(always)]
    pub const fn pagelock3(self) -> crate::common::Reg<regs::Pagelock3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc PAGELOCK4."]
    #[inline(always)]
    pub const fn pagelock4(self) -> crate::common::Reg<regs::Pagelock4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc ISR."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STANDBY."]
        #[must_use]
        #[inline(always)]
        pub const fn standby(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc STANDBY."]
        #[inline(always)]
        pub const fn set_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SECURITY."]
        #[must_use]
        #[inline(always)]
        pub const fn security(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc SECURITY."]
        #[inline(always)]
        pub const fn set_security(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("mode", &self.mode())
                .field("standby", &self.standby())
                .field("busy", &self.busy())
                .field("security", &self.security())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ mode: {:?}, standby: {=bool:?}, busy: {=bool:?}, security: {=u8:?}, key: {=u16:?} }}",
                self.mode(),
                self.standby(),
                self.busy(),
                self.security(),
                self.key()
            )
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc WAIT."]
        #[must_use]
        #[inline(always)]
        pub const fn wait(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc WAIT."]
        #[inline(always)]
        pub const fn set_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("wait", &self.wait())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ wait: {=u8:?}, key: {=u16:?} }}",
                self.wait(),
                self.key()
            )
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc PC."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PAGELOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn pagelock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAGELOCK."]
        #[inline(always)]
        pub const fn set_pagelock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PROG."]
        #[must_use]
        #[inline(always)]
        pub const fn prog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PROG."]
        #[inline(always)]
        pub const fn set_prog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("pc", &self.pc())
                .field("pagelock", &self.pagelock())
                .field("prog", &self.prog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ pc: {=bool:?}, pagelock: {=bool:?}, prog: {=bool:?} }}",
                self.pc(),
                self.pagelock(),
                self.prog()
            )
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc PC."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PAGELOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn pagelock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAGELOCK."]
        #[inline(always)]
        pub const fn set_pagelock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PROG."]
        #[must_use]
        #[inline(always)]
        pub const fn prog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PROG."]
        #[inline(always)]
        pub const fn set_prog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("pc", &self.pc())
                .field("pagelock", &self.pagelock())
                .field("prog", &self.prog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ pc: {=bool:?}, pagelock: {=bool:?}, prog: {=bool:?} }}",
                self.pc(),
                self.pagelock(),
                self.prog()
            )
        }
    }
    #[doc = "desc ISR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc PC."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PAGELOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn pagelock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PAGELOCK."]
        #[inline(always)]
        pub const fn set_pagelock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PROG."]
        #[must_use]
        #[inline(always)]
        pub const fn prog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PROG."]
        #[inline(always)]
        pub const fn set_prog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("pc", &self.pc())
                .field("pagelock", &self.pagelock())
                .field("prog", &self.prog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ pc: {=bool:?}, pagelock: {=bool:?}, prog: {=bool:?} }}",
                self.pc(),
                self.pagelock(),
                self.prog()
            )
        }
    }
    #[doc = "desc PAGELOCK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock1(pub u32);
    impl Pagelock1 {
        #[doc = "Page0 - 7."]
        #[must_use]
        #[inline(always)]
        pub const fn lock0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page0 - 7."]
        #[inline(always)]
        pub const fn set_lock0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page8 - 15."]
        #[must_use]
        #[inline(always)]
        pub const fn lock1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page8 - 15."]
        #[inline(always)]
        pub const fn set_lock1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page16 - 23."]
        #[must_use]
        #[inline(always)]
        pub const fn lock2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page16 - 23."]
        #[inline(always)]
        pub const fn set_lock2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page24 - 31."]
        #[must_use]
        #[inline(always)]
        pub const fn lock3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page24 - 31."]
        #[inline(always)]
        pub const fn set_lock3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page32 - 39."]
        #[must_use]
        #[inline(always)]
        pub const fn lock4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page32 - 39."]
        #[inline(always)]
        pub const fn set_lock4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page40 - 47."]
        #[must_use]
        #[inline(always)]
        pub const fn lock5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page40 - 47."]
        #[inline(always)]
        pub const fn set_lock5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page48 - 55."]
        #[must_use]
        #[inline(always)]
        pub const fn lock6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page48 - 55."]
        #[inline(always)]
        pub const fn set_lock6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page56 - 63."]
        #[must_use]
        #[inline(always)]
        pub const fn lock7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page56 - 63."]
        #[inline(always)]
        pub const fn set_lock7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page64 - 71."]
        #[must_use]
        #[inline(always)]
        pub const fn lock8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page64 - 71."]
        #[inline(always)]
        pub const fn set_lock8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page72 - 79."]
        #[must_use]
        #[inline(always)]
        pub const fn lock9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page72 - 79."]
        #[inline(always)]
        pub const fn set_lock9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page80 - 87."]
        #[must_use]
        #[inline(always)]
        pub const fn lock10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page80 - 87."]
        #[inline(always)]
        pub const fn set_lock10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page88 - 95."]
        #[must_use]
        #[inline(always)]
        pub const fn lock11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page88 - 95."]
        #[inline(always)]
        pub const fn set_lock11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page96 - 103."]
        #[must_use]
        #[inline(always)]
        pub const fn lock12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page96 - 103."]
        #[inline(always)]
        pub const fn set_lock12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page104 - 111."]
        #[must_use]
        #[inline(always)]
        pub const fn lock13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page104 - 111."]
        #[inline(always)]
        pub const fn set_lock13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page112 - 119."]
        #[must_use]
        #[inline(always)]
        pub const fn lock14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page112 - 119."]
        #[inline(always)]
        pub const fn set_lock14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page120 - 127."]
        #[must_use]
        #[inline(always)]
        pub const fn lock15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page120 - 127."]
        #[inline(always)]
        pub const fn set_lock15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pagelock1 {
        #[inline(always)]
        fn default() -> Pagelock1 {
            Pagelock1(0)
        }
    }
    impl core::fmt::Debug for Pagelock1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock1")
                .field("lock0", &self.lock0())
                .field("lock1", &self.lock1())
                .field("lock2", &self.lock2())
                .field("lock3", &self.lock3())
                .field("lock4", &self.lock4())
                .field("lock5", &self.lock5())
                .field("lock6", &self.lock6())
                .field("lock7", &self.lock7())
                .field("lock8", &self.lock8())
                .field("lock9", &self.lock9())
                .field("lock10", &self.lock10())
                .field("lock11", &self.lock11())
                .field("lock12", &self.lock12())
                .field("lock13", &self.lock13())
                .field("lock14", &self.lock14())
                .field("lock15", &self.lock15())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pagelock1 {{ lock0: {=bool:?}, lock1: {=bool:?}, lock2: {=bool:?}, lock3: {=bool:?}, lock4: {=bool:?}, lock5: {=bool:?}, lock6: {=bool:?}, lock7: {=bool:?}, lock8: {=bool:?}, lock9: {=bool:?}, lock10: {=bool:?}, lock11: {=bool:?}, lock12: {=bool:?}, lock13: {=bool:?}, lock14: {=bool:?}, lock15: {=bool:?}, key: {=u16:?} }}",
                self.lock0(),
                self.lock1(),
                self.lock2(),
                self.lock3(),
                self.lock4(),
                self.lock5(),
                self.lock6(),
                self.lock7(),
                self.lock8(),
                self.lock9(),
                self.lock10(),
                self.lock11(),
                self.lock12(),
                self.lock13(),
                self.lock14(),
                self.lock15(),
                self.key()
            )
        }
    }
    #[doc = "desc PAGELOCK2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock2(pub u32);
    impl Pagelock2 {
        #[doc = "Page128 - 135."]
        #[must_use]
        #[inline(always)]
        pub const fn lock16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page128 - 135."]
        #[inline(always)]
        pub const fn set_lock16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page136 - 143."]
        #[must_use]
        #[inline(always)]
        pub const fn lock17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page136 - 143."]
        #[inline(always)]
        pub const fn set_lock17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page144 - 151."]
        #[must_use]
        #[inline(always)]
        pub const fn lock18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page144 - 151."]
        #[inline(always)]
        pub const fn set_lock18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page152 - 159."]
        #[must_use]
        #[inline(always)]
        pub const fn lock19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page152 - 159."]
        #[inline(always)]
        pub const fn set_lock19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page160 - 167."]
        #[must_use]
        #[inline(always)]
        pub const fn lock20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page160 - 167."]
        #[inline(always)]
        pub const fn set_lock20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page168 - 175."]
        #[must_use]
        #[inline(always)]
        pub const fn lock21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page168 - 175."]
        #[inline(always)]
        pub const fn set_lock21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page176 - 183."]
        #[must_use]
        #[inline(always)]
        pub const fn lock22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page176 - 183."]
        #[inline(always)]
        pub const fn set_lock22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page184 - 191."]
        #[must_use]
        #[inline(always)]
        pub const fn lock23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page184 - 191."]
        #[inline(always)]
        pub const fn set_lock23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page192 - 199."]
        #[must_use]
        #[inline(always)]
        pub const fn lock24(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page192 - 199."]
        #[inline(always)]
        pub const fn set_lock24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page200 - 207."]
        #[must_use]
        #[inline(always)]
        pub const fn lock25(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page200 - 207."]
        #[inline(always)]
        pub const fn set_lock25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page208 - 215."]
        #[must_use]
        #[inline(always)]
        pub const fn lock26(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page208 - 215."]
        #[inline(always)]
        pub const fn set_lock26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page216 - 223."]
        #[must_use]
        #[inline(always)]
        pub const fn lock27(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page216 - 223."]
        #[inline(always)]
        pub const fn set_lock27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page224 - 231."]
        #[must_use]
        #[inline(always)]
        pub const fn lock28(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page224 - 231."]
        #[inline(always)]
        pub const fn set_lock28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page232 - 239."]
        #[must_use]
        #[inline(always)]
        pub const fn lock29(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page232 - 239."]
        #[inline(always)]
        pub const fn set_lock29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page240 - 247."]
        #[must_use]
        #[inline(always)]
        pub const fn lock30(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page240 - 247."]
        #[inline(always)]
        pub const fn set_lock30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page248 - 255."]
        #[must_use]
        #[inline(always)]
        pub const fn lock31(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page248 - 255."]
        #[inline(always)]
        pub const fn set_lock31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pagelock2 {
        #[inline(always)]
        fn default() -> Pagelock2 {
            Pagelock2(0)
        }
    }
    impl core::fmt::Debug for Pagelock2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock2")
                .field("lock16", &self.lock16())
                .field("lock17", &self.lock17())
                .field("lock18", &self.lock18())
                .field("lock19", &self.lock19())
                .field("lock20", &self.lock20())
                .field("lock21", &self.lock21())
                .field("lock22", &self.lock22())
                .field("lock23", &self.lock23())
                .field("lock24", &self.lock24())
                .field("lock25", &self.lock25())
                .field("lock26", &self.lock26())
                .field("lock27", &self.lock27())
                .field("lock28", &self.lock28())
                .field("lock29", &self.lock29())
                .field("lock30", &self.lock30())
                .field("lock31", &self.lock31())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pagelock2 {{ lock16: {=bool:?}, lock17: {=bool:?}, lock18: {=bool:?}, lock19: {=bool:?}, lock20: {=bool:?}, lock21: {=bool:?}, lock22: {=bool:?}, lock23: {=bool:?}, lock24: {=bool:?}, lock25: {=bool:?}, lock26: {=bool:?}, lock27: {=bool:?}, lock28: {=bool:?}, lock29: {=bool:?}, lock30: {=bool:?}, lock31: {=bool:?}, key: {=u16:?} }}",
                self.lock16(),
                self.lock17(),
                self.lock18(),
                self.lock19(),
                self.lock20(),
                self.lock21(),
                self.lock22(),
                self.lock23(),
                self.lock24(),
                self.lock25(),
                self.lock26(),
                self.lock27(),
                self.lock28(),
                self.lock29(),
                self.lock30(),
                self.lock31(),
                self.key()
            )
        }
    }
    #[doc = "desc PAGELOCK3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock3(pub u32);
    impl Pagelock3 {
        #[doc = "Page256 - 263."]
        #[must_use]
        #[inline(always)]
        pub const fn lock32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page256 - 263."]
        #[inline(always)]
        pub const fn set_lock32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page264 - 271."]
        #[must_use]
        #[inline(always)]
        pub const fn lock33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page264 - 271."]
        #[inline(always)]
        pub const fn set_lock33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page272 - 279."]
        #[must_use]
        #[inline(always)]
        pub const fn lock34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page272 - 279."]
        #[inline(always)]
        pub const fn set_lock34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page280 - 287."]
        #[must_use]
        #[inline(always)]
        pub const fn lock35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page280 - 287."]
        #[inline(always)]
        pub const fn set_lock35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page288 - 295."]
        #[must_use]
        #[inline(always)]
        pub const fn lock36(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page288 - 295."]
        #[inline(always)]
        pub const fn set_lock36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page296 - 303."]
        #[must_use]
        #[inline(always)]
        pub const fn lock37(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page296 - 303."]
        #[inline(always)]
        pub const fn set_lock37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page304 - 311."]
        #[must_use]
        #[inline(always)]
        pub const fn lock38(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page304 - 311."]
        #[inline(always)]
        pub const fn set_lock38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page312 - 319."]
        #[must_use]
        #[inline(always)]
        pub const fn lock39(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page312 - 319."]
        #[inline(always)]
        pub const fn set_lock39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page320 - 327."]
        #[must_use]
        #[inline(always)]
        pub const fn lock40(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page320 - 327."]
        #[inline(always)]
        pub const fn set_lock40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page328 - 335."]
        #[must_use]
        #[inline(always)]
        pub const fn lock41(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page328 - 335."]
        #[inline(always)]
        pub const fn set_lock41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page336 - 343."]
        #[must_use]
        #[inline(always)]
        pub const fn lock42(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page336 - 343."]
        #[inline(always)]
        pub const fn set_lock42(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page344 - 351."]
        #[must_use]
        #[inline(always)]
        pub const fn lock43(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page344 - 351."]
        #[inline(always)]
        pub const fn set_lock43(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page352 - 359."]
        #[must_use]
        #[inline(always)]
        pub const fn lock44(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page352 - 359."]
        #[inline(always)]
        pub const fn set_lock44(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page360 - 367."]
        #[must_use]
        #[inline(always)]
        pub const fn lock45(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page360 - 367."]
        #[inline(always)]
        pub const fn set_lock45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page368 - 375."]
        #[must_use]
        #[inline(always)]
        pub const fn lock46(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page368 - 375."]
        #[inline(always)]
        pub const fn set_lock46(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page376 - 383."]
        #[must_use]
        #[inline(always)]
        pub const fn lock47(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page376 - 383."]
        #[inline(always)]
        pub const fn set_lock47(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pagelock3 {
        #[inline(always)]
        fn default() -> Pagelock3 {
            Pagelock3(0)
        }
    }
    impl core::fmt::Debug for Pagelock3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock3")
                .field("lock32", &self.lock32())
                .field("lock33", &self.lock33())
                .field("lock34", &self.lock34())
                .field("lock35", &self.lock35())
                .field("lock36", &self.lock36())
                .field("lock37", &self.lock37())
                .field("lock38", &self.lock38())
                .field("lock39", &self.lock39())
                .field("lock40", &self.lock40())
                .field("lock41", &self.lock41())
                .field("lock42", &self.lock42())
                .field("lock43", &self.lock43())
                .field("lock44", &self.lock44())
                .field("lock45", &self.lock45())
                .field("lock46", &self.lock46())
                .field("lock47", &self.lock47())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pagelock3 {{ lock32: {=bool:?}, lock33: {=bool:?}, lock34: {=bool:?}, lock35: {=bool:?}, lock36: {=bool:?}, lock37: {=bool:?}, lock38: {=bool:?}, lock39: {=bool:?}, lock40: {=bool:?}, lock41: {=bool:?}, lock42: {=bool:?}, lock43: {=bool:?}, lock44: {=bool:?}, lock45: {=bool:?}, lock46: {=bool:?}, lock47: {=bool:?}, key: {=u16:?} }}",
                self.lock32(),
                self.lock33(),
                self.lock34(),
                self.lock35(),
                self.lock36(),
                self.lock37(),
                self.lock38(),
                self.lock39(),
                self.lock40(),
                self.lock41(),
                self.lock42(),
                self.lock43(),
                self.lock44(),
                self.lock45(),
                self.lock46(),
                self.lock47(),
                self.key()
            )
        }
    }
    #[doc = "desc PAGELOCK4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock4(pub u32);
    impl Pagelock4 {
        #[doc = "Page384 - 391."]
        #[must_use]
        #[inline(always)]
        pub const fn lock48(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Page384 - 391."]
        #[inline(always)]
        pub const fn set_lock48(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page392 - 399."]
        #[must_use]
        #[inline(always)]
        pub const fn lock49(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page392 - 399."]
        #[inline(always)]
        pub const fn set_lock49(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page400 - 407."]
        #[must_use]
        #[inline(always)]
        pub const fn lock50(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page400 - 407."]
        #[inline(always)]
        pub const fn set_lock50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page408 - 415."]
        #[must_use]
        #[inline(always)]
        pub const fn lock51(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Page408 - 415."]
        #[inline(always)]
        pub const fn set_lock51(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Page416 - 423."]
        #[must_use]
        #[inline(always)]
        pub const fn lock52(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Page416 - 423."]
        #[inline(always)]
        pub const fn set_lock52(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Page424 - 431."]
        #[must_use]
        #[inline(always)]
        pub const fn lock53(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Page424 - 431."]
        #[inline(always)]
        pub const fn set_lock53(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page432 - 439."]
        #[must_use]
        #[inline(always)]
        pub const fn lock54(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Page432 - 439."]
        #[inline(always)]
        pub const fn set_lock54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Page440 - 447."]
        #[must_use]
        #[inline(always)]
        pub const fn lock55(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Page440 - 447."]
        #[inline(always)]
        pub const fn set_lock55(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Page448 - 455."]
        #[must_use]
        #[inline(always)]
        pub const fn lock56(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Page448 - 455."]
        #[inline(always)]
        pub const fn set_lock56(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Page456 - 463."]
        #[must_use]
        #[inline(always)]
        pub const fn lock57(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Page456 - 463."]
        #[inline(always)]
        pub const fn set_lock57(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Page464 - 471."]
        #[must_use]
        #[inline(always)]
        pub const fn lock58(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Page464 - 471."]
        #[inline(always)]
        pub const fn set_lock58(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Page472 - 479."]
        #[must_use]
        #[inline(always)]
        pub const fn lock59(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Page472 - 479."]
        #[inline(always)]
        pub const fn set_lock59(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Page480 - 487."]
        #[must_use]
        #[inline(always)]
        pub const fn lock60(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Page480 - 487."]
        #[inline(always)]
        pub const fn set_lock60(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Page488 - 495."]
        #[must_use]
        #[inline(always)]
        pub const fn lock61(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Page488 - 495."]
        #[inline(always)]
        pub const fn set_lock61(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Page496 - 503."]
        #[must_use]
        #[inline(always)]
        pub const fn lock62(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Page496 - 503."]
        #[inline(always)]
        pub const fn set_lock62(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Page504 - 511."]
        #[must_use]
        #[inline(always)]
        pub const fn lock63(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Page504 - 511."]
        #[inline(always)]
        pub const fn set_lock63(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pagelock4 {
        #[inline(always)]
        fn default() -> Pagelock4 {
            Pagelock4(0)
        }
    }
    impl core::fmt::Debug for Pagelock4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock4")
                .field("lock48", &self.lock48())
                .field("lock49", &self.lock49())
                .field("lock50", &self.lock50())
                .field("lock51", &self.lock51())
                .field("lock52", &self.lock52())
                .field("lock53", &self.lock53())
                .field("lock54", &self.lock54())
                .field("lock55", &self.lock55())
                .field("lock56", &self.lock56())
                .field("lock57", &self.lock57())
                .field("lock58", &self.lock58())
                .field("lock59", &self.lock59())
                .field("lock60", &self.lock60())
                .field("lock61", &self.lock61())
                .field("lock62", &self.lock62())
                .field("lock63", &self.lock63())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pagelock4 {{ lock48: {=bool:?}, lock49: {=bool:?}, lock50: {=bool:?}, lock51: {=bool:?}, lock52: {=bool:?}, lock53: {=bool:?}, lock54: {=bool:?}, lock55: {=bool:?}, lock56: {=bool:?}, lock57: {=bool:?}, lock58: {=bool:?}, lock59: {=bool:?}, lock60: {=bool:?}, lock61: {=bool:?}, lock62: {=bool:?}, lock63: {=bool:?}, key: {=u16:?} }}",
                self.lock48(),
                self.lock49(),
                self.lock50(),
                self.lock51(),
                self.lock52(),
                self.lock53(),
                self.lock54(),
                self.lock55(),
                self.lock56(),
                self.lock57(),
                self.lock58(),
                self.lock59(),
                self.lock60(),
                self.lock61(),
                self.lock62(),
                self.lock63(),
                self.key()
            )
        }
    }
}
pub mod vals {
    #[doc = "Main FLASH operation, qualified by the own FLASH control chapter; see docs/flash-next-batch-audit.json."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Read operation."]
        Read = 0x0,
        #[doc = "Program an erased byte, halfword or word."]
        Program = 0x01,
        #[doc = "Erase the page selected by the triggering store."]
        PageErase = 0x02,
        #[doc = "Erase the whole main array. Not exposed by the reserved-region HAL storage API."]
        ChipErase = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
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
