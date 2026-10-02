//! Consumer build: select audited, pre-generated artifacts. No YAML/JSON parsing.
use std::{env, fs, path::PathBuf};
fn main() {
    let chips: Vec<_> = env::vars()
        .filter_map(|(name, _)| {
            name.strip_prefix("CARGO_FEATURE_CW32")
                .map(|suffix| format!("cw32{}", suffix.to_ascii_lowercase()))
        })
        .collect();
    assert_eq!(
        chips.len(),
        1,
        "select exactly one supported CW32 chip feature"
    );
    let source = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("src/chips")
        .join(&chips[0]);
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    for file in ["pac.rs", "metadata.rs"] {
        println!("cargo:rerun-if-changed={}", source.join(file).display());
        fs::copy(source.join(file), out.join(file)).expect(
            "missing generated PAC; run cargo run -p xtask -- regenerate from the workspace root before building",
        );
    }
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        for file in ["rt.rs", "device.x"] {
            println!("cargo:rerun-if-changed={}", source.join(file).display());
            fs::copy(source.join(file), out.join(file)).expect(
                "missing generated runtime; run cargo run -p xtask -- regenerate from the workspace root before building",
            );
        }
        println!("cargo:rustc-link-search={}", out.display());
    }
}
