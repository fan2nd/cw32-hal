//! Vendor SVD -> chiptool register IR + the stm32-data-shaped chip JSON.
//!
//! Register translation is performed by pinned upstream chiptool. This stage
//! does not generate Rust or silently invent missing clock/DMA/package data.
mod accelerators;
mod adc_sequence;
pub mod af;
mod classic_adc_scan;
mod classic_timer_input;
mod classic_atim_input;
pub mod clock;
mod comparator;
mod crypto;
mod dac_opa;
pub mod dma;
pub mod electrical;
mod field_access;
pub mod gpio_interrupt;
pub mod interrupts;
mod lcd;
mod lsi;
pub mod lvd_ir;
pub mod pinouts;
mod ram;
mod register_write;
mod rtc;
pub mod spi;
mod svd_access;
mod triggers;
mod vref;

use anyhow::{Context, Result, ensure};
use chiptool::{ir, svd2ir::NamespaceMode};
use cw32_data_serde::{Chip, chip};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

/// Read an authored input manifest. JSON remains a generated-output format.
pub fn load_input(path: &Path) -> Result<Input> {
    read_yaml(path)
}

fn read_yaml<T: serde::de::DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let bytes = fs::read(path).with_context(|| format!("read authored YAML {}", path.display()))?;
    parse_yaml(path, &bytes)
}

fn parse_yaml<T: serde::de::DeserializeOwned>(path: &Path, bytes: &[u8]) -> Result<T> {
    ensure!(
        path.extension().and_then(|extension| extension.to_str()) == Some("yaml"),
        "authored metadata must use a .yaml path: {}",
        path.display()
    );
    serde_yaml::from_slice(bytes).with_context(|| format!("parse authored YAML {}", path.display()))
}

// Canonical source IDs annotate existing SHA-qualified citations. Comparisons
// against reviewed evidence retain every established field; the provenance
// verifier separately checks each added source_ref against its retained digest.
fn without_source_aliases(value: &serde_json::Value) -> serde_json::Value {
    let mut value = value.clone();
    fn strip(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(fields) => {
                if fields
                    .get("sha256")
                    .is_some_and(serde_json::Value::is_string)
                    && fields
                        .get("source_ref")
                        .is_some_and(serde_json::Value::is_string)
                {
                    fields.remove("source_ref");
                }
                for value in fields.values_mut() {
                    strip(value);
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    strip(value);
                }
            }
            _ => {}
        }
    }
    strip(&mut value);
    value
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub path: String,
    pub sha256: String,
    pub url: String,
}
#[derive(Debug, Deserialize)]
pub struct Input {
    pub schema_version: u32,
    #[serde(default)]
    pub quarantine: Option<String>,
    #[serde(default)]
    pub parts_catalog: Option<String>,
    #[serde(default)]
    pub af_metadata: Option<String>,
    #[serde(default)]
    pub analog_metadata: Option<String>,
    #[serde(default)]
    pub comparator_metadata: Option<String>,
    #[serde(default)]
    pub pwm_metadata: Option<String>,
    #[serde(default)]
    pub atim_pwm_metadata: Option<String>,
    #[serde(default)]
    pub atim_complementary_metadata: Option<String>,
    #[serde(default)]
    pub halltim_metadata: Option<String>,
    #[serde(default)]
    pub pinout_metadata: Option<String>,
    #[serde(default)]
    pub alias_pin_policy: Option<String>,
    #[serde(default)]
    pub dma_metadata: Option<String>,
    #[serde(default)]
    pub clock_metadata: Option<String>,
    #[serde(default)]
    pub spi_metadata: Option<String>,
    #[serde(default)]
    pub trigger_metadata: Option<String>,
    #[serde(default)]
    pub gpio_interrupt_metadata: Option<String>,
    #[serde(default)]
    pub electrical_metadata: Option<String>,
    #[serde(default)]
    pub adc_sequence_metadata: Option<String>,
    #[serde(default)]
    pub classic_adc_scan_metadata: Option<String>,
    #[serde(default)]
    pub shared_map_evidence: Option<String>,
    pub source: Source,
    pub expected_svd_name: String,
    pub core: String,
    pub die: String,
    pub chip_prefix: String,
    #[serde(default)]
    pub firmware_targets: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub gpio_dir_pin_fields: bool,
    #[serde(default)]
    pub register_aliases: Vec<RegisterAlias>,
    #[serde(default)]
    pub supplemental_interrupts: Vec<SupplementalInterrupt>,
    #[serde(default)]
    pub interrupt_association_overrides: Vec<interrupts::AssociationOverride>,
    #[serde(default)]
    pub register_overrides: Vec<RegisterOverride>,
    #[serde(default)]
    pub field_removals: Vec<FieldRemoval>,
    #[serde(default)]
    pub register_removals: Vec<RegisterRemoval>,
    #[serde(default)]
    pub field_width_overrides: Vec<FieldWidthOverride>,
    #[serde(default)]
    pub source_notes: Vec<String>,
    pub family: String,
    pub line: String,
    pub register_version: String,
    #[serde(default)]
    pub register_versions: BTreeMap<String, String>,
    pub nvic_priority_bits: Option<u8>,
    #[serde(default)]
    pub nvic_priority_bits_evidence: String,
    pub chips: Vec<ChipInput>,
}
#[derive(Debug, Deserialize)]
pub struct RegisterRemoval {
    pub block: String,
    pub register: String,
    pub expected_byte_offset: u32,
    pub expected_fieldset: String,
    pub evidence: String,
}
#[derive(Debug, Deserialize)]
pub struct FieldRemoval {
    /// Optional explicit block scope; required for shared fieldset names.
    #[serde(default)]
    pub block: Option<String>,
    pub fieldset: String,
    pub field: String,
    pub expected_bit_offset: u32,
    pub expected_bit_size: u32,
    pub evidence: String,
}
/// Reviewed field-width erratum; the register transaction width is unchanged.
#[derive(Debug, Deserialize)]
pub struct FieldWidthOverride {
    pub block: String,
    pub fieldset: String,
    pub field: String,
    pub expected_bit_offset: u32,
    pub expected_bit_size: u32,
    pub bit_size: u32,
    pub evidence: String,
}
#[derive(Debug, Deserialize)]
pub struct RegisterOverride {
    pub block: String,
    pub register: String,
    pub expected_access: ir::Access,
    pub access: ir::Access,
    pub evidence: String,
}
#[derive(Debug, Deserialize)]
pub struct SupplementalInterrupt {
    pub name: String,
    pub number: u32,
    pub evidence: String,
}
#[derive(Debug, Deserialize)]
pub struct RegisterAlias {
    pub block: String,
    pub register: String,
    pub alias: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct ChipInput {
    pub name: String,
    #[serde(default)]
    pub memory: Vec<Vec<chip::Memory>>,
    #[serde(default)]
    pub packages: Vec<chip::Package>,
    #[serde(default)]
    pub docs: Vec<chip::Doc>,
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub profile: String,
    pub source_device: String,
    pub shared_map_evidence: Option<String>,
    pub source_sha256: String,
    pub source_url: String,
    pub peripherals: usize,
    pub register_blocks: usize,
    pub registers: usize,
    pub fieldsets: usize,
    pub fields: usize,
    pub interrupts: usize,
    pub die_pins: usize,
    pub pinouts: Option<pinouts::PinoutReport>,
    pub dma: Option<dma::DmaReport>,
    pub clocks: Option<clock::ClockReport>,
    pub applied_errata: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PartsCatalog {
    schema_version: u32,
    sources: BTreeMap<String, serde_json::Value>,
    parts: Vec<CatalogPart>,
}
#[derive(Debug, Deserialize)]
struct CatalogPart {
    name: String,
    feature: String,
    family: String,
    package: String,
    memory: Vec<chip::Memory>,
    memory_status: String,
    source_ids: Vec<String>,
}

/// Exact package and memory data has one authored source of truth, separate
/// from family register-map aliases and their source import configuration.
fn expand_chip_inputs(root: &Path, input: &Input) -> Result<Vec<ChipInput>> {
    let mut chips = input.chips.clone();
    if let Some(path) = &input.parts_catalog {
        let catalog: PartsCatalog = crate::read_yaml(root.join(path))?;
        ensure!(catalog.schema_version == 1, "unsupported parts catalog");
        for part in catalog
            .parts
            .iter()
            .filter(|part| part.family == input.line)
        {
            ensure!(
                part.feature == part.name.to_ascii_lowercase(),
                "inconsistent part feature"
            );
            ensure!(
                part.memory_status == "verified-from-official-datasheet",
                "unverified part memory"
            );
            ensure!(
                !part.package.is_empty() && !part.source_ids.is_empty(),
                "part missing package or evidence"
            );
            let mut docs = Vec::new();
            for id in &part.source_ids {
                let source = catalog
                    .sources
                    .get(id)
                    .with_context(|| format!("unknown part source {id}"))?;
                let url = source["url"].as_str().context("part source has no URL")?;
                docs.push(chip::Doc {
                    r#type: if id.ends_with("_datasheet") {
                        "datasheet"
                    } else {
                        "vendor-source"
                    }
                    .into(),
                    title: id.clone(),
                    name: id.clone(),
                    url: url.into(),
                });
            }
            chips.push(ChipInput {
                name: part.name.clone(),
                memory: vec![part.memory.clone()],
                packages: vec![chip::Package {
                    name: part.name.clone(),
                    package: part.package.clone(),
                    pins: vec![],
                }],
                docs,
            });
        }
    }
    chips.sort_by(|a, b| a.name.cmp(&b.name));
    ensure!(
        chips.windows(2).all(|pair| pair[0].name != pair[1].name),
        "duplicate chip names"
    );
    Ok(chips)
}

/// Fail before parsing if the downloaded file differs from the reviewed input.
pub fn verify_sha256(bytes: &[u8], expected: &str) -> Result<()> {
    let actual = format!("{:x}", Sha256::digest(bytes));
    ensure!(
        actual == expected,
        "source SHA-256 mismatch: expected {expected}, got {actual}"
    );
    Ok(())
}

/// Strict except for documented byte-access aliases sharing ODR at 0x54.
fn validate_register_ir(ir: &ir::IR, aliases: &[RegisterAlias]) -> Result<()> {
    let errors = chiptool::validate::validate(
        ir,
        chiptool::validate::Options {
            allow_register_overlap: true,
            allow_field_overlap: false,
            allow_enum_dup_value: false,
            allow_unused_enums: false,
            allow_unused_fieldsets: false,
        },
    );
    ensure!(
        errors.is_empty(),
        "invalid register IR: {}",
        errors.join("; ")
    );
    for (name, block) in &ir.blocks {
        let mut spans: Vec<(u32, u32, &str, bool)> = Vec::new();
        for item in &block.items {
            let ir::BlockItemInner::Register(reg) = &item.inner else {
                continue;
            };
            let offsets = match &item.array {
                None => vec![0],
                Some(ir::Array::Regular(array)) => {
                    ensure!(array.len > 0, "empty register array in {name}");
                    (0..array.len)
                        .map(|index| {
                            index
                                .checked_mul(array.stride)
                                .context("register array stride overflow")
                        })
                        .collect::<Result<Vec<_>>>()?
                }
                Some(ir::Array::Cursed(_)) => {
                    anyhow::bail!("non-regular register arrays are not qualified: {name}")
                }
            };
            for offset in offsets {
                let start = item
                    .byte_offset
                    .checked_add(offset)
                    .context("register array offset overflow")?;
                let end = start
                    .checked_add(reg.bit_size.div_ceil(8))
                    .context("register span overflow")?;
                for &(previous_start, previous_end, previous_name, previous_array) in &spans {
                    if start < previous_end && previous_start < end {
                        // Reviewed byte-width aliases remain scalar-only. Array
                        // elements must not overlap themselves or another item.
                        let allowed = item.array.is_none()
                            && !previous_array
                            && aliases.iter().any(|alias| {
                                alias.block == *name
                                    && ((alias.register == item.name
                                        && alias.alias == previous_name)
                                        || (alias.alias == item.name
                                            && alias.register == previous_name))
                            });
                        ensure!(
                            allowed,
                            "unexpected overlapping register spans in {name}: {previous_name}/{}",
                            item.name
                        );
                    }
                }
                spans.push((start, end, &item.name, item.array.is_some()));
            }
        }
    }
    Ok(())
}

/// Normal generation reads the reviewed, checked-in register YAML. It never
/// rebuilds those definitions from SVD or writes into the curated source tree.
pub fn generate(root: &Path, manifest: &Path, output: &Path) -> Result<Report> {
    generate_inner(root, manifest, output, RegisterMode::Curated)
}

/// Explicit source-import candidate generation. A human reviews the diff before
/// promoting any candidate YAML into cw32-data/registers.
pub fn import_register_candidates(root: &Path, manifest: &Path, output: &Path) -> Result<Report> {
    let output = candidate_output(root, output)?;
    let result = generate_inner(root, manifest, &output, RegisterMode::ImportCandidates);
    // Seal only files produced by this candidate operation. The next profile
    // can reuse this review directory, but unknown or edited files cannot be
    // silently republished as fresh import output.
    let files = candidate_inventory(&output)?;
    if !files.is_empty() {
        write_json(
            &output.join("candidate-import.json"),
            &serde_json::json!({
                "schema_version": 1, "format": "cw32-review-only-register-candidates", "files": files,
            }),
        )?;
    }
    result
}

fn candidate_output(root: &Path, output: &Path) -> Result<std::path::PathBuf> {
    // Resolve relative paths, .. and symlinks before writing generated files.
    // The command uses a fresh staging directory; existing child redirects
    // are also rejected if this library API is called directly.
    fs::create_dir_all(output)?;
    let output = fs::canonicalize(output)?;
    let curated = fs::canonicalize(root.join("cw32-data"))?;
    ensure!(
        !output.starts_with(&curated),
        "candidate imports must not overwrite curated data"
    );
    for active in ["chips", "field-access", "register-writes"] {
        ensure!(
            !output.join(active).exists(),
            "candidate output contains active curated metadata {active}; choose a separate review directory"
        );
    }
    for child in ["registers", "chips", "reports", "field-access-candidates"] {
        let path = output.join(child);
        reject_candidate_redirects(&path)?;
        if path.exists() {
            ensure!(
                !fs::canonicalize(path)?.starts_with(&curated),
                "candidate output redirects into curated data"
            );
        }
    }
    let files = candidate_inventory(&output)?;
    let marker = output.join("candidate-import.json");
    if !files.is_empty() || marker.exists() {
        let provenance: serde_json::Value = serde_json::from_slice(&fs::read(&marker).context(
            "nonempty candidate output lacks importer provenance; use a fresh review directory",
        )?)?;
        ensure!(
            provenance["schema_version"] == 1
                && provenance["format"] == "cw32-review-only-register-candidates",
            "output is not a recognized candidate import"
        );
        ensure!(
            provenance["files"] == serde_json::to_value(files)?,
            "candidate output contains unknown or modified files; use a fresh review directory"
        );
    }
    Ok(output)
}

// A present-run integrity inventory, not a hardware assertion or historical
// source fingerprint. The full per-profile files retain source provenance.
fn candidate_inventory(output: &Path) -> Result<BTreeMap<String, String>> {
    fn collect(root: &Path, directory: &Path, files: &mut BTreeMap<String, String>) -> Result<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            ensure!(
                !entry.file_type()?.is_symlink(),
                "candidate output redirect could overwrite curated data: {}",
                path.display()
            );
            let relative = path.strip_prefix(root)?.to_string_lossy().into_owned();
            if entry.file_type()?.is_dir() {
                ensure!(
                    ["registers", "reports", "field-access-candidates"]
                        .iter()
                        .any(|name| relative == *name || relative.starts_with(&format!("{name}/"))),
                    "unrecognized candidate output directory {relative}"
                );
                collect(root, &path, files)?;
            } else if relative != "candidate-import.json" {
                files.insert(relative, format!("{:x}", Sha256::digest(fs::read(path)?)));
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    collect(output, output, &mut files)?;
    Ok(files)
}

// Existing candidate subtrees must not redirect an individual output file or
// nested map directory into authored data (including a dangling symlink).
fn reject_candidate_redirects(path: &Path) -> Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    ensure!(
        !metadata.file_type().is_symlink(),
        "candidate output redirect could overwrite curated data: {}",
        path.display()
    );
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            reject_candidate_redirects(&entry?.path())?;
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RegisterMode {
    Curated,
    ImportCandidates,
}

fn generate_inner(
    root: &Path,
    manifest: &Path,
    output: &Path,
    mode: RegisterMode,
) -> Result<Report> {
    let input = load_input(manifest)?;
    if mode == RegisterMode::Curated {
        ensure!(
            !cw32_data_serde::support::is_retired(&input.line)
                && !cw32_data_serde::support::is_retired(&input.die)
                && !input.chips.iter().any(|chip| cw32_data_serde::support::is_retired(&chip.name)),
            "retired target: {} is not supported; no L031 package alias is provided",
            input.line
        );
        ensure!(
            !expand_chip_inputs(root, &input)?.iter().any(|chip| cw32_data_serde::support::is_retired(&chip.name)),
            "retired target in expanded part catalog"
        );
    }
    ensure!(
        input.schema_version == 1,
        "unsupported input manifest version"
    );
    ensure!(
        input.quarantine.is_none(),
        "source quarantined: {}",
        input.quarantine.as_deref().unwrap_or("")
    );
    ensure!(
        input.nvic_priority_bits.is_none() || !input.nvic_priority_bits_evidence.is_empty(),
        "NVIC override requires evidence"
    );
    let bytes = fs::read(root.join(&input.source.path))
        .with_context(|| format!("read {} (run ./d fetch-sources first)", input.source.path))?;
    verify_sha256(&bytes, &input.source.sha256)?;
    let xml = std::str::from_utf8(&bytes)?;
    // Same parse configuration as chiptool::commands::load_svd. Validation is
    // performed on the resulting IR, including intentional GPIO byte aliases.
    let config = svd_parser::Config::default()
        .expand_properties(true)
        .validate_level(svd_parser::svd::ValidateLevel::Disabled);
    let mut access_capture = if mode == RegisterMode::ImportCandidates {
        Some(svd_access::Capture::parse(xml).context("capture source SVD field access")?)
    } else {
        None
    };
    let svd = if let Some(capture) = &access_capture {
        capture.device.clone()
    } else {
        svd_parser::parse_with_config(xml, &config)?
    };
    ensure!(
        svd.name == input.expected_svd_name,
        "unexpected SVD device {}",
        svd.name
    );
    let nvic_priority_bits = input
        .nvic_priority_bits
        .or_else(|| svd.cpu.as_ref().map(|cpu| cpu.nvic_priority_bits as u8));
    let mut register_files = BTreeMap::<String, ir::IR>::new();
    let mut peripherals = Vec::new();
    let mut irqs = BTreeMap::<String, u32>::new();
    let mut irq_numbers = BTreeMap::<u32, String>::new();
    let mut pins = BTreeSet::new();
    let mut applied_overrides = BTreeSet::new();
    for p in &svd.peripherals {
        let base = match &p.derived_from {
            Some(name) => svd
                .peripherals
                .iter()
                .find(|p| p.name == *name)
                .with_context(|| format!("unknown derivedFrom {name}"))?,
            None => p,
        };
        ensure!(
            base.derived_from.is_none(),
            "nested derived peripherals require explicit handling"
        );
        let block = base
            .header_struct_name
            .clone()
            .unwrap_or_else(|| base.name.clone());
        let kind = block.trim().to_ascii_lowercase();
        let version = input
            .register_versions
            .get(&kind)
            .unwrap_or(&input.register_version);
        ensure!(
            kind.chars().all(|c| c.is_ascii_alphanumeric()),
            "unsupported peripheral kind {kind}"
        );
        if !register_files.contains_key(&kind) || p.derived_from.is_none() {
            let mut ir = match mode {
                RegisterMode::Curated => {
                    let path = root
                        .join("cw32-data/registers")
                        .join(format!("{}_{}.yaml", kind, version));
                    serde_yaml::from_slice(&fs::read(&path).with_context(|| {
                        format!("missing curated register source {}", path.display())
                    })?)?
                }
                RegisterMode::ImportCandidates => {
                    let ir = chiptool::commands::extract_peripheral(base, NamespaceMode::None)?;
                    access_capture.as_mut().unwrap().project(&p.name, &ir)?;
                    ir
                }
            };
            for correction in &input.register_overrides {
                if let Some(block) = ir.blocks.get_mut(&correction.block) {
                    ensure!(
                        !correction.evidence.is_empty(),
                        "register correction needs evidence"
                    );
                    let item = block
                        .items
                        .iter_mut()
                        .find(|i| i.name == correction.register)
                        .with_context(|| {
                            format!(
                                "unknown corrected register {}.{}",
                                correction.block, correction.register
                            )
                        })?;
                    let ir::BlockItemInner::Register(reg) = &mut item.inner else {
                        anyhow::bail!("correction target is not a register")
                    };
                    match mode {
                        RegisterMode::ImportCandidates => {
                            ensure!(
                                reg.access == correction.expected_access,
                                "correction old access does not match {}.{}",
                                correction.block,
                                correction.register
                            );
                            reg.access = correction.access.clone();
                        }
                        RegisterMode::Curated => {
                            // This is a provenance assertion, never a patch during
                            // normal generation. Curated YAML is authoritative.
                            ensure!(
                                reg.access == correction.access,
                                "curated correction provenance mismatch {}.{}",
                                correction.block,
                                correction.register
                            );
                        }
                    }
                    applied_overrides.insert(format!(
                        "{}.{} {:?} -> {:?}: {}",
                        correction.block,
                        correction.register,
                        correction.expected_access,
                        correction.access,
                        correction.evidence
                    ));
                }
            }
            for correction in &input.register_removals {
                if let Some(block) = ir.blocks.get_mut(&correction.block) {
                    ensure!(
                        !correction.evidence.trim().is_empty(),
                        "register removal requires evidence"
                    );
                    let position = block
                        .items
                        .iter()
                        .position(|r| r.name == correction.register);
                    match mode {
                        RegisterMode::ImportCandidates => {
                            let index =
                                position.context("source register removal precondition missing")?;
                            let item = &block.items[index];
                            ensure!(
                                item.byte_offset == correction.expected_byte_offset,
                                "register removal offset changed"
                            );
                            let ir::BlockItemInner::Register(reg) = &item.inner else {
                                anyhow::bail!("removed item must be register")
                            };
                            ensure!(
                                reg.fieldset.as_ref() == Some(&correction.expected_fieldset),
                                "register removal fieldset changed"
                            );
                            block.items.remove(index);
                            ir.fieldsets.remove(&correction.expected_fieldset);
                        }
                        RegisterMode::Curated => ensure!(
                            position.is_none()
                                && !ir.fieldsets.contains_key(&correction.expected_fieldset),
                            "unsupported register remains in curated YAML"
                        ),
                    }
                    applied_overrides.insert(format!(
                        "{}.{} removed: {}",
                        correction.block, correction.register, correction.evidence
                    ));
                }
            }
            for correction in &input.field_removals {
                if correction
                    .block
                    .as_deref()
                    .is_some_and(|name| name != block)
                {
                    continue;
                }
                if let Some(fieldset) = ir.fieldsets.get_mut(&correction.fieldset) {
                    ensure!(
                        !correction.evidence.is_empty(),
                        "field correction needs evidence"
                    );
                    let position = fieldset
                        .fields
                        .iter()
                        .position(|field| field.name == correction.field);
                    match mode {
                        RegisterMode::ImportCandidates => {
                            let index =
                                position.context("expected obsolete source field missing")?;
                            let field = &fieldset.fields[index];
                            ensure!(
                                field.bit_offset
                                    == ir::BitOffset::Regular(correction.expected_bit_offset)
                                    && field.bit_size == correction.expected_bit_size,
                                "obsolete field position differs from reviewed source"
                            );
                            fieldset.fields.remove(index);
                        }
                        RegisterMode::Curated => {
                            ensure!(position.is_none(), "reserved field remains in curated YAML")
                        }
                    }
                    applied_overrides.insert(format!(
                        "Removed reserved field {}.{} at bits {}+{}: {}",
                        correction.fieldset,
                        correction.field,
                        correction.expected_bit_offset,
                        correction.expected_bit_size,
                        correction.evidence
                    ));
                }
            }
            for correction in &input.field_width_overrides {
                if apply_field_width_override(&mut ir, correction, mode)? {
                    applied_overrides.insert(format!(
                        "Corrected field {}.{}.{} at bit {}: width {} -> {}: {}",
                        correction.block,
                        correction.fieldset,
                        correction.field,
                        correction.expected_bit_offset,
                        correction.expected_bit_size,
                        correction.bit_size,
                        correction.evidence
                    ));
                }
            }
            validate_register_ir(&ir, &input.register_aliases)
                .with_context(|| format!("validate {kind}"))?;
            if let Some(old) = register_files.insert(kind.clone(), ir.clone()) {
                ensure!(old == ir, "distinct source blocks share kind {kind}");
            }
        }
        if input.gpio_dir_pin_fields && p.name.starts_with("GPIO") {
            let regs = &register_files[&kind];
            let dir = regs.blocks[&block]
                .items
                .iter()
                .find(|r| r.name == "DIR")
                .context("GPIO has no DIR register")?;
            if let ir::BlockItemInner::Register(reg) = &dir.inner {
                let fields =
                    &regs.fieldsets[reg.fieldset.as_ref().context("GPIO DIR has no fields")?];
                for field in &fields.fields {
                    if let Some(n) = field.name.strip_prefix("PIN") {
                        let _: u8 = n.parse()?;
                        pins.insert(format!("P{}{n}", p.name.strip_prefix("GPIO").unwrap()));
                    }
                }
            }
        }
        let mut peripheral_irqs = Vec::new();
        for irq in &p.interrupt {
            let name = irq.name.trim().to_ascii_uppercase();
            if let Some(old) = irqs.insert(name.clone(), irq.value) {
                ensure!(old == irq.value, "conflicting IRQ values for {name}");
            }
            if let Some(old) = irq_numbers.insert(irq.value, name.clone()) {
                ensure!(
                    old == name,
                    "IRQ {} has ambiguous aliases {old}/{name}",
                    irq.value
                );
            }
            peripheral_irqs.push(chip::core::peripheral::Interrupt {
                signal: "GLOBAL".into(),
                interrupt: name,
            });
        }
        peripherals.push(chip::core::Peripheral {
            name: p.name.trim().to_ascii_uppercase(),
            address: p.base_address.try_into()?,
            registers: Some(chip::core::peripheral::Registers {
                kind,
                version: version.clone(),
                block,
            }),
            rcc: None,
            rcc_control: None,
            spi: None,
            classic_timer_input: None,
            classic_atim_input: None,
            atim_complementary: None,
            lvd: None,
            lcd: None,
            rtc_calendar: None,
            rtc_alarms: None,
            ir: None,
            gpio_interrupt: None,
            gpio: None,
            cordic: None,
            aes: None,
            trng: None,
            ram_parity: None,
            clock_limits: None,
            adc_limits: None,
            comparator_limits: None,
            reference_divider: None,
            dac_limits: None,
            opa_limits: None,
            iwdt_clock: None,
            i2c_limits: None,
            flash_limits: None,

            pins: vec![],
            interrupts: peripheral_irqs,
            dma_channels: vec![],
            triggers: vec![],
            afio: None,
        });
    }
    for irq in &input.supplemental_interrupts {
        ensure!(
            !irq.evidence.is_empty(),
            "supplemental IRQ {} needs evidence",
            irq.name
        );
        if let Some(old) = irqs.insert(irq.name.clone(), irq.number) {
            ensure!(
                old == irq.number,
                "conflicting supplemental IRQ {}",
                irq.name
            );
        }
        if let Some(old) = irq_numbers.insert(irq.number, irq.name.clone()) {
            ensure!(
                old == irq.name,
                "supplemental IRQ aliases {old}/{}",
                irq.name
            );
        }
    }
    interrupts::apply(
        &mut peripherals,
        &irqs,
        &input.interrupt_association_overrides,
    )?;
    peripherals.sort_by(|a, b| a.name.cmp(&b.name));
    let mut interrupts: Vec<_> = irqs
        .into_iter()
        .map(|(name, number)| -> Result<_> {
            Ok(chip::core::Interrupt {
                name,
                number: number.try_into()?,
            })
        })
        .collect::<Result<_>>()?;
    interrupts.sort_by_key(|i| i.number);
    let mut core = chip::Core {
        name: input.core.clone(),
        nvic_priority_bits,
        peripherals,
        interrupts,
        dma_channels: vec![],
        pins: pins
            .into_iter()
            .map(|name| chip::core::Pin { name })
            .collect(),
    };
    if mode == RegisterMode::Curated {
        fs::create_dir_all(output.join("chips"))?;
    }
    fs::create_dir_all(output.join("registers"))?;
    // Raw SVD candidates do not yet contain reviewed enums, arrays or access
    // corrections required by the production capability projections. Export
    // registers for review without advertising candidate chip/HAL metadata.
    let mut pinout_data = None;
    let mut dma_report = None;
    let mut clock_report = None;
    if mode == RegisterMode::Curated {
        let mut chips = expand_chip_inputs(root, &input)?;
        pinout_data = input
            .pinout_metadata
            .as_deref()
            .map(|path| pinouts::load(root, path, &input, &chips))
            .transpose()?;
        dma_report = input
            .dma_metadata
            .as_deref()
            .map(|path| dma::apply(root, path, &input.line, &mut core, &register_files))
            .transpose()?;
        clock_report = input
            .clock_metadata
            .as_deref()
            .map(|path| {
                clock::apply(
                    root,
                    path,
                    &input.line,
                    &mut core.peripherals,
                    &register_files,
                )
            })
            .transpose()?;
        if let Some(path) = input.gpio_interrupt_metadata.as_deref() {
            gpio_interrupt::apply(root, path, &input.line, &mut core.peripherals)?;
        }
        if let Some(path) = input.trigger_metadata.as_deref() {
            triggers::apply(root, path, &input.line, &mut core, &register_files)?;
        }
        if let Some(path) = input.spi_metadata.as_deref() {
            spi::apply(root, path, &input.line, &mut core.peripherals)?;
        }
        if let Some(path) = input.electrical_metadata.as_deref() {
            electrical::apply(root, path, &input.line, &mut core.peripherals)?;
        }
        if let Some(path) = input.adc_sequence_metadata.as_deref() {
            adc_sequence::apply(root, path, &input.line, &mut core, &register_files)?;
        }
        if let (RegisterMode::Curated, Some(path)) =
            (mode, input.classic_adc_scan_metadata.as_deref())
        {
            classic_adc_scan::apply(root, path, &input.line, &mut core, &register_files)?;
        }
        if core.peripherals.iter().any(|p| p.name == "RAM") {
            ram::apply(root, &input.line, &mut core, &register_files)?;
        }
        if let Some(pinouts) = &pinout_data {
            pinouts.project_gpio(root, &mut core)?;
        }
        accelerators::apply(root, &input.line, &mut core)?;
        crypto::apply(root, &input.line, &mut core, &register_files)?;
        for chip in &mut chips {
            let mut chip_core = core.clone();
            if let Some(pinouts) = &pinout_data {
                pinouts.project(chip, &mut chip_core)?;
            }
            if let Some(path) = input.af_metadata.as_deref() {
                af::apply(root, path, &input.line, &mut chip_core, &register_files)?;
            }
            if let Some(path) = input.pwm_metadata.as_deref() {
                af::apply_pwm(root, path, &input.line, &mut chip_core, &register_files)?;
            }
            if let Some(path) = input.atim_pwm_metadata.as_deref() {
                af::apply_atim_pwm(root, path, &input.line, &mut chip_core, &register_files)?;
            }
            if let Some(path) = input.atim_complementary_metadata.as_deref() {
                af::apply_atim_complementary(
                    root,
                    path,
                    &input.line,
                    &mut chip_core,
                    &register_files,
                )?;
            }
            if let Some(path) = input.halltim_metadata.as_deref() {
                af::apply_halltim(root, path, &input.line, &mut chip_core, &register_files)?;
            }
            if let Some(path) = input.analog_metadata.as_deref() {
                af::apply_analog(root, path, &input.line, &mut chip_core)?;
            }
            if let Some(path) = input.comparator_metadata.as_deref() {
                comparator::apply(root, path, &input.line, &mut chip_core)?;
            }
            if chip_core.peripherals.iter().any(|p| p.name == "LVD") {
                lvd_ir::apply(root, &input.line, &mut chip_core, &register_files)?;
            }
            vref::apply(root, &input.line, &mut chip_core)?;
            dac_opa::apply(root, &input.line, &mut chip_core)?;
            rtc::apply(root, &input.line, &mut chip_core, &register_files)?;
            electrical::apply_lse(root, chip, &input.line, &mut chip_core, &register_files)?;
            lsi::apply(root, chip, &input.line, &mut chip_core, &register_files)?;
            rtc::validate_native_live_lsi(root, chip, &input.line, &chip_core, &register_files)?;
            electrical::apply_mco(root, chip, &input.line, &mut chip_core, &register_files)?;
            classic_timer_input::apply(root, &input.line, &mut chip_core, &register_files)?;
            classic_atim_input::apply(root, &input.line, &chip.name, &mut chip_core, &register_files)?;
            if chip_core.peripherals.iter().any(|p| p.name == "LCD") {
                lcd::apply(root, &input.line, &mut chip_core, &register_files)?;
            }
            ensure!(
                chip.name.starts_with(&input.chip_prefix),
                "unsupported chip {}",
                chip.name
            );
            let metadata = Chip {
                name: chip.name.clone(),
                family: input.family.clone(),
                line: input.line.clone(),
                die: input.die.clone(),
                device_id: None,
                packages: chip.packages.clone(),
                memory: chip.memory.clone(),
                docs: chip.docs.clone(),
                cores: vec![chip_core],
            };
            // device_id is not present in vendor input: omit rather than claim ID 0.
            let mut json = serde_json::to_value(metadata)?;
            json.as_object_mut().unwrap().remove("device_id");
            write_json(
                &output.join("chips").join(format!("{}.json", chip.name)),
                &json,
            )?;
        }
        if !input.firmware_targets.is_empty() {
            let mut assigned = BTreeSet::new();
            for (target, members) in &input.firmware_targets {
                ensure!(members.len() >= 2 && !chips.iter().any(|c| &c.name == target),
                    "invalid package-neutral target {target}");
                let mut reference = None;
                for name in members {
                    ensure!(assigned.insert(name) && name.starts_with(target)
                        && chips.iter().any(|c| &c.name == name && c.packages.len() == 1),
                        "invalid or duplicate package {name} in {target}");
                    let path = output.join("chips").join(format!("{name}.json"));
                    let mut chip: serde_json::Value = serde_json::from_slice(&fs::read(path)?)?;
                    let object = chip.as_object_mut().unwrap();
                    object.remove("name");
                    object.remove("packages");
                    if let Some(first) = &reference {
                        ensure!(first == &chip, "{name} differs from other firmware targets in {target}");
                    } else {
                        reference = Some(chip);
                    }
                }
            }
            let groups = output.join("package-groups");
            fs::create_dir_all(&groups)?;
            write_json(&groups.join(format!("{}.json", input.line)), &input.firmware_targets)?;
        }
    }
    if let Some(capture) = &access_capture {
        capture.emit(root, output, &input, &register_files)?;
    }
    let mut report = Report {
        profile: input.line.clone(),
        source_device: input.expected_svd_name.clone(),
        shared_map_evidence: input.shared_map_evidence.clone(),
        source_sha256: input.source.sha256, source_url: input.source.url,
        peripherals: core.peripherals.len(), register_blocks: register_files.len(),
        registers: 0, fieldsets: 0, fields: 0, interrupts: core.interrupts.len(),
        die_pins: core.pins.len(),
        pinouts: pinout_data.as_ref().map(pinouts::Pinouts::report),
        dma: dma_report,
        clocks: clock_report,
        applied_errata: input.nvic_priority_bits.map(|bits| vec![format!("nvicPrioBits {} -> {}: {}", svd.cpu.as_ref().map(|c|c.nvic_priority_bits).unwrap_or(0), bits, input.nvic_priority_bits_evidence)]).unwrap_or_default(),
        limitations: vec![
            "SVD-imported registers are not silicon-tested or a guarantee the vendor SVD is error-free.".into(),
            "Generic family features have no memory-size assertion; exact part features source memory separately.".into(),
            "Exact-part core pins and physical package pins come only from reviewed package tables; package-less aliases use an explicitly documented common-package GPIO intersection when configured. Input-only/output capability remains in pinout sidecars.".into(),
            "Only configured reviewed clock and DMA projections are imported. Verified reset polarity, write keys and shared controls are transported independently in rcc_control; unresolved kernel identities are withheld.".into(),
            "Digital alternate-function routes are imported only for explicitly reviewed profiles and filtered by package pins. Analog/special routes and flash geometry remain incomplete.".into(),
            "Read-clear, write-one-clear and reset-value semantics are not represented by chiptool IR.".into(),
        ],
    };
    if mode == RegisterMode::ImportCandidates {
        report.limitations = vec![
            "Review-only SVD register candidates with explicit source corrections; no curated chip, package or HAL capability metadata is emitted.".into(),
            "Import does not change the authored register tree or establish silicon correctness.".into(),
        ];
    }
    ensure_overrides_applied(
        &applied_overrides,
        input.register_overrides.len()
            + input.field_removals.len()
            + input.register_removals.len()
            + input.field_width_overrides.len(),
    )?;
    report.applied_errata.extend(applied_overrides);
    for c in &input.interrupt_association_overrides {
        report.applied_errata.push(format!(
            "IRQ {}:{} owners {:?} -> {:?}: {}",
            c.interrupt, c.number, c.expected_owners, c.owners, c.evidence
        ));
    }
    report.limitations.extend(input.source_notes);
    if let Some(clock) = &report.clocks {
        report.limitations.extend(clock.limitations.clone());
    }
    for irq in &input.supplemental_interrupts {
        report.applied_errata.push(format!(
            "Added external interrupt {}={}: {}",
            irq.name, irq.number, irq.evidence
        ));
    }
    for (kind, ir) in &register_files {
        report.registers += ir.blocks.values().map(|b| b.items.len()).sum::<usize>();
        report.fieldsets += ir.fieldsets.len();
        report.fields += ir.fieldsets.values().map(|f| f.fields.len()).sum::<usize>();
        let version = input
            .register_versions
            .get(kind)
            .unwrap_or(&input.register_version);
        let filename = format!("{}_{}", kind, version);
        let register_path = output.join("registers").join(format!("{filename}.json"));
        if register_path.exists() {
            let old: ir::IR = serde_json::from_slice(&fs::read(&register_path)?)?;
            ensure!(
                old == *ir,
                "conflicting profiles attempted to share register version {filename}"
            );
        }
        if mode == RegisterMode::Curated {
            register_write::emit(root, output, &filename, ir)?;
            field_access::emit(root, output, &filename, ir)?;
        }
        write_json(&register_path, ir)?;
        fs::write(
            output.join("registers").join(format!("{filename}.yaml")),
            serde_yaml::to_string(ir)?,
        )?;
    }
    fs::create_dir_all(output.join("reports"))?;
    write_json(
        &output.join("reports").join(format!("{}.json", input.line)),
        &report,
    )?;
    Ok(report)
}
fn apply_field_width_override(
    ir: &mut ir::IR,
    correction: &FieldWidthOverride,
    mode: RegisterMode,
) -> Result<bool> {
    let Some(block) = ir.blocks.get(&correction.block) else {
        return Ok(false);
    };
    ensure!(
        !correction.evidence.trim().is_empty(),
        "field-width correction needs evidence"
    );
    ensure!(
        block.items.iter().any(|item| matches!(
            &item.inner, ir::BlockItemInner::Register(register)
                if register.fieldset.as_deref() == Some(correction.fieldset.as_str())
        )),
        "corrected fieldset does not belong to the reviewed block"
    );
    let fieldset = ir
        .fieldsets
        .get_mut(&correction.fieldset)
        .context("corrected fieldset is missing")?;
    ensure!(
        correction.bit_size > 0
            && correction.expected_bit_size > 0
            && correction.bit_size != correction.expected_bit_size
            && correction
                .expected_bit_offset
                .checked_add(correction.bit_size)
                .is_some_and(|end| end <= fieldset.bit_size)
            && correction
                .expected_bit_offset
                .checked_add(correction.expected_bit_size)
                .is_some_and(|end| end <= fieldset.bit_size),
        "invalid reviewed field-width correction"
    );
    let field = fieldset
        .fields
        .iter_mut()
        .find(|field| field.name == correction.field)
        .context("corrected field is missing")?;
    ensure!(
        field.bit_offset == ir::BitOffset::Regular(correction.expected_bit_offset),
        "field-width correction offset does not match reviewed source"
    );
    match mode {
        RegisterMode::ImportCandidates => {
            ensure!(
                field.bit_size == correction.expected_bit_size,
                "field-width correction old width does not match reviewed source"
            );
            field.bit_size = correction.bit_size;
        }
        RegisterMode::Curated => {
            // Curated YAML is authoritative: assert provenance without patching it.
            ensure!(
                field.bit_size == correction.bit_size,
                "curated field-width correction provenance mismatch"
            );
        }
    }
    Ok(true)
}

fn ensure_overrides_applied(applied: &BTreeSet<String>, expected: usize) -> Result<()> {
    ensure!(
        applied.len() == expected,
        "some register corrections had no matching source block"
    );
    Ok(())
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut data = serde_json::to_vec_pretty(value)?;
    data.push(b'\n');
    fs::write(path, data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture_root() -> std::path::PathBuf {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("cw32-data-test-{}-{suffix}", std::process::id()));
        fs::create_dir_all(root.join("cw32-data/registers")).unwrap();
        root
    }

    #[test]
    fn all_reviewed_metadata_generates_without_changing_register_sources() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let output = fixture_root();
        let source_snapshot: BTreeMap<_, _> = fs::read_dir(root.join("cw32-data/registers"))
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let bytes = fs::read(&path).unwrap();
                (path, bytes)
            })
            .collect();
        let mut totals = (0, 0, 0, 0, 0, 0);
        for manifest in fs::read_dir(root.join("cw32-data/inputs")).unwrap() {
            let manifest = manifest.unwrap().path();
            if manifest.extension().and_then(|v| v.to_str()) != Some("yaml") {
                continue;
            }
            if cw32_data_serde::support::is_retired(&load_input(&manifest).unwrap().line) {
                assert!(generate(root, &manifest, &output).unwrap_err().to_string().contains("retired target"));
                continue;
            }
            let report = generate(root, &manifest, &output).unwrap();
            let pins = report.pinouts.unwrap();
            let dma = report.dma.unwrap();
            let clocks = report.clocks.unwrap();
            totals.0 += pins.exact_parts;
            totals.1 += pins.physical_pin_entries;
            totals.2 += dma.physical_channels;
            totals.3 += dma.hardware_requests;
            totals.4 += clocks.supported_records;
            totals.5 += clocks.partial_records;
        }
        assert_eq!(totals, (36, 1418, 33, 341, 252, 150));
        for (path, bytes) in source_snapshot {
            assert_eq!(fs::read(path).unwrap(), bytes);
        }
        fs::remove_dir_all(output).unwrap();
    }

    #[test]
    fn normal_generation_preserves_and_uses_authored_yaml() {
        let root = fixture_root();
        // The normal pipeline validates the shared authored divider catalog,
        // including when this synthetic chip has no matching divider profile.
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../cw32-data/reference-dividers.yaml"),
            root.join("cw32-data/reference-dividers.yaml"),
        )
        .unwrap();
        // Normal generation validates the authored LSE and MCO catalogs even for
        // an unrelated synthetic chip. Keep their declared policy inputs available.
        let authored_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        for catalog_path in ["cw32-data/lse-qualified.yaml", "cw32-data/mco-qualified.yaml"] {
            let catalog = fs::read(authored_root.join(catalog_path)).unwrap();
            fs::write(root.join(catalog_path), &catalog).unwrap();
            let catalog: serde_yaml::Value = serde_yaml::from_slice(&catalog).unwrap();
            for policy in catalog["policies"].as_mapping().unwrap().keys() {
                let policy = policy.as_str().unwrap();
                let destination = root.join(policy);
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
                fs::copy(authored_root.join(policy), destination).unwrap();
            }
        }
        // The native live-LSI RTC catalog is also validated for unrelated
        // synthetic chips. Preserve its authored policy and referenced inputs.
        let rtc_catalog_path = "cw32-data/rtc-live-lsi-qualified.yaml";
        let rtc_catalog = fs::read(authored_root.join(rtc_catalog_path)).unwrap();
        fs::write(root.join(rtc_catalog_path), &rtc_catalog).unwrap();
        let rtc_catalog: serde_yaml::Value = serde_yaml::from_slice(&rtc_catalog).unwrap();
        for key in ["source_authority", "evidence", "envelope_policy"] {
            let path = rtc_catalog[key].as_str().unwrap();
            let destination = root.join(path);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(authored_root.join(path), destination).unwrap();
        }
        // ATIM input validation reads its catalog before excluding this chip.
        let atim_catalog_path = "cw32-data/classic-atim-input.yaml";
        let atim_catalog = fs::read(authored_root.join(atim_catalog_path)).unwrap();
        fs::write(root.join(atim_catalog_path), &atim_catalog).unwrap();
        let atim_catalog: serde_yaml::Value = serde_yaml::from_slice(&atim_catalog).unwrap();
        let evidence = atim_catalog["evidence"].as_str().unwrap();
        fs::copy(authored_root.join(evidence), root.join(evidence)).unwrap();
        fs::write(
            root.join("cw32-data/field-access.yaml"),
            "schema_version: 1\nregisters: {}\n",
        )
        .unwrap();
        let xml = r#"<device schemaVersion="1.1"><name>CW32TEST</name><version>1.0</version><description>Synthetic generator fixture</description><addressUnitBits>8</addressUnitBits><width>32</width><size>32</size><access>read-write</access><peripherals><peripheral><name>DEMO</name><baseAddress>0x40000000</baseAddress><registers><register><name>VALUE</name><description>Raw import text</description><addressOffset>0</addressOffset><size>32</size><access>read-write</access></register></registers></peripheral></peripherals></device>"#;
        fs::write(root.join("source.svd"), xml).unwrap();
        let authored: ir::IR = serde_json::from_str(r#"{"block/DEMO":{"description":"Authored source","items":[{"name":"VALUE","description":"Reviewed text","byte_offset":0,"access":"Read"}]}}"#).unwrap();
        let authored_path = root.join("cw32-data/registers/demo_fixture.yaml");
        let authored_bytes = serde_yaml::to_string(&authored).unwrap();
        fs::write(&authored_path, &authored_bytes).unwrap();
        let manifest = serde_json::json!({
            "schema_version":1,"source":{"path":"source.svd","url":"https://example.invalid/test-fixture","sha256":format!("{:x}",Sha256::digest(xml.as_bytes()))},
            "expected_svd_name":"CW32TEST","core":"cm0p","die":"CW32TEST","chip_prefix":"CW32TEST","family":"fixture","line":"CW32TEST","register_version":"fixture","nvic_priority_bits":null,"chips":[{"name":"CW32TEST","memory":[]}]
        });
        let manifest_path = root.join("input.yaml");
        fs::write(&manifest_path, serde_yaml::to_string(&manifest).unwrap()).unwrap();
        let legacy_manifest = root.join("input.json");
        fs::write(&legacy_manifest, serde_json::to_vec(&manifest).unwrap()).unwrap();
        assert!(
            load_input(&legacy_manifest)
                .unwrap_err()
                .to_string()
                .contains("must use a .yaml path")
        );
        generate(&root, &manifest_path, &root.join("normal")).unwrap();
        let normalized: ir::IR = serde_json::from_slice(
            &fs::read(root.join("normal/registers/demo_fixture.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(normalized, authored);
        assert_eq!(fs::read_to_string(&authored_path).unwrap(), authored_bytes);
        import_register_candidates(&root, &manifest_path, &root.join("candidates")).unwrap();
        let candidate: ir::IR = serde_json::from_slice(
            &fs::read(root.join("candidates/registers/demo_fixture.json")).unwrap(),
        )
        .unwrap();
        assert_ne!(
            candidate, authored,
            "explicit import must be independent from authored edits"
        );
        assert_eq!(fs::read_to_string(&authored_path).unwrap(), authored_bytes);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn candidate_import_rejects_dotdot_and_symlink_redirects() {
        let root = fixture_root();
        fs::create_dir_all(root.join("build")).unwrap();
        assert!(candidate_output(&root, &root.join("build/../cw32-data")).is_err());
        #[cfg(unix)]
        {
            let candidate = root.join("candidates");
            fs::create_dir_all(&candidate).unwrap();
            std::os::unix::fs::symlink(
                root.join("cw32-data/registers"),
                candidate.join("registers"),
            )
            .unwrap();
            assert!(candidate_output(&root, &candidate).is_err());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn candidate_import_rejects_relative_curated_path() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        assert!(candidate_output(root, Path::new("../cw32-data")).is_err());
    }

    #[test]
    fn correction_without_matching_block_is_rejected() {
        assert!(ensure_overrides_applied(&BTreeSet::new(), 1).is_err());
        assert!(ensure_overrides_applied(&BTreeSet::new(), 0).is_ok());
    }

    fn width_fixture() -> (ir::IR, FieldWidthOverride) {
        let ir = serde_json::from_str(r#"{
            "block/CRC":{"items":[{"name":"RESULT32","byte_offset":12,"access":"Read","fieldset":"RESULT32"}]},
            "fieldset/RESULT32":{"fields":[{"name":"RESULT32","bit_offset":0,"bit_size":32}]}
        }"#).unwrap();
        let correction = FieldWidthOverride {
            block: "CRC".into(),
            fieldset: "RESULT32".into(),
            field: "RESULT32".into(),
            expected_bit_offset: 0,
            expected_bit_size: 32,
            bit_size: 16,
            evidence: "Reviewed F020 manual section10.6.3: upper result half reserved".into(),
        };
        (ir, correction)
    }

    #[test]
    fn field_width_override_imports_but_curated_only_asserts() {
        let (mut ir, correction) = width_fixture();
        let original = ir.clone();
        assert!(apply_field_width_override(&mut ir, &correction, RegisterMode::Curated).is_err());
        assert_eq!(
            ir, original,
            "normal generation must never patch stale curated YAML"
        );
        assert!(
            apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .unwrap()
        );
        assert_eq!(ir.fieldsets["RESULT32"].fields[0].bit_size, 16);
        let ir::BlockItemInner::Register(reg) = &ir.blocks["CRC"].items[0].inner else {
            panic!()
        };
        assert_eq!(
            reg.bit_size, 32,
            "valid field width must not change bus-access width"
        );
        let corrected = ir.clone();
        assert!(apply_field_width_override(&mut ir, &correction, RegisterMode::Curated).unwrap());
        assert_eq!(ir, corrected);
        assert!(
            apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .is_err(),
            "stale source-width precondition must fail closed"
        );
    }

    #[test]
    fn field_width_override_rejects_missing_evidence_offsets_and_invalid_spans() {
        let (mut ir, mut correction) = width_fixture();
        correction.evidence.clear();
        assert!(
            apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .is_err()
        );
        correction.evidence = "reviewed".into();
        correction.expected_bit_offset = 1;
        assert!(
            apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .is_err()
        );
        correction.expected_bit_offset = 0;
        for width in [0, 32, 33] {
            correction.bit_size = width;
            assert!(
                apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                    .is_err()
            );
        }
        correction.bit_size = 16;
        correction.fieldset = "CR".into();
        assert!(
            apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .is_err()
        );
        correction.block = "OTHER".into();
        assert!(
            !apply_field_width_override(&mut ir, &correction, RegisterMode::ImportCandidates)
                .unwrap()
        );
    }

    #[test]
    fn f020_crc_width_override_is_narrow_and_preserves_input_widths() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let input: Input = crate::read_yaml(root.join("cw32-data/inputs/cw32f020.yaml")).unwrap();
        assert_eq!(input.register_versions["crc"], "cw32f020_v1");
        assert_eq!(input.field_width_overrides.len(), 1);
        let mut ir: ir::IR = serde_yaml::from_slice(
            &fs::read(root.join("cw32-data/registers/crc_cw32f020_v1.yaml")).unwrap(),
        )
        .unwrap();
        assert!(
            apply_field_width_override(
                &mut ir,
                &input.field_width_overrides[0],
                RegisterMode::Curated
            )
            .unwrap()
        );
        for (name, width) in [
            ("DR8", 8),
            ("DR16", 16),
            ("DR32", 32),
            ("RESULT16", 16),
            ("RESULT32", 16),
        ] {
            assert_eq!(ir.fieldsets[name].fields[0].bit_size, width);
        }
        assert_eq!(
            ir.fieldsets["CR"].fields[0].bit_size, 4,
            "MODE is still physically four bits"
        );
        validate_register_ir(&ir, &input.register_aliases).unwrap();
    }

    #[test]
    fn partial_register_overlaps_require_an_explicit_alias() {
        let ir: ir::IR = serde_json::from_str(
            r#"{"block/GPIO":{"items":[
            {"name":"ODR","byte_offset":84},
            {"name":"ODRHIGHBYTE","byte_offset":85,"bit_size":8}
        ]}}"#,
        )
        .unwrap();
        assert!(validate_register_ir(&ir, &[]).is_err());
        assert!(
            validate_register_ir(
                &ir,
                &[RegisterAlias {
                    block: "GPIO".into(),
                    register: "ODR".into(),
                    alias: "ODRHIGHBYTE".into()
                }]
            )
            .is_ok()
        );
    }

    #[test]
    fn source_hash_rejects_modified_input() {
        let hash = format!("{:x}", Sha256::digest(b"official"));
        assert!(verify_sha256(b"official", &hash).is_ok());
        assert!(verify_sha256(b"modified", &hash).is_err());
    }
    #[test]
    fn manifest_does_not_assign_generic_memory() {
        let input: Input =
            serde_yaml::from_str(include_str!("../../cw32-data/inputs/cw32f030.yaml")).unwrap();
        assert!(
            input
                .chips
                .iter()
                .find(|c| c.name == "CW32F030")
                .unwrap()
                .memory
                .is_empty()
        );
        assert_eq!(input.nvic_priority_bits, Some(2));
    }

    #[test]
    fn reviewed_adc_array_spans_reject_collisions() {
        let original: ir::IR = serde_yaml::from_str(include_str!(
            "../../cw32-data/registers/adc_cw32l010_v1.yaml"
        ))
        .unwrap();
        validate_register_ir(&original, &[]).unwrap();
        let mut overlap = original.clone();
        let result = overlap
            .blocks
            .get_mut("ADC")
            .unwrap()
            .items
            .iter_mut()
            .find(|r| r.name == "RESULT")
            .unwrap();
        result.array = Some(ir::Array::Regular(ir::RegularArray { len: 8, stride: 2 }));
        assert!(validate_register_ir(&overlap, &[]).is_err());
        let mut collision = original;
        let result = collision
            .blocks
            .get_mut("ADC")
            .unwrap()
            .items
            .iter_mut()
            .find(|r| r.name == "RESULT")
            .unwrap();
        result.byte_offset = 0x2c;
        assert!(validate_register_ir(&collision, &[]).is_err());
    }
}

#[cfg(test)]
mod historical_test_data;
