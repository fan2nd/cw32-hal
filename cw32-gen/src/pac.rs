//! Stage 2: read and validate normalized JSON, then render PAC and metadata.
pub use crate::schema::{validate, Ir, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

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

// Adapted from chiptool bcf538a2e7b8584ae874ee9ab72efb1576fc6152 common.rs.
// MIT OR Apache-2.0; original notice in vendor/licenses/chiptool-MIT.txt.
// CW32 extends the surface with audited reset policies and side-effect typing.
const COMMON_API: &str = r#"
/// Low-level register access, following chiptool/stm32-metapac.
/// Direct PAC access requires the caller to coordinate hardware ownership.
/// Write/read behavior markers keep unsupported RMW operations unavailable.
pub enum Ordinary {} pub enum ZeroToClear {} pub enum OneToClear {}
pub enum OneToSet {} pub enum Toggle {} pub enum Command {} pub enum Keyed {}
pub enum Mixed {} pub enum WriteOnce {} pub enum ReadClear {} pub enum Fifo {} pub enum Latch {}
use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

/// Default policy is zero-sized for ordinary registers.
#[derive(Copy,Clone)]
pub struct DefaultReset;
/// An indexed register whose documented complete reset differs per element.
/// Constructed only from explicit normalized element reset metadata.
#[derive(Copy,Clone)]
pub struct GivenReset<T:Copy>(T);
mod reset_sealed {pub trait Sealed {} impl Sealed for super::DefaultReset {} impl<T:Copy> Sealed for super::GivenReset<T> {}}
#[doc(hidden)]
pub trait Reset<T:Copy>:reset_sealed::Sealed {fn value(&self)->T;}
impl<T:Copy+Default> Reset<T> for DefaultReset {fn value(&self)->T {T::default()}}
impl<T:Copy> Reset<T> for GivenReset<T> {fn value(&self)->T {self.0}}

pub struct Reg<T: Copy, A: Access, WB=Ordinary, RB=Ordinary, D=DefaultReset> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A, WB, RB)>,
    reset: D,
}
impl<T:Copy,A:Access,WB,RB,D:Copy> Copy for Reg<T,A,WB,RB,D> {}
impl<T:Copy,A:Access,WB,RB,D:Copy> Clone for Reg<T,A,WB,RB,D> {fn clone(&self)->Self {*self}}
impl<T:Copy,A:Access,WB,RB,D> PartialEq for Reg<T,A,WB,RB,D> {fn eq(&self,other:&Self)->bool {self.ptr==other.ptr}}
impl<T:Copy,A:Access,WB,RB,D> Eq for Reg<T,A,WB,RB,D> {}
unsafe impl<T:Copy,A:Access,WB,RB,D:Send> Send for Reg<T,A,WB,RB,D> {}
unsafe impl<T:Copy,A:Access,WB,RB,D:Sync> Sync for Reg<T,A,WB,RB,D> {}

impl<T:Copy,A:Access,WB,RB> Reg<T,A,WB,RB> {
    /// # Safety
    /// Pointer must name a valid aligned register of exactly T's width/layout,
    /// access and side effects. Respect hardware clocks, power and ownership.
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr:*mut T)->Self {Self{ptr:ptr as _,phantom:PhantomData,reset:DefaultReset}}
}
impl<T:Copy,A:Access,WB,RB> Reg<T,A,WB,RB,GivenReset<T>> {
    /// # Safety
    /// from_ptr's contract applies. reset must be this exact register element's
    /// audited complete reset value, including documented reserved bits.
    #[inline(always)]
    pub const unsafe fn from_ptr_with_reset(ptr:*mut T,reset:T)->Self {Self{ptr:ptr as _,phantom:PhantomData,reset:GivenReset(reset)}}
}
impl<T:Copy,A:Access,WB,RB,D> Reg<T,A,WB,RB,D> {
    #[inline(always)] pub const fn as_ptr(&self)->*mut T {self.ptr as _}
}
impl<T:Copy,A:Read,WB,RB,D> Reg<T,A,WB,RB,D> {
    #[inline(always)] pub fn read(&self)->T {unsafe{(self.ptr as *mut T).read_volatile()}}
}
impl<T:Copy,A:Write,WB,RB,D> Reg<T,A,WB,RB,D> {
    /// Write the explicit typed value; no default merge or hardware read.
    #[inline(always)] pub fn write_value(&self,val:T) {unsafe{(self.ptr as *mut T).write_volatile(val)}}
}
impl<T:Copy,A:Write,WB,RB,D:Reset<T>> Reg<T,A,WB,RB,D> {
    /// Initialize from this register/element's audited complete reset then write
    /// once. Unknown defaults do not expose this method. A reset word is not a
    /// neutral command: callers must still obey flags, keys and reserved bits.
    #[inline(always)] pub fn write(&self,f:impl FnOnce(&mut T)) {let mut val=self.reset.value();f(&mut val);self.write_value(val);}
}
impl<T:Copy,A:Read+Write,D> Reg<T,A,Ordinary,Ordinary,D> {
    /// One read and one write, not atomic. Only ordinary RW permits modify.
    #[inline(always)] pub fn modify(&self,f:impl FnOnce(&mut T)) {let mut val=self.read();f(&mut val);self.write_value(val);}
}

"#;
/// Render register declarations from the validated IR, never from a chip template.
fn render_chip_pac(ir: &Ir, overrides: &ResetOverrides, shared: &str) -> Result<String> {
    validate(ir)?;
    let mut out =
        String::from("// Generated by cw32-gen (pac stage). Edit YAML sources, not this file.\n");
    out.push_str(&format!(
        "#[path = {path:?}]\npub mod common;\npub const UNKNOWN_RESET:u64=0x1_0000_0000;\n",
        path = format!("{shared}common.rs")
    ));
    for block in ir.blocks.values() {
        out.push_str(&format!(
            "#[path = {path:?}]\npub mod {};\n",
            block.name,
            path = format!("{shared}peripherals/{}_{}.rs", block.name, block.version)
        ));
    }
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
        let parameters =
            instance_reset_registers(&overrides[&(p.block.clone(), p.version.clone())]);
        let arguments = if parameters.is_empty() {
            String::new()
        } else {
            let values: Vec<_> = parameters
                .iter()
                .map(|name| {
                    p.register_resets
                        .iter()
                        .find(|r| &r.register == name)
                        .map(|r| u64::from(r.reset_value))
                        .or_else(|| {
                            ir.blocks[&p.block]
                                .registers
                                .iter()
                                .find(|r| &r.name == name)
                                .unwrap()
                                .reset_value
                                .map(u64::from)
                        })
                        .unwrap_or(0x1_0000_0000)
                        .to_string()
                })
                .collect();
            format!("<{}>", values.join(","))
        };
        let peripheral_type = value_name(&p.block);
        out.push_str(&format!("pub const {}: {}::{peripheral_type}{arguments} = unsafe {{ {}::{peripheral_type}::from_ptr({}_BASE as *mut ()) }};\n",p.name,p.block,p.block,p.name));
    }
    out.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n#[repr(u16)]\n#[allow(non_camel_case_types)]\npub enum Interrupt {\n");
    for i in &ir.family.interrupts {
        out.push_str(&format!("{} = {},\n", i.name, i.number));
    }
    out.push_str("}\nimpl Interrupt { pub const fn number(self)->u16 { self as u16 } }\n");
    Ok(out)
}
fn render_block(b: &crate::schema::Block, overrides: &[crate::schema::RegisterReset]) -> String {
    let mut out = String::from("// Generated shared peripheral kind/version. Do not edit.\n");
    let parameters = instance_reset_registers(overrides);
    let declaration = if parameters.is_empty() {
        String::new()
    } else {
        format!(
            "<{}>",
            parameters
                .iter()
                .map(|name| {
                    let default = b
                        .registers
                        .iter()
                        .find(|r| &r.name == name)
                        .unwrap()
                        .reset_value
                        .map(u64::from)
                        .unwrap_or(0x1_0000_0000);
                    format!("const RESET_{name}:u64={default}")
                })
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let generics = if parameters.is_empty() {
        String::new()
    } else {
        format!(
            "<{}>",
            parameters
                .iter()
                .map(|name| format!("const RESET_{name}:u64"))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let arguments = if parameters.is_empty() {
        String::new()
    } else {
        format!(
            "<{}>",
            parameters
                .iter()
                .map(|name| format!("RESET_{name}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let peripheral_type = value_name(&b.name);
    out.push_str(&format!("#[derive(Clone, Copy, PartialEq, Eq)]\npub struct {peripheral_type}{declaration} {{ ptr: *mut u8 }}\nunsafe impl{generics} Send for {peripheral_type}{arguments} {{}}\nunsafe impl{generics} Sync for {peripheral_type}{arguments} {{}}\nimpl{generics} {peripheral_type}{arguments} {{\n/// # Safety\n/// The pointer and instance reset parameters must identify this mapped register block.\n#[inline(always)] pub const unsafe fn from_ptr(ptr:*mut ())->Self {{ Self {{ptr:ptr as _}} }}\n#[inline(always)] pub const fn as_ptr(&self)->*mut () {{self.ptr as _}}\n"));
    for r in &b.registers {
        let argument = if parameters.contains(&r.name) {
            format!("<RESET_{}>", r.name)
        } else {
            String::new()
        };
        let access = match r.access.as_str() {
            "ro" => "R",
            "wo" => "W",
            _ => "RW",
        };
        let value = format!("regs::{}{argument}", value_name(&r.name));
        let varying_reset = r.array.is_some()
            && r.reset_value.is_none()
            && r.elements.iter().all(|e| e.reset_value.is_some());
        let behavior = if varying_reset
            || r.write_behavior != crate::schema::WriteBehavior::Ordinary
            || r.read_behavior != crate::schema::ReadBehavior::Ordinary
        {
            format!(
                ",crate::common::{:?},crate::common::{}",
                r.write_behavior,
                read_marker(r.read_behavior)
            )
        } else {
            String::new()
        };
        let policy = if varying_reset {
            format!(",crate::common::GivenReset<{value}>")
        } else {
            String::new()
        };
        let (index, check, offset) = indexed_offset(r.offset, r.array.as_ref());
        let constructor = if varying_reset {
            let defaults = r
                .elements
                .iter()
                .map(|e| e.reset_value.unwrap().to_string())
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "crate::common::Reg::from_ptr_with_reset(self.ptr.wrapping_add({offset}) as _,{value}([{defaults}][n]))"
            )
        } else {
            format!("crate::common::Reg::from_ptr(self.ptr.wrapping_add({offset}) as _)")
        };
        out.push_str(&format!("#[doc = {:?}]\n#[inline(always)] pub const fn {}(self,{index})->crate::common::Reg<{value},crate::common::{access}{behavior}{policy}> {{{check} unsafe {{{constructor}}} }}\n",r.description,rust_ident(&r.name.to_ascii_lowercase())));
    }
    for child in &b.blocks {
        let (index, check, offset) = indexed_offset(child.offset, child.array.as_ref());
        let ty = format!("crate::{}::{}", child.block, value_name(&child.block));
        out.push_str(&format!("#[doc = {:?}]\n#[inline(always)] pub const fn {}(self,{index})->{ty} {{{check} unsafe {{{ty}::from_ptr(self.ptr.wrapping_add({offset}) as _)}} }}\n",child.description,rust_ident(&child.name.to_ascii_lowercase())));
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
    out.push_str("pub mod regs {\n");
    for r in &b.registers {
        let register_overrides: Vec<_> = overrides
            .iter()
            .filter(|reset| reset.register == r.name)
            .collect();
        render_value(&mut out, r, &register_overrides);
    }
    out.push_str("}\n");
    out.push_str("pub mod vals {\n");
    for r in &b.registers {
        for f in &r.fields {
            if f.kind != "enum" {
                continue;
            }
            render_enum(&mut out, r, f);
        }
    }
    out.push_str("}\n");
    out
}
/// Mirrors chiptool bcf538a2's exhaustive enum / sparse-newtype policy.
/// Every masked bit pattern is valid, including documented reserved encodings.
fn render_enum(out: &mut String, register: &crate::schema::Register, field: &crate::schema::Field) {
    let name = format!("{}{}", value_name(&register.name), value_name(&field.name));
    let ty = field_word(field.bit_size);
    let count = 1u64 << field.bit_size;
    let reserved = count - field.values.len() as u64;
    let sparse = reserved >= 100 && reserved >= field.values.len() as u64;
    let mask = u32::MAX >> (32 - field.bit_size);
    out.push_str(&format!(
        "#[doc = {:?}]\n#[allow(non_camel_case_types)]\n",
        format!(
            "{} Source: {}",
            field.description,
            field.values_source.as_deref().unwrap()
        )
    ));
    if sparse {
        out.push_str(&format!("#[repr(transparent)]\n#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]\npub struct {name}({ty});\nimpl {name} {{\n"));
        for value in &field.values {
            out.push_str(&format!(
                "pub const {}:Self=Self({});\n",
                value.name, value.value
            ));
        }
        out.push_str(&format!("#[inline(always)] pub const fn from_bits(value:{ty})->Self {{Self(value & {mask:#x})}}\n#[inline(always)] pub const fn to_bits(self)->{ty} {{self.0}}\n}}\nimpl core::fmt::Debug for {name} {{fn fmt(&self,f:&mut core::fmt::Formatter<'_>)->core::fmt::Result {{match self.0 {{"));
        for value in &field.values {
            out.push_str(&format!("{}=>f.write_str({:?}),", value.value, value.name));
        }
        out.push_str("other=>core::write!(f,\"0x{:02X}\",other)}}}\n");
    } else {
        out.push_str(&format!("#[repr({ty})]\n#[derive(Copy,Clone,Debug,Eq,PartialEq,Ord,PartialOrd)]\npub enum {name} {{\n"));
        for bits in 0..count {
            let variant = field
                .values
                .iter()
                .find(|value| u64::from(value.value) == bits)
                .map(|value| value.name.clone())
                .unwrap_or_else(|| format!("_RESERVED_{bits:x}"));
            out.push_str(&format!("{variant}={bits},\n"));
        }
        // All patterns through mask are emitted above, making this total.
        out.push_str(&format!("}}\nimpl {name} {{\n#[inline(always)] pub const fn from_bits(value:{ty})->Self {{unsafe {{core::mem::transmute(value & {mask:#x})}}}}\n#[inline(always)] pub const fn to_bits(self)->{ty} {{self as {ty}}}\n}}\n"));
    }
    out.push_str(&format!("impl From<{ty}> for {name} {{#[inline(always)] fn from(value:{ty})->Self {{Self::from_bits(value)}}}}\nimpl From<{name}> for {ty} {{#[inline(always)] fn from(value:{name})->Self {{value.to_bits()}}}}\n"));
}
fn field_word(bit_size: u8) -> &'static str {
    match bit_size {
        1..=8 => "u8",
        9..=16 => "u16",
        _ => "u32",
    }
}
fn indexed_offset(base: usize, array: Option<&crate::schema::Array>) -> (String, String, String) {
    match array {
        None => (String::new(), String::new(), base.to_string()),
        Some(array) => {
            let offsets = array.offsets().expect("validated array");
            let offset = match array {
                crate::schema::Array::Regular(a) => format!("{base}+n*{}", a.stride),
                crate::schema::Array::Explicit(_) => format!(
                    "{base}+[{}][n]",
                    offsets
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            };
            (
                "n:usize".into(),
                format!("assert!(n<{});", offsets.len()),
                offset,
            )
        }
    }
}
fn render_array(array: Option<&crate::schema::Array>) -> String {
    match array {
        None => "None".into(),
        Some(crate::schema::Array::Regular(a)) => {
            format!("Some(Array::Regular {{len:{},stride:{}}})", a.len, a.stride)
        }
        Some(crate::schema::Array::Explicit(a)) => {
            format!("Some(Array::Explicit(&{:?}))", a.offsets)
        }
    }
}
/// Register values are deliberately distinct even when two layouts are identical.
/// A reset value belongs to a register, not a reusable fieldset.
fn value_name(name: &str) -> String {
    let mut result = String::new();
    for part in name.split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_ascii_uppercase());
            result.extend(chars.map(|c| c.to_ascii_lowercase()));
        }
    }
    result
}
type ResetOverrides = BTreeMap<(String, String), Vec<crate::schema::RegisterReset>>;
fn instance_reset_registers(overrides: &[crate::schema::RegisterReset]) -> BTreeSet<String> {
    overrides
        .iter()
        .map(|reset| reset.register.clone())
        .collect()
}
fn render_value(
    out: &mut String,
    r: &crate::schema::Register,
    overrides: &[&crate::schema::RegisterReset],
) {
    let name = value_name(&r.name);
    let word = format!("u{}", r.bit_size);
    let default = r.reset_value.map(u64::from).unwrap_or(0x1_0000_0000);
    let declaration = if overrides.is_empty() {
        String::new()
    } else {
        format!("<const RESET:u64={default}>")
    };
    let generics = if overrides.is_empty() {
        ""
    } else {
        "<const RESET:u64>"
    };
    let arguments = if overrides.is_empty() { "" } else { "<RESET>" };
    out.push_str(&format!("#[doc = {:?}]\n#[repr(transparent)]\n#[derive(Clone,Copy,Debug,PartialEq,Eq)]\npub struct {name}{declaration}(pub {word});\n", r.description));
    let mut evidence: BTreeMap<u32, (BTreeSet<&str>, BTreeSet<&str>)> = BTreeMap::new();
    if let Some(value) = r.reset_value {
        let entry = evidence.entry(value).or_default();
        entry.0.insert(r.reset_source.as_deref().unwrap());
        entry.1.extend(r.reset_note.as_deref());
    }
    for reset in overrides {
        let entry = evidence.entry(reset.reset_value).or_default();
        entry.0.insert(reset.reset_source.as_str());
        entry.1.extend(reset.reset_note.as_deref());
    }
    let defaults: BTreeMap<_, _> = evidence
        .into_iter()
        .map(|(value, (sources, notes))| {
            let source = sources.into_iter().collect::<Vec<_>>().join(" | ");
            let note =
                (!notes.is_empty()).then(|| notes.into_iter().collect::<Vec<_>>().join(" | "));
            (value, (source, note))
        })
        .collect();
    for (value, (source, note)) in &defaults {
        let argument = if overrides.is_empty() {
            String::new()
        } else {
            format!("<{value}>")
        };
        out.push_str(&format!("#[doc = {:?}]\nimpl Default for {name}{argument} {{fn default()->Self {{Self({value:#x})}}}}\n", format!("Documented reset word. {source} {}", note.as_deref().unwrap_or(""))));
    }
    let (reset_value, reset_source, reset_note) = if overrides.is_empty() {
        (
            format!("{:?}", r.reset_value),
            format!("{:?}", r.reset_source),
            format!("{:?}", r.reset_note),
        )
    } else {
        let values = defaults
            .keys()
            .map(|value| format!("{value}=>Some({value}),"))
            .collect::<String>();
        let sources = defaults
            .iter()
            .map(|(value, (source, _))| format!("{value}=>Some({source:?}),"))
            .collect::<String>();
        let notes = defaults
            .iter()
            .map(|(value, (_, note))| format!("{value}=>{note:?},"))
            .collect::<String>();
        (
            format!("match RESET {{{values}_=>None}}"),
            format!(
                "match RESET {{{sources}0x1_0000_0000=>{:?},_=>None}}",
                r.reset_value
                    .is_none()
                    .then_some(r.reset_source.as_deref())
                    .flatten()
            ),
            format!(
                "match RESET {{{notes}0x1_0000_0000=>{:?},_=>None}}",
                r.reset_value
                    .is_none()
                    .then_some(r.reset_note.as_deref())
                    .flatten()
            ),
        )
    };
    out.push_str(&format!("impl{generics} {name}{arguments} {{\npub const RESET_VALUE:Option<{word}>={reset_value};\npub const RESET_SOURCE:Option<&'static str>={reset_source};\npub const RESET_NOTE:Option<&'static str>={reset_note};\npub const fn from_bits(bits:{word})->Self {{Self(bits)}}\npub const fn bits(self)->{word} {{self.0}}\n"));
    if r.array.is_some() {
        out.push_str(&format!("pub const RESET_VALUES:&'static [Option<{word}>]=&{:?};\npub const RESET_SOURCES:&'static [Option<&'static str>]=&{:?};\npub const RESET_NOTES:&'static [Option<&'static str>]=&{:?};\n",r.elements.iter().map(|e|e.reset_value).collect::<Vec<_>>(),r.elements.iter().map(|e|e.reset_source.as_deref()).collect::<Vec<_>>(),r.elements.iter().map(|e|e.reset_note.as_deref()).collect::<Vec<_>>()));
    }
    for f in &r.fields {
        let method = rust_ident(&f.name.to_ascii_lowercase());
        let setter = format!("set_{}", f.name.to_ascii_lowercase());
        let enum_type = format!(
            "super::vals::{}{}",
            value_name(&r.name),
            value_name(&f.name)
        );
        let ty = match f.kind.as_str() {
            "bool" => "bool".into(),
            "enum" => enum_type.clone(),
            _ => format!(
                "u{}",
                if f.bit_size <= 8 {
                    8
                } else if f.bit_size <= 16 {
                    16
                } else {
                    32
                }
            ),
        };
        let (index, assertion, shift) = indexed_offset(usize::from(f.bit_offset), f.array.as_ref());
        let index = if index.is_empty() { "" } else { "n:usize," };
        let mask = u32::MAX >> (32 - f.bit_size);
        let read = match f.kind.as_str() {
            "bool" => format!("((self.0 >> ({shift})) & 1) != 0"),
            "enum" => format!(
                "{enum_type}::from_bits(((self.0 >> ({shift})) & {mask:#x}) as {})",
                field_word(f.bit_size)
            ),
            _ => format!("((self.0 >> ({shift})) & {mask:#x}) as {ty}"),
        };
        {
            let read_ty = ty.clone();
            out.push_str(&format!("#[doc = {:?}]\n#[inline(always)] pub const fn {method}(&self,{index})->{read_ty} {{{assertion}{read}}}\n", f.description));
        }
        {
            let value_bits = if f.kind == "enum" {
                "value.to_bits()"
            } else {
                "value"
            };
            out.push_str(&format!("#[doc = {:?}]\n#[inline(always)] pub const fn {setter}(&mut self,{index}value:{ty}) {{{assertion}self.0=(self.0 & !(({mask:#x} as {word}) << ({shift}))) | ((({value_bits} as {word}) & {mask:#x}) << ({shift}));}}\n", f.description));
        }
    }
    out.push_str("}\n");
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
pub struct Peripheral { pub name: &'static str, pub block: &'static str, pub version: &'static str, pub address: usize, pub register_resets:&'static [RegisterReset], pub clock_bit: Option<u8>, pub clock_gate:Option<RegisterBit>, pub clock_tree:Option<ClockTree>, pub reset:Option<RegisterBit>, pub reset_effects:&'static [ResetEffect], pub ownership_parent:Option<&'static str>, pub comparator:Option<ComparatorConnections>, pub opa:Option<OpaConnections>, pub dma:Option<DmaController>, pub timer_capture_mux:Option<TimerCaptureMux>, pub registers:&'static [Register], pub blocks:&'static [BlockItem], pub implemented_mask: u16, pub pulldown_mask: u16 }
#[derive(Clone,Copy,Debug)]
pub enum ClockSource {Hclk,Pclk}
#[derive(Clone,Copy,Debug)]
pub struct ClockTree {pub bus_clock:ClockSource,pub kernel_clock:Option<ClockSource>,pub source:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct DmaChannel {pub peripheral:&'static str,pub number:u8,pub index:u8,pub interrupt:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct DmaRequest {pub peripheral:&'static str,pub signal:&'static str,pub selector:u8}
#[derive(Clone,Copy,Debug)]
pub struct DmaController {pub channels:&'static [DmaChannel],pub requests:&'static [DmaRequest],pub source:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct TimerCaptureMux {pub peripheral:&'static str,pub register:&'static str,pub index:usize,pub field:&'static str,pub external_value:u32,pub source:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct DacConnection {pub peripheral:&'static str,pub channel:u8}
#[derive(Clone,Copy,Debug)]
pub struct ComparatorConnections {pub reference:Option<&'static str>,pub dac:Option<DacConnection>,pub source:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct OpaConnections {pub dac:DacConnection,pub source:&'static str}
#[derive(Clone, Copy, Debug)]
pub struct ResetEffect {pub peripheral:&'static str,pub description:&'static str,pub source:&'static str}
#[derive(Clone, Copy, Debug)]
pub struct RegisterReset {pub register:&'static str,pub reset_value:u32,pub reset_source:&'static str,pub reset_note:Option<&'static str>}
#[derive(Clone, Copy, Debug)]
pub struct RegisterBit { pub peripheral:&'static str, pub register:&'static str, pub field:&'static str, pub bit:u8, pub offset:usize, pub shared:bool }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access { ReadOnly, ReadWrite, WriteOnly }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteBehavior { Ordinary, ZeroToClear, OneToClear, OneToSet, Toggle, Command, Keyed, Mixed, WriteOnce }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadBehavior { Ordinary, Clear, Fifo, Latch }
#[derive(Clone, Copy, Debug)]
pub struct Register {pub name:&'static str,pub description:&'static str,pub offset:usize,pub bit_size:u8,pub array:Option<Array>,pub elements:&'static [RegisterElement],pub reset_value:Option<u32>,pub reset_source:Option<&'static str>,pub reset_note:Option<&'static str>,pub access:Access,pub alias_of:Option<&'static str>,pub write_behavior:WriteBehavior,pub read_behavior:ReadBehavior,pub fields:&'static [RegisterField]}
#[derive(Clone,Copy,Debug)]
pub enum Array {Regular {len:usize,stride:usize}, Explicit(&'static [usize])}
#[derive(Clone,Copy,Debug)]
pub struct RegisterElement {pub name:&'static str,pub description:&'static str,pub reset_value:Option<u32>,pub reset_source:Option<&'static str>,pub reset_note:Option<&'static str>}
#[derive(Clone,Copy,Debug)]
pub struct BlockItem {pub name:&'static str,pub offset:usize,pub block:&'static str,pub version:&'static str,pub array:Option<Array>,pub description:&'static str,pub source:&'static str,pub registers:&'static [Register],pub blocks:&'static [BlockItem]}
#[derive(Clone, Copy, Debug)]
pub struct FieldElement {pub name:&'static str,pub bit_offset:u8,pub bit_size:u8,pub access:Access,pub kind:FieldKind,pub values:&'static [EnumValue],pub description:&'static str}
#[derive(Clone, Copy, Debug)]
pub struct RegisterField {pub name:&'static str,pub description:&'static str,pub bit_offset:u8,pub bit_size:u8,pub array:Option<Array>,pub elements:&'static [FieldElement],pub array_source:Option<&'static str>,pub values_source:Option<&'static str>,pub access:Access,pub kind:FieldKind,pub values:&'static [EnumValue]}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FieldKind {Raw,Bool,Enum}
#[derive(Clone,Copy,Debug)]
pub struct EnumValue {pub name:&'static str,pub value:u32}
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
fn render_block_metadata(block: &crate::schema::Block) -> String {
    let mut out =
        String::from("// Generated shared register metadata. Do not edit.\nuse super::*;\n");
    out.push_str("pub static REGISTERS: &[Register] = &[\n");
    for r in &block.registers {
        let elements=r.elements.iter().map(|e|format!("RegisterElement {{name:{:?},description:{:?},reset_value:{:?},reset_source:{:?},reset_note:{:?}}}",e.name,e.description,e.reset_value,e.reset_source,e.reset_note)).collect::<Vec<_>>().join(",");
        let array = render_array(r.array.as_ref());
        out.push_str(&format!("Register {{name:{:?},description:{:?},offset:{},bit_size:{},array:{array},elements:&[{elements}],reset_value:{:?},reset_source:{:?},reset_note:{:?},access:Access::{},alias_of:{:?},write_behavior:WriteBehavior::{:?},read_behavior:ReadBehavior::{:?},fields:&[",r.name,r.description,r.offset,r.bit_size,r.reset_value,r.reset_source,r.reset_note,access_marker(&r.access),r.alias_of,r.write_behavior,r.read_behavior));
        for f in &r.fields {
            let values = f
                .values
                .iter()
                .map(|v| format!("EnumValue {{name:{:?},value:{}}}", v.name, v.value))
                .collect::<Vec<_>>()
                .join(",");
            let kind = value_name(&f.kind);
            let elements = f.elements.iter().map(|e| {
                let values = e.values.iter().map(|v| format!("EnumValue {{name:{:?},value:{}}}", v.name, v.value)).collect::<Vec<_>>().join(",");
                format!("FieldElement {{name:{:?},bit_offset:{},bit_size:{},access:Access::{},kind:FieldKind::{},values:&[{values}],description:{:?}}}",e.name,e.bit_offset,e.bit_size,access_marker(&e.access),value_name(&e.kind),e.description)
            }).collect::<Vec<_>>().join(",");
            out.push_str(&format!(
                "RegisterField {{name:{:?},description:{:?},bit_offset:{},bit_size:{},array:{},elements:&[{elements}],array_source:{:?},values_source:{:?},access:Access::{},kind:FieldKind::{kind},values:&[{values}]}},",
                f.name,
                f.description,
                f.bit_offset,
                f.bit_size,
                render_array(f.array.as_ref()),
                f.array_source,
                f.values_source,
                access_marker(&f.access)
            ));
        }
        out.push_str("]},\n");
    }
    out.push_str("];\n");
    out.push_str("pub static BLOCKS:&[BlockItem]=&[\n");
    for child in &block.blocks {
        out.push_str(&format!("BlockItem {{name:{:?},offset:{},block:{:?},version:{:?},array:{},description:{:?},source:{:?},registers:super::{}::REGISTERS,blocks:super::{}::BLOCKS}},\n",child.name,child.offset,child.block,child.version,render_array(child.array.as_ref()),child.description,child.source,child.block,child.block));
    }
    out.push_str("];\n");
    out
}
fn render_chip_metadata(ir: &Ir, shared: &str) -> Result<String> {
    validate(ir)?;
    let mut out = String::from("// Generated; no maintained Rust chip tables.\n");
    out.push_str(&format!(
        "include!({:?});\n",
        format!("{shared}metadata_types.rs")
    ));
    for block in ir.blocks.values() {
        out.push_str(&format!(
            "#[path = {path:?}]\nmod {};\n",
            block.name,
            path = format!("{shared}registers/{}_{}.rs", block.name, block.version)
        ));
    }
    out.push_str(&format!("pub static METADATA: Metadata = Metadata {{ schema_version:{}, name: {:?}, family: {:?}, core: {:?}, target: {:?},\nperipherals: &[\n",ir.schema_version,ir.chip.name,ir.family.name,ir.family.core,ir.family.target));
    for p in &ir.family.peripherals {
        let effects = p
            .reset_effects
            .iter()
            .map(|e| {
                format!(
                    "ResetEffect {{peripheral:{:?},description:{:?},source:{:?}}}",
                    e.peripheral, e.description, e.source
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let comparator = p
            .comparator
            .as_ref()
            .map(|connection| {
                let dac = connection
                    .dac
                    .as_ref()
                    .map(|dac| {
                        format!(
                            "Some(DacConnection {{peripheral:{:?},channel:{}}})",
                            dac.peripheral, dac.channel
                        )
                    })
                    .unwrap_or_else(|| "None".into());
                format!(
                    "Some(ComparatorConnections {{reference:{:?},dac:{dac},source:{:?}}})",
                    connection.reference, connection.source
                )
            })
            .unwrap_or_else(|| "None".into());
        let opa = p.opa.as_ref().map(|connection| {
            format!("Some(OpaConnections {{dac:DacConnection {{peripheral:{:?},channel:{}}},source:{:?}}})", connection.dac.peripheral, connection.dac.channel, connection.source)
        }).unwrap_or_else(|| "None".into());
        let dma = p.dma.as_ref().map(|dma| {
            let channels = dma.channels.iter().map(|channel| format!("DmaChannel {{peripheral:{:?},number:{},index:{},interrupt:{:?}}}", channel.peripheral, channel.number, channel.index, channel.interrupt)).collect::<Vec<_>>().join(",");
            let requests = dma.requests.iter().map(|request| format!("DmaRequest {{peripheral:{:?},signal:{:?},selector:{}}}", request.peripheral, request.signal, request.selector)).collect::<Vec<_>>().join(",");
            format!("Some(DmaController {{channels:&[{channels}],requests:&[{requests}],source:{:?}}})", dma.source)
        }).unwrap_or_else(|| "None".into());
        let clock_tree = p.clock_tree.as_ref().map(|clock| {
            let kernel = clock.kernel_clock.map(|source| format!("Some(ClockSource::{source:?})")).unwrap_or_else(|| "None".into());
            format!("Some(ClockTree {{bus_clock:ClockSource::{:?},kernel_clock:{kernel},source:{:?}}})",clock.bus_clock,clock.source)
        }).unwrap_or_else(|| "None".into());
        let timer_capture_mux = p.timer_capture_mux.as_ref().map(|mux| {
            format!("Some(TimerCaptureMux {{peripheral:{:?},register:{:?},index:{},field:{:?},external_value:{},source:{:?}}})",mux.peripheral,mux.register,mux.index,mux.field,mux.external_value,mux.source)
        }).unwrap_or_else(|| "None".into());
        let resets = p.register_resets.iter().map(|r| format!("RegisterReset {{register:{:?},reset_value:{},reset_source:{:?},reset_note:{:?}}}",r.register,r.reset_value,r.reset_source,r.reset_note)).collect::<Vec<_>>().join(",");
        out.push_str(&format!("Peripheral {{ name: {:?}, block: {:?}, version: {:?}, address: {:#x}, register_resets:&[{resets}], clock_bit: {:?}, clock_gate:{}, clock_tree:{clock_tree}, reset:{}, reset_effects:&[{effects}], ownership_parent:{:?}, comparator:{comparator}, opa:{opa}, dma:{dma}, timer_capture_mux:{timer_capture_mux}, registers:{}::REGISTERS, blocks:{}::BLOCKS, implemented_mask: {}, pulldown_mask: {} }},\n",p.name,p.block,p.version,p.address,p.clock_bit,render_register_bit(ir,p.clock_gate.as_ref()),render_register_bit(ir,p.reset.as_ref()),p.ownership_parent,p.block,p.block,p.implemented_mask,p.pulldown_mask));
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
/// Collect module definitions and instance-reset policies across selected chips.
/// A kind/version is one hardware contract, so conflicting definitions fail.
fn shared_inputs(
    irs: &[Ir],
) -> Result<(
    BTreeMap<(String, String), crate::schema::Block>,
    ResetOverrides,
)> {
    let mut blocks = BTreeMap::new();
    let mut overrides: ResetOverrides = BTreeMap::new();
    let mut names = BTreeSet::new();
    for ir in irs {
        validate(ir)?;
        if !names.insert(ir.chip.name.to_ascii_lowercase()) {
            return Err(format!("duplicate chip {}", ir.chip.name).into());
        }
        for block in ir.blocks.values() {
            let key = (block.name.clone(), block.version.clone());
            if let Some(previous) = blocks.get(&key) {
                if serde_json::to_vec(previous)? != serde_json::to_vec(block)? {
                    return Err(format!(
                        "conflicting shared register definition {}_{}",
                        key.0, key.1
                    )
                    .into());
                }
            } else {
                blocks.insert(key.clone(), block.clone());
            }
            overrides.entry(key).or_default();
        }
        for peripheral in &ir.family.peripherals {
            overrides
                .get_mut(&(peripheral.block.clone(), peripheral.version.clone()))
                .unwrap()
                .extend(peripheral.register_resets.iter().cloned());
        }
    }
    let mut filenames = BTreeSet::new();
    for (kind, version) in blocks.keys() {
        if !filenames.insert(format!("{kind}_{version}")) {
            return Err("ambiguous shared register kind/version filename".into());
        }
    }
    // JSON file order must not change generated specialization or provenance.
    for resets in overrides.values_mut() {
        resets.sort_by(|a, b| {
            (&a.register, a.reset_value, &a.reset_source, &a.reset_note).cmp(&(
                &b.register,
                b.reset_value,
                &b.reset_source,
                &b.reset_note,
            ))
        });
        resets.dedup_by(|a, b| {
            a.register == b.register
                && a.reset_value == b.reset_value
                && a.reset_source == b.reset_source
                && a.reset_note == b.reset_note
        });
    }
    Ok((blocks, overrides))
}

fn generate_shared(irs: &[Ir], out: &Path, nested: bool) -> Result<()> {
    if irs.is_empty() {
        return Err("no chip JSON selected for PAC generation".into());
    }
    let (blocks, overrides) = shared_inputs(irs)?;
    fs::create_dir_all(out.join("peripherals"))?;
    fs::create_dir_all(out.join("registers"))?;
    fs::write(out.join("common.rs"), COMMON_API)?;
    fs::write(out.join("metadata_types.rs"), METADATA_TYPES)?;
    for ((kind, version), block) in &blocks {
        fs::write(
            out.join(format!("peripherals/{kind}_{version}.rs")),
            render_block(block, &overrides[&(kind.clone(), version.clone())]),
        )?;
        fs::write(
            out.join(format!("registers/{kind}_{version}.rs")),
            render_block_metadata(block),
        )?;
    }
    for ir in irs {
        let chip_out = if nested {
            out.join("chips").join(ir.chip.name.to_ascii_lowercase())
        } else {
            out.to_owned()
        };
        let shared = if nested { "../../" } else { "" };
        fs::create_dir_all(&chip_out)?;
        fs::write(
            chip_out.join("pac.rs"),
            render_chip_pac(ir, &overrides, shared)?,
        )?;
        fs::write(
            chip_out.join("metadata.rs"),
            render_chip_metadata(ir, shared)?,
        )?;
        fs::write(chip_out.join("rt.rs"), render_runtime(ir)?)?;
        fs::write(chip_out.join("device.x"), render_device_x(ir)?)?;
    }
    Ok(())
}

/// Render a standalone chip root, with shared modules alongside its output.
pub fn render_pac(ir: &Ir) -> Result<String> {
    let (_, overrides) = shared_inputs(std::slice::from_ref(ir))?;
    render_chip_pac(ir, &overrides, "")
}

/// Render standalone metadata, selecting its shared types and register modules.
pub fn render_metadata(ir: &Ir) -> Result<String> {
    render_chip_metadata(ir, "")
}

/// Stable standalone output: no time, absolute paths or ambient state.
pub fn generate(ir: &Ir, out: &Path) -> Result<()> {
    generate_shared(std::slice::from_ref(ir), out, false)
}

/// Consume one explicit JSON artifact; keep pac.rs at the requested output root.
pub fn generate_from_json(json_path: &Path, out: &Path) -> Result<()> {
    generate(&load_json(json_path)?, out)
}

/// Re-read every persisted chip JSON, then emit each shared IP exactly once.
pub fn generate_many_from_json(json_paths: &[PathBuf], out: &Path) -> Result<()> {
    let irs = json_paths
        .iter()
        .map(|path| load_json(path))
        .collect::<Result<Vec<_>>>()?;
    generate_shared(&irs, out, true)
}
