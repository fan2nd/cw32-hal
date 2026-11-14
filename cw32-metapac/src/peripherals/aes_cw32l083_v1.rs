#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CW32L083 AES, own User Manual CN Rev2.0; see docs/l083-aes-trng-evidence.md."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aes {
    ptr: *mut u8,
}
unsafe impl Send for Aes {}
unsafe impl Sync for Aes {}
impl Aes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DATA0."]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<regs::Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc DATA1."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<regs::Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc DATA2."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<regs::Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc DATA3."]
    #[inline(always)]
    pub const fn data3(self) -> crate::common::Reg<regs::Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc KEY0."]
    #[inline(always)]
    pub const fn key0(self) -> crate::common::Reg<regs::Key0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc KEY1."]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<regs::Key1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc KEY2."]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<regs::Key2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc KEY3."]
    #[inline(always)]
    pub const fn key3(self) -> crate::common::Reg<regs::Key3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc KEY4."]
    #[inline(always)]
    pub const fn key4(self) -> crate::common::Reg<regs::Key4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc KEY5."]
    #[inline(always)]
    pub const fn key5(self) -> crate::common::Reg<regs::Key5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc KEY6."]
    #[inline(always)]
    pub const fn key6(self) -> crate::common::Reg<regs::Key6, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc KEY7."]
    #[inline(always)]
    pub const fn key7(self) -> crate::common::Reg<regs::Key7, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::Operation {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Operation::from_bits(val as u8)
        }
        #[doc = "Write 1 to start; read 0 after completion, read 1 while active. W0 abort is not documented."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: super::vals::Operation) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "desc KEYSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn keysize(&self) -> super::vals::KeySize {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::KeySize::from_bits(val as u8)
        }
        #[doc = "desc KEYSIZE."]
        #[inline(always)]
        pub const fn set_keysize(&mut self, val: super::vals::KeySize) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
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
                .field("start", &self.start())
                .field("mode", &self.mode())
                .field("keysize", &self.keysize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ start: {:?}, mode: {:?}, keysize: {:?} }}",
                self.start(),
                self.mode(),
                self.keysize()
            )
        }
    }
    #[doc = "desc DATA0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data0(pub u32);
    impl Data0 {
        #[doc = "DATA bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data0 {
        #[inline(always)]
        fn default() -> Data0 {
            Data0(0)
        }
    }
    impl core::fmt::Debug for Data0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data0")
                .field("data0", &self.data0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data0 {{ data0: {=u32:?} }}", self.data0())
        }
    }
    #[doc = "desc DATA1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data1(pub u32);
    impl Data1 {
        #[doc = "DATA bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data1 {
        #[inline(always)]
        fn default() -> Data1 {
            Data1(0)
        }
    }
    impl core::fmt::Debug for Data1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data1")
                .field("data1", &self.data1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data1 {{ data1: {=u32:?} }}", self.data1())
        }
    }
    #[doc = "desc DATA2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data2(pub u32);
    impl Data2 {
        #[doc = "DATA bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_data2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data2 {
        #[inline(always)]
        fn default() -> Data2 {
            Data2(0)
        }
    }
    impl core::fmt::Debug for Data2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data2")
                .field("data2", &self.data2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data2 {{ data2: {=u32:?} }}", self.data2())
        }
    }
    #[doc = "desc DATA3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data3(pub u32);
    impl Data3 {
        #[doc = "DATA bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn data3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_data3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data3 {
        #[inline(always)]
        fn default() -> Data3 {
            Data3(0)
        }
    }
    impl core::fmt::Debug for Data3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data3")
                .field("data3", &self.data3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data3 {{ data3: {=u32:?} }}", self.data3())
        }
    }
    #[doc = "desc KEY0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key0(pub u32);
    impl Key0 {
        #[doc = "KEY bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits31:0; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key0 {
        #[inline(always)]
        fn default() -> Key0 {
            Key0(0)
        }
    }
    impl core::fmt::Debug for Key0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key0").field("key0", &self.key0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key0 {{ key0: {=u32:?} }}", self.key0())
        }
    }
    #[doc = "desc KEY1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key1(pub u32);
    impl Key1 {
        #[doc = "KEY bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits63:32; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key1 {
        #[inline(always)]
        fn default() -> Key1 {
            Key1(0)
        }
    }
    impl core::fmt::Debug for Key1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key1").field("key1", &self.key1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key1 {{ key1: {=u32:?} }}", self.key1())
        }
    }
    #[doc = "desc KEY2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key2(pub u32);
    impl Key2 {
        #[doc = "KEY bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits95:64; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key2 {
        #[inline(always)]
        fn default() -> Key2 {
            Key2(0)
        }
    }
    impl core::fmt::Debug for Key2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key2").field("key2", &self.key2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key2 {{ key2: {=u32:?} }}", self.key2())
        }
    }
    #[doc = "desc KEY3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key3(pub u32);
    impl Key3 {
        #[doc = "KEY bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits127:96; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key3 {
        #[inline(always)]
        fn default() -> Key3 {
            Key3(0)
        }
    }
    impl core::fmt::Debug for Key3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key3").field("key3", &self.key3()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key3 {{ key3: {=u32:?} }}", self.key3())
        }
    }
    #[doc = "desc KEY4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key4(pub u32);
    impl Key4 {
        #[doc = "KEY bits159:128; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits159:128; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key4 {
        #[inline(always)]
        fn default() -> Key4 {
            Key4(0)
        }
    }
    impl core::fmt::Debug for Key4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key4").field("key4", &self.key4()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key4 {{ key4: {=u32:?} }}", self.key4())
        }
    }
    #[doc = "desc KEY5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key5(pub u32);
    impl Key5 {
        #[doc = "KEY bits191:160; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key5(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits191:160; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key5(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key5 {
        #[inline(always)]
        fn default() -> Key5 {
            Key5(0)
        }
    }
    impl core::fmt::Debug for Key5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key5").field("key5", &self.key5()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key5 {{ key5: {=u32:?} }}", self.key5())
        }
    }
    #[doc = "desc KEY6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key6(pub u32);
    impl Key6 {
        #[doc = "KEY bits223:192; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key6(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits223:192; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key6(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key6 {
        #[inline(always)]
        fn default() -> Key6 {
            Key6(0)
        }
    }
    impl core::fmt::Debug for Key6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key6").field("key6", &self.key6()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key6 {{ key6: {=u32:?} }}", self.key6())
        }
    }
    #[doc = "desc KEY7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key7(pub u32);
    impl Key7 {
        #[doc = "KEY bits255:224; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn key7(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "KEY bits255:224; index zero is the least-significant register word. No standard byte-array mapping is asserted."]
        #[inline(always)]
        pub const fn set_key7(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key7 {
        #[inline(always)]
        fn default() -> Key7 {
            Key7(0)
        }
    }
    impl core::fmt::Debug for Key7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key7").field("key7", &self.key7()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key7 {{ key7: {=u32:?} }}", self.key7())
        }
    }
}
pub mod vals {
    #[doc = "AES key length, UM section28.5.1. Encoding3 aliases128; HAL uses canonical0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum KeySize {
        Bits128 = 0x0,
        Bits192 = 0x01,
        Bits256 = 0x02,
        Bits128Alternate = 0x03,
    }
    impl KeySize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> KeySize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for KeySize {
        #[inline(always)]
        fn from(val: u8) -> KeySize {
            KeySize::from_bits(val)
        }
    }
    impl From<KeySize> for u8 {
        #[inline(always)]
        fn from(val: KeySize) -> u8 {
            KeySize::to_bits(val)
        }
    }
    #[doc = "AES direction, UM section28.5.1."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        Encrypt = 0x0,
        Decrypt = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[doc = "START command/status, CW32L083 UM Rev2.0 section 28.5.1. Writing zero is not an abort."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Operation {
        #[doc = "Read: operation complete. Write-zero behavior is unspecified."]
        Idle = 0x0,
        #[doc = "Read: operation busy. Write: start an operation."]
        Active = 0x01,
    }
    impl Operation {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Operation {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Operation {
        #[inline(always)]
        fn from(val: u8) -> Operation {
            Operation::from_bits(val)
        }
    }
    impl From<Operation> for u8 {
        #[inline(always)]
        fn from(val: Operation) -> u8 {
            Operation::to_bits(val)
        }
    }
}
