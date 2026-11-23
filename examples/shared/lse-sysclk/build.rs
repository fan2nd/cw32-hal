use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.memory.len(), 1, "select one exact package");
    assert!(metadata.peripherals.iter().any(|p| {
        p.clock_limits.as_ref().is_some_and(|c| {
            c.lse_configuration
                .as_ref()
                .is_some_and(|lse| lse.sysclk_detector.is_some())
        })
    }));
    assert!(matches!(
        metadata.name,
        "CW32F020C6U7"
            | "CW32F030C8T7"
            | "CW32A030C8T7"
            | "CW32L010F8P6"
            | "CW32L010F8U6"
            | "CW32L010Y8M6"
            | "CW32L011K8T6"
            | "CW32L011K8U6"
            | "CW32L012C8T6"
            | "CW32L012C8U6"
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
