use std::{env, fs, path::PathBuf};

fn main() {
    let chip = &cw32_metapac::metadata::METADATA;
    assert_eq!(chip.memory.len(), 1, "select an exact part");
    let memory = chip.memory[0];
    let flash = memory.iter().find(|m| m.name == "FLASH").unwrap();
    let ram = memory.iter().find(|m| m.name == "RAM").unwrap();
    assert_eq!(flash.address, 0);
    // L01x reserves the whole final erase page containing the SLIB descriptor.
    // It must never be a storage write or page-erase target, even without SLIB.
    let descriptor_bytes = if matches!(chip.line, "CW32L010" | "CW32L011" | "CW32L012") {
        512
    } else {
        0
    };
    let storage_start = flash.size.checked_sub(descriptor_bytes + 4096).unwrap();
    assert_eq!(storage_start % 512, 0);
    let layout = format!(
        "MEMORY {{\n FLASH : ORIGIN = 0, LENGTH = {storage_start}\n STORAGE : ORIGIN = {storage_start}, LENGTH = 4096\n RAM : ORIGIN = {}, LENGTH = {}\n}}\n__storage_start = ORIGIN(STORAGE);\n__storage_end = ORIGIN(STORAGE) + LENGTH(STORAGE);\n",
        ram.address, ram.size
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), layout).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
}
