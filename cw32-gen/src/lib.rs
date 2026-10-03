//! One host generator with an explicit, on-disk YAML -> JSON -> PAC boundary.
//!
//! The data stage owns source loading and normalization. The PAC stage consumes
//! only the serialized JSON interface, including when invoked by this pipeline.
pub mod data;
pub mod pac;
pub mod schema;

pub use crate::schema::Result;
use std::{fs, path::Path};

/// Normalize one chip or all chips into shared registers and chip JSON files.
pub fn generate_data(source: &Path, selector: &str, json: &Path) -> Result<Vec<String>> {
    let names = if selector == "all" {
        let mut names = Vec::new();
        for entry in fs::read_dir(source.join("chips"))? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "yaml") {
                names.push(
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .ok_or("invalid chip filename")?
                        .to_owned(),
                );
            }
        }
        names.sort();
        if names.is_empty() {
            return Err("no chip YAML sources found".into());
        }
        names
    } else {
        vec![selector.to_owned()]
    };
    let refs: Vec<_> = names.iter().map(String::as_str).collect();
    data::generate_many(source, &refs, json)?;
    Ok(names)
}

/// Run both stages, retaining and re-reading normalized JSON before any render.
/// PAC output shares common code and kind/version modules across chip roots.
pub fn generate(source: &Path, selector: &str, json: &Path, output: &Path) -> Result<()> {
    let names = generate_data(source, selector, json)?;
    let paths = names
        .iter()
        .map(|name| json.join(format!("chips/{name}.json")))
        .collect::<Vec<_>>();
    pac::generate_many_from_json(&paths, output)
}
