//! Declarative source corrections, followed by peripheral kind/version aliases.
//!
//! Both `perimap.yaml` and each `fixes/*.yaml` file are YAML lists. Selectors are
//! exact, case-sensitive names, never regular expressions or wildcards. Rules
//! for other chips are parsed and checked, but do not apply to this chip.
//! A rule for this chip that does not find its exact target is an error.
//!
//! Fixes use the original block/version names, before alias normalization. They
//! patch an existing register or field; adding hardware requires editing the
//! source model. `mode: select` resolves a canonical register file before source
//! loading; `mode: alias` (the default) only relabels a source model. Every rule
//! requires an evidence source and a nonempty reason. Use `load_blocks` before
//! `apply` so selection happens before corrections and normalization.

use crate::schema::{check_id, err, Block, Family, ReadBehavior, Result, WriteBehavior};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    source: String,
    reason: String,
}

impl Provenance {
    fn validate(&self) -> Result<()> {
        if self.source.trim().is_empty() || self.reason.trim().is_empty() {
            return Err(err("provenance source and reason must be nonempty"));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PeripheralTarget {
    chip: String,
    instance: String,
    block: Option<String>,
    version: Option<String>,
    vendor_ip: Option<String>,
    vendor_version: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Mode {
    #[default]
    Alias,
    Select,
}

/// This IR deliberately supports exactly one register block per peripheral.
#[derive(Debug, Deserialize)]
enum RegisterBlock {
    RegisterBlock,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NormalizedBlock {
    kind: String,
    version: String,
    #[serde(rename = "block")]
    _block: Option<RegisterBlock>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AliasRule {
    #[serde(default)]
    mode: Mode,
    target: PeripheralTarget,
    normalize: NormalizedBlock,
    provenance: Provenance,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixTarget {
    chip: String,
    block: String,
    version: String,
    register: String,
    field: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Access {
    Ro,
    Rw,
    Wo,
}

impl Access {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ro => "ro",
            Self::Rw => "rw",
            Self::Wo => "wo",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Patch {
    Register {
        offset: Option<usize>,
        access: Option<Access>,
        description: Option<String>,
        alias_of: Option<String>,
        write_behavior: Option<WriteBehavior>,
        read_behavior: Option<ReadBehavior>,
    },
    Field {
        bit_offset: Option<u8>,
        bit_size: Option<u8>,
        access: Option<Access>,
        description: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixRule {
    target: FixTarget,
    patch: Patch,
    provenance: Provenance,
}

struct Located<T> {
    origin: String,
    rule: T,
}

fn read_rules<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<Located<T>>> {
    let text = fs::read_to_string(path).map_err(|e| err(format!("{}: {e}", path.display())))?;
    let rules: Vec<T> =
        serde_yaml::from_str(&text).map_err(|e| err(format!("{}: {e}", path.display())))?;
    Ok(rules
        .into_iter()
        .enumerate()
        .map(|(i, rule)| Located {
            origin: format!("{} rule {}", path.display(), i + 1),
            rule,
        })
        .collect())
}

fn ids(values: &[&str]) -> Result<()> {
    for value in values {
        check_id(value)?;
    }
    Ok(())
}

fn validate_alias(rule: &AliasRule) -> Result<()> {
    let t = &rule.target;
    ids(&[
        &t.chip,
        &t.instance,
        &rule.normalize.kind,
        &rule.normalize.version,
    ])?;
    for selector in [&t.block, &t.version].into_iter().flatten() {
        check_id(selector)?;
    }
    for hint in [&t.vendor_ip, &t.vendor_version].into_iter().flatten() {
        if hint.trim().is_empty() {
            return Err(err("vendor IP/version selectors must be nonempty"));
        }
    }
    rule.provenance.validate()
}

fn load_perimap(root: &Path) -> Result<Vec<Located<AliasRule>>> {
    let path = root.join("perimap.yaml");
    let rules: Vec<Located<AliasRule>> = if path.try_exists()? {
        read_rules(&path)?
    } else {
        Vec::new()
    };
    for located in &rules {
        validate_alias(&located.rule).map_err(|e| err(format!("{}: {e}", located.origin)))?;
    }
    Ok(rules)
}

fn select_rules<'a>(
    rules: &'a [Located<AliasRule>],
    chip: &str,
    family: &Family,
) -> Result<BTreeMap<String, &'a Located<AliasRule>>> {
    let mut selected = BTreeMap::new();
    for located in rules {
        let rule = &located.rule;
        if rule.target.chip != chip {
            continue;
        }
        let t = &rule.target;
        let p = family
            .peripherals
            .iter()
            .find(|p| p.name == t.instance)
            .ok_or_else(|| {
                err(format!(
                    "{}: missing alias instance {}",
                    located.origin, t.instance
                ))
            })?;
        if t.block.as_ref().is_some_and(|block| p.block != *block)
            || t.version
                .as_ref()
                .is_some_and(|version| p.version != *version)
            || t.vendor_ip
                .as_ref()
                .is_some_and(|ip| p.vendor_ip.as_ref() != Some(ip))
            || t.vendor_version
                .as_ref()
                .is_some_and(|version| p.vendor_version.as_ref() != Some(version))
        {
            return Err(err(format!(
                "{}: alias source mismatch for {}: requested block={:?}, version={:?}, vendor_ip={:?}, vendor_version={:?}; found {}/{}, vendor_ip={:?}, vendor_version={:?}",
                located.origin, p.name, t.block, t.version, t.vendor_ip, t.vendor_version,
                p.block, p.version, p.vendor_ip, p.vendor_version
            )));
        }
        if let Some(previous) = selected.insert(p.name.clone(), located) {
            return Err(err(format!(
                "{}: conflicting aliases for {}; already mapped by {}",
                located.origin, p.name, previous.origin
            )));
        }
    }
    Ok(selected)
}

/// Resolve perimap selection before reading register files.
///
/// A select rule reads `registers/<normalize.kind>/<normalize.version>.yaml`,
/// regardless of whether the raw peripheral block/version has a source file.
/// Alias rules and unmapped peripherals retain the ordinary raw-file lookup.
/// After canonical identity verification the model receives its raw identity
/// temporarily, so existing source-targeted fixes can run before normalization.
///
/// This intermediate map has one model per raw block. Incompatible selections
/// sharing that raw key fail explicitly; give distinct source models distinct
/// raw keys rather than relying on instance iteration order.
pub fn load_blocks(root: &Path, chip: &str, family: &Family) -> Result<BTreeMap<String, Block>> {
    let rules = load_perimap(root)?;
    let selected = select_rules(&rules, chip, family)?;
    let mut blocks = BTreeMap::<String, Block>::new();
    for peripheral in &family.peripherals {
        ids(&[&peripheral.block, &peripheral.version])?;
        let selected_rule = selected.get(&peripheral.name);
        let (kind, version) = match selected_rule {
            Some(located) if located.rule.mode == Mode::Select => (
                &located.rule.normalize.kind,
                &located.rule.normalize.version,
            ),
            _ => (&peripheral.block, &peripheral.version),
        };
        let path = root
            .join("registers")
            .join(kind)
            .join(format!("{version}.yaml"));
        let context = match selected_rule {
            Some(located) => format!("{} (instance {})", located.origin, peripheral.name),
            None => format!("instance {}", peripheral.name),
        };
        let source = fs::read_to_string(&path).map_err(|e| {
            err(format!(
                "{context}: cannot load register model {}: {e}",
                path.display()
            ))
        })?;
        let mut block: Block = serde_yaml::from_str(&source)
            .map_err(|e| err(format!("{context}: {}: {e}", path.display())))?;
        if block.name != *kind || block.version != *version {
            return Err(err(format!(
                "{context}: register block reference/name/version mismatch in {}: expected {kind}/{version}, found {}/{}",
                path.display(), block.name, block.version
            )));
        }
        block.name.clone_from(&peripheral.block);
        block.version.clone_from(&peripheral.version);
        if let Some(previous) = blocks.get(&peripheral.block) {
            if serde_json::to_value(previous)? != serde_json::to_value(&block)? {
                return Err(err(format!(
                    "{context}: conflicting selected/source models for raw block {}; use distinct raw block identities",
                    peripheral.block
                )));
            }
        } else {
            blocks.insert(peripheral.block.clone(), block);
        }
    }
    Ok(blocks)
}

fn validate_fix(rule: &FixRule) -> Result<()> {
    let t = &rule.target;
    ids(&[&t.chip, &t.block, &t.version, &t.register])?;
    if let Some(field) = &t.field {
        check_id(field)?;
    }
    rule.provenance.validate()?;
    let empty = match &rule.patch {
        Patch::Register {
            offset,
            access,
            description,
            alias_of,
            write_behavior,
            read_behavior,
        } => {
            if t.field.is_some() {
                return Err(err("register patch target must not include field"));
            }
            if let Some(name) = alias_of {
                check_id(name)?;
            }
            offset.is_none()
                && access.is_none()
                && description.is_none()
                && alias_of.is_none()
                && write_behavior.is_none()
                && read_behavior.is_none()
        }
        Patch::Field {
            bit_offset,
            bit_size,
            access,
            description,
        } => {
            if t.field.is_none() {
                return Err(err("field patch target must include field"));
            }
            bit_offset.is_none() && bit_size.is_none() && access.is_none() && description.is_none()
        }
    };
    if empty {
        return Err(err("empty fix patch"));
    }
    Ok(())
}

// Disjoint edits may compose; writing the same property twice is an error even
// if the proposed values happen to agree. No filename-order override semantics.
fn claim(
    writes: &mut BTreeMap<String, String>,
    target: &FixTarget,
    property: &str,
    origin: &str,
) -> Result<()> {
    let key = format!(
        "{}/{}/{}/{}/{}",
        target.block,
        target.version,
        target.register,
        target.field.as_deref().unwrap_or(""),
        property
    );
    if let Some(previous) = writes.insert(key.clone(), origin.to_owned()) {
        return Err(err(format!(
            "conflicting fixes for {key}; already patched by {previous}"
        )));
    }
    Ok(())
}

fn apply_fix(
    rule: &FixRule,
    origin: &str,
    blocks: &mut BTreeMap<String, Block>,
    writes: &mut BTreeMap<String, String>,
) -> Result<()> {
    let t = &rule.target;
    let block = blocks
        .get_mut(&t.block)
        .ok_or_else(|| err(format!("missing fix block {}", t.block)))?;
    if block.version != t.version {
        return Err(err(format!(
            "fix version mismatch for {}: expected {}, found {}",
            t.block, t.version, block.version
        )));
    }
    let register = block
        .registers
        .iter_mut()
        .find(|r| r.name == t.register)
        .ok_or_else(|| err(format!("missing fix register {}.{}", t.block, t.register)))?;
    macro_rules! set {
        ($dst:expr, $src:expr, $name:literal) => {
            if let Some(value) = $src {
                claim(writes, t, $name, origin)?;
                $dst = value.clone();
            }
        };
    }
    match &rule.patch {
        Patch::Register {
            offset,
            access,
            description,
            alias_of,
            write_behavior,
            read_behavior,
        } => {
            set!(register.offset, offset, "offset");
            set!(register.description, description, "description");
            set!(register.write_behavior, write_behavior, "write_behavior");
            set!(register.read_behavior, read_behavior, "read_behavior");
            if let Some(value) = alias_of {
                claim(writes, t, "alias_of", origin)?;
                register.alias_of = Some(value.clone());
            }
            if let Some(value) = access {
                claim(writes, t, "access", origin)?;
                register.access = value.as_str().to_owned();
            }
        }
        Patch::Field {
            bit_offset,
            bit_size,
            access,
            description,
        } => {
            let name = t
                .field
                .as_deref()
                .ok_or_else(|| err("missing field target"))?;
            let field = register
                .fields
                .iter_mut()
                .find(|f| f.name == name)
                .ok_or_else(|| {
                    err(format!(
                        "missing fix field {}.{}.{name}",
                        t.block, t.register
                    ))
                })?;
            set!(field.bit_offset, bit_offset, "bit_offset");
            set!(field.bit_size, bit_size, "bit_size");
            set!(field.description, description, "description");
            if let Some(value) = access {
                claim(writes, t, "access", origin)?;
                field.access = value.as_str().to_owned();
            }
        }
    }
    Ok(())
}

fn apply_aliases(
    rules: &[Located<AliasRule>],
    chip: &str,
    family: &mut Family,
    blocks: &mut BTreeMap<String, Block>,
) -> Result<()> {
    let selected = select_rules(rules, chip, family)?;
    if selected.is_empty() {
        return Ok(());
    }

    let mut normalized = BTreeMap::<String, Block>::new();
    for peripheral in &mut family.peripherals {
        let mut block = blocks
            .get(&peripheral.block)
            .ok_or_else(|| err(format!("missing source block {}", peripheral.block)))?
            .clone();
        if block.name != peripheral.block || block.version != peripheral.version {
            return Err(err(format!(
                "source block identity mismatch for {}",
                peripheral.name
            )));
        }
        if let Some(located) = selected.get(&peripheral.name) {
            peripheral.block.clone_from(&located.rule.normalize.kind);
            peripheral
                .version
                .clone_from(&located.rule.normalize.version);
        }
        block.name.clone_from(&peripheral.block);
        block.version.clone_from(&peripheral.version);
        if let Some(previous) = normalized.get(&peripheral.block) {
            if serde_json::to_value(previous)? != serde_json::to_value(&block)? {
                return Err(err(format!(
                    "conflicting normalized register models for kind {} (instance {}); multiple versions of one kind within a single chip are unsupported (distinct versions across chips are supported)",
                    peripheral.block, peripheral.name
                )));
            }
        } else {
            normalized.insert(peripheral.block.clone(), block);
        }
    }
    *blocks = normalized;
    Ok(())
}

/// Apply optional YAML extensions atomically, before sorting and IR validation.
///
/// Files are processed lexically; unknown YAML keys, duplicate writes, ambiguous
/// aliases and missing targets fail with a file/rule diagnostic. Hardware validity
/// (overlaps, widths, access compatibility, etc.) is checked by schema validation
/// after this function returns. On any error the inputs are left unchanged.
pub fn apply(
    root: &Path,
    chip_name: &str,
    family: &mut Family,
    blocks: &mut BTreeMap<String, Block>,
) -> Result<()> {
    let aliases = load_perimap(root)?;
    let mut fixes: Vec<Located<FixRule>> = Vec::new();
    let fixes_dir = root.join("fixes");
    if fixes_dir.try_exists()? {
        let mut paths = Vec::new();
        for entry in fs::read_dir(&fixes_dir)? {
            let path = entry?.path();
            if matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("yaml" | "yml")
            ) {
                paths.push(path);
            }
        }
        paths.sort();
        for path in paths {
            fixes.extend(read_rules(&path)?);
        }
    }
    for located in &fixes {
        validate_fix(&located.rule).map_err(|e| err(format!("{}: {e}", located.origin)))?;
    }

    let mut patched = blocks.clone();
    let mut mapped = family.clone();
    let mut writes = BTreeMap::new();
    for located in &fixes {
        if located.rule.target.chip == chip_name {
            apply_fix(&located.rule, &located.origin, &mut patched, &mut writes)
                .map_err(|e| err(format!("{}: {e}", located.origin)))?;
        }
    }
    apply_aliases(&aliases, chip_name, &mut mapped, &mut patched)?;
    *family = mapped;
    *blocks = patched;
    Ok(())
}
