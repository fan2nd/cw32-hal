use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert!(
        matches!(
            metadata.name,
            "CW32F030C8T7" | "CW32F030K8T7" | "CW32F030K8U7" | "CW32A030C8T7"
        ),
        "select one qualified exact MCO package"
    );
    assert_eq!(
        metadata.memory.len(),
        1,
        "select the exact package memory map"
    );
    let banks = metadata.memory[0];
    assert_eq!(banks.len(), 2);
    assert!(banks.iter().any(|bank| {
        bank.name == "FLASH" && bank.address == 0x0000_0000 && bank.size == 64 * 1024
    }));
    assert!(banks.iter().any(|bank| {
        bank.name == "RAM" && bank.address == 0x2000_0000 && bank.size == 8 * 1024
    }));
    let mut memory = String::from("MEMORY {\n");
    for bank in banks {
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
