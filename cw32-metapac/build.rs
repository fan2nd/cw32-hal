//! Consumer build: select generated Rust. No generator, YAML or JSON dependency.
use std::{env, path::PathBuf};
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
    let generated = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("generated");
    let chip = generated.join("chips").join(&chips[0]);
    println!("cargo:rerun-if-changed={}", generated.display());
    for (key, file, enabled) in [
        ("PAC", "pac.rs", true),
        (
            "METADATA",
            "metadata.rs",
            env::var_os("CARGO_FEATURE_METADATA").is_some(),
        ),
        ("RT", "rt.rs", env::var_os("CARGO_FEATURE_RT").is_some()),
    ] {
        if !enabled {
            continue;
        }
        let path = chip.join(file);
        assert!(path.is_file(), "missing generated PAC; run cargo run -p xtask -- regenerate from the workspace root before building");
        println!("cargo:rustc-env=CW32_METAPAC_{key}_PATH={}", path.display());
    }
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        assert!(chip.join("device.x").is_file(), "missing generated runtime; run cargo run -p xtask -- regenerate from the workspace root before building");
        println!("cargo:rustc-link-search={}", chip.display());
    }
}
