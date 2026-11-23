use std::{env, fs, path::PathBuf};
fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.memory.len(), 1, "select an exact package");
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
    let reserved = env::var_os("CARGO_FEATURE_TIME_DRIVER").is_some();
    let (timer, ch1, ch2) = metadata
        .peripherals
        .iter()
        .filter(|t| t.classic_timer_input.is_some() && !(reserved && t.name == "GTIM1"))
        .find_map(|timer| {
            for ch1 in timer.pins.iter().filter(|p| p.signal == "CAP1") {
                if let Some(ch2) = timer
                    .pins
                    .iter()
                    .find(|p| p.signal == "CAP2" && p.pin != ch1.pin)
                {
                    return Some((timer, ch1, ch2));
                }
            }
            None
        })
        .expect("no owned timer with two qualified external phase inputs");
    let routes = format!(
        "{}: {} CAP1={} AF{}, CAP2={} AF{}\n",
        metadata.name,
        timer.name,
        ch1.pin,
        ch1.af.unwrap(),
        ch2.pin,
        ch2.af.unwrap()
    );
    fs::write(out.join("external-input-routes.txt"), routes).unwrap();
    let capture = format!(
        "InputCapture::new(p.{}, Some(CaptureInput::from_pin(p.{}, Pull::None)), Some(CaptureInput::from_pin(p.{}, Pull::None)), None, None, Hertz(1_000_000), CountingMode::EdgeAlignedUp).unwrap()",
        timer.name, ch1.pin, ch2.pin
    );
    let qei = format!(
        "Qei::new(p.{}, p.{}, p.{}, Config::default()).unwrap()",
        timer.name, ch1.pin, ch2.pin
    );
    fs::write(out.join("capture-constructor.rs"), capture).unwrap();
    fs::write(out.join("qei-constructor.rs"), qei).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
