use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    configure_ram();
    let time_driver = select_time_driver();
    let x030 = matches!(metadata.line, "CW32F030" | "CW32A030");
    // Own-family-qualified DMA controller; peripheral requests are bounded below.
    let f020 = metadata.line == "CW32F020";
    let l012_dma = metadata.line == "CW32L012";
    let four_channel_dma = matches!(
        metadata.line,
        "CW32L031" | "CW32L052" | "CW32R031"
    );
    let dma = x030 || metadata.line == "CW32L083" || f020 || four_channel_dma || l012_dma;
    let classic_adc = matches!(
        metadata.line,
        "CW32F030"
            | "CW32A030"
            | "CW32F020"
            | "CW32F002"
            | "CW32F003"
            | "CW32L031"
            | "CW32R031"
            | "CW32L052"
            | "CW32L083"
    );
    let sequence_adc = matches!(metadata.line, "CW32L010" | "CW32L011");
    let adc = classic_adc || sequence_adc;
    let dual_adc = metadata.line == "CW32L012";
    let uart = true;
    let spi = true;
    let i2c = true;
    let serial = uart || spi || i2c;
    let cfgs = [
        "mco_f030",
        "rcc_hse",
        "rcc_lse",
        "rcc_lse_startup_analog",
        "rcc_lse_native_consumers",
        "rcc_pll",
        "rcc_lsi_sysclk",
        "rcc_external_clock",
        "aes_cw32l083_v1",
        "trng_cw32l083_v1",
        "cordic_cw32l012_v1",
        "eau_cw32l012_v1",
        "adc",
        "vc",
        "dac_cw32l012_v1",
        "opa_cw32l012_v1",
        "vc_two_speed",
        "vc_ready",
        "vc_cr2",
        "vref",
        "vref_vcdiv",
        "vref_vc2ref",
        "lvd_low",
        "ir_sysctrl",
        "ir_modes",
        "ir_software",
        "ir_invert",
        "rtc",
        "rtc_alarm_direct_access",
        "rtc_native_live_lsi",
        "awt",
        "awt_lse",
        "autotrim",
        "lptim",
        "lcd",
        "lptim_cr0",
        "halltim_cw32l012_v1",
        "flash",
        "flash_v1",
        "flash_cw32f002_v1",
        "flash_cw32f003_v1",
        "flash_cw32f020_v1",
        "flash_cw32l031_v1",
        "flash_cw32l010_v1",
        "flash_cw32l011_v1",
        "flash_cw32l012_v1",
        "flash_cw32l083_v1",
        "gpio_v1",
        "gpio_cw32f002_v1",
        "gpio_cw32l031_v1",
        "gpio_cw32l010_v1",
        "gpio_cw32l011_v1",
        "gpio_cw32l012_v1",
        "rcc_v1",
        "rcc_cw32f020_v1",
        "rcc_cw32f002_v1",
        "rcc_cw32f003_v1",
        "rcc_cw32l031_v1",
        "rcc_cw32l010_v1",
        "rcc_cw32l011_v1",
        "rcc_cw32l012_v1",
        "gpio_cw32l052_v1",
        "gpio_cw32l083_v1",
        "rcc_cw32l052_v1",
        "rcc_cw32l083_v1",
        "cw32l052",
        "cw32l083",
        "cw32x030",
        "gpio_af",
        "gpio_exti",
        "gpio_irq_level",
        "crc_poly_8005",
        "crc_input_16bit",
        "crc_input_32bit",
        "wwdt",
        "crc",
        "crc_32bit",
        "uart",
        "uart_v1",
        "uart_cw32f002_v1",
        "uart_cw32l010_v1",
        "uart_cw32l012_v1",
        "uart_cw32l052_v1",
        "uart_cw32l083_v1",
        "uart_cw32l031_v1",
        "spi",
        "spi_v1",
        "spi_cw32f002_v1",
        "spi_cw32l010_v1",
        "spi_cw32l012_v1",
        "spi_cw32l031_v1",
        "i2c_cw32l031_v1",
        "i2c_cw32l012_v1",
        "i2c",
        "i2c_v1",
        "i2c_cw32f002_v1",
        "i2c_cw32l010_v1",
        "i2c_cw32l011_v1",
        "dma_v1",
        "dma_cw32f020_v1",
        "dma_cw32l031_v1",
        "dma_cw32l012_v1",
        "uart_dma_tx",
        "uart_dma_rx",
        "spi_dma",
        "spi_async_pio",
        "adc_cw32l010_v1",
        "adc_cw32l011_v1",
        "adc_cw32l012_v1",
        "adc_v1",
        "adc_cw32f020_v1",
        "adc_cw32f002_v1",
        "adc_cw32f003_v1",
        "adc_cw32l031_v1",
        "adc_cw32l052_v1",
        "adc_cw32l083_v1",
        "atim",
        "atim_classic",
        "atim_buffered",
        "atim_classic_complementary",
        "atim_classic_input",
        "atim_v1",
        "atim_cw32f003_v1",
        "atim_cw32l052_v1",
        "atim_cw32l010_v1",
        "atim_cw32l012_v1",
        "gtim_classic",
        "gtim_buffered",
        "gtim_cw32l010_v1",
        "gtim_cw32l012_v1",
        "gtim_v1",
        "gtim_cw32f002_v1",
        "gtim_cw32l031_v1",
        "gtim_cw32l052_v1",
        "rtc_v1",
        "rtc_cw32f020_v1",
        "rtc_cw32l010_v1",
        "rtc_cw32l031_v1",
        "rtc_cw32l052_v1",
        "rtc_cw32l011_v1",
        "rtc_cw32l012_v1",
        "trigger_btim1_update",
        "btim",
        "btim_v1",
        "btim_cw32f002_v1",
        "btim_cw32l031_v1",
        "btim_cw32l052_v1",
        "btim_cw32l010_v1",
        "btim_cw32l012_v1",
        "iwdt",
        "cw32f002",
        "cw32f003",
        "cw32f020",
        "cw32l010",
        "cw32l011",
        "cw32l012",
        "cw32l031",
        "cw32r031",
    ];
    for cfg in cfgs {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }
    generate_trigger_routes();
    // Own-manual-qualified timer IP; version selection is checked before use.
    for (name, versions) in [
        ("AWT", &["v1", "cw32f030_v1", "cw32l031_v1"][..]),
        ("AUTOTRIM", &["cw32l052_v1", "cw32l083_v1"][..]),
        ("LPTIM", &["cw32l010_v1", "cw32l012_v1", "cw32l083_v1"][..]),
    ] {
        if let Some(p) = metadata.peripherals.iter().find(|p| p.name == name) {
            let registers = p.registers.as_ref().unwrap();
            assert!(
                versions.contains(&registers.version),
                "unreviewed low-power timer IP"
            );
            println!("cargo:rustc-cfg={}", name.to_ascii_lowercase());
            if registers.version == "cw32l012_v1" {
                println!("cargo:rustc-cfg=lptim_cr0");
            }
        }
    }
    if let Some(p) = metadata.peripherals.iter().find(|p| p.name == "HALLTIM") {
        let r = p.registers.as_ref().unwrap();
        assert_eq!(metadata.line, "CW32L012");
        assert_eq!((r.kind, r.version), ("halltim", "cw32l012_v1"));
        assert!(p.rcc.is_some() && p.rcc_control.is_some() && !p.pins.is_empty());
        println!("cargo:rustc-cfg=halltim_cw32l012_v1");
    }
    if let Some(p) = metadata.peripherals.iter().find(|p| p.name == "LCD") {
        assert!(p.lcd.is_some(), "LCD needs source-qualified chip facts");
        println!("cargo:rustc-cfg=lcd");
    }
    // Capabilities come from the selected, reviewed register IR, not family aliases.
    let comparators: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "vc"))
        .collect();
    if let Some(first) = comparators.first() {
        println!("cargo:rustc-cfg=vc");
        let regs = first.registers.as_ref().unwrap();
        let control = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case("CR0"))
            .unwrap();
        let response = control
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case("RESP"))
            .unwrap();
        assert_eq!(response.enumm, Some("ResponseSpeed"));
        assert!([1, 2].contains(&response.bit_size));
        if response.bit_size == 1 {
            println!("cargo:rustc-cfg=vc_two_speed");
        }
        let status = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case("SR"))
            .unwrap();
        if status
            .fields
            .iter()
            .any(|f| f.name.eq_ignore_ascii_case("READY"))
        {
            println!("cargo:rustc-cfg=vc_ready");
        }
        if regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap()
            .items
            .iter()
            .any(|r| r.name.eq_ignore_ascii_case("CR2"))
        {
            println!("cargo:rustc-cfg=vc_cr2");
        }
        for p in &comparators {
            assert_eq!(p.registers.as_ref().unwrap().version, regs.version);
            assert!(
                p.rcc_control.is_some(),
                "comparator clock control is unqualified"
            );
            assert_eq!(p.comparator_limits, first.comparator_limits);
        }
    }
    let reference_dividers: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.reference_divider.is_some())
        .collect();
    for peripheral in &reference_dividers {
        let facts = peripheral.reference_divider.as_ref().unwrap();
        let registers = peripheral.registers.as_ref().unwrap();
        assert!(matches!(registers.kind, "vcdiv" | "vc2ref"));
        assert_eq!(facts.consumers.len(), 2);
        assert!(facts.consumers.contains(&facts.clock_owner));
        let fieldset = if registers.kind == "vcdiv" {
            "DIV"
        } else {
            "REF"
        };
        let control = registers
            .ir
            .fieldsets
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(fieldset))
            .unwrap();
        let divider = control
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case("DIV"))
            .unwrap();
        let input = control
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case("VIN"))
            .unwrap();
        assert_eq!(divider.enumm, Some("DividerRatio"));
        assert_eq!(
            divider.bit_size,
            if registers.kind == "vcdiv" { 3 } else { 4 }
        );
        assert_eq!(input.enumm, Some("InputSource"));
        let clock = metadata
            .peripherals
            .iter()
            .find(|p| p.name == facts.clock_owner)
            .unwrap();
        assert!(clock.rcc_control.is_some());
        for name in facts.consumers {
            let consumer = comparators.iter().find(|p| p.name == *name).unwrap();
            assert_eq!(consumer.rcc_control, clock.rcc_control);
        }
        println!("cargo:rustc-cfg=vref");
        println!("cargo:rustc-cfg=vref_{}", registers.kind);
    }
    for kind in ["dac", "opa"] {
        for p in metadata.peripherals.iter().filter(|p| {
            p.registers.as_ref().is_some_and(|r| {
                if kind == "gpio" {
                    matches!(r.kind.as_ref(), "gpio" | "gpioc" | "gpiof")
                } else {
                    r.kind == kind
                }
            })
        }) {
            assert_eq!(p.registers.as_ref().unwrap().version, "cw32l012_v1");
            assert!(p.rcc_control.is_some());
            println!("cargo:rustc-cfg={kind}_cw32l012_v1");
        }
    }
    let assert_version = |name: &str, expected: &str| {
        let registers = metadata
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .expect("verified backend peripheral is missing")
            .registers
            .as_ref()
            .expect("verified backend registers are missing");
        assert_eq!(
            registers.version, expected,
            "{name}: unreviewed register version for {}",
            metadata.line
        );
    };
    if let Some(peripheral) = metadata.peripherals.iter().find(|p| p.name == "RTC") {
        let regs = peripheral.registers.as_ref().unwrap();
        assert!(matches!(
            regs.version,
            "v1" | "cw32f020_v1"
                | "cw32l010_v1"
                | "cw32l011_v1"
                | "cw32l012_v1"
                | "cw32l031_v1"
                | "cw32l052_v1"
        ));
        assert!(peripheral.rtc_calendar.is_some());
        println!("cargo:rustc-cfg=rtc");
        println!("cargo:rustc-cfg=rtc_{}", regs.version);
        let alarms = peripheral
            .rtc_alarms
            .as_ref()
            .expect("RTC alarm source facts");
        assert_eq!(alarms.alarm_a_ignore_bit, 1);
        assert!(!alarms.alarm_b_configuration_supported);
        if alarms.async_wait {
            assert!(alarms.direct_register_access);
            assert_eq!(peripheral.interrupts.len(), 1);
            assert_eq!(peripheral.interrupts[0].interrupt, "RTC");
            assert_eq!(peripheral.interrupts[0].signal, "GLOBAL");
            assert!(
                !metadata
                    .peripherals
                    .iter()
                    .filter(|p| p.name != "RTC")
                    .any(|p| p.interrupts.iter().any(|i| i.interrupt == "RTC")),
                "RTC vector must be dedicated"
            );
            println!("cargo:rustc-cfg=rtc_alarm_direct_access");
        } else {
            assert!(!alarms.direct_register_access);
        }
    }
    if metadata.line == "CW32L012" {
        assert_version("CORDIC", "cw32l012_v1");
        assert_version("EAU", "cw32l012_v1");
        println!("cargo:rustc-cfg=cordic_cw32l012_v1");
        println!("cargo:rustc-cfg=eau_cw32l012_v1");
        generate_accelerator_domains();
    }
    if metadata.line == "CW32L083" {
        assert_version("AES", "cw32l083_v1");
        assert_version("TRNG", "cw32l083_v1");
        generate_crypto_facts();
        println!("cargo:rustc-cfg=aes_cw32l083_v1");
        println!("cargo:rustc-cfg=trng_cw32l083_v1");
    }
    // Each mapping is independently qualified against that family's BTIM chapter.
    let btim_version = match metadata.line {
        "CW32F030" | "CW32A030" | "CW32F020" => "v1",
        "CW32F002" | "CW32F003" => "cw32f002_v1",
        "CW32L031" | "CW32R031" | "CW32L083" => "cw32l031_v1",
        "CW32L052" => "cw32l052_v1",
        "CW32L010" | "CW32L011" => "cw32l010_v1",
        "CW32L012" => "cw32l012_v1",
        _ => panic!("unverified basic-timer family"),
    };
    for name in ["BTIM1", "BTIM2", "BTIM3"] {
        assert_version(name, btim_version);
    }
    println!("cargo:rustc-cfg=btim");
    println!("cargo:rustc-cfg=btim_{btim_version}");
    let atim_version = match metadata.line {
        "CW32F030" | "CW32A030" | "CW32L031" | "CW32R031" | "CW32L083" => Some("v1"),
        "CW32F003" => Some("cw32f003_v1"),
        "CW32L052" => Some("cw32l052_v1"),
        "CW32L010" | "CW32L011" => Some("cw32l010_v1"),
        "CW32L012" => Some("cw32l012_v1"),
        _ => None,
    };
    if let Some(version) = atim_version {
        assert_version("ATIM", version);
        println!("cargo:rustc-cfg=atim");
        println!("cargo:rustc-cfg=atim_{version}");
        println!(
            "cargo:rustc-cfg={}",
            if matches!(version, "cw32l010_v1" | "cw32l012_v1") {
                "atim_buffered"
            } else {
                "atim_classic"
            }
        );
    }
    let mut classic_atim_input = false;
    for peripheral in metadata.peripherals {
        match &peripheral.classic_atim_input {
            None => {}
            Some(cw32_metapac::metadata::PeripheralClassicAtimInput::ClassicPairedV1) => {
                assert_eq!(peripheral.name, "ATIM");
                let regs = peripheral
                    .registers
                    .as_ref()
                    .expect("ATIM input registers missing");
                assert_eq!(regs.kind, "atim");
                assert_eq!(regs.block, "Atim");
                assert!(matches!(regs.version, "v1" | "cw32f003_v1" | "cw32l052_v1"));
                assert_eq!(atim_version, Some(regs.version));
                assert!(
                    !classic_atim_input,
                    "duplicate native ATIM input qualification"
                );
                classic_atim_input = true;
            }
        }
    }
    if classic_atim_input {
        println!("cargo:rustc-cfg=atim_classic_input");
    }
    // GTIM versions are qualified independently of mere peripheral presence.
    let gtim_version = match metadata.line {
        "CW32F030" | "CW32A030" | "CW32F020" => Some("v1"),
        "CW32F002" | "CW32F003" => Some("cw32f002_v1"),
        "CW32L031" | "CW32R031" | "CW32L083" => Some("cw32l031_v1"),
        "CW32L052" => Some("cw32l052_v1"),
        "CW32L010" | "CW32L011" => Some("cw32l010_v1"),
        "CW32L012" => Some("cw32l012_v1"),
        _ => None,
    };
    if let Some(version) = gtim_version {
        for peripheral in metadata
            .peripherals
            .iter()
            .filter(|p| p.name.starts_with("GTIM"))
        {
            assert_version(peripheral.name, version);
        }
        if matches!(version, "cw32l010_v1" | "cw32l012_v1") {
            println!("cargo:rustc-cfg=gtim_buffered");
        } else {
            println!("cargo:rustc-cfg=gtim_classic");
        }
        println!("cargo:rustc-cfg=gtim_{version}");
    }
    let flash_version = match metadata.line {
        "CW32F030" | "CW32A030" => Some("v1"),
        "CW32F002" => Some("cw32f002_v1"),
        "CW32F003" => Some("cw32f003_v1"),
        "CW32F020" => Some("cw32f020_v1"),
        "CW32L031" | "CW32R031" | "CW32L052" => Some("cw32l031_v1"),
        "CW32L010" => Some("cw32l010_v1"),
        "CW32L011" => Some("cw32l011_v1"),
        "CW32L012" => Some("cw32l012_v1"),
        "CW32L083" => Some("cw32l083_v1"),
        _ => None,
    };
    let flash_storage = flash_version.is_some();
    let mut flash_storage_size = None;
    if let Some(version) = flash_version {
        assert_version("FLASH", version);
        println!("cargo:rustc-cfg=flash");
        println!("cargo:rustc-cfg=flash_{version}");
        // Only reviewed exact ordering codes qualify for this storage API.
        // Generic profiles lack memory metadata. Package-neutral aliases keep
        // their verified memory metadata but are not qualified for this API.
        let expected_size = match metadata.name {
            "CW32F002F3P7" | "CW32F002F3U7" => Some(16384),
            "CW32F003E4P7" | "CW32F003F4P7" | "CW32F003F4U7" => Some(20480),
            "CW32F020C6U7" | "CW32F020F6U7" | "CW32F020K6U7" | "CW32F030F6P7" => Some(32768),
            "CW32A030C8T7" | "CW32F030C8T7" | "CW32F030F8V7" | "CW32F030K8T7" | "CW32F030K8U7"
            | "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8P6" | "CW32L031F8U6"
            | "CW32L031K8U6" | "CW32L031K8V6" | "CW32L052C8T6" | "CW32L052R8S6"
            | "CW32L052R8T6" | "CW32R031C8U6" | "CW32L010F8P6"
            | "CW32L010F8U6" | "CW32L010Y8M6" | "CW32L011K8T6" | "CW32L011K8U6"
            | "CW32L012C8T6" | "CW32L012C8U6" => Some(65536),
            "CW32L083RBT6" => Some(131072),
            "CW32L083MCT6" | "CW32L083RCS6" | "CW32L083RCT6" | "CW32L083VCT6" => Some(262144),
            _ => None,
        };
        if let Some(expected_size) = expected_size {
            assert_eq!(metadata.memory.len(), 1);
            let flash = metadata.memory[0]
                .iter()
                .find(|m| m.name == "FLASH")
                .unwrap();
            assert_eq!(flash.kind, cw32_metapac::metadata::MemoryRegionKind::Flash);
            assert_eq!((flash.address, flash.size), (0, expected_size));
            flash_storage_size = Some(flash.size);
        }
    }
    if f020 {
        qualify_f020_dma();
        println!("cargo:rustc-cfg=dma_cw32f020_v1");
    } else if four_channel_dma {
        qualify_four_channel_dma();
        println!("cargo:rustc-cfg=dma_cw32l031_v1");
    } else if l012_dma {
        qualify_l012_dma();
        println!("cargo:rustc-cfg=dma_cw32l012_v1");
    } else if dma {
        assert_version("DMA", "v1");
        assert_eq!(metadata.dma_channels.len(), 5);
        for (index, channel) in metadata.dma_channels.iter().enumerate() {
            assert_eq!(channel.dma, "DMA");
            assert_eq!(channel.channel as usize, index);
            assert_version(&format!("DMACHANNEL{}", index + 1), "v1");
        }
        println!("cargo:rustc-cfg=dma_v1");
    }
    // Qualification is distinct from a shared register layout. The five classic
    // families have own RX evidence; L012 remains TX-only. Inventory is not admission.
    // Keep SPI's paired directions independent of the UART direction policy.
    let uart_dma_profile = match metadata.line {
        "CW32F030" | "CW32A030" => Some((3, "v1", true)),
        "CW32L083" => Some((6, "cw32l083_v1", true)),
        "CW32F020" => Some((3, "v1", true)),
        "CW32L031" | "CW32R031" => Some((3, "cw32l031_v1", true)),
        "CW32L052" => Some((3, "cw32l052_v1", true)),
        "CW32L012" => Some((3, "cw32l012_v1", true)),
        _ => None,
    };
    let spi_dma_profile = match metadata.line {
        "CW32F030" | "CW32A030" | "CW32F020" => Some((2, "v1")),
        "CW32L083" | "CW32L052" => Some((2, "cw32l031_v1")),
        "CW32L031" | "CW32R031" => Some((1, "cw32l031_v1")),
        "CW32L012" => Some((3, "cw32l012_v1")),
        _ => None,
    };
    // Own-manual IRQ byte-master qualification is separate from DMA admission.
    // Consume the existing SPI/IRQ facts; shared IP alone grants no capability.
    let spi_async_pio_profile: Option<(&str, &[(&str, &str, u32)])> = match metadata.line {
        "CW32F002" | "CW32F003" => Some(("cw32f002_v1", &[("SPI", "SPI", 25)])),
        "CW32F020" => Some(("v1", &[("SPI1", "SPI1", 25), ("SPI2", "SPI2", 26)])),
        "CW32L031" | "CW32R031" => Some(("cw32l031_v1", &[("SPI1", "SPI1", 25)])),
        "CW32L052" => Some(("cw32l031_v1", &[("SPI1", "SPI1", 25), ("SPI2", "SPI2", 26)])),
        "CW32L010" | "CW32L011" => Some(("cw32l010_v1", &[("SPI", "SPI", 25)])),
        "CW32L012" => Some((
            "cw32l012_v1",
            &[
                ("SPI1", "SPI1", 25),
                ("SPI2", "SPI23", 26),
                ("SPI3", "SPI23", 26),
            ],
        )),
        _ => None,
    };
    if let Some((version, instances)) = spi_async_pio_profile {
        assert!(spi);
        let actual_instances = metadata
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "spi"))
            .count();
        assert_eq!(
            actual_instances,
            instances.len(),
            "unreviewed IRQ SPI inventory"
        );
        for &(name, irq_name, irq_number) in instances {
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|p| p.name == name)
                .expect("qualified IRQ SPI instance is missing");
            let registers = peripheral
                .registers
                .as_ref()
                .expect("qualified IRQ SPI registers are missing");
            assert_eq!(
                (registers.kind, registers.version, registers.block),
                ("spi", version, "Spi")
            );
            assert_eq!(
                peripheral.interrupts.len(),
                1,
                "unreviewed IRQ SPI source count"
            );
            let irq = &peripheral.interrupts[0];
            assert_eq!((irq.signal, irq.interrupt), ("GLOBAL", irq_name));
            let vector = metadata
                .interrupts
                .iter()
                .find(|i| i.name == irq.interrupt)
                .expect("qualified IRQ SPI vector is missing");
            assert_eq!(vector.number, irq_number);
            // L012 SPI2/3 share SPI23; every other admitted SPI vector is dedicated.
            let mut actual_peers: Vec<_> = metadata
                .peripherals
                .iter()
                .filter(|p| p.interrupts.iter().any(|i| i.interrupt == irq.interrupt))
                .map(|p| p.name)
                .collect();
            let mut expected_peers: Vec<_> = instances
                .iter()
                .filter(|(_, expected_irq, _)| *expected_irq == irq_name)
                .map(|(name, _, _)| *name)
                .collect();
            actual_peers.sort_unstable();
            expected_peers.sort_unstable();
            assert_eq!(
                actual_peers, expected_peers,
                "unreviewed IRQ SPI shared vector"
            );
        }
        println!("cargo:rustc-cfg=spi_async_pio");
    }
    let qualify_requests = |prefix: &str, count: u8, version: &str, signals: &[&str]| {
        assert!(dma, "staged requests require a qualified DMA controller");
        let mut pairs = Vec::new();
        for index in 1..=count {
            let name = format!("{prefix}{index}");
            assert_version(&name, version);
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|p| p.name == name)
                .unwrap();
            if matches!(metadata.line, "CW32F030" | "CW32A030" | "CW32L083") {
                assert_eq!(peripheral.dma_channels.len(), 2);
            }
            let mut selectors = Vec::new();
            for &signal in signals {
                let routes: Vec<_> = peripheral
                    .dma_channels
                    .iter()
                    .filter(|r| r.signal == signal)
                    .collect();
                assert_eq!(
                    routes.len(),
                    1,
                    "staged direction requires one qualified route"
                );
                let route = routes[0];
                // The reviewed request table applies to every physical channel.
                // No channel-limited/muxed/remapped record may become a blanket impl.
                assert_eq!(route.dma, Some("DMA"));
                assert!(
                    route.channel.is_none() && route.dmamux.is_none() && route.remap.is_empty()
                );
                let request = route.request.expect("DMA request missing");
                assert!(request < 64);
                assert!(!selectors.contains(&request));
                selectors.push(request);
                pairs.push((peripheral.name, route.signal));
            }
        }
        pairs
    };
    let (uart_dma_tx_pairs, uart_dma_rx_pairs) =
        if let Some((uarts, uart_ip, receive)) = uart_dma_profile {
            (
                qualify_requests("UART", uarts, uart_ip, &["TX"]),
                if receive {
                    qualify_requests("UART", uarts, uart_ip, &["RX"])
                } else {
                    Vec::new()
                },
            )
        } else {
            (Vec::new(), Vec::new())
        };
    // RX deliberately reuses TxDma::validate_staging. An RX-only profile needs
    // separate review; it must not accidentally compile without that helper.
    assert!(
        uart_dma_rx_pairs
            .iter()
            .all(|(name, _)| { uart_dma_tx_pairs.contains(&(*name, "TX")) })
    );
    for &(name, _) in &uart_dma_rx_pairs {
        let peripheral = metadata
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .unwrap();
        let request = |signal| {
            peripheral
                .dma_channels
                .iter()
                .find(|route| route.signal == signal)
                .unwrap()
                .request
        };
        assert_ne!(request("RX"), request("TX"));
    }
    let spi_dma_pairs = if let Some((count, version)) = spi_dma_profile {
        if metadata.line == "CW32L012" {
            // Native DMA admission above reached qualify_l012_dma: four
            // channels, DMACH12/34 and exact memory (or the empty generic map).
            assert!(l012_dma);
            assert_eq!((count, version), (3, "cw32l012_v1"));
            // Reuse the independently qualified PIO inventory and complete
            // GLOBAL/vector peer checks above for the required SPI error IRQ.
            let (pio_version, instances) = spi_async_pio_profile
                .expect("native SPI DMA requires its qualified IRQ inventory");
            assert_eq!(pio_version, version);
            assert_eq!(
                instances,
                &[
                    ("SPI1", "SPI1", 25),
                    ("SPI2", "SPI23", 26),
                    ("SPI3", "SPI23", 26),
                ]
            );
        }
        let pairs = qualify_requests("SPI", count, version, &["RX", "TX"]);
        if matches!(
            metadata.line,
            "CW32F020" | "CW32L031" | "CW32L052" | "CW32R031" | "CW32L012"
        ) {
            for index in 1..=count {
                let name = format!("SPI{index}");
                let peripheral = metadata
                    .peripherals
                    .iter()
                    .find(|p| p.name == name)
                    .unwrap();
                assert_eq!(peripheral.dma_channels.len(), 2);
                for (signal, selector) in [
                    ("RX", 4 + 2 * u32::from(index)),
                    ("TX", 5 + 2 * u32::from(index)),
                ] {
                    let route = peripheral
                        .dma_channels
                        .iter()
                        .find(|route| route.signal == signal)
                        .unwrap();
                    assert_eq!(route.request, Some(selector));
                }
            }
        }
        pairs
    } else {
        Vec::new()
    };
    if !uart_dma_tx_pairs.is_empty() {
        println!("cargo:rustc-cfg=uart_dma_tx");
    }
    if !uart_dma_rx_pairs.is_empty() {
        println!("cargo:rustc-cfg=uart_dma_rx");
    }
    if !spi_dma_pairs.is_empty() {
        println!("cargo:rustc-cfg=spi_dma");
    }
    let selected: &[&str] = match metadata.line {
        "CW32F030" | "CW32A030" => {
            assert_version("GPIOA", "v1");
            assert_version("SYSCTRL", "v1");
            assert_version("ADC", "v1");
            for name in ["GTIM1", "GTIM2", "GTIM3", "GTIM4"] {
                assert_version(name, "v1");
            }
            &[
                "gpio_v1",
                "rcc_v1",
                "cw32x030",
                "gpio_exti",
                "crc_32bit",
                "adc_v1",
                "gtim_v1",
            ]
        }
        "CW32F020" => {
            assert_version("FLASH", "cw32f020_v1");
            assert_version("GPIOA", "v1");
            assert_version("SYSCTRL", "cw32f020_v1");
            assert_version("CRC", "cw32f020_v1");
            assert_version("ADC", "cw32f020_v1");
            for name in ["GTIM1", "GTIM2", "GTIM3", "GTIM4"] {
                assert_version(name, "v1");
            }
            &[
                "gpio_v1",
                "rcc_cw32f020_v1",
                "cw32f020",
                "adc_cw32f020_v1",
                "gtim_v1",
            ]
        }
        "CW32F002" => {
            assert_version("SYSCTRL", "cw32f002_v1");
            assert_version("FLASH", "cw32f002_v1");
            assert_version("GPIOA", "cw32f002_v1");
            assert_version("ADC", "cw32f002_v1");
            &[
                "gpio_cw32f002_v1",
                "rcc_cw32f002_v1",
                "cw32f002",
                "adc_cw32f002_v1",
            ]
        }
        "CW32F003" => {
            assert_version("SYSCTRL", "cw32f003_v1");
            assert_version("FLASH", "cw32f003_v1");
            assert_version("GPIOA", "cw32f002_v1");
            assert_version("ADC", "cw32f003_v1");
            &[
                "gpio_cw32f002_v1",
                "rcc_cw32f003_v1",
                "cw32f003",
                "adc_cw32f003_v1",
            ]
        }
        "CW32L010" => {
            assert_version("SYSCTRL", "cw32l010_v1");
            assert_version("FLASH", "cw32l010_v1");
            assert_version("GPIOA", "cw32l010_v1");
            {
                assert_version("ADC", "cw32l010_v1");
                &[
                    "gpio_cw32l010_v1",
                    "rcc_cw32l010_v1",
                    "cw32l010",
                    "adc_cw32l010_v1",
                ]
            }
        }
        "CW32L011" => {
            assert_version("SYSCTRL", "cw32l011_v1");
            assert_version("FLASH", "cw32l011_v1");
            assert_version("GPIOA", "cw32l011_v1");
            {
                assert_version("ADC", "cw32l011_v1");
                &[
                    "gpio_cw32l011_v1",
                    "rcc_cw32l011_v1",
                    "cw32l011",
                    "adc_cw32l011_v1",
                ]
            }
        }
        "CW32L012" => {
            for name in ["GPIOA", "SYSCTRL", "FLASH"] {
                assert_version(name, "cw32l012_v1");
            }
            for name in ["ADC1", "ADC2"] {
                assert_version(name, "cw32l012_v1");
            }
            assert_version("BGR", "cw32l012_v1");
            &[
                "gpio_cw32l012_v1",
                "rcc_cw32l012_v1",
                "cw32l012",
                "adc_cw32l012_v1",
            ]
        }
        "CW32L031" | "CW32R031" => {
            for name in ["GPIOA", "SYSCTRL", "FLASH"] {
                assert_version(name, "cw32l031_v1");
            }
            assert_version("ADC", "cw32l031_v1");
            match metadata.line {
                "CW32L031" => &[
                    "gpio_cw32l031_v1",
                    "rcc_cw32l031_v1",
                    "cw32l031",
                    "adc_cw32l031_v1",
                ],
                "CW32R031" => &[
                    "gpio_cw32l031_v1",
                    "rcc_cw32l031_v1",
                    "cw32r031",
                    "adc_cw32l031_v1",
                ],
                _ => unreachable!(),
            }
        }
        "CW32L052" => {
            assert_version("GPIOA", "cw32l052_v1");
            assert_version("SYSCTRL", "cw32l052_v1");
            assert_version("FLASH", "cw32l031_v1");
            assert_version("ADC", "cw32l052_v1");
            &[
                "gpio_cw32l052_v1",
                "rcc_cw32l052_v1",
                "cw32l052",
                "adc_cw32l052_v1",
            ]
        }
        "CW32L083" => {
            for name in ["GPIOA", "SYSCTRL", "FLASH"] {
                assert_version(name, "cw32l083_v1");
            }
            assert_version("ADC", "cw32l083_v1");
            &[
                "gpio_cw32l083_v1",
                "rcc_cw32l083_v1",
                "cw32l083",
                "adc_cw32l083_v1",
            ]
        }
        _ => panic!("HAL backend is not implemented for this family"),
    };
    for cfg in selected {
        println!("cargo:rustc-cfg={cfg}");
    }
    // Presence is independent of register version. Backend assertions above
    // qualify the selected device before exposing its peripheral module.
    if metadata
        .peripherals
        .iter()
        .any(|p| p.registers.as_ref().is_some_and(|r| r.kind == "adc"))
    {
        println!("cargo:rustc-cfg=adc");
    }
    if x030 {
        assert_version("FLASH", "v1");
    }
    // All GPIO backends now have reviewed IRQ field/command masks.
    let exti = true;
    if exti {
        println!("cargo:rustc-cfg=gpio_exti");
    }
    assert_version(
        "CRC",
        match metadata.line {
            "CW32F002" | "CW32F003" => "cw32f002_v1",
            "CW32F020" => "cw32f020_v1",
            "CW32L010" | "CW32L011" | "CW32L012" => "cw32l010_v1",
            "CW32L031" | "CW32R031" | "CW32L083" => "cw32l031_v1",
            "CW32L052" => "cw32l052_v1",
            _ => "v1",
        },
    );
    println!("cargo:rustc-cfg=crc");
    if !matches!(metadata.line, "CW32F002" | "CW32F003") {
        println!("cargo:rustc-cfg=crc_poly_8005");
    }
    if x030 || f020 {
        println!("cargo:rustc-cfg=crc_input_16bit");
        println!("cargo:rustc-cfg=crc_input_32bit");
    }
    if !matches!(metadata.line, "CW32L010" | "CW32L011") {
        assert_version(
            "WWDT",
            match metadata.line {
                "CW32L012" => "cw32l012_v1",
                "CW32L031" | "CW32L052" | "CW32L083" | "CW32R031" => "cw32l031_v1",
                _ => "v1",
            },
        );
        let vector = metadata
            .interrupts
            .iter()
            .find(|irq| irq.name == "WDT")
            .expect("qualified WWDT requires the shared WDT vector");
        assert_eq!(vector.number, 0, "unexpected WWDT vector number");
        println!("cargo:rustc-cfg=wwdt");
    }
    for (enabled, kind, prefix, version) in [
        (
            uart,
            "uart",
            "UART",
            match metadata.line {
                "CW32F002" | "CW32F003" => "cw32f002_v1",
                "CW32L010" | "CW32L011" => "cw32l010_v1",
                "CW32L012" => "cw32l012_v1",
                "CW32L052" => "cw32l052_v1",
                "CW32L083" => "cw32l083_v1",
                "CW32L031" | "CW32R031" => "cw32l031_v1",
                _ => "v1",
            },
        ),
        (
            spi,
            "spi",
            "SPI",
            match metadata.line {
                "CW32F002" | "CW32F003" => "cw32f002_v1",
                "CW32L010" | "CW32L011" => "cw32l010_v1",
                "CW32L012" => "cw32l012_v1",
                "CW32L031" | "CW32R031" | "CW32L052" | "CW32L083" => "cw32l031_v1",
                _ => "v1",
            },
        ),
        (
            i2c,
            "i2c",
            "I2C",
            match metadata.line {
                "CW32F002" | "CW32F003" => "cw32f002_v1",
                "CW32L010" => "cw32l010_v1",
                "CW32L011" => "cw32l011_v1",
                "CW32L012" => "cw32l012_v1",
                "CW32L031" | "CW32R031" | "CW32L052" | "CW32L083" => "cw32l031_v1",
                _ => "v1",
            },
        ),
    ] {
        if enabled {
            for p in metadata
                .peripherals
                .iter()
                .filter(|p| p.name.starts_with(prefix))
            {
                assert_version(p.name, version);
            }
            println!("cargo:rustc-cfg={kind}");
            println!("cargo:rustc-cfg={kind}_{version}");
        }
    }
    if serial {
        println!("cargo:rustc-cfg=gpio_af");
    }
    assert_version(
        "IWDT",
        if matches!(
            metadata.line,
            "CW32L031" | "CW32R031" | "CW32L052" | "CW32L083"
        ) {
            "cw32l031_v1"
        } else {
            "v1"
        },
    );
    println!("cargo:rustc-cfg=iwdt");
    assert_eq!(metadata.nvic_priority_bits, Some(2));
    let mut out =
        String::from("// Generated from selected-chip PAC metadata and reviewed pinout data.\n");
    if let Some(facts) = metadata
        .peripherals
        .iter()
        .find(|p| p.name == "ATIM")
        .and_then(|p| p.atim_complementary.as_ref())
    {
        let expected = if matches!(atim_version, Some("cw32l010_v1" | "cw32l012_v1")) {
            (4, 1008, 1)
        } else {
            assert_eq!(atim_version, Some("v1"));
            assert!(matches!(metadata.line, "CW32F030" | "CW32A030"));
            println!("cargo:rustc-cfg=atim_classic_complementary");
            (3, 1010, 0)
        };
        assert_eq!(
            (
                facts.channels,
                facts.dead_time_max_ticks,
                facts.break_inputs
            ),
            expected
        );
        out.push_str(&format!("impl crate::timer::sealed::ComplementaryInstance for crate::peripherals::ATIM {{ const DEAD_TIME_MAX_TICKS: u16 = {}; }}\nimpl crate::timer::ComplementaryInstance for crate::peripherals::ATIM {{}}\n", facts.dead_time_max_ticks));
    }

    if dma {
        assert!(metadata.memory.len() <= 1);
        let sram = metadata
            .memory
            .first()
            .and_then(|regions| {
                regions
                    .iter()
                    .find(|region| region.kind == cw32_metapac::metadata::MemoryRegionKind::Ram)
            })
            .map(|region| {
                assert_eq!(region.address, 0x2000_0000);
                // Capacity is the selected part fact from parts.yaml, not a
                // common-family guess. Generic profiles have no memory map.
                assert!(region.size > 0);
                assert!(region.address.checked_add(region.size).is_some());
                (region.address, region.size)
            });
        out.push_str(&format!(
            "pub(crate) const DMA_CHANNEL_COUNT: usize = {};\npub(crate) const DMA_COPY_SRAM: Option<(u32, u32)> = {sram:?};\n",
            metadata.dma_channels.len()
        ));
    }
    generate_gpio(&mut out);
    generate_rcc(&mut out, time_driver);
    generate_time_driver(&mut out, time_driver);
    generate_gpio_interrupts(&mut out);
    generate_electrical(&mut out);
    qualify_awt_lse();
    generate_mco(&mut out, time_driver);
    generate_lvd_ir(&mut out);
    if flash_storage {
        out.push_str(&format!(
            "pub(crate) const FLASH_STORAGE_SIZE: Option<u32> = {flash_storage_size:?};\n"
        ));
    }
    out.push_str("embassy_hal_internal::peripherals! {\n");
    for peripheral in metadata.peripherals {
        if Some(peripheral.name) != time_driver {
            out.push_str(&format!("    {},\n", peripheral.name));
        }
    }
    for p in metadata.peripherals {
        if let Some(ir) = &p.ir {
            if !metadata.peripherals.iter().any(|p| p.name == ir.owner) {
                out.push_str(&format!("    {},\n", ir.owner));
            }
        }
    }
    for channel in metadata.dma_channels {
        out.push_str(&format!("    {},\n", channel.name));
    }
    for pin in metadata.pins {
        if safe_pin(pin.name) {
            out.push_str(&format!("    {},\n", pin.name));
        }
    }
    out.push_str("}\nembassy_hal_internal::interrupt_mod! {\n");
    for interrupt in metadata.interrupts {
        out.push_str(&format!("    {},\n", interrupt.name));
    }
    out.push_str("}\n");
    for peripheral in metadata
        .peripherals
        .iter()
        .filter(|p| matches!(p.name, "AWT" | "LPTIM" | "AUTOTRIM"))
    {
        let irq = peripheral
            .interrupts
            .iter()
            .find(|i| i.signal == "GLOBAL")
            .expect("reviewed timer GLOBAL interrupt is missing");
        let kind = peripheral.name.to_ascii_lowercase();
        out.push_str(&format!(
            "crate::{kind}::impl_instance!({}, {});\n",
            peripheral.name, irq.interrupt
        ));
    }
    for p in metadata.peripherals.iter().filter(|p| p.name == "HALLTIM") {
        out.push_str(&format!("crate::halltim::impl_instance!({});\n", p.name));
    }
    generate_lcd(&mut out);
    generate_rtc(&mut out);
    generate_native_live_lsi_rtc(&mut out);
    for pin in metadata.pins {
        if safe_pin(pin.name) {
            // Own manuals identify PB4 on F002/F003 and PB0 on other classic lines.
            // ExRef is not a conversion channel on F002/F003; qualify from the
            // bonded GPIO inventory instead of inventing an ADC input mux.
            let reference_pin = match metadata.line {
                "CW32F002" | "CW32F003" => Some("PB4"),
                "CW32F020" | "CW32F030" | "CW32A030" | "CW32L031" | "CW32L052"
                | "CW32L083" | "CW32R031" | "CW32W031" => Some("PB0"),
                _ => None,
            };
            if reference_pin == Some(pin.name) {
                assert!(metadata.peripherals.iter().any(|p| p.name == "ADC"));
                out.push_str(&format!(
                    "impl crate::adc::SealedExternalReferencePin<crate::peripherals::ADC> for crate::peripherals::{} {{}}\nimpl crate::adc::ExternalReferencePin<crate::peripherals::ADC> for crate::peripherals::{} {{}}\n",
                    pin.name, pin.name
                ));
            }
            let port = pin.name.as_bytes()[1] - b'A';
            let number: u8 = pin.name[2..].parse().expect("invalid pin metadata");
            out.push_str(&format!(
                "crate::gpio::impl_pin!({}, {}, {});\n",
                pin.name, port, number
            ));
            if exti {
                let bank = format!("GPIO{}", char::from(b'A' + port));
                let irq = metadata
                    .peripherals
                    .iter()
                    .find(|p| p.name == bank)
                    .and_then(|p| p.interrupts.iter().find(|i| i.signal == "GLOBAL"))
                    .expect("reviewed GPIO GLOBAL interrupt is missing");
                out.push_str(&format!(
                    "impl crate::exti::ExtiPin for crate::peripherals::{} {{ type Interrupt = crate::interrupt::typelevel::{}; }}\n",
                    pin.name, irq.interrupt
                ));
            }
        }
    }
    if serial {
        if uart {
            for peripheral in metadata
                .peripherals
                .iter()
                .filter(|p| p.name.starts_with("UART"))
            {
                let number: u8 = peripheral.name[4..]
                    .parse()
                    .expect("invalid UART instance number");
                let irq = peripheral
                    .interrupts
                    .iter()
                    .find(|i| i.signal == "GLOBAL")
                    .expect("verified UART global interrupt is missing");
                out.push_str(&format!(
                    "crate::usart::impl_instance!({}, {}, {});\n",
                    peripheral.name,
                    irq.interrupt,
                    number - 1
                ));
            }
        }
        if spi {
            for peripheral in metadata
                .peripherals
                .iter()
                .filter(|p| p.name.starts_with("SPI"))
            {
                let limits = peripheral
                    .spi
                    .as_ref()
                    .expect("verified SPI clock limits are missing");
                let irq = peripheral
                    .interrupts
                    .iter()
                    .find(|i| i.signal == "GLOBAL")
                    .expect("verified SPI global interrupt is missing");
                out.push_str(&format!(
                    "crate::spi::impl_instance!({}, {}, {}, {});\n",
                    peripheral.name,
                    limits.maximum_frequency,
                    limits.minimum_divisor,
                    irq.interrupt,
                ));
            }
        }
        for p in metadata.peripherals {
            if let Some(limits) = &p.dac_limits {
                out.push_str(&format!("crate::dac::impl_instance!({});\n", p.name));
                out.push_str(&format!("pub(crate) const DAC_SUPPLY_RANGE_MV: (u16,u16) = {:?};\npub(crate) const DAC_MAX_CODE: u16 = {};\n", limits.supply_mv, (1u16 << limits.resolution_bits) - 1));
                for pin in p.pins {
                    assert!(pin.af.is_none());
                    out.push_str(&format!(
                        "crate::dac::impl_pin!({}, {}, {});\n",
                        p.name,
                        pin.pin,
                        pin.signal.strip_prefix("OUT").unwrap()
                    ));
                }
            }
            if let Some(limits) = &p.opa_limits {
                out.push_str(&format!(
                    "crate::opamp::impl_instance!({}, {}, {:?}, {});\n",
                    p.name,
                    p.name.strip_prefix("OPA").unwrap(),
                    limits.supply_mv,
                    limits.output_headroom_mv
                ));
                for pin in p.pins {
                    assert!(pin.af.is_none());
                    out.push_str(&format!(
                        "crate::opamp::impl_pin!({}, {}, {});\n",
                        p.name, pin.pin, pin.signal
                    ));
                }
            }
        }
        for peripheral in &reference_dividers {
            let facts = peripheral.reference_divider.as_ref().unwrap();
            out.push_str(&format!(
                "crate::vref::impl_instance!({}, {}, {}, {}, {}, {:?}, {});\n",
                peripheral.name,
                facts.clock_owner,
                facts.consumers[0],
                facts.consumers[1],
                facts.negative_mux,
                facts.supply_mv,
                facts.input_uses_vdda
            ));
        }
        if matches!(metadata.line, "CW32L010" | "CW32L011") {
            for (name, number) in [("VC1", 14), ("VC2", 15)] {
                let vector = metadata
                    .interrupts
                    .iter()
                    .find(|irq| irq.name == name)
                    .expect("native comparator requires its dedicated vector");
                assert_eq!(vector.number, number, "unexpected comparator vector number");
            }
        }
        for peripheral in &comparators {
            out.push_str(&format!(
                "crate::comparator::impl_instance!({});\n",
                peripheral.name
            ));
        }
        if let Some(first) = comparators.first() {
            let limits = first
                .comparator_limits
                .as_ref()
                .expect("comparator electrical limits missing");
            out.push_str(&format!(
                "pub(crate) const COMPARATOR_SUPPLY_RANGE_MV: (u16,u16) = {:?};\n",
                limits.supply_mv
            ));
            out.push_str(&format!(
                "pub(crate) const COMPARATOR_INPUT_USES_VDDA: bool = {};\n",
                limits.input_uses_vdda
            ));
        }
        if adc {
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|p| p.registers.as_ref().is_some_and(|r| r.kind == "adc"))
                .expect("ADC instance missing");
            let links: Vec<_> = peripheral
                .interrupts
                .iter()
                .filter(|i| i.signal == "GLOBAL")
                .collect();
            assert_eq!(links.len(), 1, "software ADC requires one GLOBAL IRQ");
            let irq = links[0].interrupt;
            assert_eq!(
                metadata.interrupts.iter().filter(|i| i.name == irq).count(),
                1
            );
            assert!(
                !metadata
                    .peripherals
                    .iter()
                    .any(|other| other.name != peripheral.name
                        && other.interrupts.iter().any(|i| i.interrupt == irq)),
                "software ADC IRQ must be dedicated"
            );
            out.push_str(&format!(
                "crate::adc::impl_instance!({}, {});\n",
                peripheral.name, irq
            ));
        }
        if dual_adc {
            assert_l012_adc_shared_irq();
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|p| p.name == "ADC1")
                .expect("ADC1 missing");
            let links: Vec<_> = peripheral
                .interrupts
                .iter()
                .filter(|i| i.signal == "GLOBAL")
                .collect();
            assert_eq!(links.len(), 1, "ADC1 requires one GLOBAL IRQ");
            let irq = links[0].interrupt;
            assert_eq!(
                metadata.interrupts.iter().filter(|i| i.name == irq).count(),
                1
            );
            assert!(
                !metadata
                    .peripherals
                    .iter()
                    .any(|other| other.name != peripheral.name
                        && other.interrupts.iter().any(|i| i.interrupt == irq)),
                "ADC1 software IRQ must be dedicated"
            );
            out.push_str(&format!(
                "crate::adc::impl_instance!(ADC1, ADC2, {}, Dedicated);\n",
                irq
            ));
            out.push_str("crate::adc::impl_instance!(ADC2, ADC1, ADC2_DAC, SharedDac);\n");
        }
        if i2c {
            for peripheral in metadata
                .peripherals
                .iter()
                .filter(|p| p.name.starts_with("I2C"))
            {
                if peripheral.registers.as_ref().unwrap().version == "cw32l012_v1" {
                    let irq = command_i2c_irq(peripheral);
                    out.push_str(&format!(
                        "crate::i2c::impl_instance!({}, {});\n",
                        peripheral.name, irq
                    ));
                } else {
                    let irq = classic_i2c_irq(peripheral);
                    out.push_str(&format!(
                        "crate::i2c::impl_instance!({}, {});\n",
                        peripheral.name, irq
                    ));
                }
            }
        }
        if gtim_version.is_some() {
            for peripheral in metadata
                .peripherals
                .iter()
                .filter(|p| p.name.starts_with("GTIM") && Some(p.name) != time_driver)
            {
                let name = peripheral.name;
                let number: u8 = if name == "GTIM" {
                    1
                } else {
                    name[4..].parse().expect("invalid GTIM number")
                };
                let irq = timer_overflow_irq(peripheral);
                out.push_str(&format!(
                    "crate::timer::impl_gtim!({}, {}, {});\n",
                    name, number, irq
                ));
                if let Some(input) = &peripheral.classic_timer_input {
                    out.push_str(&format!(
                        "crate::timer::impl_classic_input!({}, {}, {:?});\n",
                        name,
                        input.capture_mux.to_ascii_lowercase(),
                        input.encoder_fixed_reload
                    ));
                }
            }
        }
        if atim_version.is_some() {
            let peripheral = metadata
                .peripherals
                .iter()
                .find(|p| p.name == "ATIM")
                .unwrap();
            let irq = timer_overflow_irq(peripheral);
            out.push_str(&format!("crate::timer::impl_atim_overflow!({irq});\n"));
            if classic_atim_input {
                out.push_str("crate::timer::impl_classic_atim_input!();\n");
            }
        }
        generate_btim_interrupts(&mut out, btim_version);
        if dma {
            for channel in metadata.dma_channels {
                let physical = format!("DMACHANNEL{}", channel.channel + 1);
                let peripheral = metadata
                    .peripherals
                    .iter()
                    .find(|p| p.name == physical)
                    .expect("DMA register instance missing");
                let irq = peripheral
                    .interrupts
                    .iter()
                    .find(|i| i.signal == "GLOBAL")
                    .expect("DMA shared interrupt mapping missing");
                out.push_str(&format!(
                    "crate::dma::impl_channel!({}, {}, {});\n",
                    channel.name, channel.channel, irq.interrupt
                ));
            }
        }
        for peripheral in metadata.peripherals {
            if Some(peripheral.name) == time_driver {
                continue;
            }
            // Preserve x030's raw inventory. Other families emit only verified
            // direction pairs: L083 and the five classic families UART/SPI
            // RX+TX, and L012 UART TX alone.
            for route in peripheral.dma_channels {
                let staged_pair = uart_dma_tx_pairs
                    .iter()
                    .chain(&uart_dma_rx_pairs)
                    .chain(&spi_dma_pairs)
                    .any(|&(name, signal)| name == peripheral.name && signal == route.signal);
                if (x030 || staged_pair) && route.dma == Some("DMA") {
                    let request = route.request.expect("DMA request missing");
                    out.push_str(&format!(
                        "crate::dma::impl_request!({}_{}, {}, {}, {});\n",
                        peripheral.name, route.signal, peripheral.name, route.signal, request
                    ));
                }
            }
            for pin in peripheral.pins {
                if !safe_pin(pin.pin) {
                    continue;
                }
                assert!(
                    metadata.pins.iter().any(|p| p.name == pin.pin),
                    "AF pin not bonded on selected chip"
                );
                if (adc && peripheral.name == "ADC")
                    || (dual_adc && matches!(peripheral.name, "ADC1" | "ADC2"))
                {
                    if pin.signal.starts_with("IN") {
                        // Source labels are not always hardware mux values: R031
                        // ADC_IN0..8 select mux4..12. Use explicit reviewed metadata.
                        let channel = pin
                            .adc_mux
                            .expect("ADC pin route requires an explicit hardware mux");
                        assert!(
                            channel
                                <= if dual_adc {
                                    11
                                } else if sequence_adc {
                                    13
                                } else {
                                    12
                                },
                            "external ADC mux is not a verified pin route"
                        );
                        assert!(pin.af.is_none(), "analog route cannot use a digital AF");
                        out.push_str(&format!(
                            "crate::adc::impl_pin!({}, {}, {});\n",
                            peripheral.name, pin.pin, channel
                        ));
                    }
                    continue;
                }
                if peripheral
                    .registers
                    .as_ref()
                    .is_some_and(|r| r.kind == "vc")
                {
                    // Digital output AF metadata remains PAC-only in this polling subset.
                    let Some(mux) = pin.comparator_mux else {
                        continue;
                    };
                    let direction = if pin.signal.starts_with("INP") {
                        "INP"
                    } else if pin.signal.starts_with("INN") {
                        "INN"
                    } else {
                        panic!("unsupported comparator route")
                    };
                    assert!(pin.af.is_none() && pin.adc_mux.is_none());
                    out.push_str(&format!(
                        "crate::comparator::impl_pin!({}, {}, {}, {});\n",
                        peripheral.name, pin.pin, direction, mux
                    ));
                    continue;
                }
                let Some(af) = pin.af else {
                    continue;
                };
                if peripheral.name == "HALLTIM" {
                    let role = match pin.signal {
                        "CH1" => "Ch1",
                        "CH2" => "Ch2",
                        "CH3" => "Ch3",
                        _ => panic!("unreviewed Hall input signal"),
                    };
                    assert_eq!(af, 9);
                    out.push_str(&format!(
                        "crate::halltim::impl_pin!({}, {}, {}, {});\n",
                        peripheral.name, role, pin.pin, af
                    ));
                }
                if uart && peripheral.name.starts_with("UART") {
                    let role = match pin.signal {
                        "TX" => Some("TxPin"),
                        "RX" => Some("RxPin"),
                        "RTS" => Some("RtsPin"),
                        "CTS" => Some("CtsPin"),
                        _ => None,
                    };
                    if let Some(role) = role {
                        out.push_str(&format!(
                            "crate::usart::impl_pin!({}, {}, {}, {});\n",
                            pin.pin, peripheral.name, role, af
                        ));
                    }
                }
                if i2c && peripheral.name.starts_with("I2C") {
                    let role = match pin.signal {
                        "SCL" => Some("SclPin"),
                        "SDA" => Some("SdaPin"),
                        _ => None,
                    };
                    if let Some(role) = role {
                        out.push_str(&format!(
                            "crate::i2c::impl_pin!({}, {}, {}, {});\n",
                            role, peripheral.name, pin.pin, af
                        ));
                    }
                }
                let buffered_input = matches!(gtim_version, Some("cw32l010_v1" | "cw32l012_v1"))
                    && (peripheral.name.starts_with("GTIM") || peripheral.name == "ATIM");
                let capture_role = match pin.signal {
                    "CAP1" if peripheral.classic_timer_input.is_some() => Some("Ch1"),
                    "CAP2" if peripheral.classic_timer_input.is_some() => Some("Ch2"),
                    "CAP3" if peripheral.classic_timer_input.is_some() => Some("Ch3"),
                    "CAP4" if peripheral.classic_timer_input.is_some() => Some("Ch4"),
                    "CH1" if buffered_input => Some("Ch1"),
                    "CH2" if buffered_input => Some("Ch2"),
                    "CH3" if buffered_input => Some("Ch3"),
                    "CH4" if buffered_input => Some("Ch4"),
                    _ => None,
                };
                if let Some(role) = capture_role {
                    out.push_str(&format!(
                        "crate::timer::impl_capture_pin!({}, {}, {}, {});\n",
                        peripheral.name, role, pin.pin, af
                    ));
                }
                if gtim_version.is_some()
                    && peripheral.name.starts_with("GTIM")
                    && Some(peripheral.name) != time_driver
                {
                    let role = match pin.signal {
                        "CH1" => Some("Ch1"),
                        "CH2" => Some("Ch2"),
                        "CH3" => Some("Ch3"),
                        "CH4" => Some("Ch4"),
                        _ => None,
                    };
                    if let Some(role) = role {
                        out.push_str(&format!(
                            "crate::timer::impl_timer_pin!({}, {}, {}, {});\n",
                            peripheral.name, role, pin.pin, af
                        ));
                    }
                }
                let classic_atim_capture_role = match pin.signal {
                    "CAP1A" => Some("Ch1A"),
                    "CAP1B" => Some("Ch1B"),
                    "CAP2A" => Some("Ch2A"),
                    "CAP2B" => Some("Ch2B"),
                    "CAP3A" => Some("Ch3A"),
                    "CAP3B" => Some("Ch3B"),
                    _ => None,
                };
                if let Some(role) = classic_atim_capture_role {
                    assert_eq!(peripheral.name, "ATIM");
                    assert!(
                        classic_atim_input,
                        "native capture pin without ATIM protocol"
                    );
                    out.push_str(&format!(
                        "crate::timer::impl_classic_atim_capture_pin!(ATIM, {}, {}, {});\n",
                        role, pin.pin, af
                    ));
                }
                if atim_version.is_some() && peripheral.name == "ATIM" {
                    let role = match pin.signal {
                        "CH1A" | "CH1" => Some("Ch1"),
                        "CH2A" | "CH2" => Some("Ch2"),
                        "CH3A" | "CH3" => Some("Ch3"),
                        "CH4" if matches!(atim_version, Some("cw32l010_v1" | "cw32l012_v1")) => {
                            Some("Ch4")
                        }
                        _ => None,
                    };
                    if let Some(role) = role {
                        out.push_str(&format!(
                            "crate::timer::impl_timer_pin!(ATIM, {}, {}, {});\n",
                            role, pin.pin, af
                        ));
                    }
                }
                if peripheral.name == "ATIM" && peripheral.atim_complementary.is_some() {
                    match pin.signal {
                        "CH1N" | "CH2N" | "CH3N" | "CH4N" | "CH1B" | "CH2B" | "CH3B" => out
                            .push_str(&format!(
                                "crate::timer::impl_complementary_pin!(Ch{}, {}, {});\n",
                                &pin.signal[2..3],
                                pin.pin,
                                af
                            )),
                        "BK" if matches!(atim_version, Some("cw32l010_v1" | "cw32l012_v1")) => out
                            .push_str(&format!(
                                "crate::timer::impl_timer_break_pin!({}, {});\n",
                                pin.pin, af
                            )),
                        _ => {}
                    }
                }
                if spi && peripheral.name.starts_with("SPI") {
                    let role = match pin.signal {
                        "SCK" => Some("SckPin"),
                        "MOSI" => Some("MosiPin"),
                        "MISO" => Some("MisoPin"),
                        _ => None,
                    };
                    if let Some(role) = role {
                        out.push_str(&format!(
                            "crate::spi::impl_pin!({}, {}, {}, {});\n",
                            role, peripheral.name, pin.pin, af
                        ));
                    }
                }
            }
        }
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("_generated.rs"),
        out,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

// F020 UM CN1.4 pp93–94/134–138: qualify its native two-channel controller
// and the typed channel view before exposing any safe DMA capability.
fn qualify_f020_dma() {
    use cw32_metapac::metadata::{METADATA, MemoryRegionKind, PeripheralRegisters, ir};

    assert_eq!(METADATA.line, "CW32F020");
    match METADATA.name {
        "CW32F020" => assert!(METADATA.memory.is_empty()),
        "CW32F020C6U7" | "CW32F020K6U7" | "CW32F020F6U7" => {
            assert_eq!(METADATA.memory.len(), 1);
            let ram: Vec<_> = METADATA.memory[0]
                .iter()
                .filter(|region| region.kind == MemoryRegionKind::Ram)
                .collect();
            assert_eq!(ram.len(), 1);
            assert_eq!((ram[0].address, ram[0].size), (0x2000_0000, 8192));
        }
        _ => panic!("unreviewed F020 DMA selection"),
    }
    let peripheral = |name: &str| {
        METADATA
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .expect("missing F020 DMA peripheral")
    };
    let controller = peripheral("DMA");
    assert_eq!(controller.address, 0x4002_0000);
    let regs = controller.registers.as_ref().unwrap();
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("dma", "cw32f020_v1", "Dma")
    );
    assert_eq!(METADATA.dma_channels.len(), 2);
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| {
                p.registers
                    .as_ref()
                    .is_some_and(|r| matches!(r.kind, "dma" | "dmachannel"))
            })
            .count(),
        3
    );
    for (index, name, physical, address, irq, number) in [
        (0, "DMA_CH1", "DMACHANNEL1", 0x4002_0020, "DMACH1", 9),
        (1, "DMA_CH2", "DMACHANNEL2", 0x4002_0040, "DMACH2", 10),
    ] {
        let channel = &METADATA.dma_channels[index];
        assert_eq!(
            (channel.name, channel.dma, channel.channel as usize),
            (name, "DMA", index)
        );
        let p = peripheral(physical);
        assert_eq!(p.address, address);
        let channel_regs = p.registers.as_ref().unwrap();
        assert_eq!(
            (channel_regs.kind, channel_regs.version, channel_regs.block),
            ("dmachannel", "v1", "Dmachannel")
        );
        // The physical channel's GLOBAL mapping is authoritative, not DMA's
        // root DMACH1 mapping; CH2 has its own IRQ10.
        assert_eq!(p.interrupts.len(), 1);
        assert_eq!(
            (p.interrupts[0].signal, p.interrupts[0].interrupt),
            ("GLOBAL", irq)
        );
        assert_eq!(
            METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == irq && i.number == number)
                .count(),
            1
        );
        assert_eq!(p.rcc_control, controller.rcc_control);
    }
    assert!(!METADATA.interrupts.iter().any(|i| {
        i.number == 11 || (i.name.starts_with("DMACH") && !matches!(i.name, "DMACH1" | "DMACH2"))
    }));

    fn fields(
        regs: &PeripheralRegisters,
        name: &str,
        offset: u32,
        access: ir::Access,
    ) -> &'static [ir::Field] {
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(item.array.is_none());
        assert_eq!(item.byte_offset, offset);
        let ir::BlockItemInner::Register(register) = &item.inner else {
            panic!("F020 DMA item must be a register")
        };
        assert_eq!(register.access, access);
        assert_eq!(register.bit_size, 32);
        let fieldset = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == register.fieldset)
            .unwrap();
        assert!(fieldset.name.eq_ignore_ascii_case(name));
        assert_eq!(fieldset.bit_size, 32);
        fieldset.fields
    }
    for (name, offset, access) in [
        ("ISR", 0, ir::Access::Read),
        ("ICR", 4, ir::Access::ReadWrite),
    ] {
        let fields = fields(regs, name, offset, access);
        assert_eq!(fields.len(), 4);
        for (name, bit) in [("TC1", 0), ("TE1", 1), ("TC2", 4), ("TE2", 5)] {
            let field = fields
                .iter()
                .find(|f| f.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert!(field.array.is_none());
            assert_eq!(field.bit_size, 1);
            assert!(matches!(&field.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
        }
    }
    for physical in ["DMACHANNEL1", "DMACHANNEL2"] {
        let regs = peripheral(physical).registers.as_ref().unwrap();
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        assert_eq!(block.items.len(), 5);
        let descriptors: &[(&str, u32, &[(&str, u32, u32, Option<&str>)])] = &[
            (
                "CSR",
                0,
                &[
                    ("EN", 0, 1, None),
                    ("TCIE", 1, 1, None),
                    ("TEIE", 2, 1, None),
                    ("TRANS", 3, 1, Some("TransferMode")),
                    ("SRCINC", 4, 1, None),
                    ("DSTINC", 5, 1, None),
                    ("SIZE", 6, 2, Some("Width")),
                    ("STATUS", 8, 3, Some("Status")),
                ],
            ),
            ("CNT", 4, &[("CNT", 0, 16, None), ("REPEAT", 16, 4, None)]),
            ("SRCADDR", 8, &[("SRCADDR", 0, 32, None)]),
            ("DSTADDR", 12, &[("DSTADDR", 0, 32, None)]),
            (
                "TRIG",
                16,
                &[
                    ("TYPE_", 0, 1, Some("Trigger")),
                    ("SOFTSRC", 1, 1, None),
                    ("HARDSRC", 2, 6, None),
                ],
            ),
        ];
        for &(name, offset, expected) in descriptors {
            let fields = fields(regs, name, offset, ir::Access::ReadWrite);
            assert_eq!(fields.len(), expected.len());
            for &(name, bit, width, enumm) in expected {
                let field = fields
                    .iter()
                    .find(|f| f.name.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(field.array.is_none());
                assert_eq!(field.bit_size, width);
                assert_eq!(field.enumm, enumm);
                assert!(matches!(&field.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
            }
        }
        // Existing common typed enums are admitted against F020's own manual,
        // including STATUS2 as an error description, never a drain proof.
        let enums: &[(&str, u32, &[(&str, u64)])] = &[
            ("Width", 2, &[("Byte", 0), ("HalfWord", 1), ("Word", 2)]),
            ("Trigger", 1, &[("Software", 0), ("Hardware", 1)]),
            ("TransferMode", 1, &[("Bulk", 0), ("Block", 1)]),
            (
                "Status",
                3,
                &[
                    ("Initial", 0),
                    ("AddressOutOfRange", 1),
                    ("Stopped", 2),
                    ("SourceError", 3),
                    ("DestinationError", 4),
                    ("Complete", 5),
                ],
            ),
        ];
        for &(name, width, variants) in enums {
            let enumm = regs.ir.enums.iter().find(|e| e.name == name).unwrap();
            assert_eq!(enumm.bit_size, width);
            assert_eq!(enumm.variants.len(), variants.len());
            for &(name, value) in variants {
                assert_eq!(
                    enumm
                        .variants
                        .iter()
                        .filter(|v| v.name == name && v.value == value)
                        .count(),
                    1
                );
            }
        }
    }
    // Reuse the shared RCC metadata/IR boundary. No DMA-local gate or reset
    // implementation is introduced, and admission never resets dirty state.
    let control = controller.rcc_control.as_ref().unwrap();
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "HCLK"));
    assert_eq!(
        (control.enable.register, control.enable.field),
        ("ahben", "dma")
    );
    assert!(control.enable_active_value && control.enable_write_key.is_none());
    let reset = control.reset.as_ref().unwrap();
    assert_eq!((reset.register, reset.field), ("ahbrst", "dma"));
    assert_eq!(control.reset_asserted_value, Some(false));
    let sysctrl = peripheral("SYSCTRL").registers.as_ref().unwrap();
    assert_eq!(sysctrl.version, "cw32f020_v1");
    let (_, enable_bit, enable_width) = rcc_field(sysctrl, &control.enable);
    let (_, reset_bit, reset_width) = rcc_field(sysctrl, reset);
    assert_eq!(
        (enable_bit, enable_width, reset_bit, reset_width),
        (0, 1, 0, 1)
    );
}

// Own-source qualification for the complete native four-channel cohort. Shared
// PAC layout is reuse, not a substitute for each family's manual qualification.
fn qualify_four_channel_dma() {
    use cw32_metapac::metadata::{METADATA, MemoryRegionKind, PeripheralRegisters, ir};

    match (METADATA.line, METADATA.name) {
        ("CW32L031", "CW32L031")
        | ("CW32L052", "CW32L052")
        | ("CW32R031", "CW32R031") => assert!(METADATA.memory.is_empty()),
        (
            "CW32L031",
            "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8P6" | "CW32L031F8U6" | "CW32L031K8U6"
            | "CW32L031K8V6",
        )
        | ("CW32L052", "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6")
        | ("CW32R031", "CW32R031C8U6") => {
            assert_eq!(METADATA.memory.len(), 1);
            assert_eq!(METADATA.memory[0].len(), 2);
            for (name, kind, address, size) in [
                ("FLASH", MemoryRegionKind::Flash, 0, 65536),
                ("RAM", MemoryRegionKind::Ram, 0x2000_0000, 8192),
            ] {
                let regions: Vec<_> = METADATA.memory[0]
                    .iter()
                    .filter(|region| region.kind == kind)
                    .collect();
                assert_eq!(regions.len(), 1);
                assert_eq!(
                    (regions[0].name, regions[0].address, regions[0].size),
                    (name, address, size)
                );
            }
        }
        _ => panic!("unreviewed four-channel DMA selection"),
    }
    let native_version = if METADATA.line == "CW32L052" {
        "cw32l052_v1"
    } else {
        "cw32l031_v1"
    };
    let peripheral = |name: &str| {
        let matches: Vec<_> = METADATA
            .peripherals
            .iter()
            .filter(|p| p.name == name)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "missing or duplicate four-channel peripheral"
        );
        matches[0]
    };
    let controller = peripheral("DMA");
    assert_eq!(controller.address, 0x4002_0000);
    let regs = controller.registers.as_ref().unwrap();
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("dma", "cw32l031_v1", "Dma")
    );
    assert_eq!(METADATA.dma_channels.len(), 4);
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| {
                p.registers
                    .as_ref()
                    .is_some_and(|r| matches!(r.kind, "dma" | "dmachannel"))
            })
            .count(),
        5
    );
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| p.name.starts_with("DMACHANNEL"))
            .count(),
        4
    );
    for (index, name, physical, address, irq, number) in [
        (0, "DMA_CH1", "DMACHANNEL1", 0x4002_0020, "DMACH1", 9),
        (1, "DMA_CH2", "DMACHANNEL2", 0x4002_0040, "DMACH23", 10),
        (2, "DMA_CH3", "DMACHANNEL3", 0x4002_0060, "DMACH23", 10),
        (3, "DMA_CH4", "DMACHANNEL4", 0x4002_0080, "DMACH4", 11),
    ] {
        let channel = &METADATA.dma_channels[index];
        assert_eq!(
            (channel.name, channel.dma, channel.channel as usize),
            (name, "DMA", index)
        );
        let p = peripheral(physical);
        assert_eq!(p.address, address);
        let channel_regs = p.registers.as_ref().unwrap();
        assert_eq!(
            (channel_regs.kind, channel_regs.version, channel_regs.block),
            ("dmachannel", "v1", "Dmachannel")
        );
        // Inspect each physical GLOBAL mapping: CH2/3 share IRQ10, whereas
        // IRQ11 is native DMACH4, never DMACH45 or F020's reserved slot.
        assert_eq!(p.interrupts.len(), 1);
        assert_eq!(
            (p.interrupts[0].signal, p.interrupts[0].interrupt),
            ("GLOBAL", irq)
        );
        assert_eq!(
            METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == irq && i.number == number)
                .count(),
            1
        );
        assert_eq!(p.rcc, controller.rcc);
        assert_eq!(p.rcc_control, controller.rcc_control);
    }
    let vectors: Vec<_> = METADATA
        .interrupts
        .iter()
        .filter(|i| i.name.starts_with("DMACH") || (9..=11).contains(&i.number))
        .map(|i| (i.name, i.number))
        .collect();
    assert_eq!(vectors.len(), 3);
    for expected in [("DMACH1", 9), ("DMACH23", 10), ("DMACH4", 11)] {
        assert!(vectors.contains(&expected));
    }

    fn fields(
        regs: &PeripheralRegisters,
        name: &str,
        offset: u32,
        access: ir::Access,
    ) -> &'static [ir::Field] {
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        assert!(block.extends.is_none());
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(item.array.is_none());
        assert_eq!(item.byte_offset, offset);
        let ir::BlockItemInner::Register(register) = &item.inner else {
            panic!("four-channel DMA item must be a register")
        };
        assert_eq!(register.access, access);
        assert_eq!(register.bit_size, 32);
        let fieldset = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == register.fieldset)
            .unwrap();
        assert!(fieldset.name.eq_ignore_ascii_case(name));
        assert!(fieldset.extends.is_none());
        assert_eq!(fieldset.bit_size, 32);
        fieldset.fields
    }
    fn field(fields: &[ir::Field], name: &str, bit: u32, width: u32, enumm: Option<&str>) {
        let matches: Vec<_> = fields
            .iter()
            .filter(|f| f.name.eq_ignore_ascii_case(name))
            .collect();
        assert_eq!(matches.len(), 1);
        let f = matches[0];
        assert!(f.array.is_none());
        assert_eq!(f.bit_size, width);
        assert_eq!(f.enumm, enumm);
        assert!(matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
    }
    let controller_block = regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap();
    assert_eq!(controller_block.items.len(), 22);
    for (name, offset, access) in [
        ("ISR", 0, ir::Access::Read),
        ("ICR", 4, ir::Access::ReadWrite),
    ] {
        let flags = fields(regs, name, offset, access);
        assert_eq!(flags.len(), 8);
        for (name, bit) in [
            ("TC1", 0),
            ("TE1", 1),
            ("TC2", 4),
            ("TE2", 5),
            ("TC3", 8),
            ("TE3", 9),
            ("TC4", 12),
            ("TE4", 13),
        ] {
            field(flags, name, bit, 1, None);
        }
    }
    let descriptors: &[(&str, u32, &[(&str, u32, u32, Option<&str>)])] = &[
        (
            "CSR",
            0,
            &[
                ("EN", 0, 1, None),
                ("TCIE", 1, 1, None),
                ("TEIE", 2, 1, None),
                ("TRANS", 3, 1, Some("TransferMode")),
                ("SRCINC", 4, 1, None),
                ("DSTINC", 5, 1, None),
                ("SIZE", 6, 2, Some("Width")),
                ("STATUS", 8, 3, Some("Status")),
            ],
        ),
        ("CNT", 4, &[("CNT", 0, 16, None), ("REPEAT", 16, 4, None)]),
        ("SRCADDR", 8, &[("SRCADDR", 0, 32, None)]),
        ("DSTADDR", 12, &[("DSTADDR", 0, 32, None)]),
        (
            "TRIG",
            16,
            &[
                ("TYPE_", 0, 1, Some("Trigger")),
                ("SOFTSRC", 1, 1, None),
                ("HARDSRC", 2, 6, None),
            ],
        ),
    ];
    for number in 1..=4 {
        let channel_regs = peripheral(&format!("DMACHANNEL{number}"))
            .registers
            .as_ref()
            .unwrap();
        let block = channel_regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == channel_regs.block)
            .unwrap();
        assert_eq!(block.items.len(), 5);
        for &(name, offset, expected) in descriptors {
            let typed = fields(channel_regs, name, offset, ir::Access::ReadWrite);
            let native = fields(
                regs,
                &format!("{name}{number}"),
                32 * number + offset,
                ir::Access::ReadWrite,
            );
            assert_eq!(typed.len(), expected.len());
            assert_eq!(native.len(), expected.len());
            for &(name, bit, width, enumm) in expected {
                field(typed, name, bit, width, enumm);
                // The flat native controller descriptors do not carry enums;
                // typed channel enums are separately qualified below.
                field(native, name, bit, width, None);
            }
        }
        // Each own manual agrees on these existing typed domains. STATUS2
        // describes a stop-request error, not an acknowledgment of bus drain.
        let enums: &[(&str, u32, &[(&str, u64)])] = &[
            ("Width", 2, &[("Byte", 0), ("HalfWord", 1), ("Word", 2)]),
            ("Trigger", 1, &[("Software", 0), ("Hardware", 1)]),
            ("TransferMode", 1, &[("Bulk", 0), ("Block", 1)]),
            (
                "Status",
                3,
                &[
                    ("Initial", 0),
                    ("AddressOutOfRange", 1),
                    ("Stopped", 2),
                    ("SourceError", 3),
                    ("DestinationError", 4),
                    ("Complete", 5),
                ],
            ),
        ];
        for &(name, width, variants) in enums {
            let enumm = channel_regs
                .ir
                .enums
                .iter()
                .find(|e| e.name == name)
                .unwrap();
            assert_eq!(enumm.bit_size, width);
            assert_eq!(enumm.variants.len(), variants.len());
            for &(name, value) in variants {
                assert_eq!(
                    enumm
                        .variants
                        .iter()
                        .filter(|v| v.name == name && v.value == value)
                        .count(),
                    1
                );
            }
        }
    }
    // One shared clock/reset boundary for all four channels; runtime admission
    // retains the established clean-entry premise and never repairs dirty state.
    let control = controller.rcc_control.as_ref().unwrap();
    assert!(controller.rcc.is_some());
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "HCLK"));
    assert_eq!(
        (control.enable.register, control.enable.field),
        ("ahben", "dma")
    );
    assert!(control.enable_active_value && control.enable_write_key.is_none());
    let reset = control.reset.as_ref().unwrap();
    assert_eq!((reset.register, reset.field), ("ahbrst", "dma"));
    assert_eq!(control.reset_asserted_value, Some(false));
    let sysctrl = peripheral("SYSCTRL").registers.as_ref().unwrap();
    assert_eq!(
        (sysctrl.kind, sysctrl.version, sysctrl.block),
        ("sysctrl", native_version, "Sysctrl")
    );
    assert_eq!(rcc_field(sysctrl, &control.enable), (0x30 / 4, 0, 1));
    assert_eq!(rcc_field(sysctrl, reset), (0x40 / 4, 0, 1));

    // Each own UARTx chapter and byte-SRAM/TDR example qualifies UART1..3 TX.
    // Separate own RX qualification covers RDR; native UART versions stay distinct.
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "uart"))
            .count(),
        3
    );
    for (name, address, irq, selector) in [
        ("UART1", 0x4001_3800, 27, 1),
        ("UART2", 0x4000_4400, 28, 3),
        ("UART3", 0x4000_4800, 29, 5),
    ] {
        let uart = peripheral(name);
        assert_eq!(uart.address, address);
        let regs = uart.registers.as_ref().unwrap();
        assert_eq!(
            (regs.kind, regs.version, regs.block),
            ("uart", native_version, "Uart")
        );
        assert_eq!(uart.interrupts.len(), 1);
        assert_eq!(
            (uart.interrupts[0].signal, uart.interrupts[0].interrupt),
            ("GLOBAL", name)
        );
        assert_eq!(
            METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == name && i.number == irq)
                .count(),
            1
        );
        let tx: Vec<_> = uart
            .dma_channels
            .iter()
            .filter(|r| r.signal == "TX")
            .collect();
        assert_eq!(tx.len(), 1);
        assert_eq!(tx[0].request, Some(selector));
        let tdr = fields(regs, "TDR", 0x28, ir::Access::Write);
        assert_eq!(tdr.len(), 1);
        field(tdr, "TDR", 0, 9, None);
        field(
            fields(regs, "CR2", 4, ir::Access::ReadWrite),
            "DMATX",
            7,
            1,
            None,
        );
    }
}

// L012 keeps its native eight-register channel view, paired DMA vectors and
// keyed clock enable. Qualify its own source-derived metadata independently.
fn qualify_l012_dma() {
    use cw32_metapac::metadata::{METADATA, MemoryRegionKind, PeripheralRegisters, ir};

    match (METADATA.line, METADATA.name) {
        ("CW32L012", "CW32L012") => assert!(METADATA.memory.is_empty()),
        ("CW32L012", "CW32L012C8T6" | "CW32L012C8U6") => {
            assert_eq!(METADATA.memory.len(), 1);
            assert_eq!(METADATA.memory[0].len(), 2);
            for (name, kind, address, size) in [
                ("FLASH", MemoryRegionKind::Flash, 0, 65536),
                ("RAM", MemoryRegionKind::Ram, 0x2000_0000, 8192),
            ] {
                let regions: Vec<_> = METADATA.memory[0]
                    .iter()
                    .filter(|region| region.kind == kind)
                    .collect();
                assert_eq!(regions.len(), 1);
                assert_eq!(
                    (regions[0].name, regions[0].address, regions[0].size),
                    (name, address, size)
                );
            }
        }
        _ => panic!("unreviewed L012 DMA selection"),
    }
    let peripheral = |name: &str| {
        let matches: Vec<_> = METADATA
            .peripherals
            .iter()
            .filter(|p| p.name == name)
            .collect();
        assert_eq!(matches.len(), 1, "missing or duplicate L012 DMA peripheral");
        matches[0]
    };
    let controller = peripheral("DMA");
    assert_eq!(controller.address, 0x4002_0000);
    let regs = controller.registers.as_ref().unwrap();
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("dma", "cw32l012_v1", "Dma")
    );
    assert_eq!(controller.interrupts.len(), 1);
    assert_eq!(
        (
            controller.interrupts[0].signal,
            controller.interrupts[0].interrupt
        ),
        ("GLOBAL", "DMACH12")
    );
    assert_eq!(METADATA.dma_channels.len(), 4);
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| {
                p.registers
                    .as_ref()
                    .is_some_and(|r| matches!(r.kind, "dma" | "dmachannel"))
            })
            .count(),
        5
    );
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| p.name.starts_with("DMACHANNEL"))
            .count(),
        4
    );
    for (index, name, physical, address, irq, number) in [
        (0, "DMA_CH1", "DMACHANNEL1", 0x4002_0020, "DMACH12", 9),
        (1, "DMA_CH2", "DMACHANNEL2", 0x4002_0040, "DMACH12", 9),
        (2, "DMA_CH3", "DMACHANNEL3", 0x4002_0060, "DMACH34", 10),
        (3, "DMA_CH4", "DMACHANNEL4", 0x4002_0080, "DMACH34", 10),
    ] {
        let channel = &METADATA.dma_channels[index];
        assert_eq!(
            (channel.name, channel.dma, channel.channel as usize),
            (name, "DMA", index)
        );
        let p = peripheral(physical);
        assert_eq!(p.address, address);
        let channel_regs = p.registers.as_ref().unwrap();
        assert_eq!(
            (channel_regs.kind, channel_regs.version, channel_regs.block),
            ("dmachannel", "cw32l012_v1", "Dmachannel")
        );
        // Inspect every physical GLOBAL mapping: CH1/2 share IRQ9 and CH3/4
        // share IRQ10. The root's first IRQ does not qualify all four channels.
        assert_eq!(p.interrupts.len(), 1);
        assert_eq!(
            (p.interrupts[0].signal, p.interrupts[0].interrupt),
            ("GLOBAL", irq)
        );
        assert_eq!(
            METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == irq && i.number == number)
                .count(),
            1
        );
        assert_eq!(p.rcc, controller.rcc);
        assert_eq!(p.rcc_control, controller.rcc_control);
    }
    let vectors: Vec<_> = METADATA
        .interrupts
        .iter()
        .filter(|i| i.name.starts_with("DMACH") || (9..=11).contains(&i.number))
        .map(|i| (i.name, i.number))
        .collect();
    assert_eq!(vectors.len(), 3);
    for expected in [("DMACH12", 9), ("DMACH34", 10), ("CORDIC", 11)] {
        assert!(vectors.contains(&expected));
    }

    fn fields(
        regs: &PeripheralRegisters,
        name: &str,
        offset: u32,
        access: ir::Access,
    ) -> &'static [ir::Field] {
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        assert!(block.extends.is_none());
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(item.array.is_none());
        assert_eq!(item.byte_offset, offset);
        let ir::BlockItemInner::Register(register) = &item.inner else {
            panic!("L012 DMA item must be a register")
        };
        assert_eq!(register.access, access);
        assert_eq!(register.bit_size, 32);
        let fieldset = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == register.fieldset)
            .unwrap();
        assert!(fieldset.name.eq_ignore_ascii_case(name));
        assert!(fieldset.extends.is_none());
        assert_eq!(fieldset.bit_size, 32);
        fieldset.fields
    }
    fn field(fields: &[ir::Field], name: &str, bit: u32, width: u32, enumm: Option<&str>) {
        let matches: Vec<_> = fields
            .iter()
            .filter(|f| f.name.eq_ignore_ascii_case(name))
            .collect();
        assert_eq!(matches.len(), 1);
        let f = matches[0];
        assert!(f.array.is_none());
        assert_eq!(f.bit_size, width);
        assert_eq!(f.enumm, enumm);
        assert!(matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
    }
    let controller_block = regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap();
    assert_eq!(regs.ir.blocks.len(), 1);
    assert_eq!(regs.ir.fieldsets.len(), 34);
    assert!(regs.ir.enums.is_empty());
    assert_eq!(controller_block.items.len(), 34);
    for (name, offset, access) in [
        ("ISR", 0, ir::Access::Read),
        ("ICR", 4, ir::Access::ReadWrite),
    ] {
        let flags = fields(regs, name, offset, access);
        assert_eq!(flags.len(), 8);
        for (name, bit) in [
            ("TC1", 0),
            ("TE1", 1),
            ("TC2", 4),
            ("TE2", 5),
            ("TC3", 8),
            ("TE3", 9),
            ("TC4", 12),
            ("TE4", 13),
        ] {
            field(flags, name, bit, 1, None);
        }
    }
    // Native current-value registers are separate from the configured
    // descriptor. Per-field RO and ICR command semantics are checked by the
    // existing sidecar/projection verifier; selected IR cannot represent them.
    let descriptors: &[(&str, u32, ir::Access, &[(&str, u32, u32, Option<&str>)])] = &[
        (
            "CSR",
            0,
            ir::Access::ReadWrite,
            &[
                ("EN", 0, 1, None),
                ("TCIE", 1, 1, None),
                ("TEIE", 2, 1, None),
                ("TRANS", 3, 1, Some("TransferMode")),
                ("SRCINC", 4, 1, None),
                ("DSTINC", 5, 1, None),
                ("SIZE", 6, 2, Some("Width")),
                ("STATUS", 8, 3, Some("Status")),
                ("RESTART", 11, 1, None),
                ("TC", 12, 1, None),
                ("TE", 13, 1, None),
                ("SRCLOAD", 14, 1, None),
                ("DSTLOAD", 15, 1, None),
            ],
        ),
        (
            "CNT",
            4,
            ir::Access::ReadWrite,
            &[("CNT", 0, 16, None), ("REPEAT", 16, 4, None)],
        ),
        (
            "SRCADDR",
            8,
            ir::Access::ReadWrite,
            &[("SRCADDR", 0, 32, None)],
        ),
        (
            "DSTADDR",
            12,
            ir::Access::ReadWrite,
            &[("DSTADDR", 0, 32, None)],
        ),
        (
            "TRIG",
            16,
            ir::Access::ReadWrite,
            &[
                ("TYPE_", 0, 1, Some("Trigger")),
                ("SOFTSRC", 1, 1, None),
                ("HARDSRC", 2, 6, None),
            ],
        ),
        (
            "CCNT",
            20,
            ir::Access::Read,
            &[("CCNT", 0, 16, None), ("CREPEAT", 16, 4, None)],
        ),
        (
            "CSRCADDR",
            24,
            ir::Access::Read,
            &[("CSRCADDR", 0, 32, None)],
        ),
        (
            "CDSTADDR",
            28,
            ir::Access::Read,
            &[("CDSTADDR", 0, 32, None)],
        ),
    ];
    for number in 1..=4 {
        let channel_regs = peripheral(&format!("DMACHANNEL{number}"))
            .registers
            .as_ref()
            .unwrap();
        let block = channel_regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == channel_regs.block)
            .unwrap();
        assert_eq!(channel_regs.ir.blocks.len(), 1);
        assert_eq!(channel_regs.ir.fieldsets.len(), 8);
        assert_eq!(block.items.len(), 8);
        for &(name, offset, ref access, expected) in descriptors {
            let typed = fields(channel_regs, name, offset, access.clone());
            let native = fields(
                regs,
                &format!("{name}{number}"),
                32 * number + offset,
                access.clone(),
            );
            assert_eq!(typed.len(), expected.len());
            assert_eq!(native.len(), expected.len());
            for &(name, bit, width, enumm) in expected {
                field(typed, name, bit, width, enumm);
                // The flat native controller descriptors do not carry enums;
                // typed channel enums are separately qualified below.
                field(native, name, bit, width, None);
            }
        }
        // L012 UM CN1.4 section 8.8.3/4 supplies these native typed domains.
        // STATUS2 is a stop-request error, not an acknowledgment of bus drain.
        let enums: &[(&str, u32, &[(&str, u64)])] = &[
            ("Width", 2, &[("Byte", 0), ("HalfWord", 1), ("Word", 2)]),
            ("Trigger", 1, &[("Software", 0), ("Hardware", 1)]),
            ("TransferMode", 1, &[("Bulk", 0), ("Block", 1)]),
            (
                "Status",
                3,
                &[
                    ("Initial", 0),
                    ("AddressOutOfRange", 1),
                    ("Stopped", 2),
                    ("SourceError", 3),
                    ("DestinationError", 4),
                    ("Complete", 5),
                ],
            ),
        ];
        assert_eq!(channel_regs.ir.enums.len(), enums.len());
        for &(name, width, variants) in enums {
            let enumm = channel_regs
                .ir
                .enums
                .iter()
                .find(|e| e.name == name)
                .unwrap();
            assert_eq!(enumm.bit_size, width);
            assert_eq!(enumm.variants.len(), variants.len());
            for &(name, value) in variants {
                assert_eq!(
                    enumm
                        .variants
                        .iter()
                        .filter(|v| v.name == name && v.value == value)
                        .count(),
                    1
                );
            }
        }
    }
    // One shared clock/reset boundary for all four channels; runtime admission
    // retains the established clean-entry premise and never repairs dirty state.
    let control = controller.rcc_control.as_ref().unwrap();
    let rcc = controller.rcc.as_ref().unwrap();
    assert_eq!(rcc.bus_clock, "HCLK");
    assert_eq!(
        rcc.kernel_clock,
        cw32_metapac::metadata::PeripheralRccKernelClock::Clock("HCLK")
    );
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "HCLK"));
    assert_eq!(
        (control.enable.register, control.enable.field),
        ("ahben", "dma")
    );
    assert!(control.enable_active_value);
    let key = control.enable_write_key.as_ref().unwrap();
    assert_eq!(
        (key.field.register, key.field.field, key.value),
        ("ahben", "key", 0x5a5a)
    );
    let reset = control.reset.as_ref().unwrap();
    assert_eq!((reset.register, reset.field), ("ahbrst", "dma"));
    assert_eq!(control.reset_asserted_value, Some(false));
    let sysctrl = peripheral("SYSCTRL").registers.as_ref().unwrap();
    assert_eq!(
        (sysctrl.kind, sysctrl.version, sysctrl.block),
        ("sysctrl", "cw32l012_v1", "Sysctrl")
    );
    assert_eq!(rcc_field(sysctrl, &control.enable), (0x30 / 4, 0, 1));
    assert_eq!(rcc_field(sysctrl, reset), (0x40 / 4, 0, 1));
    assert_eq!(rcc_field(sysctrl, &key.field), (0x30 / 4, 16, 16));
    assert!(
        fields(sysctrl, "AHBRST", 0x40, ir::Access::ReadWrite)
            .iter()
            .all(|f| !f.name.eq_ignore_ascii_case("KEY"))
    );
    let shared = Some("DMA-DMACHANNEL1-DMACHANNEL2-DMACHANNEL3-DMACHANNEL4");
    assert_eq!(control.shared_enable_group, shared);
    assert_eq!(control.shared_reset_group, shared);

    // L012 UM CN1.4 section 21.7.1.5 demonstrates SRAM-byte-to-TDR DMA.
    // SIZE describes memory width, not arbitrary byte-wide peripheral MMIO.
    // The common UARTx layout and request table qualify UART1..3 TX only.
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "uart"))
            .count(),
        3
    );
    for (name, address, irq, selector, rcc_name, rcc_bit) in [
        ("UART1", 0x4000_0c00, 27, 1, "uart1", 3),
        ("UART2", 0x4000_1000, 28, 3, "uart2", 4),
        ("UART3", 0x4000_2000, 29, 5, "uart3", 8),
    ] {
        let uart = peripheral(name);
        assert_eq!(uart.address, address);
        let regs = uart.registers.as_ref().unwrap();
        assert_eq!(
            (regs.kind, regs.version, regs.block),
            ("uart", "cw32l012_v1", "Uart")
        );
        assert_eq!(uart.interrupts.len(), 1);
        assert_eq!(
            (uart.interrupts[0].signal, uart.interrupts[0].interrupt),
            ("GLOBAL", name)
        );
        assert_eq!(
            METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == name && i.number == irq)
                .count(),
            1
        );
        let tx: Vec<_> = uart
            .dma_channels
            .iter()
            .filter(|r| r.signal == "TX")
            .collect();
        assert_eq!(tx.len(), 1);
        assert_eq!(tx[0].request, Some(selector));
        assert_eq!(tx[0].dma, Some("DMA"));
        assert!(selector < 64);
        assert!(tx[0].channel.is_none() && tx[0].dmamux.is_none() && tx[0].remap.is_empty());
        let rx = uart.dma_channels.iter().find(|r| r.signal == "RX").unwrap();
        assert_eq!((rx.request, rx.dma), (Some(selector - 1), Some("DMA")));
        field(fields(regs, "RDR", 0x24, ir::Access::Read), "RDR", 0, 9, None);
        field(fields(regs, "CR2", 4, ir::Access::ReadWrite), "DMARX", 6, 1, None);
        let tdr = fields(regs, "TDR", 0x28, ir::Access::Write);
        assert_eq!(tdr.len(), 3);
        field(tdr, "TDR", 0, 9, None);
        field(tdr, "IDLE", 9, 1, None);
        field(tdr, "BREAK_", 10, 1, None);
        field(
            fields(regs, "CR2", 4, ir::Access::ReadWrite),
            "DMATX",
            7,
            1,
            None,
        );
        let control = uart.rcc_control.as_ref().unwrap();
        assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "PCLK"));
        assert_eq!(
            (control.enable.register, control.enable.field),
            ("apben1", rcc_name)
        );
        assert!(control.enable_active_value);
        let key = control.enable_write_key.as_ref().unwrap();
        assert_eq!(
            (key.field.register, key.field.field, key.value),
            ("apben1", "key", 0x5a5a)
        );
        let reset = control.reset.as_ref().unwrap();
        assert_eq!((reset.register, reset.field), ("apbrst1", rcc_name));
        assert_eq!(control.reset_asserted_value, Some(false));
        assert_eq!(rcc_field(sysctrl, &control.enable), (0x38 / 4, rcc_bit, 1));
        assert_eq!(rcc_field(sysctrl, &key.field), (0x38 / 4, 16, 16));
        assert_eq!(rcc_field(sysctrl, reset), (0x48 / 4, rcc_bit, 1));
    }
    assert!(
        fields(sysctrl, "APBRST1", 0x48, ir::Access::ReadWrite)
            .iter()
            .all(|f| !f.name.eq_ignore_ascii_case("KEY"))
    );
}

// Restrict debug/reset/input-only pads according to each reviewed family.
// L012 PF3/BOOT is bidirectional and must not inherit the x030 exclusion.
fn safe_pin(name: &str) -> bool {
    match cw32_metapac::metadata::METADATA.line {
        "CW32F002" | "CW32F003" => !matches!(name, "PA2" | "PA5" | "PC5"),
        "CW32L010" => !matches!(name, "PA7" | "PA8" | "PB7"),
        "CW32L011" | "CW32L012" => !matches!(name, "PA13" | "PA14"),
        _ => !matches!(name, "PA13" | "PA14" | "PF3"),
    }
}

// Direct selected-family PAC views, following embassy-stm32's gpio_block().
// Unlike its uniform stride, CW32L012 compresses the F-bank address. Use each
// metadata address, and prove the accessed subset against the actual bank IR.
fn generate_gpio(out: &mut String) {
    use cw32_metapac::metadata::{PeripheralRegisters, ir};
    let metadata = &cw32_metapac::metadata::METADATA;
    fn register(
        regs: &PeripheralRegisters,
        name: &str,
    ) -> (&'static ir::BlockItem, &'static ir::Register) {
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing GPIO register {}.{name}", regs.block));
        let ir::BlockItemInner::Register(reg) = &item.inner else {
            panic!("expected register")
        };
        (item, reg)
    }
    fn fields(regs: &PeripheralRegisters, name: &str) -> &'static [ir::Field] {
        let (_, reg) = register(regs, name);
        regs.ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == reg.fieldset)
            .unwrap()
            .fields
    }
    fn offset(field: &ir::Field) -> u32 {
        let ir::BitOffset::Regular(ref position) = field.bit_offset else {
            panic!("nonlinear GPIO field")
        };
        assert!(field.array.is_none());
        position.offset
    }
    let gpios: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.name.starts_with("GPIO"))
        .collect();
    let common = gpios
        .iter()
        .find(|p| p.name == "GPIOA")
        .unwrap()
        .registers
        .as_ref()
        .unwrap();
    let sysctrl = metadata
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap()
        .registers
        .as_ref()
        .unwrap();
    let af_width = fields(common, "AFRL")[0].bit_size;
    assert!(matches!(af_width, 3 | 4));
    let af_mask = (1u32 << af_width) - 1;
    // Field width and documented selector domain are distinct on F020/L03x/L05x/L08x.
    let af_max = if matches!(metadata.line, "CW32F030" | "CW32A030" | "CW32L012") {
        15
    } else {
        7
    };
    assert!(af_max <= af_mask);

    // Unsafe AnyPin::steal validates family output-capable pads, not bonding on
    // the selected package. Safe singleton projection below remains unchanged.
    let mut pin_masks = [0u16; 6];
    let mut pull_down_masks = [0u16; 6];
    for p in &gpios {
        let port = usize::from(p.name.as_bytes()[4] - b'A');
        let facts = p.gpio.as_ref().expect("missing GPIO pad capabilities");
        assert!(port < 6);
        pin_masks[port] = facts.output_mask;
        pull_down_masks[port] = facts.pull_down_mask;
    }
    let mut clock_masks = [0u32; 6];
    out.push_str("pub(crate) fn gpio_block(port: u8) -> crate::pac::gpio::Gpio {\nlet address = match port {\n");
    for p in gpios {
        let port = usize::from(p.name.as_bytes()[4] - b'A');
        let actual = p.registers.as_ref().unwrap();
        assert_eq!(actual.version, common.version);
        let gate = p.rcc.as_ref().unwrap().enable.as_ref().unwrap();
        assert!(gate.register.eq_ignore_ascii_case("AHBEN"));
        let field = fields(sysctrl, gate.register)
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(gate.field))
            .unwrap();
        assert_eq!(field.bit_size, 1);
        clock_masks[port] = 1 << offset(field);
        // RMW/read/command widths and permissions must match the canonical
        // family view. Fields not used by HAL may legitimately differ by bank.
        let mut names = vec![
            "DIR",
            "OPENDRAIN",
            "PUR",
            "ANALOG",
            "RISEIE",
            "FALLIE",
            "IDR",
            "ODR",
            "BRR",
            "BSRR",
            "TOG",
        ];
        if pull_down_masks[port] != 0 {
            names.push("PDR");
        }
        if p.gpio_interrupt.as_ref().unwrap().level_trigger {
            names.extend(["HIGHIE", "LOWIE"]);
        }
        if common.version == "v1" {
            names.extend(["SPEED", "LOCK"]);
        }
        if common.version == "cw32l052_v1" {
            names.push("LOCK");
        }
        if common.version == "cw32l083_v1" {
            names.push("LCKR");
        }
        if pin_masks[port] & 0xff != 0 {
            names.push("AFRL");
        }
        if pin_masks[port] & 0xff00 != 0 {
            names.push("AFRH");
        }
        for name in names {
            let (a, ar) = register(actual, name);
            let (c, cr) = register(common, name);
            assert_eq!(
                (a.byte_offset, &a.array, ar.bit_size, &ar.access),
                (c.byte_offset, &c.array, cr.bit_size, &cr.access),
                "{} {name} differs from family GPIO view",
                p.name
            );
            let pins = if name == "PDR" {
                pull_down_masks[port]
            } else {
                pin_masks[port]
            };
            for pin in 0..16 {
                if pins & (1 << pin) == 0
                    || (name == "AFRL" && pin >= 8)
                    || (name == "AFRH" && pin < 8)
                {
                    continue;
                }
                let (position, width) = if name.starts_with("AFR") {
                    ((pin % 8) * 4, af_width)
                } else {
                    (pin, 1)
                };
                for regs in [actual, common] {
                    assert!(
                        fields(regs, name)
                            .iter()
                            .any(|f| offset(f) == position && f.bit_size == width),
                        "{} {name}: absent or different field for pin {pin}",
                        p.name
                    );
                }
            }
            if matches!(name, "LOCK" | "LCKR") {
                for regs in [actual, common] {
                    assert!(
                        fields(regs, name)
                            .iter()
                            .any(|f| f.name.eq_ignore_ascii_case("KEY")
                                && offset(f) == 16
                                && f.bit_size == 16)
                    );
                }
            }
        }
        // EXTI extends the common-view proof beyond synchronous GPIO. All raw
        // accesses require exact widths, offsets and permissions, even where
        // vendor fieldsets are sparse. ICR commands use own-manual metadata:
        // sparse PAC fields cannot narrow documented write-one no-op bits.
        let irq = p
            .gpio_interrupt
            .as_ref()
            .expect("missing GPIO interrupt facts");
        let idr_access = register(common, "IDR").1.access.clone();
        assert!(matches!(
            idr_access,
            ir::Access::Read | ir::Access::ReadWrite
        ));
        let mut irq_registers = vec![
            ("RISEIE", 0x24, ir::Access::ReadWrite),
            ("FALLIE", 0x28, ir::Access::ReadWrite),
            ("ISR", 0x34, ir::Access::Read),
            ("ICR", 0x38, ir::Access::ReadWrite),
            ("IDR", 0x50, idr_access),
        ];
        if irq.level_trigger {
            irq_registers.extend([
                ("HIGHIE", 0x2c, ir::Access::ReadWrite),
                ("LOWIE", 0x30, ir::Access::ReadWrite),
            ]);
        } else {
            for regs in [actual, common] {
                let block = regs
                    .ir
                    .blocks
                    .iter()
                    .find(|b| b.name == regs.block)
                    .unwrap();
                assert!(
                    !block
                        .items
                        .iter()
                        .any(|item| [0x2c, 0x30].contains(&item.byte_offset)),
                    "edge-only GPIO has unexpected native-level registers"
                );
            }
        }
        for (name, byte_offset, access) in irq_registers {
            for regs in [actual, common] {
                let (item, reg) = register(regs, name);
                assert_eq!(
                    (
                        item.byte_offset,
                        item.array.as_ref(),
                        reg.bit_size,
                        &reg.access
                    ),
                    (byte_offset, None, 32, &access),
                    "{} {name}: unproved EXTI register layout",
                    p.name
                );
                for pin in 0..16 {
                    if irq.serviced_mask & (1 << pin) != 0 {
                        assert!(
                            fields(regs, name)
                                .iter()
                                .any(|f| offset(f) == pin && f.bit_size == 1),
                            "{} {name}: unproved EXTI serviced bit {pin}",
                            p.name
                        );
                    }
                }
                if name == "ICR" {
                    for field in fields(regs, name) {
                        assert_eq!(field.bit_size, 1);
                        assert!(
                            offset(field) < 16 && irq.clear_noop_mask & (1 << offset(field)) != 0,
                            "GPIO ICR field is outside its documented command domain"
                        );
                    }
                }
            }
        }
        out.push_str(&format!("{port} => {:#x}usize,\n", p.address));
    }
    let keyed = matches!(metadata.line, "CW32L010" | "CW32L011" | "CW32L012");
    assert_eq!(
        fields(sysctrl, "AHBEN")
            .iter()
            .any(|f| f.name.eq_ignore_ascii_case("KEY") && offset(f) == 16 && f.bit_size == 16),
        keyed
    );
    for port in 0..6 {
        assert!(pin_masks[port] == 0 || clock_masks[port] != 0);
    }
    out.push_str("_ => panic!(\"invalid GPIO port\"),\n};\nunsafe { crate::pac::gpio::Gpio::from_ptr(address as *mut ()) }\n}\n");
    out.push_str(&format!(
        "pub(crate) const GPIO_PIN_MASKS: [u16; 6] = {pin_masks:?};\n"
    ));
    out.push_str("pub(crate) fn gpio_rcc(port: u8) -> crate::rcc::RccInfo { match port {\n");
    for peripheral in metadata
        .peripherals
        .iter()
        .filter(|p| p.name.starts_with("GPIO"))
    {
        assert!(
            peripheral.rcc_control.is_some(),
            "missing GPIO clock control"
        );
        let port = peripheral.name.as_bytes()[4] - b'A';
        out.push_str(&format!(
            "{port} => <peripherals::{} as crate::rcc::SealedRccPeripheral>::RCC_INFO,\n",
            peripheral.name
        ));
    }
    out.push_str("_ => panic!(\"invalid GPIO port\"), } }\n");
    if !matches!(metadata.line, "CW32L010" | "CW32L011") {
        out.push_str(&format!(
            "pub(crate) const GPIO_PULL_DOWN_MASKS: [u16; 6] = {pull_down_masks:?};\n"
        ));
    }
    out.push_str(&format!("pub(crate) const GPIO_AF_MASK: u32 = {af_mask};\npub(crate) const GPIO_AF_MAX: u32 = {af_max};\n"));
}

// Admission and fault containment for ADC2 rely on these exact own-L012
// source facts. DAC has a shared vector but a separate configuration domain;
// its partially qualified analog kernel clock is not needed or inferred here.
fn assert_l012_adc_shared_irq() {
    use cw32_metapac::metadata::{ir, Peripheral, PeripheralRegisters, METADATA};
    use std::collections::BTreeMap;

    assert_eq!(METADATA.line, "CW32L012");
    let peripheral = |name| -> &Peripheral {
        let matches: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == name).collect();
        assert_eq!(matches.len(), 1, "L012 ADC shared-vector owner missing or duplicated");
        matches[0]
    };
    let adc1 = peripheral("ADC1");
    let adc2 = peripheral("ADC2");
    let dac = peripheral("DAC");
    let sysctrl = peripheral("SYSCTRL");
    for (p, address, kind, block) in [
        (adc1, 0x4000_0000, "adc", "Adc"),
        (adc2, 0x4000_0100, "adc", "Adc"),
        (dac, 0x4000_00c0, "dac", "Dac"),
        (sysctrl, 0x4000_4000, "sysctrl", "Sysctrl"),
    ] {
        assert_eq!(p.address, address);
        let regs = p.registers.as_ref().unwrap();
        assert_eq!((regs.kind, regs.version, regs.block), (kind, "cw32l012_v1", block));
    }
    for (p, name, number) in [(adc1, "ADC1", 12), (adc2, "ADC2_DAC", 30), (dac, "ADC2_DAC", 30)] {
        assert_eq!(p.interrupts.len(), 1);
        assert_eq!((p.interrupts[0].signal, p.interrupts[0].interrupt), ("GLOBAL", name));
        let entries: Vec<_> = METADATA.interrupts.iter()
            .filter(|i| i.name == name || i.number == number).collect();
        assert_eq!(entries.len(), 1, "L012 ADC vector duplicated or aliased");
        assert_eq!((entries[0].name, entries[0].number), (name, number));
    }
    // Resolve every link by number so an alternate name or duplicate mapping
    // cannot silently add another owner of IRQ30.
    let mut owners = Vec::new();
    for p in METADATA.peripherals {
        for link in p.interrupts {
            let entries: Vec<_> = METADATA.interrupts.iter().filter(|i| i.name == link.interrupt).collect();
            assert_eq!(entries.len(), 1, "unresolved or duplicated interrupt name");
            if entries[0].number == 30 {
                owners.push((p.name, link.signal, link.interrupt));
            }
        }
    }
    owners.sort_unstable();
    assert_eq!(owners, [("ADC2", "GLOBAL", "ADC2_DAC"), ("DAC", "GLOBAL", "ADC2_DAC")]);

    let register = |regs: &PeripheralRegisters, name: &str, offset, access: ir::Access| {
        let blocks: Vec<_> = regs.ir.blocks.iter().filter(|b| b.name == regs.block).collect();
        assert_eq!(blocks.len(), 1);
        let items: Vec<_> = blocks[0].items.iter().filter(|r| r.name.eq_ignore_ascii_case(name)).collect();
        assert_eq!(items.len(), 1, "L012 ADC/DAC control missing or duplicated");
        let item = items[0];
        assert_eq!(item.byte_offset, offset);
        assert!(item.array.is_none());
        let ir::BlockItemInner::Register(reg) = &item.inner else { panic!("expected scalar register") };
        assert_eq!(reg.bit_size, 32);
        assert_eq!(reg.access, access);
        let fieldsets: Vec<_> = regs.ir.fieldsets.iter().filter(|f| Some(f.name) == reg.fieldset).collect();
        assert_eq!(fieldsets.len(), 1);
        assert_eq!(fieldsets[0].bit_size, 32);
        let mut fields = BTreeMap::new();
        for field in fieldsets[0].fields {
            assert!(field.array.is_none());
            let ir::BitOffset::Regular(bit) = &field.bit_offset else { panic!("expected scalar field") };
            assert!(fields.insert(field.name.to_ascii_uppercase(), (bit.offset, field.bit_size)).is_none());
        }
        fields
    };
    let geometry = |fields: &[(&str, u32, u32)]| -> BTreeMap<String, (u32, u32)> {
        fields.iter().map(|(name, bit, width)| (name.to_string(), (*bit, *width))).collect()
    };
    let dacregs = dac.registers.as_ref().unwrap();
    let block = dacregs.ir.blocks.iter().find(|b| b.name == dacregs.block).unwrap();
    assert_eq!(block.items.len(), 14, "unaccounted DAC register requires shared-IRQ review");
    for (name, offset) in [
        ("CR0", 0), ("SWTRGR", 4), ("DHR12R1", 8), ("DHR12L1", 12),
        ("DHR8R1", 16), ("DHR12R2", 20), ("DHR12L2", 24), ("DHR8R2", 28),
        ("DHR12RD", 32), ("DHR12LD", 36), ("DHR8RD", 40),
        ("DOR1", 44), ("DOR2", 48), ("CR1", 52),
    ] {
        register(dacregs, name, offset, ir::Access::ReadWrite);
    }
    assert_eq!(register(dacregs, "CR0", 0, ir::Access::ReadWrite), geometry(&[
        ("EN1", 0, 1), ("TEN1", 1, 1), ("TSEL1", 2, 4), ("WAVE1", 6, 2),
        ("MAMP1", 8, 4), ("DMAEN1", 12, 1), ("DMAUDRIE1", 13, 1),
        ("EN2", 16, 1), ("TEN2", 17, 1), ("TSEL2", 18, 4), ("WAVE2", 22, 2),
        ("MAMP2", 24, 4), ("DMAEN2", 28, 1), ("DMAUDRIE2", 29, 1),
    ]));
    assert_eq!(register(dacregs, "CR1", 0x34, ir::Access::ReadWrite), geometry(&[("C1OUT", 0, 1), ("C2OUT", 1, 1)]));
    for (name, offset) in [("DOR1", 0x2c), ("DOR2", 0x30)] {
        // The register is RW for DMAUDR W0C; DATA is hardware RO. ADC2
        // admission neither writes this mixed register nor needs to read it.
        assert_eq!(register(dacregs, name, offset, ir::Access::ReadWrite), geometry(&[("DATA", 0, 12), ("DMAUDR", 15, 1)]));
    }
    for adc in [adc1, adc2] {
        let regs = adc.registers.as_ref().unwrap();
        assert_eq!(register(regs, "IER", 0x70, ir::Access::ReadWrite), geometry(&[
            ("EOC", 0, 1), ("EOS", 1, 1), ("AWDL", 3, 1), ("AWDH", 4, 1),
            ("DMAEOC", 5, 1), ("DMAEOS", 6, 1),
        ]));
        for (name, offset, access) in [("ISR", 0x78, ir::Access::Read), ("ICR", 0x74, ir::Access::ReadWrite)] {
            assert_eq!(register(regs, name, offset, access), geometry(&[
                ("EOC", 0, 1), ("EOS", 1, 1), ("AWDL", 3, 1), ("AWDH", 4, 1),
            ]));
        }
    }

    let sysregs = sysctrl.registers.as_ref().unwrap();
    for (p, enable, enable_offset, reset, reset_offset, field, bit, group) in [
        (adc1, "APBEN1", 0x38u32, "APBRST1", 0x48u32, "ADC", 0u8, Some("ADC1-ADC2")),
        (adc2, "APBEN1", 0x38, "APBRST1", 0x48, "ADC", 0, Some("ADC1-ADC2")),
        (dac, "APBEN2", 0x34, "APBRST2", 0x44, "DAC", 10, None),
    ] {
        // DAC deliberately has no fully qualified legacy rcc/kernel-clock
        // record. Admission uses only its authoritative rcc_control domain.
        let control = p.rcc_control.as_ref().expect("L012 analog configuration domain missing");
        assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "PCLK"));
        assert!(control.enable_active_value);
        assert_eq!(control.reset_asserted_value, Some(false));
        assert_eq!((control.shared_enable_group, control.shared_reset_group), (group, group));
        assert!(control.enable.register.eq_ignore_ascii_case(enable));
        assert!(control.enable.field.eq_ignore_ascii_case(field));
        let reset_field = control.reset.as_ref().unwrap();
        assert!(reset_field.register.eq_ignore_ascii_case(reset));
        assert!(reset_field.field.eq_ignore_ascii_case(field));
        assert_eq!(rcc_field(sysregs, &control.enable), (u8::try_from(enable_offset / 4).unwrap(), bit, 1));
        assert_eq!(rcc_field(sysregs, reset_field), (u8::try_from(reset_offset / 4).unwrap(), bit, 1));
        let key = control.enable_write_key.as_ref().expect("L012 analog gate key missing");
        assert!(key.field.register.eq_ignore_ascii_case(enable));
        assert!(key.field.field.eq_ignore_ascii_case("KEY"));
        assert_eq!(key.value, 0x5a5a);
        assert_eq!(rcc_field(sysregs, &key.field), (u8::try_from(enable_offset / 4).unwrap(), 16, 16));
        let enable_fields = register(sysregs, enable, enable_offset, ir::Access::ReadWrite);
        assert_eq!(enable_fields.get(field), Some(&(u32::from(bit), 1)));
        assert_eq!(enable_fields.get("KEY"), Some(&(16, 16)));
        let reset_fields = register(sysregs, reset, reset_offset, ir::Access::ReadWrite);
        assert_eq!(reset_fields.get(field), Some(&(u32::from(bit), 1)));
    }
    assert_eq!(adc1.rcc_control, adc2.rcc_control);
    assert_ne!(adc1.rcc_control.as_ref().unwrap().enable, dac.rcc_control.as_ref().unwrap().enable);
    assert_ne!(adc1.rcc_control.as_ref().unwrap().reset, dac.rcc_control.as_ref().unwrap().reset);
}

// L012 is a separate command controller. Its own source-qualified vectors,
// control fields and package routes must not inherit classic I2C assumptions.
fn command_i2c_irq(peripheral: &cw32_metapac::metadata::Peripheral) -> &'static str {
    use cw32_metapac::metadata::{METADATA, MemoryRegionKind};
    assert_eq!(METADATA.family, "CW32L0");
    assert_eq!(METADATA.line, "CW32L012");
    assert!(matches!(
        METADATA.name,
        "CW32L012" | "CW32L012C8T6" | "CW32L012C8U6"
    ));
    let mut instances: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| {
            p.name.starts_with("I2C") || p.registers.as_ref().is_some_and(|r| r.kind == "i2c")
        })
        .map(|p| p.name)
        .collect();
    instances.sort_unstable();
    assert_eq!(instances, ["I2C1", "I2C2"], "unreviewed L012 I2C inventory");
    let (address, number, bit, routes) = match peripheral.name {
        "I2C1" => (0x4000_5800, 23, 6, [("PA4", "SCL", 2), ("PA5", "SDA", 2)]),
        "I2C2" => (0x4000_5c00, 24, 11, [("PB0", "SCL", 8), ("PB1", "SDA", 8)]),
        _ => panic!("unreviewed L012 I2C instance"),
    };
    assert_eq!(peripheral.address, address);
    let regs = peripheral
        .registers
        .as_ref()
        .expect("L012 I2C registers missing");
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("i2c", "cw32l012_v1", "I2c")
    );
    assert_eq!(peripheral.interrupts.len(), 1);
    let link = &peripheral.interrupts[0];
    assert_eq!((link.signal, link.interrupt), ("GLOBAL", peripheral.name));
    let vectors: Vec<_> = METADATA
        .interrupts
        .iter()
        .filter(|i| i.name == link.interrupt)
        .collect();
    assert_eq!(vectors.len(), 1, "L012 I2C GLOBAL must resolve uniquely");
    assert_eq!(vectors[0].number, number);
    assert_eq!(
        METADATA
            .interrupts
            .iter()
            .filter(|i| i.number == number)
            .count(),
        1,
        "L012 I2C vector number has an alias"
    );
    let mut irq_owners = 0;
    for owner in METADATA.peripherals {
        for source in owner.interrupts {
            let resolved: Vec<_> = METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == source.interrupt)
                .collect();
            assert_eq!(resolved.len(), 1, "unresolved or duplicated interrupt name");
            if resolved[0].number == number {
                assert_eq!(
                    (owner.name, source.signal),
                    (peripheral.name, "GLOBAL"),
                    "L012 I2C IRQ has another owner or source"
                );
                irq_owners += 1;
            }
        }
    }
    assert_eq!(irq_owners, 1);
    let controllers: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| p.name == "SYSCTRL")
        .collect();
    assert_eq!(controllers.len(), 1);
    let sysctrl = controllers[0];
    assert_eq!(sysctrl.address, 0x4000_4000);
    let sysregs = sysctrl.registers.as_ref().unwrap();
    assert_eq!(
        (sysregs.kind, sysregs.version, sysregs.block),
        ("sysctrl", "cw32l012_v1", "Sysctrl")
    );
    let control = peripheral
        .rcc_control
        .as_ref()
        .expect("L012 I2C RCC control missing");
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "PCLK"));
    assert!(control.enable_active_value);
    assert_eq!(control.reset_asserted_value, Some(false));
    assert!(control.shared_enable_group.is_none() && control.shared_reset_group.is_none());
    assert!(control.enable.register.eq_ignore_ascii_case("APBEN2"));
    assert!(control.enable.field.eq_ignore_ascii_case(peripheral.name));
    let reset = control.reset.as_ref().expect("L012 I2C reset missing");
    assert!(reset.register.eq_ignore_ascii_case("APBRST2"));
    assert!(reset.field.eq_ignore_ascii_case(peripheral.name));
    // rcc_field returns a word offset; assert the actual byte geometry here.
    assert_eq!(rcc_field(sysregs, &control.enable), (0x34 / 4, bit, 1));
    assert_eq!(rcc_field(sysregs, reset), (0x44 / 4, bit, 1));
    let key = control
        .enable_write_key
        .as_ref()
        .expect("L012 I2C gate key missing");
    assert!(key.field.register.eq_ignore_ascii_case("APBEN2"));
    assert!(key.field.field.eq_ignore_ascii_case("KEY"));
    assert_eq!(rcc_field(sysregs, &key.field), (0x34 / 4, 16, 16));
    assert_eq!(key.value, 0x5a5a);
    for (kind, target) in [("enable", &control.enable), ("reset", reset)] {
        let (offset, target_bit, _) = rcc_field(sysregs, target);
        let physical = sysctrl.address + u64::from(offset) * 4;
        let mut owners = 0;
        // Compare physical overlapping fields across every controller and both
        // operation kinds, including differently named aliases.
        for owner in METADATA.peripherals {
            let Some(other) = &owner.rcc_control else {
                continue;
            };
            let banks: Vec<_> = METADATA
                .peripherals
                .iter()
                .filter(|p| p.name == other.controller)
                .collect();
            assert_eq!(banks.len(), 1);
            let bank = banks[0];
            for (other_kind, reference) in [
                ("enable", Some(&other.enable)),
                ("reset", other.reset.as_ref()),
            ] {
                let Some(reference) = reference else { continue };
                let (other_offset, other_bit, width) =
                    rcc_field(bank.registers.as_ref().unwrap(), reference);
                if bank.address + u64::from(other_offset) * 4 == physical
                    && u32::from(other_bit) <= u32::from(target_bit)
                    && u32::from(target_bit) < u32::from(other_bit) + width
                {
                    assert_eq!(
                        (owner.name, other_kind),
                        (peripheral.name, kind),
                        "L012 I2C gate/reset has an aliased physical owner"
                    );
                    owners += 1;
                }
            }
        }
        assert_eq!(owners, 1);
    }
    for (pin, signal, af) in routes {
        assert_eq!(
            peripheral
                .pins
                .iter()
                .filter(|p| (p.pin, p.signal, p.af) == (pin, signal, Some(af)))
                .count(),
            1
        );
        assert_eq!(
            METADATA.pins.iter().filter(|p| p.name == pin).count(),
            1,
            "selected L012 I2C route is not bonded"
        );
    }
    if METADATA.name == "CW32L012" {
        assert!(
            METADATA.memory.is_empty(),
            "family profile must not invent an executable memory map"
        );
    } else {
        assert_eq!(METADATA.memory.len(), 1);
        let memory = METADATA.memory[0];
        assert_eq!(memory.len(), 2);
        assert_eq!(
            (
                memory[0].name,
                &memory[0].kind,
                memory[0].address,
                memory[0].size
            ),
            ("FLASH", &MemoryRegionKind::Flash, 0, 65536)
        );
        assert_eq!(
            (
                memory[1].name,
                &memory[1].kind,
                memory[1].address,
                memory[1].size
            ),
            ("RAM", &MemoryRegionKind::Ram, 0x2000_0000, 8192)
        );
    }
    link.interrupt
}

// Admit only the own-source-qualified classic families/IPs. The mask-and-wake
// protocol needs a physically exclusive vector and reset, including aliases.
fn classic_i2c_irq(peripheral: &cw32_metapac::metadata::Peripheral) -> &'static str {
    use cw32_metapac::metadata::METADATA;
    let (version, instances): (&str, &[&str]) = match METADATA.line {
        "CW32F030" | "CW32A030" | "CW32F020" => ("v1", &["I2C1", "I2C2"]),
        "CW32F002" | "CW32F003" => ("cw32f002_v1", &["I2C"]),
        "CW32L010" => ("cw32l010_v1", &["I2C1"]),
        "CW32L011" => ("cw32l011_v1", &["I2C"]),
        "CW32L031" | "CW32R031" => ("cw32l031_v1", &["I2C1"]),
        "CW32L052" | "CW32L083" => ("cw32l031_v1", &["I2C1", "I2C2"]),
        _ => panic!("unreviewed classic I2C family"),
    };
    let regs = peripheral
        .registers
        .as_ref()
        .expect("I2C registers missing");
    assert_eq!(regs.kind, "i2c");
    assert_eq!(regs.version, version, "unreviewed classic I2C IP");
    assert_eq!(regs.block, "I2c");
    let mut actual: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "i2c"))
        .map(|p| p.name)
        .collect();
    actual.sort_unstable();
    assert_eq!(actual, instances, "unreviewed classic I2C inventory");
    assert!(instances.contains(&peripheral.name));
    assert_eq!(
        peripheral.interrupts.len(),
        1,
        "classic I2C needs one dedicated GLOBAL link"
    );
    let link = &peripheral.interrupts[0];
    assert_eq!(link.signal, "GLOBAL");
    let vectors: Vec<_> = METADATA
        .interrupts
        .iter()
        .filter(|i| i.name == link.interrupt)
        .collect();
    assert_eq!(vectors.len(), 1, "classic I2C GLOBAL must resolve uniquely");
    let number = vectors[0].number;
    assert_eq!(
        METADATA
            .interrupts
            .iter()
            .filter(|i| i.number == number)
            .count(),
        1,
        "classic I2C vector number has an alias"
    );
    let mut sources = 0;
    for owner in METADATA.peripherals {
        for source in owner.interrupts {
            let resolved: Vec<_> = METADATA
                .interrupts
                .iter()
                .filter(|i| i.name == source.interrupt)
                .collect();
            if source.interrupt == link.interrupt || resolved.iter().any(|i| i.number == number) {
                assert_eq!(
                    owner.name, peripheral.name,
                    "classic I2C IRQ has another owner"
                );
                assert_eq!(
                    source.signal, "GLOBAL",
                    "classic I2C IRQ has another source"
                );
                sources += 1;
            }
        }
    }
    assert_eq!(sources, 1, "classic I2C IRQ must have exactly one source");
    let sysctrl = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap()
        .registers
        .as_ref()
        .unwrap();
    let control = peripheral
        .rcc_control
        .as_ref()
        .expect("classic I2C RCC metadata missing");
    assert_eq!(control.controller, "SYSCTRL");
    assert!(
        control.shared_reset_group.is_none(),
        "classic I2C reset is shared"
    );
    assert_eq!(control.reset_asserted_value, Some(false));
    let reset = control.reset.as_ref().expect("classic I2C reset missing");
    let (offset, bit, width) = rcc_field(sysctrl, reset);
    assert_eq!(width, 1);
    for owner in METADATA.peripherals {
        let Some(other) = &owner.rcc_control else {
            continue;
        };
        let Some(other_reset) = &other.reset else {
            continue;
        };
        if other.controller != control.controller {
            continue;
        }
        let (other_offset, other_bit, other_width) = rcc_field(sysctrl, other_reset);
        if other_offset == offset
            && u32::from(other_bit) <= u32::from(bit)
            && u32::from(bit) < u32::from(other_bit) + other_width
        {
            assert_eq!(
                owner.name, peripheral.name,
                "classic I2C reset has an aliased physical owner"
            );
            assert!(other.shared_reset_group.is_none());
        }
    }
    link.interrupt
}

// Emit only source-qualified associations, using the selected bank's actual
// GLOBAL link. A changed route or vector membership needs a new source review.
// Each event owner gets only its own actual GLOBAL route. L012 GTIM3/4
// retain distinct state while sharing GTIM34; no other vector owner is admitted.
fn timer_overflow_irq(peripheral: &cw32_metapac::metadata::Peripheral) -> &'static str {
    let metadata = &cw32_metapac::metadata::METADATA;
    let regs = peripheral
        .registers
        .as_ref()
        .expect("timer registers missing");
    let shared = regs.kind == "gtim"
        && regs.version == "cw32l012_v1"
        && matches!(peripheral.name, "GTIM3" | "GTIM4");
    let (expected_irq, number) = match peripheral.name {
        "ATIM" => {
            assert_eq!((regs.kind, regs.block), ("atim", "Atim"));
            ("ATIM", 13)
        }
        "GTIM" => ("GTIM", 16),
        "GTIM1" => ("GTIM1", 16),
        "GTIM2" => ("GTIM2", 17),
        "GTIM3" if shared => ("GTIM34", 18),
        "GTIM4" if shared => ("GTIM34", 18),
        "GTIM3" => ("GTIM3", 18),
        "GTIM4" => ("GTIM4", 19),
        _ => panic!("unqualified counter event instance"),
    };
    if peripheral.name != "ATIM" {
        assert_eq!((regs.kind, regs.block), ("gtim", "Gtim"));
    }
    assert_eq!(
        peripheral.interrupts.len(),
        1,
        "timer requires one GLOBAL link"
    );
    let irq = &peripheral.interrupts[0];
    assert_eq!(irq.signal, "GLOBAL");
    assert_eq!(irq.interrupt, expected_irq);
    let vectors: Vec<_> = metadata
        .interrupts
        .iter()
        .filter(|i| i.name == expected_irq)
        .collect();
    assert_eq!(vectors.len(), 1);
    assert_eq!(vectors[0].number, number);
    assert_eq!(
        metadata
            .interrupts
            .iter()
            .filter(|i| i.number == number)
            .count(),
        1
    );
    let mut owners = Vec::new();
    for candidate in metadata.peripherals {
        for link in candidate.interrupts {
            if link.interrupt == expected_irq {
                assert_eq!(link.signal, "GLOBAL");
                owners.push(candidate.name);
            }
        }
    }
    owners.sort_unstable();
    let mut expected_owners = if shared {
        vec!["GTIM3", "GTIM4"]
    } else {
        vec![peripheral.name]
    };
    expected_owners.sort_unstable();
    assert_eq!(owners, expected_owners, "unqualified timer vector owners");
    irq.interrupt
}

fn generate_btim_interrupts(out: &mut String, version: &str) {
    let metadata = &cw32_metapac::metadata::METADATA;
    let mut banks: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "btim"))
        .collect();
    banks.sort_by_key(|p| p.name);
    assert_eq!(
        banks.iter().map(|p| p.name).collect::<Vec<_>>(),
        ["BTIM1", "BTIM2", "BTIM3"],
        "unreviewed BTIM bank inventory"
    );
    for bank in banks {
        let regs = bank.registers.as_ref().unwrap();
        assert_eq!(regs.version, version);
        assert_eq!(regs.block, "Btim");
        let number = match bank.name {
            "BTIM1" => 20,
            "BTIM2" => 21,
            "BTIM3" => 22,
            _ => unreachable!(),
        };
        let (expected_irq, partner) = match (metadata.line, bank.name) {
            ("CW32L052", "BTIM2") => ("BTIM2_LPTIM", Some("LPTIM")),
            ("CW32L083", "BTIM2") => ("BTIM2_LPTIM1", Some("LPTIM")),
            ("CW32L012", "BTIM3") => ("BTIM3_HALLTIM", Some("HALLTIM")),
            _ => (bank.name, None),
        };
        assert_eq!(bank.interrupts.len(), 1, "BTIM requires one GLOBAL link");
        let link = &bank.interrupts[0];
        assert_eq!(link.signal, "GLOBAL");
        assert_eq!(link.interrupt, expected_irq, "unreviewed BTIM IRQ route");
        let vectors: Vec<_> = metadata
            .interrupts
            .iter()
            .filter(|i| i.name == link.interrupt)
            .collect();
        assert_eq!(vectors.len(), 1, "BTIM GLOBAL link must name one vector");
        assert_eq!(vectors[0].number, number, "unreviewed BTIM IRQ number");
        assert_eq!(
            metadata
                .interrupts
                .iter()
                .filter(|i| i.number == number)
                .count(),
            1,
            "BTIM vector number must be unique"
        );
        let mut owners = Vec::new();
        for peripheral in metadata.peripherals {
            for irq in peripheral.interrupts {
                if irq.interrupt == link.interrupt {
                    assert_eq!(irq.signal, "GLOBAL");
                    owners.push(peripheral.name);
                }
            }
        }
        owners.sort_unstable();
        let mut expected_owners = vec![bank.name];
        expected_owners.extend(partner);
        expected_owners.sort_unstable();
        assert_eq!(owners, expected_owners, "unreviewed BTIM IRQ owners");
        out.push_str(&format!(
            "crate::timer::impl_btim!({}, {});\n",
            bank.name, link.interrupt
        ));
    }
}

// Bank order, compact state slots and complete vector memberships all come
// from actual GPIO peripherals and their GLOBAL links, not family-name tables.
fn generate_gpio_interrupts(out: &mut String) {
    let metadata = &cw32_metapac::metadata::METADATA;
    let mut banks: Vec<_> = metadata
        .peripherals
        .iter()
        .filter(|p| p.name.starts_with("GPIO"))
        .collect();
    banks.sort_by_key(|p| p.name);
    let mut serviced = [0u16; 6];
    let mut commands = [0u16; 6];
    let mut state_indices = [usize::MAX; 6];
    let mut groups = std::collections::BTreeMap::<&str, Vec<u8>>::new();
    let mut level_trigger = None;
    for (index, bank) in banks.iter().enumerate() {
        assert!(bank.name.len() == 5);
        let port = bank.name.as_bytes()[4] - b'A';
        assert!(port < 6);
        let facts = bank
            .gpio_interrupt
            .as_ref()
            .expect("missing GPIO interrupt facts");
        assert_ne!(facts.serviced_mask, 0);
        assert_eq!(facts.serviced_mask & !facts.clear_noop_mask, 0);
        assert_eq!(
            *level_trigger.get_or_insert(facts.level_trigger),
            facts.level_trigger,
            "selected GPIO family has differing trigger policies"
        );
        serviced[usize::from(port)] = facts.serviced_mask;
        commands[usize::from(port)] = facts.clear_noop_mask;
        state_indices[usize::from(port)] = index;
        let links: Vec<_> = bank
            .interrupts
            .iter()
            .filter(|i| i.signal == "GLOBAL")
            .collect();
        assert_eq!(
            links.len(),
            1,
            "GPIO bank must have exactly one GLOBAL interrupt"
        );
        let irq = links[0].interrupt;
        assert_eq!(
            metadata.interrupts.iter().filter(|i| i.name == irq).count(),
            1,
            "GPIO GLOBAL link must name one real interrupt"
        );
        groups.entry(irq).or_default().push(port);
    }
    if level_trigger == Some(true) {
        println!("cargo:rustc-cfg=gpio_irq_level");
    }
    out.push_str(&format!(
        "pub(crate) const GPIO_SERVICED_MASKS: [u16; 6] = {serviced:?};\n"
    ));
    out.push_str(&format!(
        "pub(crate) const GPIO_CLEAR_NOOP_MASKS: [u16; 6] = {commands:?};\n"
    ));
    out.push_str(&format!(
        "pub(crate) const GPIO_BANK_COUNT: usize = {};\n",
        banks.len()
    ));
    // Use explicit absent slots instead of host-width usize::MAX literals.
    let state_indices: Vec<_> = state_indices
        .iter()
        .map(|index| {
            if *index == usize::MAX {
                "usize::MAX".into()
            } else {
                index.to_string()
            }
        })
        .collect();
    out.push_str(&format!(
        "pub(crate) const GPIO_STATE_INDEX: [usize; 6] = [{}];\n",
        state_indices.join(", ")
    ));
    let mut impls = String::from("// Generated from selected-chip GPIO GLOBAL interrupt links.\n");
    for (irq, ports) in groups {
        impls.push_str(&format!("impl sealed::GpioInterrupt for typelevel::{irq} {{}}\nimpl GpioInterrupt for typelevel::{irq} {{ const BANKS: &'static [u8] = &{ports:?}; }}\n"));
        if ports.len() == 1 {
            impls.push_str(&format!(
                "impl PortInterrupt for typelevel::{irq} {{ const PORT: u8 = {}; }}\n",
                ports[0]
            ));
        }
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("_generated_exti.rs"),
        impls,
    )
    .unwrap();
}

// Exact owned-LSE AWT admission reuses current LSE and AWT metadata. It must
// remain closed when another package or another timer later gains LSE support.
fn qualify_awt_lse() {
    use cw32_metapac::metadata::{METADATA, ir};

    let expected = match METADATA.name {
        "CW32F020C6U7" => Some(("CW32F020", "cw32f030_v1")),
        "CW32F030C8T7" => Some(("CW32F030", "cw32f030_v1")),
        "CW32A030C8T7" => Some(("CW32A030", "cw32f030_v1")),
        "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6" => Some(("CW32L031", "cw32l031_v1")),
        "CW32R031C8U6" => Some(("CW32R031", "cw32l031_v1")),

        _ => None,
    };
    let sysctrl = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .expect("missing SYSCTRL for AWT source admission");
    let lse = sysctrl
        .clock_limits
        .as_ref()
        .and_then(|c| c.lse_configuration.as_ref());
    let awts: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "awt"))
        .collect();
    assert!(
        awts.len() <= 1,
        "owned LSE AWT requires one standalone timer"
    );
    let declared = lse.and_then(|c| c.awt_source);
    assert_eq!(
        declared.is_some(),
        expected.is_some(),
        "exact LSE AWT source qualification missing or unexpected"
    );
    assert_eq!(
        lse.is_some() && !awts.is_empty(),
        expected.is_some(),
        "standalone AWT/LSE intersection changed; review exact admission"
    );
    let Some((line, version)) = expected else {
        return;
    };
    assert_eq!(METADATA.line, line);
    assert_eq!(declared, Some(3));
    let lse = lse.unwrap();
    assert!(lse.startup_consumers.is_none() && lse.native_low_power.is_none());
    assert_eq!((lse.nominal_hz, lse.maximum_hz), (32_768, 1_000_000));
    for (signal, pin) in [("LSE_IN", "PC14"), ("LSE_OUT", "PC15")] {
        let routes: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == signal).collect();
        assert_eq!(routes.len(), 1);
        assert_eq!((routes[0].pin, routes[0].af), (pin, None));
        assert_eq!(METADATA.pins.iter().filter(|p| p.name == pin).count(), 1);
    }
    let awt = awts[0];
    assert_eq!((awt.name, awt.address), ("AWT", 0x4001_4c00));
    let regs = awt.registers.as_ref().unwrap();
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("awt", version, "Awt")
    );
    let control = awt.rcc_control.as_ref().expect("AWT configuration clock");
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "PCLK"));
    assert_eq!(
        (control.enable.register, control.enable.field),
        ("apben2", "awt")
    );
    assert!(control.enable_active_value && control.enable_write_key.is_none());
    let reset = control.reset.as_ref().expect("AWT reset metadata");
    assert_eq!((reset.register, reset.field), ("apbrst2", "awt"));
    assert_eq!(control.reset_asserted_value, Some(false));
    assert_eq!(awt.interrupts.len(), 1);
    assert_eq!(
        (awt.interrupts[0].signal, awt.interrupts[0].interrupt),
        ("GLOBAL", "AWT")
    );
    assert_eq!(
        METADATA
            .interrupts
            .iter()
            .filter(|i| i.name == "AWT" && i.number == 30)
            .count(),
        1
    );

    let block = regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap();
    let cr = block
        .items
        .iter()
        .find(|r| r.name.eq_ignore_ascii_case("CR"))
        .unwrap();
    assert_eq!(cr.byte_offset, 0);
    assert!(cr.array.is_none());
    let ir::BlockItemInner::Register(cr) = &cr.inner else {
        panic!("AWT CR must be a register");
    };
    assert_eq!(cr.access, ir::Access::ReadWrite);
    assert_eq!(cr.bit_size, 32);
    let fields = regs
        .ir
        .fieldsets
        .iter()
        .find(|f| Some(f.name) == cr.fieldset)
        .unwrap();
    for (name, offset, width) in [("SRC", 8, 3), ("PRS", 4, 4), ("MD", 1, 2), ("EN", 0, 1)] {
        let field = fields
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(field.array.is_none());
        assert_eq!(field.bit_size, width);
        assert!(matches!(&field.bit_offset, ir::BitOffset::Regular(p) if p.offset == offset));
        if name == "SRC" {
            let source = regs
                .ir
                .enums
                .iter()
                .find(|e| Some(e.name) == field.enumm)
                .unwrap();
            assert_eq!(source.bit_size, 3);
            assert_eq!(
                source
                    .variants
                    .iter()
                    .filter(|v| v.name.eq_ignore_ascii_case("LSE") && v.value == 3)
                    .count(),
                1
            );
            assert_eq!(source.variants.iter().filter(|v| v.value == 3).count(), 1);
        }
    }
    println!("cargo:rustc-cfg=awt_lse");
}

// Exact-part MCO admission, independent of family-wide alternate-function lists.
fn generate_mco(out: &mut String, time_driver: Option<&str>) {
    use cw32_metapac::metadata::{METADATA, PeripheralRegisters, ir};
    use std::fmt::Write;

    let qualified: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter_map(|p| {
            p.clock_limits
                .as_ref()
                .and_then(|c| c.mco.as_ref())
                .map(|m| (p, m))
        })
        .collect();
    assert_eq!(
        qualified.len(),
        usize::from(matches!(
            METADATA.name,
            "CW32F030C8T7" | "CW32F030K8T7" | "CW32F030K8U7" | "CW32A030C8T7"
        ))
    );
    let Some(&(sysctrl, mco)) = qualified.first() else {
        return;
    };
    let (line, lse_bonded) = match METADATA.name {
        "CW32F030C8T7" => ("CW32F030", true),
        "CW32F030K8T7" | "CW32F030K8U7" => ("CW32F030", false),
        "CW32A030C8T7" => ("CW32A030", true),
        _ => unreachable!("unqualified exact MCO part"),
    };
    assert_eq!(METADATA.line, line);
    assert_eq!((sysctrl.name, sysctrl.address), ("SYSCTRL", 0x4001_0000));
    assert_ne!(time_driver, Some(sysctrl.name));
    assert_eq!(
        METADATA
            .peripherals
            .iter()
            .filter(|p| p.name == "SYSCTRL")
            .count(),
        1
    );
    let sys = sysctrl.registers.as_ref().unwrap();
    assert_eq!(
        (sys.kind, sys.version, sys.block),
        ("sysctrl", "v1", "Sysctrl")
    );
    let gpios: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| p.name == "GPIOA")
        .collect();
    assert_eq!(gpios.len(), 1);
    let gpio = gpios[0];
    assert_eq!(gpio.address, 0x4800_0000);
    let regs = gpio.registers.as_ref().unwrap();
    assert_eq!(
        (regs.kind, regs.version, regs.block),
        ("gpio", "v1", "Gpio")
    );
    assert_eq!(METADATA.pins.iter().filter(|p| p.name == "PA8").count(), 1);
    assert!(safe_pin("PA8"));
    let routes: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == "MCO").collect();
    assert_eq!(routes.len(), 1);
    let route = routes[0];
    assert_eq!((route.pin, route.signal, route.af), ("PA8", "MCO", Some(4)));
    assert!(route.adc_mux.is_none() && route.comparator_mux.is_none());
    // These are the actual reservation maps consulted by Flex::new. None can
    // claim PA8, even if HSE/LSE is retained from an earlier boot stage.
    for (signal, pin) in [("HSE_IN", "PF0"), ("HSE_OUT", "PF1")] {
        let pins: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == signal).collect();
        assert_eq!(pins.len(), 1);
        assert_eq!((pins[0].pin, pins[0].af), (pin, None));
        assert_eq!(METADATA.pins.iter().filter(|p| p.name == pin).count(), 1);
    }
    for (signal, pin) in [("LSE_IN", "PC14"), ("LSE_OUT", "PC15")] {
        if lse_bonded {
            let pins: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == signal).collect();
            assert_eq!(pins.len(), 1);
            assert_eq!((pins[0].pin, pins[0].af), (pin, None));
            assert_eq!(METADATA.pins.iter().filter(|p| p.name == pin).count(), 1);
        } else {
            // K8 packages retain the GPIOC bank and die-level LSE registers,
            // but neither unbonded pin nor any LSE route may become a token.
            assert!(!METADATA.pins.iter().any(|p| p.name == pin));
            assert!(
                !METADATA
                    .peripherals
                    .iter()
                    .flat_map(|p| p.pins)
                    .any(|p| p.pin == pin || p.signal == signal)
            );
        }
    }

    assert_eq!(
        (mco.disabled_source, mco.hclk_source, mco.pclk_source),
        (0, 1, 2)
    );
    assert_eq!(
        mco.divisors,
        &[
            (0, 1),
            (1, 2),
            (2, 8),
            (3, 64),
            (4, 128),
            (5, 256),
            (6, 512),
            (7, 1024)
        ]
    );
    assert_eq!(mco.maximum_output_hz, 20_000_000);
    assert_eq!(mco.supply_mv, (2400, 5500));
    assert_eq!(mco.temperature_c, (-40, 105));
    assert_eq!(mco.maximum_load_pf, 50);
    assert_eq!(mco.gpio_speed, 1);
    // These are conditions of the unmeasured vendor fmax table, not waveform
    // properties established by software, cycle bounds, or integer division.
    assert_eq!(mco.fmax_condition_duty_percent, (45, 55));
    assert_eq!(mco.fmax_condition_edge_sum_period_ratio, (2, 3));
    assert!(!mco.electrical_values_measured);

    fn fields(regs: &PeripheralRegisters, name: &str, byte_offset: u32) -> &'static [ir::Field] {
        let blocks: Vec<_> = regs
            .ir
            .blocks
            .iter()
            .filter(|b| b.name == regs.block)
            .collect();
        assert_eq!(blocks.len(), 1);
        let items: Vec<_> = blocks[0]
            .items
            .iter()
            .filter(|r| r.name.eq_ignore_ascii_case(name))
            .collect();
        assert_eq!(items.len(), 1);
        let item = items[0];
        assert_eq!(item.byte_offset, byte_offset);
        assert!(item.array.is_none());
        let ir::BlockItemInner::Register(reg) = &item.inner else {
            panic!("MCO requires a register")
        };
        assert_eq!(reg.access, ir::Access::ReadWrite);
        assert_eq!(reg.bit_size, 32);
        let sets: Vec<_> = regs
            .ir
            .fieldsets
            .iter()
            .filter(|f| Some(f.name) == reg.fieldset)
            .collect();
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0].bit_size, 32);
        assert!(sets[0].extends.is_none());
        sets[0].fields
    }
    fn field(fields: &[ir::Field], name: &str, bit: u32, width: u32) {
        let found: Vec<_> = fields
            .iter()
            .filter(|f| f.name.eq_ignore_ascii_case(name))
            .collect();
        assert_eq!(found.len(), 1);
        let f = found[0];
        assert_eq!(f.bit_size, width);
        assert!(f.array.is_none() && f.enumm.is_none());
        let ir::BitOffset::Regular(position) = &f.bit_offset else {
            panic!("MCO requires a linear field")
        };
        assert_eq!(position.offset, bit);
    }
    let mco_fields = fields(sys, "MCO", 0x70);
    assert_eq!(mco_fields.len(), 2);
    field(mco_fields, "SOURCE", 0, 4);
    field(mco_fields, "DIV", 4, 3);
    for (name, offset) in [
        ("DIR", 0x00),
        ("OPENDRAIN", 0x04),
        ("SPEED", 0x08),
        ("PDR", 0x0c),
        ("PUR", 0x10),
        ("ANALOG", 0x1c),
        ("RISEIE", 0x24),
        ("FALLIE", 0x28),
        ("HIGHIE", 0x2c),
        ("LOWIE", 0x30),
    ] {
        field(fields(regs, name, offset), "PIN8", 8, 1);
    }
    field(fields(regs, "AFRH", 0x14), "AFR8", 0, 4);
    let lock = fields(regs, "LOCK", 0x3c);
    assert_eq!(lock.len(), 17);
    for pin in 0..16 {
        field(lock, &format!("PIN{pin}"), pin, 1);
    }
    field(lock, "KEY", 16, 16);
    let pads = gpio.gpio.as_ref().unwrap();
    assert_ne!(pads.output_mask & (1 << 8), 0);
    assert_ne!(pads.pull_down_mask & (1 << 8), 0);
    let irq = gpio.gpio_interrupt.as_ref().unwrap();
    assert!(irq.level_trigger);
    assert_ne!(irq.serviced_mask & (1 << 8), 0);
    // Normal per-pin LOCK, Speed1 and the real retained bank gate are required.
    // No private GPIO policy or synthetic peripheral is emitted for MCO.
    let control = gpio.rcc_control.as_ref().unwrap();
    assert_eq!((control.controller, control.bus_clock), ("SYSCTRL", "HCLK"));
    assert_eq!(
        (control.enable.register, control.enable.field),
        ("ahben", "gpioa")
    );
    assert!(control.enable_active_value && control.enable_write_key.is_none());
    let reset = control.reset.as_ref().unwrap();
    assert_eq!((reset.register, reset.field), ("ahbrst", "gpioa"));
    assert_eq!(control.reset_asserted_value, Some(false));
    assert!(control.shared_enable_group.is_none() && control.shared_reset_group.is_none());
    field(fields(sys, "AHBEN", 0x30), "GPIOA", 4, 1);
    field(fields(sys, "AHBRST", 0x40), "GPIOA", 4, 1);
    let gate = gpio.rcc.as_ref().unwrap();
    assert_eq!(gate.bus_clock, "HCLK");
    assert_eq!(gate.enable.as_ref(), Some(&control.enable));
    assert_eq!(gate.reset.as_ref(), Some(reset));

    writeln!(
        out,
        "pub(crate) const MCO_DISABLED_SOURCE: u8 = {};",
        mco.disabled_source
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_HCLK_SOURCE: u8 = {};",
        mco.hclk_source
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_PCLK_SOURCE: u8 = {};",
        mco.pclk_source
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_DIVISORS: [(u8, u16); 8] = {:?};",
        mco.divisors
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_MAXIMUM_OUTPUT_HZ: u32 = {};",
        mco.maximum_output_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_SUPPLY_MV: (u16, u16) = {:?};",
        mco.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_TEMPERATURE_C: (i16, i16) = {:?};",
        mco.temperature_c
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_MAXIMUM_LOAD_PF: u16 = {};",
        mco.maximum_load_pf
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const MCO_GPIO_SPEED: u8 = {};",
        mco.gpio_speed
    )
    .unwrap();
    writeln!(out, "pub(crate) const MCO_AF: u8 = {};", route.af.unwrap()).unwrap();
    println!("cargo:rustc-cfg=mco_f030");
}

// Selected chip facts are data, not register-version selection logic.
fn generate_electrical(out: &mut String) {
    use std::fmt::Write;
    let peripherals = cw32_metapac::metadata::METADATA.peripherals;
    let c = peripherals
        .iter()
        .find_map(|p| p.clock_limits.as_ref())
        .expect("missing SYSCTRL electrical facts");
    writeln!(
        out,
        "pub(crate) const RCC_HSI_FREQUENCY_HZ: u32 = {};",
        c.hsi_frequency_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSI_ERROR_PERCENT: u32 = {};",
        c.hsi_error_percent
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSI_SUPPLY_RANGE_MV: (u16,u16) = {:?};",
        c.hsi_supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSI_TEMPERATURE_RANGE_C: (i16,i16) = {:?};",
        c.hsi_temperature_c
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LOW_VOLTAGE_THRESHOLD_MV: u16 = {};",
        c.low_voltage_threshold_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LOW_VOLTAGE_BUS_MAX_HZ: u32 = {};",
        c.low_voltage_bus_max_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HIGH_VOLTAGE_BUS_MAX_HZ: u32 = {};",
        c.high_voltage_bus_max_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_FACTORY_HSI_TRIM_ADDRESS: usize = {};",
        c.factory_hsi_trim_address
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_DEFAULT_HSI_DIV: crate::rcc::HsiDiv = crate::rcc::HsiDiv::Div{};",
        c.default_hsi_divisor
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_INITIAL_FLASH_WAIT: u32 = {};",
        c.initial_flash_wait
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_FLASH_WAIT_STEP_HZ: u32 = {};",
        c.flash_wait_step_hz
    )
    .unwrap();
    if c.hse.is_some() {
        println!("cargo:rustc-cfg=rcc_hse");
    }
    if c.hse.is_some() || c.hex.is_some() {
        println!("cargo:rustc-cfg=rcc_external_clock");
    }
    generate_lse(out, c);
    generate_lse_configuration(out, c);
    generate_pll(out, c);
    generate_factory_lsi(out, c);
    generate_hse(out, c);
    generate_hex(out, c);
    let a = peripherals
        .iter()
        .find_map(|p| p.adc_limits.as_ref())
        .expect("missing ADC electrical facts");
    for p in peripherals.iter().filter_map(|p| p.adc_limits.as_ref()) {
        assert_eq!(
            p, a,
            "ADC limits differ; use per-instance constants before adding this chip"
        );
    }
    if let Some(sequence) = &a.sequence {
        assert!(sequence.programmable_order && sequence.per_slot_result);
        if a.classic_scan.is_none() {
            assert!(sequence.per_slot_sample_time);
            assert_eq!(sequence.maximum_length, 8);
        }
        writeln!(
            out,
            "pub(crate) const ADC_SEQUENCE_MAX_LEN: usize = {};",
            sequence.maximum_length
        )
        .unwrap();
    }
    if let Some(scan) = &a.classic_scan {
        writeln!(
            out,
            "pub(crate) const ADC_SEQUENCE_SLOTS_PER_REGISTER: usize = {};",
            scan.slots_per_sequence_register
        )
        .unwrap();
        let sequence = a
            .sequence
            .as_ref()
            .expect("missing classic ADC sequence capabilities");
        assert!(matches!(sequence.maximum_length, 4 | 8));
        assert!(
            sequence.programmable_order
                && !sequence.per_slot_sample_time
                && sequence.per_slot_result
        );
        writeln!(
            out,
            "pub(crate) const ADC_BUFFERED_REQUIRES_SINGLE_CHANNEL: bool = {};",
            scan.buffered_requires_single_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_INTERNAL_REQUIRES_SINGLE_CHANNEL: bool = {};",
            scan.internal_requires_single_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_FIRST_INTERNAL_CHANNEL: u8 = {};",
            scan.first_internal_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_SUPPLY_CHANNEL: u8 = {};",
            scan.supply_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_TEMPERATURE_CHANNEL: Option<u8> = {:?};",
            scan.temperature_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_BANDGAP_CHANNEL: Option<u8> = {:?};",
            scan.bandgap_channel
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_TEMPERATURE_STARTUP_US: u32 = {};",
            scan.temperature_startup_us
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_BANDGAP_STARTUP_US: u32 = {};",
            scan.bandgap_startup_us
        )
        .unwrap();
    }
    writeln!(
        out,
        "pub(crate) const ADC_SUPPLY_RANGE_MV: (u16,u16) = {:?};",
        a.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const ADC_SAMPLE_CYCLES: &[u16] = &{:?};",
        a.sample_cycles
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const ADC_COMPARISON_CYCLES: u16 = {};",
        a.comparison_cycles
    )
    .unwrap();
    let bands = |bs: &[cw32_metapac::metadata::PeripheralAdcBand]| {
        bs.iter()
            .map(|b| {
                (
                    b.supply_min_mv,
                    b.maximum_clock_hz,
                    b.maximum_sample_rate_hz,
                    b.minimum_acquisition_ps,
                )
            })
            .collect::<Vec<_>>()
    };
    writeln!(
        out,
        "pub(crate) const ADC_SUPPLY_BANDS: &[(u16,u32,u32,u32)] = &{:?};",
        bands(a.supply_bands)
    )
    .unwrap();
    if !a.internal_1v5_bands.is_empty() {
        writeln!(
            out,
            "pub(crate) const ADC_INTERNAL_1V5_BANDS: &[(u16,u32,u32,u32)] = &{:?};",
            bands(a.internal_1v5_bands)
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_INTERNAL_2V5_BANDS: &[(u16,u32,u32,u32)] = &{:?};",
            bands(a.internal_2v5_bands)
        )
        .unwrap();
    }
    if a.sample_cycles.len() > 4 {
        writeln!(
            out,
            "pub(crate) const ADC_MINIMUM_CLOCK_HZ: u32 = {};",
            a.minimum_clock_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_INTERNAL_ACQUISITION_PS: u64 = {};",
            a.internal_acquisition_ps
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "pub(crate) const ADC_INPUT_FOLLOWER_MAXIMUM_RATE_HZ: u32 = {};",
            a.input_follower_maximum_rate_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const ADC_TEMPERATURE_ACQUISITION_MAXIMUM_RATE_HZ: u32 = {};",
            a.temperature_acquisition_maximum_rate_hz
        )
        .unwrap();
    }
    let w = peripherals
        .iter()
        .find_map(|p| p.iwdt_clock.as_ref())
        .expect("missing IWDT clock qualification");
    writeln!(out, "pub(crate) const IWDT_CLOCK: crate::wdg::ClockRange = crate::wdg::ClockRange {{ typical_hz: {}, min_hz: {}, max_hz: {} }};", w.typical_hz, w.minimum_hz, w.maximum_hz).unwrap();
    writeln!(
        out,
        "pub(crate) const IWDT_USES_LSI: bool = {};",
        w.uses_lsi
    )
    .unwrap();
    let i = peripherals
        .iter()
        .find_map(|p| p.i2c_limits.as_ref())
        .expect("missing I2C frequency qualification");
    for p in peripherals.iter().filter_map(|p| p.i2c_limits.as_ref()) {
        assert_eq!(
            p, i,
            "I2C limits differ; use per-instance constants before adding this chip"
        );
    }
    writeln!(
        out,
        "pub(crate) const I2C_MAXIMUM_FREQUENCY_HZ: u32 = {};",
        i.maximum_frequency_hz
    )
    .unwrap();
    if let Some(w) = &i.waveform {
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_MAXIMUM_FREQUENCY_HZ: [u32;3] = {:?};",
            w.maximum_frequency_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_LOW: [u32;3] = {:?};",
            w.low
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_HIGH: [u32;3] = {:?};",
            w.high
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_HOLD: [u32;3] = {:?};",
            w.hold
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_SETUP: [u32;3] = {:?};",
            w.setup
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_DATA_SETUP: [u32;3] = {:?};",
            w.data_setup
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_DATA_VALID: [u32;3] = {:?};",
            w.data_valid
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const I2C_TIMING_RISE: [u32;3] = {:?};",
            w.rise
        )
        .unwrap();
    }
    if let Some(f) = peripherals.iter().find_map(|p| p.flash_limits.as_ref()) {
        writeln!(
            out,
            "pub(crate) const FLASH_SUPPLY_RANGE_MV: (u16,u16) = {:?};",
            f.supply_mv
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_LOCK_GROUP_BYTES: u32 = {};",
            f.lock_group_bytes
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_LOCK_MASK: u64 = {};",
            f.lock_mask
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_MAX_HCLK_HZ: u32 = {};",
            f.maximum_hclk_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_LOW_VOLTAGE_THRESHOLD_MV: u16 = {};",
            f.low_voltage_threshold_mv
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_LOW_VOLTAGE_MAX_HCLK_HZ: u32 = {};",
            f.low_voltage_maximum_hclk_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_WAIT_STEP_HZ: u32 = {};",
            f.wait_step_hz
        )
        .unwrap();
        writeln!(
            out,
            "pub(crate) const FLASH_MAX_WAIT_STATES: u32 = {};",
            f.maximum_wait_states
        )
        .unwrap();
    }
}

// Generate the same trait + RCC_INFO boundary used by Embassy STM32. CW32's
// independently verified controller facts remain separate from kernel clocks.
// Project package-qualified pads and use only reviewed native PAC getters.
fn generate_lse(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use cw32_metapac::metadata::ir;
    use std::fmt::Write;
    let sysctrl = cw32_metapac::metadata::METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .expect("missing SYSCTRL");
    let regs = sysctrl.registers.as_ref().unwrap();
    let block = regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap();
    let fieldset = |name: &str, offset: u32| {
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .expect("missing LSE ownership register");
        assert!(item.array.is_none());
        assert_eq!(item.byte_offset, offset);
        let ir::BlockItemInner::Register(register) = &item.inner else {
            panic!("LSE ownership control is not a register")
        };
        assert_eq!(register.bit_size, 32);
        assert!(matches!(
            register.access,
            ir::Access::ReadWrite | ir::Access::Read
        ));
        regs.ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == register.fieldset)
            .unwrap()
    };
    let cr1 = fieldset("CR1", 4);
    let lse_register = block
        .items
        .iter()
        .any(|r| r.name.eq_ignore_ascii_case("LSE"));
    assert_eq!(
        c.lse.is_some(),
        lse_register,
        "LSE presence lacks qualified facts"
    );
    let Some(lse) = &c.lse else {
        assert!(
            !cr1.fields
                .iter()
                .any(|f| matches!(f.name, "LSEEN" | "LSELOCK"))
        );
        assert!(
            !sysctrl
                .pins
                .iter()
                .any(|p| matches!(p.signal, "LSE_IN" | "LSE_OUT"))
        );
        out.push_str(
            "pub(crate) const RCC_LSE_PINS: (Option<u8>, Option<u8>) = (None, None);\n\
            pub(crate) fn rcc_lse_owned_pads() -> (bool, bool) { (false, false) }\n",
        );
        return;
    };
    let control = fieldset("LSE", 0x24);
    let field = |fields: &ir::FieldSet, name: &str, offset: u32| {
        let field = fields
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name))
            .expect("missing native LSE ownership getter");
        assert!(field.array.is_none() && field.enumm.is_none());
        assert_eq!(field.bit_size, 1);
        assert!(matches!(&field.bit_offset, ir::BitOffset::Regular(p) if p.offset == offset));
    };
    field(cr1, "LSEEN", 4);
    field(cr1, "LSELOCK", 5);
    field(control, "MODE", 6);
    assert!(lse.pin_lock || !lse.pin_lock_requires_enable_lock);
    assert_eq!(
        lse.pin_lock,
        control
            .fields
            .iter()
            .any(|f| f.name.eq_ignore_ascii_case("PINLOCK"))
    );
    if lse.pin_lock {
        field(control, "PINLOCK", 17);
    }
    let mut pins = [None; 2];
    for (index, signal) in ["LSE_IN", "LSE_OUT"].iter().enumerate() {
        let found: Vec<_> = sysctrl
            .pins
            .iter()
            .filter(|p| p.signal == *signal)
            .collect();
        assert!(found.len() <= 1, "ambiguous LSE pad");
        if let Some(pin) = found.first() {
            assert!(pin.af.is_none(), "oscillator pad is not a digital AF");
            let port = pin.pin.as_bytes()[1] - b'A';
            let bit: u8 = pin.pin[2..].parse().unwrap();
            assert!(port < 6 && bit < 16);
            pins[index] = Some(port * 16 + bit);
        }
    }
    writeln!(
        out,
        "pub(crate) const RCC_LSE_PINS: (Option<u8>, Option<u8>) = ({:?}, {:?});",
        pins[0], pins[1]
    )
    .unwrap();
    out.push_str(
        "pub(crate) fn rcc_lse_owned_pads() -> (bool, bool) {\n\
        let cr1 = crate::pac::SYSCTRL.cr1().read();\n\
        let lse = crate::pac::SYSCTRL.lse().read();\n",
    );
    let locked = match (lse.pin_lock, lse.pin_lock_requires_enable_lock) {
        (true, true) => "lse.pinlock() && cr1.lselock()",
        (true, false) => "lse.pinlock()",
        (false, false) => "false",
        (false, true) => unreachable!(),
    };
    writeln!(out, "let pad_lock = {locked};").unwrap();
    out.push_str("(cr1.lseen() || pad_lock, (cr1.lseen() && !lse.mode()) || pad_lock)\n}\n");
}

// Shared buffered inspection emission for independently qualified L052/L083.
// Family branches bind native facts; the existing L052 output stays byte-identical.
fn generate_l052_l083_factory_lsi(
    output: &mut String,
    c: &cw32_metapac::metadata::PeripheralClockLimits,
) {
    use cw32_metapac::metadata::{METADATA, ir};
    use std::fmt::Write;
    let l083 = METADATA.line == "CW32L083";
    assert!(matches!(METADATA.line, "CW32L052" | "CW32L083"));
    assert!(if l083 {
        matches!(METADATA.name, "CW32L083RBT6" | "CW32L083RCT6" | "CW32L083RCS6" | "CW32L083MCT6" | "CW32L083VCT6")
    } else { matches!(METADATA.name, "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6") });
    // The shared template retains the old internal names for L052. Only the
    // qualified L083 instantiation receives its own generated symbol prefix.
    let mut generated = String::new();
    let out = &mut generated;
    let lsi = c.lsi_sysclk.as_ref().unwrap();
    let lse = c.lse_configuration.as_ref().expect("L052 hardware startup facts");
    let native = lse.startup_consumers.as_ref().expect("L052 native consumers");
    assert_eq!(native.startup_analog, !l083);
    assert!(native.lptim.gate_controls_work && native.lcd.gate_controls_work);
    assert_eq!((native.autotrim_source, native.lptim.source, native.lcd.source), (3, 2, 1));
    assert_eq!(lsi.uarts, if l083 { &["UART1", "UART2", "UART3", "UART4", "UART5", "UART6"][..] } else { &["UART1", "UART2", "UART3"][..] });
    assert_eq!(lsi.gpio_banks, if l083 { &["GPIOA", "GPIOB", "GPIOC", "GPIOD", "GPIOE", "GPIOF"][..] } else { &["GPIOA", "GPIOB", "GPIOC", "GPIOD", "GPIOF"][..] });
    let gpio_start = 2 + lsi.uarts.len();
    let work_start = gpio_start + lsi.gpio_banks.len();
    let gate_count = work_start + 2;
    let hse_bank_index = lsi.gpio_banks.iter().position(|p| *p == "GPIOF").unwrap();
    let lse_bank_index = lsi.gpio_banks.iter().position(|p| *p == "GPIOC").unwrap();
    let gate_names: Vec<_> = ["RTC", "AUTOTRIM"].into_iter().chain(lsi.uarts.iter().copied())
        .chain(lsi.gpio_banks.iter().copied()).chain(["LPTIM", "LCD"]).collect();
    assert_eq!(gate_names.len(), gate_count);
    assert!(lsi.awt_allowed_sources.is_empty());
    assert!(!METADATA.peripherals.iter().any(|p| p.name == "AWT"));
    let peripheral = |name: &str| METADATA.peripherals.iter().find(|p| p.name == name).unwrap();
    let register = |name: &str, reg: &str| {
        let regs = peripheral(name).registers.as_ref().unwrap();
        let block = regs.ir.blocks.iter().find(|b| b.name == regs.block).unwrap();
        let items: Vec<_> = block.items.iter().filter(|r| r.name.eq_ignore_ascii_case(reg)).collect();
        assert_eq!(items.len(), 1);
        let item = items[0];
        assert!(item.array.is_none());
        let ir::BlockItemInner::Register(r) = &item.inner else { panic!("not a register") };
        assert_eq!(r.bit_size, 32);
        assert!(matches!(r.access, ir::Access::Read | ir::Access::ReadWrite));
        regs.ir.fieldsets.iter().find(|f| Some(f.name) == r.fieldset).unwrap()
    };
    let mask = |name: &str, reg: &str, field: &str, expected_bit: u32, expected_width: u32| {
        let fields: Vec<_> = register(name, reg).fields.iter().filter(|f| f.name.eq_ignore_ascii_case(field)).collect();
        assert_eq!(fields.len(), 1);
        let f = fields[0];
        assert!(f.array.is_none());
        assert_eq!(f.bit_size, expected_width);
        let ir::BitOffset::Regular(bit) = &f.bit_offset else { panic!("non-scalar L052 field") };
        assert_eq!(bit.offset, expected_bit);
        ((1u32 << f.bit_size) - 1) << bit.offset
    };
    for name in if l083 { &["CR0", "CR1", "CR2", "IER"][..] } else { &["CR0", "CR1", "IER"][..] } {
        let bits = mask("SYSCTRL", name, "KEY", 16, 16);
        writeln!(out, "pub(crate) const RCC_L052_LSI_{name}_KEY_MASK: u32 = {bits};").unwrap();
    }
    for (name, bit) in [("HSI", 15), ("HSE", 19), ("LSE", 15), ("LSI", 15)] {
        mask("SYSCTRL", name, "STABLE", bit, 1);
    }
    let rtc_mask = !mask("RTC", "CR1", "WINDOW", 1, 1);
    let lcd_mask = !mask("LCD", "CR1", "INTF", 12, 1);
    let lock_register = if l083 { "LCKR" } else { "LOCK" };
    let gpio_key_mask = mask("GPIOF", lock_register, "KEY", 16, 16);
    assert_eq!(gpio_key_mask, mask("GPIOC", lock_register, "KEY", 16, 16));
    if l083 {
        for name in lsi.gpio_banks { assert_eq!(gpio_key_mask, mask(name, lock_register, "KEY", 16, 16)); }
        let flash_key = mask("FLASH", "CR2", "KEY", 16, 16);
        mask("SYSCTRL", "CR2", "FLASHWAIT", 4, 3);
        let mut ready = 0; let mut events = 0;
        for (name, bit) in [("HSISTABLE", 11), ("HSESTABLE", 12), ("PLLSTABLE", 13), ("LSISTABLE", 14), ("LSESTABLE", 15)] {
            ready |= mask("SYSCTRL", "ISR", name, bit, 1);
        }
        for (name, bit) in [("HSIRDY", 0), ("HSERDY", 1), ("PLLRDY", 2), ("LSIRDY", 3), ("LSERDY", 4)] {
            events |= mask("SYSCTRL", "ISR", name, bit, 1);
        }
        mask("SYSCTRL", "CR1", "PLLEN", 2, 1);
        writeln!(out, "pub(crate) const RCC_L052_LSI_FLASH_CR2_KEY_MASK: u32 = {flash_key};\npub(crate) const RCC_L052_LSI_ISR_READY_MASK: u32 = {ready};\npub(crate) const RCC_L052_LSI_ISR_EVENT_MASK: u32 = {events};\npub(crate) const RCC_L052_LSI_HSE_PIN_STEPS: u8 = 12;").unwrap();
        assert_eq!(METADATA.interrupts.iter().filter(|i| i.name == "CLKFAULT" && i.number == 31).count(), 1);
        assert_eq!(METADATA.interrupts.iter().filter(|i| i.number == 31).count(), 1);
    }
    let lsi_routes: Vec<_> = if l083 { native.lsi_output_routes.iter().map(|r| (r.pin, r.af)).collect() } else { Vec::new() };
    let pll_routes: Vec<_> = if l083 { peripheral("SYSCTRL").pins.iter().filter(|r| r.signal == "PLL_OUT")
        .map(|r| { assert!(r.adc_mux.is_none() && r.comparator_mux.is_none()); (r.pin, r.af.unwrap()) }).collect() } else { Vec::new() };
    let unbonded_lsi: Vec<_> = if l083 { [("PD5", 6), ("PF2", 4)].into_iter()
        .filter(|(name, _)| !METADATA.pins.iter().any(|p| p.name == *name)).collect() } else { Vec::new() };
    let unbonded_pll: Vec<_> = if l083 && !METADATA.pins.iter().any(|p| p.name == "PF8") { vec![("PF8", 3)] } else { Vec::new() };
    if l083 {
        let expected_lsi = match METADATA.name { "CW32L083VCT6" => vec![("PC4",6),("PD5",6),("PF2",4)], "CW32L083MCT6" => vec![("PC4",6),("PF2",4)], _ => vec![("PC4",6)] };
        let expected_pll = if matches!(METADATA.name, "CW32L083MCT6" | "CW32L083VCT6") { vec![("PC12",5),("PC13",1),("PF8",3)] } else { vec![("PC12",5),("PC13",1)] };
        assert_eq!(lsi_routes, expected_lsi);
        assert_eq!(pll_routes, expected_pll);
        assert_eq!(lsi_routes.len() + unbonded_lsi.len(), 3);
        assert_eq!(pll_routes.len() + unbonded_pll.len(), 3);
    }
    // These source-qualified AF sets are independently checked against the
    // complete own raw AF table in lsi::apply_l083 before projection. Metadata
    // owns the bonded roster; native IR owns every emitted field and mask.
    let valid_afs = |pin: &str| -> &'static [u8] { match pin {
        "PC4" => &[0,1,2,3,6], "PD5" | "PC13" => &[0,1,2,3,4,5,6,7],
        "PF2" => &[0,1,3,4,5,6], "PC12" => &[0,1,2,3,4,5,6], "PF8" => &[0,1,3,4,5],
        _ => panic!("unqualified native clock output"),
    }};
    let faults = if l083 { "let flags = crate::pac::SYSCTRL.isr().read();\nif (monitor_hse && (flags.hsefail() || flags.hsefault())) || (monitor_lse && (flags.lsefail() || flags.lsefault())) || ((monitor_hse || monitor_lse) && cortex_m::peripheral::NVIC::is_pending(crate::pac::Interrupt::CLKFAULT)) { return Err(crate::rcc::Error::ExternalClockFault); }\n" } else { "let flags = crate::pac::SYSCTRL.isr().read();\nif (monitor_hse && (flags.hsefail() || flags.hsefault())) || (monitor_lse && (flags.lsefail() || flags.lsefault())) { return Err(crate::rcc::Error::ExternalClockFault); }\n" };
    // This is a code emitter, not a runtime gate adapter: each emitted window
    // names its actual RCC_INFO, register reads, and ownership predicate.
    let window = |out: &mut String, name: &str, result: &str, body: &str| {
        writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nlet original_gate = info.is_enabled();\nlet original_reset = info.reset_asserted();").unwrap();
        out.push_str(faults);
        out.push_str("if original_reset { return Err(crate::rcc::Error::LsiClockInUse); }\nlet buffered = info.inspect_for_init(cs, timeout, || {\nif !info.is_enabled() || info.reset_asserted() { return None; }\n");
        out.push_str(body);
        out.push_str("\nif !info.is_enabled() || info.reset_asserted() { return None; }\nSome(value)\n}).map_err(crate::rcc::lsi_inspection_error)?;\nif info.is_enabled() != original_gate { return Err(crate::rcc::Error::LsiGateRestoreTimeout); }\n");
        out.push_str(faults);
        writeln!(out, "if info.reset_asserted() != original_reset {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nlet {result} = buffered.ok_or(crate::rcc::Error::LsiClockInUse)?;").unwrap();
    };
    let consumer_header = r#"
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RccL052LsiConsumers {
    pub(crate) rtc_cr0: u32, pub(crate) rtc_cr1: u32,
    pub(crate) rtc_cr2: u32, pub(crate) rtc_compen: u32,
    pub(crate) autotrim_cr: u32, pub(crate) lvd_cr0: u32, pub(crate) lvd_cr1: u32,
    pub(crate) uart_cr2: [u32; 3], pub(crate) gpio_filter: [u32; 5],
    pub(crate) gpioc_afrl: u32, pub(crate) mco: u32,
    // RTC, AUTOTRIM, UART1/2/3, GPIOA/B/C/D/F, LPTIM, LCD.
    pub(crate) gates: [bool; 12], pub(crate) resets: [bool; 12],
    // None means the original work gate was off: no local MMIO occurred.
    pub(crate) lptim: Option<(u32, u32)>, pub(crate) lcd: Option<(u32, u32)>,
}
pub(crate) fn rcc_l052_lsi_consumers(timeout: u32, cs: critical_section::CriticalSection<'_>, cold: bool, monitor_hse: bool, monitor_lse: bool) -> Result<RccL052LsiConsumers, crate::rcc::Error> {
    use crate::rcc::SealedRccPeripheral;
    let mut gates = [false; 12]; let mut resets = [false; 12];
"#;
    if l083 {
        let mut header = consumer_header.replace("[u32; 3]", "[u32; 6]").replace("[u32; 5]", "[u32; 6]")
            .replace("[bool; 12]", "[bool; 16]").replace("[false; 12]", "[false; 16]")
            .replace("cold: bool, monitor_hse", "cold: bool, stop_pll: bool, monitor_hse")
            .replace("// RTC, AUTOTRIM, UART1/2/3, GPIOA/B/C/D/F, LPTIM, LCD.", "// RTC, AUTOTRIM, UART1..6, GPIOA..F, LPTIM, LCD.");
        let extra = format!("pub(crate) gpio_banks: [RccL052LsiPadBank; 6],\n    pub(crate) lsi_routes: [u32; {}], pub(crate) pll_routes: [u32; {}],\n    pub(crate) unbonded_lsi_routes: [u32; {}], pub(crate) unbonded_pll_routes: [u32; {}],\n    ", lsi_routes.len(), pll_routes.len(), unbonded_lsi.len(), unbonded_pll.len());
        header = header.replace("pub(crate) gpioc_afrl", &(extra + "pub(crate) gpioc_afrl"));
        out.push_str(&header);
    } else { out.push_str(consumer_header); }
    window(out, "RTC", "rtc", &format!("let cr0 = crate::pac::RTC.cr0().read();\nlet cr1 = crate::pac::RTC.cr1().read();\nlet cr2 = crate::pac::RTC.cr2().read();\nlet compen = crate::pac::RTC.compen().read();\nlet source = u8::from(cr1.source());\nlet allowed = if cold {{ {:?}.contains(&source) }} else {{ matches!(source, 0 | 2 | 4 | 5 | 6 | 7) }};\nlet value = ((cr0.0, cr1.0 & {rtc_mask}, cr2.0, compen.0), allowed);", lsi.rtc_allowed_sources));
    out.push_str("gates[0] = original_gate; resets[0] = original_reset;\nif !rtc.1 { return Err(crate::rcc::Error::LsiClockInUse); }\n");
    window(out, "AUTOTRIM", "autotrim", &format!("let cr = crate::pac::AUTOTRIM.cr().read();\nlet source = u8::from(cr.src());\nlet mode = u8::from(cr.md());\nlet allowed = !cr.auto() && matches!(mode, 0 | 1 | 3) && source <= 4 && (!cr.en() || (mode == 3 && cr.prs().to_bits() != 0)) && (!cold || {});\nlet value = (cr.0, allowed);", if l083 { "(source != 1 && mode != 1)" } else { "source != 1" }));
    writeln!(out, "gates[1] = original_gate; resets[1] = original_reset;\nif !autotrim.1 {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nlet mut uart_cr2 = [0; {}];", lsi.uarts.len()).unwrap();
    for (i, name) in lsi.uarts.iter().enumerate() {
        window(out, name, "uart", &format!("let cr2 = crate::pac::{name}.cr2().read();\nlet source = u8::from(cr2.{}());\nlet value = (cr2.0, !cold || {:?}.contains(&source));", if l083 { "source" } else { "sorce" }, lsi.uart_allowed_sources));
        writeln!(out, "gates[{}] = original_gate; resets[{}] = original_reset;\nif !uart.1 {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nuart_cr2[{i}] = uart.0;", i + 2, i + 2).unwrap();
    }
    if l083 {
        out.push_str("let mut gpio_filter = [0; 6];\nlet mut gpio_banks = [RccL052LsiPadBank::EMPTY; 6];\nlet gpioc_afrl;\n");
        for (label, routes) in [("lsi_routes", &lsi_routes), ("pll_routes", &pll_routes), ("unbonded_lsi_routes", &unbonded_lsi), ("unbonded_pll_routes", &unbonded_pll)] {
            let mutable = if routes.is_empty() { "" } else { "mut " };
            writeln!(out, "let {mutable}{label} = [0; {}];", routes.len()).unwrap();
        }
        for (i, name) in lsi.gpio_banks.iter().enumerate() {
            let mutable = if [&lsi_routes, &pll_routes, &unbonded_lsi, &unbonded_pll]
                .into_iter().flatten().any(|(pin, _)| pin.as_bytes()[1] == name.as_bytes()[4]) { "mut " } else { "" };
            let mut body = format!("let r = crate::pac::{name};\nlet filter = r.filter().read();\nlet source = filter.fltclk();\nlet {mutable}allowed = !cold || {:?}.contains(&source);\nlet afrl = r.afrl().read(); let afrh = r.afrh().read();\n", lsi.gpio_filter_allowed_sources);
            for (label, routes) in [("lsi_routes", &lsi_routes), ("pll_routes", &pll_routes), ("unbonded_lsi_routes", &unbonded_lsi), ("unbonded_pll_routes", &unbonded_pll)] {
                for (j, (pin, af)) in routes.iter().enumerate() {
                    if pin.as_bytes()[1] != name.as_bytes()[4] { continue; }
                    let bit: u8 = pin[2..].parse().unwrap(); let reg = if bit < 8 { "afrl" } else { "afrh" };
                    mask(name, reg, &format!("AFR{bit}"), u32::from(bit % 8) * 4, 4);
                    let predicate = match label {
                        "lsi_routes" => format!("{:?}.contains(&af) && (!cold || af != {af})", valid_afs(pin)),
                        "pll_routes" => format!("{:?}.contains(&af) && (!stop_pll || af != {af})", valid_afs(pin)),
                        "unbonded_lsi_routes" => "af == 0".into(),
                        "unbonded_pll_routes" => "!stop_pll || af == 0".into(),
                        _ => unreachable!(),
                    };
                    writeln!(body, "let af = {reg}.afr{bit}(); allowed &= {predicate};\n{label}[{j}] = {reg}.0;").unwrap();
                }
            }
            body.push_str("let bank = RccL052LsiPadBank { gate: original_gate, reset: original_reset,\n");
            for reg in ["lock", "dir", "pur", "pdr", "riseie", "fallie", "highie", "lowie", "opendrain", "filter", "afrl", "afrh", "analog"] {
                let accessor = if reg == "lock" { "lckr" } else { reg };
                register(name, accessor);
                writeln!(body, "{reg}: r.{accessor}().read().0{},", if reg == "lock" { format!(" & {}", !gpio_key_mask) } else { String::new() }).unwrap();
            }
            body.push_str("};\nlet value = (filter.0, afrl.0, afrh.0, bank, allowed && filter.0 == bank.filter && afrl.0 == bank.afrl && afrh.0 == bank.afrh);");
            window(out, name, "gpio", &body);
            writeln!(out, "gates[{}] = original_gate; resets[{}] = original_reset;\nif !gpio.4 {{ return Err(crate::rcc::Error::LsiClockInUse); }}\ngpio_filter[{i}] = gpio.0; gpio_banks[{i}] = gpio.3;", i + gpio_start, i + gpio_start).unwrap();
            if *name == "GPIOC" { out.push_str("gpioc_afrl = gpio.1;\n"); }
        }
    } else {
    out.push_str("let mut gpio_filter = [0; 5]; let gpioc_afrl;\n");
    for (i, name) in lsi.gpio_banks.iter().enumerate() {
        let direct = if *name == "GPIOC" {
            format!("let afrl = crate::pac::GPIOC.afrl().read();\nlet af = afrl.afr4();\nlet allowed_af = {:?}.contains(&af) || (!cold && af == 6);\nlet value = (filter.0, afrl.0, allowed && allowed_af);", lsi.lsi_output_allowed_af)
        } else { "let value = (filter.0, 0, allowed);".into() };
        window(out, name, "gpio", &format!("let filter = crate::pac::{name}.filter().read();\nlet source = filter.fltclk();\nlet allowed = !cold || {:?}.contains(&source);\n{direct}", lsi.gpio_filter_allowed_sources));
        writeln!(out, "gates[{}] = original_gate; resets[{}] = original_reset;\nif !gpio.2 {{ return Err(crate::rcc::Error::LsiClockInUse); }}\ngpio_filter[{i}] = gpio.0;", i + 5, i + 5).unwrap();
        if *name == "GPIOC" { out.push_str("gpioc_afrl = gpio.1;\n"); }
    }
    }
    for (i, name, body) in [
        (work_start, "LPTIM", "let cr = crate::pac::LPTIM.cr().read(); let cfgr = crate::pac::LPTIM.cfgr().read();\nlet allowed = !cold || (!cr.en() && cr.arst() && cr.srst());\n(Some((cr.0, cfgr.0)), allowed)".to_string()),
        (work_start + 1, "LCD", format!("let cr0 = crate::pac::LCD.cr0().read(); let cr1 = crate::pac::LCD.cr1().read();\nlet allowed = !cold || (!cr0.en() && !cr0.bump());\n(Some((cr0.0, cr1.0 & {lcd_mask})), allowed)")),
    ] {
        writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nlet gate = info.is_enabled(); let reset = info.reset_asserted();\nlet buffered = if !gate {{ (None, true) }} else if reset {{ (None, false) }} else {{ {body} }};\nlet gate_changed = info.is_enabled() != gate; let reset_changed = info.reset_asserted() != reset;").unwrap();
        out.push_str(faults);
        writeln!(out, "if gate_changed || reset_changed || (gate && reset) || !buffered.1 {{ return Err(crate::rcc::Error::LsiClockInUse); }}\ngates[{i}] = gate; resets[{i}] = reset;\nlet {} = buffered.0;", name.to_ascii_lowercase()).unwrap();
    }
    out.push_str("let mco = crate::pac::SYSCTRL.mco().read();\nlet source = mco.source();\n");
    out.push_str(faults);
    writeln!(out, "if !({:?}.contains(&source) || (!cold && source == 4)) {{ return Err(crate::rcc::Error::LsiClockInUse); }}", lsi.mco_allowed_sources).unwrap();
    if l083 { out.push_str("if stop_pll && source == 7 { return Err(crate::rcc::Error::LsiClockInUse); }\n"); }
    out.push_str("let lvd_cr0 = crate::pac::LVD.cr0().read().0; let lvd_cr1 = crate::pac::LVD.cr1().read().0;\n");
    out.push_str(faults);
    for (i, name) in gate_names.iter().enumerate() {
        writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nlet gate_changed = info.is_enabled() != gates[{i}];\nlet reset_changed = info.reset_asserted() != resets[{i}];").unwrap();
        if i < work_start { out.push_str("if gate_changed { return Err(crate::rcc::Error::LsiGateRestoreTimeout); }\n"); }
        out.push_str(faults);
        out.push_str("if gate_changed || reset_changed { return Err(crate::rcc::Error::LsiClockInUse); }\n");
    }
    if l083 { out.push_str("Ok(RccL052LsiConsumers { gpio_banks, lsi_routes, pll_routes, unbonded_lsi_routes, unbonded_pll_routes, rtc_cr0: rtc.0.0, rtc_cr1: rtc.0.1, rtc_cr2: rtc.0.2, rtc_compen: rtc.0.3, autotrim_cr: autotrim.0, lvd_cr0, lvd_cr1, uart_cr2, gpio_filter, gpioc_afrl, mco: mco.0, gates, resets, lptim, lcd })\n}\n"); } else {
    out.push_str("Ok(RccL052LsiConsumers { rtc_cr0: rtc.0.0, rtc_cr1: rtc.0.1, rtc_cr2: rtc.0.2, rtc_compen: rtc.0.3, autotrim_cr: autotrim.0, lvd_cr0, lvd_cr1, uart_cr2, gpio_filter, gpioc_afrl, mco: mco.0, gates, resets, lptim, lcd })\n}\n");

    }
    let sysctrl = peripheral("SYSCTRL");
    let pins: Vec<_> = ["HSE_IN", "HSE_OUT", "LSE_IN", "LSE_OUT"].into_iter().map(|signal| {
        let pins: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == signal && p.af.is_none()).collect();
        assert_eq!(pins.len(), 1);
        let pin = pins[0].pin;
        (pin.as_bytes()[1] - b'A', pin[2..].parse::<u8>().unwrap())
    }).collect();
    assert_eq!(pins, [(5, 0), (5, 1), (2, 14), (2, 15)]);
    let hse_bank = format!("GPIO{}", char::from(b'A' + pins[0].0));
    let lse_bank = format!("GPIO{}", char::from(b'A' + pins[2].0));
    if !l083 { assert_eq!(peripheral(&hse_bank).gpio.as_ref().unwrap().pull_down_mask, 243); }
    let pad_fields = ["lock", "dir", "pur", "pdr", "riseie", "fallie", "highie", "lowie", "opendrain", "filter", "afrl", "afrh", "analog"];
    for (port, bit) in &pins {
        let name = format!("GPIO{}", char::from(b'A' + port));
        for reg in pad_fields {
            let reg = if l083 && reg == "lock" { "lckr" } else { reg };
            register(&name, reg);
            if !matches!(reg, "afrl" | "afrh") { mask(&name, reg, &format!("PIN{bit}"), u32::from(*bit), 1); }
        }
        assert_ne!(peripheral(&name).gpio.as_ref().unwrap().pull_down_mask & (1 << bit), 0);
        mask(&name, if *bit < 8 { "AFRL" } else { "AFRH" }, &format!("AFR{bit}"), u32::from(bit % 8) * 4, 4);
    }
    out.push_str(r#"
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RccL052LsiPadBank {
    pub(crate) gate: bool, pub(crate) reset: bool,
    pub(crate) lock: u32, pub(crate) dir: u32, pub(crate) pur: u32, pub(crate) pdr: u32,
    pub(crate) riseie: u32, pub(crate) fallie: u32, pub(crate) highie: u32, pub(crate) lowie: u32,
    pub(crate) opendrain: u32, pub(crate) filter: u32, pub(crate) afrl: u32, pub(crate) afrh: u32, pub(crate) analog: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RccL052LsiPads { pub(crate) hse: RccL052LsiPadBank, pub(crate) lse: RccL052LsiPadBank }
pub(crate) fn rcc_l052_lsi_pads(timeout: u32, cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<RccL052LsiPads, crate::rcc::Error> {
use crate::rcc::SealedRccPeripheral;
"#);
    for (name, local) in [(&hse_bank, "hse"), (&lse_bank, "lse")] {
        let mut body = format!("let r = crate::pac::{name};\nlet value = RccL052LsiPadBank {{ gate: original_gate, reset: original_reset,\n");
        for reg in pad_fields {
            let accessor = if l083 && reg == "lock" { "lckr" } else { reg };
            writeln!(body, "{reg}: r.{accessor}().read().0{},", if reg == "lock" { format!(" & {}", !gpio_key_mask) } else { String::new() }).unwrap();
        }
        body.push_str("};");
        window(out, name, local, &body);
    }
    if l083 {
        for (name, local) in [(&hse_bank, "hse"), (&lse_bank, "lse")] {
            writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nif info.is_enabled() != {local}.gate {{ return Err(crate::rcc::Error::LsiGateRestoreTimeout); }}").unwrap();
            out.push_str(faults);
            writeln!(out, "if info.reset_asserted() != {local}.reset {{ return Err(crate::rcc::Error::LsiClockInUse); }}").unwrap();
        }
        // Full bank records also cover direct outputs outside oscillator banks.
        out.push_str("Ok(RccL052LsiPads { hse, lse })\n}\nimpl RccL052LsiPadBank {\nconst EMPTY: Self = Self { gate: false, reset: false, lock: 0, dir: 0, pur: 0, pdr: 0, riseie: 0, fallie: 0, highie: 0, lowie: 0, opendrain: 0, filter: 0, afrl: 0, afrh: 0, analog: 0 };\n}\nimpl RccL052LsiPads {\npub(crate) fn hse_gate_enabled(mut self) -> Self { self.hse.gate = true; self }\n");
    } else {
    out.push_str("Ok(RccL052LsiPads { hse, lse })\n}\nimpl RccL052LsiPads {\npub(crate) fn hse_gate_enabled(mut self) -> Self { self.hse.gate = true; self }\n");
    }
    let hse_input = mask(&hse_bank, "DIR", &format!("PIN{}", pins[0].1), u32::from(pins[0].1), 1);
    let hse_output = mask(&hse_bank, "DIR", &format!("PIN{}", pins[1].1), u32::from(pins[1].1), 1);
    let lse_input = mask(&lse_bank, "DIR", &format!("PIN{}", pins[2].1), u32::from(pins[2].1), 1);
    let lse_output = mask(&lse_bank, "DIR", &format!("PIN{}", pins[3].1), u32::from(pins[3].1), 1);
    let hse_af_input = mask(&hse_bank, "AFRL", &format!("AFR{}", pins[0].1), u32::from(pins[0].1) * 4, 4);
    let hse_af_output = mask(&hse_bank, "AFRL", &format!("AFR{}", pins[1].1), u32::from(pins[1].1) * 4, 4);
    if !l083 {
    writeln!(out, "pub(crate) fn hse_configured(mut self, bypass: bool) -> Self {{\nlet mask = if bypass {{ {hse_input} }} else {{ {} }};\nself.hse.gate = true; self.hse.dir |= mask;", hse_input | hse_output).unwrap();
    for reg in ["lock", "pur", "pdr", "riseie", "fallie", "highie", "lowie", "opendrain", "filter"] {
        writeln!(out, "self.hse.{reg} &= !mask;").unwrap();
    }
    writeln!(out, "self.hse.afrl &= !(if bypass {{ {hse_af_input} }} else {{ {} }});\nself.hse.analog = (self.hse.analog & !mask) | if bypass {{ 0 }} else {{ mask }};\nself\n}}", hse_af_input | hse_af_output).unwrap();
    }
    writeln!(out, "pub(crate) fn lse_configured(mut self, bypass: bool) -> Self {{ if bypass {{ self.lse.analog &= !{lse_input}; }} self }}").unwrap();
    for (source, input, output, afinput, afoutput) in [
        ("hse", hse_input, hse_output, hse_af_input, hse_af_output),
        ("lse", lse_input, lse_output, mask(&lse_bank, "AFRH", "AFR14", 24, 4), mask(&lse_bank, "AFRH", "AFR15", 28, 4)),
    ] {
        let argument = if source == "lse" { ", unused: bool" } else { "" };
        let analog = if source == "lse" { "if unused || !bypass { mask } else { 0 }" } else { "if bypass { 0 } else { mask }" };
        writeln!(out, "pub(crate) fn {source}_matches(self, bypass: bool{argument}) -> bool {{\nlet r = self.{source}; let mask = if bypass {{ {input} }} else {{ {} }};\n!r.reset && r.dir & mask == mask && r.analog & mask == {analog}\n&& r.{} & (if bypass {{ {afinput} }} else {{ {} }}) == 0", input | output, if source == "hse" { "afrl" } else { "afrh" }, afinput | afoutput).unwrap();
        for reg in ["lock", "pur", "pdr", "riseie", "fallie", "highie", "lowie", "opendrain", "filter"] { writeln!(out, "&& r.{reg} & mask == 0").unwrap(); }
        out.push_str("\n}\n");
    }
    if l083 {
        out.push_str("pub(crate) fn hse_startup_allowed(self, bypass: bool) -> bool {\n!self.hse.reset\n");
        for route in lse.output_routes {
            let port = route.pin.as_bytes()[1] - b'A';
            let bit: u8 = route.pin[2..].parse().unwrap();
            for (i, pad) in pins[..2].iter().enumerate() {
                if (port, bit) != *pad { continue; }
                let reg = if bit < 8 { "afrl" } else { "afrh" };
                let position = u32::from(bit % 8) * 4;
                let bits = mask(&hse_bank, reg, &format!("AFR{bit}"), position, 4);
                let selected = u32::from(route.af) << position;
                writeln!(out, "&& ({}self.hse.{reg} & {bits} != {selected})", if i == 1 { "bypass || " } else { "" }).unwrap();
            }
        }
        out.push_str("\n}\n}\n");
        // L083 applies exactly one native pad-register delta per acknowledged
        // phase. Arrays contain whole words, so every overlapping route moves
        // with its shared bank's exact expected word.
        writeln!(out, "impl RccL052LsiPadBank {{\npub(crate) fn hse_configured_step(mut self, bypass: bool, step: u8) -> Self {{\nassert!(step < if bypass {{ 12 }} else {{ 24 }});\nlet (mask, af_mask) = if step < 12 {{ ({hse_input}, {hse_af_input}) }} else {{ ({hse_output}, {hse_af_output}) }};").unwrap();
        out.push_str("match step % 12 {\n0 => self.lock &= !mask, 1 => self.dir |= mask, 2 => self.pur &= !mask, 3 => self.pdr &= !mask,\n4 => self.riseie &= !mask, 5 => self.fallie &= !mask, 6 => self.highie &= !mask, 7 => self.lowie &= !mask,\n8 => self.opendrain &= !mask, 9 => self.filter &= !mask, 10 => self.afrl &= !af_mask,\n11 => self.analog = (self.analog & !mask) | if bypass { 0 } else { mask }, _ => unreachable!(),\n}\nself\n}\n}\nimpl RccL052LsiPads {\npub(crate) fn hse_configured_step(mut self, bypass: bool, step: u8) -> Self { self.hse = self.hse.hse_configured_step(bypass, step); self }\n}\nimpl RccL052LsiConsumers {\n");
        writeln!(out, "pub(crate) fn hse_gate_enabled(mut self) -> Self {{ self.gates[{}] = true; self.gpio_banks[{hse_bank_index}].gate = true; self }}", gpio_start + hse_bank_index).unwrap();
        writeln!(out, "pub(crate) fn hse_configured_step(mut self, bypass: bool, step: u8) -> Self {{\nself.gpio_banks[{hse_bank_index}] = self.gpio_banks[{hse_bank_index}].hse_configured_step(bypass, step);\nself.gpio_filter[{hse_bank_index}] = self.gpio_banks[{hse_bank_index}].filter;").unwrap();
        for (label, routes) in [("lsi_routes", &lsi_routes), ("pll_routes", &pll_routes), ("unbonded_lsi_routes", &unbonded_lsi), ("unbonded_pll_routes", &unbonded_pll)] {
            for (i, (pin, _)) in routes.iter().enumerate() {
                if pin.as_bytes()[1] != b'F' { continue; }
                let bit: u8 = pin[2..].parse().unwrap(); let reg = if bit < 8 { "afrl" } else { "afrh" };
                writeln!(out, "self.{label}[{i}] = self.gpio_banks[{hse_bank_index}].{reg};").unwrap();
            }
        }
        out.push_str("self\n}\n");
        writeln!(out, "pub(crate) fn lse_configured(mut self, bypass: bool) -> Self {{ if bypass {{ self.gpio_banks[{lse_bank_index}].analog &= !{lse_input}; }} self }}\n}}").unwrap();
    } else {
    out.push_str("}\nimpl RccL052LsiConsumers {\npub(crate) fn hse_gate_enabled(mut self) -> Self { self.gates[9] = true; self }\n");
    writeln!(out, "pub(crate) fn hse_configured(mut self, bypass: bool) -> Self {{ self.gates[9] = true; self.gpio_filter[4] &= !(if bypass {{ {hse_input} }} else {{ {} }}); self }}\n}}", hse_input | hse_output).unwrap();

    }
    // HSE owns this permanent gate enable. It is deliberately not restored;
    // every read-only pad window above still uses central temporary ownership.
    out.push_str("pub(crate) fn rcc_l052_lsi_enable_hse_pins(timeout: u32, cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<(), crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\n");
    writeln!(out, "let info = crate::peripherals::{hse_bank}::RCC_INFO;").unwrap();
    out.push_str(faults);
    out.push_str("if info.reset_asserted() { return Err(crate::rcc::Error::LsiClockInUse); }\ninfo.enable_with_cs_readback(cs, crate::rcc::Readback::Poll { attempts: timeout, spin: true }).map_err(|_| crate::rcc::Error::LsiGateEnableTimeout)?;\n");
    out.push_str(faults);
    out.push_str("if !info.is_enabled() || info.reset_asserted() { return Err(crate::rcc::Error::LsiClockInUse); }\n");
    out.push_str("Ok(())\n}\n");
    if l083 {
        out.push_str("pub(crate) fn rcc_l052_lsi_configure_hse_pin_step(bypass: bool, step: u8, _timeout: u32, _cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<(), crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\nif step >= if bypass { 12 } else { 24 } { return Err(crate::rcc::Error::LsiClockInUse); }\nlet info = crate::peripherals::GPIOF::RCC_INFO;\n");
        out.push_str(faults);
        out.push_str("if !info.is_enabled() || info.reset_asserted() { return Err(crate::rcc::Error::LsiClockInUse); }\nlet r = crate::pac::GPIOF;\nmatch step {\n");
        for (i, (_, bit)) in pins[..2].iter().enumerate() {
            for (j, reg) in pad_fields.iter().filter(|r| **r != "afrh").enumerate() {
                let reg = if *reg == "lock" { "lckr" } else { *reg };
                let field = if reg == "afrl" { format!("afr{bit}") } else { format!("pin{bit}") };
                let value = match reg { "dir" => "true", "afrl" => "0", "analog" if i == 0 => "!bypass", "analog" => "true", _ => "false" };
                let normalize = if reg == "lckr" { !gpio_key_mask } else { u32::MAX };
                let keyed = if reg == "lckr" { "target.set_key(0x5a5a);" } else { "" };
                writeln!(out, "{} => {{ let before = r.{reg}().read(); let mut target = before; {keyed} target.set_{field}({value});\nlet old = before.0 & {normalize}; let expected = target.0 & {normalize};\nr.{reg}().write(|w| *w = target);\nlet observed = r.{reg}().read().0 & {normalize};\nlet gate_or_reset_changed = !info.is_enabled() || info.reset_asserted();", i * 12 + j).unwrap();
                out.push_str(faults);
                out.push_str("if gate_or_reset_changed || (observed != old && observed != expected) { return Err(crate::rcc::Error::LsiClockInUse); }\n},\n");
            }
        }
        out.push_str("_ => return Err(crate::rcc::Error::LsiClockInUse),\n}\nOk(())\n}\n");
    } else {
    out.push_str("pub(crate) fn rcc_l052_lsi_configure_hse_pins(bypass: bool, _timeout: u32, _cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<(), crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\n");
    writeln!(out, "let info = crate::peripherals::{hse_bank}::RCC_INFO;").unwrap();
    out.push_str(faults);
    out.push_str("if !info.is_enabled() || info.reset_asserted() { return Err(crate::rcc::Error::LsiClockInUse); }\n");
    writeln!(out, "let r = crate::pac::{hse_bank};").unwrap();
    for (i, (_, bit)) in pins[..2].iter().enumerate() {
        if i == 1 { out.push_str("if !bypass {\n"); }
        writeln!(out, "r.lock().modify(|w| {{ w.set_key(0x5a5a); w.set_pin{bit}(false); }});\nr.dir().modify(|w| w.set_pin{bit}(true));").unwrap();
        for reg in ["pur", "pdr", "riseie", "fallie", "highie", "lowie", "opendrain", "filter"] { writeln!(out, "r.{reg}().modify(|w| w.set_pin{bit}(false));").unwrap(); }
        writeln!(out, "r.afrl().modify(|w| w.set_afr{bit}(0));\nr.analog().modify(|w| w.set_pin{bit}({}));", if i == 0 { "!bypass" } else { "true" }).unwrap();
        if i == 1 { out.push_str("}\n"); }
    }
    out.push_str(faults);
    out.push_str("if !info.is_enabled() || info.reset_asserted() { return Err(crate::rcc::Error::LsiClockInUse); }\nOk(())\n}\n");
    }
    out.push_str("pub(crate) fn rcc_l052_lsi_configure_lse_pins(bypass: bool, timeout: u32, cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<(), crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\nif bypass {\n");
    window(out, &lse_bank, "()", &format!("crate::pac::{lse_bank}.analog().modify(|w| w.set_pin{}(false));\nlet value = ();", pins[2].1));
    out.push_str("}\nOk(())\n}\n");

    let lse_header = r#"
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RccL052LsiLseSnapshot {
    consumers: RccL052LsiConsumers, pads: RccL052LsiPads,
    rtc_reset: [u32; 13], routes: [u32; 2], route_gates: [bool; 2], route_resets: [bool; 2],
}
pub(crate) fn rcc_l052_lsi_lse_preflight(bypass: bool, unused: bool, timeout: u32, cs: critical_section::CriticalSection<'_>, monitor_hse: bool, monitor_lse: bool) -> Result<RccL052LsiLseSnapshot, crate::rcc::Error> {
use crate::rcc::SealedRccPeripheral;
let consumers = rcc_l052_lsi_consumers(timeout, cs, false, monitor_hse, monitor_lse)?;
let pads = rcc_l052_lsi_pads(timeout, cs, monitor_hse, monitor_lse)?;
"#;
    if l083 { out.push_str(&lse_header.replace("unused: bool, timeout", "unused: bool, stop_pll: bool, timeout")
        .replace("timeout, cs, false, monitor_hse", "timeout, cs, false, stop_pll, monitor_hse")); }
    else { out.push_str(lse_header); }
    out.push_str(faults);
    out.push_str("if !pads.lse_matches(bypass, unused) { return Err(crate::rcc::Error::LsePinConflict); }\n");
    let order = ["CR0", "CR1", "CR2", "IER", "ISR", "COMPEN", "DATE", "TIME", "ALARMA", "ALARMB", "TAMPDATE", "TAMPTIME", "AWTARR"];
    assert_eq!(lse.rtc_reset.len(), order.len());
    let mut reset_body = "let snapshot = || {\nlet cr0 = crate::pac::RTC.cr0().read().0;\nlet cr1 = crate::pac::RTC.cr1().read().0;\n".to_string();
    let reset: Vec<_> = order.iter().map(|name| {
        let records: Vec<_> = lse.rtc_reset.iter().filter(|r| r.register == *name).collect();
        assert_eq!(records.len(), 1); register("RTC", name); records[0]
    }).collect();
    writeln!(reset_body, "if cr0 & {} != {} || cr1 & {} != {} {{ return None; }}\nSome([", reset[0].mask, reset[0].value & reset[0].mask, reset[1].mask, reset[1].value & reset[1].mask).unwrap();
    for (i, r) in reset.iter().enumerate() {
        let read = if i == 0 { "cr0".into() } else if i == 1 { "cr1".into() } else { format!("crate::pac::RTC.{}().read().0", r.register.to_ascii_lowercase()) };
        writeln!(reset_body, "{read} & {},", r.mask).unwrap();
    }
    reset_body.push_str("] )\n};\nlet first = snapshot(); let second = snapshot();\nlet value = (first, second);");
    window(out, "RTC", "rtc", &reset_body);
    let values: Vec<_> = reset.iter().map(|r| r.value & r.mask).collect();
    writeln!(out, "let rtc_reset = rtc.0.ok_or(crate::rcc::Error::LseClockInUse)?;\nif rtc.1 != Some(rtc_reset) || rtc_reset != {values:?} {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
    out.push_str("let auto = crate::pac::autotrim::regs::Cr(consumers.autotrim_cr);\n");
    writeln!(out, "if u8::from(auto.src()) == {} {{ return Err(crate::rcc::Error::LseClockInUse); }}", native.autotrim_source).unwrap();
    writeln!(out, "if let Some((cr, cfgr)) = consumers.lptim {{ if crate::pac::lptim::regs::Cr(cr).en() && u8::from(crate::pac::lptim::regs::Cfgr(cfgr).iclksrc()) == {} {{ return Err(crate::rcc::Error::LseClockInUse); }} }}", native.lptim.source).unwrap();
    out.push_str("if let Some((cr, _)) = consumers.lcd { let cr = crate::pac::lcd::regs::Cr0(cr); if cr.en() || cr.bump() { return Err(crate::rcc::Error::LseClockInUse); } }\n");
    writeln!(out, "if consumers.uart_cr2.iter().any(|cr| u8::from(crate::pac::uart::regs::Cr2(*cr).{}()) == {}) || crate::pac::sysctrl::regs::Mco(consumers.mco).source() == {} {{ return Err(crate::rcc::Error::LseClockInUse); }}", if l083 { "source" } else { "sorce" }, lse.uart_source, lse.mco_source).unwrap();
    assert_eq!(lse.output_routes.len(), 2);
    out.push_str("let mut routes = [0; 2]; let mut route_gates = [false; 2]; let mut route_resets = [false; 2];\n");
    for (i, route) in lse.output_routes.iter().enumerate() {
        let name = format!("GPIO{}", char::from(route.pin.as_bytes()[1]));
        let bit: u8 = route.pin[2..].parse().unwrap();
        let reg = if bit < 8 { "afrl" } else { "afrh" };
        mask(&name, reg, &format!("AFR{bit}"), u32::from(bit % 8) * 4, 4);
        window(out, &name, "route", &format!("let af = crate::pac::{name}.{reg}().read(); let value = (af.0, af.afr{bit}() != {});", route.af));
        writeln!(out, "if !route.1 {{ return Err(crate::rcc::Error::LseClockInUse); }}\nroutes[{i}] = route.0; route_gates[{i}] = original_gate; route_resets[{i}] = original_reset;").unwrap();
    }
    if l083 {
        for (i, route) in lse.output_routes.iter().enumerate() {
            let name = format!("GPIO{}", char::from(route.pin.as_bytes()[1]));
            writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nif info.is_enabled() != route_gates[{i}] {{ return Err(crate::rcc::Error::LsiGateRestoreTimeout); }}").unwrap();
            out.push_str(faults);
            writeln!(out, "if info.reset_asserted() != route_resets[{i}] {{ return Err(crate::rcc::Error::LsiClockInUse); }}").unwrap();
        }
    }
    out.push_str(faults);
    out.push_str("Ok(RccL052LsiLseSnapshot { consumers, pads, rtc_reset, routes, route_gates, route_resets })\n}\nimpl RccL052LsiLseSnapshot {\n");
    if l083 { out.push_str("pub(crate) fn hse_configured_step(mut self, bypass: bool, step: u8) -> Self { self.consumers = self.consumers.hse_configured_step(bypass, step); self.pads = self.pads.hse_configured_step(bypass, step);\n"); }
    else { out.push_str("pub(crate) fn hse_configured(mut self, bypass: bool) -> Self { self.consumers = self.consumers.hse_configured(bypass); self.pads = self.pads.hse_configured(bypass);\n"); }
    for (i, route) in lse.output_routes.iter().enumerate() {
        if route.pin.as_bytes()[1] - b'A' == pins[0].0 {
            let bit: u8 = route.pin[2..].parse().unwrap();
            writeln!(out, "self.routes[{i}] = self.pads.hse.{}; self.route_gates[{i}] = true;", if bit < 8 { "afrl" } else { "afrh" }).unwrap();
        }
    }
    out.push_str("self\n}\npub(crate) fn hse_gate_enabled(mut self) -> Self { self.consumers = self.consumers.hse_gate_enabled(); self.pads = self.pads.hse_gate_enabled();\n");
    for (i, route) in lse.output_routes.iter().enumerate() {
        if route.pin.as_bytes()[1] - b'A' == pins[0].0 {
            writeln!(out, "self.route_gates[{i}] = true;").unwrap();
        }
    }
    if l083 { out.push_str("self\n}\npub(crate) fn lse_configured(mut self, bypass: bool) -> Self { self.pads = self.pads.lse_configured(bypass); self.consumers = self.consumers.lse_configured(bypass); self }\n}\n"); }
    else { out.push_str("self\n}\npub(crate) fn lse_configured(mut self, bypass: bool) -> Self { self.pads = self.pads.lse_configured(bypass); self }\n}\n"); }
    if l083 { output.push_str(&generated.replace("L052", "L083").replace("l052", "l083")); }
    else { output.push_str(&generated); }
}

// A native live-only capability is distinct from the old cold-consumer profiles.
// Validate the complete transported record wherever it affects generated code.
fn l010_live_lsi_qualified(c: &cw32_metapac::metadata::PeripheralClockLimits) -> bool {
    use cw32_metapac::metadata::{METADATA, PeripheralLsiSysclk};
    let sysctrl = METADATA.peripherals.iter().find(|p| p.name == "SYSCTRL").unwrap();
    let native = sysctrl.registers.as_ref().unwrap().version == "cw32l010_v1";
    let exact = matches!(METADATA.name, "CW32L010F8P6" | "CW32L010F8U6" | "CW32L010Y8M6");
    if native || METADATA.line == "CW32L010" || exact {
        assert!(native && METADATA.line == "CW32L010");
        assert_eq!(c.lsi_sysclk.is_some(), exact, "missing or unexpected native live-LSI profile");
        if exact {
            assert_eq!(c.lsi_sysclk.as_ref().unwrap(), &PeripheralLsiSysclk {
                nominal_hz: 32_800, minimum_hz: 31_816, maximum_hz: 33_784,
                supply_mv: (1620, 5500), temperature_c: (-40, 85),
                factory_trim_address: 0x0010_07c2,
                rtc_allowed_sources: &[0, 1, 2, 3], awt_allowed_sources: &[0, 1],
                uart_allowed_sources: &[0, 1, 2, 3], uarts: &["UART1", "UART2"],
                gpio_banks: &["GPIOA", "GPIOB"],
                gpio_filter_allowed_sources: &[0, 1, 2, 3, 5, 7],
                mco_allowed_sources: &[0, 1, 2, 3, 4, 5, 6],
                lsi_output_pin: None, lsi_output_allowed_af: &[], rcc_irq: 4,
            });
        }
    }
    native && exact
}

// Own L011 admission and projection. Its three-bank GPIO layout and 41 kHz
// endpoint are independent of L010/L012; generic L011 remains unqualified.
fn l011_live_lsi_qualified(c: &cw32_metapac::metadata::PeripheralClockLimits) -> bool {
    use cw32_metapac::metadata::{ir, METADATA, PeripheralLsiSysclk, PeripheralPin};
    let owners: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == "SYSCTRL").collect();
    assert_eq!(owners.len(), 1);
    let sysctrl = owners[0];
    let regs = sysctrl.registers.as_ref().unwrap();
    let native = regs.version == "cw32l011_v1";
    let exact = matches!(METADATA.name, "CW32L011K8T6" | "CW32L011K8U6");
    if !(native || METADATA.line == "CW32L011" || exact) { return false; }
    assert!(native && METADATA.line == "CW32L011");
    assert!(exact || METADATA.name == "CW32L011");
    assert_eq!(METADATA.peripherals.iter().filter(|p| p.clock_limits.is_some()).count(), 1);
    assert_eq!(sysctrl.clock_limits.as_ref().unwrap(), c);
    assert_eq!(c.lsi_sysclk.is_some(), exact, "missing or unexpected L011 live-LSI profile");
    assert!(c.pll.is_none() && c.hex.is_none());
    assert_eq!(sysctrl.pins, &[
        PeripheralPin { pin: "PB7", signal: "HSE_OUT", af: None, adc_mux: None, comparator_mux: None },
        PeripheralPin { pin: "PC13", signal: "HSE_IN", af: None, adc_mux: None, comparator_mux: None },
        PeripheralPin { pin: "PC14", signal: "LSE_IN", af: None, adc_mux: None, comparator_mux: None },
        PeripheralPin { pin: "PC15", signal: "LSE_OUT", af: None, adc_mux: None, comparator_mux: None },
    ]);
    assert_eq!(METADATA.pins.iter().map(|p| p.name).collect::<Vec<_>>(), [
        "PA0", "PA1", "PA10", "PA11", "PA12", "PA13", "PA14", "PA15", "PA2", "PA3", "PA4", "PA5", "PA6", "PA7", "PA8", "PA9",
        "PB0", "PB1", "PB3", "PB4", "PB5", "PB6", "PB7", "PC13", "PC14", "PC15",
    ]);
    let inventory = [
        ("ADC", 0x40000000, "adc", "cw32l011_v1", "Adc"),
        ("ATIM", 0x40001400, "atim", "cw32l010_v1", "Atim"),
        ("BTIM1", 0x40004800, "btim", "cw32l010_v1", "Btim"),
        ("BTIM2", 0x40004840, "btim", "cw32l010_v1", "Btim"),
        ("BTIM3", 0x40004880, "btim", "cw32l010_v1", "Btim"),
        ("CRC", 0x40023000, "crc", "cw32l010_v1", "Crc"),
        ("FLASH", 0x40022000, "flash", "cw32l011_v1", "Flash"),
        ("GPIOA", 0x48000000, "gpio", "cw32l011_v1", "Gpio"),
        ("GPIOB", 0x48000100, "gpio", "cw32l011_v1", "Gpio"),
        ("GPIOC", 0x48000200, "gpio", "cw32l011_v1", "Gpio"),
        ("GTIM1", 0x40001800, "gtim", "cw32l010_v1", "Gtim"),
        ("GTIM2", 0x40001c00, "gtim", "cw32l010_v1", "Gtim"),
        ("I2C", 0x40005800, "i2c", "cw32l011_v1", "I2c"),
        ("IRMOD", 0x40004080, "irmod", "cw32l011_v1", "Irmod"),
        ("IWDT", 0x40005000, "iwdt", "v1", "Iwdt"),
        ("LPTIM", 0x40006000, "lptim", "cw32l010_v1", "Lptim"),
        ("LVD", 0x400000d0, "lvd", "cw32l010_v1", "Lvd"),
        ("RAM", 0x40022400, "ram", "cw32l011_v1", "Ram"),
        ("RTC", 0x40004400, "rtc", "cw32l011_v1", "Rtc"),
        ("SPI", 0x40000800, "spi", "cw32l010_v1", "Spi"),
        ("SYSCTRL", 0x40004000, "sysctrl", "cw32l011_v1", "Sysctrl"),
        ("UART1", 0x40000c00, "uart", "cw32l010_v1", "Uart"),
        ("UART2", 0x40001000, "uart", "cw32l010_v1", "Uart"),
        ("UART3", 0x40002000, "uart", "cw32l010_v1", "Uart"),
        ("VC1", 0x400000a4, "vc", "cw32l011_v1", "Vc"),
        ("VC2", 0x400000b4, "vc", "cw32l011_v1", "Vc"),
        ("VCDIV", 0x400000a0, "vcdiv", "cw32l010_v1", "Vcdiv"),
    ];
    assert_eq!(METADATA.peripherals.len(), inventory.len());
    for (name, address, kind, version, block) in inventory {
        let matches: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == name).collect();
        assert_eq!(matches.len(), 1);
        let p = matches[0];
        let r = p.registers.as_ref().unwrap();
        assert_eq!(p.address, address);
        assert_eq!((r.kind, r.version, r.block), (kind, version, block));
    }
    // Exact readable fieldsets for every source/owner register used by this
    // target. Access checks bind the selected PAC register IR; existing
    // field-level STABLE overlays are separately source-checked, not repaired.
    let control = |owner: &str, register: &str, byte_offset, access: ir::Access, expected: &[(&str, u32, u32)]| {
        let p = METADATA.peripherals.iter().find(|p| p.name == owner).unwrap();
        let regs = p.registers.as_ref().unwrap();
        let blocks: Vec<_> = regs.ir.blocks.iter().filter(|b| b.name == regs.block).collect();
        assert_eq!(blocks.len(), 1);
        let items: Vec<_> = blocks[0].items.iter().filter(|r| r.name.eq_ignore_ascii_case(register)).collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].byte_offset, byte_offset);
        assert!(items[0].array.is_none());
        let ir::BlockItemInner::Register(r) = &items[0].inner else { panic!("L011 control must be a register") };
        assert_eq!(r.bit_size, 32);
        assert_eq!(r.access, access);
        assert_ne!(r.access, ir::Access::Write);
        let fieldsets: Vec<_> = regs.ir.fieldsets.iter().filter(|f| Some(f.name) == r.fieldset).collect();
        assert_eq!(fieldsets.len(), 1);
        let fs = fieldsets[0];
        assert_eq!(fs.bit_size, 32);
        assert_eq!(fs.fields.len(), expected.len());
        for &(name, bit_offset, width) in expected {
            let fields: Vec<_> = fs.fields.iter().filter(|f| f.name.eq_ignore_ascii_case(name)).collect();
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].bit_size, width);
            assert!(fields[0].array.is_none());
            assert!(matches!(&fields[0].bit_offset, ir::BitOffset::Regular(p) if p.offset == bit_offset));
        }
    };
    control("SYSCTRL", "CR0", 0, ir::Access::ReadWrite, &[("SYSCLK", 0, 3), ("PCLKPRS", 3, 2), ("HCLKPRS", 5, 3), ("KEY", 16, 16)]);
    control("SYSCTRL", "CR1", 4, ir::Access::ReadWrite, &[("HSIEN", 0, 1), ("HSEEN", 1, 1), ("LSIEN", 3, 1), ("LSEEN", 4, 1), ("LSELOCK", 5, 1), ("LSECCS", 6, 1), ("HSECCS", 7, 1), ("CLKCCS", 8, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "CR2", 8, ir::Access::ReadWrite, &[("SWDIO", 1, 1), ("LOCKUP", 2, 1), ("WAKEUPCLK", 3, 1), ("FLASHWAIT", 4, 3), ("RTCLPM", 7, 1), ("HSEBRKEN", 10, 1), ("LSEBRKEN", 11, 1), ("CLLBRKEN", 12, 1), ("DSBRKEN", 13, 1), ("LVDBRKEN", 14, 1), ("RAMBRKEN", 15, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "IER", 12, ir::Access::ReadWrite, &[("HSIRDY", 0, 1), ("HSERDY", 1, 1), ("LSIRDY", 3, 1), ("LSERDY", 4, 1), ("LSEFAIL", 5, 1), ("HSEFAIL", 6, 1), ("LSEFAULT", 7, 1), ("HSEFAULT", 8, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "ISR", 16, ir::Access::Read, &[("HSIRDY", 0, 1), ("HSERDY", 1, 1), ("LSIRDY", 3, 1), ("LSERDY", 4, 1), ("LSEFAIL", 5, 1), ("HSEFAIL", 6, 1), ("LSEFAULT", 7, 1), ("HSEFAULT", 8, 1), ("HSISTABLE", 11, 1), ("HSESTABLE", 12, 1), ("LSISTABLE", 14, 1), ("LSESTABLE", 15, 1)]);
    control("SYSCTRL", "ICR", 20, ir::Access::ReadWrite, &[("HSIRDY", 0, 1), ("HSERDY", 1, 1), ("LSIRDY", 3, 1), ("LSERDY", 4, 1), ("LSEFAIL", 5, 1), ("HSEFAIL", 6, 1), ("LSEFAULT", 7, 1), ("HSEFAULT", 8, 1)]);
    control("SYSCTRL", "HSI", 24, ir::Access::ReadWrite, &[("TRIM", 0, 11), ("DIV", 11, 4), ("STABLE", 15, 1)]);
    control("SYSCTRL", "HSE", 28, ir::Access::ReadWrite, &[("DRIVER", 0, 4), ("WAITCYCLE", 4, 2), ("MODE", 6, 1), ("HEXENPOL", 7, 1), ("DETCNT", 8, 11), ("STABLE", 19, 1), ("PDRIVER", 20, 4), ("DIGFLT", 24, 1)]);
    control("SYSCTRL", "LSI", 32, ir::Access::ReadWrite, &[("TRIM", 0, 10), ("WAITCYCLE", 10, 2), ("STABLE", 15, 1)]);
    control("SYSCTRL", "LSE", 36, ir::Access::ReadWrite, &[("DRIVER", 0, 4), ("WAITCYCLE", 4, 2), ("MODE", 6, 1), ("PDRIVER", 8, 4), ("PINLOCK", 17, 1), ("STABLE", 18, 1)]);
    control("SYSCTRL", "MCO", 112, ir::Access::ReadWrite, &[("SOURCE", 0, 4), ("DIV", 4, 3)]);
    control("SYSCTRL", "AHBEN", 48, ir::Access::ReadWrite, &[("FLASH", 1, 1), ("CRC", 2, 1), ("GPIOA", 4, 1), ("GPIOB", 5, 1), ("GPIOC", 6, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "APBEN1", 56, ir::Access::ReadWrite, &[("ADC", 0, 1), ("VC", 1, 1), ("SPI1", 2, 1), ("UART1", 3, 1), ("UART2", 4, 1), ("ATIM", 5, 1), ("GTIM1", 6, 1), ("GTIM2", 7, 1), ("UART3", 8, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "APBEN2", 52, ir::Access::ReadWrite, &[("RTC", 1, 1), ("BTIM123", 2, 1), ("IWDT", 4, 1), ("I2C1", 6, 1), ("LPTIM", 7, 1), ("KEY", 16, 16)]);
    control("SYSCTRL", "AHBRST", 64, ir::Access::ReadWrite, &[("FLASH", 1, 1), ("CRC", 2, 1), ("GPIOA", 4, 1), ("GPIOB", 5, 1), ("GPIOC", 6, 1)]);
    control("SYSCTRL", "APBRST1", 72, ir::Access::ReadWrite, &[("ADC", 0, 1), ("VC", 1, 1), ("SPI1", 2, 1), ("UART1", 3, 1), ("UART2", 4, 1), ("ATIM", 5, 1), ("GTIM1", 6, 1), ("GTIM2", 7, 1), ("UART3", 8, 1)]);
    control("SYSCTRL", "APBRST2", 68, ir::Access::ReadWrite, &[("RTC", 1, 1), ("BTIM123", 2, 1), ("IWDT", 4, 1), ("I2C1", 6, 1), ("LPTIM", 7, 1)]);
    control("FLASH", "CR2", 4, ir::Access::ReadWrite, &[("WAIT", 0, 3), ("KEY", 16, 16)]);
    control("RTC", "CR0", 4, ir::Access::ReadWrite, &[("INTERVAL", 0, 3), ("H24", 3, 1), ("RTC1HZ", 5, 2), ("START", 7, 1)]);
    control("RTC", "CR1", 8, ir::Access::ReadWrite, &[("WAIT", 2, 1), ("SOURCE", 8, 3)]);
    control("RTC", "CR2", 12, ir::Access::ReadWrite, &[("AWTPRS", 0, 2), ("AWTSRC", 2, 1), ("TAMPEDGE", 3, 1), ("RTCOUT", 4, 2), ("TAMPEN", 6, 1), ("AWTEN", 7, 1), ("ALARMAEN", 9, 1), ("ALARMBEN", 10, 1)]);
    control("RTC", "IER", 48, ir::Access::ReadWrite, &[("ALARMA", 0, 1), ("ALARMB", 1, 1), ("AWTIMER", 2, 1), ("TAMP", 3, 1), ("TAMPOV", 4, 1), ("INTERVAL", 6, 1)]);
    control("RTC", "PSC", 64, ir::Access::ReadWrite, &[("PSC2", 0, 20), ("PSC1", 20, 8)]);
    control("ADC", "CR", 0, ir::Access::ReadWrite, &[("EN", 0, 1), ("BGREN", 1, 1), ("TSEN", 2, 1), ("CONT", 3, 1), ("CLK", 4, 2), ("ENS", 6, 3)]);
    control("VC1", "CR0", 0, ir::Access::ReadWrite, &[("EN", 0, 1), ("RESP", 1, 1), ("HYS", 2, 1), ("IE", 3, 1), ("POL", 4, 1), ("WINDOW", 5, 1), ("INP", 6, 2), ("INN", 8, 2)]);
    control("VC1", "CR1", 4, ir::Access::ReadWrite, &[("FLTTIME", 0, 4), ("FLTCLK", 4, 1), ("FALLIE", 5, 1), ("RISEIE", 6, 1), ("HIGHIE", 7, 1), ("BLANKATCH1", 8, 1), ("BLANKATCH2", 9, 1), ("BLANKATCH3", 10, 1), ("BLANKATCH4", 11, 1), ("BLANKATCH5", 12, 1), ("BLANKATCH6", 13, 1), ("BLANKTIME", 14, 3), ("BLANKLVL", 17, 1)]);
    control("VC2", "CR0", 0, ir::Access::ReadWrite, &[("EN", 0, 1), ("RESP", 1, 1), ("HYS", 2, 1), ("IE", 3, 1), ("POL", 4, 1), ("WINDOW", 5, 1), ("INP", 6, 2), ("INN", 8, 2)]);
    control("VC2", "CR1", 4, ir::Access::ReadWrite, &[("FLTTIME", 0, 4), ("FLTCLK", 4, 1), ("FALLIE", 5, 1), ("RISEIE", 6, 1), ("HIGHIE", 7, 1), ("BLANKATCH1", 8, 1), ("BLANKATCH2", 9, 1), ("BLANKATCH3", 10, 1), ("BLANKATCH4", 11, 1), ("BLANKATCH5", 12, 1), ("BLANKATCH6", 13, 1), ("BLANKTIME", 14, 3), ("BLANKLVL", 17, 1)]);
    control("LVD", "CR0", 0, ir::Access::ReadWrite, &[("EN", 0, 1), ("ACTION", 1, 1), ("SOURCE", 2, 1), ("VTH", 4, 3), ("FLTCLK", 8, 1)]);
    control("LVD", "CR1", 4, ir::Access::ReadWrite, &[("IE", 0, 1), ("LEVEL", 1, 1), ("RISE", 2, 1), ("FALL", 3, 1), ("FLTTIME", 4, 4)]);
    // ICR transport is RW, but own source defines R1W0 flags. Qualification
    // does not authorize any ICR operation; runtime leaves it untouched.
    let controls = [
        ("ADC", "PCLK", "apben1", "apbrst1", "adc", None),
        ("ATIM", "PCLK", "apben1", "apbrst1", "atim", None),
        ("BTIM1", "PCLK", "apben2", "apbrst2", "btim123", Some("BTIM1-BTIM2-BTIM3")),
        ("BTIM2", "PCLK", "apben2", "apbrst2", "btim123", Some("BTIM1-BTIM2-BTIM3")),
        ("BTIM3", "PCLK", "apben2", "apbrst2", "btim123", Some("BTIM1-BTIM2-BTIM3")),
        ("CRC", "HCLK", "ahben", "ahbrst", "crc", None),
        ("FLASH", "HCLK", "ahben", "ahbrst", "flash", None),
        ("GPIOA", "HCLK", "ahben", "ahbrst", "gpioa", None),
        ("GPIOB", "HCLK", "ahben", "ahbrst", "gpiob", None),
        ("GPIOC", "HCLK", "ahben", "ahbrst", "gpioc", None),
        ("GTIM1", "PCLK", "apben1", "apbrst1", "gtim1", None),
        ("GTIM2", "PCLK", "apben1", "apbrst1", "gtim2", None),
        ("I2C", "PCLK", "apben2", "apbrst2", "i2c1", None),
        ("IWDT", "PCLK", "apben2", "apbrst2", "iwdt", None),
        ("LPTIM", "PCLK", "apben2", "apbrst2", "lptim", None),
        ("RTC", "PCLK", "apben2", "apbrst2", "rtc", None),
        ("SPI", "PCLK", "apben1", "apbrst1", "spi1", None),
        ("UART1", "PCLK", "apben1", "apbrst1", "uart1", None),
        ("UART2", "PCLK", "apben1", "apbrst1", "uart2", None),
        ("UART3", "PCLK", "apben1", "apbrst1", "uart3", None),
        ("VC1", "PCLK", "apben1", "apbrst1", "vc", Some("VC1-VC2")),
        ("VC2", "PCLK", "apben1", "apbrst1", "vc", Some("VC1-VC2")),
    ];
    assert_eq!(METADATA.peripherals.iter().filter(|p| p.rcc_control.is_some()).count(), controls.len());
    for (name, bus, enable, reset, field, shared) in controls {
        let p = METADATA.peripherals.iter().find(|p| p.name == name).unwrap();
        let r = p.rcc_control.as_ref().unwrap();
        assert_eq!((r.controller, r.bus_clock), ("SYSCTRL", bus));
        assert_eq!((r.enable.register, r.enable.field), (enable, field));
        assert!(r.enable_active_value);
        let key = r.enable_write_key.as_ref().unwrap();
        assert_eq!((key.field.register, key.field.field, key.value), (enable, "key", 0x5a5a));
        let reset_field = r.reset.as_ref().unwrap();
        assert_eq!((reset_field.register, reset_field.field), (reset, field));
        assert_eq!(r.reset_asserted_value, Some(false));
        assert_eq!(r.shared_enable_group, shared);
        assert_eq!(r.shared_reset_group, shared);
    }
    let rtc = METADATA.peripherals.iter().find(|p| p.name == "RTC").unwrap().rtc_calendar.as_ref().unwrap();
    assert_eq!((rtc.source, rtc.source_encoding), ("HSIOSC", 3));
    assert_eq!((rtc.nominal_hz, rtc.minimum_hz, rtc.maximum_hz), (96_000_000, 94_080_000, 97_920_000));
    assert_eq!((rtc.prescaler_first, rtc.prescaler_second, rtc.calendar_divisor), (120, 400_000, 96_000_000));
    assert_eq!(rtc.factory_trim_address, 0x0010_07c0);
    assert_eq!((rtc.supply_mv, rtc.temperature_c), ((1700, 5500), (-40, 85)));
    if !exact {
        assert!(METADATA.memory.is_empty());
        return false;
    }
    assert_eq!(METADATA.memory.len(), 1);
    assert_eq!(METADATA.memory[0].iter().map(|m| (m.name, m.address, m.size)).collect::<Vec<_>>(), [
        ("FLASH", 0, 65_536), ("RAM", 0x2000_0000, 6144),
    ]);
    assert_eq!(c.lsi_sysclk.as_ref().unwrap(), &PeripheralLsiSysclk {
        nominal_hz: 32_800, minimum_hz: 29_520, maximum_hz: 41_000,
        supply_mv: (1700, 5500), temperature_c: (-40, 85),
        factory_trim_address: 0x0010_07c2,
        rtc_allowed_sources: &[0, 1, 2, 3], awt_allowed_sources: &[0, 1],
        uart_allowed_sources: &[0, 1, 2, 3], uarts: &["UART1", "UART2", "UART3"],
        gpio_banks: &["GPIOA", "GPIOB", "GPIOC"],
        gpio_filter_allowed_sources: &[0, 1, 2, 3, 5, 7],
        mco_allowed_sources: &[0, 1, 2, 3, 4, 5, 6],
        lsi_output_pin: None, lsi_output_allowed_af: &[], rcc_irq: 4,
    });
    true
}

// L012 has two native direct LSI outputs. The legacy PB11 scalar is only a
// compatibility anchor; never derive observer cardinality from that scalar.
fn l012_live_lsi_qualified(c: &cw32_metapac::metadata::PeripheralClockLimits) -> bool {
    use cw32_metapac::metadata::{ir, METADATA, PeripheralLsiSysclk, PeripheralPin};
    let owners: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == "SYSCTRL").collect();
    assert_eq!(owners.len(), 1);
    let sysctrl = owners[0];
    let regs = sysctrl.registers.as_ref().unwrap();
    let native = regs.version == "cw32l012_v1";
    let exact = matches!(METADATA.name, "CW32L012C8T6" | "CW32L012C8U6");
    if !(native || METADATA.line == "CW32L012" || exact) { return false; }
    assert!(native && METADATA.line == "CW32L012");
    assert!(exact || METADATA.name == "CW32L012");
    assert_eq!(c.lsi_sysclk.is_some(), exact, "missing or unexpected L012 live-LSI profile");
    assert!(c.pll.is_none());
    let routes: Vec<_> = sysctrl.pins.iter().filter(|p| p.signal == "LSI_OUT").cloned().collect();
    if !exact {
        assert!(routes.is_empty(), "generic L012 must not acquire direct LSI projections");
        return false;
    }
    assert_eq!(c.lsi_sysclk.as_ref().unwrap(), &PeripheralLsiSysclk {
        nominal_hz: 32_800, minimum_hz: 29_520, maximum_hz: 36_080,
        supply_mv: (1700, 5500), temperature_c: (-40, 85),
        factory_trim_address: 0x0010_07c2,
        rtc_allowed_sources: &[0, 1, 2, 3], awt_allowed_sources: &[0, 1],
        uart_allowed_sources: &[0, 1, 2, 3], uarts: &["UART1", "UART2", "UART3"],
        gpio_banks: &["GPIOA", "GPIOB", "GPIOC", "GPIOF"],
        gpio_filter_allowed_sources: &[0, 1, 2, 3, 5, 6, 7],
        mco_allowed_sources: &[0, 1, 2, 3, 4, 5, 6, 9],
        lsi_output_pin: Some("PB11"), lsi_output_allowed_af: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], rcc_irq: 4,
    });
    assert_eq!(routes, [
        PeripheralPin { pin: "PB11", signal: "LSI_OUT", af: Some(4), adc_mux: None, comparator_mux: None },
        PeripheralPin { pin: "PF3", signal: "LSI_OUT", af: Some(2), adc_mux: None, comparator_mux: None },
    ]);
    assert_eq!(METADATA.peripherals.iter().filter(|p| p.clock_limits.is_some()).count(), 1);
    assert_eq!(regs.kind, "sysctrl");
    assert_eq!(regs.block, "Sysctrl");
    let field = |owner: &str, register: &str, byte_offset, name: &str, bit_offset, width| {
        let peripherals: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == owner).collect();
        assert_eq!(peripherals.len(), 1);
        let regs = peripherals[0].registers.as_ref().unwrap();
        assert_eq!(regs.version, "cw32l012_v1");
        let block = regs.ir.blocks.iter().find(|b| b.name == regs.block).unwrap();
        let items: Vec<_> = block.items.iter().filter(|r| r.name.eq_ignore_ascii_case(register)).collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].byte_offset, byte_offset);
        assert!(items[0].array.is_none());
        let ir::BlockItemInner::Register(r) = &items[0].inner else { panic!("L012 control must be a register") };
        assert_eq!(r.bit_size, 32);
        assert_ne!(r.access, ir::Access::Write);
        let fs = regs.ir.fieldsets.iter().find(|f| Some(f.name) == r.fieldset).unwrap();
        let fields: Vec<_> = fs.fields.iter().filter(|f| f.name.eq_ignore_ascii_case(name)).collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].bit_size, width);
        assert!(fields[0].array.is_none());
        assert!(matches!(&fields[0].bit_offset, ir::BitOffset::Regular(p) if p.offset == bit_offset));
    };
    for (register, offset, name, bit, width) in [
        ("LSI",32,"TRIM",0,9), ("LSI",32,"WAITCYCLE",10,2), ("LSI",32,"STABLE",15,1),
        ("HSI",24,"TRIM",0,11), ("HSI",24,"DIV",11,4), ("HSI",24,"STABLE",15,1),
        ("HSE",28,"STABLE",19,1), ("LSE",36,"STABLE",18,1),
    ] { field("SYSCTRL", register, offset, name, bit, width); }
    for (pin, gpio, register, offset, name) in [
        ("PB11", "GPIOB", "AFRH", 20, "PIN11"), ("PF3", "GPIOF", "AFRL", 24, "PIN3"),
    ] {
        assert_eq!(METADATA.pins.iter().filter(|p| p.name == pin).count(), 1);
        field(gpio, register, offset, name, 12, 4);
    }
    true
}

fn generate_factory_lsi(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use cw32_metapac::metadata::METADATA;
    use std::fmt::Write;
    let l010 = l010_live_lsi_qualified(c);
    let l011 = l011_live_lsi_qualified(c);
    let l012 = l012_live_lsi_qualified(c);
    let Some(lsi) = &c.lsi_sysclk else { return };
    println!("cargo:rustc-cfg=rcc_lsi_sysclk");
    let sysctrl = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap()
        .registers
        .as_ref()
        .unwrap();
    let block = sysctrl
        .ir
        .blocks
        .iter()
        .find(|b| b.name == sysctrl.block)
        .unwrap();
    let register = block
        .items
        .iter()
        .find(|r| r.name.eq_ignore_ascii_case("LSI"))
        .unwrap();
    if l010 || l011 || l012 {
        assert_eq!(register.byte_offset, 32);
        assert!(register.array.is_none());
    }
    let cw32_metapac::metadata::ir::BlockItemInner::Register(register) = &register.inner else {
        panic!("LSI is not a register")
    };
    let fields = sysctrl
        .ir
        .fieldsets
        .iter()
        .find(|f| Some(f.name) == register.fieldset)
        .unwrap();
    let stable = fields
        .fields
        .iter()
        .find(|f| f.name.eq_ignore_ascii_case("STABLE"))
        .unwrap();
    assert_eq!(stable.bit_size, 1);
    assert!(stable.array.is_none());
    let cw32_metapac::metadata::ir::BitOffset::Regular(offset) = &stable.bit_offset else {
        panic!("LSI stability field is not scalar")
    };
    if l010 || l011 || l012 {
        assert_eq!(offset.offset, 15);
        assert_eq!(register.bit_size, 32);
        assert_ne!(register.access, cw32_metapac::metadata::ir::Access::Write);
        for (name, expected_offset, width) in [("TRIM", 0, if l012 { 9 } else { 10 }), ("WAITCYCLE", 10, 2)] {
            let field = fields.fields.iter().find(|f| f.name.eq_ignore_ascii_case(name)).unwrap();
            assert!(field.array.is_none());
            assert_eq!(field.bit_size, width);
            assert!(matches!(&field.bit_offset, cw32_metapac::metadata::ir::BitOffset::Regular(p) if p.offset == expected_offset));
        }
    }
    // Derive the read-only normalization mask from the selected PAC IR, never
    // duplicate a register bit definition in the runtime backend.
    writeln!(
        out,
        "pub(crate) const RCC_LSI_PARAMETERS_MASK: u32 = {};",
        !(1u32 << offset.offset)
    )
    .unwrap();

    let l031 = sysctrl.version == "cw32l031_v1";
    if l031 {
        assert!(matches!(
            (METADATA.line, METADATA.name),
            ("CW32L031", "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6")
                | ("CW32R031", "CW32R031C8U6")
        ));
    }
    let parameter_sources: &[(&str, u32, u32)] = match sysctrl.version {
        "cw32l010_v1" => &[("HSI", 24, 15), ("HSE", 28, 19)],
        "cw32l011_v1" | "cw32l012_v1" => &[("HSI", 24, 15), ("HSE", 28, 19), ("LSE", 36, 18)],
        "cw32f002_v1" | "cw32f003_v1" => &[("HSI", 24, 15), ("HEX", 28, 19)],
        "cw32l031_v1" | "cw32l052_v1" => &[("HSI", 24, 15), ("HSE", 28, 19), ("LSE", 36, 15)],
        "cw32l083_v1" => &[("HSI", 24, 15), ("HSE", 28, 19), ("LSE", 36, 15), ("PLL", 40, 15)],
        _ => &[],
    };
    {
        // These startup latches are already read-only in the selected PAC's
        // reviewed field-access policy. Derive masks from their actual IR;
        // the runtime compares parameters and checks typed readiness separately.
        for &(name, byte_offset, stable_bit) in parameter_sources {
            let item = block
                .items
                .iter()
                .find(|r| r.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert_eq!(item.byte_offset, byte_offset);
            assert!(item.array.is_none());
            let cw32_metapac::metadata::ir::BlockItemInner::Register(register) = &item.inner else {
                panic!("source control is not a register")
            };
            assert_eq!(register.bit_size, 32);
            if l010 || l011 || l012 { assert_ne!(register.access, cw32_metapac::metadata::ir::Access::Write); }
            let fields = sysctrl
                .ir
                .fieldsets
                .iter()
                .find(|f| Some(f.name) == register.fieldset)
                .unwrap();
            let stable = fields
                .fields
                .iter()
                .find(|f| f.name.eq_ignore_ascii_case("STABLE"))
                .unwrap();
            assert_eq!(stable.bit_size, 1);
            assert!(stable.array.is_none());
            let cw32_metapac::metadata::ir::BitOffset::Regular(position) = &stable.bit_offset
            else {
                panic!("startup stability field is not scalar")
            };
            assert_eq!(position.offset, stable_bit);
            let parameters_mask = !1u32.checked_shl(position.offset).unwrap();
            // L011 retains the whole inherited LSE word, while still validating its IR above.
            if !l011 || name != "LSE" {
                writeln!(
                    out,
                    "pub(crate) const RCC_LSI_{name}_PARAMETERS_MASK: u32 = {parameters_mask};"
                )
                .unwrap();
            }
        }
    }

    for (name, value) in [
        ("NOMINAL_HZ", lsi.nominal_hz),
        ("MINIMUM_HZ", lsi.minimum_hz),
        ("MAXIMUM_HZ", lsi.maximum_hz),
        ("FACTORY_TRIM_ADDRESS", lsi.factory_trim_address),
    ] {
        writeln!(out, "pub(crate) const RCC_LSI_{name}: u32 = {value};").unwrap();
    }
    writeln!(
        out,
        "pub(crate) const RCC_LSI_SUPPLY_MV: (u16, u16) = {:?};",
        lsi.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSI_TEMPERATURE_C: (i16, i16) = {:?};",
        lsi.temperature_c
    )
    .unwrap();
    let observer_irq = if l010 || l011 || l012 || l031 || matches!(sysctrl.version, "cw32l052_v1" | "cw32l083_v1") { "SYSCTRL" } else { "RCC" };
    assert!(
        METADATA
            .interrupts
            .iter()
            .any(|i| i.name == observer_irq && i.number == u32::from(lsi.rcc_irq))
    );
    if l011 {
        // Own complete source/owner shapes and three-bank topology were checked
        // above. Live reuse never reaches standalone AWT, UART.CR2 or cold
        // consumer generation; L011 has no direct LSI_OUT pin projection.
        return;
    }
    if l012 {
        // The complete record, both bonded routes and actual field shapes were
        // checked before this live-only return. No cold/AWT/singular-output
        // generator may infer extra L012 observation windows from the scalar.
        for name in ["GPIOA", "GPIOB", "GPIOC", "GPIOF"] {
            let gpio = METADATA.peripherals.iter().find(|p| p.name == name).unwrap();
            let regs = gpio.registers.as_ref().unwrap();
            assert_eq!((regs.kind, regs.version, regs.block), ("gpio", "cw32l012_v1", "Gpio"));
        }
        let uarts: Vec<_> = METADATA.peripherals.iter().filter(|p|
            p.registers.as_ref().is_some_and(|r| r.kind == "uart")
        ).map(|p| p.name).collect();
        assert_eq!(uarts, lsi.uarts);
        return;
    }
    if l010 {
        // Native GPIOB has a separate kind. This leaf must precede the generic
        // bank roster and every cold-consumer/AWT/UART generator below.
        for (name, kind) in [("GPIOA", "gpio"), ("GPIOB", "gpiob")] {
            let gpio = METADATA.peripherals.iter().find(|p| p.name == name).unwrap();
            let regs = gpio.registers.as_ref().unwrap();
            assert_eq!(regs.kind, kind);
            assert_eq!(regs.version, "cw32l010_v1");
        }
        let uarts: Vec<_> = METADATA.peripherals.iter().filter(|p|
            p.registers.as_ref().is_some_and(|r| r.kind == "uart")
        ).map(|p| p.name).collect();
        assert_eq!(uarts, lsi.uarts);
        return;
    }
    let peripherals = |kind: &str| {
        METADATA
            .peripherals
            .iter()
            .filter(|p| {
                p.registers.as_ref().is_some_and(|r| {
                    if kind == "gpio" {
                        matches!(r.kind.as_ref(), "gpio" | "gpioc" | "gpiof")
                    } else {
                        r.kind == kind
                    }
                })
            })
            .map(|p| p.name)
            .collect::<Vec<_>>()
    };
    assert_eq!(peripherals("uart"), lsi.uarts);
    assert_eq!(peripherals("gpio"), lsi.gpio_banks);
    // Register layout and own-source gate/reset semantics were validated before
    // metadata emission. All MMIO below uses the selected typed PAC and the one
    // existing central gate-inspection protocol; no local I/O abstraction.
    let has_rtc = METADATA.peripherals.iter().any(|p| p.name == "RTC");
    assert_eq!(has_rtc, !lsi.rtc_allowed_sources.is_empty());
    if matches!(sysctrl.version, "cw32l052_v1" | "cw32l083_v1") {
        generate_l052_l083_factory_lsi(out, c);
        return;
    }
    let mut roots = Vec::new();
    if has_rtc {
        roots.push((
            "RTC",
            "u8::from(crate::pac::RTC.cr1().read().source())".to_string(),
            lsi.rtc_allowed_sources,
        ));
    }
    roots.push((
        "AWT",
        "u8::from(crate::pac::AWT.cr().read().src())".to_string(),
        lsi.awt_allowed_sources,
    ));
    for uart in lsi.uarts {
        roots.push((
            uart,
            format!("u8::from(crate::pac::{uart}.cr2().read().source())"),
            lsi.uart_allowed_sources,
        ));
    }
    let gates = roots.len() + lsi.gpio_banks.len();
    let sources = gates + 1 + usize::from(lsi.lsi_output_pin.is_some());
    // Only the separately qualified F002/F003 and L031 IPs get strict windows.
    // Hardware absence chooses roots, never the gate/reset checking strategy.
    let hex_windows = matches!(sysctrl.version, "cw32f002_v1" | "cw32f003_v1");
    let strict_windows = hex_windows || l031;
    if hex_windows {
        let hex_gpio_banks: std::collections::BTreeSet<_> = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == "SYSCTRL")
            .unwrap()
            .pins
            .iter()
            .filter(|p| matches!(p.signal, "HEX_PB00" | "HEX_PB01"))
            .map(|p| format!("GPIO{}", char::from(p.pin.as_bytes()[1])))
            .collect();
        assert_eq!(hex_gpio_banks.len(), 1);
        let hex_gpio = hex_gpio_banks.iter().next().unwrap();
        let index = roots.len()
            + lsi
                .gpio_banks
                .iter()
                .position(|p| *p == hex_gpio.as_str())
                .unwrap();
        writeln!(
            out,
            "pub(crate) const RCC_LSI_HEX_GPIO_GATE_INDEX: usize = {index};"
        )
        .unwrap();
    }
    if l031 {
        assert_eq!((gates, sources), (9, 11));
        assert_eq!(lsi.lsi_output_pin, Some("PB11"));
        let controller = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == "SYSCTRL")
            .unwrap();
        let mut indices = [None; 2];
        for (index, signal) in ["HSE_IN", "HSE_OUT"].iter().enumerate() {
            let pins = controller
                .pins
                .iter()
                .filter(|p| p.signal == *signal)
                .collect::<Vec<_>>();
            assert!(pins.len() <= 1, "ambiguous L031 HSE gate projection");
            if let Some(pin) = pins.first() {
                assert!(pin.af.is_none());
                let gpio = format!("GPIO{}", char::from(pin.pin.as_bytes()[1]));
                assert_eq!(gpio, "GPIOF");
                indices[index] = Some(
                    roots.len()
                        + lsi
                            .gpio_banks
                            .iter()
                            .position(|p| *p == gpio.as_str())
                            .unwrap(),
                );
            }
        }
        assert_eq!(
            indices,
            if METADATA.name == "CW32L031F8U6" {
                [None, None]
            } else {
                [Some(8), Some(8)]
            }
        );
        writeln!(out, "pub(crate) const RCC_LSI_HSE_GPIO_GATE_INDICES: (Option<usize>, Option<usize>) = {:?};", (indices[0], indices[1])).unwrap();
    }
    writeln!(
        out,
        "pub(crate) type RccFactoryLsiConsumers = ([u8; {sources}], [bool; {gates}]);"
    )
    .unwrap();
    let monitor_arguments = if l031 {
        ", monitor_hse: bool, monitor_lse: bool"
    } else {
        ""
    };
    writeln!(out, "pub(crate) fn rcc_factory_lsi_consumers(timeout: u32, cs: critical_section::CriticalSection<'_>{monitor_arguments}) -> Result<RccFactoryLsiConsumers, crate::rcc::Error> {{\nuse crate::rcc::SealedRccPeripheral;\nlet mut sources = [0; {sources}];\nlet mut gates = [false; {gates}];").unwrap();
    // Each buffered semantic result is examined only after the central helper
    // restores its gate. Restore errors dominate even a rejected selector.
    let strict_window = |out: &mut String,
                         index: usize,
                         name: &str,
                         read: &str,
                         allowed: &[u8],
                         extra: Option<(&str, &[u8])>| {
        // Fixed target monitor flags are captured before any source write. A
        // successful central restoration and original-gate check precede faults;
        // relevant faults precede post-reset and buffered semantic refusals.
        let faults = if l031 {
            "let flags = crate::pac::SYSCTRL.isr().read();\nif (monitor_hse && (flags.hsefail() || flags.hsefault())) || (monitor_lse && (flags.lsefail() || flags.lsefault())) { return Err(crate::rcc::Error::ExternalClockFault); }\n"
        } else {
            ""
        };
        let (extra_read, extra_check, result, release) = if let Some((read, allowed)) = extra {
            assert!(l031, "only L031 adds an output to a strict GPIO window");
            (
                format!("let af = {read};\n"),
                format!(" || !{allowed:?}.contains(&af)"),
                "(source, af)",
                format!(
                    "let (source, af) = buffered?;\nsources[{index}] = source;\nsources[{}] = af;",
                    gates + 1
                ),
            )
        } else {
            (
                String::new(),
                String::new(),
                "source",
                format!("sources[{index}] = buffered?;"),
            )
        };
        writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\ngates[{index}] = info.is_enabled();\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nlet buffered = info.inspect_for_init(cs, timeout, || {{\nif !info.is_enabled() || info.reset_asserted() {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nlet source = {read};\n{extra_read}if !info.is_enabled() || info.reset_asserted() || !{allowed:?}.contains(&source){extra_check} {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nOk({result})\n}}).map_err(crate::rcc::lsi_inspection_error)?;\nif info.is_enabled() != gates[{index}] {{ return Err(crate::rcc::Error::LsiGateRestoreTimeout); }}\n{faults}if info.reset_asserted() {{ return Err(crate::rcc::Error::LsiClockInUse); }}\n{release}").unwrap();
    };
    for (i, (name, read, allowed)) in roots.iter().enumerate() {
        if strict_windows {
            strict_window(out, i, name, read, allowed, None);
        } else {
            writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LsiClockInUse); }}\ngates[{i}] = info.is_enabled();\nlet source = info.inspect_for_init(cs, timeout, || {read}).map_err(crate::rcc::lsi_inspection_error)?;\nif info.reset_asserted() || info.is_enabled() != gates[{i}] || !{:?}.contains(&source) {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nsources[{i}] = source;", allowed).unwrap();
        }
    }
    let output = lsi.lsi_output_pin.map(|pin| {
        assert_eq!(pin.as_bytes().first(), Some(&b'P'));
        let gpio = format!("GPIO{}", char::from(pin.as_bytes()[1]));
        let bit: u8 = pin[2..].parse().unwrap();
        assert!(bit < 16 && lsi.gpio_banks.contains(&gpio.as_str()));
        assert!(!lsi.lsi_output_allowed_af.is_empty());
        (gpio, bit, if bit < 8 { "afrl" } else { "afrh" })
    });
    assert_eq!(output.is_none(), lsi.lsi_output_allowed_af.is_empty());
    for (i, gpio) in lsi.gpio_banks.iter().enumerate() {
        let index = roots.len() + i;
        if strict_windows {
            // F002/F003 retain their proved output absence. L031 reads GPIOB
            // FILTER and AFR11 together, including on physically unbonded F8U6.
            assert!(l031 || output.is_none());
            let extra = output
                .as_ref()
                .filter(|(output_gpio, _, _)| *gpio == output_gpio.as_str())
                .map(|(_, bit, afreg)| format!("crate::pac::{gpio}.{afreg}().read().afr{bit}()"));
            strict_window(
                out,
                index,
                gpio,
                &format!("crate::pac::{gpio}.filter().read().fltclk()"),
                lsi.gpio_filter_allowed_sources,
                extra
                    .as_deref()
                    .map(|read| (read, lsi.lsi_output_allowed_af)),
            );
            continue;
        }
        writeln!(out, "let info = crate::peripherals::{gpio}::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LsiClockInUse); }}\ngates[{index}] = info.is_enabled();").unwrap();
        if let Some((_, output_bit, afreg)) = output
            .as_ref()
            .filter(|(output_gpio, _, _)| *gpio == output_gpio.as_str())
        {
            writeln!(out, "let (source, af) = info.inspect_for_init(cs, timeout, || (crate::pac::{gpio}.filter().read().fltclk(), crate::pac::{gpio}.{afreg}().read().afr{output_bit}())).map_err(crate::rcc::lsi_inspection_error)?;\nif !{:?}.contains(&af) {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nsources[{}] = af;", lsi.lsi_output_allowed_af, gates + 1).unwrap();
        } else {
            writeln!(out, "let source = info.inspect_for_init(cs, timeout, || crate::pac::{gpio}.filter().read().fltclk()).map_err(crate::rcc::lsi_inspection_error)?;").unwrap();
        }
        writeln!(out, "if info.reset_asserted() || info.is_enabled() != gates[{index}] || !{:?}.contains(&source) {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nsources[{index}] = source;", lsi.gpio_filter_allowed_sources).unwrap();
    }
    writeln!(out, "let source = crate::pac::SYSCTRL.mco().read().source();\nif !{:?}.contains(&source) {{ return Err(crate::rcc::Error::LsiClockInUse); }}\nsources[{gates}] = source;\nOk((sources, gates))\n}}", lsi.mco_allowed_sources).unwrap();
}

fn generate_pll(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use std::fmt::Write;
    let Some(pll) = &c.pll else { return };
    assert!(pll.hsi_supported && c.hse.is_some());
    println!("cargo:rustc-cfg=rcc_pll");
    writeln!(
        out,
        "pub(crate) const RCC_PLL_HSE_SUPPORTED: bool = {};",
        pll.hse_supported
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_INPUT_RANGE_HZ: (u32, u32) = {:?};",
        pll.input_range_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_OUTPUT_RANGE_HZ: (u32, u32) = {:?};",
        pll.output_range_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_INPUT_BINS_HZ: [(u32, u32); 4] = {:?};",
        pll.input_ranges_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_OUTPUT_BINS_HZ: [(u32, u32); 5] = {:?};",
        pll.output_ranges_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_MULTIPLIER_RANGE: (u8, u8) = {:?};",
        pll.multiplier_range
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_SUPPLY_MV: (u16, u16) = {:?};",
        pll.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_TEMPERATURE_C: (i16, i16) = {:?};",
        pll.temperature_c
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_STARTUP_ENCODING: u8 = {:?};",
        pll.startup_encoding
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_PLL_RESERVED_DEBUG_DEFAULT: u8 = {:?};",
        pll.reserved_debug_default
    )
    .unwrap();
}

fn generate_hse(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use cw32_metapac::metadata::ir;
    use std::fmt::Write;
    let Some(hse) = &c.hse else { return };
    assert_eq!(hse.crystal.supply_mv, hse.bypass.supply_mv);
    assert_eq!(hse.crystal.temperature_c, hse.bypass.temperature_c);
    assert!(hse.ccs_requires_lsi);
    // HSE initialization also retains qualified HSI. CCS only needs LSI
    // coverage over the board conditions admitted by both source envelopes.
    let supply_mv = (
        hse.crystal.supply_mv.0.max(c.hsi_supply_mv.0),
        hse.crystal.supply_mv.1.min(c.hsi_supply_mv.1),
    );
    let temperature_c = (
        hse.crystal.temperature_c.0.max(c.hsi_temperature_c.0),
        hse.crystal.temperature_c.1.min(c.hsi_temperature_c.1),
    );
    assert!(supply_mv.0 <= supply_mv.1);
    assert!(temperature_c.0 <= temperature_c.1);
    assert!(hse.ccs_lsi_supply_mv.0 <= supply_mv.0 && hse.ccs_lsi_supply_mv.1 >= supply_mv.1);
    assert!(
        hse.ccs_lsi_temperature_c.0 <= temperature_c.0
            && hse.ccs_lsi_temperature_c.1 >= temperature_c.1
    );
    writeln!(
        out,
        "pub(crate) const RCC_HSE_CRYSTAL_RANGE_HZ: (u32,u32) = {:?};",
        (hse.crystal.minimum_hz, hse.crystal.maximum_hz)
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_BYPASS_RANGE_HZ: (u32,u32) = {:?};",
        (hse.bypass.minimum_hz, hse.bypass.maximum_hz)
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_SUPPLY_MV: (u16,u16) = {:?};",
        hse.crystal.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_TEMPERATURE_C: (i16,i16) = {:?};",
        hse.crystal.temperature_c
    )
    .unwrap();
    if let Some(ranges) = hse.frequency_ranges_hz {
        writeln!(
            out,
            "pub(crate) const RCC_HSE_FREQUENCY_RANGES_HZ: [(u32,u32);4] = {:?};",
            ranges
        )
        .unwrap();
    }
    writeln!(
        out,
        "pub(crate) const RCC_HSE_CCS_CYCLE_COUNT: u32 = {};",
        hse.ccs_cycle_count
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_CCS_NUMERATOR_HZ: u64 = {};",
        hse.ccs_numerator_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_CCS_MAXIMUM_COUNT: u16 = {};",
        hse.ccs_maximum_count
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HSE_CCS_LSI_MAXIMUM_HZ: u32 = {};",
        hse.ccs_lsi_maximum_hz
    )
    .unwrap();
    let sysctrl = cw32_metapac::metadata::METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap();
    let regs = sysctrl.registers.as_ref().unwrap();
    if matches!(
        regs.version,
        "cw32l010_v1" | "cw32l011_v1" | "cw32l012_v1" | "cw32l052_v1"
    ) {
        let divisor = hse
            .fixed_ccs_hsi_divisor
            .expect("missing source-qualified fixed CCS divisor");
        assert!(divisor != 0);
        writeln!(
            out,
            "pub(crate) const RCC_FIXED_CCS_HSI_DIVISOR: u32 = {divisor};"
        )
        .unwrap();
    }
    if regs.version == "cw32l012_v1" {
        let range = c
            .hsi_operating_range_hz
            .expect("missing source-qualified incoming HSIOSC range");
        assert!(range.0 > 0 && range.0 <= range.1);
        writeln!(
            out,
            "pub(crate) const RCC_HSI_OPERATING_RANGE_HZ: (u32,u32) = {range:?};"
        )
        .unwrap();
    }
    let hse_register = &regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap()
        .items
        .iter()
        .find(|r| r.name.eq_ignore_ascii_case("HSE"))
        .expect("missing HSE register")
        .inner;
    let ir::BlockItemInner::Register(hse_register) = hse_register else {
        panic!("HSE control is not a register")
    };
    // The reviewed F020 IP calls the same two-bit selector FREQ. Select the
    // exact own-IP field name, then verify its enum, width and location.
    let range_name = match regs.version {
        "cw32f020_v1" => Some("FREQ"),
        "v1" | "cw32l031_v1" | "cw32l052_v1" | "cw32l083_v1" => Some("FREQRANGE"),
        "cw32l010_v1" | "cw32l011_v1" | "cw32l012_v1" => None,
        _ => panic!("unqualified HSE selector IP"),
    };
    let fields = regs
        .ir
        .fieldsets
        .iter()
        .find(|f| Some(f.name) == hse_register.fieldset)
        .unwrap()
        .fields;
    assert_eq!(
        hse.frequency_ranges_hz.is_some(),
        range_name.is_some(),
        "HSE range data must match the qualified selector IP"
    );
    if let Some(name) = range_name {
        let field = fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(name))
            .expect("missing qualified HSE frequency selector");
        assert!(field.array.is_none());
        assert_eq!(field.bit_size, 2);
        assert!(matches!(&field.bit_offset, ir::BitOffset::Regular(p) if p.offset == 2));
        assert_eq!(field.enumm, Some("HseRange"));
    } else {
        assert!(
            !fields.iter().any(|f| f.name.eq_ignore_ascii_case("FREQ")
                || f.name.eq_ignore_ascii_case("FREQRANGE")
                || f.enumm == Some("HseRange")),
            "unexpected HSE selector on selector-free IP"
        );
    }
    let mut pins = [None; 2];
    for (index, signal) in ["HSE_IN", "HSE_OUT"].iter().enumerate() {
        let found: Vec<_> = sysctrl
            .pins
            .iter()
            .filter(|p| p.signal == *signal)
            .collect();
        assert!(found.len() <= 1, "ambiguous oscillator pad");
        if let Some(pin) = found.first() {
            assert!(
                pin.af.is_none(),
                "oscillator pad must not be represented as a digital AF"
            );
            let port = pin.pin.as_bytes()[1] - b'A';
            let bit: u8 = pin.pin[2..].parse().unwrap();
            pins[index] = Some((port, bit));
        }
    }
    // Only these existing backends inspect retained VC pad ownership during RCC.
    if matches!(regs.version, "cw32l010_v1" | "cw32l011_v1") {
        // Project existing own-source comparator routes onto the exact oscillator
        // pads. Masks are ordered OSC_IN, OSC_OUT; no family/pin literals in RCC.
        for owner in ["VC1", "VC2"] {
            for input in ["INP", "INN"] {
                let mut masks = [0u32; 2];
                if let Some(vc) = cw32_metapac::metadata::METADATA
                    .peripherals
                    .iter()
                    .find(|p| p.name == owner)
                {
                    for route in vc.pins.iter().filter(|p| p.signal.starts_with(input)) {
                        let mux = route.comparator_mux.expect("missing HSE/VC analog mux");
                        let port = route.pin.as_bytes()[1] - b'A';
                        let bit: u8 = route.pin[2..].parse().unwrap();
                        for (index, pin) in pins.iter().enumerate() {
                            if *pin == Some((port, bit)) {
                                masks[index] |= 1u32
                                    .checked_shl(u32::from(mux))
                                    .expect("HSE/VC mux exceeds input mask");
                            }
                        }
                    }
                }
                writeln!(
                    out,
                    "pub(crate) const RCC_HSE_{owner}_{input}_MASKS: [u32;2] = {:?};",
                    masks
                )
                .unwrap();
            }
        }
    }
    // Use the actual pad bank's PAC registers. Some HSE-capable GPIO blocks
    // have neither a lock register nor native high/low-level interrupts.
    let gpio_fields = |port: u8, name: &str| -> Option<&'static [ir::Field]> {
        let gpio = format!("GPIO{}", char::from(b'A' + port));
        let regs = cw32_metapac::metadata::METADATA
            .peripherals
            .iter()
            .find(|p| p.name == gpio)
            .expect("missing HSE GPIO bank")
            .registers
            .as_ref()
            .unwrap();
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))?;
        let ir::BlockItemInner::Register(reg) = &item.inner else {
            panic!("HSE pad control is not a register")
        };
        assert!(item.array.is_none());
        assert_eq!(reg.bit_size, 32);
        assert_eq!(reg.access, ir::Access::ReadWrite);
        Some(
            regs.ir
                .fieldsets
                .iter()
                .find(|f| Some(f.name) == reg.fieldset)
                .unwrap()
                .fields,
        )
    };
    let pad_registers = pins.map(|pin| {
        pin.map(|(port, bit)| {
            let gpio = format!("GPIO{}", char::from(b'A' + port));
            let pads = cw32_metapac::metadata::METADATA
                .peripherals
                .iter()
                .find(|p| p.name == gpio)
                .unwrap()
                .gpio
                .as_ref()
                .expect("missing HSE GPIO pad capabilities");
            let mut registers = vec!["dir", "pur"];
            // A generic PDR field does not establish pull-down support on this pad.
            if pads.pull_down_mask & (1 << bit) != 0 {
                assert!(gpio_fields(port, "pdr").is_some(), "missing HSE pad PDR");
                registers.push("pdr");
            }
            registers.extend(["riseie", "fallie"]);
            registers.extend(
                ["highie", "lowie"]
                    .into_iter()
                    .filter(|name| gpio_fields(port, name).is_some()),
            );
            registers.extend(["opendrain", "filter", "analog"]);
            let lock = ["lock", "lckr"]
                .into_iter()
                .find(|name| gpio_fields(port, name).is_some());
            if let Some(lock) = lock {
                registers.push(lock);
                assert!(gpio_fields(port, lock).unwrap().iter().any(|f| {
                    f.name.eq_ignore_ascii_case("key")
                        && f.array.is_none()
                        && f.bit_size == 16
                        && matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == 16)
                }));
            }
            for name in &registers {
                let fields = gpio_fields(port, name).expect("missing HSE pad control register");
                assert!(
                    fields.iter().any(|f| {
                        f.name.eq_ignore_ascii_case(&format!("pin{bit}"))
                            && f.array.is_none()
                            && f.bit_size == 1
                            && matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == u32::from(bit))
                    }),
                    "missing HSE pad control field {name}.pin{bit}"
                );
            }
            let afreg = if bit < 8 { "afrl" } else { "afrh" };
            // The locked IR uses AFRn or PINn for the same pad-local mux.
            // Validate its physical field, then emit the actual typed PAC name.
            let af_fields: Vec<_> = gpio_fields(port, afreg)
                .expect("missing HSE pad AF register")
                .iter()
                .filter(|f| {
                    (f.name.eq_ignore_ascii_case(&format!("afr{bit}"))
                        || f.name.eq_ignore_ascii_case(&format!("pin{bit}")))
                        && f.array.is_none()
                        && matches!(f.bit_size, 3 | 4)
                        && matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == u32::from(bit % 8) * 4)
                })
                .collect();
            assert_eq!(af_fields.len(), 1, "missing or ambiguous HSE pad AF field");
            let af_field = af_fields[0].name.to_ascii_lowercase();
            registers.retain(|name| !matches!(*name, "dir" | "analog" | "lock" | "lckr"));
            (lock, registers, af_field)
        })
    });
    writeln!(
        out,
        "pub(crate) const RCC_HSE_PINS: (Option<u8>,Option<u8>) = {:?};",
        (
            pins[0].map(|(p, b)| p * 16 + b),
            pins[1].map(|(p, b)| p * 16 + b)
        )
    )
    .unwrap();
    let l010_live = l010_live_lsi_qualified(c);
    if l010_live {
        let inventory: Vec<_> = sysctrl.pins.iter()
            .filter(|p| p.signal.starts_with("HSE"))
            .map(|p| (p.signal, p.pin, p.af)).collect();
        assert_eq!(inventory.len(), 2, "unexpected native HSE pad inventory");
        assert!(inventory.contains(&("HSE_IN", "PA0", None)));
        assert!(inventory.contains(&("HSE_OUT", "PA1", None)));
        assert_eq!(pins, [Some((0, 0)), Some((0, 1))]);
    }
    let l012 = regs.version == "cw32l012_v1";
    let l012_live = l012_live_lsi_qualified(c);
    if l012_live {
        let inventory: Vec<_> = sysctrl.pins.iter()
            .filter(|p| p.signal.starts_with("HSE"))
            .map(|p| (p.signal, p.pin, p.af)).collect();
        assert_eq!(inventory.len(), 2, "unexpected native L012 HSE pad inventory");
        assert!(inventory.contains(&("HSE_IN", "PF0", None)));
        assert!(inventory.contains(&("HSE_OUT", "PF1", None)));
        assert_eq!(pins, [Some((5, 0)), Some((5, 1))]);
    }
    let l012_target = l012
        && c.lse_configuration
            .as_ref()
            .is_some_and(|lse| lse.sysclk_detector.is_some());
    let argument = if l010_live { ", lsi_sysclk: bool" } else if l012 { ", target: crate::rcc::L012HseTarget" } else { "" };
    let matcher_argument = if l012 && !l012_target {
        ", _target: crate::rcc::L012HseTarget"
    } else {
        argument
    };
    let forwarded = if l010_live { ", lsi_sysclk" } else if l012 { ", target" } else { "" };
    // Keep actual gate failures ahead of faults, and faults ahead of buffered
    // reset/pad refusals. Only exact-qualified L012 can emit these target checks.
    let target_faults = "if target == crate::rcc::L012HseTarget::LseSysclk { let flags = crate::pac::SYSCTRL.isr().read(); if flags.hsefail() || flags.hsefault() || flags.lsefail() || flags.lsefault() { return Err(crate::rcc::Error::ExternalClockFault); } }\n";
    // Without a bonded input, neither crystal nor bypass is available.
    if pins[0].is_none() {
        let unused_argument = if l012 { ", _target: crate::rcc::L012HseTarget" } else { "" };
        writeln!(out, "pub(crate) fn rcc_configure_hse_pins(_bypass: bool, _timeout: u32, _cs: critical_section::CriticalSection<'_>{unused_argument}) -> Result<(), crate::rcc::Error> {{\nErr(crate::rcc::Error::HsePinsUnavailable)\n}}").unwrap();
        writeln!(out, "pub(crate) fn rcc_hse_pins_match(_bypass: bool, _timeout: u32, _cs: critical_section::CriticalSection<'_>{unused_argument}) -> Result<bool, crate::rcc::Error> {{\nOk(false)\n}}").unwrap();
        return;
    }
    writeln!(out, "pub(crate) fn rcc_configure_hse_pins(bypass: bool, timeout: u32, cs: critical_section::CriticalSection<'_>{argument}) -> Result<(), crate::rcc::Error> {{").unwrap();
    if l010_live {
        out.push_str("if lsi_sysclk {\nlet info = gpio_rcc(0);\nif info.reset_asserted() { return Err(crate::rcc::Error::HsePinInUse); }\ninfo.enable_with_cs_readback(cs, crate::rcc::Readback::Poll { attempts: timeout, spin: true }).map_err(|_| crate::rcc::Error::HsePinConfigurationTimeout)?;\nif info.reset_asserted() || !info.is_enabled() { return Err(crate::rcc::Error::HsePinInUse); }\nlet r = crate::pac::GPIOA;\n");
        for (index, pin) in pins.iter().enumerate() {
            let (_, bit) = pin.unwrap();
            let (lock, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if index == 1 { out.push_str("if !bypass {\n"); }
            if let Some(lock) = lock {
                writeln!(out, "r.{lock}().modify(|w| {{ w.set_key(0x5a5a); w.set_pin{bit}(false); }});").unwrap();
            }
            writeln!(out, "r.dir().modify(|w| w.set_pin{bit}(true));").unwrap();
            for reg in registers {
                writeln!(out, "r.{reg}().modify(|w| w.set_pin{bit}(false));").unwrap();
            }
            let afreg = if bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "r.{afreg}().modify(|w| w.set_{af_field}(0));").unwrap();
            writeln!(out, "r.analog().modify(|w| w.set_pin{bit}({}));", if index == 0 { "!bypass" } else { "true" }).unwrap();
            if index == 1 { out.push_str("}\n"); }
        }
        out.push_str("if info.reset_asserted() || !info.is_enabled() { return Err(crate::rcc::Error::HsePinInUse); }\n} else {\n");
    }
    if l012_live {
        out.push_str("if target == crate::rcc::L012HseTarget::LiveLsiSysclk {\nlet info = gpio_rcc(5);\nif info.reset_asserted() { return Err(crate::rcc::Error::HseClockInUse); }\ninfo.enable_with_cs_readback(cs, crate::rcc::Readback::Poll { attempts: timeout, spin: true }).map_err(|_| crate::rcc::Error::HsePinConfigurationTimeout)?;\nif info.reset_asserted() || !info.is_enabled() { return Err(crate::rcc::Error::HseClockInUse); }\nlet r = crate::pac::GPIOF;\n");
        for (index, pin) in pins.iter().enumerate() {
            let (_, bit) = pin.unwrap();
            let (lock, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if index == 1 { out.push_str("if !bypass {\n"); }
            if let Some(lock) = lock {
                writeln!(out, "r.{lock}().modify(|w| {{ w.set_key(0x5a5a); w.set_pin{bit}(false); }});").unwrap();
            }
            writeln!(out, "r.dir().modify(|w| w.set_pin{bit}(true));").unwrap();
            for reg in registers {
                writeln!(out, "r.{reg}().modify(|w| w.set_pin{bit}(false));").unwrap();
            }
            let afreg = if bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "r.{afreg}().modify(|w| w.set_{af_field}(0));").unwrap();
            writeln!(out, "r.analog().modify(|w| w.set_pin{bit}({}));", if index == 0 { "!bypass" } else { "true" }).unwrap();
            if index == 1 { out.push_str("}\n"); }
        }
        out.push_str("if info.reset_asserted() || !info.is_enabled() { return Err(crate::rcc::Error::HseClockInUse); }\n} else {\n");
    }
    for (index, pin) in pins.iter().enumerate() {
        if index == 1 {
            out.push_str("if !bypass {\n");
        }
        if let Some((port, bit)) = pin {
            let p = char::from(b'A' + port);
            let (lock, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if l012_target {
                out.push_str(target_faults);
                writeln!(out, "if target == crate::rcc::L012HseTarget::LseSysclk && gpio_rcc({port}).reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
            }
            writeln!(out, "gpio_rcc({port}).enable_with_cs_readback(cs, crate::rcc::Readback::Poll {{ attempts: timeout, spin: true }}).map_err(|_| crate::rcc::Error::HsePinConfigurationTimeout)?;\nlet r = crate::pac::GPIO{p};").unwrap();
            if l012_target {
                out.push_str(target_faults);
                writeln!(out, "if target == crate::rcc::L012HseTarget::LseSysclk && (gpio_rcc({port}).reset_asserted() || !gpio_rcc({port}).is_enabled()) {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
            }
            if let Some(lock) = lock {
                writeln!(
                    out,
                    "r.{lock}().modify(|w| {{ w.set_key(0x5a5a); w.set_pin{bit}(false); }});"
                )
                .unwrap();
            }
            // Disable the driver first; then pulls, edge/level IRQs, open drain
            // and filter, and finally the analog/digital input selector. Each
            // RMW preserves unrelated pads and the shared filter-clock field.
            writeln!(out, "r.dir().modify(|w| w.set_pin{bit}(true));").unwrap();
            for reg in registers {
                writeln!(out, "r.{reg}().modify(|w| w.set_pin{bit}(false));").unwrap();
            }
            let afreg = if *bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "r.{afreg}().modify(|w| w.set_{af_field}(0));").unwrap();
            writeln!(
                out,
                "r.analog().modify(|w| w.set_pin{bit}({}));",
                if index == 0 { "!bypass" } else { "true" }
            )
            .unwrap();
            if l012_target {
                out.push_str(target_faults);
                writeln!(out, "if target == crate::rcc::L012HseTarget::LseSysclk && (gpio_rcc({port}).reset_asserted() || !gpio_rcc({port}).is_enabled()) {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
            }
        } else {
            out.push_str("return Err(crate::rcc::Error::HsePinsUnavailable);\n");
        }
        if index == 1 {
            out.push_str("}\n");
        }
    }
    if l010_live || l012_live { out.push_str("}\n"); }
    let mismatch = if l010_live {
        "if lsi_sysclk { crate::rcc::Error::ClockConfigurationTimeout } else { crate::rcc::Error::HsePinConfigurationTimeout }"
    } else if l012_live {
        "if target == crate::rcc::L012HseTarget::LiveLsiSysclk { crate::rcc::Error::ClockConfigurationTimeout } else { crate::rcc::Error::HsePinConfigurationTimeout }"
    } else {
        "crate::rcc::Error::HsePinConfigurationTimeout"
    };
    writeln!(out, "if !rcc_hse_pins_match(bypass, timeout, cs{forwarded})? {{ return Err({mismatch}); }}\nOk(())\n}}").unwrap();
    writeln!(out, "pub(crate) fn rcc_hse_pins_match(bypass: bool, timeout: u32, cs: critical_section::CriticalSection<'_>{matcher_argument}) -> Result<bool, crate::rcc::Error> {{").unwrap();
    if l010_live {
        out.push_str("if lsi_sysclk {\nlet info = gpio_rcc(0);\nif info.reset_asserted() { return Err(crate::rcc::Error::HsePinInUse); }\nlet result = info.inspect_for_init(cs, timeout, || {\nlet operational_before = info.is_enabled() && !info.reset_asserted();\nlet r = crate::pac::GPIOA;\nlet mut matches = true;\n");
        for (index, pin) in pins.iter().enumerate() {
            let (_, bit) = pin.unwrap();
            let (_, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if index == 1 { out.push_str("if !bypass {\n"); }
            let afreg = if bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "matches &= r.dir().read().pin{bit}() && r.analog().read().pin{bit}() == {} && r.{afreg}().read().{af_field}() == 0", if index == 0 { "!bypass" } else { "true" }).unwrap();
            for reg in registers { writeln!(out, "&& !r.{reg}().read().pin{bit}()").unwrap(); }
            out.push_str(";\n");
            if index == 1 { out.push_str("}\n"); }
        }
        out.push_str("(operational_before && info.is_enabled() && !info.reset_asserted(), matches)\n}).map_err(|error| match error {\ncrate::rcc::ClockInspectionError::EnableFailed { restore_failed } => crate::rcc::Error::LsiConfigurationGateTimeout { enable_failed: true, restore_failed },\ncrate::rcc::ClockInspectionError::RestoreFailed => crate::rcc::Error::LsiConfigurationGateTimeout { enable_failed: false, restore_failed: true },\n})?;\nif !result.0 || info.reset_asserted() { return Err(crate::rcc::Error::HsePinInUse); }\nreturn Ok(result.1);\n}\n");
    }
    if l012_live {
        out.push_str("if target == crate::rcc::L012HseTarget::LiveLsiSysclk {\nlet info = gpio_rcc(5);\nif info.reset_asserted() { return Err(crate::rcc::Error::HseClockInUse); }\nlet result = info.inspect_for_init(cs, timeout, || {\nif !info.is_enabled() || info.reset_asserted() { return (false, false); }\nlet r = crate::pac::GPIOF;\nlet mut matches = true;\n");
        for (index, pin) in pins.iter().enumerate() {
            let (_, bit) = pin.unwrap();
            let (_, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if index == 1 { out.push_str("if !bypass {\n"); }
            let afreg = if bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "matches &= r.dir().read().pin{bit}() && r.analog().read().pin{bit}() == {} && r.{afreg}().read().{af_field}() == 0", if index == 0 { "!bypass" } else { "true" }).unwrap();
            for reg in registers { writeln!(out, "&& !r.{reg}().read().pin{bit}()").unwrap(); }
            out.push_str(";\n");
            if index == 1 { out.push_str("}\n"); }
        }
        out.push_str("(info.is_enabled() && !info.reset_asserted(), matches)\n}).map_err(|error| match error {\ncrate::rcc::ClockInspectionError::EnableFailed { restore_failed } => crate::rcc::Error::LsiConfigurationGateTimeout { enable_failed: true, restore_failed },\ncrate::rcc::ClockInspectionError::RestoreFailed => crate::rcc::Error::LsiConfigurationGateTimeout { enable_failed: false, restore_failed: true },\n})?;\nif !result.0 || info.reset_asserted() { return Err(crate::rcc::Error::HseClockInUse); }\nreturn Ok(result.1);\n}\n");
    }
    out.push_str("let mut matches = true;\n");
    for (index, pin) in pins.iter().enumerate() {
        if index == 1 {
            out.push_str("if !bypass {\n");
        }
        if let Some((port, bit)) = pin {
            let p = char::from(b'A' + port);
            let (_, registers, af_field) = pad_registers[index].as_ref().unwrap();
            if l012_target {
                out.push_str(target_faults);
                writeln!(out, "if target == crate::rcc::L012HseTarget::LseSysclk && gpio_rcc({port}).reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
                writeln!(out, "let result = gpio_rcc({port}).inspect_for_init(cs, timeout, || {{\nif target == crate::rcc::L012HseTarget::LseSysclk && (gpio_rcc({port}).reset_asserted() || !gpio_rcc({port}).is_enabled()) {{ return None; }}\nlet r = crate::pac::GPIO{p};\nlet pad_matches =").unwrap();
            } else {
                writeln!(out, "matches &= gpio_rcc({port}).inspect_for_init(cs, timeout, || {{\nlet r = crate::pac::GPIO{p};").unwrap();
            }
            let afreg = if *bit < 8 { "afrl" } else { "afrh" };
            writeln!(out, "r.dir().read().pin{bit}() && r.analog().read().pin{bit}() == {} && r.{afreg}().read().{af_field}() == 0", if index == 0 { "!bypass" } else { "true" }).unwrap();
            for reg in registers {
                writeln!(out, "&& !r.{reg}().read().pin{bit}()").unwrap();
            }
            if l012_target {
                writeln!(out, ";\nif target == crate::rcc::L012HseTarget::LseSysclk && (gpio_rcc({port}).reset_asserted() || !gpio_rcc({port}).is_enabled()) {{ return None; }}\nSome(pad_matches)\n}}).map_err(|error| if target == crate::rcc::L012HseTarget::LseSysclk {{ crate::rcc::lse_target_inspection_error(error) }} else {{ crate::rcc::Error::HsePinConfigurationTimeout }})?;").unwrap();
                out.push_str(target_faults);
                writeln!(out, "if target == crate::rcc::L012HseTarget::LseSysclk && gpio_rcc({port}).reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nmatches &= result.ok_or(crate::rcc::Error::LseClockInUse)?;").unwrap();
            } else {
                out.push_str("}).map_err(|_| crate::rcc::Error::HsePinConfigurationTimeout)?;\n");
            }
        } else {
            out.push_str("return Ok(false);\n");
        }
        if index == 1 {
            out.push_str("}\n");
        }
    }
    out.push_str("Ok(matches)\n}\n");
}

// Direct digital HEX pads have their own F002/F003 projection. Their GPIO
// block has no LOCK register; do not reuse the crystal/HSE pad programming.
fn generate_hex(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use std::fmt::Write;
    let Some(hex) = &c.hex else { return };
    writeln!(
        out,
        "pub(crate) const RCC_HEX_RANGE_HZ: (u32, u32) = {:?};",
        (hex.input.minimum_hz, hex.input.maximum_hz)
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HEX_SUPPLY_MV: (u16, u16) = {:?};",
        hex.input.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_HEX_TEMPERATURE_C: (i16, i16) = {:?};",
        hex.input.temperature_c
    )
    .unwrap();
    let sysctrl = cw32_metapac::metadata::METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap();
    let mut pins = [None; 2];
    for (index, signal) in ["HEX_PB00", "HEX_PB01"].iter().enumerate() {
        let found: Vec<_> = sysctrl
            .pins
            .iter()
            .filter(|p| p.signal == *signal)
            .collect();
        assert!(found.len() <= 1, "ambiguous HEX pad");
        if let Some(pin) = found.first() {
            assert!(
                pin.af.is_none(),
                "HEX input must not be a numbered digital AF"
            );
            assert_eq!(pin.pin, format!("PB{index}"));
            pins[index] = Some(16 + index as u8);
        }
    }
    writeln!(
        out,
        "pub(crate) const RCC_HEX_PINS: [Option<u8>; 2] = {:?};",
        pins
    )
    .unwrap();
    out.push_str("pub(crate) fn rcc_configure_hex_pin(input: crate::rcc::HexInput, timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<(), crate::rcc::Error> {\nmatch input {\n");
    for (index, pin) in pins.iter().enumerate() {
        writeln!(out, "crate::rcc::HexInput::Pb{index} => {{").unwrap();
        if pin.is_some() {
            out.push_str("gpio_rcc(1).enable_with_cs_readback(cs, crate::rcc::Readback::Poll { attempts: timeout, spin: true }).map_err(|_| crate::rcc::Error::HexPinConfigurationTimeout)?;\nlet r = crate::pac::GPIOB;\n");
            // Disable output first, then pad-local features, then digital input.
            // RMW preserves every unrelated pad and the shared filter clock.
            for (reg, value) in [
                ("dir", true),
                ("pur", false),
                ("pdr", false),
                ("riseie", false),
                ("fallie", false),
                ("highie", false),
                ("lowie", false),
                ("opendrain", false),
                ("filter", false),
            ] {
                writeln!(out, "r.{reg}().modify(|w| w.set_pin{index}({value}));").unwrap();
            }
            writeln!(out, "r.afrl().modify(|w| w.set_afr{index}(0));\nr.analog().modify(|w| w.set_pin{index}(false));").unwrap();
        } else {
            out.push_str("return Err(crate::rcc::Error::HexPinUnavailable);\n");
        }
        out.push_str("}\n");
    }
    out.push_str("}\nfor _ in 0..timeout { if rcc_hex_pin_matches(input, timeout, cs)? { return Ok(()); } core::hint::spin_loop(); }\nErr(crate::rcc::Error::HexPinConfigurationTimeout)\n}\n");
    out.push_str("pub(crate) fn rcc_hex_pin_matches(input: crate::rcc::HexInput, timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, crate::rcc::Error> {\nmatch input {\n");
    for (index, pin) in pins.iter().enumerate() {
        writeln!(out, "crate::rcc::HexInput::Pb{index} => {{").unwrap();
        if pin.is_some() {
            out.push_str(
                "gpio_rcc(1).inspect_for_init(cs, timeout, || { let r = crate::pac::GPIOB;\n",
            );
            writeln!(out, "r.dir().read().pin{index}() && !r.analog().read().pin{index}() && r.afrl().read().afr{index}() == 0").unwrap();
            for reg in [
                "pur",
                "pdr",
                "riseie",
                "fallie",
                "highie",
                "lowie",
                "opendrain",
                "filter",
            ] {
                writeln!(out, "&& !r.{reg}().read().pin{index}()").unwrap();
            }
            out.push_str("}).map_err(|_| crate::rcc::Error::HexPinConfigurationTimeout)\n");
        } else {
            out.push_str("Ok(false)\n");
        }
        out.push_str("}\n");
    }
    out.push_str("}\n}\n");
}

fn generate_rcc(out: &mut String, time_driver: Option<&str>) {
    use cw32_metapac::metadata::{METADATA, PeripheralRccKernelClock};
    use std::collections::BTreeMap;
    let sysctrl = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap()
        .registers
        .as_ref()
        .unwrap();
    let mut enable_users = BTreeMap::new();
    let mut reset_users = BTreeMap::new();
    for peripheral in METADATA.peripherals {
        if let Some(control) = &peripheral.rcc_control {
            *enable_users
                .entry((control.enable.register, control.enable.field))
                .or_insert(0usize) += 1;
            if let Some(reset) = &control.reset {
                *reset_users
                    .entry((reset.register, reset.field))
                    .or_insert(0usize) += 1;
            }
        }
    }
    for peripheral in METADATA.peripherals {
        let Some(control) = &peripheral.rcc_control else {
            continue;
        };
        assert_eq!(control.controller, "SYSCTRL");
        let (enable_offset, enable_bit, width) = rcc_field(sysctrl, &control.enable);
        assert_eq!(width, 1);
        let reset = control.reset.as_ref().map(|reset| {
            let (offset, bit, width) = rcc_field(sysctrl, reset);
            assert_eq!(width, 1);
            (offset, bit)
        });
        assert_eq!(reset.is_some(), control.reset_asserted_value.is_some());
        let key = control.enable_write_key.as_ref().map(|key| {
            assert!(
                key.field
                    .register
                    .eq_ignore_ascii_case(control.enable.register)
            );
            let (offset, bit, width) = rcc_field(sysctrl, &key.field);
            assert_eq!(offset, enable_offset);
            assert!(u32::from(bit) + width <= 32);
            assert!(width == 32 || key.value < (1u32 << width));
            let mask = (u32::MAX >> (32 - width)) << bit;
            assert_eq!(mask & (1 << enable_bit), 0);
            (mask, key.value << bit)
        });
        if let Some(rcc) = &peripheral.rcc {
            let enable = rcc.enable.as_ref().unwrap();
            assert!(
                enable
                    .register
                    .eq_ignore_ascii_case(control.enable.register)
            );
            assert!(enable.field.eq_ignore_ascii_case(control.enable.field));
            assert_eq!(
                rcc.reset.as_ref().map(|r| rcc_field(sysctrl, r)),
                control.reset.as_ref().map(|r| rcc_field(sysctrl, r))
            );
        }
        let registers = peripheral.registers.as_ref().unwrap();
        let kind = registers.kind;
        let version = registers.version;
        // Readback and lifetime behavior are HAL policy, not invented hardware
        // facts. Retain original per-driver limits and error behavior.
        let policy_kind = if peripheral.name.starts_with("GPIO") {
            "gpio"
        } else {
            kind
        };
        let (enable_readback, disable_readback, reset_readback) = match policy_kind {
            "uart" => (
                "Poll { attempts: 100_000, spin: false }",
                "Poll { attempts: 100_000, spin: false }",
                false,
            ),
            "spi" => ("Poll { attempts: 100_000, spin: true }", "None", true),
            "i2c" if version == "cw32l012_v1" => {
                ("Poll { attempts: 32, spin: true }", "Once", true)
            }
            "i2c" => ("Poll { attempts: 100_000, spin: true }", "None", true),
            "gtim" | "atim" => ("Once", "Once", true),
            "gpio" if matches!(version, "cw32l010_v1" | "cw32l011_v1") => {
                ("Poll { attempts: 100_000, spin: true }", "None", false)
            }
            "adc" => ("Once", "Once", true),
            _ => ("Once", "None", false),
        };
        let independently_owned = matches!(
            kind,
            "uart" | "spi" | "i2c" | "gtim" | "atim" | "adc" | "lptim" | "halltim" | "autotrim"
        );
        let low_adc =
            kind == "adc" && matches!(version, "cw32l010_v1" | "cw32l011_v1" | "cw32l012_v1");
        let shared_enable = control.shared_enable_group.is_some()
            || enable_users[&(control.enable.register, control.enable.field)] > 1;
        let shared_reset = control.shared_reset_group.is_some()
            || control
                .reset
                .as_ref()
                .is_some_and(|r| reset_users[&(r.register, r.field)] > 1);
        // Shared reset/clock groups may contain bootloader or analog owners
        // outside HAL accounting. A first/last-user refcount is insufficient.
        let allow_reset = independently_owned && !low_adc && !shared_reset;
        let allow_disable = (independently_owned
            || matches!(kind, "crc" | "cordic" | "eau" | "aes" | "trng"))
            && !shared_enable;
        if matches!(
            METADATA.name,
            "CW32F030C8T7" | "CW32F030K8T7" | "CW32F030K8U7" | "CW32A030C8T7"
        ) && peripheral.name == "GPIOA"
        {
            // MCO relies on this actual classic Flex gate policy. Once is a
            // barrier, not an acknowledgement poll; the leaf checks the gate.
            assert_eq!(version, "v1");
            assert_eq!((enable_readback, disable_readback, reset_readback), ("Once", "None", false));
            assert!(!allow_reset && !allow_disable);
        }
        let bus = match control.bus_clock {
            "PCLK" => "pclk",
            "HCLK" => "hclk",
            other => panic!("unqualified RCC bus {other}"),
        };
        let kernel = match peripheral.rcc.as_ref().map(|rcc| &rcc.kernel_clock) {
            Some(PeripheralRccKernelClock::Clock("PCLK")) => Some("pclk"),
            Some(PeripheralRccKernelClock::Clock("HCLK")) => Some("hclk"),
            _ => None,
        };
        let frequency = kernel
            .map(|clock| format!("crate::rcc::try_clocks().map(|c| c.{clock})"))
            .unwrap_or_else(|| "None".into());
        let kernel_bounds = kernel
            .map(|clock| format!("crate::rcc::try_clocks().map(|c| c.{clock}_bounds())"))
            .unwrap_or_else(|| "None".into());
        let name = if Some(peripheral.name) == time_driver {
            "TimeDriverPeripheral".to_string()
        } else {
            format!("peripherals::{}", peripheral.name)
        };
        out.push_str(&format!(
            "impl crate::rcc::SealedRccPeripheral for {name} {{\n\
             const RCC_INFO: crate::rcc::RccInfo = unsafe {{ crate::rcc::RccInfo::new(\n\
             ({enable_offset}, {enable_bit}), {reset:?}, {key:?}, {}, {},\n\
             crate::rcc::RccPolicy {{ enable_readback: crate::rcc::Readback::{enable_readback},\n\
             disable_readback: crate::rcc::Readback::{disable_readback}, reset_readback: {reset_readback},\n\
             allow_reset: {allow_reset}, allow_disable: {allow_disable} }}) }};\n\
             fn bus_frequency() -> Option<crate::time::Hertz> {{ crate::rcc::try_clocks().map(|c| c.{bus}) }}\n\
             fn bus_clock_bounds() -> Option<crate::rcc::ClockBounds> {{ crate::rcc::try_clocks().map(|c| c.{bus}_bounds()) }}\n\
             fn frequency() -> Option<crate::time::Hertz> {{ {frequency} }}\n\
             fn kernel_clock_bounds() -> Option<crate::rcc::ClockBounds> {{ {kernel_bounds} }}\n\
             }}\nimpl crate::rcc::RccPeripheral for {name} {{}}\n",
            control.enable_active_value, control.reset_asserted_value.unwrap_or(false),
        ));
    }
}

fn rcc_field(
    sysctrl: &cw32_metapac::metadata::PeripheralRegisters,
    reference: &cw32_metapac::metadata::PeripheralRccRegister,
) -> (u8, u8, u32) {
    use cw32_metapac::metadata::ir;
    let block = sysctrl
        .ir
        .blocks
        .iter()
        .find(|b| b.name == sysctrl.block)
        .unwrap();
    let item = block
        .items
        .iter()
        .find(|r| r.name.eq_ignore_ascii_case(reference.register))
        .unwrap();
    assert!(item.array.is_none());
    assert_eq!(item.byte_offset % 4, 0);
    let ir::BlockItemInner::Register(register) = &item.inner else {
        panic!("RCC control is not a register")
    };
    assert_eq!(register.access, ir::Access::ReadWrite);
    assert_eq!(register.bit_size, 32);
    let fields = sysctrl
        .ir
        .fieldsets
        .iter()
        .find(|f| Some(f.name) == register.fieldset)
        .unwrap();
    let field = fields
        .fields
        .iter()
        .find(|f| f.name.eq_ignore_ascii_case(reference.field))
        .unwrap();
    assert!(field.array.is_none());
    let ir::BitOffset::Regular(position) = &field.bit_offset else {
        panic!("nonlinear RCC field")
    };
    assert!(field.bit_size > 0 && position.offset + field.bit_size <= 32);
    (
        (item.byte_offset / 4).try_into().unwrap(),
        position.offset.try_into().unwrap(),
        field.bit_size,
    )
}

// Inclusive Q1.31 bounds are rounded inward by data-gen from own-manual rationals.
fn generate_accelerator_domains() {
    let facts = cw32_metapac::metadata::METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "CORDIC")
        .unwrap()
        .cordic
        .as_ref()
        .expect("missing CORDIC numeric domains");
    let mut out = String::new();
    for domain in facts.domains {
        let (name, minimum, maximum) = (domain.name, domain.minimum, domain.maximum);
        out.push_str(&format!(
            "pub(super) const {name}: (i32, i32) = ({minimum}, {maximum});\n"
        ));
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("_cordic_domains.rs"),
        out,
    )
    .unwrap();
}

// The extra IR token, where needed, represents SYSCTRL.IRMOD itself. It is
// generated from reviewed real-register metadata, not a fabricated peripheral.
fn generate_lvd_ir(out: &mut String) {
    use cw32_metapac::metadata::METADATA;
    let lvd = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "LVD")
        .unwrap();
    let facts = lvd
        .lvd
        .as_ref()
        .expect("missing LVD own-family qualification");
    let version = lvd.registers.as_ref().unwrap().version;
    assert!(matches!(version, "v1" | "cw32l031_v1" | "cw32l010_v1"));
    if version == "cw32l010_v1" {
        let vector = METADATA
            .interrupts
            .iter()
            .find(|irq| irq.name == "LVD")
            .expect("native LVD requires its dedicated vector");
        assert_eq!(vector.number, 1, "unexpected native LVD vector number");
        println!("cargo:rustc-cfg=lvd_low");
    }
    out.push_str(&format!(
        "pub(crate) const LVD_THRESHOLDS_MV: &[u16] = &{:?};\n",
        facts.thresholds_mv
    ));
    out.push_str(&format!(
        "pub(crate) const LVD_SUPPLY_NAME: &str = {:?};\n",
        facts.supply_name
    ));
    for input in facts.inputs {
        if safe_pin(input.pin) {
            out.push_str(&format!(
                "crate::lvd::impl_input_pin!({}, {});\n",
                input.pin, input.selector
            ));
        }
    }
    let p = METADATA
        .peripherals
        .iter()
        .find(|p| p.ir.is_some())
        .expect("missing IR subfunction");
    let ir = p.ir.as_ref().unwrap();
    if p.name == "SYSCTRL" {
        println!("cargo:rustc-cfg=ir_sysctrl");
    }
    if ir.mode_configurable {
        println!("cargo:rustc-cfg=ir_modes");
    }
    if ir.software_control {
        println!("cargo:rustc-cfg=ir_software");
    }
    if ir.invert {
        println!("cargo:rustc-cfg=ir_invert");
    }
    for pin in p
        .pins
        .iter()
        .filter(|p| p.signal == "IR_OUT" && safe_pin(p.pin))
    {
        out.push_str(&format!(
            "crate::ir::impl_output_pin!({}, {});\n",
            pin.pin,
            pin.af.unwrap()
        ));
    }
}

// Own-L083 source-backed geometry; typed commands remain in the authored PAC IR.
fn generate_crypto_facts() {
    let mut out = String::new();
    for name in ["AES", "TRNG"] {
        let peripheral = cw32_metapac::metadata::METADATA
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .unwrap();
        let fields: Vec<(&str, u8)> = if name == "AES" {
            let facts = peripheral.aes.as_ref().expect("missing AES geometry");
            vec![
                ("block_words", facts.block_words),
                ("key_words_128", facts.key_words_128),
                ("key_words_192", facts.key_words_192),
                ("key_words_256", facts.key_words_256),
            ]
        } else {
            let facts = peripheral.trng.as_ref().expect("missing TRNG geometry");
            vec![("output_words", facts.output_words)]
        };
        for (field, value) in fields {
            out.push_str(&format!(
                "pub const {}_{}: usize = {value};\n",
                name,
                field.to_ascii_uppercase()
            ));
        }
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("_crypto.rs"),
        out,
    )
    .unwrap();
}

// Emit only the meaningful register-layout capability; no synthetic RAM gate.
fn configure_ram() {
    use cw32_metapac::metadata::METADATA;
    println!("cargo:rustc-check-cfg=cfg(ram_enable_status)");
    let ram = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "RAM")
        .unwrap();
    let regs = ram.registers.as_ref().unwrap();
    assert!(ram.rcc.is_none() && ram.rcc_control.is_none());
    let fields = regs
        .ir
        .fieldsets
        .iter()
        .find(|f| f.name == "Ier")
        .unwrap()
        .fields;
    let has_en = fields.iter().any(|f| f.name == "en");
    assert_eq!(
        has_en,
        ram.ram_parity
            .as_ref()
            .expect("missing RAM parity facts")
            .enable_status
    );
    if has_en {
        println!("cargo:rustc-cfg=ram_enable_status");
    }
}

// Derive LCD register access and pin routes from reviewed chip facts and typed IR.
fn generate_lcd(out: &mut String) {
    use cw32_metapac::metadata::{METADATA, ir};
    use std::fmt::Write;
    let Some(p) = METADATA.peripherals.iter().find(|p| p.name == "LCD") else {
        return;
    };
    let facts = p.lcd.as_ref().unwrap();
    let regs = p.registers.as_ref().unwrap();
    writeln!(out, "impl crate::lcd::sealed::Instance for crate::peripherals::LCD {{ const RAM_REGISTERS: &'static [u8] = &{:?}; const LSI_TYPICAL_HZ: u32 = {}; }} impl crate::lcd::Instance for crate::peripherals::LCD {{}}", facts.ram_registers, facts.lsi_typical_hz).unwrap();
    for pin in p.pins.iter().filter(|pin| safe_pin(pin.pin)) {
        if let Some(n) = pin.signal.strip_prefix("COM") {
            writeln!(out,"impl crate::lcd::sealed::ComPin<crate::peripherals::LCD, {n}> for crate::peripherals::{} {{}} impl crate::lcd::ComPin<crate::peripherals::LCD, {n}> for crate::peripherals::{} {{}}",pin.pin,pin.pin).unwrap();
        } else if let Some(n) = pin.signal.strip_prefix("SEG") {
            writeln!(out,"impl crate::lcd::sealed::SegPin<crate::peripherals::LCD, {n}> for crate::peripherals::{} {{}} impl crate::lcd::SegPin<crate::peripherals::LCD, {n}> for crate::peripherals::{} {{}}",pin.pin,pin.pin).unwrap();
        }
    }
    let block = regs
        .ir
        .blocks
        .iter()
        .find(|b| b.name == regs.block)
        .unwrap();
    let clear = block.items.iter().find(|r| r.byte_offset == 8).unwrap();
    let ir::BlockItemInner::Register(clear_reg) = &clear.inner else {
        panic!("LCD event clear must be register")
    };
    let clear_type = clear_reg.fieldset.unwrap();
    assert!(matches!(clear_type, "Icr" | "Intclr"));
    writeln!(out, "pub(crate) fn lcd_clear_frame() {{ let mut command=crate::pac::lcd::regs::{clear_type}::write_noop();command.set_intf(false);crate::pac::LCD.{}().write_value(command); }}",clear.name.to_ascii_lowercase()).unwrap();
    out.push_str("pub(crate) fn lcd_write_word(index: u8, pixels: u32) { match index {\n");
    for n in facts.ram_registers {
        let name = format!("RAM{n}");
        let fieldset = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(&name))
            .unwrap();
        assert_eq!(fieldset.fields.len(), 32);
        writeln!(out, "{n} => crate::pac::LCD.ram{n}().write(|w| {{").unwrap();
        for f in fieldset.fields {
            assert_eq!(f.bit_size, 1);
            let ir::BitOffset::Regular(ref offset) = f.bit_offset else {
                panic!("LCD pixel must have regular offset")
            };
            writeln!(
                out,
                "w.set_{}(pixels & (1u32 << {}) != 0);",
                f.name.to_ascii_lowercase(),
                offset.offset
            )
            .unwrap();
        }
        out.push_str("}),\n");
    }
    out.push_str("_ => unreachable!(\"unsupported LCD RAM register\"), } }\n");
    out.push_str(
        "pub(crate) fn lcd_enable_pin(is_com: bool, number: u8) { match (is_com, number) {\n",
    );
    for (is_com, number, register, bit) in facts
        .segments
        .iter()
        .map(|n| {
            (
                false,
                *n,
                if *n < 32 { "PINEN1" } else { "PINEN2" },
                u32::from(*n % 32),
            )
        })
        .chain((0..8u8).map(|n| {
            (
                true,
                n,
                if n < 4 { "PINEN2" } else { "PINEN1" },
                u32::from(if n < 4 { 24 + n } else { 35 - n }),
            )
        }))
    {
        let fieldset = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(register))
            .unwrap();
        let f=fieldset.fields.iter().find(|f|matches!(f.bit_offset,ir::BitOffset::Regular(ref o) if o.offset<=bit && o.offset+f.bit_size>bit)).expect("LCD pin bit missing");
        let ir::BitOffset::Regular(ref offset) = f.bit_offset else {
            unreachable!()
        };
        let field = f.name.to_ascii_lowercase();
        let value = if f.bit_size == 1 {
            "true".to_owned()
        } else {
            format!("w.{field}() | (1 << {})", bit - offset.offset)
        };
        writeln!(
            out,
            "({is_com}, {number}) => crate::pac::LCD.{}().modify(|w| w.set_{field}({value})),",
            register.to_ascii_lowercase()
        )
        .unwrap();
    }
    out.push_str("_ => unreachable!(\"unsupported LCD pin\"), } }\n");
}

// This RTC capability is independently qualified. A live-LSI SYSCLK profile,
// matching IP version, or runtime SYSCLK selection alone never grants it.
fn generate_native_live_lsi_rtc(out: &mut String) {
    use cw32_metapac::metadata::{ir, METADATA};
    use std::fmt::Write;
    let exact = match METADATA.name {
        "CW32L010F8P6" => Some(("CW32L010", "TSSOP20")),
        "CW32L010F8U6" => Some(("CW32L010", "QFN20")),
        "CW32L010Y8M6" => Some(("CW32L010", "SOP16")),
        "CW32L011K8T6" => Some(("CW32L011", "LQFP32")),
        "CW32L011K8U6" => Some(("CW32L011", "QFN32")),
        "CW32L012C8T6" => Some(("CW32L012", "LQFP48")),
        "CW32L012C8U6" => Some(("CW32L012", "QFN48")),
        _ => None,
    };
    let native_ip = METADATA.peripherals.iter().filter_map(|p| p.registers.as_ref()).any(|r|
        matches!(r.kind, "rtc" | "sysctrl")
            && matches!(r.version, "cw32l010_v1" | "cw32l011_v1" | "cw32l012_v1"));
    let family = match METADATA.line {
        "CW32L010" => Some(("cw32l010_v1", 10, 31816, 33784, (1620, 5500))),
        "CW32L011" => Some(("cw32l011_v1", 10, 29520, 41000, (1700, 5500))),
        "CW32L012" => Some(("cw32l012_v1", 9, 29520, 36080, (1700, 5500))),
        _ => None,
    };
    if family.is_none() && exact.is_none() && !native_ip { return; }
    let (version, trim, minimum, maximum, supply) = family.expect("native RTC IP/part family mismatch");
    assert!(native_ip);
    assert_eq!(METADATA.family, "CW32L0");
    assert!(exact.is_some_and(|(line, _)| line == METADATA.line) || METADATA.name == METADATA.line,
        "native RTC family/name mismatch");
    // Packages are deliberately absent from the transported metadata model.
    // Validate the current generated chip input without adding a model field.
    let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../cw32-data/data/chips").join(format!("{}.json", METADATA.name));
    println!("cargo:rerun-if-changed={}", path.display());
    let chip: serde_json::Value = serde_json::from_slice(&fs::read(&path)
        .expect("native RTC current chip JSON must exist")).expect("invalid native RTC chip JSON");
    assert_eq!(chip["name"], METADATA.name);
    assert_eq!(chip["line"], METADATA.line);
    assert_eq!(chip["family"], METADATA.family);
    let packages = chip["packages"].as_array().expect("native RTC package inventory required");
    if let Some((_, package)) = exact {
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0]["name"], METADATA.name);
        assert_eq!(packages[0]["package"], package);
    } else {
        assert!(packages.is_empty(), "generic native RTC must remain package-neutral");
    }
    let owner = |name, kind, block, address| {
        let owners: Vec<_> = METADATA.peripherals.iter().filter(|p| p.name == name).collect();
        assert_eq!(owners.len(), 1, "native RTC owner must be unique");
        let p = owners[0];
        let r = p.registers.as_ref().expect("native RTC registers required");
        assert_eq!(p.address, address);
        assert_eq!((r.kind, r.version, r.block), (kind, version, block));
        p
    };
    let rtc = owner("RTC", "rtc", "Rtc", 0x40004400);
    let sysctrl = owner("SYSCTRL", "sysctrl", "Sysctrl", 0x40004000);
    assert_eq!(METADATA.peripherals.iter().filter(|p| p.clock_limits.is_some()).count(), 1);
    let clock = sysctrl.clock_limits.as_ref().expect("native RTC oscillator facts required");
    assert_eq!(clock.lsi_sysclk.is_some(), exact.is_some());
    assert_eq!(clock.lse_configuration.is_some(), exact.is_some(), "native source enum requires existing rcc_lse admission");
    let old = rtc.rtc_calendar.as_ref().unwrap();
    assert_eq!((old.source, old.source_encoding), ("HSIOSC", 3));
    assert_eq!(old.factory_trim_address, clock.factory_hsi_trim_address);
    assert_eq!(old.nominal_hz, clock.hsi_frequency_hz);
    assert_eq!(old.calendar_divisor, clock.hsi_frequency_hz);
    assert_eq!((old.prescaler_first, old.prescaler_second),
        (if METADATA.line == "CW32L010" { 60 } else { 120 }, 400000));
    let field = |p: &cw32_metapac::metadata::Peripheral, reg: &str, offset, name: &str, bit, width, access| {
        let regs = p.registers.as_ref().unwrap();
        let blocks: Vec<_> = regs.ir.blocks.iter().filter(|b| b.name == regs.block).collect();
        assert_eq!(blocks.len(), 1);
        let items: Vec<_> = blocks[0].items.iter().filter(|r| r.name.eq_ignore_ascii_case(reg)).collect();
        assert_eq!(items.len(), 1);
        let item = items[0];
        assert_eq!(item.byte_offset, offset);
        assert!(item.array.is_none());
        let ir::BlockItemInner::Register(r) = &item.inner else { panic!("native RTC control must be a register") };
        assert_eq!((r.bit_size, &r.access), (32, &access));
        let fieldsets: Vec<_> = regs.ir.fieldsets.iter().filter(|f| Some(f.name) == r.fieldset).collect();
        assert_eq!(fieldsets.len(), 1);
        let fields: Vec<_> = fieldsets[0].fields.iter().filter(|f| f.name.eq_ignore_ascii_case(name)).collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].bit_size, width);
        assert!(fields[0].array.is_none());
        if p.name == "RTC" && reg == "CR1" && name == "SOURCE" { assert!(fields[0].enumm.is_none()); }
        assert!(matches!(&fields[0].bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
    };
    for (reg, offset, name, bit, width) in [
        ("KEY", 0, "KEY", 0, 8), ("CR0", 4, "START", 7, 1), ("CR0", 4, "H24", 3, 1),
        ("CR1", 8, "SOURCE", 8, 3), ("CR1", 8, "WAIT", 2, 1),
        ("PSC", 64, "PSC1", 20, 8), ("PSC", 64, "PSC2", 0, 20),
        ("COMPCFR1", 16, "EN", 15, 1),
    ] {
        field(rtc, reg, offset, name, bit, width,
            if reg == "KEY" { ir::Access::Write } else { ir::Access::ReadWrite });
    }
    let cr1 = rtc.registers.as_ref().unwrap().ir.fieldsets.iter()
        .find(|f| f.name.eq_ignore_ascii_case("CR1")).unwrap();
    for (name, bit) in [("ACCESS", 0), ("WINDOW", 1)] {
        assert_eq!(cr1.fields.iter().any(|f| f.name.eq_ignore_ascii_case(name)), METADATA.line == "CW32L010");
        if METADATA.line == "CW32L010" { field(rtc, "CR1", 8, name, bit, 1, ir::Access::ReadWrite); }
    }
    for (reg, offset, name, bit, width) in [
        ("CR1", 4, "LSIEN", 3, 1), ("LSI", 32, "TRIM", 0, trim),
        ("LSI", 32, "WAITCYCLE", 10, 2), ("LSI", 32, "STABLE", 15, 1),
    ] { field(sysctrl, reg, offset, name, bit, width, ir::Access::ReadWrite); }
    let Some(lsi) = clock.lsi_sysclk.as_ref() else { return; };
    assert!(exact.is_some());
    assert_eq!((lsi.nominal_hz, lsi.minimum_hz, lsi.maximum_hz, lsi.supply_mv,
        lsi.temperature_c, lsi.factory_trim_address),
        (32800, minimum, maximum, supply, (-40, 85), 0x001007c2));
    assert_eq!(lsi.rtc_allowed_sources, &[0, 1, 2, 3]);
    assert!(lsi.maximum_hz <= 1_000_000);
    assert_eq!(lsi.nominal_hz, 1 * 16400 * 2);
    println!("cargo:rustc-cfg=rtc_native_live_lsi");
    writeln!(out, "pub(crate) const RCC_NATIVE_LSI_RTC_SOURCE: u8 = 2;").unwrap();
    writeln!(out, "pub(crate) const RCC_NATIVE_LSI_RTC_FIRST_DIVISOR: u16 = 1;").unwrap();
    writeln!(out, "pub(crate) const RCC_NATIVE_LSI_RTC_SECOND_DIVISOR: u32 = 16400;").unwrap();
    writeln!(out, "pub(crate) const RCC_NATIVE_LSI_RTC_CALENDAR_DIVISOR: u32 = 32800;").unwrap();
}

fn generate_rtc(out: &mut String) {
    use cw32_metapac::metadata::METADATA;
    use std::fmt::Write;
    let Some(peripheral) = METADATA.peripherals.iter().find(|p| p.name == "RTC") else {
        return;
    };
    let f = peripheral.rtc_calendar.as_ref().expect("RTC source facts");
    let source_is_lsi = match f.source {
        "LSI" => { assert_eq!(f.source_encoding, 2); true }
        "HSIOSC" => {
            assert_eq!(f.source_encoding, 3);
            let clock = METADATA.peripherals.iter().find(|p| p.name == "SYSCTRL")
                .unwrap().clock_limits.as_ref().unwrap();
            assert_eq!(f.factory_trim_address, clock.factory_hsi_trim_address);
            assert_eq!(f.nominal_hz, clock.hsi_frequency_hz);
            assert_eq!(f.minimum_hz, (u64::from(clock.hsi_frequency_hz) * u64::from(100 - clock.hsi_error_percent) / 100) as u32);
            assert_eq!(f.maximum_hz, (u64::from(clock.hsi_frequency_hz) * u64::from(100 + clock.hsi_error_percent) / 100) as u32);
            assert_eq!(f.supply_mv, clock.hsi_supply_mv);
            assert_eq!(f.temperature_c, clock.hsi_temperature_c);
            false
        }
        _ => panic!("unqualified RTC semantic source identity"),
    };
    writeln!(out, "impl crate::rtc::sealed::Instance for crate::peripherals::RTC {{ const SOURCE_IS_LSI: bool = {source_is_lsi}; const SOURCE: u8 = {}; const SOURCE_NOMINAL_HZ: u32 = {}; const SOURCE_MINIMUM_HZ: u32 = {}; const SOURCE_MAXIMUM_HZ: u32 = {}; const TEMPERATURE_C: (i16,i16) = {:?}; const SUPPLY_MV: (u16,u16) = {:?}; #[cfg(not(any(rtc_cw32l010_v1,rtc_cw32l011_v1,rtc_cw32l012_v1)))] const FACTORY_TRIM_ADDRESS: usize = {}; const CALENDAR_DIVISOR: u32 = {}; #[cfg(any(rtc_cw32l010_v1,rtc_cw32l011_v1,rtc_cw32l012_v1))] const PRESCALER_FIRST: u16 = {}; #[cfg(any(rtc_cw32l010_v1,rtc_cw32l011_v1,rtc_cw32l012_v1))] const PRESCALER_SECOND: u32 = {}; }}", f.source_encoding,f.nominal_hz,f.minimum_hz,f.maximum_hz,f.temperature_c,f.supply_mv,f.factory_trim_address,f.calendar_divisor,f.prescaler_first,f.prescaler_second).unwrap();
}

// Project source-event mappings into destination-local types, never a global ITR enum.
fn generate_trigger_routes() {
    use cw32_metapac::metadata::{METADATA, TriggerRegister};
    let routes: Vec<_> = METADATA
        .peripherals
        .iter()
        .flat_map(|p| p.triggers.iter().map(move |route| (p, route)))
        .collect();
    if routes.is_empty() {
        return;
    }
    let adc_version = match METADATA.line {
        "CW32L010" => "cw32l010_v1",
        "CW32L011" => "cw32l011_v1",
        _ => panic!("unqualified trigger family"),
    };
    assert_eq!(routes.len(), 2, "unqualified trigger set");
    let field = |actual: &TriggerRegister, owner, reg, name, role, offset, size, value| {
        assert_eq!(
            (
                actual.peripheral,
                actual.register,
                actual.field,
                actual.role,
                actual.bit_offset,
                actual.bit_size,
                actual.value
            ),
            (owner, reg, name, role, offset, size, value)
        );
    };
    let mut timer = None;
    let mut adc = None;
    for (destination, route) in routes {
        assert_eq!(
            destination.registers.as_ref().unwrap().version,
            if destination.name == "ADC" {
                adc_version
            } else {
                "cw32l010_v1"
            }
        );
        assert_eq!(route.source, "BTIM1_TRGO");
        assert_eq!(route.registers.len(), 2);
        field(
            &route.registers[0],
            "BTIM1",
            "CR2",
            "MMS",
            "source_event",
            4,
            3,
            2,
        );
        let source = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == "BTIM1")
            .unwrap();
        assert_eq!(source.registers.as_ref().unwrap().version, "cw32l010_v1");
        match destination.name {
            "BTIM2" => {
                assert!(timer.is_none(), "duplicate BTIM2 route");
                assert_eq!(route.signal, "TRGI");
                field(
                    &route.registers[1],
                    "BTIM2",
                    "SMCR",
                    "TRGISRC",
                    "selector",
                    7,
                    4,
                    8,
                );
                timer = Some(format!(
                    "pub(super) const BTIM1_UPDATE_MMS: u8 = {};\n\
                     /// Qualified BTIM2 TRGI source; values belong only to BTIM2.\n\
                     #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
                     #[repr(u8)]\n\
                     pub enum Btim2TriggerInput {{\n\
                     /// BTIM1 UPDATE emitted through TRGO.\n\
                     Btim1Update = {},\n}}\n",
                    route.registers[0].value, route.registers[1].value
                ));
            }
            "ADC" => {
                assert!(adc.is_none(), "duplicate ADC route");
                assert_eq!(route.signal, "START_CONVERSION");
                field(
                    &route.registers[1],
                    "ADC",
                    "TRIGGER",
                    "BTIM1TRGO",
                    "independent_enable",
                    13,
                    1,
                    1,
                );
                adc = Some(
                    "/// Qualified ADC independent trigger enable.\n\
                    #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
                    pub enum TriggerInput {\n\
                    /// BTIM1 UPDATE emitted through TRGO.\n\
                    Btim1Update,\n}\n",
                );
            }
            _ => panic!("unqualified trigger destination"),
        }
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(
        out.join("timer_trigger_routes.rs"),
        timer.expect("missing BTIM2 mapping"),
    )
    .unwrap();
    fs::write(
        out.join("adc_trigger_routes.rs"),
        adc.expect("missing ADC mapping"),
    )
    .unwrap();
    println!("cargo:rustc-cfg=trigger_btim1_update");
}

// Select an actual independently owned timer and vector from generated metadata.
fn select_time_driver() -> Option<&'static str> {
    let gtim = env::var_os("CARGO_FEATURE_TIME_DRIVER_GTIM").is_some();
    let gtim1 = env::var_os("CARGO_FEATURE_TIME_DRIVER_GTIM1").is_some();
    let enabled = env::var_os("CARGO_FEATURE__TIME_DRIVER").is_some();
    assert!(
        !(gtim && gtim1),
        "select exactly one time-driver-gtim or time-driver-gtim1 feature"
    );
    assert!(
        !enabled || gtim || gtim1,
        "_time-driver requires an explicit timer selector"
    );
    let name = if gtim {
        "GTIM"
    } else if gtim1 {
        "GTIM1"
    } else {
        return None;
    };
    let metadata = &cw32_metapac::metadata::METADATA;
    let p = metadata
        .peripherals
        .iter()
        .find(|p| p.name == name)
        .unwrap_or_else(|| panic!("selected time driver {name} is absent on {}", metadata.line));
    let regs = p.registers.as_ref().unwrap();
    assert_eq!(regs.kind, "gtim");
    assert!(matches!(
        regs.version,
        "v1" | "cw32f002_v1" | "cw32l010_v1" | "cw32l012_v1" | "cw32l031_v1" | "cw32l052_v1"
    ));
    let irq = p
        .interrupts
        .iter()
        .find(|i| i.signal == "GLOBAL")
        .expect("time driver has no GLOBAL IRQ");
    assert!(
        !metadata.peripherals.iter().any(|other| other.name != name
            && other
                .interrupts
                .iter()
                .any(|i| i.interrupt == irq.interrupt)),
        "time-driver IRQ must not be shared"
    );
    let control = p
        .rcc_control
        .as_ref()
        .expect("time driver has no qualified RCC control");
    assert_eq!(control.bus_clock, "PCLK");
    assert!(control.shared_enable_group.is_none() && control.shared_reset_group.is_none());
    Some(name)
}

fn generate_time_driver(out: &mut String, selected: Option<&str>) {
    let Some(name) = selected else { return };
    let metadata = &cw32_metapac::metadata::METADATA;
    let p = metadata
        .peripherals
        .iter()
        .find(|p| p.name == name)
        .unwrap();
    let irq = p
        .interrupts
        .iter()
        .find(|i| i.signal == "GLOBAL")
        .unwrap()
        .interrupt;
    let number = metadata
        .interrupts
        .iter()
        .find(|i| i.name == irq)
        .unwrap()
        .number;
    out.push_str(&format!(
        "mod time_driver_resource {{ pub(crate) enum {name} {{}} }}\n\
         pub(crate) type TimeDriverPeripheral = time_driver_resource::{name};\n\
         pub(crate) use crate::pac::{name} as TIME_DRIVER_REGS;\n\
         pub(crate) type TimeDriverInterrupt = crate::interrupt::typelevel::{irq};\n\
         const _: () = assert!(crate::pac::Interrupt::{irq} as u16 == {number});\n\
         mod time_driver_vector {{\n\
         #[unsafe(no_mangle)] unsafe extern \"C\" fn {irq}() {{ crate::time_driver::on_interrupt(); }}\n\
         }}\n"
    ));
}

// Native low-power oscillator/RTC hardware has no amplitude translation. Authored exact-part facts feed these constants; IR checks bind
// every handwritten MMIO field to its own selected PAC layout.
fn generate_native_low_power_lse_configuration(
    out: &mut String,
    lse: &cw32_metapac::metadata::PeripheralLseConfiguration,
) {
    use cw32_metapac::metadata::{METADATA, ir};
    use std::fmt::Write;
    let native = lse
        .native_low_power
        .as_ref()
        .expect("missing own native low-power LSE facts");
    let l010 = METADATA.line == "CW32L010";
    let l011 = METADATA.line == "CW32L011";
    let l012 = METADATA.line == "CW32L012";
    assert!(l010 || l011 || l012);
    assert_eq!(
        native.monitor_reference,
        if l010 {
            "inherited_legal"
        } else {
            "factory_trim"
        }
    );
    assert_eq!(
        native.lsi_factory_trim_address,
        if l010 { None } else { Some(0x0010_07c2) }
    );
    assert_eq!(native.detector_margin_lse_edges, 1);
    assert_eq!((native.drive_bits, native.startup_drive_bits), (4, 4));
    assert_eq!(
        native.monitored_lsi_maximum_hz,
        if l011 { 41000 } else { 36080 }
    );
    assert_eq!(
        (native.detector_lse_edges, native.detector_lsi_cycles),
        (128, 256)
    );
    assert_eq!(
        (
            native.rtc_first_divisor,
            native.rtc_second_divisor,
            native.rtc_calendar_divisor
        ),
        (1, 16384, 32768)
    );
    assert!(lse.configurable_ccs && lse.startup_consumers.is_none() && lse.awt_source.is_none());
    assert_eq!((lse.nominal_hz, lse.maximum_hz), (32768, 100000));
    assert_eq!(*lse.startup_cycles, [256, 1024, 4096, 16384]);
    assert_eq!((lse.rtc_source, lse.uart_source, lse.mco_source), (0, 2, 6));
    let field = |name: &str, reg: &str, offset: u32, field: &str, bit: u32, width: u32| {
        let peripheral = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .unwrap();
        let regs = peripheral.registers.as_ref().unwrap();
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(reg))
            .unwrap();
        assert!(item.array.is_none());
        assert_eq!(item.byte_offset, offset);
        let ir::BlockItemInner::Register(r) = &item.inner else {
            panic!("native field is not a register")
        };
        assert_eq!(r.bit_size, 32);
        assert!(matches!(r.access, ir::Access::Read | ir::Access::ReadWrite));
        let fs = regs
            .ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == r.fieldset)
            .unwrap();
        let f = fs
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(field))
            .unwrap();
        assert!(f.array.is_none());
        assert_eq!(f.bit_size, width);
        assert!(matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
        f
    };
    if (l010 || l011 || l012) && lse.sysclk_detector.is_some() {
        for (reg, offset, name, bit, width) in [
            ("CR0", 0, "SYSCLK", 0, 3),
            ("CR0", 0, "PCLKPRS", 3, 2),
            ("CR0", 0, "HCLKPRS", 5, 3),
            ("CR0", 0, "KEY", 16, 16),
            ("CR1", 4, "CLKCCS", 8, 1),
            ("CR2", 8, "FLASHWAIT", 4, 3),
            ("AHBEN", 48, "FLASH", 1, 1),
            ("HSI", 24, "TRIM", 0, 11),
            ("HSI", 24, "DIV", 11, 4),
            ("HSI", 24, "STABLE", 15, 1),
        ] {
            field("SYSCTRL", reg, offset, name, bit, width);
        }
        field("FLASH", "CR2", 4, "WAIT", 0, 3);
        field("FLASH", "CR2", 4, "KEY", 16, 16);
        for owner in ["SYSCTRL", "FLASH"] {
            let regs = METADATA
                .peripherals
                .iter()
                .find(|p| p.name == owner)
                .and_then(|p| p.registers.as_ref())
                .unwrap();
            assert_eq!(
                regs.version,
                if l012 {
                    "cw32l012_v1"
                } else if l011 {
                    "cw32l011_v1"
                } else {
                    "cw32l010_v1"
                }
            );
            if owner == "SYSCTRL" {
                let cr0 = regs
                    .ir
                    .fieldsets
                    .iter()
                    .find(|f| f.name.eq_ignore_ascii_case("CR0"))
                    .unwrap();
                let selector = cr0
                    .fields
                    .iter()
                    .find(|f| f.name.eq_ignore_ascii_case("SYSCLK"))
                    .unwrap();
                let enumeration = regs
                    .ir
                    .enums
                    .iter()
                    .find(|e| Some(e.name) == selector.enumm)
                    .unwrap();
                assert!(
                    enumeration
                        .variants
                        .iter()
                        .any(|v| v.name.eq_ignore_ascii_case("LSE") && v.value == 4)
                );
                assert_eq!(
                    regs.ir
                        .fieldsets
                        .iter()
                        .find(|f| f.name.eq_ignore_ascii_case("HSI"))
                        .unwrap()
                        .fields
                        .len(),
                    3,
                    "L010/L011/L012 HSI has no programmable WAIT field"
                );
            }
        }
        if l011 || l012 {
            field("SYSCTRL", "APBEN1", 56, "UART3", 8, 1);
            field("SYSCTRL", "APBRST1", 72, "UART3", 8, 1);
            field("UART3", "CR1", 0, "SOURCE", 12, 2);
            let regs = METADATA
                .peripherals
                .iter()
                .find(|p| p.name == "UART3")
                .and_then(|p| p.registers.as_ref())
                .unwrap();
            let cr1 = regs
                .ir
                .fieldsets
                .iter()
                .find(|f| f.name.eq_ignore_ascii_case("CR1"))
                .unwrap();
            let source = cr1
                .fields
                .iter()
                .find(|f| f.name.eq_ignore_ascii_case("SOURCE"))
                .unwrap();
            let enumeration = regs
                .ir
                .enums
                .iter()
                .find(|e| Some(e.name) == source.enumm)
                .unwrap();
            assert_eq!(
                enumeration
                    .variants
                    .iter()
                    .filter(|v| v.name.eq_ignore_ascii_case("LSI") && v.value == 3)
                    .count(),
                1
            );
        }
    }
    if l012 && lse.sysclk_detector.is_some() {
        // Normalize only the selected IR's read-only startup status; runtime
        // checks STABLE separately while preserving all other source bits.
        for (register, offset, bit) in [("HSI", 24, 15), ("HSE", 28, 19)] {
            let stable = field("SYSCTRL", register, offset, "STABLE", bit, 1);
            let ir::BitOffset::Regular(position) = &stable.bit_offset else {
                unreachable!("validated scalar stability field")
            };
            let mask = 1u32.checked_shl(position.offset).unwrap();
            writeln!(
                out,
                "pub(crate) const RCC_{register}_STABLE_MASK: u32 = {mask};"
            )
            .unwrap();
        }
        // These checks bind the own L012 work-gate and retained consumer fields.
        for (owner, reg, offset, name, bit, width) in [
            ("FLASH", "CR2", 4, "FETCH", 3, 1),
            ("FLASH", "CR2", 4, "CACHE", 4, 1),
            ("FLASH", "CR2", 4, "CACHEINVALID", 5, 1),
            ("SYSCTRL", "CR1", 4, "HSIEN", 0, 1),
            ("SYSCTRL", "CR1", 4, "HSEEN", 1, 1),
            ("SYSCTRL", "CR1", 4, "LSIEN", 3, 1),
            ("SYSCTRL", "CR1", 4, "HSECCS", 7, 1),
            ("SYSCTRL", "CR1", 4, "LSECCS", 6, 1),
            ("SYSCTRL", "CR1", 4, "LSELOCK", 5, 1),
            ("SYSCTRL", "IER", 12, "HSIRDY", 0, 1),
            ("SYSCTRL", "IER", 12, "LSIRDY", 3, 1),
            ("SYSCTRL", "MCO", 112, "SOURCE", 0, 4),
            ("SYSCTRL", "LSI", 32, "TRIM", 0, 9),
            ("SYSCTRL", "LSI", 32, "WAITCYCLE", 10, 2),
            ("SYSCTRL", "LSI", 32, "STABLE", 15, 1),
            ("SYSCTRL", "APBEN1", 56, "ADC", 0, 1),
            ("SYSCTRL", "APBEN1", 56, "VC", 1, 1),
            ("SYSCTRL", "APBEN1", 56, "UART1", 3, 1),
            ("SYSCTRL", "APBEN1", 56, "UART2", 4, 1),
            ("SYSCTRL", "APBEN1", 56, "UART3", 8, 1),
            ("SYSCTRL", "APBRST1", 72, "ADC", 0, 1),
            ("SYSCTRL", "APBRST1", 72, "VC", 1, 1),
            ("SYSCTRL", "APBRST1", 72, "UART1", 3, 1),
            ("SYSCTRL", "APBRST1", 72, "UART2", 4, 1),
            ("SYSCTRL", "APBRST1", 72, "UART3", 8, 1),
            ("SYSCTRL", "APBEN2", 52, "RTC", 1, 1),
            ("SYSCTRL", "APBEN2", 52, "I2C1", 6, 1),
            ("SYSCTRL", "APBEN2", 52, "LPTIM", 7, 1),
            ("SYSCTRL", "APBEN2", 52, "OPA", 9, 1),
            ("SYSCTRL", "APBEN2", 52, "DAC", 10, 1),
            ("SYSCTRL", "APBEN2", 52, "I2C2", 11, 1),
            ("SYSCTRL", "APBRST2", 68, "RTC", 1, 1),
            ("SYSCTRL", "APBRST2", 68, "I2C1", 6, 1),
            ("SYSCTRL", "APBRST2", 68, "LPTIM", 7, 1),
            ("SYSCTRL", "APBRST2", 68, "OPA", 9, 1),
            ("SYSCTRL", "APBRST2", 68, "DAC", 10, 1),
            ("SYSCTRL", "APBRST2", 68, "I2C2", 11, 1),
            ("SYSCTRL", "APBEN1", 56, "KEY", 16, 16),
            ("SYSCTRL", "APBEN2", 52, "KEY", 16, 16),
            ("RTC", "CR1", 8, "SOURCE", 8, 3),
            ("RTC", "PSC", 64, "PSC1", 20, 8),
            ("RTC", "PSC", 64, "PSC2", 0, 20),
            ("UART1", "CR1", 0, "SOURCE", 12, 2),
            ("UART2", "CR1", 0, "SOURCE", 12, 2),
            ("UART3", "CR1", 0, "SOURCE", 12, 2),
            ("I2C1", "MCR0", 16, "CLKSRC", 6, 2),
            ("I2C1", "SCR0", 272, "CLKSRC", 6, 2),
            ("I2C2", "MCR0", 16, "CLKSRC", 6, 2),
            ("I2C2", "SCR0", 272, "CLKSRC", 6, 2),
            ("LPTIM", "CR0", 16, "EN", 0, 1),
            ("LPTIM", "CFGR", 12, "ICLKSRC", 25, 2),
            ("LPTIM", "CFGR", 12, "TRIGSEL", 12, 4),
            ("LPTIM", "CFGR", 12, "TRIGEN", 17, 2),
            ("ADC1", "CR", 0, "EN", 0, 1),
            ("ADC1", "CR", 0, "CLK", 2, 2),
            ("ADC2", "CR", 0, "EN", 0, 1),
            ("ADC2", "CR", 0, "CLK", 2, 2),
            ("LVD", "CR0", 0, "EN", 0, 1),
            ("LVD", "CR0", 0, "FLTCLK", 8, 1),
            ("LVD", "CR1", 4, "FLTTIME", 4, 4),
            ("VC1", "CR0", 0, "EN", 0, 1),
            ("VC1", "CR1", 4, "FLTTIME", 0, 4),
            ("VC1", "CR1", 4, "FLTCLK", 4, 1),
            ("VC1", "CR1", 4, "BLANKTIME", 8, 3),
            ("VC2", "CR0", 0, "EN", 0, 1),
            ("VC2", "CR1", 4, "FLTTIME", 0, 4),
            ("VC2", "CR1", 4, "FLTCLK", 4, 1),
            ("VC2", "CR1", 4, "BLANKTIME", 8, 3),
            ("VC3", "CR0", 0, "EN", 0, 1),
            ("VC3", "CR1", 4, "FLTTIME", 0, 4),
            ("VC3", "CR1", 4, "FLTCLK", 4, 1),
            ("VC3", "CR1", 4, "BLANKTIME", 8, 3),
            ("VC4", "CR0", 0, "EN", 0, 1),
            ("VC4", "CR1", 4, "FLTTIME", 0, 4),
            ("VC4", "CR1", 4, "FLTCLK", 4, 1),
            ("VC4", "CR1", 4, "BLANKTIME", 8, 3),
            ("OPA1", "CAL", 4, "CALEN", 0, 1),
            ("OPA1", "CAL", 4, "START", 2, 1),
            ("OPA1", "CAL", 4, "AZRUN", 10, 1),
            ("OPA2", "CAL", 4, "CALEN", 0, 1),
            ("OPA2", "CAL", 4, "START", 2, 1),
            ("OPA2", "CAL", 4, "AZRUN", 10, 1),
            ("DAC", "CR0", 0, "EN1", 0, 1),
            ("DAC", "CR0", 0, "TEN1", 1, 1),
            ("DAC", "CR0", 0, "WAVE1", 6, 2),
            ("DAC", "CR0", 0, "DMAEN1", 12, 1),
            ("DAC", "CR0", 0, "EN2", 16, 1),
            ("DAC", "CR0", 0, "TEN2", 17, 1),
            ("DAC", "CR0", 0, "WAVE2", 22, 2),
            ("DAC", "CR0", 0, "DMAEN2", 28, 1),
        ] {
            field(owner, reg, offset, name, bit, width);
        }
        for owner in ["VC1", "VC2", "VC3", "VC4"] {
            let regs = METADATA
                .peripherals
                .iter()
                .find(|p| p.name == owner)
                .and_then(|p| p.registers.as_ref())
                .unwrap();
            let blank = regs
                .ir
                .fieldsets
                .iter()
                .find(|f| f.name.eq_ignore_ascii_case("CR2"))
                .unwrap();
            assert_eq!(blank.fields.len(), 32);
            assert!(blank.fields.iter().all(|f| f.bit_size == 1));
        }
    }
    for (reg, offset, name, bit, width) in [
        ("CR1", 4, "LSEEN", 4, 1),
        ("CR1", 4, "LSELOCK", 5, 1),
        ("CR1", 4, "LSECCS", 6, 1),
        ("CR1", 4, "KEY", 16, 16),
        ("LSE", 36, "DRIVER", 0, 4),
        ("LSE", 36, "WAITCYCLE", 4, 2),
        ("LSE", 36, "MODE", 6, 1),
        ("LSE", 36, "PDRIVER", 8, 4),
        ("LSE", 36, "PINLOCK", 17, 1),
        ("LSE", 36, "STABLE", 18, 1),
        ("LSI", 32, "TRIM", 0, if l012 { 9 } else { 10 }),
        ("LSI", 32, "WAITCYCLE", 10, 2),
        ("LSI", 32, "STABLE", 15, 1),
        (
            "AHBEN",
            48,
            if l010 { "GPIOB" } else { "GPIOC" },
            if l010 { 5 } else { 6 },
            1,
        ),
        ("AHBEN", 48, "KEY", 16, 16),
        (
            "AHBRST",
            64,
            if l010 { "GPIOB" } else { "GPIOC" },
            if l010 { 5 } else { 6 },
            1,
        ),
    ] {
        field("SYSCTRL", reg, offset, name, bit, width);
    }
    for reg in ["IER", "ISR"] {
        let offset = if reg == "IER" { 12 } else { 16 };
        for (name, bit) in [("LSERDY", 4), ("LSEFAIL", 5), ("LSEFAULT", 7)] {
            field("SYSCTRL", reg, offset, name, bit, 1);
        }
    }
    field("SYSCTRL", "ISR", 16, "LSESTABLE", 15, 1);
    field("RTC", "CR0", 4, "H24", 3, 1);
    if l010 {
        field("RTC", "CR1", 8, "ACCESS", 0, 1);
    }
    for (name, bit, width) in [("WAIT", 2, 1), ("SOURCE", 8, 3)] {
        field("RTC", "CR1", 8, name, bit, width);
    }
    field("RTC", "PSC", 64, "PSC1", 20, 8);
    field("RTC", "PSC", 64, "PSC2", 0, 20);
    for name in ["UART1", "UART2"] {
        field(name, "CR1", 0, "SOURCE", 12, 2);
    }
    if !l010 {
        field("UART3", "CR1", 0, "SOURCE", 12, 2);
    }
    if l012 {
        for name in ["I2C1", "I2C2"] {
            field(name, "MCR0", 0x10, "CLKSRC", 6, 2);
            field(name, "SCR0", 0x110, "CLKSRC", 6, 2);
        }
    }
    field("LPTIM", if l012 { "CR0" } else { "CR" }, 16, "EN", 0, 1);
    for (name, bit, width) in [
        ("ICLKSRC", 25, 2),
        ("TRIGEN", 17, 2),
        (
            "TRIGSEL",
            if l012 { 12 } else { 13 },
            if l012 { 4 } else { 3 },
        ),
    ] {
        field("LPTIM", "CFGR", 12, name, bit, width);
    }
    let gpio = if l010 { "GPIOB" } else { "GPIOC" };
    let pad_bits: &[u32] = if l010 { &[0, 1] } else { &[14, 15] };
    for &bit in pad_bits {
        for (reg, offset) in [
            ("DIR", 0),
            ("ANALOG", 28),
            ("OPENDRAIN", 4),
            ("PUR", 16),
            ("RISEIE", 36),
            ("FALLIE", 40),
            ("FILTER", 64),
        ] {
            field(gpio, reg, offset, &format!("PIN{bit}"), bit, 1);
        }
        field(
            gpio,
            if l010 { "AFRL" } else { "AFRH" },
            if l010 { 24 } else { 20 },
            &format!("{}{bit}", if l012 { "PIN" } else { "AFR" }),
            (bit % 8) * 4,
            if l012 { 4 } else { 3 },
        );
    }
    let rtc_routes: &[(&str, u8)] = if l010 {
        &[("PB4", 2), ("PB6", 2)]
    } else if l011 {
        &[("PA1", 3), ("PA3", 3)]
    } else {
        &[
            ("PA1", 3),
            ("PA3", 3),
            ("PB14", 4),
            ("PB15", 4),
            ("PC13", 4),
        ]
    };
    assert_eq!(
        native
            .rtc_output_routes
            .iter()
            .map(|r| (r.pin, r.af))
            .collect::<Vec<_>>(),
        rtc_routes
    );
    // Only already-open output banks are observed by the native leaf.
    for route in native
        .rtc_output_routes
        .iter()
        .chain(lse.output_routes.iter())
    {
        let port = &route.pin[..2];
        let bit: u32 = route.pin[2..].parse().unwrap();
        let name = format!("GPIO{}", &port[1..]);
        field(&name, "DIR", 0, &format!("PIN{bit}"), bit, 1);
        field(&name, "ANALOG", 28, &format!("PIN{bit}"), bit, 1);
        field(
            &name,
            if bit < 8 { "AFRL" } else { "AFRH" },
            if bit < 8 { 24 } else { 20 },
            &format!("{}{bit}", if l012 { "PIN" } else { "AFR" }),
            (bit % 8) * 4,
            if l012 { 4 } else { 3 },
        );
    }
    let sysctrl = METADATA
        .peripherals
        .iter()
        .find(|p| p.name == "SYSCTRL")
        .unwrap();
    assert!(sysctrl.pins.iter().any(|p| p.signal == "LSE_IN"
        && p.pin == if l010 { "PB1" } else { "PC14" }
        && p.af.is_none()));
    assert!(sysctrl.pins.iter().any(|p| p.signal == "LSE_OUT"
        && p.pin == if l010 { "PB0" } else { "PC15" }
        && p.af.is_none()));
    writeln!(
        out,
        "pub(crate) const RCC_LSE_NOMINAL_HZ: u32 = {};",
        lse.nominal_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_MAXIMUM_HZ: u32 = {};",
        lse.maximum_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_SUPPLY_MV: (u16,u16) = {:?};",
        lse.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_TEMPERATURE_C: (i16,i16) = {:?};",
        lse.temperature_c
    )
    .unwrap();
    for (name, value) in [
        ("MONITORED_LSI_MAXIMUM_HZ", native.monitored_lsi_maximum_hz),
        ("DETECTOR_LSE_EDGES", u32::from(native.detector_lse_edges)),
        (
            "DETECTOR_MARGIN_LSE_EDGES",
            u32::from(native.detector_margin_lse_edges),
        ),
        ("DETECTOR_LSI_CYCLES", u32::from(native.detector_lsi_cycles)),
        ("RTC_SECOND_DIVISOR", u32::from(native.rtc_second_divisor)),
        ("RTC_CALENDAR_DIVISOR", native.rtc_calendar_divisor),
    ] {
        writeln!(out, "pub(crate) const RCC_LSE_{name}: u32 = {value};").unwrap();
    }
    writeln!(
        out,
        "pub(crate) const RCC_LSE_RTC_FIRST_DIVISOR: u16 = {};",
        native.rtc_first_divisor
    )
    .unwrap();
    for (name, value) in [
        ("RTC_SOURCE", lse.rtc_source),
        ("UART_SOURCE", lse.uart_source),
        ("MCO_SOURCE", lse.mco_source),
    ] {
        writeln!(out, "pub(crate) const RCC_LSE_{name}: u8 = {value};").unwrap();
    }
    if let Some(address) = native.lsi_factory_trim_address {
        writeln!(
            out,
            "pub(crate) const RCC_LSE_LSI_FACTORY_TRIM_ADDRESS: usize = {address};"
        )
        .unwrap();
    }
    // Field locations above are the source-reviewed native PAC contract.
    out.push_str("pub(crate) const RCC_LSE_CHANGE_MASK: u32 = 0x00040f7f;\n");
}

fn generate_lse_configuration(out: &mut String, c: &cw32_metapac::metadata::PeripheralClockLimits) {
    use cw32_metapac::metadata::{METADATA, ir};
    use std::fmt::Write;
    let expected = matches!(
        METADATA.name,
        "CW32F030C8T7"
            | "CW32A030C8T7"
            | "CW32F020C6U7"
            | "CW32L031C8T6"
            | "CW32L031C8U6"
            | "CW32L031F8U6"
            | "CW32R031C8U6"
            | "CW32L052C8T6"
            | "CW32L052R8S6"
            | "CW32L052R8T6"
            | "CW32L083RBT6"
            | "CW32L083RCT6"
            | "CW32L083RCS6"
            | "CW32L083MCT6"
            | "CW32L083VCT6"
            | "CW32L010F8P6"
            | "CW32L010F8U6"
            | "CW32L010Y8M6"
            | "CW32L011K8T6"
            | "CW32L011K8U6"
            | "CW32L012C8T6"
            | "CW32L012C8U6"
    );
    assert_eq!(
        c.lse_configuration.is_some(),
        expected,
        "exact-package LSE qualification missing or unexpected"
    );
    let Some(lse) = &c.lse_configuration else {
        return;
    };
    println!("cargo:rustc-cfg=rcc_lse");
    let l010_sysclk_qualified = matches!(
        METADATA.name,
        "CW32L010F8P6" | "CW32L010F8U6" | "CW32L010Y8M6"
    );
    let l011_sysclk_qualified = matches!(METADATA.name, "CW32L011K8T6" | "CW32L011K8U6");
    let l012_sysclk_qualified = matches!(METADATA.name, "CW32L012C8T6" | "CW32L012C8U6");
    let classic_sysclk_qualified = matches!(
        METADATA.name,
        "CW32F020C6U7" | "CW32F030C8T7" | "CW32A030C8T7"
    );
    let l031_sysclk_qualified = matches!(
        METADATA.name,
        "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6" | "CW32R031C8U6"
    );
    let l052_sysclk_qualified = matches!(
        METADATA.name,
        "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6"
    );
    let l083_sysclk_qualified = matches!(
        METADATA.name,
        "CW32L083RBT6" | "CW32L083RCT6" | "CW32L083RCS6" | "CW32L083MCT6" | "CW32L083VCT6"
    );
    assert_eq!(
        lse.sysclk_detector.is_some(),
        classic_sysclk_qualified
            || l031_sysclk_qualified
            || l052_sysclk_qualified
            || l083_sysclk_qualified
            || l010_sysclk_qualified
            || l011_sysclk_qualified
            || l012_sysclk_qualified,
        "LSE SYSCLK detector qualification missing or unexpected"
    );
    if let Some(detector) = &lse.sysclk_detector {
        assert_eq!(
            (
                detector.lse_edges,
                detector.lsi_cycles,
                detector.margin_lse_edges
            ),
            (128, 256, 1)
        );
        if l010_sysclk_qualified {
            assert_eq!(METADATA.line, "CW32L010");
            assert!(lse.configurable_ccs && l010_live_lsi_qualified(c) && c.pll.is_none());
            let native = lse
                .native_low_power
                .as_ref()
                .expect("L010 own inherited monitor facts");
            assert_eq!(native.monitor_reference, "inherited_legal");
            assert_eq!(native.lsi_factory_trim_address, None);
            assert_eq!(native.monitored_lsi_maximum_hz, 36_080);
            assert_eq!(
                (
                    detector.lse_edges,
                    detector.lsi_cycles,
                    detector.margin_lse_edges
                ),
                (
                    native.detector_lse_edges,
                    native.detector_lsi_cycles,
                    native.detector_margin_lse_edges
                )
            );
            assert_eq!(
                c.hse
                    .as_ref()
                    .expect("L010 fixed CCS facts")
                    .fixed_ccs_hsi_divisor,
                Some(12)
            );
            assert_eq!((c.hsi_frequency_hz, c.hsi_error_percent), (48_000_000, 2));
            assert_eq!(c.factory_hsi_trim_address, 0x0010_07c0);
            assert_eq!(c.hsi_supply_mv, (1620, 5500));
            assert_eq!(c.hsi_temperature_c, (-40, 85));
            assert_eq!(lse.supply_mv, c.hsi_supply_mv);
            assert_eq!(lse.temperature_c, c.hsi_temperature_c);
            assert_eq!(c.low_voltage_threshold_mv, 1800);
            assert_eq!(c.low_voltage_bus_max_hz, 24_000_000);
            assert_eq!(c.high_voltage_bus_max_hz, 48_000_000);
            assert_eq!(
                (c.flash_wait_step_hz, c.initial_flash_wait),
                (24_000_000, 1)
            );
            // Native helper below binds the own selector, HSI and Flash fields.
            // This historical monitor keeps its inherited-legal 36,080 Hz bound.
        } else if l011_sysclk_qualified {
            assert_eq!(METADATA.line, "CW32L011");
            assert!(lse.configurable_ccs && l011_live_lsi_qualified(c) && c.pll.is_none());
            let native = lse
                .native_low_power
                .as_ref()
                .expect("L011 own factory monitor facts");
            assert_eq!(native.monitor_reference, "factory_trim");
            assert_eq!(native.lsi_factory_trim_address, Some(0x0010_07c2));
            assert_eq!(native.monitored_lsi_maximum_hz, 41_000);
            assert_eq!(
                (
                    detector.lse_edges,
                    detector.lsi_cycles,
                    detector.margin_lse_edges
                ),
                (
                    native.detector_lse_edges,
                    native.detector_lsi_cycles,
                    native.detector_margin_lse_edges
                )
            );
            assert_eq!(
                c.hse
                    .as_ref()
                    .expect("L011 fixed CCS facts")
                    .fixed_ccs_hsi_divisor,
                Some(24)
            );
            assert_eq!((c.hsi_frequency_hz, c.hsi_error_percent), (96_000_000, 2));
            assert_eq!(c.factory_hsi_trim_address, 0x0010_07c0);
            assert_eq!(c.hsi_supply_mv, (1700, 5500));
            assert_eq!(c.hsi_temperature_c, (-40, 85));
            assert_eq!(lse.supply_mv, c.hsi_supply_mv);
            assert_eq!(lse.temperature_c, c.hsi_temperature_c);
            assert_eq!(c.low_voltage_threshold_mv, 1800);
            assert_eq!(c.low_voltage_bus_max_hz, 24_000_000);
            assert_eq!(c.high_voltage_bus_max_hz, 96_000_000);
            assert_eq!(
                (c.flash_wait_step_hz, c.initial_flash_wait),
                (24_000_000, 3)
            );
            assert_eq!(c.default_hsi_divisor, 24);
            writeln!(out, "const _: () = {{ assert!(crate::rcc::HsiDiv::Div24 as u8 == 14); assert!(crate::rcc::HsiDiv::Div24.divisor() == 24); assert!(crate::rcc::HsiDiv::Div32 as u8 == 0); assert!(crate::rcc::HsiDiv::Div32.divisor() == 32); }};").unwrap();
            // Native helper below binds the own selector, HSI and Flash fields.
            // This historical LSE proof grants neither the separate LSI SYSCLK
            // capability nor an RTC LSI identity.
        } else if l012_sysclk_qualified {
            assert_eq!(METADATA.line, "CW32L012");
            assert!(lse.configurable_ccs && l012_live_lsi_qualified(c) && c.pll.is_none());
            let native = lse
                .native_low_power
                .as_ref()
                .expect("L012 own factory monitor facts");
            assert_eq!(native.monitor_reference, "factory_trim");
            assert_eq!(native.lsi_factory_trim_address, Some(0x0010_07c2));
            assert_eq!(native.monitored_lsi_maximum_hz, 36_080);
            assert_eq!(
                (
                    detector.lse_edges,
                    detector.lsi_cycles,
                    detector.margin_lse_edges
                ),
                (
                    native.detector_lse_edges,
                    native.detector_lsi_cycles,
                    native.detector_margin_lse_edges
                )
            );
            assert_eq!(
                c.hse
                    .as_ref()
                    .expect("L012 fixed CCS facts")
                    .fixed_ccs_hsi_divisor,
                Some(24)
            );
            assert_eq!((c.hsi_frequency_hz, c.hsi_error_percent), (96_000_000, 2));
            assert_eq!(c.factory_hsi_trim_address, 0x0010_07c0);
            assert_eq!(c.hsi_supply_mv, (1700, 5500));
            assert_eq!(c.hsi_temperature_c, (-40, 85));
            assert_eq!(lse.supply_mv, c.hsi_supply_mv);
            assert_eq!(lse.temperature_c, c.hsi_temperature_c);
            assert_eq!(c.low_voltage_threshold_mv, 1800);
            assert_eq!(c.low_voltage_bus_max_hz, 24_000_000);
            assert_eq!(c.high_voltage_bus_max_hz, 96_000_000);
            assert_eq!(
                (c.flash_wait_step_hz, c.initial_flash_wait),
                (24_000_000, 3)
            );
            assert_eq!(c.default_hsi_divisor, 12);
            writeln!(out, "const _: () = {{ assert!(crate::rcc::HsiDiv::Div12 as u8 == 11); assert!(crate::rcc::HsiDiv::Div12.divisor() == 12); assert!(crate::rcc::HsiDiv::Div24 as u8 == 14); assert!(crate::rcc::HsiDiv::Div24.divisor() == 24); assert!(crate::rcc::HsiDiv::Div32 as u8 == 0); assert!(crate::rcc::HsiDiv::Div32.divisor() == 32); }};").unwrap();
            // Native helper below binds the own selector, HSI and Flash fields.
            // This historical LSE proof grants neither the separate LSI SYSCLK
            // capability nor an RTC LSI identity.
        } else if classic_sysclk_qualified {
            assert!(
                c.lsi_sysclk.is_some(),
                "classic LSE SYSCLK requires own factory-LSI facts"
            );
        } else {
            assert!(
                (l031_sysclk_qualified || l052_sysclk_qualified || l083_sysclk_qualified)
                    && lse.configurable_ccs
            );
            let native_lsi_qualified = matches!(
                (METADATA.line, METADATA.name),
                ("CW32L031", "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6")
                    | ("CW32R031", "CW32R031C8U6")
                    | ("CW32L052", "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6")
                    | ("CW32L083", "CW32L083RBT6" | "CW32L083RCT6" | "CW32L083RCS6" | "CW32L083MCT6" | "CW32L083VCT6")
            );
            assert_eq!(
                c.lsi_sysclk.is_some(),
                native_lsi_qualified,
                "Only the independent L031 exact3, R031 exact1, L052 exact3 and L083 exact5 policies qualify native LSI SYSCLK"
            );
            let rtc = METADATA
                .peripherals
                .iter()
                .find(|p| p.name == "RTC")
                .and_then(|p| p.rtc_calendar.as_ref())
                .expect("LSE SYSCLK requires own RTC factory monitor facts");
            let supply_mv = match METADATA.line {
                "CW32L031" | "CW32L052" | "CW32L083" => (1650, 5500),
                "CW32R031" => (2200, 3600),

                _ => panic!("unqualified LSE SYSCLK factory monitor family"),
            };
            assert_eq!((rtc.source, rtc.source_encoding), ("LSI", 2));
            assert_eq!(rtc.factory_trim_address, 0x0010_0a02);
            assert_eq!(
                (rtc.nominal_hz, rtc.minimum_hz, rtc.maximum_hz),
                (32_800, 31_816, 33_784)
            );
            assert_eq!(rtc.temperature_c, (-40, 85));
            assert_eq!(rtc.supply_mv, supply_mv);
            assert_eq!(lse.temperature_c, rtc.temperature_c);
            assert_eq!(lse.supply_mv, rtc.supply_mv);
            if let Some(lsi) = &c.lsi_sysclk {
                assert!(native_lsi_qualified);
                assert!(l031_sysclk_qualified || l052_sysclk_qualified || l083_sysclk_qualified);
                assert_eq!(lsi.factory_trim_address, rtc.factory_trim_address);
                assert_eq!(
                    (lsi.nominal_hz, lsi.minimum_hz, lsi.maximum_hz),
                    (rtc.nominal_hz, rtc.minimum_hz, rtc.maximum_hz)
                );
                assert_eq!(lsi.supply_mv, rtc.supply_mv);
                assert_eq!(lsi.temperature_c, rtc.temperature_c);
            }
            if l052_sysclk_qualified {
                assert_eq!(METADATA.line, "CW32L052");
                assert_eq!(
                    c.hse
                        .as_ref()
                        .expect("L052 fixed CCS facts")
                        .fixed_ccs_hsi_divisor,
                    Some(6)
                );
                assert_eq!((c.hsi_frequency_hz, c.hsi_error_percent), (48_000_000, 2));
                assert_eq!(c.hsi_supply_mv, rtc.supply_mv);
                assert_eq!(c.hsi_temperature_c, rtc.temperature_c);
                // The native field checks below independently bind LSI
                // TRIM[9:0], WAITCYCLE[11:10] and STABLE[15].
            }
            if l083_sysclk_qualified {
                assert_eq!(METADATA.line, "CW32L083");
                assert_eq!(
                    c.hse
                        .as_ref()
                        .expect("L083 own HSE facts")
                        .fixed_ccs_hsi_divisor,
                    None,
                    "L083 conservative fallback is not a hardware fixed-divisor fact"
                );
                assert!(c.pll.is_some(), "L083 retains its own PLL metadata");
                assert_eq!((c.hsi_frequency_hz, c.hsi_error_percent), (48_000_000, 2));
                assert_eq!(c.factory_hsi_trim_address, 0x0010_0a00);
                assert_eq!(c.hsi_supply_mv, rtc.supply_mv);
                assert_eq!(c.hsi_temperature_c, rtc.temperature_c);
                assert_eq!(c.low_voltage_threshold_mv, 1800);
                assert_eq!(c.low_voltage_bus_max_hz, 24_000_000);
                assert_eq!(c.high_voltage_bus_max_hz, 64_000_000);
                assert_eq!(
                    (c.flash_wait_step_hz, c.initial_flash_wait),
                    (24_000_000, 2)
                );
                assert!(
                    !lse.startup_consumers
                        .as_ref()
                        .expect("L083 native consumers")
                        .startup_analog
                );
            }
        }
        for (name, value) in [
            ("LSE_EDGES", detector.lse_edges),
            ("LSI_CYCLES", detector.lsi_cycles),
            ("MARGIN_LSE_EDGES", detector.margin_lse_edges),
        ] {
            writeln!(
                out,
                "pub(crate) const RCC_LSE_SYSCLK_{name}: u16 = {value};"
            )
            .unwrap();
        }
    }
    if matches!(METADATA.line, "CW32L010" | "CW32L011" | "CW32L012") {
        generate_native_low_power_lse_configuration(out, lse);
        return;
    }
    assert!(
        lse.native_low_power.is_none(),
        "native low-power facts leaked to an unqualified line"
    );
    let peripheral = |name: &str| {
        METADATA
            .peripherals
            .iter()
            .find(|p| p.name == name)
            .expect("missing LSE dependency")
    };
    let register = |name: &str, reg: &str, offset: Option<u32>| {
        let regs = peripheral(name).registers.as_ref().unwrap();
        let block = regs
            .ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap();
        let item = block
            .items
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(reg))
            .expect("missing LSE dependency register");
        if let Some(offset) = offset {
            assert_eq!(item.byte_offset, offset);
        }
        assert!(item.array.is_none());
        let ir::BlockItemInner::Register(r) = &item.inner else {
            panic!("not a register")
        };
        assert_eq!(r.bit_size, 32);
        assert!(matches!(r.access, ir::Access::Read | ir::Access::ReadWrite));
        regs.ir
            .fieldsets
            .iter()
            .find(|f| Some(f.name) == r.fieldset)
            .unwrap()
    };
    let has_register = |name: &str, reg: &str| {
        let regs = peripheral(name).registers.as_ref().unwrap();
        regs.ir
            .blocks
            .iter()
            .find(|b| b.name == regs.block)
            .unwrap()
            .items
            .iter()
            .any(|r| r.name.eq_ignore_ascii_case(reg))
    };
    let field = |name: &str, reg: &str, field: &str, bit: u32, width: u32| {
        let f = register(name, reg, None)
            .fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(field))
            .expect("missing native LSE field");
        assert!(f.array.is_none());
        assert_eq!(f.bit_size, width);
        assert!(matches!(&f.bit_offset, ir::BitOffset::Regular(p) if p.offset == bit));
    };
    assert_eq!(
        lse.configurable_ccs,
        matches!(
            METADATA.line,
            "CW32L031" | "CW32R031" | "CW32L052" | "CW32L083"
        )
    );
    writeln!(
        out,
        "pub(crate) const RCC_LSE_CONFIGURABLE_CCS: bool = {};",
        lse.configurable_ccs
    )
    .unwrap();
    assert_eq!(
        lse.startup_consumers.is_some(),
        matches!(METADATA.line, "CW32L052" | "CW32L083")
    );
    if let Some(native) = &lse.startup_consumers {
        // Native AUTOTRIM/LPTIM/LCD consumers do not imply a second analog bank.
        println!("cargo:rustc-cfg=rcc_lse_native_consumers");
        assert_eq!(native.startup_analog, METADATA.line == "CW32L052");
        if native.startup_analog {
            field("SYSCTRL", "LSE", "PDRIVER", 8, 2);
            field("SYSCTRL", "LSE", "PAMP", 10, 2);
            println!("cargo:rustc-cfg=rcc_lse_startup_analog");
        }
    }
    assert_eq!(lse.nominal_hz, 32768);
    assert_eq!(*lse.startup_cycles, [256, 1024, 4096, 16384]);
    writeln!(
        out,
        "pub(crate) const RCC_LSE_NOMINAL_HZ: u32 = {};",
        lse.nominal_hz
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_SUPPLY_MV: (u16,u16) = {:?};",
        lse.supply_mv
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_TEMPERATURE_C: (i16,i16) = {:?};",
        lse.temperature_c
    )
    .unwrap();
    writeln!(
        out,
        "pub(crate) const RCC_LSE_MAXIMUM_HZ: u32 = {};",
        lse.maximum_hz
    )
    .unwrap();
    writeln!(out, "pub(crate) const RCC_LSE_RTC_SOURCE: crate::pac::rtc::vals::Source = crate::pac::rtc::vals::Source::from_bits({});", lse.rtc_source).unwrap();
    for (name, bit, width) in [
        ("DRIVER", 0, 2),
        ("AMP", 2, 2),
        ("WAITCYCLE", 4, 2),
        ("MODE", 6, 1),
        ("STABLE", 15, 1),
    ] {
        field("SYSCTRL", "LSE", name, bit, width);
    }
    field("SYSCTRL", "CR1", "LSEEN", 4, 1);
    field("SYSCTRL", "CR1", "LSELOCK", 5, 1);
    field("SYSCTRL", "CR1", "LSECCS", 6, 1);
    field("SYSCTRL", "CR1", "LSIEN", 3, 1);
    field("SYSCTRL", "LSI", "TRIM", 0, 10);
    field("SYSCTRL", "LSI", "WAITCYCLE", 10, 2);
    field("SYSCTRL", "LSI", "STABLE", 15, 1);
    let sysctrl = peripheral("SYSCTRL");
    let pins: Vec<_> = ["LSE_IN", "LSE_OUT"]
        .into_iter()
        .map(|signal| {
            let found: Vec<_> = sysctrl
                .pins
                .iter()
                .filter(|p| p.signal == signal && p.af.is_none())
                .collect();
            assert_eq!(found.len(), 1);
            let name = found[0].pin;
            let port = name.as_bytes()[1] - b'A';
            let bit: u8 = name[2..].parse().unwrap();
            (port, bit)
        })
        .collect();
    for (port, bit) in &pins {
        let gpio = format!("GPIO{}", char::from(b'A' + port));
        register(&gpio, "DIR", Some(lse.gpio_dir_offset));
        if let Some(offset) = lse.gpio_speed_offset {
            register(&gpio, "SPEED", Some(offset));
        } else {
            assert!(!has_register(&gpio, "SPEED"));
        }
        for reg in [
            "DIR",
            "ANALOG",
            "PUR",
            "PDR",
            "OPENDRAIN",
            "FILTER",
            "RISEIE",
            "FALLIE",
            "HIGHIE",
            "LOWIE",
        ] {
            if matches!(reg, "HIGHIE" | "LOWIE") && !has_register(&gpio, reg) {
                assert!(lse.configurable_ccs);
                continue;
            }
            field(&gpio, reg, &format!("PIN{bit}"), u32::from(*bit), 1);
        }
        let lock_registers: Vec<_> = ["LOCK", "LCKR"]
            .into_iter()
            .filter(|name| has_register(&gpio, name))
            .collect();
        assert_eq!(
            lock_registers.len(),
            usize::from(!lse.configurable_ccs || lse.startup_consumers.is_some()),
            "native GPIO lock capability differs from qualified roster"
        );
        for reg in lock_registers {
            field(&gpio, reg, &format!("PIN{bit}"), u32::from(*bit), 1);
        }
        field(
            &gpio,
            if *bit < 8 { "AFRL" } else { "AFRH" },
            &format!("AFR{bit}"),
            u32::from(*bit % 8) * 4,
            4,
        );
    }
    out.push_str("pub(crate) fn rcc_lse_pins_match(bypass: bool, unused: bool, timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, crate::rcc::Error> {\nlet mut matches = true;\n");
    for (i, (port, bit)) in pins.iter().enumerate() {
        if i == 1 {
            out.push_str("if !bypass {\n");
        }
        writeln!(out,"let info = gpio_rcc({port});\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LsePinConflict); }}\nmatches &= info.inspect_for_init(cs, timeout, || {{\nlet r=crate::pac::GPIO{};\nr.dir().read().pin{bit}()\n&& r.analog().read().pin{bit}() == {}\n&& r.{}().read().afr{bit}() == 0",char::from(b'A'+port),if i==0 {"(unused || !bypass)"}else{"true"},if *bit<8{"afrl"}else{"afrh"}).unwrap();
        for reg in [
            "pur",
            "pdr",
            "opendrain",
            "filter",
            "riseie",
            "fallie",
            "highie",
            "lowie",
            "lock",
            "lckr",
        ] {
            let gpio = format!("GPIO{}", char::from(b'A' + port));
            if has_register(&gpio, reg) {
                writeln!(out, "&& !r.{reg}().read().pin{bit}()").unwrap();
            }
        }
        out.push_str("}).map_err(|_| crate::rcc::Error::LsePinConflict)?;\n");
        if i == 1 {
            out.push_str("}\n");
        }
    }
    out.push_str("Ok(matches)\n}\n");
    // Admission already proved reset-like pad configuration. Crystal requires
    // no pad change; bypass only enables the selected input's digital buffer.
    let (port, bit) = pins[0];
    writeln!(out,"pub(crate) fn rcc_configure_lse_pins(bypass: bool, timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<(), crate::rcc::Error> {{\nif !rcc_lse_pins_match(bypass, true, timeout, cs)? {{ return Err(crate::rcc::Error::LsePinConflict); }}\nif bypass {{\ngpio_rcc({port}).inspect_for_init(cs, timeout, || crate::pac::GPIO{}.analog().modify(|w| w.set_pin{bit}(false))).map_err(|_| crate::rcc::Error::LsePinConflict)?;\n}}\nif !rcc_lse_pins_match(bypass, false, timeout, cs)? {{ return Err(crate::rcc::Error::LsePinConflict); }}\nOk(())\n}}", char::from(b'A'+port)).unwrap();
    if lse.startup_consumers.is_some() {
        for (name, bit, width) in [("EN", 0, 1), ("MD", 1, 2), ("AUTO", 3, 1), ("SRC", 8, 3)] {
            field("AUTOTRIM", "CR", name, bit, width);
        }
        out.push_str("pub(crate) fn rcc_lse_monitor_can_freeze(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\nlet info = crate::peripherals::AUTOTRIM::RCC_INFO;\nif info.reset_asserted() { return Err(crate::rcc::Error::LseClockInUse); }\ninfo.inspect_for_init(cs, timeout, || { let r=crate::pac::AUTOTRIM.cr().read(); !r.auto() && matches!(u8::from(r.md()), 0 | 1 | 3) && u8::from(r.src()) <= 4 && (!r.en() || r.md() == crate::pac::autotrim::vals::Mode::Timer) }).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)\n}\n");
    }
    out.push_str("pub(crate) fn rcc_lse_consumers_idle(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\n");
    // Inspect controls before reading a running calendar's other registers.
    out.push_str("let info = crate::peripherals::RTC::RCC_INFO;\nif info.reset_asserted() { return Err(crate::rcc::Error::LseClockInUse); }\nlet rtc_idle = info.inspect_for_init(cs, timeout, || {\nlet snapshot = || {\n");
    let order = [
        "CR0", "CR1", "CR2", "IER", "ISR", "COMPEN", "DATE", "TIME", "ALARMA", "ALARMB",
        "TAMPDATE", "TAMPTIME", "AWTARR",
    ];
    assert_eq!(lse.rtc_reset.len(), order.len());
    for (i, name) in order.iter().enumerate() {
        let matching: Vec<_> = lse
            .rtc_reset
            .iter()
            .filter(|r| r.register == *name)
            .collect();
        assert_eq!(matching.len(), 1);
        let r = matching[0];
        register("RTC", name, Some(r.byte_offset));
        writeln!(
            out,
            "{}(crate::pac::RTC.{}().read().0 & {} == {})",
            if i == 0 { "" } else { "&& " },
            name.to_ascii_lowercase(),
            r.mask,
            r.value & r.mask
        )
        .unwrap();
    }
    out.push_str("};\nsnapshot() && snapshot()\n}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)?;\nif !rtc_idle { return Ok(false); }\n");
    if let Some(source) = lse.awt_source {
        field("AWT", "CR", "SRC", 8, 3);
        writeln!(out,"let info = crate::peripherals::AWT::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || u8::from(crate::pac::AWT.cr().read().src()) == {source}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}").unwrap();
    }
    if let Some(native) = &lse.startup_consumers {
        out.push_str("if crate::pac::SYSCTRL.cr0().read().sysclk() == crate::pac::sysctrl::vals::Sysclk::Lse || !rcc_lse_monitor_can_freeze(timeout, cs)? { return Ok(false); }\n");
        writeln!(out,"let info = crate::peripherals::AUTOTRIM::RCC_INFO;\nif info.inspect_for_init(cs, timeout, || u8::from(crate::pac::AUTOTRIM.cr().read().src()) == {}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}", native.autotrim_source).unwrap();
        field("LPTIM", "CR", "EN", 0, 1);
        field("LPTIM", "CFGR", "ICLKSRC", 25, 2);
        field("LCD", "CR0", "EN", 0, 1);
        field("LCD", "CR1", "CLKCS", 7, 1);
        assert!(native.lptim.gate_controls_work && native.lcd.gate_controls_work);
        // These gates also stop work. Never enable either gate to read locals.
        for (name, expression) in [
            (
                "LPTIM",
                format!(
                    "crate::pac::LPTIM.cr().read().en() && u8::from(crate::pac::LPTIM.cfgr().read().iclksrc()) == {}",
                    native.lptim.source
                ),
            ),
            (
                "LCD",
                format!(
                    "crate::pac::LCD.cr0().read().en() && u8::from(crate::pac::LCD.cr1().read().clkcs()) == {}",
                    native.lcd.source
                ),
            ),
        ] {
            writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nlet enabled = info.is_enabled();\nif enabled && (info.reset_asserted() || ({expression})) {{ return Ok(false); }}\nif info.is_enabled() != enabled {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
        }
    }
    let uart_source_field = if METADATA.line == "CW32L052" {
        "sorce"
    } else {
        "source"
    };
    let uarts: Vec<_> = METADATA
        .peripherals
        .iter()
        .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "uart"))
        .collect();
    if let Some(native) = &lse.startup_consumers {
        assert_eq!(
            uarts.iter().map(|p| p.name).collect::<Vec<_>>(),
            native.uarts
        );
    } else {
        assert_eq!(uarts.len(), 3);
    }
    for uart in uarts {
        field(uart.name, "CR2", uart_source_field, 8, 2);
        writeln!(out,"let info = crate::peripherals::{}::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || u8::from(crate::pac::{}.cr2().read().{uart_source_field}()) == {}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}",uart.name,uart.name,lse.uart_source).unwrap();
    }
    field("SYSCTRL", "MCO", "SOURCE", 0, 4);
    if METADATA.line == "CW32L083" {
        // Own MCO table has defined source encodings 0 through 9 only.
        out.push_str("if crate::pac::SYSCTRL.mco().read().source() > 9 { return Ok(false); }\n");
    }
    writeln!(
        out,
        "if crate::pac::SYSCTRL.mco().read().source() == {} {{ return Ok(false); }}",
        lse.mco_source
    )
    .unwrap();
    for route in lse.output_routes {
        let port = route.pin.as_bytes()[1] - b'A';
        let bit: u8 = route.pin[2..].parse().unwrap();
        let gpio = format!("GPIO{}", char::from(b'A' + port));
        let afreg = if bit < 8 { "afrl" } else { "afrh" };
        field(
            &gpio,
            afreg,
            &format!("AFR{bit}"),
            u32::from(bit % 8) * 4,
            4,
        );
        writeln!(out,"let info = gpio_rcc({port});\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || crate::pac::{gpio}.{afreg}().read().afr{bit}() == {}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}",route.af).unwrap();
    }
    out.push_str("Ok(true)\n}\n");

    if lse.configurable_ccs {
        // Native source fields and full GPIO register rosters are checked by
        // the own-source qualification before this direct PAC projection.
        field("SYSCTRL", "CR0", "SYSCLK", 0, 3);
        field("SYSCTRL", "ISR", "LSISTABLE", 14, 1);
        field("SYSCTRL", "IER", "LSIRDY", 3, 1);
        field("SYSCTRL", "ISR", "LSIRDY", 3, 1);
        out.push_str("pub(crate) fn rcc_lsi_consumers_idle(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, crate::rcc::Error> {\nuse crate::rcc::SealedRccPeripheral;\nif !rcc_lse_consumers_idle(timeout, cs)? { return Ok(false); }\nif !matches!(crate::pac::SYSCTRL.cr0().read().sysclk(), crate::pac::sysctrl::vals::Sysclk::Hsi | crate::pac::sysctrl::vals::Sysclk::Hse | crate::pac::sysctrl::vals::Sysclk::Lse) { return Ok(false); }\nif !matches!(crate::pac::SYSCTRL.mco().read().source(), 0..=3 | 5 | 6 | 8 | 9) { return Ok(false); }\n");
        if lse.awt_source.is_some() {
            out.push_str("let info = crate::peripherals::AWT::RCC_INFO;\nif info.reset_asserted() { return Err(crate::rcc::Error::LseClockInUse); }\nif !info.inspect_for_init(cs, timeout, || matches!(crate::pac::AWT.cr().read().src(), crate::pac::awt::vals::Source::Hsiosc | crate::pac::awt::vals::Source::Hse | crate::pac::awt::vals::Source::Lse | crate::pac::awt::vals::Source::Etr)).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? { return Ok(false); }\n");
        }
        if lse.startup_consumers.is_some() {
            out.push_str("let info = crate::peripherals::AUTOTRIM::RCC_INFO;\nif info.inspect_for_init(cs, timeout, || { let r = crate::pac::AUTOTRIM.cr().read(); u8::from(r.src()) == 1 || (r.en() && r.md() == crate::pac::autotrim::vals::Mode::LsiCalibration) }).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? { return Ok(false); }\n");
            for (name, expression) in [
                (
                    "LPTIM",
                    "crate::pac::LPTIM.cr().read().en() && u8::from(crate::pac::LPTIM.cfgr().read().iclksrc()) == 3",
                ),
                (
                    "LCD",
                    "crate::pac::LCD.cr0().read().en() && u8::from(crate::pac::LCD.cr1().read().clkcs()) == 0",
                ),
            ] {
                writeln!(out, "let info = crate::peripherals::{name}::RCC_INFO;\nlet enabled = info.is_enabled();\nif enabled && (info.reset_asserted() || ({expression})) {{ return Ok(false); }}\nif info.is_enabled() != enabled {{ return Err(crate::rcc::Error::LseClockInUse); }}").unwrap();
            }
        }
        for uart in METADATA
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "uart"))
        {
            writeln!(out, "let info = crate::peripherals::{}::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || crate::pac::{}.cr2().read().{uart_source_field}() == crate::pac::uart::vals::Source::Lsi).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}", uart.name, uart.name).unwrap();
        }
        for gpio in METADATA
            .peripherals
            .iter()
            .filter(|p| p.registers.as_ref().is_some_and(|r| r.kind == "gpio"))
        {
            field(gpio.name, "FILTER", "FLTCLK", 16, 3);
            writeln!(out, "let info = crate::peripherals::{}::RCC_INFO;\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || crate::pac::{}.filter().read().fltclk() == 5).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}", gpio.name, gpio.name).unwrap();
        }
        if let Some(native) = &lse.startup_consumers {
            for route in native.lsi_output_routes {
                let port = route.pin.as_bytes()[1] - b'A';
                let bit: u8 = route.pin[2..].parse().unwrap();
                let gpio = format!("GPIO{}", char::from(b'A' + port));
                let afreg = if bit < 8 { "afrl" } else { "afrh" };
                field(
                    &gpio,
                    afreg,
                    &format!("AFR{bit}"),
                    u32::from(bit % 8) * 4,
                    4,
                );
                assert!(METADATA.pins.iter().any(|p| p.name == route.pin));
                writeln!(out,"let info = gpio_rcc({port});\nif info.reset_asserted() {{ return Err(crate::rcc::Error::LseClockInUse); }}\nif info.inspect_for_init(cs, timeout, || crate::pac::{gpio}.{afreg}().read().afr{bit}() == {}).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? {{ return Ok(false); }}",route.af).unwrap();
            }
        } else if METADATA.pins.iter().any(|p| p.name == "PB11") {
            field("GPIOB", "AFRH", "AFR11", 12, 4);
            out.push_str("let info = crate::peripherals::GPIOB::RCC_INFO;\nif info.reset_asserted() { return Err(crate::rcc::Error::LseClockInUse); }\nif info.inspect_for_init(cs, timeout, || crate::pac::GPIOB.afrh().read().afr11() == 1).map_err(|_| crate::rcc::Error::RetainedClockInspectionTimeout)? { return Ok(false); }\n");
        }
        out.push_str("Ok(true)\n}\n");
    }
}
