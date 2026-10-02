//! Stage 2: read and validate normalized JSON, then render PAC and metadata.
pub use crate::schema::{validate, Ir, Result};
use std::{fs, path::Path};

pub fn load_json(path: &Path) -> Result<Ir> {
    let doc: crate::schema::ChipDocument = serde_json::from_str(&fs::read_to_string(path)?)?;
    if doc.schema_version != crate::schema::SCHEMA_VERSION {
        return Err("unsupported chip JSON schema version".into());
    }
    let root = path
        .parent()
        .and_then(Path::parent)
        .ok_or("chip JSON must be in <root>/chips/")?;
    let mut blocks = std::collections::BTreeMap::new();
    for reference in doc.registers {
        crate::schema::check_id(&reference.kind)?;
        crate::schema::check_id(&reference.version)?;
        let file = root
            .join("registers")
            .join(format!("{}_{}.json", reference.kind, reference.version));
        let register: crate::schema::RegisterDocument =
            serde_json::from_str(&fs::read_to_string(file)?)?;
        if register.schema_version != crate::schema::SCHEMA_VERSION
            || register.block.name != reference.kind
            || register.block.version != reference.version
            || blocks.insert(reference.kind, register.block).is_some()
        {
            return Err("invalid or duplicate register JSON reference".into());
        }
    }
    let ir = Ir {
        schema_version: doc.schema_version,
        chip: doc.chip,
        family: doc.family,
        blocks,
    };
    validate(&ir)?;
    Ok(ir)
}

const RAW_API: &str = r#"
/// Raw MMIO register handle. Direction, side effects and bus access width are typed.
/// The trailing width type defaults to u32 for existing 32-bit consumers.
pub struct Reg<A,W=Ordinary,R=Ordinary,T=u32> { address: usize, _access: core::marker::PhantomData<(A,W,R,T)> }
impl<A,W,R,T> Copy for Reg<A,W,R,T> {}
impl<A,W,R,T> Clone for Reg<A,W,R,T> {fn clone(&self)->Self {*self}}
mod register_value_sealed {
    pub trait Sealed {}
    impl Sealed for u8 {} impl Sealed for u16 {} impl Sealed for u32 {}
}
/// Hardware access words. Sealed to the three audited unsigned bus widths.
pub trait RegisterValue: register_value_sealed::Sealed + Copy
    + core::ops::Not<Output=Self> + core::ops::BitAnd<Output=Self>
    + core::ops::BitOr<Output=Self> {}
impl RegisterValue for u8 {} impl RegisterValue for u16 {} impl RegisterValue for u32 {}
pub enum RW {} pub enum RO {} pub enum WO {}
/// Behavior markers prevent ordinary read/modify/write on side-effect registers.
pub enum Ordinary {} pub enum ZeroToClear {} pub enum OneToClear {}
pub enum OneToSet {} pub enum Toggle {} pub enum Command {} pub enum Keyed {}
pub enum Mixed {} pub enum WriteOnce {} pub enum ReadClear {} pub enum Fifo {} pub enum Latch {}
/// Pure register field descriptor. No hardware operation is performed.
pub struct Field<A,T=u32> { mask:T, shift:u8, _access:core::marker::PhantomData<A> }
impl<A,T:Copy> Field<A,T> {
    pub const fn new(mask:T,shift:u8)->Self {Self{mask,shift,_access:core::marker::PhantomData}}
    pub const fn mask(&self)->T {self.mask}
    pub const fn shift(&self)->u8 {self.shift}
}
/// A one-bit boolean field; unlike raw fields its value API is bool.
pub struct BoolField<A,T=u32> {raw:Field<A,T>}
impl<A,T:Copy> BoolField<A,T> {
 pub const fn new(mask:T,shift:u8)->Self {Self{raw:Field::new(mask,shift)}}
 pub const fn mask(&self)->T {self.raw.mask()}
 pub const fn shift(&self)->u8 {self.raw.shift()}
}
/// A discrete field with explicit known values. Reserved encodings read as None.
pub struct EnumField<A,T,U=u32> {raw:Field<A,U>,_value:core::marker::PhantomData<T>}
impl<A,T,U:Copy> EnumField<A,T,U> {
 pub const fn new(mask:U,shift:u8)->Self {Self{raw:Field::new(mask,shift),_value:core::marker::PhantomData}}
 pub const fn mask(&self)->U {self.raw.mask()}
 pub const fn shift(&self)->u8 {self.raw.shift()}
}
// Primitive specializations keep the field helpers const while matching the
// register word type. No field helper itself performs a hardware access.
macro_rules! field_access {
 ($word:ty) => {
  impl Field<RO,$word> {pub const fn read(&self,word:$word)->$word {(word&self.mask)>>self.shift}}
  impl Field<RW,$word> {
   pub const fn read(&self,word:$word)->$word {(word&self.mask)>>self.shift}
   pub const fn write(&self,word:$word,value:$word)->$word {(word&!self.mask)|((value<<self.shift)&self.mask)}
  }
  impl Field<WO,$word> {pub const fn write(&self,word:$word,value:$word)->$word {(word&!self.mask)|((value<<self.shift)&self.mask)}}
  impl BoolField<RO,$word> {pub const fn read(&self,word:$word)->bool {word&self.raw.mask!=0}}
  impl BoolField<RW,$word> {
   pub const fn read(&self,word:$word)->bool {word&self.raw.mask!=0}
   pub const fn write(&self,word:$word,value:bool)->$word {(word&!self.raw.mask)|if value {self.raw.mask}else{0}}
  }
  impl BoolField<WO,$word> {pub const fn write(&self,word:$word,value:bool)->$word {(word&!self.raw.mask)|if value {self.raw.mask}else{0}}}
  impl<T:TryFrom<u32>> EnumField<RO,T,$word> {pub fn read(&self,word:$word)->Option<T> {T::try_from(((word&self.raw.mask)>>self.raw.shift) as u32).ok()}}
  impl<T:TryFrom<u32>+Into<u32>> EnumField<RW,T,$word> {
   pub fn read(&self,word:$word)->Option<T> {T::try_from(((word&self.raw.mask)>>self.raw.shift) as u32).ok()}
   pub fn write(&self,word:$word,value:T)->$word {(word&!self.raw.mask)|(((value.into()<<self.raw.shift) as $word)&self.raw.mask)}
  }
  impl<T:Into<u32>> EnumField<WO,T,$word> {pub fn write(&self,word:$word,value:T)->$word {(word&!self.raw.mask)|(((value.into()<<self.raw.shift) as $word)&self.raw.mask)}}
 };
}
field_access!(u8); field_access!(u16); field_access!(u32);
impl<A,W,R,T:RegisterValue> Reg<A,W,R,T> {
    /// # Safety
    /// Address must be a valid aligned register with the specified access mode.
    pub const unsafe fn from_address(address: usize) -> Self { Self { address, _access: core::marker::PhantomData } }
    pub const fn address(&self) -> usize { self.address }
}
impl<W,R,T:RegisterValue> Reg<RO,W,R,T> {
    /// # Safety
    /// Clock/power and ownership must permit a read; account for side effects.
    pub unsafe fn read(&self) -> T { unsafe { read_value(self.address) } }
}
impl<W,R,T:RegisterValue> Reg<RW,W,R,T> {
    /// # Safety
    /// Clock/power and ownership must permit a read; account for side effects.
    pub unsafe fn read(&self) -> T { unsafe { read_value(self.address) } }
    /// # Safety
    /// Value must obey reserved bits, keys and register semantics; serialize access.
    pub unsafe fn write(&self, value: T) { unsafe { write_value(self.address, value) } }
}
impl<W,R,T:RegisterValue> Reg<WO,W,R,T> {
    /// # Safety
    /// Value must obey reserved bits, keys and register semantics; serialize access.
    pub unsafe fn write(&self, value: T) { unsafe { write_value(self.address, value) } }
}
impl<T:RegisterValue> Reg<RW,Ordinary,Ordinary,T> {
    /// # Safety
    /// Serialize access and obey reserved bits. Only ordinary RW exposes this method.
    pub unsafe fn modify(&self, clear:T, set:T) { unsafe { self.write((self.read()&!clear)|set) } }
}
macro_rules! side_effect_writes {
    ($access:ty) => {
        impl<R,T:RegisterValue> Reg<$access,ZeroToClear,R,T> {
            /// Write zero only to selected flags, preserving other flags with ones.
            /// # Safety
            /// Mask must select valid clearable flags; obey reserved bits and ownership.
            pub unsafe fn clear(&self,mask:T) {unsafe {write_value(self.address,!mask)}}
        }
        impl<R,T:RegisterValue> Reg<$access,OneToClear,R,T> {
            /// # Safety
            /// Mask must select valid clearable flags; obey reserved bits and ownership.
            pub unsafe fn clear(&self,mask:T) {unsafe {write_value(self.address,mask)}}
        }
        impl<R,T:RegisterValue> Reg<$access,OneToSet,R,T> {
            /// # Safety
            /// Mask must select valid settable bits; obey ownership and reserved bits.
            pub unsafe fn set(&self,mask:T) {unsafe {write_value(self.address,mask)}}
        }
        impl<R,T:RegisterValue> Reg<$access,Toggle,R,T> {
            /// # Safety
            /// Mask must select valid toggle bits; obey ownership and reserved bits.
            pub unsafe fn toggle(&self,mask:T) {unsafe {write_value(self.address,mask)}}
        }
        impl<R,T:RegisterValue> Reg<$access,Command,R,T> {
            /// # Safety
            /// Value must be a valid command and all hardware preconditions must hold.
            pub unsafe fn command(&self,value:T) {unsafe {write_value(self.address,value)}}
        }
    }
}
side_effect_writes!(RW);
side_effect_writes!(WO);
// Monomorphization preserves the actual MMIO transaction width. In particular,
// a narrow read is never implemented as a u32 read followed by truncation.
#[inline] unsafe fn read_value<T:RegisterValue>(address:usize)->T { unsafe { core::ptr::read_volatile(address as *const T) } }
#[inline] unsafe fn write_value<T:RegisterValue>(address:usize,value:T) { unsafe { core::ptr::write_volatile(address as *mut T,value) } }
/// # Safety
/// Address must identify a mapped aligned readable register. Respect clock,
/// power, exclusive ownership and register side effects.
#[inline] pub unsafe fn read(address: usize) -> u32 { unsafe { core::ptr::read_volatile(address as *const u32) } }
/// # Safety
/// Address and value must be valid for the register. Preserve reserved bits,
/// provide write keys and synchronize with interrupts/DMA and other owners.
#[inline] pub unsafe fn write(address: usize, value: u32) { unsafe { core::ptr::write_volatile(address as *mut u32,value) } }
/// # Safety
/// Read/write contracts apply. This is NOT atomic. Only use for ordinary RW
/// registers, never W1C/W0C, write-only or read-side-effect registers.
#[inline] pub unsafe fn modify(address: usize, clear: u32, set: u32) { unsafe { write(address,(read(address)&!clear)|set) } }
"#;
/// Render register declarations from the validated IR, never from a chip template.
pub fn render_pac(ir: &Ir) -> Result<String> {
    validate(ir)?;
    let mut out =
        String::from("// Generated by cw32-gen (pac stage). Edit YAML sources, not this file.\n");
    out.push_str(RAW_API);
    for c in &ir.family.constants {
        let ty = if c.name.ends_with("_ADDRESS") {
            "usize"
        } else {
            "u32"
        };
        out.push_str(&format!(
            "#[doc = {:?}]\npub const {}: {ty} = {:#x};\n",
            c.description, c.name, c.value
        ));
    }
    for p in &ir.family.peripherals {
        out.push_str(&format!(
            "pub const {}_BASE: usize = {:#x};\n",
            p.name, p.address
        ));
        out.push_str(&format!("pub const {}: {}::RegisterBlock = unsafe {{ {}::RegisterBlock::from_address({}_BASE) }};\n",p.name,p.block,p.block,p.name));
    }
    for b in ir.blocks.values() {
        out.push_str(&format!("pub mod {} {{\n#[derive(Clone, Copy)]\npub struct RegisterBlock {{ address: usize }}\nimpl RegisterBlock {{\n/// # Safety\n/// The address must identify this register block on the selected chip.\npub const unsafe fn from_address(address:usize)->Self {{ Self {{address}} }}\n",b.name));
        for r in &b.registers {
            let behavior = if r.bit_size == 32
                && r.write_behavior == crate::schema::WriteBehavior::Ordinary
                && r.read_behavior == crate::schema::ReadBehavior::Ordinary
            {
                String::new()
            } else {
                format!(
                    ",super::{:?},super::{}",
                    r.write_behavior,
                    read_marker(r.read_behavior)
                )
            };
            let width = if r.bit_size == 32 {
                String::new()
            } else {
                format!(",u{}", r.bit_size)
            };
            out.push_str(&format!("#[doc = {:?}]\npub const fn {}(&self)->super::Reg<super::{}{behavior}{width}> {{ unsafe {{ super::Reg::from_address(self.address + {}) }} }}\n",r.description,rust_ident(&r.name.to_ascii_lowercase()),r.access.to_ascii_uppercase(),r.name));
        }
        out.push_str("}\n");
        for r in &b.registers {
            out.push_str(&format!(
                "#[doc = {:?}]\npub const {}: usize = {:#x};\n",
                r.description, r.name, r.offset
            ));
        }
        for c in &b.constants {
            out.push_str(&format!(
                "#[doc = {:?}]\npub const {}: u32 = {:#x};\n",
                c.description, c.name, c.value
            ));
        }
        out.push_str("pub mod fields {\n");
        for r in &b.registers {
            if r.fields.is_empty() {
                continue;
            }
            out.push_str(&format!(
                "pub mod {} {{\n",
                rust_ident(&r.name.to_ascii_lowercase())
            ));
            for f in &r.fields {
                let mask = (u32::MAX >> (32 - f.bit_size)) << f.bit_offset;
                let access = f.access.to_ascii_uppercase();
                let width = if r.bit_size == 32 {
                    String::new()
                } else {
                    format!(",u{}", r.bit_size)
                };
                match f.kind.as_str() {
                    "bool" => out.push_str(&format!("#[doc = {:?}]\npub const {}: crate::BoolField<crate::{access}{width}> = crate::BoolField::new({mask:#x}, {});\n",f.description,f.name,f.bit_offset)),
                    "enum" => {
                        let value_name=format!("{}Value",f.name);
                        out.push_str(&format!("#[derive(Clone,Copy,Debug,PartialEq,Eq)]\n#[repr(u32)]\n#[allow(non_camel_case_types)]\npub enum {value_name} {{\n"));
                        for v in &f.values {out.push_str(&format!("{} = {},\n",v.name,v.value));}
                        out.push_str(&format!("}}\nimpl TryFrom<u32> for {value_name} {{type Error=();fn try_from(value:u32)->Result<Self,()> {{match value {{"));
                        for v in &f.values {out.push_str(&format!("{}=>Ok(Self::{}),",v.value,v.name));}
                        out.push_str("_=>Err(())}}}\n");
                        out.push_str(&format!("impl From<{value_name}> for u32 {{fn from(value:{value_name})->u32 {{value as u32}}}}\n"));
                        out.push_str(&format!("#[doc = {:?}]\npub const {}:crate::EnumField<crate::{access},{value_name}{width}>=crate::EnumField::new({mask:#x},{});\n",f.description,f.name,f.bit_offset));
                    }
                    _ => out.push_str(&format!("#[doc = {:?}]\npub const {}: crate::Field<crate::{access}{width}> = crate::Field::new({mask:#x}, {});\n",f.description,f.name,f.bit_offset))
                }
            }
            out.push_str("}\n");
        }
        out.push_str("}\n}\n");
    }
    out.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n#[repr(u16)]\n#[allow(non_camel_case_types)]\npub enum Interrupt {\n");
    for i in &ir.family.interrupts {
        out.push_str(&format!("{} = {},\n", i.name, i.number));
    }
    out.push_str("}\nimpl Interrupt { pub const fn number(self)->u16 { self as u16 } }\n");
    Ok(out)
}
/// Render cortex-m-rt's device vector table from validated IRQ numbers.
/// Missing indices are reserved zero words, never compacted out of the table.
pub fn render_runtime(ir: &Ir) -> Result<String> {
    validate(ir)?;
    let interrupts: std::collections::BTreeMap<_, _> = ir
        .family
        .interrupts
        .iter()
        .map(|irq| (irq.number, irq.name.as_str()))
        .collect();
    let length = interrupts
        .last_key_value()
        .map_or(0, |(&n, _)| usize::from(n) + 1);
    let mut out = String::from(
        "// Generated by cw32-gen. cortex-m-rt device ABI; do not edit.\n\
         #[repr(C)]\n\
         pub union Vector { handler: unsafe extern \"C\" fn(), reserved: usize }\n\
         unsafe extern \"C\" {\n",
    );
    for name in interrupts.values() {
        out.push_str(&format!("    fn {name}();\n"));
    }
    out.push_str(&format!(
        "}}\n#[used]\n#[unsafe(no_mangle)]\n\
         #[unsafe(link_section = \".vector_table.interrupts\")]\n\
         pub static __INTERRUPTS: [Vector; {length}] = [\n",
    ));
    for number in 0..length {
        if let Some(name) = interrupts.get(&(number as u16)) {
            out.push_str(&format!(
                "    Vector {{ handler: {name} }}, // IRQ {number}\n"
            ));
        } else {
            out.push_str(&format!(
                "    Vector {{ reserved: 0 }}, // IRQ {number}: reserved\n"
            ));
        }
    }
    out.push_str("];\n");
    Ok(out)
}

/// Default aliases for exactly the IRQ symbols referenced by the device table.
pub fn render_device_x(ir: &Ir) -> Result<String> {
    validate(ir)?;
    let mut interrupts: Vec<_> = ir.family.interrupts.iter().collect();
    interrupts.sort_by_key(|irq| irq.number);
    let mut out = String::from("/* Generated by cw32-gen. Included by cortex-m-rt link.x. */\n");
    for irq in interrupts {
        out.push_str(&format!("PROVIDE({} = DefaultHandler);\n", irq.name));
    }
    Ok(out)
}

fn read_marker(behavior: crate::schema::ReadBehavior) -> &'static str {
    use crate::schema::ReadBehavior;
    match behavior {
        ReadBehavior::Ordinary => "Ordinary",
        ReadBehavior::Clear => "ReadClear",
        ReadBehavior::Fifo => "Fifo",
        ReadBehavior::Latch => "Latch",
    }
}
fn rust_ident(name: &str) -> String {
    if [
        "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "abstract", "become", "box", "do", "final", "macro", "override",
        "priv", "try", "typeof", "unsized", "virtual", "yield", "gen",
    ]
    .contains(&name)
    {
        format!("r#{name}")
    } else {
        name.into()
    }
}
fn access_marker(access: &str) -> &'static str {
    match access {
        "ro" => "ReadOnly",
        "rw" => "ReadWrite",
        "wo" => "WriteOnly",
        _ => unreachable!("validated access"),
    }
}
fn render_register_bit(ir: &Ir, reference: Option<&crate::schema::RegisterBit>) -> String {
    let Some(reference) = reference else {
        return "None".into();
    };
    let target = ir
        .family
        .peripherals
        .iter()
        .find(|p| p.name == reference.peripheral)
        .unwrap();
    let register = ir.blocks[&target.block]
        .registers
        .iter()
        .find(|r| r.name == reference.register)
        .unwrap();
    let shared = ir
        .family
        .peripherals
        .iter()
        .filter(|p| {
            [&p.clock_gate, &p.reset].into_iter().flatten().any(|r| {
                r.peripheral == reference.peripheral
                    && r.register == reference.register
                    && r.bit == reference.bit
            })
        })
        .count()
        > 1;
    format!(
        "Some(RegisterBit {{peripheral:{:?},register:{:?},field:{:?},bit:{},offset:{},shared:{}}})",
        reference.peripheral,
        reference.register,
        reference.field,
        reference.bit,
        register.offset,
        shared
    )
}
const METADATA_TYPES: &str = r#"
#[derive(Clone, Copy, Debug)]
pub struct Metadata { pub schema_version:u32, pub name: &'static str, pub family: &'static str, pub core: &'static str, pub target: &'static str, pub peripherals: &'static [Peripheral], pub interrupts: &'static [Interrupt], pub pins: &'static [Pin], pub memory: &'static [Memory], pub interrupt_bindings:&'static [InterruptBinding], pub pin_routes:&'static [PinRoute], pub remaps:&'static [Remap], pub quirks:&'static [Quirk] }
#[derive(Clone, Copy, Debug)]
pub struct Peripheral { pub name: &'static str, pub block: &'static str, pub version: &'static str, pub address: usize, pub clock_bit: Option<u8>, pub clock_gate:Option<RegisterBit>, pub reset:Option<RegisterBit>, pub ownership_parent:Option<&'static str>, pub registers:&'static [Register], pub implemented_mask: u16, pub pulldown_mask: u16 }
#[derive(Clone, Copy, Debug)]
pub struct RegisterBit { pub peripheral:&'static str, pub register:&'static str, pub field:&'static str, pub bit:u8, pub offset:usize, pub shared:bool }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access { ReadOnly, ReadWrite, WriteOnly }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteBehavior { Ordinary, ZeroToClear, OneToClear, OneToSet, Toggle, Command, Keyed, Mixed, WriteOnce }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadBehavior { Ordinary, Clear, Fifo, Latch }
#[derive(Clone, Copy, Debug)]
pub struct Register {pub name:&'static str,pub offset:usize,pub bit_size:u8,pub access:Access,pub alias_of:Option<&'static str>,pub write_behavior:WriteBehavior,pub read_behavior:ReadBehavior,pub fields:&'static [RegisterField]}
#[derive(Clone, Copy, Debug)]
pub struct RegisterField {pub name:&'static str,pub bit_offset:u8,pub bit_size:u8,pub access:Access}
#[derive(Clone, Copy, Debug)]
pub struct Interrupt { pub name: &'static str, pub number: u16 }
#[derive(Clone, Copy, Debug)]
pub struct Pin { pub name: &'static str, pub port: &'static str, pub number: u8 }
#[derive(Clone, Copy, Debug)]
pub struct Memory { pub name: &'static str, pub address: usize, pub size: usize }
#[derive(Clone, Copy, Debug)]
pub struct InterruptBinding {pub peripheral:&'static str,pub signal:&'static str,pub interrupt:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct PinRoute {pub pin:&'static str,pub peripheral:&'static str,pub signal:&'static str,pub af:Option<u8>,pub remap:Option<&'static str>}
#[derive(Clone,Copy,Debug)]
pub struct Remap {pub name:&'static str,pub peripheral:&'static str,pub description:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct Quirk {pub peripheral:Option<&'static str>,pub name:&'static str,pub description:&'static str,pub source:&'static str}
"#;
pub fn render_metadata(ir: &Ir) -> Result<String> {
    validate(ir)?;
    let mut out = String::from("// Generated; no maintained Rust chip tables.\n");
    out.push_str(METADATA_TYPES);
    for block in ir.blocks.values() {
        out.push_str(&format!(
            "static REGISTERS_{}: &[Register] = &[\n",
            block.name.to_ascii_uppercase()
        ));
        for r in &block.registers {
            out.push_str(&format!("Register {{name:{:?},offset:{},bit_size:{},access:Access::{},alias_of:{:?},write_behavior:WriteBehavior::{:?},read_behavior:ReadBehavior::{:?},fields:&[",r.name,r.offset,r.bit_size,access_marker(&r.access),r.alias_of,r.write_behavior,r.read_behavior));
            for f in &r.fields {
                out.push_str(&format!(
                    "RegisterField {{name:{:?},bit_offset:{},bit_size:{},access:Access::{}}},",
                    f.name,
                    f.bit_offset,
                    f.bit_size,
                    access_marker(&f.access)
                ));
            }
            out.push_str("]},\n");
        }
        out.push_str("];\n");
    }
    out.push_str(&format!("pub static METADATA: Metadata = Metadata {{ schema_version:{}, name: {:?}, family: {:?}, core: {:?}, target: {:?},\nperipherals: &[\n",ir.schema_version,ir.chip.name,ir.family.name,ir.family.core,ir.family.target));
    for p in &ir.family.peripherals {
        out.push_str(&format!("Peripheral {{ name: {:?}, block: {:?}, version: {:?}, address: {:#x}, clock_bit: {:?}, clock_gate:{}, reset:{}, ownership_parent:{:?}, registers:REGISTERS_{}, implemented_mask: {}, pulldown_mask: {} }},\n",p.name,p.block,p.version,p.address,p.clock_bit,render_register_bit(ir,p.clock_gate.as_ref()),render_register_bit(ir,p.reset.as_ref()),p.ownership_parent,p.block.to_ascii_uppercase(),p.implemented_mask,p.pulldown_mask));
    }
    out.push_str("], interrupts: &[\n");
    for i in &ir.family.interrupts {
        out.push_str(&format!(
            "Interrupt {{ name: {:?}, number: {} }},\n",
            i.name, i.number
        ));
    }
    out.push_str("], pins: &[\n");
    for p in &ir.chip.pins {
        out.push_str(&format!(
            "Pin {{ name: {:?}, port: {:?}, number: {} }},\n",
            p.name,
            format!("GPIO{}", p.port),
            p.number
        ));
    }
    out.push_str("], memory: &[\n");
    for m in &ir.chip.memory {
        out.push_str(&format!(
            "Memory {{ name: {:?}, address: {}, size: {} }},\n",
            m.name, m.address, m.size
        ));
    }
    out.push_str("], interrupt_bindings: &[\n");
    for p in &ir.family.peripherals {
        for i in &p.interrupts {
            out.push_str(&format!(
                "InterruptBinding {{peripheral:{:?},signal:{:?},interrupt:{:?}}},\n",
                p.name, i.signal, i.interrupt
            ));
        }
    }
    out.push_str("], pin_routes: &[\n");
    for r in &ir.chip.pin_routes {
        out.push_str(&format!(
            "PinRoute {{pin:{:?},peripheral:{:?},signal:{:?},af:{:?},remap:{:?}}},\n",
            r.pin, r.peripheral, r.signal, r.af, r.remap
        ));
    }
    out.push_str("], remaps: &[\n");
    for r in &ir.chip.remaps {
        out.push_str(&format!(
            "Remap {{name:{:?},peripheral:{:?},description:{:?}}},\n",
            r.name, r.peripheral, r.description
        ));
    }
    out.push_str("], quirks: &[\n");
    for q in &ir.chip.quirks {
        out.push_str(&format!(
            "Quirk {{peripheral:None,name:{:?},description:{:?},source:{:?}}},\n",
            q.name, q.description, q.source
        ));
    }
    for p in &ir.family.peripherals {
        for q in &p.quirks {
            out.push_str(&format!(
                "Quirk {{peripheral:Some({:?}),name:{:?},description:{:?},source:{:?}}},\n",
                p.name, q.name, q.description, q.source
            ));
        }
    }
    out.push_str("] };\n");
    Ok(out)
}
/// Stable output: no time, source absolute paths, hash-map ordering or ambient state.
pub fn generate(ir: &Ir, out: &Path) -> Result<()> {
    fs::create_dir_all(out)?;
    fs::write(out.join("pac.rs"), render_pac(ir)?)?;
    fs::write(out.join("metadata.rs"), render_metadata(ir)?)?;
    fs::write(out.join("rt.rs"), render_runtime(ir)?)?;
    fs::write(out.join("device.x"), render_device_x(ir)?)?;
    Ok(())
}

/// Consume the explicit artifact produced by the data stage.
pub fn generate_from_json(json_path: &Path, out: &Path) -> Result<()> {
    generate(&load_json(json_path)?, out)
}
