#![cfg(feature = "metadata")]
use cw32_metapac::metadata::{METADATA, ir::BlockItemInner};
#[test]
fn every_peripheral_register_reference_resolves() {
    assert!(METADATA.nvic_priority_bits.is_some());
    assert!(!METADATA.peripherals.is_empty());
    let mut irq_numbers = std::collections::BTreeSet::new();
    for irq in METADATA.interrupts {
        assert!(irq_numbers.insert(irq.number), "duplicate interrupt number");
    }
    for peripheral in METADATA.peripherals {
        let Some(registers) = &peripheral.registers else {
            continue;
        };
        assert!(
            registers
                .ir
                .blocks
                .iter()
                .any(|b| b.name == registers.block),
            "{} has unresolved block {}",
            peripheral.name,
            registers.block
        );
        for block in registers.ir.blocks {
            for item in block.items {
                match &item.inner {
                    BlockItemInner::Register(register) => {
                        if let Some(name) = register.fieldset {
                            assert!(
                                registers.ir.fieldsets.iter().any(|f| f.name == name),
                                "{}.{} has unresolved fieldset {name}",
                                block.name,
                                item.name
                            );
                        }
                    }
                    BlockItemInner::Block(block) => {
                        assert!(registers.ir.blocks.iter().any(|b| b.name == block.block));
                    }
                }
            }
        }
        for fieldset in registers.ir.fieldsets {
            for field in fieldset.fields {
                if let Some(name) = field.enumm {
                    assert!(registers.ir.enums.iter().any(|e| e.name == name));
                }
            }
        }
    }
}

#[test]
fn reviewed_topology_references_resolve() {
    let mut pins = std::collections::BTreeSet::new();
    for pin in METADATA.pins {
        assert!(pins.insert(pin.name), "duplicate GPIO pin");
        assert!(pin.name.len() >= 3 && pin.name.starts_with('P'));
        assert!(pin.name[2..].parse::<u8>().is_ok_and(|n| n < 16));
        let gpio = format!("GPIO{}", &pin.name[1..2]);
        assert!(METADATA.peripherals.iter().any(|p| p.name == gpio));
    }
    let mut channels = std::collections::BTreeSet::new();
    for channel in METADATA.dma_channels {
        assert!(channels.insert(channel.name), "duplicate DMA channel");
        assert!(METADATA.peripherals.iter().any(|p| p.name == channel.dma));
        let physical = format!("DMACHANNEL{}", channel.channel + 1);
        let peripheral = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == physical)
            .unwrap();
        assert!(!peripheral.interrupts.is_empty());
    }
    for peripheral in METADATA.peripherals {
        for route in peripheral.dma_channels {
            if let Some(dma) = route.dma {
                assert!(METADATA.peripherals.iter().any(|p| p.name == dma));
            }
            if let Some(channel) = route.channel {
                assert!(channels.contains(channel));
            }
            assert!(route.request.is_some_and(|request| request < 64));
        }
        if let Some(rcc) = &peripheral.rcc {
            let controller = METADATA
                .peripherals
                .iter()
                .find(|p| p.name == "SYSCTRL")
                .unwrap();
            let registers = controller.registers.as_ref().unwrap();
            let block = registers
                .ir
                .blocks
                .iter()
                .find(|b| b.name == registers.block)
                .unwrap();
            for field in rcc.enable.iter().chain(rcc.reset.iter()) {
                let item = block
                    .items
                    .iter()
                    .find(|i| i.name == field.register)
                    .unwrap();
                let BlockItemInner::Register(register) = &item.inner else {
                    panic!("RCC field target is not a register")
                };
                let fieldset = registers
                    .ir
                    .fieldsets
                    .iter()
                    .find(|f| Some(f.name) == register.fieldset)
                    .unwrap();
                assert!(fieldset.fields.iter().any(|f| f.name == field.field));
            }
        }
    }
}

#[test]
fn adc_mux_is_explicit_and_independent_of_signal_labels() {
    for peripheral in METADATA.peripherals {
        for route in peripheral.pins {
            assert!(METADATA.pins.iter().any(|p| p.name == route.pin));
            if matches!(peripheral.name, "ADC" | "ADC1" | "ADC2") {
                assert!(route.af.is_none());
                let mux = route
                    .adc_mux
                    .expect("qualified ADC route must carry hardware mux");
                // L010/L011 have fourteen external sources (mux0..13); L012
                // has twelve per converter (mux0..11), while
                // classic ADCs have thirteen (mux0..12). Internal sources are
                // never GPIO routes. Bounds follow the actual selected family.
                let external_sources = match METADATA.line {
                    "CW32L010" | "CW32L011" => 14,
                    "CW32L012" => 12,
                    _ => 13,
                };
                assert!(mux < external_sources);
                let logical: u8 = route.signal.strip_prefix("IN").unwrap().parse().unwrap();
                if METADATA.name.starts_with("CW32R031") {
                    assert_eq!(mux, logical + 4);
                } else {
                    assert_eq!(mux, logical);
                }
            } else {
                assert!(route.adc_mux.is_none());
            }
        }
    }
}

#[test]
fn verified_clock_controls_resolve_without_completing_partial_kernels() {
    use cw32_metapac::metadata::{PeripheralRccRegister, ir};
    let mut controls = 0;
    for peripheral in METADATA.peripherals {
        let Some(control) = &peripheral.rcc_control else {
            continue;
        };
        controls += 1;
        assert!(!control.bus_clock.is_empty());
        let controller = METADATA
            .peripherals
            .iter()
            .find(|p| p.name == control.controller)
            .unwrap();
        let registers = controller.registers.as_ref().unwrap();
        let resolve = |reference: &PeripheralRccRegister| {
            let block = registers
                .ir
                .blocks
                .iter()
                .find(|b| b.name == registers.block)
                .unwrap();
            let item = block
                .items
                .iter()
                .find(|i| i.name == reference.register)
                .unwrap();
            let BlockItemInner::Register(register) = &item.inner else {
                panic!("clock control references a block")
            };
            assert_eq!(register.access, ir::Access::ReadWrite);
            let fields = registers
                .ir
                .fieldsets
                .iter()
                .find(|f| Some(f.name) == register.fieldset)
                .unwrap();
            fields
                .fields
                .iter()
                .find(|f| f.name == reference.field)
                .unwrap()
                .bit_size
        };
        assert_eq!(resolve(&control.enable), 1);
        if let Some(reset) = &control.reset {
            assert_eq!(resolve(reset), 1);
            assert!(control.reset_asserted_value.is_some());
        } else {
            assert!(control.reset_asserted_value.is_none());
        }
        if let Some(key) = &control.enable_write_key {
            assert_eq!(key.field.register, control.enable.register);
            let bits = resolve(&key.field);
            assert!(bits == 32 || key.value < (1 << bits));
        }
        // A peripheral-local clock selector still cannot be represented by Rcc.
        if peripheral.name.starts_with("UART")
            || (METADATA.line == "CW32L012" && peripheral.name.starts_with("I2C"))
        {
            assert!(peripheral.rcc.is_none());
        }
    }
    assert!(controls > 0);
}
