use cw32_metapac::metadata::{MemoryRegionKind, Metadata};
use std::{env, fs, path::PathBuf};

fn check_route(
    metadata: &Metadata,
    name: &str,
    address: u64,
    irq: u32,
    scl: &str,
    sda: &str,
    af: u8,
) {
    let mut matches = metadata.peripherals.iter().filter(|p| p.name == name);
    let peripheral = matches.next().expect("missing example I2C controller");
    assert!(matches.next().is_none(), "duplicate example I2C controller");
    assert_eq!(peripheral.address, address);
    let registers = peripheral.registers.as_ref().unwrap();
    assert_eq!(registers.kind, "i2c");
    assert_eq!(registers.version, "cw32l012_v1");
    assert_eq!(registers.block, "I2c");
    assert_eq!(peripheral.interrupts.len(), 1);
    assert_eq!(peripheral.interrupts[0].signal, "GLOBAL");
    assert_eq!(peripheral.interrupts[0].interrupt, name);
    assert_eq!(
        metadata
            .interrupts
            .iter()
            .filter(|i| i.name == name && i.number == irq)
            .count(),
        1,
        "the example must use its own dedicated interrupt"
    );
    for (pin, signal) in [(scl, "SCL"), (sda, "SDA")] {
        assert!(!matches!(pin, "PA13" | "PA14"));
        assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
        assert_eq!(
            peripheral
                .pins
                .iter()
                .filter(|p| p.pin == pin && p.signal == signal && p.af == Some(af))
                .count(),
            1,
            "the selected package must expose the exact signal and AF"
        );
    }
}

fn main() {
    // These exact packages are qualified by the L012 datasheet and generated
    // part metadata. The family alias has no exact package or memory map.
    let chip_features = [
        ("CARGO_FEATURE_CW32L012C8T6", "CW32L012C8T6", "LQFP48"),
        ("CARGO_FEATURE_CW32L012C8U6", "CW32L012C8U6", "QFN48"),
    ];
    let selected: Vec<_> = chip_features
        .iter()
        .filter(|(feature, _, _)| env::var_os(feature).is_some())
        .collect();
    assert_eq!(
        selected.len(),
        1,
        "select exactly one qualified exact chip feature"
    );
    let (_, chip, package) = *selected[0];
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.name, chip,
        "selected {package} part must match its metadata"
    );
    assert_eq!(metadata.line, "CW32L012");
    check_route(metadata, "I2C1", 0x4000_5800, 23, "PA4", "PA5", 2);
    check_route(metadata, "I2C2", 0x4000_5c00, 24, "PB0", "PB1", 8);

    // Derive the linker regions from the exact selected part, then require its
    // documented 64 KiB Flash and 8 KiB RAM. No generic or enlarged fallback.
    assert_eq!(metadata.memory.len(), 1);
    let banks = metadata.memory[0];
    assert_eq!(banks.len(), 2);
    assert_eq!(banks.iter().filter(|bank| bank.name == "FLASH").count(), 1);
    assert_eq!(banks.iter().filter(|bank| bank.name == "RAM").count(), 1);
    let mut memory = String::from("MEMORY {\n");
    for bank in banks {
        match bank.name {
            "FLASH" => {
                assert_eq!(bank.kind, MemoryRegionKind::Flash);
                assert_eq!((bank.address, bank.size), (0, 64 * 1024));
            }
            "RAM" => {
                assert_eq!(bank.kind, MemoryRegionKind::Ram);
                assert_eq!((bank.address, bank.size), (0x2000_0000, 8 * 1024));
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
    for (feature, _, _) in chip_features {
        println!("cargo:rerun-if-env-changed={feature}");
    }
}
