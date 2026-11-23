use std::{env, fs, path::PathBuf};

fn btim_irq(metadata: &cw32_metapac::metadata::Metadata, bank: &str, number: u32) -> &'static str {
    let mut peripherals = metadata.peripherals.iter().filter(|p| p.name == bank);
    let peripheral = peripherals.next().expect("selected chip lacks a BTIM bank");
    assert!(peripherals.next().is_none(), "duplicate BTIM bank");
    assert_eq!(peripheral.registers.as_ref().unwrap().kind, "btim");
    let mut links = peripheral
        .interrupts
        .iter()
        .filter(|i| i.signal == "GLOBAL");
    let link = links.next().expect("BTIM bank lacks a GLOBAL IRQ link");
    assert!(links.next().is_none(), "ambiguous BTIM GLOBAL IRQ link");
    let mut interrupts = metadata
        .interrupts
        .iter()
        .filter(|i| i.name == link.interrupt);
    let interrupt = interrupts
        .next()
        .expect("BTIM IRQ is absent from NVIC metadata");
    assert!(interrupts.next().is_none(), "ambiguous BTIM NVIC entry");
    assert_eq!(interrupt.number, number, "unexpected BTIM IRQ number");
    interrupt.name
}

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package with a qualified memory map"
    );
    let mut memory = String::from("MEMORY {\n");
    for bank in metadata.memory[0] {
        assert!(matches!(bank.name, "FLASH" | "RAM"));
        memory.push_str(&format!(
            "  {} : ORIGIN = {:#x}, LENGTH = {}\n",
            bank.name, bank.address, bank.size
        ));
    }
    memory.push_str("}\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), memory).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.x");
    for cfg in [
        "example_awt",
        "example_lse_awt",
        "example_l012",
        "example_l052",
        "example_l083",
    ] {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }
    assert_eq!(btim_irq(metadata, "BTIM1", 20), "BTIM1");
    let btim2_irq = btim_irq(metadata, "BTIM2", 21);
    let btim3_irq = btim_irq(metadata, "BTIM3", 22);
    let btim_shared = match (btim2_irq, btim3_irq) {
        ("BTIM2", "BTIM3") => false,
        ("BTIM2_LPTIM", "BTIM3") => {
            assert_eq!(metadata.line, "CW32L052");
            true
        }
        ("BTIM2_LPTIM1", "BTIM3") => {
            assert_eq!(metadata.line, "CW32L083");
            true
        }
        ("BTIM2", "BTIM3_HALLTIM") => {
            assert_eq!(metadata.line, "CW32L012");
            println!("cargo:rustc-cfg=example_l012");
            true
        }
        _ => panic!("unsupported selected-chip BTIM IRQ association"),
    };
    assert!(
        env::var_os("CARGO_FEATURE_BTIM_SHARED").is_none() || btim_shared,
        "btim-shared requires L012, L052 or L083 with its actual shared BTIM IRQ; selected {}",
        metadata.name
    );
    let lse_awt = matches!(
        metadata.name,
        "CW32F020C6U7"
            | "CW32F030C8T7"
            | "CW32A030C8T7"
            | "CW32L031C8T6"
            | "CW32L031C8U6"
            | "CW32L031F8U6"
            | "CW32R031C8U6"
    );
    assert!(
        env::var_os("CARGO_FEATURE_LSE_AWT").is_none() || lse_awt,
        "lse-awt requires an exact package qualified for owned LSE AWT; selected {}",
        metadata.name
    );
    if lse_awt {
        println!("cargo:rustc-cfg=example_lse_awt");
    }
    if metadata.peripherals.iter().any(|p| p.name == "AWT") {
        println!("cargo:rustc-cfg=example_awt");
    }
    if btim2_irq == "BTIM2_LPTIM" {
        println!("cargo:rustc-cfg=example_l052");
    }
    if btim2_irq == "BTIM2_LPTIM1" {
        println!("cargo:rustc-cfg=example_l083");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
