use std::{collections::BTreeMap, env, fs, path::PathBuf};

fn event_irq(
    metadata: &cw32_metapac::metadata::Metadata,
    bank: &str,
    kind: &str,
    number: u32,
) -> &'static str {
    let mut peripherals = metadata.peripherals.iter().filter(|p| p.name == bank);
    let peripheral = peripherals.next().expect("selected event timer is absent");
    assert!(peripherals.next().is_none(), "duplicate timer bank");
    assert_eq!(peripheral.registers.as_ref().unwrap().kind, kind);
    let mut links = peripheral
        .interrupts
        .iter()
        .filter(|i| i.signal == "GLOBAL");
    let link = links
        .next()
        .expect("timer lacks its GLOBAL IRQ association");
    assert!(links.next().is_none(), "ambiguous timer GLOBAL IRQ");
    let mut interrupts = metadata
        .interrupts
        .iter()
        .filter(|i| i.name == link.interrupt);
    let interrupt = interrupts
        .next()
        .expect("timer IRQ is absent from NVIC metadata");
    assert!(interrupts.next().is_none(), "ambiguous NVIC entry");
    assert_eq!(interrupt.number, number, "unexpected timer IRQ number");
    interrupt.name
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
    println!("cargo:rustc-check-cfg=cfg(example_atim)");
    println!("cargo:rustc-check-cfg=cfg(example_l012)");

    let primary = match metadata.line {
        "CW32F002" | "CW32F003" => "GTIM",
        "CW32L012" => "GTIM3",
        "CW32F030" | "CW32A030" | "CW32F020" | "CW32L031" | "CW32R031"
        | "CW32L052" | "CW32L083" | "CW32L010" | "CW32L011" => "GTIM1",
        _ => panic!("unqualified example family"),
    };
    let l012 = metadata.line == "CW32L012";
    let mut banks = vec![(primary, "gtim", if l012 { 18 } else { 16 })];
    if l012 {
        println!("cargo:rustc-cfg=example_l012");
        banks.push(("GTIM4", "gtim", 18));
        fs::write(out.join("secondary-token.rs"), "p.GTIM4").unwrap();
    }
    if metadata.peripherals.iter().any(|p| p.name == "ATIM") {
        println!("cargo:rustc-cfg=example_atim");
        banks.push(("ATIM", "atim", 13));
    } else {
        assert!(matches!(metadata.line, "CW32F002" | "CW32F020"));
    }
    let mut vectors: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut receipt = format!("{}\n", metadata.name);
    for (bank, kind, number) in banks {
        let irq = event_irq(metadata, bank, kind, number);
        assert_eq!(
            irq,
            if l012 && kind == "gtim" {
                "GTIM34"
            } else {
                bank
            }
        );
        vectors.entry(irq).or_default().push(bank);
        receipt.push_str(&format!("{bank}: {irq} IRQ{number}\n"));
    }
    let mut bindings = String::from("hal::bind_interrupts!(struct Irqs {\n");
    for (irq, banks) in vectors {
        let handlers = banks
            .iter()
            .map(|bank| format!("hal::timer::OverflowInterruptHandler<hal::peripherals::{bank}>"))
            .collect::<Vec<_>>()
            .join(", ");
        bindings.push_str(&format!("    {irq} => {handlers};\n"));
    }
    bindings.push_str("});\n");
    fs::write(out.join("event-bindings.rs"), bindings).unwrap();
    fs::write(out.join("event-bindings.txt"), receipt).unwrap();
    fs::write(out.join("primary-token.rs"), format!("p.{primary}")).unwrap();
    // This separate binary owns the primary bank for direct-pin capture.
    // CAP1 is the separately qualified classic role; buffered input uses CH1.
    let timer = metadata
        .peripherals
        .iter()
        .find(|p| p.name == primary)
        .unwrap();
    let signal = if timer.classic_timer_input.is_some() {
        "CAP1"
    } else {
        "CH1"
    };
    let pin = timer
        .pins
        .iter()
        .find(|p| p.signal == signal)
        .expect("primary capture timer has no qualified channel-one pad");
    let irq = event_irq(metadata, primary, "gtim", if l012 { 18 } else { 16 });
    fs::write(out.join("capture-bindings.rs"), format!(
        "hal::bind_interrupts!(struct CaptureIrqs {{ {irq} => hal::timer::input_capture::CaptureInterruptHandler<hal::peripherals::{primary}>; }});"
    )).unwrap();
    fs::write(out.join("capture-constructor.rs"), format!(
        "InputCapture::new(p.{primary}, Some(CaptureInput::from_pin(p.{}, Pull::None)), None, None, None, Hertz(1_000_000), CountingMode::EdgeAlignedUp).unwrap()", pin.pin
    )).unwrap();
    fs::write(
        out.join("capture-route.txt"),
        format!(
            "{}: {primary} {signal}={} AF{}; IRQ={irq}\n",
            metadata.name,
            pin.pin,
            pin.af.unwrap()
        ),
    )
    .unwrap();
    println!("cargo:rustc-check-cfg=cfg(example_classic_capture)");
    if timer.classic_timer_input.is_some() {
        println!("cargo:rustc-cfg=example_classic_capture");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
