#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIOC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpioc {
    ptr: *mut u8,
}
unsafe impl Send for Gpioc {}
unsafe impl Sync for Gpioc {}
impl Gpioc {
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
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR0."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc AFR1."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR1."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc AFR2."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR2."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc AFR3."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR3."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc AFR4."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR4."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc AFR5."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc AFR5."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
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
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afrl {{ afr0: {=u8:?}, afr1: {=u8:?}, afr2: {=u8:?}, afr3: {=u8:?}, afr4: {=u8:?}, afr5: {=u8:?} }}",
                self.afr0(),
                self.afr1(),
                self.afr2(),
                self.afr3(),
                self.afr4(),
                self.afr5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Analog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Analog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc BRR2."]
        #[must_use]
        #[inline(always)]
        pub const fn brr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR2."]
        #[inline(always)]
        pub const fn set_brr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc BRR4."]
        #[must_use]
        #[inline(always)]
        pub const fn brr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR4."]
        #[inline(always)]
        pub const fn set_brr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BRR5."]
        #[must_use]
        #[inline(always)]
        pub const fn brr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR5."]
        #[inline(always)]
        pub const fn set_brr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("brr2", &self.brr2())
                .field("brr3", &self.brr3())
                .field("brr4", &self.brr4())
                .field("brr5", &self.brr5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Brr {{ brr0: {=bool:?}, brr1: {=bool:?}, brr2: {=bool:?}, brr3: {=bool:?}, brr4: {=bool:?}, brr5: {=bool:?} }}",
                self.brr0(),
                self.brr1(),
                self.brr2(),
                self.brr3(),
                self.brr4(),
                self.brr5()
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
        #[doc = "desc BSS2."]
        #[must_use]
        #[inline(always)]
        pub const fn bss2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS2."]
        #[inline(always)]
        pub const fn set_bss2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc BSS4."]
        #[must_use]
        #[inline(always)]
        pub const fn bss4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS4."]
        #[inline(always)]
        pub const fn set_bss4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BSS5."]
        #[must_use]
        #[inline(always)]
        pub const fn bss5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS5."]
        #[inline(always)]
        pub const fn set_bss5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc BRR0."]
        #[must_use]
        #[inline(always)]
        pub const fn brr0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR0."]
        #[inline(always)]
        pub const fn set_brr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BRR1."]
        #[must_use]
        #[inline(always)]
        pub const fn brr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR1."]
        #[inline(always)]
        pub const fn set_brr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc BRR2."]
        #[must_use]
        #[inline(always)]
        pub const fn brr2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR2."]
        #[inline(always)]
        pub const fn set_brr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc BRR3."]
        #[must_use]
        #[inline(always)]
        pub const fn brr3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR3."]
        #[inline(always)]
        pub const fn set_brr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc BRR4."]
        #[must_use]
        #[inline(always)]
        pub const fn brr4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR4."]
        #[inline(always)]
        pub const fn set_brr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc BRR5."]
        #[must_use]
        #[inline(always)]
        pub const fn brr5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR5."]
        #[inline(always)]
        pub const fn set_brr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("bss2", &self.bss2())
                .field("bss3", &self.bss3())
                .field("bss4", &self.bss4())
                .field("bss5", &self.bss5())
                .field("brr0", &self.brr0())
                .field("brr1", &self.brr1())
                .field("brr2", &self.brr2())
                .field("brr3", &self.brr3())
                .field("brr4", &self.brr4())
                .field("brr5", &self.brr5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsrr {{ bss0: {=bool:?}, bss1: {=bool:?}, bss2: {=bool:?}, bss3: {=bool:?}, bss4: {=bool:?}, bss5: {=bool:?}, brr0: {=bool:?}, brr1: {=bool:?}, brr2: {=bool:?}, brr3: {=bool:?}, brr4: {=bool:?}, brr5: {=bool:?} }}",
                self.bss0(),
                self.bss1(),
                self.bss2(),
                self.bss3(),
                self.bss4(),
                self.bss5(),
                self.brr0(),
                self.brr1(),
                self.brr2(),
                self.brr3(),
                self.brr4(),
                self.brr5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dir {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fallie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fallie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("fltclk", &self.fltclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filter {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, fltclk: {=u8:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Highie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Highie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lowie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lowie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Odr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
            )
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opendrain {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opendrain {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pur {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Riseie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Riseie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
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
        #[doc = "desc PIN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pin2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN2."]
        #[inline(always)]
        pub const fn set_pin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "desc PIN4."]
        #[must_use]
        #[inline(always)]
        pub const fn pin4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN4."]
        #[inline(always)]
        pub const fn set_pin4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PIN5."]
        #[must_use]
        #[inline(always)]
        pub const fn pin5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN5."]
        #[inline(always)]
        pub const fn set_pin5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5()
            )
        }
    }
}
