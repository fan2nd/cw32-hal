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
    let adc = metadata
        .peripherals
        .iter()
        .find(|p| p.name == "ADC")
        .unwrap();
    let mut pins = adc.pins.iter().filter(|p| p.adc_mux.is_some());
    let first = pins.next().expect("missing qualified external ADC input");
    let second = pins
        .find(|p| p.pin != first.pin)
        .expect("need two bonded ADC inputs");
    let routes = format!(
        "{}: first={} {} mux{}, second={} {} mux{}\n",
        metadata.name,
        first.pin,
        first.signal,
        first.adc_mux.unwrap(),
        second.pin,
        second.signal,
        second.adc_mux.unwrap()
    );
    fs::write(out.join("analog-input-routes.txt"), routes).unwrap();
    fs::write(
        out.join("channel-pins.rs"),
        format!("(p.{}, p.{})", first.pin, second.pin),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
