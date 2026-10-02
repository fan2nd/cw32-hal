//! Explicit host tools. Does not depend on the consumer PAC or HAL.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct Temp(PathBuf);
impl Temp {
    fn new() -> Result<Self> {
        let root = std::env::temp_dir().join(format!(
            "cw32-xtask-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        fs::create_dir(&root)?;
        Ok(Self(root))
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn files(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    fn visit(root: &Path, dir: &Path, result: &mut BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
        for e in fs::read_dir(dir)? {
            let p = e?.path();
            if p.is_dir() {
                visit(root, &p, result)?;
            } else {
                result.insert(p.strip_prefix(root)?.to_owned(), fs::read(&p)?);
            }
        }
        Ok(())
    }
    let mut result = BTreeMap::new();
    if root.exists() {
        visit(root, root, &mut result)?;
    }
    Ok(result)
}
fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    for (name, bytes) in files(src)? {
        let p = dst.join(name);
        fs::create_dir_all(p.parent().unwrap())?;
        fs::write(p, bytes)?;
    }
    Ok(())
}
fn regenerate(root: &Path, check: bool) -> Result<()> {
    let temp = Temp::new()?;
    let data = temp.0.join("data");
    let pac = temp.0.join("chips");
    cw32_gen::generate(&root.join("cw32-data"), "all", &data, &pac)?;
    for (src, dst) in [
        (data, root.join("generated-data")),
        (pac, root.join("cw32-metapac/src/chips")),
    ] {
        if check {
            let expected = files(&src)?;
            let actual = files(&dst)?;
            if expected != actual {
                return Err(format!("Generated output drift or missing artifacts in {}. Run cargo run -p xtask -- regenerate", dst.display()).into());
            }
        } else {
            if dst.exists() {
                fs::remove_dir_all(&dst)?;
            }
            copy_tree(&src, &dst)?;
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [command] if command == "regenerate" => regenerate(root, false)?,
        [command, flag] if command == "regenerate" && flag == "--check" => {
            regenerate(root, true)?;
        }
        _ => return Err("usage: cargo run -p xtask -- regenerate [--check]".into()),
    }
    println!(
        "JSON/PAC/metadata {}",
        if args.len() == 2 {
            "match YAML byte-for-byte"
        } else {
            "generated from YAML"
        }
    );
    Ok(())
}
