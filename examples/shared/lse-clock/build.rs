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
    assert!(matches!(
        metadata.name,
        "CW32F030C8T7"
            | "CW32A030C8T7"
            | "CW32F020C6U7"
            | "CW32L031C8T6"
            | "CW32L031C8U6"
            | "CW32L031F8U6"
            | "CW32R031C8U6"
            | "CW32L052C8T6"
            | "CW32L052R8S6"
            | "CW32L052R8T6"
            | "CW32L083RBT6"
            | "CW32L083RCT6"
            | "CW32L083RCS6"
            | "CW32L083MCT6"
            | "CW32L083VCT6"
    ));
    println!("cargo:rustc-check-cfg=cfg(gpio_has_speed)");
    let lse = metadata
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap()
        .clock_limits
        .as_ref()
        .unwrap()
        .lse_configuration
        .as_ref()
        .unwrap();
    if lse.gpio_speed_offset.is_some() {
        println!("cargo:rustc-cfg=gpio_has_speed");
    }
    println!("cargo:rustc-check-cfg=cfg(lse_startup_analog)");
    if lse
        .startup_consumers
        .as_ref()
        .is_some_and(|n| n.startup_analog)
    {
        println!("cargo:rustc-cfg=lse_startup_analog");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
