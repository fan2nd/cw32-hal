use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(cw32_metapac::FIRMWARE_TARGET, "cw32l012c8");
    assert_eq!(metadata.memory.len(), 1);
    let mut memory = String::from("MEMORY {\n");
    for bank in metadata.memory[0] {
        assert!(matches!(bank.name, "FLASH" | "RAM"));
        // L012's final 512-byte page holds protection descriptors.
        // Reserve the preceding page for the calibration shared by all images.
        let size = if bank.name == "FLASH" {
            bank.size - 1024
        } else {
            bank.size
        };
        memory.push_str(&format!(
            "  {} : ORIGIN = {:#x}, LENGTH = {}\n",
            bank.name, bank.address, size
        ));
    }
    memory.push_str("}\n__foc_calibration = ORIGIN(FLASH) + LENGTH(FLASH);\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), memory).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    let log_level = if env::var("CARGO_PKG_NAME")
        .unwrap()
        .ends_with("algorithm-timing")
    {
        "info"
    } else {
        "trace"
    };
    println!("cargo:rustc-env=DEFMT_LOG={log_level}");
    println!("cargo:rerun-if-changed=../firmware-build.rs");
}
