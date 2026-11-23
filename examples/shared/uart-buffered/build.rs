use std::{env, fs, path::PathBuf};

fn main() {
    if env::var_os("CARGO_FEATURE_IDLE_NOTIFY").is_some() {
        assert!(
            [
                "CARGO_FEATURE_CW32L010F8P6",
                "CARGO_FEATURE_CW32L011K8T6",
                "CARGO_FEATURE_CW32L012C8T6",
            ]
            .iter()
            .any(|feature| env::var_os(feature).is_some()),
            "idle-notify requires a native L010, L011 or L012 package"
        );
    }
    let chip_features = [
        "CARGO_FEATURE_CW32F030C8T7",
        "CARGO_FEATURE_CW32F002F3P7",
        "CARGO_FEATURE_CW32L031C8T6",
        "CARGO_FEATURE_CW32L052C8T6",
        "CARGO_FEATURE_CW32L083RCT6",
        "CARGO_FEATURE_CW32L010F8P6",
        "CARGO_FEATURE_CW32L011K8T6",
        "CARGO_FEATURE_CW32L012C8T6",
    ];
    assert_eq!(
        chip_features
            .iter()
            .filter(|feature| env::var_os(feature).is_some())
            .count(),
        1,
        "select exactly one supported chip feature"
    );

    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package with a qualified memory map"
    );
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
}
