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
    address: u64,
    request: u32,
    irq: u32,
) {
    assert!(!matches!(pin, "PA13" | "PA14"));
    assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
    let uart = peripheral(metadata, uart);
    assert_eq!(uart.address, address);
    let registers = uart.registers.as_ref().unwrap();
    assert_eq!(registers.kind, "uart");
    assert_eq!(registers.version, "cw32l012_v1");
    assert_eq!(registers.block, "Uart");
    assert_eq!(
        uart.pins
            .iter()
            .filter(|p| p.pin == pin && p.signal == "TX" && p.af == Some(1))
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
        ("CARGO_FEATURE_CW32L012C8T6", "CW32L012C8T6"),
        ("CARGO_FEATURE_CW32L012C8U6", "CW32L012C8U6"),
    ];
    let selected: Vec<_> = chip_features
        .iter()
        .filter(|(feature, _)| env::var_os(feature).is_some())
        .collect();
    assert_eq!(selected.len(), 1, "select exactly one exact chip feature");
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.name, selected[0].1);
    assert_eq!(metadata.line, "CW32L012");

    check_tx_route(metadata, "UART1", "PA9", 0x4000_0c00, 1, 27);
    check_tx_route(metadata, "UART2", "PA7", 0x4000_1000, 3, 28);
    check_tx_route(metadata, "UART3", "PA8", 0x4000_2000, 5, 29);

    let dma = peripheral(metadata, "DMA");
    assert_eq!(dma.address, 0x4002_0000);
    let registers = dma.registers.as_ref().unwrap();
    assert_eq!(registers.kind, "dma");
    assert_eq!(registers.version, "cw32l012_v1");
    assert_eq!(registers.block, "Dma");
    assert_eq!(
        metadata
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "dma"))
            .count(),
        1
    );
    assert_eq!(metadata.dma_channels.len(), 4);
    assert_eq!(
        metadata
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "dmachannel"))
            .count(),
        4
    );
    for (index, channel, instance, address, vector, irq) in [
        (0, "DMA_CH1", "DMACHANNEL1", 0x4002_0020, "DMACH12", 9),
        (1, "DMA_CH2", "DMACHANNEL2", 0x4002_0040, "DMACH12", 9),
        (2, "DMA_CH3", "DMACHANNEL3", 0x4002_0060, "DMACH34", 10),
        (3, "DMA_CH4", "DMACHANNEL4", 0x4002_0080, "DMACH34", 10),
    ] {
        let channels: Vec<_> = metadata
            .dma_channels
            .iter()
            .filter(|c| c.name == channel && c.dma == "DMA" && c.channel == index)
            .collect();
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0].dmamux, None);
        assert_eq!(channels[0].dmamux_channel, None);
        let instance = peripheral(metadata, instance);
        assert_eq!(instance.address, address);
        let registers = instance.registers.as_ref().unwrap();
        assert_eq!(registers.kind, "dmachannel");
        assert_eq!(registers.version, "cw32l012_v1");
        assert_eq!(registers.block, "Dmachannel");
        check_irq(metadata, instance, vector, irq);
    }
    let mut dma_vectors: Vec<_> = metadata
        .interrupts
        .iter()
        .filter(|irq| irq.name.starts_with("DMA"))
        .map(|irq| (irq.name, irq.number))
        .collect();
    dma_vectors.sort_unstable();
    assert_eq!(dma_vectors, [("DMACH12", 9), ("DMACH34", 10)]);
    // IRQ11 belongs to CORDIC, never a third DMA vector.
    check_irq(metadata, peripheral(metadata, "CORDIC"), "CORDIC", 11);

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
    for (feature, _) in chip_features {
        println!("cargo:rerun-if-env-changed={feature}");
    }
}
