#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc IRMOD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irmod {
    ptr: *mut u8,
}
unsafe impl Send for Irmod {}
unsafe impl Sync for Irmod {}
impl Irmod {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input selection register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Input selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc MOD."]
        #[must_use]
        #[inline(always)]
        pub const fn mod_(&self) -> super::vals::IrMode {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::IrMode::from_bits(val as u8)
        }
        #[doc = "desc MOD."]
        #[inline(always)]
        pub const fn set_mod_(&mut self, val: super::vals::IrMode) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc IRSW."]
        #[must_use]
        #[inline(always)]
        pub const fn irsw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRSW."]
        #[inline(always)]
        pub const fn set_irsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc INV."]
        #[must_use]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc INV."]
        #[inline(always)]
        pub const fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("mod_", &self.mod_())
                .field("irsw", &self.irsw())
                .field("inv", &self.inv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ mod_: {:?}, irsw: {=bool:?}, inv: {=bool:?} }}",
                self.mod_(),
                self.irsw(),
                self.inv()
            )
        }
    }
}
pub mod vals {
    #[doc = "Exact own-family infrared boolean combinations; sources are configured separately."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IrMode {
        #[doc = "GTIM1_CH1 & GTIM2_CH1 & IRSW"]
        Gtim1Ch1AndGtim2Ch1AndIrsw = 0x0,
        #[doc = "GTIM1_CH1 & GTIM2_CH2 & IRSW"]
        Gtim1Ch1AndGtim2Ch2AndIrsw = 0x01,
        #[doc = "GTIM1_CH2 & GTIM2_CH1 & IRSW"]
        Gtim1Ch2AndGtim2Ch1AndIrsw = 0x02,
        #[doc = "GTIM1_CH2 & GTIM2_CH2 & IRSW"]
        Gtim1Ch2AndGtim2Ch2AndIrsw = 0x03,
        #[doc = "GTIM1_CH1 | GTIM2_CH1 | IRSW"]
        Gtim1Ch1OrGtim2Ch1OrIrsw = 0x04,
        #[doc = "GTIM1_CH1 | GTIM2_CH2 | IRSW"]
        Gtim1Ch1OrGtim2Ch2OrIrsw = 0x05,
        #[doc = "GTIM1_CH2 | GTIM2_CH1 | IRSW"]
        Gtim1Ch2OrGtim2Ch1OrIrsw = 0x06,
        #[doc = "GTIM1_CH2 | GTIM2_CH2 | IRSW"]
        Gtim1Ch2OrGtim2Ch2OrIrsw = 0x07,
        #[doc = "UART1_TXD & GTIM1_CH1 & IRSW"]
        Uart1TxdAndGtim1Ch1AndIrsw = 0x08,
        #[doc = "UART1_TXD | GTIM1_CH1 | IRSW"]
        Uart1TxdOrGtim1Ch1OrIrsw = 0x09,
        #[doc = "UART1_TXD & GTIM1_CH2 & IRSW"]
        Uart1TxdAndGtim1Ch2AndIrsw = 0x0a,
        #[doc = "UART1_TXD | GTIM1_CH2 | IRSW"]
        Uart1TxdOrGtim1Ch2OrIrsw = 0x0b,
        #[doc = "UART2_TXD & GTIM2_CH1 & IRSW"]
        Uart2TxdAndGtim2Ch1AndIrsw = 0x0c,
        #[doc = "UART2_TXD | GTIM2_CH1 | IRSW"]
        Uart2TxdOrGtim2Ch1OrIrsw = 0x0d,
        #[doc = "UART2_TXD & GTIM2_CH2 & IRSW"]
        Uart2TxdAndGtim2Ch2AndIrsw = 0x0e,
        #[doc = "UART2_TXD | GTIM2_CH2 | IRSW"]
        Uart2TxdOrGtim2Ch2OrIrsw = 0x0f,
    }
    impl IrMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMode {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMode {
        #[inline(always)]
        fn from(val: u8) -> IrMode {
            IrMode::from_bits(val)
        }
    }
    impl From<IrMode> for u8 {
        #[inline(always)]
        fn from(val: IrMode) -> u8 {
            IrMode::to_bits(val)
        }
    }
}
