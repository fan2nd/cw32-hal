#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIOF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiof {
    ptr: *mut u8,
}
unsafe impl Send for Gpiof {}
unsafe impl Sync for Gpiof {}
impl Gpiof {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DIR."]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc OPENDRAIN."]
    #[inline(always)]
    pub const fn opendrain(self) -> crate::common::Reg<regs::Opendrain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SPEED."]
    #[inline(always)]
    pub const fn speed(self) -> crate::common::Reg<regs::Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PDR."]
    #[inline(always)]
    pub const fn pdr(self) -> crate::common::Reg<regs::Pdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PUR."]
    #[inline(always)]
    pub const fn pur(self) -> crate::common::Reg<regs::Pur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc AFRL."]
    #[inline(always)]
    pub const fn afrl(self) -> crate::common::Reg<regs::Afrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc ANALOG."]
    #[inline(always)]
    pub const fn analog(self) -> crate::common::Reg<regs::Analog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc DRIVER."]
    #[inline(always)]
    pub const fn driver(self) -> crate::common::Reg<regs::Driver, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn riseie(self) -> crate::common::Reg<regs::Riseie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn fallie(self) -> crate::common::Reg<regs::Fallie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn highie(self) -> crate::common::Reg<regs::Highie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn lowie(self) -> crate::common::Reg<regs::Lowie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc LOCK."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc FILTER."]
    #[inline(always)]
    pub const fn filter(self) -> crate::common::Reg<regs::Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc IDR."]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc ODR."]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc ODRLOWBYTE."]
    #[inline(always)]
    pub const fn odrlowbyte(self) -> crate::common::Reg<regs::Odrlowbyte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc BRR."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc BSRR."]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc TOG."]
    #[inline(always)]
    pub const fn tog(self) -> crate::common::Reg<regs::Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc AFRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afrl(pub u32);
    impl Afrl {
        #[doc = "desc AFR0."]
        #[must_use]
        #[inline(always)]
        pub const fn afr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR0."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc AFR1."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR1."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc AFR3."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR3."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "desc AFR6."]
        #[must_use]
        #[inline(always)]
        pub const fn afr6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR6."]
        #[inline(always)]
        pub const fn set_afr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc AFR7."]
        #[must_use]
        #[inline(always)]
        pub const fn afr7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR7."]
        #[inline(always)]
        pub const fn set_afr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Afrl {
        #[inline(always)]
        fn default() -> Afrl {
            Afrl(0)
        }
    }
    impl core::fmt::Debug for Afrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afrl")
                .field("afr0", &self.afr0())
                .field("afr1", &self.afr1())
                .field("afr3", &self.afr3())
                .field("afr6", &self.afr6())
                .field("afr7", &self.afr7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afrl {{ afr0: {=u8:?}, afr1: {=u8:?}, afr3: {=u8:?}, afr6: {=u8:?}, afr7: {=u8:?} }}",
                self.afr0(),
                self.afr1(),
                self.afr3(),
                self.afr6(),
                self.afr7()
            )
        }
    }
    #[doc = "desc ANALOG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Analog(pub u32);
    impl Analog {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Analog {
        #[inline(always)]
        fn default() -> Analog {
            Analog(0)
        }
    }
    impl core::fmt::Debug for Analog {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Analog")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Analog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Analog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc BRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "desc BRR0."]
        #[must_use]
        #[inline(always)]
        pub const fn brr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR0."]
        #[inline(always)]
        pub const fn set_brr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BRR1."]
        #[must_use]
        #[inline(always)]
        pub const fn brr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR1."]
        #[inline(always)]
        pub const fn set_brr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BRR3."]
        #[must_use]
        #[inline(always)]
        pub const fn brr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR3."]
        #[inline(always)]
        pub const fn set_brr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BRR6."]
        #[must_use]
        #[inline(always)]
        pub const fn brr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR6."]
        #[inline(always)]
        pub const fn set_brr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BRR7."]
        #[must_use]
        #[inline(always)]
        pub const fn brr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR7."]
        #[inline(always)]
        pub const fn set_brr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            f.debug_struct("Brr")
                .field("brr0", &self.brr0())
                .field("brr1", &self.brr1())
                .field("brr3", &self.brr3())
                .field("brr6", &self.brr6())
                .field("brr7", &self.brr7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Brr {{ brr0: {=bool:?}, brr1: {=bool:?}, brr3: {=bool:?}, brr6: {=bool:?}, brr7: {=bool:?} }}",
                self.brr0(),
                self.brr1(),
                self.brr3(),
                self.brr6(),
                self.brr7()
            )
        }
    }
    #[doc = "desc BSRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "desc BSS0."]
        #[must_use]
        #[inline(always)]
        pub const fn bss0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS0."]
        #[inline(always)]
        pub const fn set_bss0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BSS1."]
        #[must_use]
        #[inline(always)]
        pub const fn bss1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS1."]
        #[inline(always)]
        pub const fn set_bss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BSS3."]
        #[must_use]
        #[inline(always)]
        pub const fn bss3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS3."]
        #[inline(always)]
        pub const fn set_bss3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BSS6."]
        #[must_use]
        #[inline(always)]
        pub const fn bss6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS6."]
        #[inline(always)]
        pub const fn set_bss6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BSS7."]
        #[must_use]
        #[inline(always)]
        pub const fn bss7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS7."]
        #[inline(always)]
        pub const fn set_bss7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Bsrr {
        #[inline(always)]
        fn default() -> Bsrr {
            Bsrr(0)
        }
    }
    impl core::fmt::Debug for Bsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsrr")
                .field("bss0", &self.bss0())
                .field("bss1", &self.bss1())
                .field("bss3", &self.bss3())
                .field("bss6", &self.bss6())
                .field("bss7", &self.bss7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsrr {{ bss0: {=bool:?}, bss1: {=bool:?}, bss3: {=bool:?}, bss6: {=bool:?}, bss7: {=bool:?} }}",
                self.bss0(),
                self.bss1(),
                self.bss3(),
                self.bss6(),
                self.bss7()
            )
        }
    }
    #[doc = "desc DIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dir(pub u32);
    impl Dir {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Dir {
        #[inline(always)]
        fn default() -> Dir {
            Dir(0)
        }
    }
    impl core::fmt::Debug for Dir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dir")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dir {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc DRIVER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Driver(pub u32);
    impl Driver {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Driver {
        #[inline(always)]
        fn default() -> Driver {
            Driver(0)
        }
    }
    impl core::fmt::Debug for Driver {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Driver")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Driver {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Driver {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fallie(pub u32);
    impl Fallie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fallie {
        #[inline(always)]
        fn default() -> Fallie {
            Fallie(0)
        }
    }
    impl core::fmt::Debug for Fallie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fallie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fallie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fallie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc FILTER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filter(pub u32);
    impl Filter {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Filter {
        #[inline(always)]
        fn default() -> Filter {
            Filter(0)
        }
    }
    impl core::fmt::Debug for Filter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Filter")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("fltclk", &self.fltclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filter {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, fltclk: {=u8:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7(),
                self.fltclk()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Highie(pub u32);
    impl Highie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Highie {
        #[inline(always)]
        fn default() -> Highie {
            Highie(0)
        }
    }
    impl core::fmt::Debug for Highie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Highie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Highie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Highie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "Interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc IDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idr(pub u32);
    impl Idr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Idr {
        #[inline(always)]
        fn default() -> Idr {
            Idr(0)
        }
    }
    impl core::fmt::Debug for Idr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idr")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
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
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc LOCK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "KEY = 0x5A5A."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "KEY = 0x5A5A."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    impl core::fmt::Debug for Lock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lock")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lock {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, key: {=u16:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7(),
                self.key()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lowie(pub u32);
    impl Lowie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lowie {
        #[inline(always)]
        fn default() -> Lowie {
            Lowie(0)
        }
    }
    impl core::fmt::Debug for Lowie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lowie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lowie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lowie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc ODR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odr(pub u32);
    impl Odr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Odr {
        #[inline(always)]
        fn default() -> Odr {
            Odr(0)
        }
    }
    impl core::fmt::Debug for Odr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odr")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Odr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc ODRLOWBYTE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odrlowbyte(pub u8);
    impl Odrlowbyte {
        #[doc = "desc LOWBYTE."]
        #[must_use]
        #[inline(always)]
        pub const fn lowbyte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc LOWBYTE."]
        #[inline(always)]
        pub const fn set_lowbyte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Odrlowbyte {
        #[inline(always)]
        fn default() -> Odrlowbyte {
            Odrlowbyte(0)
        }
    }
    impl core::fmt::Debug for Odrlowbyte {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odrlowbyte")
                .field("lowbyte", &self.lowbyte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odrlowbyte {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Odrlowbyte {{ lowbyte: {=u8:?} }}", self.lowbyte())
        }
    }
    #[doc = "desc OPENDRAIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opendrain(pub u32);
    impl Opendrain {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Opendrain {
        #[inline(always)]
        fn default() -> Opendrain {
            Opendrain(0)
        }
    }
    impl core::fmt::Debug for Opendrain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opendrain")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opendrain {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opendrain {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc PDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdr(pub u32);
    impl Pdr {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Pdr {
        #[inline(always)]
        fn default() -> Pdr {
            Pdr(0)
        }
    }
    impl core::fmt::Debug for Pdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdr")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc PUR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pur(pub u32);
    impl Pur {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Pur {
        #[inline(always)]
        fn default() -> Pur {
            Pur(0)
        }
    }
    impl core::fmt::Debug for Pur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pur")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pur {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Riseie(pub u32);
    impl Riseie {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Riseie {
        #[inline(always)]
        fn default() -> Riseie {
            Riseie(0)
        }
    }
    impl core::fmt::Debug for Riseie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Riseie")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Riseie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Riseie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc SPEED."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Speed(pub u32);
    impl Speed {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Speed {
        #[inline(always)]
        fn default() -> Speed {
            Speed(0)
        }
    }
    impl core::fmt::Debug for Speed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Speed")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Speed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Speed {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
    #[doc = "desc TOG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tog(pub u32);
    impl Tog {
        #[doc = "desc PIN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pin0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN0."]
        #[inline(always)]
        pub const fn set_pin0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PIN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pin1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN1."]
        #[inline(always)]
        pub const fn set_pin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PIN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pin3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN3."]
        #[inline(always)]
        pub const fn set_pin3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PIN6."]
        #[must_use]
        #[inline(always)]
        pub const fn pin6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN6."]
        #[inline(always)]
        pub const fn set_pin6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PIN7."]
        #[must_use]
        #[inline(always)]
        pub const fn pin7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN7."]
        #[inline(always)]
        pub const fn set_pin7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Tog {
        #[inline(always)]
        fn default() -> Tog {
            Tog(0)
        }
    }
    impl core::fmt::Debug for Tog {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tog")
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin3", &self.pin3())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin3: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin3(),
                self.pin6(),
                self.pin7()
            )
        }
    }
}
