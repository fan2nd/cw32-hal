use cw32_metapac::metadata::{MemoryRegionKind, Metadata, Peripheral};
use std::{env, fs, path::PathBuf};

fn peripheral<'a>(metadata: &'a Metadata, name: &str) -> &'a Peripheral {
    let mut matches = metadata.peripherals.iter().filter(|p| p.name == name);
    let peripheral = matches.next().expect("missing example peripheral");
    assert!(matches.next().is_none(), "duplicate example peripheral");
    peripheral
}

fn check_irq(metadata: &Metadata, peripheral: &Peripheral, vector: &str, number: u32) {
    assert_eq!(peripheral.interrupts.len(), 1);
    assert_eq!(peripheral.interrupts[0].signal, "GLOBAL");
    assert_eq!(peripheral.interrupts[0].interrupt, vector);
    assert_eq!(
        metadata
            .interrupts
            .iter()
            .filter(|irq| irq.name == vector && irq.number == number)
            .count(),
        1,
        "example vector must match the selected native interrupt"
    );
}

fn check_tx_route(
    metadata: &Metadata,
    uart: &str,
    pin: &str,
    af: u8,
    request: u32,
    irq: u32,
    version: &str,
) {
    assert!(!matches!(pin, "PA13" | "PA14" | "PF3"));
    assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
    let uart = peripheral(metadata, uart);
    let registers = uart.registers.as_ref().unwrap();
    assert_eq!(registers.kind, "uart");
    assert_eq!(registers.version, version);
    assert_eq!(
        uart.pins
            .iter()
            .filter(|p| p.pin == pin && p.signal == "TX" && p.af == Some(af))
            .count(),
        1,
        "the selected package must expose this exact UART TX pin and AF"
    );
    let mut tx = uart
        .dma_channels
        .iter()
        .filter(|route| route.signal == "TX");
    let tx_route = tx.next().expect("missing UART TX DMA request");
    assert!(tx.next().is_none(), "duplicate UART TX DMA request");
    assert_eq!(tx_route.dma, Some("DMA"));
    assert_eq!(tx_route.request, Some(request));
    assert_eq!(tx_route.channel, None);
    assert_eq!(tx_route.dmamux, None);
    assert!(tx_route.remap.is_empty());
    check_irq(metadata, uart, uart.name, irq);
}

fn main() {
    let chip_features = [
        ("CARGO_FEATURE_CW32L031C8T6", "CW32L031C8T6", "CW32L031"),
        ("CARGO_FEATURE_CW32L031C8U6", "CW32L031C8U6", "CW32L031"),
        ("CARGO_FEATURE_CW32L031F8P6", "CW32L031F8P6", "CW32L031"),
        ("CARGO_FEATURE_CW32L031F8U6", "CW32L031F8U6", "CW32L031"),
        ("CARGO_FEATURE_CW32L031K8U6", "CW32L031K8U6", "CW32L031"),
        ("CARGO_FEATURE_CW32L031K8V6", "CW32L031K8V6", "CW32L031"),
        ("CARGO_FEATURE_CW32L052C8T6", "CW32L052C8T6", "CW32L052"),
        ("CARGO_FEATURE_CW32L052R8S6", "CW32L052R8S6", "CW32L052"),
        ("CARGO_FEATURE_CW32L052R8T6", "CW32L052R8T6", "CW32L052"),
        ("CARGO_FEATURE_CW32R031C8U6", "CW32R031C8U6", "CW32R031"),

    ];
    let selected: Vec<_> = chip_features
        .iter()
        .filter(|(feature, _, _)| env::var_os(feature).is_some())
        .collect();
    assert_eq!(selected.len(), 1, "select exactly one exact chip feature");
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.name, selected[0].1);
    assert_eq!(metadata.line, selected[0].2);

    let (has_uart1, uart3_pin, uart3_af, uart_version) = match metadata.name {
        "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031K8U6" | "CW32L031K8V6" | "CW32R031C8U6" => (true, "PA10", 6, "cw32l031_v1"),
        "CW32L031F8P6" | "CW32L031F8U6" => (false, "PA10", 6, "cw32l031_v1"),
        "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6" => (true, "PA9", 1, "cw32l052_v1"),
        _ => unreachable!(),
    };
    if has_uart1 {
        check_tx_route(metadata, "UART1", "PA8", 2, 1, 27, uart_version);
    }
    check_tx_route(metadata, "UART2", "PA6", 2, 3, 28, uart_version);
    check_tx_route(metadata, "UART3", uart3_pin, uart3_af, 5, 29, uart_version);

    assert_eq!(metadata.dma_channels.len(), 4);
    for (index, channel, instance, vector, irq) in [
        (0, "DMA_CH1", "DMACHANNEL1", "DMACH1", 9),
        (1, "DMA_CH2", "DMACHANNEL2", "DMACH23", 10),
        (2, "DMA_CH3", "DMACHANNEL3", "DMACH23", 10),
        (3, "DMA_CH4", "DMACHANNEL4", "DMACH4", 11),
    ] {
        assert_eq!(
            metadata
                .dma_channels
                .iter()
                .filter(|c| c.name == channel && c.dma == "DMA" && c.channel == index)
                .count(),
            1
        );
        check_irq(metadata, peripheral(metadata, instance), vector, irq);
    }

    // Derive both regions from this exact selected part, with no generic map.
    assert_eq!(metadata.memory.len(), 1);
    let banks = metadata.memory[0];
    assert_eq!(banks.len(), 2);
    assert_eq!(banks.iter().filter(|bank| bank.name == "FLASH").count(), 1);
    assert_eq!(banks.iter().filter(|bank| bank.name == "RAM").count(), 1);
    let mut memory = String::from("MEMORY {\n");
    for bank in banks {
        match bank.name {
            "FLASH" => {
                assert_eq!(bank.kind, MemoryRegionKind::Flash);
                assert_eq!((bank.address, bank.size), (0, 64 * 1024));
            }
            "RAM" => {
                assert_eq!(bank.kind, MemoryRegionKind::Ram);
                assert_eq!((bank.address, bank.size), (0x2000_0000, 8 * 1024));
            }
            _ => panic!("unexpected memory region"),
        }
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
    println!("cargo:rerun-if-changed=build.rs");
    for (feature, _, _) in chip_features {
        println!("cargo:rerun-if-env-changed={feature}");
    }
}
