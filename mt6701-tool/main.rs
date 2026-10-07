mod config;
mod mcp2221;

use anyhow::{ensure, Context, Result};
use clap::{Parser, Subcommand};
use config::{Registers, Settings, REGISTERS};
use hidapi::HidApi;
use mcp2221::{Bridge, Transport, PID, VID};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(version, about = "Configure MT6701QT over MCP2221A USB HID / I2C")]
struct Cli {
    /// Select one adapter by its USB serial number
    #[arg(long, global = true)]
    serial: Option<String>,
    /// Select one adapter by the exact HID path printed by devices
    #[arg(long, global = true, conflicts_with = "serial")]
    path: Option<String>,
    /// Seven-bit MT6701 address (0x06 or 0x46)
    #[arg(long, global = true, default_value = "0x06", value_parser = parse_address)]
    address: u8,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List matching HID adapters without accessing the sensor
    Devices,
    /// Read angle and documented configuration registers
    Read,
    /// Preview changes; --apply writes RAM, --eeprom saves persistently
    Configure {
        #[command(flatten)]
        settings: Settings,
        #[arg(long, conflicts_with = "eeprom")]
        apply: bool,
        /// Program EEPROM, then power-cycle manually and run verify
        #[arg(long, requires = "vdd_mv")]
        eeprom: bool,
        /// Actual sensor supply, entered by operator (not measured by this tool)
        #[arg(long, requires = "eeprom", value_parser = clap::value_parser!(u16).range(4501..5500))]
        vdd_mv: Option<u16>,
        /// New JSON backup/expected-state file; existing files are never overwritten
        #[arg(long)]
        backup: Option<PathBuf>,
    },
    /// Compare registers with a previous write snapshot (after power-cycle for EEPROM)
    Verify { snapshot: PathBuf },
    /// Explicitly cancel a stuck MCP2221A I2C transaction
    Recover,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    version: u8,
    address: u8,
    adapter_path: String,
    adapter_serial: Option<String>,
    registers: [u8; 11],
    before: Registers,
    expected: Registers,
    eeprom_requested: bool,
    supply_mv: Option<u16>,
}

fn parse_address(s: &str) -> Result<u8, String> {
    let value = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u8::from_str_radix(hex, 16)
    } else {
        s.parse()
    }
    .map_err(|_| "invalid I2C address".to_string())?;
    if matches!(value, 0x06 | 0x46) {
        Ok(value)
    } else {
        Err("MT6701 address must be 0x06 or 0x46 (7-bit)".into())
    }
}

fn read_config(bridge: &Bridge<impl Transport>) -> Result<Registers> {
    let mut values = [0; 11];
    for (i, reg) in REGISTERS.iter().enumerate() {
        values[i] = bridge
            .read_register(*reg)
            .with_context(|| format!("reading MT6701 register 0x{reg:02X}"))?;
    }
    Ok(values)
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    // Validate settings and verification snapshots before opening a USB device.
    let expected_snapshot = match &cli.command {
        Command::Verify { snapshot } => {
            let s: Snapshot = serde_json::from_slice(&fs::read(snapshot)?)?;
            ensure!(
                s.version == 1 && s.registers == REGISTERS,
                "unsupported snapshot format"
            );
            ensure!(
                s.address == cli.address,
                "snapshot address differs; pass --address 0x{:02X}",
                s.address
            );
            Some(s)
        }
        Command::Configure { settings, .. } => {
            settings.build([0; 11])?;
            None
        }
        _ => None,
    };
    let api = HidApi::new().context("initializing USB HID")?;
    let devices: Vec<_> = api
        .device_list()
        .filter(|d| d.vendor_id() == VID && d.product_id() == PID)
        .collect();
    if matches!(cli.command, Command::Devices) {
        for d in &devices {
            println!(
                "serial={} product={} path={}",
                d.serial_number().unwrap_or("<none>"),
                d.product_string().unwrap_or("<unknown>"),
                d.path().to_string_lossy()
            );
        }
        println!("{} MCP2221(A) HID adapter(s)", devices.len());
        return Ok(());
    }
    let selected: Vec<_> = devices
        .into_iter()
        .filter(|d| {
            cli.serial
                .as_deref()
                .is_none_or(|s| d.serial_number() == Some(s))
                && cli
                    .path
                    .as_deref()
                    .is_none_or(|p| d.path().to_bytes() == p.as_bytes())
        })
        .collect();
    ensure!(
        selected.len() == 1,
        "found {} matching adapters; run devices and select exactly one using --serial or --path",
        selected.len()
    );
    let info = selected[0];
    let bridge = Bridge {
        transport: info.open_device(&api)?,
        address: cli.address,
    };
    match cli.command {
        Command::Devices => unreachable!(),
        Command::Recover => {
            bridge.recover()?;
            println!("I2C engine idle.");
        }
        Command::Read => {
            let values = read_config(&bridge)?;
            for (reg, value) in REGISTERS.iter().zip(values) {
                println!("0x{reg:02X} = 0x{value:02X}");
            }
            config::describe(&values);
            // The datasheet requires 0x03 to be read before 0x04.
            let high = bridge.read_register(0x03)?;
            let low = bridge.read_register(0x04)?;
            let angle = ((high as u16) << 6) | (low >> 2) as u16;
            println!(
                "Angle: {angle}/16384 = {:.4} deg",
                angle as f64 * 360.0 / 16384.0
            );
        }
        Command::Verify { .. } => {
            let snapshot = expected_snapshot.unwrap();
            if let Some(serial) = &snapshot.adapter_serial {
                ensure!(
                    info.serial_number() == Some(serial.as_str()),
                    "adapter serial differs from snapshot"
                );
            }
            config::verify(&snapshot.expected, &read_config(&bridge)?)?;
            println!("All 11 configuration bytes match. EEPROM persistence is confirmed only if you power-cycled the sensor before this command.");
        }
        Command::Configure {
            settings,
            apply,
            eeprom,
            vdd_mv,
            backup,
        } => {
            let before = read_config(&bridge)?;
            let expected = settings.build(before)?;
            config::describe(&expected);
            let changes: Vec<_> = REGISTERS
                .iter()
                .copied()
                .zip(before.into_iter().zip(expected))
                .filter(|(_, (old, new))| old != new)
                .collect();
            for (reg, (old, new)) in &changes {
                println!("0x{reg:02X}: 0x{old:02X} -> 0x{new:02X}");
            }
            if !apply && !eeprom {
                println!("Preview only: {} byte(s) would change. Use --apply or --eeprom --vdd-mv 5000 to write.", changes.len());
                return Ok(());
            }
            // EEPROM may still need saving when RAM already matches, e.g. after --apply.
            let backup = backup.unwrap_or_else(|| {
                PathBuf::from(format!(
                    "mt6701-{}.json",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos()
                ))
            });
            let snapshot = Snapshot {
                version: 1,
                address: cli.address,
                adapter_path: info.path().to_string_lossy().into_owned(),
                adapter_serial: info.serial_number().map(str::to_owned),
                registers: REGISTERS,
                before,
                expected,
                eeprom_requested: eeprom,
                supply_mv: vdd_mv,
            };
            // Finish a durable backup before the first configuration write.
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&backup)
                .with_context(|| format!("creating backup {}", backup.display()))?;
            file.write_all(&serde_json::to_vec_pretty(&snapshot)?)?;
            file.sync_all()?;
            println!(
                "Backup and expected state: {}",
                fs::canonicalize(&backup)?.display()
            );
            for (reg, (_, value)) in changes {
                bridge
                    .write_register(reg, value)
                    .context("partial RAM update possible; EEPROM has not been requested yet")?;
            }
            config::verify(&expected, &read_config(&bridge)?)
                .context("RAM verification failed; EEPROM has not been requested")?;
            println!("RAM readback verified.");
            if eeprom {
                println!("Programming EEPROM; keep sensor powered...");
                bridge.program_eeprom()?;
                println!("Programming wait finished. Power-cycle MT6701, keep I2C wiring, then run: mt6701-tool --address 0x{:02X} verify \"{}\"", cli.address, backup.display());
            } else {
                println!("Volatile write only; EEPROM was not programmed.");
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cli_rejects_unsafe_or_ambiguous_arguments() {
        for args in [
            vec!["tool", "configure", "--ppr", "1025"],
            vec!["tool", "configure", "--ppr", "0"],
            vec!["tool", "configure", "--ppr", "1024", "--eeprom"],
            vec![
                "tool",
                "configure",
                "--ppr",
                "1024",
                "--eeprom",
                "--vdd-mv",
                "4500",
            ],
            vec![
                "tool",
                "configure",
                "--ppr",
                "1024",
                "--eeprom",
                "--vdd-mv",
                "5500",
            ],
            vec![
                "tool",
                "configure",
                "--ppr",
                "1024",
                "--apply",
                "--eeprom",
                "--vdd-mv",
                "5000",
            ],
            vec!["tool", "--address", "0x0c", "read"],
        ] {
            assert!(Cli::try_parse_from(&args).is_err(), "{args:?}");
        }
        assert!(Cli::try_parse_from([
            "tool",
            "configure",
            "--abz",
            "--ppr",
            "1024",
            "--eeprom",
            "--vdd-mv",
            "5000"
        ])
        .is_ok());
    }
}
