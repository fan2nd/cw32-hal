use cw32_metapac::metadata::{MemoryRegionKind, Metadata};
use std::{collections::HashSet, env, fs, path::PathBuf};

struct Route {
    name: &'static str,
    address: u64,
    irq: &'static str,
    number: u32,
    sck: &'static str,
    mosi: &'static str,
    miso: &'static str,
    af: u8,
    cs: &'static str,
}

const OLD_SPI1: Route = Route {
    name: "SPI1",
    address: 0x4001_3000,
    irq: "SPI1",
    number: 25,
    sck: "PA5",
    mosi: "PA7",
    miso: "PA6",
    af: 5,
    cs: "PB0",
};
const OLD_SPI2: Route = Route {
    name: "SPI2",
    address: 0x4000_3800,
    irq: "SPI2",
    number: 26,
    sck: "PA2",
    mosi: "PA1",
    miso: "PA0",
    af: 5,
    cs: "PB1",
};
const SMALL_SPI: Route = Route {
    name: "SPI",
    address: 0x4001_3000,
    irq: "SPI",
    number: 25,
    sck: "PA0",
    mosi: "PA1",
    miso: "PA4",
    af: 3,
    cs: "PB0",
};
const L010_SPI: Route = Route {
    name: "SPI",
    address: 0x4000_0800,
    irq: "SPI",
    number: 25,
    sck: "PA5",
    mosi: "PA6",
    miso: "PA3",
    af: 3,
    cs: "PA4",
};
const L011_SPI: Route = Route {
    name: "SPI",
    address: 0x4000_0800,
    irq: "SPI",
    number: 25,
    sck: "PA5",
    mosi: "PA7",
    miso: "PA6",
    af: 3,
    cs: "PB0",
};
const L012_SPI1: Route = Route {
    name: "SPI1",
    address: 0x4000_0800,
    irq: "SPI1",
    number: 25,
    sck: "PA5",
    mosi: "PA7",
    miso: "PA6",
    af: 3,
    cs: "PB0",
};
const L012_SPI2: Route = Route {
    name: "SPI2",
    address: 0x4000_0400,
    irq: "SPI23",
    number: 26,
    sck: "PA2",
    mosi: "PA1",
    miso: "PA0",
    af: 8,
    cs: "PB1",
};
const L012_SPI3: Route = Route {
    name: "SPI3",
    address: 0x4000_3400,
    irq: "SPI23",
    number: 26,
    sck: "PA9",
    mosi: "PA11",
    miso: "PA10",
    af: 8,
    cs: "PB2",
};

// Exact selected-package facts. Identical route constants do not qualify another
// family: every row has its own line/version/name check and its own memory sizes.
struct Selection {
    feature: &'static str,
    name: &'static str,
    line: &'static str,
    version: &'static str,
    flash: u32,
    ram: u32,
    routes: &'static [Route],
}

const SELECTIONS: &[Selection] = &[
    Selection {
        feature: "CARGO_FEATURE_CW32F020C6U7",
        name: "CW32F020C6U7",
        line: "CW32F020",
        version: "v1",
        flash: 32768,
        ram: 8192,
        routes: &[OLD_SPI1, OLD_SPI2],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32F002F3P7",
        name: "CW32F002F3P7",
        line: "CW32F002",
        version: "cw32f002_v1",
        flash: 16384,
        ram: 2048,
        routes: &[SMALL_SPI],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32F003E4P7",
        name: "CW32F003E4P7",
        line: "CW32F003",
        version: "cw32f002_v1",
        flash: 20480,
        ram: 3072,
        routes: &[SMALL_SPI],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L010F8P6",
        name: "CW32L010F8P6",
        line: "CW32L010",
        version: "cw32l010_v1",
        flash: 65536,
        ram: 4096,
        routes: &[L010_SPI],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L011K8T6",
        name: "CW32L011K8T6",
        line: "CW32L011",
        version: "cw32l010_v1",
        flash: 65536,
        ram: 6144,
        routes: &[L011_SPI],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L012C8T6",
        name: "CW32L012C8T6",
        line: "CW32L012",
        version: "cw32l012_v1",
        flash: 65536,
        ram: 8192,
        routes: &[L012_SPI1, L012_SPI2, L012_SPI3],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L012C8U6",
        name: "CW32L012C8U6",
        line: "CW32L012",
        version: "cw32l012_v1",
        flash: 65536,
        ram: 8192,
        routes: &[L012_SPI1, L012_SPI2, L012_SPI3],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L031C8T6",
        name: "CW32L031C8T6",
        line: "CW32L031",
        version: "cw32l031_v1",
        flash: 65536,
        ram: 8192,
        routes: &[OLD_SPI1],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32R031C8U6",
        name: "CW32R031C8U6",
        line: "CW32R031",
        version: "cw32l031_v1",
        flash: 65536,
        ram: 8192,
        routes: &[OLD_SPI1],
    },
    Selection {
        feature: "CARGO_FEATURE_CW32L052C8T6",
        name: "CW32L052C8T6",
        line: "CW32L052",
        version: "cw32l031_v1",
        flash: 65536,
        ram: 8192,
        routes: &[OLD_SPI1, OLD_SPI2],
    },
];

fn check_pin(metadata: &Metadata, pin: &str) {
    assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
    // Stronger than the HAL's safe_pin: these examples also avoid oscillator,
    // BOOT and reset aliases. Physical package tables are cited in README.md.
    let forbidden: &[&str] = match metadata.line {
        "CW32F002" | "CW32F003" => &["PA2", "PA5", "PC5"],
        "CW32L010" => &["PA7", "PA8", "PB7", "PA0", "PA1", "PB0", "PB1"],
        "CW32L011" => &["PA13", "PA14", "PC13", "PB7", "PC14", "PC15"],
        _ => &["PA13", "PA14", "PF3", "PF0", "PF1", "PC14", "PC15"],
    };
    assert!(
        !forbidden.contains(&pin),
        "example must preserve special pads"
    );
}

fn check_route(metadata: &Metadata, selected: &Selection, route: &Route) {
    let mut found = metadata.peripherals.iter().filter(|p| p.name == route.name);
    let peripheral = found.next().expect("missing example SPI controller");
    assert!(found.next().is_none());
    assert_eq!(peripheral.address, route.address);
    let registers = peripheral.registers.as_ref().unwrap();
    assert_eq!(
        (registers.kind, registers.version, registers.block),
        ("spi", selected.version, "Spi")
    );
    assert_eq!(peripheral.interrupts.len(), 1);
    assert_eq!(peripheral.interrupts[0].signal, "GLOBAL");
    assert_eq!(peripheral.interrupts[0].interrupt, route.irq);
    assert_eq!(
        metadata
            .interrupts
            .iter()
            .filter(|i| i.name == route.irq && i.number == route.number)
            .count(),
        1
    );
    let owners: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.interrupts.iter().any(|i| i.interrupt == route.irq))
        .map(|p| p.name)
        .collect();
    if route.irq == "SPI23" {
        assert_eq!(metadata.line, "CW32L012");
        assert_eq!(owners, ["SPI2", "SPI3"]);
    } else {
        assert_eq!(owners, [route.name]);
    }
    for (pin, signal) in [
        (route.sck, "SCK"),
        (route.mosi, "MOSI"),
        (route.miso, "MISO"),
    ] {
        check_pin(metadata, pin);
        assert_eq!(
            peripheral
                .pins
                .iter()
                .filter(|p| p.pin == pin && p.signal == signal && p.af == Some(route.af))
                .count(),
            1
        );
    }
    check_pin(metadata, route.cs);
}

fn main() {
    let selected: Vec<_> = SELECTIONS
        .iter()
        .filter(|s| env::var_os(s.feature).is_some())
        .collect();
    assert_eq!(
        selected.len(),
        1,
        "select exactly one qualified exact chip feature"
    );
    let selected = selected[0];
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.name, selected.name);
    assert_eq!(metadata.line, selected.line);
    assert_eq!(
        metadata
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "spi"))
            .count(),
        selected.routes.len()
    );
    let mut pins = HashSet::new();
    for route in selected.routes {
        check_route(metadata, selected, route);
        for pin in [route.sck, route.mosi, route.miso, route.cs] {
            assert!(pins.insert(pin), "example buses must own separate pins");
        }
    }

    // Derive only the selected exact package's regions, then verify its own
    // documented size. A family alias or larger backend map is never a fallback.
    assert_eq!(metadata.memory.len(), 1);
    let banks = metadata.memory[0];
    assert_eq!(banks.len(), 2);
    assert_eq!(banks.iter().filter(|b| b.name == "FLASH").count(), 1);
    assert_eq!(banks.iter().filter(|b| b.name == "RAM").count(), 1);
    let mut memory = String::from("MEMORY {\n");
    for bank in banks {
        match bank.name {
            "FLASH" => {
                assert_eq!(bank.kind, MemoryRegionKind::Flash);
                assert_eq!((bank.address, bank.size), (0, selected.flash));
            }
            "RAM" => {
                assert_eq!(bank.kind, MemoryRegionKind::Ram);
                assert_eq!((bank.address, bank.size), (0x2000_0000, selected.ram));
            }
            _ => panic!("unexpected memory region"),
        }
        memory.push_str(&format!(
            "  {} : ORIGIN = {:#x}, LENGTH = {}\n",
            bank.name, bank.address, bank.size
        ));
    }
    memory.push_str("}\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), memory).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rerun-if-changed=build.rs");
    for selection in SELECTIONS {
        println!("cargo:rerun-if-env-changed={}", selection.feature);
    }
}
