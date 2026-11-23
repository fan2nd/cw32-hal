use cw32_metapac::metadata::{Metadata, Peripheral, PeripheralClassicAtimInput};
use std::{collections::BTreeSet, env, fs, path::PathBuf};

fn input_pin(
    metadata: &Metadata,
    timer: &Peripheral,
    signal: &str,
    name: &str,
    af: u8,
) -> &'static str {
    let mut routes = timer
        .pins
        .iter()
        .filter(|p| p.signal == signal && p.pin == name);
    let route = routes
        .next()
        .expect("missing independently qualified native input route");
    assert!(routes.next().is_none(), "duplicate native input route");
    assert_eq!(route.af, Some(af), "native input route has unexpected AF");
    // Use the actual canonical GPIO token name, never an SDK macro or raw
    // datasheet pad spelling (for example PA00 instead of the Rust token PA0).
    let mut pins = metadata.pins.iter().filter(|pin| pin.name == route.pin);
    let pin = pins
        .next()
        .expect("input pad is not bonded in the selected package");
    assert!(pins.next().is_none(), "duplicate canonical GPIO pin");
    let bytes = pin.name.as_bytes();
    assert!(bytes.len() >= 3 && bytes[0] == b'P' && (b'A'..=b'F').contains(&bytes[1]));
    let bit: u8 = pin.name[2..]
        .parse()
        .expect("non-canonical GPIO bit number");
    assert!(bit < 16);
    assert_eq!(pin.name, format!("P{}{bit}", char::from(bytes[1])));
    let bank = format!("GPIO{}", char::from(bytes[1]));
    assert!(
        metadata
            .peripherals
            .iter()
            .any(|p| p.name == bank && p.registers.as_ref().is_some_and(|r| r.kind == "gpio"))
    );
    // Match the HAL's retained debug/reset/input-only exclusion for these families.
    assert!(
        if metadata.line == "CW32F003" {
            !matches!(pin.name, "PA2" | "PA5" | "PC5")
        } else {
            !matches!(pin.name, "PA13" | "PA14" | "PF3")
        },
        "excluded GPIO token"
    );
    pin.name
}

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package memory map"
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut memory = String::from("MEMORY {\n");
    for region in metadata.memory[0] {
        assert!(matches!(region.name, "FLASH" | "RAM"));
        memory.push_str(&format!(
            "  {} : ORIGIN = {:#x}, LENGTH = {}\n",
            region.name, region.address, region.size
        ));
    }
    memory.push_str("}\n");
    fs::write(out.join("memory.x"), memory).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.x");

    let mut timers = metadata.peripherals.iter().filter(|p| p.name == "ATIM");
    let timer = timers.next().expect("selected part has no ATIM");
    assert!(timers.next().is_none());
    assert_eq!(
        timer.classic_atim_input,
        Some(PeripheralClassicAtimInput::ClassicPairedV1)
    );
    let regs = timer.registers.as_ref().unwrap();
    assert_eq!(regs.kind, "atim");
    let (expected_version, pads, af) = match metadata.name {
        "CW32F003E4P7" => ("cw32f003_v1", ["PB6", "PB5", "PB3", "PB2", "PA0", "PA1"], 4),
        "CW32L052C8T6" => ("cw32l052_v1", ["PA5", "PA7", "PA4", "PB0", "PA3", "PB1"], 7),
        // R031 PA3 is radio-reserved and lacks its own corroborated CH3A route.
        "CW32R031C8U6" => ("v1", ["PA5", "PA7", "PA4", "PB0", "PA10", "PB1"], 7),
        "CW32F030C8T7" | "CW32A030C8T7" | "CW32L031C8T6" | "CW32L083MCT6" => {
            ("v1", ["PA5", "PA7", "PA4", "PB0", "PA3", "PB1"], 7)
        }
        _ => panic!("unqualified native input example selection"),
    };
    assert_eq!(regs.version, expected_version);
    let signals = ["CAP1A", "CAP1B", "CAP2A", "CAP2B", "CAP3A", "CAP3B"];
    let mut owned = BTreeSet::new();
    let mut canonical_pins = Vec::new();
    let mut receipt = format!("{}: ATIM {}\n", metadata.name, regs.version);
    for (signal, pad) in signals.into_iter().zip(pads) {
        let pin = input_pin(metadata, timer, signal, pad, af);
        assert!(
            owned.insert(pin),
            "capture halves must own distinct bonded pads"
        );
        canonical_pins.push(pin);
        receipt.push_str(&format!("{signal}={pin} AF{af}\n"));
    }
    let wrappers = canonical_pins
        .iter()
        .map(|pin| format!("Some(ClassicAtimCaptureInput::from_pin(p.{pin}, Pull::None))"))
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(
        out.join("capture-constructor.rs"),
        format!("ClassicAtimInputCapture::new(p.ATIM, {wrappers}, Hertz(1_000_000)).unwrap()"),
    )
    .unwrap();
    fs::write(
        out.join("qei-constructor.rs"),
        format!(
            "ClassicAtimQei::new(p.ATIM, p.{}, p.{}, ClassicAtimQeiConfig::default()).unwrap()",
            canonical_pins[0], canonical_pins[1]
        ),
    )
    .unwrap();
    fs::write(out.join("external-input-routes.txt"), receipt).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
