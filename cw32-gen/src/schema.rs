//! Shared, versioned normalized IR and validation. No YAML loader or PAC renderer.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// Version 4 adds audited per-register reset values and their evidence. Older
/// normalized JSON must be regenerated; unknown legacy fields are rejected.
/// An omitted reset value is unknown, never an implicit zero. An omitted source
/// YAML `bit_size` still means a 32-bit bus transaction.
pub const SCHEMA_VERSION: u32 = 4;

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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    pub bit_offset: u8,
    pub bit_size: u8,
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
model!(Block { name: String, version: String, registers: Vec<Register>, constants: Vec<Constant> });
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
    for b in ir.blocks.values() {
        check_id(&b.name)?;
        check_id(&b.version)?;
        unique(
            b.registers
                .iter()
                .map(|x| x.name.as_str())
                .chain(b.constants.iter().map(|x| x.name.as_str())),
            "register/constant",
        )?;
        let mut offsets: BTreeMap<usize, &str> = BTreeMap::new();
        for r in &b.registers {
            if ![8, 16, 32].contains(&r.bit_size) {
                return Err(err("register access width must be 8, 16 or 32 bits"));
            }
            let width_mask = u32::MAX >> (32 - r.bit_size);
            if r.reset_value.is_some_and(|value| value & !width_mask != 0) {
                return Err(err(format!(
                    "{}.{}: reset value exceeds register width",
                    b.name, r.name
                )));
            }
            if r.reset_value.is_some()
                && r.reset_source
                    .as_ref()
                    .is_none_or(|source| source.trim().is_empty())
            {
                return Err(err(format!(
                    "{}.{}: known reset value requires authoritative reset_source",
                    b.name, r.name
                )));
            }
            if [&r.reset_source, &r.reset_note]
                .into_iter()
                .flatten()
                .any(|s| s.trim().is_empty())
            {
                return Err(err(
                    "reset evidence and notes must be nonempty when present",
                ));
            }
            let bytes = usize::from(r.bit_size / 8);
            let end = r
                .offset
                .checked_add(bytes)
                .ok_or_else(|| err("register offset overflow"))?;
            if r.offset % bytes != 0 || !["ro", "rw", "wo"].contains(&r.access.as_str()) {
                return Err(err("invalid register offset or access mode"));
            }
            if (r.access == "ro" && r.write_behavior != WriteBehavior::Ordinary)
                || (r.access == "wo" && r.read_behavior != ReadBehavior::Ordinary)
            {
                return Err(err("register behavior conflicts with access direction"));
            }
            let canonical = if let Some(alias) = &r.alias_of {
                check_id(alias)?;
                let target = b
                    .registers
                    .iter()
                    .find(|v| v.name == *alias)
                    .ok_or_else(|| err("register alias target does not exist"))?;
                let target_end = target
                    .offset
                    .checked_add(usize::from(target.bit_size / 8))
                    .ok_or_else(|| err("register alias target offset overflow"))?;
                if target.name == r.name
                    || target.alias_of.is_some()
                    || r.offset < target.offset
                    || end > target_end
                    || target.access != r.access
                {
                    return Err(err(format!("{}.{} at {:#x}: register alias must reference a distinct canonical register containing its full byte range with the same access", b.name, r.name, r.offset)));
                }
                if let (Some(value), Some(target_value)) = (r.reset_value, target.reset_value) {
                    let shift = (r.offset - target.offset) * 8;
                    if value != (target_value >> shift) & width_mask {
                        return Err(err(format!(
                            "{}.{}: reset value conflicts with its canonical alias",
                            b.name, r.name
                        )));
                    }
                }
                for p in ir.family.peripherals.iter().filter(|p| p.block == b.name) {
                    let effective = |register: &Register| {
                        p.register_resets
                            .iter()
                            .find(|reset| reset.register == register.name)
                            .map(|reset| reset.reset_value)
                            .or(register.reset_value)
                    };
                    if let (Some(value), Some(target_value)) = (effective(r), effective(target)) {
                        let shift = (r.offset - target.offset) * 8;
                        if value != (target_value >> shift) & width_mask {
                            return Err(err(format!(
                                "{}.{}: instance reset conflicts with its canonical alias",
                                p.name, r.name
                            )));
                        }
                    }
                }
                alias.as_str()
            } else {
                r.name.as_str()
            };
            // Track byte extents, not just starting offsets: a u16 at +2 may
            // not silently overlap a u32 at +0. Explicit aliases may select a
            // narrower part of the canonical register (e.g. GPIO ODRHIGHBYTE),
            // or use different same-address widths (e.g. CRC DR8/DR16/DR32).
            for offset in r.offset..end {
                if let Some(previous) = offsets.insert(offset, canonical) {
                    if previous != canonical {
                        return Err(err(format!("overlapping register offsets require explicit alias_of: {}.{} at {:#x} overlaps canonical {} at byte {offset:#x}", b.name, r.name, r.offset, previous)));
                    }
                }
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
                let mask = (u32::MAX >> (32 - f.bit_size)) << f.bit_offset;
                if occupied & mask != 0 {
                    return Err(err("overlapping register fields"));
                }
                occupied |= mask;
            }
        }
    }
    validate_peripheral_relationships(ir)?;
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

fn validate_quirks(quirks: &[Quirk]) -> Result<()> {
    unique(quirks.iter().map(|q| q.name.as_str()), "quirk")?;
    for q in quirks {
        if q.description.trim().is_empty() || q.source.trim().is_empty() {
            return Err(err("quirks require description and source"));
        }
    }
    Ok(())
}

fn validate_peripheral_relationships(ir: &Ir) -> Result<()> {
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
                || field.access == "ro"
                || register.access == "ro"
            {
                return Err(err(
                    "clock/reset must reference a writable one-bit field at the declared bit",
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
        for register in &ir.blocks[&p.block].registers {
            let address = p
                .address
                .checked_add(register.offset)
                .filter(|a| *a <= u32::MAX as usize - usize::from(register.bit_size / 8 - 1))
                .ok_or_else(|| err("peripheral register address overflow"))?;
            for byte in 0..usize::from(register.bit_size / 8) {
                let address = address + byte;
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
