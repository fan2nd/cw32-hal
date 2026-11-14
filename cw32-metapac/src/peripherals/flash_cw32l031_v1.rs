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
