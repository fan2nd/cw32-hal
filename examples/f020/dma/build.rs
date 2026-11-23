use std::{env, fs, path::PathBuf};

fn main() {
    let chip_features = [
        ("CARGO_FEATURE_CW32F020C6U7", "CW32F020C6U7"),
        ("CARGO_FEATURE_CW32F020K6U7", "CW32F020K6U7"),
        ("CARGO_FEATURE_CW32F020F6U7", "CW32F020F6U7"),
    ];
    let selected: Vec<_> = chip_features
        .iter()
        .filter(|(feature, _)| env::var_os(feature).is_some())
        .collect();
    assert_eq!(
        selected.len(),
        1,
        "select exactly one exact F020 chip feature"
    );

    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.name, selected[0].1);
    assert_eq!(metadata.line, "CW32F020");
    assert_eq!(
        metadata.memory.len(),
        1,
        "the selected exact package must have a qualified memory map"
    );
    assert_eq!(metadata.memory[0].len(), 2);
    let tx_pins = match metadata.name {
        "CW32F020C6U7" | "CW32F020K6U7" => ["PA8", "PA2"],
        "CW32F020F6U7" => ["PA6", "PA2"],
        _ => unreachable!(),
    };
    for tx_pin in tx_pins {
        assert!(metadata.pins.iter().any(|pin| pin.name == tx_pin));
    }

    // Linker ranges come from the selected exact part, never a generic map.
    let mut memory = String::from("MEMORY {\n");
    for bank in metadata.memory[0] {
        match bank.name {
            "FLASH" => assert_eq!((bank.address, bank.size), (0, 32 * 1024)),
            "RAM" => assert_eq!((bank.address, bank.size), (0x2000_0000, 8 * 1024)),
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
}
