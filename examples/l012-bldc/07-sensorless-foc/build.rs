fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEFMT_LOG");
    // 从工作区根目录构建时也默认显示诊断；尊重调用方显式过滤设置。
    if std::env::var_os("DEFMT_LOG").is_none() {
        println!("cargo:rustc-env=DEFMT_LOG=info");
    }
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
