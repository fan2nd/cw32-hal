//! Stage 1: layered YAML source data -> normalized, validated JSON IR.
pub use crate::schema::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let source = fs::read_to_string(path).map_err(|e| err(format!("{}: {e}", path.display())))?;
    serde_yaml::from_str(&source).map_err(|e| err(format!("{}: {e}", path.display())))
}

/// Load canonical family IP selections and all explicit nested block references.
fn load_blocks(root: &Path, family: &Family) -> Result<BTreeMap<String, Block>> {
    let mut pending: BTreeSet<_> = family
        .peripherals
        .iter()
        .map(|p| (p.block.clone(), p.version.clone()))
        .collect();
    let mut blocks = BTreeMap::<String, Block>::new();
    while let Some((kind, version)) = pending.pop_first() {
        check_id(&kind)?;
        check_id(&version)?;
        if let Some(block) = blocks.get(&kind) {
            if block.version != version {
                return Err(err(format!(
                    "conflicting register versions for {kind}: {}/{version}; multiple versions of one kind within a single chip are unsupported",
                    block.version
                )));
            }
            continue;
        }
        let path = root
            .join("registers")
            .join(&kind)
            .join(format!("{version}.yaml"));
        let block: Block = read(&path)?;
        if block.name != kind || block.version != version {
            return Err(err(format!(
                "register block reference/name/version mismatch in {}: expected {kind}/{version}, found {}/{}",
                path.display(),
                block.name,
                block.version
            )));
        }
        pending.extend(
            block
                .blocks
                .iter()
                .map(|item| (item.block.clone(), item.version.clone())),
        );
        blocks.insert(kind, block);
    }
    Ok(blocks)
}

/// Resolve register versions and die-level pins; rejects unknown YAML keys.
pub fn load(root: &Path, chip: &str) -> Result<Ir> {
    check_id(chip)?;
    let selected_chip = chip.to_owned();
    let mut chip: Chip = read(&root.join("chips").join(format!("{chip}.yaml")))?;
    if !chip.name.eq_ignore_ascii_case(&selected_chip) {
        return Err(err("chip filename/name mismatch"));
    }
    check_id(&chip.family)?;
    let mut family: Family = read(&root.join("families").join(format!("{}.yaml", chip.family)))?;
    if family.name != chip.family {
        return Err(err("family reference/name mismatch"));
    }
    let mut blocks = load_blocks(root, &family)?;
    chip.pins.sort_by(|a, b| a.name.cmp(&b.name));
    chip.memory.sort_by(|a, b| a.name.cmp(&b.name));
    chip.pin_routes.sort_by(|a, b| {
        (&a.pin, &a.peripheral, &a.signal, a.af, &a.remap).cmp(&(
            &b.pin,
            &b.peripheral,
            &b.signal,
            b.af,
            &b.remap,
        ))
    });
    chip.remaps.sort_by(|a, b| a.name.cmp(&b.name));
    chip.quirks.sort_by(|a, b| a.name.cmp(&b.name));
    for p in &mut family.peripherals {
        if let Some(dma) = &mut p.dma {
            dma.channels.sort_by_key(|channel| channel.index);
            dma.requests.sort_by_key(|request| request.selector);
        }
        p.reset_effects
            .sort_by(|a, b| a.peripheral.cmp(&b.peripheral));
        p.register_resets
            .sort_by(|a, b| a.register.cmp(&b.register));
        p.interrupts
            .sort_by(|a, b| (&a.signal, &a.interrupt).cmp(&(&b.signal, &b.interrupt)));
        p.quirks.sort_by(|a, b| a.name.cmp(&b.name));
    }
    family.peripherals.sort_by(|a, b| a.name.cmp(&b.name));
    family.interrupts.sort_by_key(|i| i.number);
    family.constants.sort_by(|a, b| a.name.cmp(&b.name));
    for b in blocks.values_mut() {
        b.blocks
            .sort_by(|a, b| (a.offset, &a.name).cmp(&(b.offset, &b.name)));
        // Array offsets and element metadata deliberately retain source index
        // order; sorting either independently would change the hardware API.
        b.registers
            .sort_by(|a, b| (a.offset, &a.name).cmp(&(b.offset, &b.name)));
        for r in &mut b.registers {
            for f in &mut r.fields {
                f.values.sort_by_key(|v| v.value);
                for element in &mut f.elements {
                    element.values.sort_by_key(|v| v.value);
                }
            }
            r.fields.sort_by_key(|f| f.bit_offset);
        }
        b.constants.sort_by(|a, b| a.name.cmp(&b.name));
    }
    let ir = Ir {
        schema_version: SCHEMA_VERSION,
        chip,
        family,
        blocks,
    };
    validate(&ir)?;
    Ok(ir)
}

/// Emit separate normalized chip and reusable register JSON artifacts.
pub fn write_json(ir: &Ir, out: &Path) -> Result<std::path::PathBuf> {
    validate(ir)?;
    fs::create_dir_all(out.join("chips"))?;
    fs::create_dir_all(out.join("registers"))?;
    let document = ChipDocument {
        schema_version: ir.schema_version,
        chip: ir.chip.clone(),
        family: ir.family.clone(),
        registers: ir
            .blocks
            .values()
            .map(|b| RegisterRef {
                kind: b.name.clone(),
                version: b.version.clone(),
            })
            .collect(),
    };
    for b in ir.blocks.values() {
        let register = RegisterDocument {
            schema_version: ir.schema_version,
            block: b.clone(),
        };
        let path = out
            .join("registers")
            .join(format!("{}_{}.json", b.name, b.version));
        let content = format!("{}\n", serde_json::to_string_pretty(&register)?);
        if path.exists() && fs::read_to_string(&path)? != content {
            return Err(err(
                "refusing to overwrite a different kind/version definition; use a fresh output directory or split IP version",
            ));
        }
        fs::write(path, content)?;
    }
    let path = out
        .join("chips")
        .join(format!("{}.json", ir.chip.name.to_ascii_lowercase()));
    fs::write(
        &path,
        format!("{}\n", serde_json::to_string_pretty(&document)?),
    )?;
    Ok(path)
}
pub fn generate(root: &Path, chip: &str, out: &Path) -> Result<std::path::PathBuf> {
    write_json(&load(root, chip)?, out)
}

/// Generate a publication set, checking that a reused kind/version means one
/// register definition across every selected chip. No last-writer-wins merges.
pub fn generate_many(root: &Path, chips: &[&str], out: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut names = std::collections::BTreeSet::new();
    let mut definitions = BTreeMap::new();
    let mut chip_paths = std::collections::BTreeSet::new();
    let mut register_paths = BTreeMap::new();
    let mut irs = Vec::new();
    for chip in chips {
        if !names.insert(*chip) {
            return Err(err("duplicate chip in publication set"));
        }
        let ir = load(root, chip)?;
        if !chip_paths.insert(ir.chip.name.to_ascii_lowercase()) {
            return Err(err(
                "chip names collide after output filename normalization",
            ));
        }
        for b in ir.blocks.values() {
            let key = (b.name.clone(), b.version.clone());
            let path = format!("{}_{}", b.name, b.version);
            if register_paths
                .get(&path)
                .is_some_and(|previous| previous != &key)
            {
                return Err(err(format!("register artifact filename collision: {path}")));
            }
            register_paths.insert(path, key.clone());
            let bytes = serde_json::to_vec(b)?;
            if definitions.get(&key).is_some_and(|old| *old != bytes) {
                return Err(err(format!(
                    "kind/version collision: {}/{}; split a new IP version for layout differences",
                    key.0, key.1
                )));
            }
            definitions.insert(key, bytes);
        }
        irs.push(ir);
    }
    irs.sort_by(|a, b| a.chip.name.cmp(&b.chip.name));
    irs.iter().map(|ir| write_json(ir, out)).collect()
}
