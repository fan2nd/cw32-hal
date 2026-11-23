use cw32_metapac::metadata::{MemoryRegionKind, Metadata, Peripheral};
use std::{collections::HashSet, env, fs, path::PathBuf};

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
    let mut matches = metadata.interrupts.iter().filter(|irq| irq.name == vector);
    let irq = matches.next().expect("missing native interrupt vector");
    assert!(
        matches.next().is_none(),
        "duplicate native interrupt vector"
    );
    assert_eq!(irq.number, number);
    assert_eq!(
        metadata
            .interrupts
            .iter()
            .filter(|irq| irq.number == number)
            .count(),
        1
    );
}

struct Route {
    name: &'static str,
    address: u64,
    irq: &'static str,
    number: u32,
    sck: &'static str,
    mosi: &'static str,
    miso: &'static str,
    af: u8,
    cs: &'static str,
    rx_request: u32,
    tx_request: u32,
}

const ROUTES: &[Route] = &[
    Route {
        name: "SPI1",
        address: 0x4000_0800,
        irq: "SPI1",
        number: 25,
        sck: "PA5",
        mosi: "PA7",
        miso: "PA6",
        af: 3,
        cs: "PB0",
        rx_request: 6,
        tx_request: 7,
    },
    Route {
        name: "SPI2",
        address: 0x4000_0400,
        irq: "SPI23",
        number: 26,
        sck: "PA2",
        mosi: "PA1",
        miso: "PA0",
        af: 8,
        cs: "PB1",
        rx_request: 8,
        tx_request: 9,
    },
    Route {
        name: "SPI3",
        address: 0x4000_3400,
        irq: "SPI23",
        number: 26,
        sck: "PA9",
        mosi: "PA11",
        miso: "PA10",
        af: 8,
        cs: "PB2",
        rx_request: 10,
        tx_request: 11,
    },
];

fn check_pin(metadata: &Metadata, pin: &str) {
    assert_eq!(metadata.pins.iter().filter(|p| p.name == pin).count(), 1);
    assert!(
        !matches!(
            pin,
            "PA13" | "PA14" | "PF3" | "PF0" | "PF1" | "PC14" | "PC15"
        ),
        "example must preserve debug, reset, BOOT and oscillator pads"
    );
    // Physical package positions come from the source ledger, not Pin metadata.
}

fn check_spi_route(metadata: &Metadata, route: &Route) {
    let spi = peripheral(metadata, route.name);
    assert_eq!(spi.address, route.address);
    let registers = spi.registers.as_ref().unwrap();
    assert_eq!(
        (registers.kind, registers.version, registers.block),
        ("spi", "cw32l012_v1", "Spi")
    );
    check_irq(metadata, spi, route.irq, route.number);
    let owners: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.interrupts.iter().any(|i| i.interrupt == route.irq))
        .map(|p| p.name)
        .collect();
    if route.irq == "SPI23" {
        assert_eq!(owners, ["SPI2", "SPI3"]);
    } else {
        assert_eq!(owners, ["SPI1"]);
    }
    for (pin, signal) in [
        (route.sck, "SCK"),
        (route.mosi, "MOSI"),
        (route.miso, "MISO"),
    ] {
        check_pin(metadata, pin);
        assert_eq!(
            spi.pins
                .iter()
                .filter(|p| p.pin == pin && p.signal == signal && p.af == Some(route.af))
                .count(),
            1,
            "the exact package must expose the selected SPI pin and AF"
        );
    }
    check_pin(metadata, route.cs);
    assert_eq!(spi.dma_channels.len(), 2);
    for (signal, request) in [("RX", route.rx_request), ("TX", route.tx_request)] {
        let mut requests = spi.dma_channels.iter().filter(|r| r.signal == signal);
        let request_route = requests.next().expect("missing native SPI DMA request");
        assert!(
            requests.next().is_none(),
            "duplicate native SPI DMA request"
        );
        assert_eq!(request_route.dma, Some("DMA"));
        assert_eq!(request_route.request, Some(request));
        assert_eq!(request_route.channel, None);
        assert_eq!(request_route.dmamux, None);
        assert!(request_route.remap.is_empty());
    }
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

    let spi_names: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "spi"))
        .map(|p| p.name)
        .collect();
    assert_eq!(spi_names, ["SPI1", "SPI2", "SPI3"]);
    let mut selected_pins = HashSet::new();
    for route in ROUTES {
        check_spi_route(metadata, route);
        for pin in [route.sck, route.mosi, route.miso, route.cs] {
            assert!(
                selected_pins.insert(pin),
                "buses must own distinct pins and CS"
            );
        }
    }

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
