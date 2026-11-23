use std::{env, fs, path::PathBuf};
fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.memory.len(), 1, "select one exact package");
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
    println!("cargo:rustc-check-cfg=cfg(gpio_speed)");
    println!("cargo:rustc-check-cfg=cfg(classic_pll)");
    if matches!(metadata.line, "CW32F020" | "CW32F030" | "CW32A030") {
        println!("cargo:rustc-cfg=classic_pll");
    }
    let gpio = metadata
        .peripherals
        .iter()
        .find(|p| p.name == "GPIOB")
        .unwrap();
    if gpio.registers.as_ref().unwrap().version == "v1" {
        println!("cargo:rustc-cfg=gpio_speed");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
