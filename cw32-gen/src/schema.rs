//! Shared, versioned normalized IR and validation. No YAML loader or PAC renderer.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// Version 5 adds explicit indexed fields, registers and subblocks; reset evidence remains. Older
/// normalized JSON must be regenerated; unknown legacy fields are rejected.
/// An omitted reset value is unknown, never an implicit zero. An omitted source
/// YAML `bit_size` still means a 32-bit bus transaction.
pub const SCHEMA_VERSION: u32 = 5;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteBehavior {
    #[default]
    Ordinary,
    ZeroToClear,
    OneToClear,
    OneToSet,
    Toggle,
    Command,
    Keyed,
    Mixed,
    WriteOnce,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadBehavior {
    #[default]
    Ordinary,
    Clear,
    Fifo,
    Latch,
}
macro_rules! model { ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
    #[derive(Debug, Clone, Deserialize, Serialize)] #[serde(deny_unknown_fields)]
    pub struct $name { $(pub $field: $ty),* }
}; }
model!(Constant {
    name: String,
    value: u32,
    description: String
});
model!(EnumValue {
    name: String,
    value: u32
});
fn raw_kind() -> String {
    "raw".into()
}
/// Maximum expansion of one array. Explicit limits reject unreasonable source
/// data before allocation while allowing arrays much larger than these devices.
const MAX_ARRAY_LEN: usize = 65_536;
const MAX_EXPANDED_REGISTERS: usize = 1_048_576;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegularArray {
    pub len: usize,
    pub stride: usize,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplicitArray {
    pub offsets: Vec<usize>,
}
/// Index order is source order, including for irregular arrays. Offsets are
/// relative to the enclosing register or block item's declared base offset.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Array {
    Regular(RegularArray),
    Explicit(ExplicitArray),
}
impl Array {
    pub fn offsets(&self) -> Result<Vec<usize>> {
        let offsets = match self {
            Self::Regular(array) => {
                if array.len == 0 || array.len > MAX_ARRAY_LEN || array.stride == 0 {
                    return Err(err("invalid regular array length or stride"));
                }
                (0..array.len)
                    .map(|index| {
                        index
                            .checked_mul(array.stride)
                            .ok_or_else(|| err("array offset multiplication overflow"))
                    })
                    .collect::<Result<Vec<_>>>()?
            }
            Self::Explicit(array) => {
                if array.offsets.is_empty() || array.offsets.len() > MAX_ARRAY_LEN {
                    return Err(err("invalid explicit array length"));
                }
                let mut seen = BTreeSet::new();
                if array.offsets.iter().any(|offset| !seen.insert(*offset)) {
                    return Err(err("duplicate explicit array offset"));
                }
                array.offsets.clone()
            }
        };
        Ok(offsets)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldArray {
    pub len: u8,
    pub stride: u8,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    pub bit_offset: u8,
    pub bit_size: u8,
    /// Explicit repeated fields: element n begins at bit_offset + n * stride.
    #[serde(default)]
    pub array: Option<FieldArray>,
    pub access: String,
    pub description: String,
    #[serde(default = "raw_kind")]
    pub kind: String,
    #[serde(default)]
    pub values: Vec<EnumValue>,
}
model!(PeripheralInterrupt {
    signal: String,
    interrupt: String
});
model!(Quirk {
    name: String,
    description: String,
    source: String
});
model!(PinRoute { pin:String, peripheral:String, signal:String, af:Option<u8>, remap:Option<String> });
model!(Remap {
    name: String,
    peripheral: String,
    description: String
});
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Register {
    pub name: String,
    pub offset: usize,
    #[serde(default)]
    pub array: Option<Array>,
    /// Original register identities and reset evidence, in array index order.
    #[serde(default)]
    pub elements: Vec<RegisterElement>,
    /// Hardware bus access width, not merely the count of implemented bits.
    #[serde(
        default = "default_register_bit_size",
        skip_serializing_if = "is_default_register_bit_size"
    )]
    pub bit_size: u8,
    /// Full reset word, including documented reserved bits. None means unknown,
    /// variable, or not applicable, and never enables a fabricated Default.
    #[serde(default)]
    pub reset_value: Option<u32>,
    /// Authoritative manual/version/section evidence for this reset word.
    #[serde(default)]
    pub reset_source: Option<String>,
    /// Qualifications, including why a complete reset word is not known.
    #[serde(default)]
    pub reset_note: Option<String>,
    pub access: String,
    pub description: String,
    #[serde(default)]
    pub alias_of: Option<String>,
    #[serde(default)]
    pub write_behavior: WriteBehavior,
    #[serde(default)]
    pub read_behavior: ReadBehavior,
    #[serde(default)]
    pub fields: Vec<Field>,
}
fn default_register_bit_size() -> u8 {
    32
}
fn is_default_register_bit_size(value: &u8) -> bool {
    *value == 32
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterElement {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub reset_value: Option<u32>,
    #[serde(default)]
    pub reset_source: Option<String>,
    #[serde(default)]
    pub reset_note: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockItem {
    pub name: String,
    pub offset: usize,
    pub block: String,
    pub version: String,
    #[serde(default)]
    pub array: Option<Array>,
    pub description: String,
    pub source: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub name: String,
    pub version: String,
    pub registers: Vec<Register>,
    #[serde(default)]
    pub blocks: Vec<BlockItem>,
    pub constants: Vec<Constant>,
}
model!(RegisterBit {
    peripheral: String,
    register: String,
    field: String,
    bit: u8
});
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterReset {
    pub register: String,
    pub reset_value: u32,
    pub reset_source: String,
    #[serde(default)]
    pub reset_note: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peripheral {
    pub name: String,
    pub block: String,
    pub version: String,
    pub address: usize,
    /// Known instance-specific resets override the reusable block's value.
    #[serde(default)]
    pub register_resets: Vec<RegisterReset>,
    pub clock_bit: Option<u8>,
    #[serde(default, alias = "clock")]
    pub clock_gate: Option<RegisterBit>,
    #[serde(default)]
    pub reset: Option<RegisterBit>,
    /// This view shares ownership with its parent and must not receive an independent HAL token.
    #[serde(default)]
    pub ownership_parent: Option<String>,
    #[serde(default)]
    pub vendor_ip: Option<String>,
    #[serde(default)]
    pub vendor_version: Option<String>,
    #[serde(default)]
    pub implemented_mask: u16,
    #[serde(default)]
    pub pulldown_mask: u16,
    #[serde(default)]
    pub interrupts: Vec<PeripheralInterrupt>,
    #[serde(default)]
    pub quirks: Vec<Quirk>,
}
model!(Interrupt {
    name: String,
    number: u16
});
model!(Family { name: String, core: String, target: String, peripherals: Vec<Peripheral>, interrupts: Vec<Interrupt>, constants: Vec<Constant> });
model!(Memory {
    name: String,
    address: usize,
    size: usize
});
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Chip {
    pub name: String,
    pub family: String,
    pub pins: Vec<Pin>,
    pub memory: Vec<Memory>,
    #[serde(default)]
    pub pin_routes: Vec<PinRoute>,
    #[serde(default)]
    pub remaps: Vec<Remap>,
    #[serde(default)]
    pub quirks: Vec<Quirk>,
}

model!(Pin {
    name: String,
    port: String,
    number: u8
});
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ir {
    pub schema_version: u32,
    pub chip: Chip,
    pub family: Family,
    pub blocks: BTreeMap<String, Block>,
}
model!(RegisterRef {
    kind: String,
    version: String
});
model!(ChipDocument { schema_version:u32, chip:Chip, family:Family, registers:Vec<RegisterRef> });
model!(RegisterDocument {
    schema_version: u32,
    block: Block
});
pub fn err(message: impl Into<String>) -> Box<dyn std::error::Error> {
    message.into().into()
}
fn identifier(s: &str) -> bool {
    let mut c = s.chars();
    c.next()
        .is_some_and(|x| x.is_ascii_alphabetic() || x == '_')
        && c.all(|x| x.is_ascii_alphanumeric() || x == '_')
        && ![
            "self", "Self", "type", "mod", "fn", "pub", "struct", "enum", "match", "crate",
            "super", "use",
        ]
        .contains(&s)
}
pub fn check_id(s: &str) -> Result<()> {
    if !identifier(s) {
        return Err(err(format!("invalid identifier: {s}")));
    }
    Ok(())
}
fn unique<'a>(values: impl Iterator<Item = &'a str>, label: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        check_id(value)?;
        if !seen.insert(value) {
            return Err(err(format!("duplicate {label}: {value}")));
        }
    }
    Ok(())
}
/// Validate IR before rendering. Validation is independent of output formatting.
pub fn validate(ir: &Ir) -> Result<()> {
    if ir.schema_version != SCHEMA_VERSION {
        return Err(err("unsupported normalized IR schema version"));
    }
    check_id(&ir.chip.name)?;
    check_id(&ir.family.name)?;
    unique(
        ir.family.peripherals.iter().map(|x| x.name.as_str()),
        "peripheral",
    )?;
    unique(
        ir.family.interrupts.iter().map(|x| x.name.as_str()),
        "interrupt",
    )?;
    unique(
        ir.family.constants.iter().map(|x| x.name.as_str()),
        "constant",
    )?;
    unique(ir.chip.memory.iter().map(|x| x.name.as_str()), "memory")?;
    unique(ir.chip.pins.iter().map(|x| x.name.as_str()), "pin")?;
    let mut irq_numbers = BTreeSet::new();
    for i in &ir.family.interrupts {
        if i.number > 31 || !irq_numbers.insert(i.number) {
            return Err(err("invalid or duplicate Cortex-M0+ external IRQ number"));
        }
    }
    for p in &ir.family.peripherals {
        if p.address % 4 != 0
            || p.clock_bit.is_some_and(|x| x >= 32)
            || p.pulldown_mask & !p.implemented_mask != 0
        {
            return Err(err("invalid peripheral address/clock/pin mask"));
        }
        let block = ir
            .blocks
            .get(&p.block)
            .ok_or_else(|| err("unknown peripheral block"))?;
        if block.version != p.version {
            return Err(err("peripheral version mismatch"));
        }
        unique(
            p.register_resets
                .iter()
                .map(|reset| reset.register.as_str()),
            "instance register reset",
        )?;
        for reset in &p.register_resets {
            let r = block
                .registers
                .iter()
                .find(|r| r.name == reset.register)
                .ok_or_else(|| err("instance reset refers to unknown register"))?;
            if r.array.is_some() {
                return Err(err(
                    "instance register_resets cannot target an indexed register",
                ));
            }
            if ![8, 16, 32].contains(&r.bit_size)
                || reset.reset_value & !(u32::MAX >> (32 - r.bit_size)) != 0
                || reset.reset_source.trim().is_empty()
                || reset
                    .reset_note
                    .as_ref()
                    .is_some_and(|note| note.trim().is_empty())
            {
                return Err(err("invalid instance reset width or evidence"));
            }
        }
    }
    for p in &ir.family.peripherals {
        let mut refs = BTreeSet::new();
        for i in &p.interrupts {
            check_id(&i.signal)?;
            if !ir.family.interrupts.iter().any(|v| v.name == i.interrupt)
                || !refs.insert((&i.signal, &i.interrupt))
            {
                return Err(err("unknown/duplicate peripheral IRQ reference"));
            }
        }
        validate_quirks(&p.quirks)?;
    }
    validate_quirks(&ir.chip.quirks)?;
    unique(ir.chip.remaps.iter().map(|r| r.name.as_str()), "remap")?;
    for r in &ir.chip.remaps {
        if !ir.family.peripherals.iter().any(|p| p.name == r.peripheral) || r.description.is_empty()
        {
            return Err(err("invalid remap metadata"));
        }
    }
    let mut routes = BTreeSet::new();
    for r in &ir.chip.pin_routes {
        check_id(&r.signal)?;
        if !ir.chip.pins.iter().any(|p| p.name == r.pin)
            || !ir.family.peripherals.iter().any(|p| p.name == r.peripheral)
            || r.af.is_some_and(|x| x > 15)
            || r.remap.as_ref().is_some_and(|name| {
                !ir.chip
                    .remaps
                    .iter()
                    .any(|m| m.name == *name && m.peripheral == r.peripheral)
            })
            || !routes.insert((&r.pin, &r.peripheral, &r.signal, r.af, &r.remap))
        {
            return Err(err("invalid/duplicate pin route"));
        }
    }
    let expanded = validate_blocks(ir)?;
    validate_peripheral_relationships(ir, &expanded)?;
    for (index, m) in ir.chip.memory.iter().enumerate() {
        let end = m
            .address
            .checked_add(m.size)
            .ok_or_else(|| err("memory overflow"))?;
        if m.size == 0 || end > (u32::MAX as usize) {
            return Err(err("invalid memory region"));
        }
        for n in &ir.chip.memory[..index] {
            if m.address < n.address + n.size && n.address < end {
                return Err(err("overlapping memory regions"));
            }
        }
    }
    {
        let mut identities = BTreeSet::new();
        for pin in &ir.chip.pins {
            check_id(&pin.port)?;
            if pin.number >= 16 || !identities.insert((&pin.port, pin.number)) {
                return Err(err("invalid/duplicate chip pin"));
            }
            if pin.name != format!("P{}{}", pin.port, pin.number) {
                return Err(err("pin token/port/number mismatch"));
            }
            let p = ir
                .family
                .peripherals
                .iter()
                .find(|p| p.name == format!("GPIO{}", pin.port))
                .ok_or_else(|| err("pin references unknown GPIO port"))?;
            if p.implemented_mask & (1 << pin.number) == 0 {
                return Err(err("pin not implemented on GPIO port"));
            }
        }
    }
    Ok(())
}

/// All byte ranges are expanded before overlap checks, including irregular
/// arrays and nested blocks. Canonical identities retain every block index so
/// aliases cannot accidentally authorize overlap between separate instances.
#[derive(Clone)]
struct PhysicalRange {
    start: usize,
    end: usize,
    canonical: String,
}

fn item_offsets(array: &Option<Array>) -> Result<Vec<usize>> {
    array
        .as_ref()
        .map(Array::offsets)
        .unwrap_or_else(|| Ok(vec![0]))
}

fn register_reset(register: &Register, index: usize) -> Option<u32> {
    if register.array.is_some() {
        register.elements[index].reset_value
    } else {
        register.reset_value
    }
}

fn validate_reset(
    value: Option<u32>,
    source: &Option<String>,
    note: &Option<String>,
    width_mask: u32,
    context: &str,
) -> Result<()> {
    if value.is_some_and(|value| value & !width_mask != 0) {
        return Err(err(format!(
            "{context}: reset value exceeds register width"
        )));
    }
    if value.is_some()
        && source
            .as_ref()
            .is_none_or(|source| source.trim().is_empty())
    {
        return Err(err(format!(
            "{context}: known reset value requires authoritative reset_source"
        )));
    }
    if [source, note]
        .into_iter()
        .flatten()
        .any(|s| s.trim().is_empty())
    {
        return Err(err(format!(
            "{context}: reset evidence and notes must be nonempty when present"
        )));
    }
    Ok(())
}

fn validate_blocks(ir: &Ir) -> Result<BTreeMap<String, Vec<PhysicalRange>>> {
    let mut local = BTreeMap::new();
    for (key, b) in &ir.blocks {
        check_id(&b.name)?;
        check_id(&b.version)?;
        if *key != b.name {
            return Err(err("register block map key/name mismatch"));
        }
        unique(
            b.registers
                .iter()
                .map(|r| r.name.as_str())
                .chain(b.blocks.iter().map(|item| item.name.as_str()))
                .chain(b.constants.iter().map(|c| c.name.as_str())),
            "register/subblock/constant",
        )?;
        let mut methods = BTreeSet::from(["from_ptr".to_owned(), "as_ptr".to_owned()]);
        for name in b
            .registers
            .iter()
            .map(|r| &r.name)
            .chain(b.blocks.iter().map(|item| &item.name))
        {
            if !methods.insert(name.to_ascii_lowercase()) {
                return Err(err(format!(
                    "{}: duplicate register/subblock accessor {name}",
                    b.name
                )));
            }
        }
        let mut original_names = BTreeSet::new();
        let mut ranges = Vec::new();
        // Validate all element counts and widths before resolving alias targets.
        for r in &b.registers {
            if ![8, 16, 32].contains(&r.bit_size) {
                return Err(err("register access width must be 8, 16 or 32 bits"));
            }
            let offsets = item_offsets(&r.array)?;
            if (r.array.is_some() && r.elements.len() != offsets.len())
                || (r.array.is_none() && !r.elements.is_empty())
            {
                return Err(err(format!("{}.{}: indexed registers require one metadata element per offset; scalar elements must be empty", b.name, r.name)));
            }
            let width_mask = u32::MAX >> (32 - r.bit_size);
            validate_reset(
                r.reset_value,
                &r.reset_source,
                &r.reset_note,
                width_mask,
                &format!("{}.{}", b.name, r.name),
            )?;
            if r.array.is_some() {
                unique(
                    r.elements.iter().map(|element| element.name.as_str()),
                    "register element",
                )?;
                for element in &r.elements {
                    if element.description.trim().is_empty()
                        || !original_names.insert(element.name.as_str())
                    {
                        return Err(err("register elements require unique original names and nonempty descriptions"));
                    }
                    validate_reset(
                        element.reset_value,
                        &element.reset_source,
                        &element.reset_note,
                        width_mask,
                        &format!("{}.{}", b.name, element.name),
                    )?;
                }
                if r.reset_value.is_some_and(|value| {
                    r.elements
                        .iter()
                        .any(|element| element.reset_value != Some(value))
                }) {
                    return Err(err(format!("{}.{}: shared array reset requires every element to have the same known reset", b.name, r.name)));
                }
            } else if !original_names.insert(r.name.as_str()) {
                return Err(err("duplicate original register name"));
            }
            if !["ro", "rw", "wo"].contains(&r.access.as_str()) {
                return Err(err("invalid register access mode"));
            }
            if (r.access == "ro" && r.write_behavior != WriteBehavior::Ordinary)
                || (r.access == "wo" && r.read_behavior != ReadBehavior::Ordinary)
            {
                return Err(err("register behavior conflicts with access direction"));
            }
            unique(r.fields.iter().map(|f| f.name.as_str()), "field")?;
            let mut occupied = 0u32;
            for f in &r.fields {
                if f.bit_size == 0
                    || u16::from(f.bit_offset) + u16::from(f.bit_size) > u16::from(r.bit_size)
                    || !["rw", "ro", "wo"].contains(&f.access.as_str())
                    || (r.access != "rw" && f.access != r.access)
                {
                    return Err(err("invalid field width/offset/access"));
                }
                if !["raw", "bool", "enum"].contains(&f.kind.as_str())
                    || (f.kind == "bool" && f.bit_size != 1)
                    || (f.kind != "enum" && !f.values.is_empty())
                    || (f.kind == "enum" && f.values.is_empty())
                {
                    return Err(err("invalid field kind/values"));
                }
                unique(f.values.iter().map(|v| v.name.as_str()), "enum variant")?;
                let mut values = BTreeSet::new();
                for v in &f.values {
                    if v.value > (u32::MAX >> (32 - f.bit_size)) || !values.insert(v.value) {
                        return Err(err("invalid/duplicate enum value"));
                    }
                }
                let (len, stride) = f
                    .array
                    .as_ref()
                    .map(|a| (a.len, a.stride))
                    .unwrap_or((1, 0));
                if len == 0
                    || (f.array.is_some() && stride < f.bit_size)
                    || u16::from(f.bit_offset)
                        + u16::from(len - 1) * u16::from(stride)
                        + u16::from(f.bit_size)
                        > u16::from(r.bit_size)
                {
                    return Err(err("invalid indexed field extent/stride"));
                }
                for n in 0..len {
                    let offset = f.bit_offset + n * stride;
                    let mask = (u32::MAX >> (32 - f.bit_size)) << offset;
                    if occupied & mask != 0 {
                        return Err(err("overlapping register fields"));
                    }
                    occupied |= mask;
                }
            }
        }
        for r in &b.registers {
            let offsets = item_offsets(&r.array)?;
            let target = r
                .alias_of
                .as_ref()
                .map(|name| {
                    check_id(name)?;
                    b.registers
                        .iter()
                        .find(|target| target.name == *name)
                        .ok_or_else(|| err("register alias target does not exist"))
                })
                .transpose()?;
            let target_offsets = target
                .map(|target| item_offsets(&target.array))
                .transpose()?;
            if let Some(target) = target {
                if target.name == r.name
                    || target.alias_of.is_some()
                    || target.access != r.access
                    || (target.array.is_some()
                        && (r.array.is_none()
                            || target_offsets.as_ref().unwrap().len() != offsets.len()))
                {
                    return Err(err("register alias requires a distinct canonical register with the same access and matching array length"));
                }
            }
            let bytes = usize::from(r.bit_size / 8);
            let width_mask = u32::MAX >> (32 - r.bit_size);
            for (index, offset) in offsets.into_iter().enumerate() {
                let start = r
                    .offset
                    .checked_add(offset)
                    .ok_or_else(|| err("register array offset overflow"))?;
                let end = start
                    .checked_add(bytes)
                    .ok_or_else(|| err("register offset overflow"))?;
                if start % bytes != 0 {
                    return Err(err(format!(
                        "{}.{}[{index}]: misaligned register offset",
                        b.name, r.name
                    )));
                }
                let canonical = if let Some(target) = target {
                    let target_index = if target.array.is_some() { index } else { 0 };
                    let target_start = target
                        .offset
                        .checked_add(target_offsets.as_ref().unwrap()[target_index])
                        .ok_or_else(|| err("register alias target offset overflow"))?;
                    let target_end = target_start
                        .checked_add(usize::from(target.bit_size / 8))
                        .ok_or_else(|| err("register alias target offset overflow"))?;
                    if start < target_start || end > target_end {
                        return Err(err(format!("{}.{}[{index}]: register alias must be contained in its canonical element", b.name, r.name)));
                    }
                    let shift = (start - target_start) * 8;
                    let check_reset = |value: Option<u32>, canonical: Option<u32>| -> Result<()> {
                        if let (Some(value), Some(canonical)) = (value, canonical) {
                            if value != (canonical >> shift) & width_mask {
                                return Err(err(format!("{}.{}[{index}]: reset value conflicts with its canonical alias", b.name, r.name)));
                            }
                        }
                        Ok(())
                    };
                    check_reset(
                        register_reset(r, index),
                        register_reset(target, target_index),
                    )?;
                    for p in ir.family.peripherals.iter().filter(|p| p.block == b.name) {
                        let effective = |register: &Register, element: usize| {
                            p.register_resets
                                .iter()
                                .find(|reset| reset.register == register.name)
                                .map(|reset| reset.reset_value)
                                .or_else(|| register_reset(register, element))
                        };
                        check_reset(effective(r, index), effective(target, target_index))
                            .map_err(|error| err(format!("{}: instance {error}", p.name)))?;
                    }
                    format!("{}[{target_index}]", target.name)
                } else {
                    format!("{}[{index}]", r.name)
                };
                if ranges.len() == MAX_EXPANDED_REGISTERS {
                    return Err(err("expanded block register count exceeds limit"));
                }
                ranges.push(PhysicalRange {
                    start,
                    end,
                    canonical,
                });
            }
        }
        for item in &b.blocks {
            check_id(&item.block)?;
            check_id(&item.version)?;
            item_offsets(&item.array)?;
            let target = ir
                .blocks
                .get(&item.block)
                .ok_or_else(|| err("unknown nested block"))?;
            if target.version != item.version {
                return Err(err("nested block version mismatch"));
            }
            if item.description.trim().is_empty() || item.source.trim().is_empty() {
                return Err(err("nested blocks require nonempty description and source"));
            }
        }
        local.insert(b.name.clone(), ranges);
    }
    let mut expanded = BTreeMap::new();
    let mut visiting = BTreeSet::new();
    for name in ir.blocks.keys() {
        expand_block(name, ir, &local, &mut visiting, &mut expanded)?;
    }
    Ok(expanded)
}

fn expand_block(
    name: &str,
    ir: &Ir,
    local: &BTreeMap<String, Vec<PhysicalRange>>,
    visiting: &mut BTreeSet<String>,
    expanded: &mut BTreeMap<String, Vec<PhysicalRange>>,
) -> Result<()> {
    if expanded.contains_key(name) {
        return Ok(());
    }
    if !visiting.insert(name.to_owned()) {
        return Err(err("cyclic nested block reference"));
    }
    // Bound recursive nesting and total expansion separately from individual
    // arrays to reject multiplicative allocation attacks in malformed input.
    if visiting.len() > 256 {
        return Err(err("nested block depth exceeds 256"));
    }
    let mut ranges = local[name].clone();
    for item in &ir.blocks[name].blocks {
        expand_block(&item.block, ir, local, visiting, expanded)?;
        let offsets = item_offsets(&item.array)?;
        let child = &expanded[&item.block];
        let total = child
            .len()
            .checked_mul(offsets.len())
            .and_then(|count| count.checked_add(ranges.len()))
            .filter(|count| *count <= MAX_EXPANDED_REGISTERS)
            .ok_or_else(|| err("expanded block register count exceeds limit"))?;
        ranges.reserve(total - ranges.len());
        for (index, offset) in offsets.into_iter().enumerate() {
            let base = item
                .offset
                .checked_add(offset)
                .ok_or_else(|| err("nested block offset overflow"))?;
            for range in child {
                let start = base
                    .checked_add(range.start)
                    .ok_or_else(|| err("nested register offset overflow"))?;
                let end = base
                    .checked_add(range.end)
                    .ok_or_else(|| err("nested register offset overflow"))?;
                if start % (range.end - range.start) != 0 {
                    return Err(err("misaligned nested register offset"));
                }
                ranges.push(PhysicalRange {
                    start,
                    end,
                    canonical: format!("{}[{index}].{}", item.name, range.canonical),
                });
            }
        }
    }
    if ranges.len() > MAX_EXPANDED_REGISTERS {
        return Err(err("expanded block register count exceeds limit"));
    }
    let mut bytes = BTreeMap::new();
    for range in &ranges {
        for address in range.start..range.end {
            if let Some(previous) = bytes.insert(address, range.canonical.as_str()) {
                if previous != range.canonical {
                    return Err(err(format!("overlapping register offsets require explicit alias_of: {name}.{} overlaps {previous} at byte {address:#x}", range.canonical)));
                }
            }
        }
    }
    ranges.sort_by(|a, b| (a.start, a.end, &a.canonical).cmp(&(b.start, b.end, &b.canonical)));
    visiting.remove(name);
    expanded.insert(name.to_owned(), ranges);
    Ok(())
}

fn validate_quirks(quirks: &[Quirk]) -> Result<()> {
    unique(quirks.iter().map(|q| q.name.as_str()), "quirk")?;
    for q in quirks {
        if q.description.trim().is_empty() || q.source.trim().is_empty() {
            return Err(err("quirks require description and source"));
        }
    }
    Ok(())
}

fn validate_peripheral_relationships(
    ir: &Ir,
    expanded: &BTreeMap<String, Vec<PhysicalRange>>,
) -> Result<()> {
    let peripherals: BTreeMap<_, _> = ir
        .family
        .peripherals
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();
    for p in &ir.family.peripherals {
        let mut visited = BTreeSet::from([p.name.as_str()]);
        let mut next = p.ownership_parent.as_deref();
        while let Some(name) = next {
            if !visited.insert(name) {
                return Err(err("cyclic peripheral ownership_parent"));
            }
            let owner = peripherals
                .get(name)
                .ok_or_else(|| err("unknown peripheral ownership_parent"))?;
            next = owner.ownership_parent.as_deref();
        }
        for reference in [&p.clock_gate, &p.reset].into_iter().flatten() {
            let target = peripherals
                .get(reference.peripheral.as_str())
                .ok_or_else(|| err("unknown clock/reset peripheral"))?;
            let register = ir.blocks[&target.block]
                .registers
                .iter()
                .find(|r| r.name == reference.register)
                .ok_or_else(|| err("unknown clock/reset register"))?;
            let field = register
                .fields
                .iter()
                .find(|f| f.name == reference.field)
                .ok_or_else(|| err("unknown clock/reset field"))?;
            if reference.bit >= 32
                || field.bit_offset != reference.bit
                || field.bit_size != 1
                || field.array.is_some()
                || register.array.is_some()
                || field.kind != "bool"
                || field.access == "ro"
                || register.access == "ro"
            {
                return Err(err(
                    "clock/reset must reference a scalar writable boolean field at the declared bit",
                ));
            }
        }
        if p.block == "gpio"
            && p.clock_gate
                .as_ref()
                .is_some_and(|c| p.clock_bit.is_some_and(|b| b != c.bit))
        {
            return Err(err("GPIO clock_bit disagrees with clock_gate"));
        }
    }
    let ancestor = |child: &Peripheral, name: &str| {
        let mut next = child.ownership_parent.as_deref();
        while let Some(parent) = next {
            if parent == name {
                return true;
            }
            next = peripherals[parent].ownership_parent.as_deref();
        }
        false
    };
    let mut addresses: BTreeMap<usize, Vec<&Peripheral>> = BTreeMap::new();
    for p in &ir.family.peripherals {
        for range in &expanded[&p.block] {
            let address = p
                .address
                .checked_add(range.start)
                .ok_or_else(|| err("peripheral register address overflow"))?;
            let end = p
                .address
                .checked_add(range.end)
                .filter(|end| end.saturating_sub(1) <= u32::MAX as usize)
                .ok_or_else(|| err("peripheral register address overflow"))?;
            for address in address..end {
                for previous in addresses.entry(address).or_default().iter() {
                    if previous.name != p.name
                        && !ancestor(p, &previous.name)
                        && !ancestor(previous, &p.name)
                    {
                        return Err(err(format!("overlapping peripheral registers at {address:#x}: {} and {}; explicit ownership_parent required", previous.name, p.name)));
                    }
                }
                addresses.get_mut(&address).unwrap().push(p);
            }
        }
    }
    Ok(())
}
