fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEFMT_LOG");
    // Keep diagnostics visible even when building from the workspace root.
    // An explicit caller filter (including "off") still takes precedence.
    if std::env::var_os("DEFMT_LOG").is_none() {
        println!("cargo:rustc-env=DEFMT_LOG=info");
    }
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
