use cw32_metapac::metadata::Metadata;
use std::{env, fs, path::PathBuf};

fn check_spi_pins(metadata: &Metadata, instance: &str, pins: &[(&str, &str)]) {
    let mut peripherals = metadata.peripherals.iter().filter(|p| p.name == instance);
    let peripheral = peripherals.next().expect("missing example SPI controller");
    assert!(peripherals.next().is_none());
    for &(pin, signal) in pins {
        assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
        assert_eq!(
            peripheral
                .pins
                .iter()
                .filter(|p| p.pin == pin && p.signal == signal && p.af == Some(5))
                .count(),
            1,
            "example requires the selected package's bonded SPI signal at AF5"
        );
    }
}

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    let line = match metadata.name {
        "CW32F030C8T7" => "CW32F030",
        "CW32A030C8T7" => "CW32A030",
        "CW32F020C6U7" => "CW32F020",
        "CW32L031C8T6" => "CW32L031",
        "CW32L052C8T6" => "CW32L052",
        "CW32R031C8U6" => "CW32R031",

        "CW32L083RBT6" | "CW32L083RCS6" | "CW32L083RCT6" | "CW32L083MCT6"
        | "CW32L083VCT6" => "CW32L083",
        _ => panic!("select a qualified exact SPI DMA example package"),
    };
    assert_eq!(metadata.line, line);
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package with a qualified memory map"
    );
    check_spi_pins(
        metadata,
        "SPI1",
        &[("PA5", "SCK"), ("PA7", "MOSI"), ("PA6", "MISO")],
    );
    // Both binaries use a separately owned PB0 software chip select.
    assert_eq!(metadata.pins.iter().filter(|p| p.name == "PB0").count(), 1);
    let spi2_example = env::var_os("CARGO_FEATURE_SPI2_EXAMPLE").is_some();
    assert_eq!(
        spi2_example,
        matches!(metadata.line, "CW32F020" | "CW32L052" | "CW32L083"),
        "spi2-example is enabled only by eligible exact chip selections"
    );
    if spi2_example {
        check_spi_pins(
            metadata,
            "SPI2",
            &[("PA2", "SCK"), ("PA1", "MOSI"), ("PA0", "MISO")],
        );
    }
    let mut memory = String::from("MEMORY {\n");
    for bank in metadata.memory[0] {
        assert!(matches!(bank.name, "FLASH" | "RAM"));
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
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SPI2_EXAMPLE");
}
