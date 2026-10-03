//! Pure generation of associations; the tables never install hardware handlers.
use cw32_metapac::metadata::Metadata;
use std::{collections::BTreeSet, fmt::Write, string::String};

pub fn associations(md: &Metadata) -> String {
    let mut out = String::from(
        "// Generated from cw32-metapac metadata. No driver or ISR registration is implied.\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct InterruptBinding { pub peripheral: &'static str, pub signal: &'static str, pub interrupt: &'static str, pub number: u16 }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct PinRoute { pub pin: &'static str, pub peripheral: &'static str, pub signal: &'static str, pub af: Option<u8>, pub remap: Option<&'static str> }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct Quirk { pub peripheral: Option<&'static str>, pub name: &'static str, pub description: &'static str, pub source: &'static str }\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct Remap { pub name: &'static str, pub peripheral: &'static str, pub description: &'static str }\n\
         pub const INTERRUPT_BINDINGS: &[InterruptBinding] = &[\n",
    );
    let mut seen = BTreeSet::new();
    for binding in md.interrupt_bindings {
        assert!(
            seen.insert((binding.peripheral, binding.signal, binding.interrupt)),
            "duplicate peripheral/signal/vector association"
        );
        // Only the complete triple is unique: many signals/peripherals may
        // share one vector, and one signal may refer to multiple vectors.
        let irq = md
            .interrupts
            .iter()
            .find(|i| i.name == binding.interrupt)
            .expect("interrupt association references an unknown vector");
        writeln!(out, "    InterruptBinding {{ peripheral: {:?}, signal: {:?}, interrupt: {:?}, number: {} }},",
                 binding.peripheral, binding.signal, binding.interrupt, irq.number).unwrap();
    }
    out.push_str("];\npub const PIN_ROUTES: &[PinRoute] = &[\n");
    for route in md.pin_routes {
        writeln!(
            out,
            "    PinRoute {{ pin: {:?}, peripheral: {:?}, signal: {:?}, af: {:?}, remap: {:?} }},",
            route.pin, route.peripheral, route.signal, route.af, route.remap
        )
        .unwrap();
    }
    out.push_str("];\npub const QUIRKS: &[Quirk] = &[\n");
    for quirk in md.quirks {
        writeln!(
            out,
            "    Quirk {{ peripheral: {:?}, name: {:?}, description: {:?}, source: {:?} }},",
            quirk.peripheral, quirk.name, quirk.description, quirk.source
        )
        .unwrap();
    }
    out.push_str("];\npub const REMAPS: &[Remap] = &[\n");
    for remap in md.remaps {
        writeln!(
            out,
            "    Remap {{ name: {:?}, peripheral: {:?}, description: {:?} }},",
            remap.name, remap.peripheral, remap.description
        )
        .unwrap();
    }
    out.push_str("];\n");
    out
}

/// Driver contracts supported by this HAL, not a map from chip names to families.
/// Every entry is selected independently using the peripheral's register metadata.
const DRIVER_IPS: &[(&str, &[&str])] = &[
    ("gpio", &["l012", "f030"]),
    ("sysctrl", &["l012", "f030"]),
    ("gtim", &["l012", "f030"]),
    ("adc", &["l012", "f030"]),
    ("atim", &["l012", "f030"]),
    ("cordic", &["l012"]),
    ("eau", &["l012"]),
    ("opa", &["l012"]),
    ("vc", &["l012", "f030"]),
    ("vcref", &["l012"]),
    ("dac", &["l012"]),
    ("bgr", &["l012"]),
];
const GPIO_CAPABILITIES: &[&str] = &[
    "gpio_has_speed",
    "gpio_has_drive_strength",
    "gpio_has_level_interrupts",
    "gpio_pulldown_indexed",
];

pub fn field<'a>(
    p: &'a cw32_metapac::metadata::Peripheral,
    register: &str,
    name: &str,
) -> &'a cw32_metapac::metadata::RegisterField {
    p.registers
        .iter()
        .find(|r| r.name == register)
        .and_then(|r| r.fields.iter().find(|f| f.name == name))
        .expect("driver requires the audited register field")
}

fn gpio_capabilities(p: &cw32_metapac::metadata::Peripheral) -> BTreeSet<&'static str> {
    use cw32_metapac::metadata::{Access, ReadBehavior, WriteBehavior};
    let mut result = BTreeSet::new();
    let indexed = |name: &str| {
        let Some(r) = p.registers.iter().find(|r| r.name == name) else {
            return false;
        };
        assert!(
            matches!(r.access, Access::ReadWrite)
                && matches!(r.write_behavior, WriteBehavior::Ordinary)
                && matches!(r.read_behavior, ReadBehavior::Ordinary)
                && r.bit_size == 32
                && r.array.is_none(),
            "GPIO capability requires an ordinary scalar RW register"
        );
        let f = field(p, name, "PIN");
        assert!(
            f.bit_offset == 0
                && f.bit_size == 1
                && matches!(f.access, Access::ReadWrite)
                && f.array.is_some_and(|a| a.len == 16 && a.stride == 1),
            "GPIO capability requires the audited indexed PIN layout"
        );
        true
    };
    if indexed("SPEED") {
        result.insert("gpio_has_speed");
    }
    if indexed("DRIVER") {
        result.insert("gpio_has_drive_strength");
    }
    let high = indexed("HIGHIE");
    let low = indexed("LOWIE");
    assert_eq!(
        high, low,
        "GPIO level interrupt capability requires both polarities"
    );
    if high {
        result.insert("gpio_has_level_interrupts");
    }
    // Per-pin masks still decide which actual pins can use this register.
    // Do not treat a register layout as evidence of package bonding.
    if p.registers
        .iter()
        .find(|r| r.name == "PDR")
        .is_some_and(|r| r.fields.iter().any(|f| f.name == "PIN"))
    {
        assert!(indexed("PDR"));
        result.insert("gpio_pulldown_indexed");
    } else {
        let r = p.registers.iter().find(|r| r.name == "PDR").unwrap();
        let f = field(p, "PDR", "PIN3");
        assert!(
            matches!(r.access, Access::ReadWrite)
                && matches!(r.write_behavior, WriteBehavior::Ordinary)
                && matches!(r.read_behavior, ReadBehavior::Ordinary)
                && r.bit_size == 32
                && r.array.is_none()
                && matches!(f.access, Access::ReadWrite)
                && f.bit_offset == 3
                && f.bit_size == 1
                && f.array.is_none(),
            "unaudited scalar pull-down layout"
        );
    }
    result
}

/// Pure metadata selection also used by external mixed/absent-IP validation.
pub fn driver_cfgs(md: &Metadata) -> BTreeSet<String> {
    let mut enabled = BTreeSet::new();
    let mut gpio = None;
    for p in md.peripherals {
        if let Some((_, versions)) = DRIVER_IPS.iter().find(|(kind, _)| *kind == p.block) {
            assert!(
                versions.contains(&p.version),
                "unsupported HAL IP contract: {}_{}",
                p.block,
                p.version
            );
        }
        enabled.insert(p.block.to_owned());
        enabled.insert(format!("{}_{}", p.block, p.version));
        enabled.insert(format!("peri_{}", p.name.to_ascii_lowercase()));
        if p.block == "gpio" {
            let caps = gpio_capabilities(p);
            if let Some(previous) = &gpio {
                assert_eq!(
                    previous, &caps,
                    "heterogeneous GPIO capabilities need per-instance drivers"
                );
            }
            gpio = Some(caps);
        }
    }
    for cap in gpio.into_iter().flatten() {
        enabled.insert(cap.to_owned());
    }
    enabled
}

pub fn emit_driver_cfgs(md: &Metadata) {
    let mut declared = BTreeSet::new();
    for (kind, versions) in DRIVER_IPS {
        declared.insert((*kind).to_owned());
        for version in *versions {
            declared.insert(format!("{kind}_{version}"));
        }
    }
    declared.extend(GPIO_CAPABILITIES.iter().map(|c| (*c).to_owned()));
    // Only instance predicates explicitly consumed by hand-written drivers need
    // declaration when absent. All selected metadata instances are declared below.
    declared.extend(["peri_adc1", "peri_adc2", "peri_gtim1"].map(str::to_owned));
    let enabled = driver_cfgs(md);
    declared.extend(enabled.iter().cloned());
    for cfg in declared {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }
    for cfg in enabled {
        println!("cargo:rustc-cfg={cfg}");
    }
}
