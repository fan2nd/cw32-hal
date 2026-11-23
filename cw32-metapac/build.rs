use std::collections::BTreeSet;
use std::env;

mod chip_peripheral_versions;
use chip_peripheral_versions::CHIP_PERIPHERAL_VERSIONS;
#[cfg(feature = "rt")]
use std::path::PathBuf;

mod package_groups;

fn main() {
    #[cfg(feature = "rt")]
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let selected: BTreeSet<_> = env::vars()
        .filter_map(|(key, _)| {
            key.strip_prefix("CARGO_FEATURE_CW32")
                .map(|suffix| format!("cw32{}", suffix.to_ascii_lowercase().replace('_', "-")))
        })
        .collect();
    let chip_core_name = match selected.len() {
        0 => panic!("No cw32xx Cargo feature enabled"),
        1 => selected.first().unwrap().as_str(),
        _ => {
            // Dependencies may select the neutral target and/or exact package features.
            let (_, members) = package_groups::PACKAGE_GROUPS
                .iter()
                .find(|(target, members)| {
                    selected
                        .iter()
                        .all(|name| name == target || members.contains(&name.as_str()))
                })
                .expect("Multiple cw32xx Cargo features enabled for different firmware targets");
            members[0]
        }
    };

    let firmware_target = package_groups::PACKAGE_GROUPS
        .iter()
        .find(|(_, members)| members.contains(&chip_core_name))
        .map_or(chip_core_name, |(target, _)| *target);
    println!("cargo:rustc-env=CW32_FIRMWARE_TARGET={firmware_target}");

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
    println!("cargo:rerun-if-changed=package_groups.rs");
    println!("cargo:rerun-if-changed=chip_peripheral_versions.rs");
}
