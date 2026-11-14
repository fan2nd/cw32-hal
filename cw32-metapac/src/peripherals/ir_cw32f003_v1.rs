#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc IR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir {
    ptr: *mut u8,
}
unsafe impl Send for Ir {}
unsafe impl Sync for Ir {}
impl Ir {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc IRMOD."]
    #[inline(always)]
    pub const fn irmod(self) -> crate::common::Reg<regs::Irmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc IRMOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irmod(pub u32);
    impl Irmod {
        #[doc = "desc MOD."]
        #[must_use]
        #[inline(always)]
        pub const fn mod_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MOD."]
        #[inline(always)]
        pub const fn set_mod_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Irmod {
        #[inline(always)]
        fn default() -> Irmod {
            Irmod(0)
        }
    }
    impl core::fmt::Debug for Irmod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irmod").field("mod_", &self.mod_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irmod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Irmod {{ mod_: {=u8:?} }}", self.mod_())
        }
    }
}
