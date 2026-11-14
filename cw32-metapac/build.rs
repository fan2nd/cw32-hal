use std::collections::BTreeSet;
use std::env;

mod chip_peripheral_versions;
use chip_peripheral_versions::CHIP_PERIPHERAL_VERSIONS;
#[cfg(feature = "rt")]
use std::path::PathBuf;

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}

fn main() {
    #[cfg(feature = "rt")]
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let chip_core_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_CW32"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No cw32xx Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple cw32xx Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase()
    .replace('_', "-");

    // Register all generated IP cfgs, then enable only the selected chip's set.
    // Unselected implementations are never compiled, for PAC or metadata builds.
    let all_versions: BTreeSet<_> = CHIP_PERIPHERAL_VERSIONS
        .iter()
        .flat_map(|(_, versions)| versions.iter().copied())
        .collect();
    for version in all_versions {
        println!("cargo:rustc-check-cfg=cfg({version})");
    }
    let (_, versions) = CHIP_PERIPHERAL_VERSIONS
        .iter()
        .find(|(chip, _)| *chip == chip_core_name)
        .expect("selected chip is missing from generated peripheral inventory");
    for version in *versions {
        println!("cargo:rustc-cfg={version}");
    }

    #[cfg(feature = "rt")]
    println!(
        "cargo:rustc-link-search={}/src/chips/{}",
        crate_dir.display(),
        chip_core_name,
    );

    println!(
        "cargo:rustc-env=CW32_METAPAC_PAC_PATH=chips/{}/pac.rs",
        chip_core_name
    );
    println!(
        "cargo:rustc-env=CW32_METAPAC_METADATA_PATH=chips/{}/metadata.rs",
        chip_core_name
    );

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=chip_peripheral_versions.rs");
}
