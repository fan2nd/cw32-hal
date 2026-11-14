#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIOA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
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
    #[doc = "desc AFRH."]
    #[inline(always)]
    pub const fn afrh(self) -> crate::common::Reg<regs::Afrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
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
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::RW> {
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
    #[doc = "desc AFRH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afrh(pub u32);
    impl Afrh {
        #[doc = "desc AFR8."]
        #[must_use]
        #[inline(always)]
        pub const fn afr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR8."]
        #[inline(always)]
        pub const fn set_afr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc AFR9."]
        #[must_use]
        #[inline(always)]
        pub const fn afr9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR9."]
        #[inline(always)]
        pub const fn set_afr9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc AFR10."]
        #[must_use]
        #[inline(always)]
        pub const fn afr10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR10."]
        #[inline(always)]
        pub const fn set_afr10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc AFR11."]
        #[must_use]
        #[inline(always)]
        pub const fn afr11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR11."]
        #[inline(always)]
        pub const fn set_afr11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "desc AFR12."]
        #[must_use]
        #[inline(always)]
        pub const fn afr12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR12."]
        #[inline(always)]
        pub const fn set_afr12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc AFR13."]
        #[must_use]
        #[inline(always)]
        pub const fn afr13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR13."]
        #[inline(always)]
        pub const fn set_afr13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc AFR14."]
        #[must_use]
        #[inline(always)]
        pub const fn afr14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR14."]
        #[inline(always)]
        pub const fn set_afr14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc AFR15."]
        #[must_use]
        #[inline(always)]
        pub const fn afr15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR15."]
        #[inline(always)]
        pub const fn set_afr15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Afrh {
        #[inline(always)]
        fn default() -> Afrh {
            Afrh(0)
        }
    }
    impl core::fmt::Debug for Afrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afrh")
                .field("afr8", &self.afr8())
                .field("afr9", &self.afr9())
                .field("afr10", &self.afr10())
                .field("afr11", &self.afr11())
                .field("afr12", &self.afr12())
                .field("afr13", &self.afr13())
                .field("afr14", &self.afr14())
                .field("afr15", &self.afr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afrh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afrh {{ afr8: {=u8:?}, afr9: {=u8:?}, afr10: {=u8:?}, afr11: {=u8:?}, afr12: {=u8:?}, afr13: {=u8:?}, afr14: {=u8:?}, afr15: {=u8:?} }}",
                self.afr8(),
                self.afr9(),
                self.afr10(),
                self.afr11(),
                self.afr12(),
                self.afr13(),
                self.afr14(),
                self.afr15()
            )
        }
    }
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
        #[doc = "desc AFR2."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR2."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
        #[doc = "desc AFR4."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR4."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc AFR5."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFR5."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
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
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
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
                "Afrl {{ afr0: {=u8:?}, afr1: {=u8:?}, afr2: {=u8:?}, afr3: {=u8:?}, afr4: {=u8:?}, afr5: {=u8:?}, afr6: {=u8:?}, afr7: {=u8:?} }}",
                self.afr0(),
                self.afr1(),
                self.afr2(),
                self.afr3(),
                self.afr4(),
                self.afr5(),
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Analog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Analog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc BRR8."]
        #[must_use]
        #[inline(always)]
        pub const fn brr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR8."]
        #[inline(always)]
        pub const fn set_brr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc BRR9."]
        #[must_use]
        #[inline(always)]
        pub const fn brr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR9."]
        #[inline(always)]
        pub const fn set_brr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BRR10."]
        #[must_use]
        #[inline(always)]
        pub const fn brr10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR10."]
        #[inline(always)]
        pub const fn set_brr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BRR11."]
        #[must_use]
        #[inline(always)]
        pub const fn brr11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR11."]
        #[inline(always)]
        pub const fn set_brr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BRR12."]
        #[must_use]
        #[inline(always)]
        pub const fn brr12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR12."]
        #[inline(always)]
        pub const fn set_brr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BRR13."]
        #[must_use]
        #[inline(always)]
        pub const fn brr13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR13."]
        #[inline(always)]
        pub const fn set_brr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BRR14."]
        #[must_use]
        #[inline(always)]
        pub const fn brr14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR14."]
        #[inline(always)]
        pub const fn set_brr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc BRR15."]
        #[must_use]
        #[inline(always)]
        pub const fn brr15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR15."]
        #[inline(always)]
        pub const fn set_brr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("brr6", &self.brr6())
                .field("brr7", &self.brr7())
                .field("brr8", &self.brr8())
                .field("brr9", &self.brr9())
                .field("brr10", &self.brr10())
                .field("brr11", &self.brr11())
                .field("brr12", &self.brr12())
                .field("brr13", &self.brr13())
                .field("brr14", &self.brr14())
                .field("brr15", &self.brr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Brr {{ brr0: {=bool:?}, brr1: {=bool:?}, brr2: {=bool:?}, brr3: {=bool:?}, brr4: {=bool:?}, brr5: {=bool:?}, brr6: {=bool:?}, brr7: {=bool:?}, brr8: {=bool:?}, brr9: {=bool:?}, brr10: {=bool:?}, brr11: {=bool:?}, brr12: {=bool:?}, brr13: {=bool:?}, brr14: {=bool:?}, brr15: {=bool:?} }}",
                self.brr0(),
                self.brr1(),
                self.brr2(),
                self.brr3(),
                self.brr4(),
                self.brr5(),
                self.brr6(),
                self.brr7(),
                self.brr8(),
                self.brr9(),
                self.brr10(),
                self.brr11(),
                self.brr12(),
                self.brr13(),
                self.brr14(),
                self.brr15()
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
        #[doc = "desc BSS8."]
        #[must_use]
        #[inline(always)]
        pub const fn bss8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS8."]
        #[inline(always)]
        pub const fn set_bss8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc BSS9."]
        #[must_use]
        #[inline(always)]
        pub const fn bss9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS9."]
        #[inline(always)]
        pub const fn set_bss9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BSS10."]
        #[must_use]
        #[inline(always)]
        pub const fn bss10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS10."]
        #[inline(always)]
        pub const fn set_bss10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc BSS11."]
        #[must_use]
        #[inline(always)]
        pub const fn bss11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS11."]
        #[inline(always)]
        pub const fn set_bss11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc BSS12."]
        #[must_use]
        #[inline(always)]
        pub const fn bss12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS12."]
        #[inline(always)]
        pub const fn set_bss12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc BSS13."]
        #[must_use]
        #[inline(always)]
        pub const fn bss13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS13."]
        #[inline(always)]
        pub const fn set_bss13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc BSS14."]
        #[must_use]
        #[inline(always)]
        pub const fn bss14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS14."]
        #[inline(always)]
        pub const fn set_bss14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc BSS15."]
        #[must_use]
        #[inline(always)]
        pub const fn bss15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSS15."]
        #[inline(always)]
        pub const fn set_bss15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "desc BRR6."]
        #[must_use]
        #[inline(always)]
        pub const fn brr6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR6."]
        #[inline(always)]
        pub const fn set_brr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc BRR7."]
        #[must_use]
        #[inline(always)]
        pub const fn brr7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR7."]
        #[inline(always)]
        pub const fn set_brr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc BRR8."]
        #[must_use]
        #[inline(always)]
        pub const fn brr8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR8."]
        #[inline(always)]
        pub const fn set_brr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc BRR9."]
        #[must_use]
        #[inline(always)]
        pub const fn brr9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR9."]
        #[inline(always)]
        pub const fn set_brr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc BRR10."]
        #[must_use]
        #[inline(always)]
        pub const fn brr10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR10."]
        #[inline(always)]
        pub const fn set_brr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc BRR11."]
        #[must_use]
        #[inline(always)]
        pub const fn brr11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR11."]
        #[inline(always)]
        pub const fn set_brr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc BRR12."]
        #[must_use]
        #[inline(always)]
        pub const fn brr12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR12."]
        #[inline(always)]
        pub const fn set_brr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc BRR13."]
        #[must_use]
        #[inline(always)]
        pub const fn brr13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR13."]
        #[inline(always)]
        pub const fn set_brr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc BRR14."]
        #[must_use]
        #[inline(always)]
        pub const fn brr14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR14."]
        #[inline(always)]
        pub const fn set_brr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc BRR15."]
        #[must_use]
        #[inline(always)]
        pub const fn brr15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRR15."]
        #[inline(always)]
        pub const fn set_brr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("bss6", &self.bss6())
                .field("bss7", &self.bss7())
                .field("bss8", &self.bss8())
                .field("bss9", &self.bss9())
                .field("bss10", &self.bss10())
                .field("bss11", &self.bss11())
                .field("bss12", &self.bss12())
                .field("bss13", &self.bss13())
                .field("bss14", &self.bss14())
                .field("bss15", &self.bss15())
                .field("brr0", &self.brr0())
                .field("brr1", &self.brr1())
                .field("brr2", &self.brr2())
                .field("brr3", &self.brr3())
                .field("brr4", &self.brr4())
                .field("brr5", &self.brr5())
                .field("brr6", &self.brr6())
                .field("brr7", &self.brr7())
                .field("brr8", &self.brr8())
                .field("brr9", &self.brr9())
                .field("brr10", &self.brr10())
                .field("brr11", &self.brr11())
                .field("brr12", &self.brr12())
                .field("brr13", &self.brr13())
                .field("brr14", &self.brr14())
                .field("brr15", &self.brr15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsrr {{ bss0: {=bool:?}, bss1: {=bool:?}, bss2: {=bool:?}, bss3: {=bool:?}, bss4: {=bool:?}, bss5: {=bool:?}, bss6: {=bool:?}, bss7: {=bool:?}, bss8: {=bool:?}, bss9: {=bool:?}, bss10: {=bool:?}, bss11: {=bool:?}, bss12: {=bool:?}, bss13: {=bool:?}, bss14: {=bool:?}, bss15: {=bool:?}, brr0: {=bool:?}, brr1: {=bool:?}, brr2: {=bool:?}, brr3: {=bool:?}, brr4: {=bool:?}, brr5: {=bool:?}, brr6: {=bool:?}, brr7: {=bool:?}, brr8: {=bool:?}, brr9: {=bool:?}, brr10: {=bool:?}, brr11: {=bool:?}, brr12: {=bool:?}, brr13: {=bool:?}, brr14: {=bool:?}, brr15: {=bool:?} }}",
                self.bss0(),
                self.bss1(),
                self.bss2(),
                self.bss3(),
                self.bss4(),
                self.bss5(),
                self.bss6(),
                self.bss7(),
                self.bss8(),
                self.bss9(),
                self.bss10(),
                self.bss11(),
                self.bss12(),
                self.bss13(),
                self.bss14(),
                self.bss15(),
                self.brr0(),
                self.brr1(),
                self.brr2(),
                self.brr3(),
                self.brr4(),
                self.brr5(),
                self.brr6(),
                self.brr7(),
                self.brr8(),
                self.brr9(),
                self.brr10(),
                self.brr11(),
                self.brr12(),
                self.brr13(),
                self.brr14(),
                self.brr15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dir {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fallie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fallie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("fltclk", &self.fltclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filter {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?}, fltclk: {=u8:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15(),
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Highie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Highie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
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
                .field("pin0", &self.pin0())
                .field("pin1", &self.pin1())
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin2", &self.pin2())
                .field("pin3", &self.pin3())
                .field("pin4", &self.pin4())
                .field("pin5", &self.pin5())
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lock {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?}, key: {=u16:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15(),
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lowie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lowie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Odr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opendrain {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opendrain {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pur {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Riseie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Riseie {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
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
        #[doc = "desc PIN8."]
        #[must_use]
        #[inline(always)]
        pub const fn pin8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN8."]
        #[inline(always)]
        pub const fn set_pin8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PIN9."]
        #[must_use]
        #[inline(always)]
        pub const fn pin9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN9."]
        #[inline(always)]
        pub const fn set_pin9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PIN10."]
        #[must_use]
        #[inline(always)]
        pub const fn pin10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN10."]
        #[inline(always)]
        pub const fn set_pin10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PIN11."]
        #[must_use]
        #[inline(always)]
        pub const fn pin11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN11."]
        #[inline(always)]
        pub const fn set_pin11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PIN12."]
        #[must_use]
        #[inline(always)]
        pub const fn pin12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN12."]
        #[inline(always)]
        pub const fn set_pin12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PIN13."]
        #[must_use]
        #[inline(always)]
        pub const fn pin13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN13."]
        #[inline(always)]
        pub const fn set_pin13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PIN14."]
        #[must_use]
        #[inline(always)]
        pub const fn pin14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN14."]
        #[inline(always)]
        pub const fn set_pin14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PIN15."]
        #[must_use]
        #[inline(always)]
        pub const fn pin15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN15."]
        #[inline(always)]
        pub const fn set_pin15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("pin6", &self.pin6())
                .field("pin7", &self.pin7())
                .field("pin8", &self.pin8())
                .field("pin9", &self.pin9())
                .field("pin10", &self.pin10())
                .field("pin11", &self.pin11())
                .field("pin12", &self.pin12())
                .field("pin13", &self.pin13())
                .field("pin14", &self.pin14())
                .field("pin15", &self.pin15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tog {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin4: {=bool:?}, pin5: {=bool:?}, pin6: {=bool:?}, pin7: {=bool:?}, pin8: {=bool:?}, pin9: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?} }}",
                self.pin0(),
                self.pin1(),
                self.pin2(),
                self.pin3(),
                self.pin4(),
                self.pin5(),
                self.pin6(),
                self.pin7(),
                self.pin8(),
                self.pin9(),
                self.pin10(),
                self.pin11(),
                self.pin12(),
                self.pin13(),
                self.pin14(),
                self.pin15()
            )
        }
    }
}
