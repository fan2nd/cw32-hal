use std::{env, fs, path::PathBuf};
fn main() {
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
    let expected = matches!(metadata.line, "CW32L010" | "CW32L011" | "CW32L012");
    assert_eq!(
        std::env::var_os("CARGO_FEATURE_HSI_SOURCE").is_some(),
        expected
    );
    println!("cargo:rustc-check-cfg=cfg(gpio_speed)");
    if metadata
        .peripherals
        .iter()
        .find(|p| p.name == "GPIOA")
        .unwrap()
        .registers
        .as_ref()
        .unwrap()
        .version
        == "v1"
    {
        println!("cargo:rustc-cfg=gpio_speed");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
