use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "select one exact package with a qualified memory map"
    );
    println!("cargo:rustc-check-cfg=cfg(example_l083)");
    println!("cargo:rustc-check-cfg=cfg(example_f020)");
    println!("cargo:rustc-check-cfg=cfg(example_four_channel)");
    println!("cargo:rustc-check-cfg=cfg(example_rx_uart1)");
    println!("cargo:rustc-check-cfg=cfg(example_rx_uart2)");
    println!("cargo:rustc-check-cfg=cfg(example_rx_uart3)");
    let pins: &[&str] = match metadata.line {
        "CW32F030" | "CW32A030" => &["PA2", "PA3"],
        "CW32L083" => {
            println!("cargo:rustc-cfg=example_l083");
            &["PA6", "PA7", "PA8", "PA9", "PA4", "PB5"]
        }
        "CW32F020" => {
            println!("cargo:rustc-cfg=example_f020");
            &["PA6", "PA7"]
        }
        "CW32L031" | "CW32L052" | "CW32R031" => {
            println!("cargo:rustc-cfg=example_four_channel");
            &["PA6", "PA7"]
        }
        _ => panic!("select a qualified exact UART DMA package"),
    };
    for pin in pins {
        assert!(metadata.pins.iter().any(|route| route.name == *pin));
    }

    // The seven finite branches select the standalone RX route. The combined
    // application uses UART2 PA6/AF2 TX and PA7/AF2 RX on all fourteen parts.
    let classic_rx = match (metadata.line, metadata.name) {
        ("CW32F020", "CW32F020C6U7" | "CW32F020K6U7") => {
            Some(("UART1", "PA9", 2, "example_rx_uart1", 32 * 1024))
        }
        ("CW32F020", "CW32F020F6U7") => {
            Some(("UART2", "PA7", 2, "example_rx_uart2", 32 * 1024))
        }
        (
            "CW32L031",
            "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6" | "CW32L031K8U6"
            | "CW32L031K8V6",
        ) => Some(("UART1", "PA9", 2, "example_rx_uart1", 64 * 1024)),
        ("CW32L031", "CW32L031F8P6") => {
            Some(("UART2", "PA7", 2, "example_rx_uart2", 64 * 1024))
        }
        ("CW32L052", "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6") => {
            Some(("UART3", "PA10", 1, "example_rx_uart3", 64 * 1024))
        }
        ("CW32R031", "CW32R031C8U6") => {
            Some(("UART1", "PA9", 2, "example_rx_uart1", 64 * 1024))
        }
        ("CW32F030" | "CW32A030" | "CW32L083", _) => None,
        _ => panic!("select one of the thirteen qualified exact classic packages"),
    };
    if let Some((rx_uart, rx_pin, rx_af, rx_cfg, flash_size)) = classic_rx {
        println!("cargo:rustc-cfg={rx_cfg}");
        for (uart, signal, pin, af) in [
            ("UART2", "TX", "PA6", 2),
            ("UART2", "RX", "PA7", 2),
            (rx_uart, "RX", rx_pin, rx_af),
        ] {
            assert!(metadata.pins.iter().any(|bond| bond.name == pin));
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|peripheral| peripheral.name == uart)
                .expect("qualified UART is present");
            assert!(peripheral.pins.iter().any(|route| {
                route.pin == pin && route.signal == signal && route.af == Some(af)
            }));
        }
        let banks = metadata.memory[0];
        assert_eq!(banks.len(), 2);
        assert!(banks.iter().any(|bank| {
            bank.name == "FLASH"
                && bank.kind == cw32_metapac::metadata::MemoryRegionKind::Flash
                && bank.address == 0
                && bank.size == flash_size
        }));
        assert!(banks.iter().any(|bank| {
            bank.name == "RAM"
                && bank.kind == cw32_metapac::metadata::MemoryRegionKind::Ram
                && bank.address == 0x2000_0000
                && bank.size == 8 * 1024
        }));
    }
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
    println!("cargo:rerun-if-changed=build.rs");
}
