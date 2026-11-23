// Adapted from embassy-rs/stm32-data at 37a22f31552ba1fd29b3ef192c4578b84abee6e1.
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Write as _};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use chiptool::generate::CommonModule;
use chiptool::{generate, ir, transform};
use regex::Regex;

mod data;
mod field_access;
mod register_write;
use data::*;

pub struct Options {
    pub chips: Vec<String>,
    pub out_dir: PathBuf,
    pub data_dir: PathBuf,
}

pub struct Gen {
    opts: Options,
    all_peripheral_versions: HashSet<(String, String)>,
    metadata_dedup: HashMap<String, String>,
    chip_peripheral_versions: BTreeMap<String, BTreeSet<String>>,
}

impl Gen {
    pub fn new(opts: Options) -> Self {
        Self {
            opts,
            all_peripheral_versions: HashSet::new(),
            metadata_dedup: HashMap::new(),
            chip_peripheral_versions: BTreeMap::new(),
        }
    }

    fn gen_chip(&mut self, chip_core_name: &str, chip: &Chip, core: &Core) {
        let mut ir = ir::IR::new();

        let mut dev = ir::Device {
            nvic_priority_bits: core.nvic_priority_bits,
            interrupts: Vec::new(),
            peripherals: Vec::new(),
        };

        let mut peripheral_versions: BTreeMap<String, String> = BTreeMap::new();

        for p in &core.peripherals {
            let mut ir_peri = ir::Peripheral {
                name: p.name.clone(),
                array: None,
                base_address: p.address,
                block: None,
                description: None,
                interrupts: BTreeMap::new(),
            };

            if let Some(bi) = &p.registers {
                if let Some(old_version) =
                    peripheral_versions.insert(bi.kind.clone(), bi.version.clone())
                {
                    if old_version != bi.version {
                        panic!(
                            "Peripheral {} has multiple versions: {} and {}",
                            bi.kind, old_version, bi.version
                        );
                    }
                }
                ir_peri.block = Some(format!("{}::{}", bi.kind, bi.block));
            }

            dev.peripherals.push(ir_peri);
        }

        for irq in &core.interrupts {
            dev.interrupts.push(ir::Interrupt {
                name: irq.name.clone(),
                description: None,
                value: irq.number,
            });
        }

        ir.devices.insert("".to_string(), dev);

        self.chip_peripheral_versions.insert(
            chip_core_name.to_ascii_lowercase(),
            peripheral_versions
                .iter()
                .map(|(module, version)| format!("{module}_{version}"))
                .collect(),
        );

        let mut extra = String::new();

        for (module, version) in &peripheral_versions {
            self.all_peripheral_versions
                .insert((module.clone(), version.clone()));
            writeln!(
                &mut extra,
                "pub use crate::peripherals::{}_{} as {};",
                module, version, module
            )
            .unwrap();
        }

        // Cleanups!
        transform::sort::Sort {}.run(&mut ir).unwrap();
        transform::sanitize::Sanitize::default()
            .run(&mut ir)
            .unwrap();

        // ==============================
        // Setup chip dir

        let chip_dir = self
            .opts
            .out_dir
            .join("src/chips")
            .join(chip_core_name.to_ascii_lowercase());
        fs::create_dir_all(&chip_dir).unwrap();

        // ==============================
        // generate pac.rs

        let data = generate::render(&ir, &gen_opts()).unwrap().to_string();
        let data = data.replace("] ", "]\n");

        // Remove inner attributes like #![no_std]
        let data = Regex::new("# *! *\\[.*\\]").unwrap().replace_all(&data, "");

        let mut file = File::create(chip_dir.join("pac.rs")).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        file.write_all(extra.as_bytes()).unwrap();

        let mut device_x = String::new();

        for irq in &core.interrupts {
            writeln!(&mut device_x, "PROVIDE({} = DefaultHandler);", irq.name).unwrap();
        }

        // ==============================
        // generate metadata.rs

        // (peripherals, interrupts, dma_channels) are often equal across multiple chips.
        // To reduce bloat, deduplicate them.
        let mut data = String::new();
        write!(
            &mut data,
            "
                pub(crate) static PERIPHERALS: &[Peripheral] = {};
                pub(crate) static INTERRUPTS: &[Interrupt] = {};
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = {};
                pub(crate) static PINS: &[Pin] = {};
            ",
            stringify(&core.peripherals),
            stringify(&core.interrupts),
            stringify(&core.dma_channels),
            stringify(&core.pins),
        )
        .unwrap();

        let out_dir = self.opts.out_dir.clone();
        let n = self.metadata_dedup.len();
        let deduped_file = self.metadata_dedup.entry(data.clone()).or_insert_with(|| {
            let ir_regex = Regex::new("\":ir_for:([a-z0-9]+):\"").unwrap();
            let mut data = ir_regex
                .replace_all(&data, "&${1}_regs::REGISTERS")
                .to_string();

            for (module, version) in &peripheral_versions {
                writeln!(
                    &mut data,
                    "pub use crate::registers::{}_{} as {}_regs;",
                    module, version, module
                )
                .unwrap();
            }

            let file = format!("metadata_{:04}.rs", n);
            let path = out_dir.join("src/chips").join(&file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, data).unwrap();

            file
        });

        let memories = chip
            .memory
            .iter()
            .map(|memory| stringify(memory))
            .collect::<Vec<_>>()
            .join(",");

        let data = format!(
            "include!(\"../{}\");
            use crate::metadata::PeripheralRccKernelClock::{{Clock, Mux}};
            pub static METADATA: Metadata = Metadata {{
                name: {:?},
                family: {:?},
                line: {:?},
                memory: &[{}],
                peripherals: PERIPHERALS,
                nvic_priority_bits: {:?},
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            }};",
            deduped_file, &chip.name, &chip.family, &chip.line, memories, &core.nvic_priority_bits,
        );

        let mut file = File::create(chip_dir.join("metadata.rs")).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        // ==============================
        // generate device.x

        File::create(chip_dir.join("device.x"))
            .unwrap()
            .write_all(device_x.as_bytes())
            .unwrap();
    }

    fn load_chip(&mut self, name: &str) -> Chip {
        let chip_path = self
            .opts
            .data_dir
            .join("chips")
            .join(format!("{}.json", name));
        let chip = fs::read(chip_path).unwrap_or_else(|_| panic!("Could not load chip {}", name));
        serde_json::from_slice(&chip).unwrap()
    }

    pub fn run_gen(&mut self) {
        // Validate every requested and embedded identity before creating output.
        // In particular, stale chip JSON must not restore a retired Cargo feature.
        for name in &self.opts.chips.clone() {
            assert!(!cw32_data_serde::support::is_retired(name), "retired target: {name}");
            let chip = self.load_chip(name);
            assert!(
                !cw32_data_serde::support::is_retired(&chip.name)
                    && !cw32_data_serde::support::is_retired(&chip.line),
                "retired target in chip metadata: {} ({})", chip.name, chip.line
            );
        }
        fs::create_dir_all(self.opts.out_dir.join("src/peripherals")).unwrap();
        fs::create_dir_all(self.opts.out_dir.join("src/registers")).unwrap();
        fs::create_dir_all(self.opts.out_dir.join("src/chips")).unwrap();

        let mut chip_core_names: Vec<String> = Vec::new();

        for chip_name in &self.opts.chips.clone() {
            println!("Generating {}...", chip_name);

            let mut chip = self.load_chip(chip_name);

            // Cleanup
            for core in &mut chip.cores {
                for irq in &mut core.interrupts {
                    irq.name = irq.name.to_ascii_uppercase();
                }
                for p in &mut core.peripherals {
                    for irq in &mut p.interrupts {
                        irq.interrupt = irq.interrupt.to_ascii_uppercase();
                    }

                    if let Some(rcc) = &mut p.rcc {
                        for reference in rcc.enable.iter_mut().chain(rcc.reset.iter_mut()) {
                            sanitize_rcc_reference(reference);
                        }
                        if let PeripheralRccKernelClock::Mux(reference) = &mut rcc.kernel_clock {
                            sanitize_rcc_reference(reference);
                        }
                    }

                    if let Some(control) = &mut p.rcc_control {
                        sanitize_rcc_reference(&mut control.enable);
                        if let Some(reset) = &mut control.reset {
                            sanitize_rcc_reference(reset);
                        }
                        if let Some(key) = &mut control.enable_write_key {
                            sanitize_rcc_reference(&mut key.field);
                        }
                    }

                    if let Some(registers) = &mut p.registers {
                        registers.ir = format!(":ir_for:{}:", registers.kind);
                        // The vendor SVD starts with uppercase block names. Keep
                        // the static metadata reference identical to the sanitized
                        // block name emitted below by upstream chiptool.
                        let mut names = ir::IR::new();
                        names.blocks.insert(
                            registers.block.clone(),
                            ir::Block {
                                extends: None,
                                description: None,
                                items: Vec::new(),
                            },
                        );
                        transform::sanitize::Sanitize::default()
                            .run(&mut names)
                            .unwrap();
                        registers.block = names.blocks.into_keys().next().unwrap();
                    }
                }
            }

            // Generate
            for core in &chip.cores {
                let chip_core_name = match chip.cores.len() {
                    1 => chip_name.clone(),
                    _ => format!("{}-{}", chip_name, core.name),
                };

                chip_core_names.push(chip_core_name.clone());
                self.gen_chip(&chip_core_name, &chip, core)
            }
        }

        for (module, version) in &self.all_peripheral_versions {
            println!("loading {} {}", module, version);

            let regs_path = Path::new(&self.opts.data_dir)
                .join("registers")
                .join(&format!("{}_{}.json", module, version));

            let mut ir: ir::IR = serde_json::from_slice(&fs::read(regs_path).unwrap()).unwrap();
            let write_seeds = register_write::render(&self.opts.data_dir, module, version);

            transform::expand_extends::ExpandExtends {}
                .run(&mut ir)
                .unwrap();

            transform::map_names(&mut ir, |k, s| match k {
                transform::NameKind::Block => *s = s.to_string(),
                transform::NameKind::Fieldset => *s = format!("regs::{}", s),
                transform::NameKind::Enum => *s = format!("vals::{}", s),
                _ => {}
            });

            transform::sort::Sort {}.run(&mut ir).unwrap();
            transform::sanitize::Sanitize::default()
                .run(&mut ir)
                .unwrap();

            let items = generate::render(&ir, &gen_opts()).unwrap();
            let items =
                field_access::apply(&self.opts.data_dir, &format!("{module}_{version}"), items);
            let file = File::create(
                self.opts
                    .out_dir
                    .join("src/peripherals")
                    .join(format!("{}_{}.rs", module, version)),
            )
            .unwrap();

            let mut file = BufWriter::new(file);

            // Allow a few warning
            file.write_all(
                b"#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]",
            )
            .unwrap();

            let data = items.to_string().replace("] ", "]\n");

            // Remove inner attributes like #![no_std]
            let re = Regex::new("# *! *\\[.*\\]").unwrap();
            let data = re.replace_all(&data, "");
            file.write_all(data.as_bytes()).unwrap();
            file.write_all(write_seeds.as_bytes()).unwrap();

            let ir = crate::data::ir::IR::from_chiptool(ir);
            let mut data = String::new();

            write!(
                &mut data,
                "
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = {};
                ",
                stringify(&ir),
            )
            .unwrap();

            let mut file = File::create(
                self.opts
                    .out_dir
                    .join("src/registers")
                    .join(format!("{}_{}.rs", module, version)),
            )
            .unwrap();
            file.write_all(data.as_bytes()).unwrap();
        }

        // Ordinary module groups share versioned implementations. The build script
        // enables only the versions referenced by the selected chip.
        let modules = gen_peripheral_modules(&self.all_peripheral_versions);
        for group in ["peripherals", "registers"] {
            fs::write(
                self.opts.out_dir.join("src").join(group).join("mod.rs"),
                &modules,
            )
            .unwrap();
        }
        fs::write(
            self.opts.out_dir.join("chip_peripheral_versions.rs"),
            gen_chip_peripheral_versions(&self.chip_peripheral_versions),
        )
        .unwrap();

        // Generate Cargo.toml
        let mut contents = include_bytes!("../res/Cargo.toml").to_vec();
        for name in &chip_core_names {
            writeln!(&mut contents, "{} = []", name.to_ascii_lowercase()).unwrap();
        }
        // Only advertise groups whose complete package data is in this render.
        let mut groups = BTreeMap::<String, Vec<String>>::new();
        let group_dir = self.opts.data_dir.join("package-groups");
        if group_dir.exists() {
            for entry in fs::read_dir(group_dir).unwrap() {
                let path = entry.unwrap().path();
                let family: BTreeMap<String, Vec<String>> =
                    serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
                for (target, members) in family {
                    if members.iter().all(|name| self.opts.chips.contains(name)) {
                        groups.insert(target.to_ascii_lowercase(),
                            members.iter().map(|s| s.to_ascii_lowercase()).collect());
                    }
                }
            }
        }
        let mut table = String::from("pub const PACKAGE_GROUPS: &[(&str, &[&str])] = &[\n");
        for (target, members) in &groups {
            writeln!(&mut contents, "{target} = [{:?}]", members[0]).unwrap();
            writeln!(table, "({target:?}, &{members:?}),").unwrap();
        }
        table.push_str("];\n");
        fs::write(self.opts.out_dir.join("package_groups.rs"), table).unwrap();
        fs::write(self.opts.out_dir.join("Cargo.toml"), contents).unwrap();

        // Generate src/all_chips.rs
        {
            let contents = gen_all_chips(&self.opts.chips);
            fs::write(self.opts.out_dir.join("src/all_chips.rs"), contents).unwrap();
        }

        // Generate src/all_peripheral_versions.rs
        {
            let contents = gen_all_peripheral_versions(&self.all_peripheral_versions);
            fs::write(
                self.opts.out_dir.join("src/all_peripheral_versions.rs"),
                contents,
            )
            .unwrap();
        }

        // Copy upstream-shaped host smoke tests without hand-editing generated files.
        fs::create_dir_all(self.opts.out_dir.join("tests")).unwrap();
        fs::write(
            self.opts.out_dir.join("tests/f030.rs"),
            include_bytes!("../res/tests/f030.rs"),
        )
        .unwrap();
        fs::write(
            self.opts.out_dir.join("tests/metadata.rs"),
            include_bytes!("../res/tests/metadata.rs"),
        )
        .unwrap();

        fs::write(
            self.opts.out_dir.join("tests/uart_fields.rs"),
            include_bytes!("../res/tests/uart_fields.rs"),
        )
        .unwrap();

        // copy misc files
        fs::write(
            self.opts.out_dir.join("README.md"),
            include_bytes!("../res/README.md"),
        )
        .unwrap();
        fs::write(
            self.opts.out_dir.join("build.rs"),
            include_bytes!("../res/build.rs"),
        )
        .unwrap();
        fs::write(
            self.opts.out_dir.join("src/lib.rs"),
            include_bytes!("../res/src/lib.rs"),
        )
        .unwrap();
        fs::write(
            self.opts.out_dir.join("src/common.rs"),
            chiptool::generate::COMMON_MODULE,
        )
        .unwrap();
        fs::write(
            self.opts.out_dir.join("src/metadata.rs"),
            include_bytes!("../res/src/metadata.rs"),
        )
        .unwrap();
    }
}

/// Keep static RCC references in the same naming domain as the generated Rust
/// register IR. Chip JSON retains authored vendor identifiers. Use chiptool's
/// exact sanitizer (including keywords and digit boundaries), not hand casing.
fn sanitize_rcc_reference(reference: &mut PeripheralRccRegister) {
    let mut names: ir::IR = serde_json::from_value(serde_json::json!({
        "block/Control": {"items": [{"name": reference.register, "byte_offset": 0, "fieldset": "Fields"}]},
        "fieldset/Fields": {"fields": [{"name": reference.field, "bit_offset": 0, "bit_size": 1}]}
    })).unwrap();
    transform::sanitize::Sanitize::default()
        .run(&mut names)
        .unwrap();
    reference.register = names.blocks.values().next().unwrap().items[0].name.clone();
    reference.field = names.fieldsets.values().next().unwrap().fields[0]
        .name
        .clone();
}

#[cfg(test)]
mod reference_tests {
    use super::*;
    #[test]
    fn static_rcc_names_use_exact_chiptool_sanitizer() {
        for (register, field, expected_register, expected_field) in [
            ("APBEN2", "UART1", "apben2", "uart1"),
            ("CLK_CTRL", "TYPE", "clk_ctrl", "type_"),
        ] {
            let mut reference = PeripheralRccRegister {
                register: register.into(),
                field: field.into(),
            };
            sanitize_rcc_reference(&mut reference);
            assert_eq!(reference.register, expected_register);
            assert_eq!(reference.field, expected_field);
        }
    }

    #[test]
    fn shared_modules_use_sorted_ordinary_cfg_gated_declarations() {
        let versions = HashSet::from([
            ("spi".into(), "v2".into()),
            ("adc".into(), "v1".into()),
            ("spi".into(), "v1".into()),
        ]);
        assert_eq!(
            gen_peripheral_modules(&versions),
            "#[cfg(adc_v1)] pub mod adc_v1;\n#[cfg(spi_v1)] pub mod spi_v1;\n#[cfg(spi_v2)] pub mod spi_v2;\n",
        );
    }

    #[test]
    fn selected_chip_inventory_preserves_only_its_own_versions() {
        let chips = BTreeMap::from([
            ("cw32b".into(), BTreeSet::from(["spi_v2".into()])),
            (
                "cw32a".into(),
                BTreeSet::from(["spi_v1".into(), "adc_v1".into()]),
            ),
        ]);
        assert_eq!(
            gen_chip_peripheral_versions(&chips),
            "// Generated selected-chip IP inventory for build.rs.\npub const CHIP_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &[\n(\"cw32a\", &[\"adc_v1\",\"spi_v1\",]),\n(\"cw32b\", &[\"spi_v2\",]),\n];\n",
        );
    }
}

fn stringify<T: Debug>(metadata: T) -> String {
    let mut metadata = format!("{:#?}", metadata);
    if metadata.starts_with('[') {
        metadata = format!("&{}", metadata);
    }

    metadata.replace(": [", ": &[")
}

fn gen_opts() -> generate::Options {
    generate::Options::new()
        .with_common_module(CommonModule::External("crate::common".parse().unwrap()))
}

fn gen_all_chips(chips: &[String]) -> String {
    let mut contents = String::new();
    writeln!(&mut contents, "pub static ALL_CHIPS: &[&str] = &[").unwrap();
    for chip in chips.iter() {
        writeln!(&mut contents, "    {:?},", chip).unwrap();
    }
    writeln!(&mut contents, "];").unwrap();
    contents
}

fn gen_all_peripheral_versions(all_versions: &HashSet<(String, String)>) -> String {
    let mut version_map = BTreeMap::<_, BTreeSet<_>>::new();
    for (kind, version) in all_versions.iter() {
        version_map.entry(kind).or_default().insert(version);
    }

    let mut contents = String::new();
    writeln!(
        &mut contents,
        "pub static ALL_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &["
    )
    .unwrap();
    for (kind, versions) in version_map.iter() {
        write!(&mut contents, "    ({:?}, &[", kind).unwrap();
        for version in versions.iter() {
            write!(&mut contents, "{:?}, ", version).unwrap();
        }
        writeln!(&mut contents, "]),").unwrap();
    }
    writeln!(&mut contents, "];").unwrap();
    contents
}

/// Declare each shared IP version once, under its selected-chip cfg.
fn gen_peripheral_modules(all_versions: &HashSet<(String, String)>) -> String {
    let versions: BTreeSet<_> = all_versions.iter().collect();
    let mut contents = String::new();
    for (kind, version) in versions {
        writeln!(
            &mut contents,
            "#[cfg({kind}_{version})] pub mod {kind}_{version};"
        )
        .unwrap();
    }
    contents
}

/// Build-script input, deliberately independent of PAC and register-IR modules.
fn gen_chip_peripheral_versions(chips: &BTreeMap<String, BTreeSet<String>>) -> String {
    let mut contents = String::from(
        "// Generated selected-chip IP inventory for build.rs.\npub const CHIP_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &[\n",
    );
    for (chip, versions) in chips {
        write!(&mut contents, "({chip:?}, &[").unwrap();
        for version in versions {
            write!(&mut contents, "{version:?},").unwrap();
        }
        writeln!(&mut contents, "]),").unwrap();
    }
    contents.push_str("];\n");
    contents
}
