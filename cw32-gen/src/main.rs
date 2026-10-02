use std::path::Path;
fn main() -> cw32_gen::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.first().and_then(|s| s.to_str()) {
        Some("generate") if args.len() == 5 => cw32_gen::generate(
            Path::new(&args[1]),
            args[2].to_str().ok_or("invalid chip selector")?,
            Path::new(&args[3]),
            Path::new(&args[4]),
        ),
        Some("data") if args.len() == 4 => {
            cw32_gen::generate_data(
                Path::new(&args[1]),
                args[2].to_str().ok_or("invalid chip selector")?,
                Path::new(&args[3]),
            )?;
            Ok(())
        }
        Some("pac") if args.len() == 3 => {
            cw32_gen::pac::generate_from_json(Path::new(&args[1]), Path::new(&args[2]))
        }
        _ => Err("usage: cw32-gen generate <source> <chip|all> <json-dir> <pac-root> | data <source> <chip|all> <json-dir> | pac <chip.json> <pac-dir>".into()),
    }
}
