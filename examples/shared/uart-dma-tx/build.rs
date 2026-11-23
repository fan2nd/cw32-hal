use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package with a qualified memory map"
    );
    println!("cargo:rustc-check-cfg=cfg(example_l083)");
    let tx_pin = match metadata.line {
        "CW32F030" | "CW32A030" => "PA2",
        "CW32L083" => {
            println!("cargo:rustc-cfg=example_l083");
            "PA6"
        }
        _ => panic!("select a qualified exact x030 or L083 package"),
    };
    assert!(metadata.pins.iter().any(|pin| pin.name == tx_pin));
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
