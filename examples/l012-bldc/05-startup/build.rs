fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEFMT_LOG");
    // 即使从工作区根目录构建，也保持诊断日志可见。
    // 调用方显式设置的过滤条件（包括 "off"）仍优先。
    if std::env::var_os("DEFMT_LOG").is_none() {
        println!("cargo:rustc-env=DEFMT_LOG=info");
    }
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
