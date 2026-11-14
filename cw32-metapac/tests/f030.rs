//! Host-side checks of generated PAC shape and reviewed CW32F030 facts.
//!
//! These tests never dereference hardware peripheral pointers. They verify
//! addresses, access-width types, value bitfields, IRQs and metadata consistency.
//! One test uses an aligned RAM-backed register block to check access widths.
//! They do not test silicon behavior or perform peripheral MMIO.

#![cfg(all(feature = "cw32f030", feature = "pac"))]

use core::mem::size_of;
use cw32_metapac as pac;
use pac::common::{Access, R, RW, Reg, W};

fn register<T: Copy, A: Access>(reg: Reg<T, A>, address: usize, bytes: usize) {
    assert_eq!(reg.as_ptr() as usize, address);
    assert_eq!(size_of::<T>(), bytes);
}

#[test]
fn vendor_peripheral_bases_match() {
    assert_eq!(pac::GPIOA.as_ptr() as usize, 0x4800_0000);
    assert_eq!(pac::GPIOB.as_ptr() as usize, 0x4800_0400);
    assert_eq!(pac::GPIOC.as_ptr() as usize, 0x4800_0800);
    assert_eq!(pac::GPIOF.as_ptr() as usize, 0x4800_1400);
    assert_eq!(pac::SYSCTRL.as_ptr() as usize, 0x4001_0000);
    assert_eq!(pac::UART1.as_ptr() as usize, 0x4001_3800);
    assert_eq!(pac::CRC.as_ptr() as usize, 0x4002_3000);
}

#[test]
fn gpio_register_offsets_and_byte_aliases_match() {
    for port in [pac::GPIOA, pac::GPIOB] {
        let base = port.as_ptr() as usize;
        register(port.dir(), base, 4);
        register(port.opendrain(), base + 0x04, 4);
        register(port.speed(), base + 0x08, 4);
        register(port.pdr(), base + 0x0c, 4);
        register(port.pur(), base + 0x10, 4);
        register(port.afrh(), base + 0x14, 4);
        register(port.afrl(), base + 0x18, 4);
        register(port.analog(), base + 0x1c, 4);
        register(port.isr(), base + 0x34, 4);
        register(port.icr(), base + 0x38, 4);
        register(port.lock(), base + 0x3c, 4);
        register(port.idr(), base + 0x50, 4);
        register(port.odr(), base + 0x54, 4);
        register(port.odrlowbyte(), base + 0x54, 1);
        register(port.odrhighbyte(), base + 0x55, 1);
        register(port.brr(), base + 0x58, 4);
        register(port.bsrr(), base + 0x5c, 4);
        register(port.tog(), base + 0x60, 4);
    }
    register(pac::GPIOC.afrh(), 0x4800_0814, 4);
    register(pac::GPIOC.odrhighbyte(), 0x4800_0855, 1);
    register(pac::GPIOF.afrl(), 0x4800_1418, 4);
    register(pac::GPIOF.odrlowbyte(), 0x4800_1454, 1);
}

#[test]
fn gpio_status_access_is_read_only_after_reviewed_errata() {
    // Manual EN V1.0 sections 9.6.14/18: ISR/IDR are RO on all GPIO ports.
    let _: Reg<pac::gpio::regs::Isr, R> = pac::GPIOA.isr();
    let _: Reg<pac::gpio::regs::Isr, R> = pac::GPIOB.isr();
    let _: Reg<pac::gpioc::regs::Isr, R> = pac::GPIOC.isr();
    let _: Reg<pac::gpiof::regs::Isr, R> = pac::GPIOF.isr();
    let _: Reg<pac::gpio::regs::Idr, R> = pac::GPIOA.idr();
    let _: Reg<pac::gpio::regs::Idr, R> = pac::GPIOB.idr();
    let _: Reg<pac::gpioc::regs::Idr, R> = pac::GPIOC.idr();
    let _: Reg<pac::gpiof::regs::Idr, R> = pac::GPIOF.idr();
}

#[test]
fn timestamp_and_timer_status_access_follow_reference_manual() {
    // Manual EN V1.0 sections 12.5.10/11 and 14.8.12.
    let _: Reg<pac::rtc::regs::Tampdate, R> = pac::RTC.tampdate();
    let _: Reg<pac::rtc::regs::Tamptime, R> = pac::RTC.tamptime();
    let _: Reg<pac::gtim::regs::Isr, R> = pac::GTIM1.isr();
    let _: Reg<pac::gtim::regs::Isr, R> = pac::GTIM2.isr();
    let _: Reg<pac::gtim::regs::Isr, R> = pac::GTIM3.isr();
    let _: Reg<pac::gtim::regs::Isr, R> = pac::GTIM4.isr();
}

#[test]
fn crc_aliases_preserve_8_16_32_bit_access_widths() {
    let base = pac::CRC.as_ptr() as usize;
    register(pac::CRC.dr8(), base + 8, 1);
    register(pac::CRC.dr16(), base + 8, 2);
    register(pac::CRC.dr32(), base + 8, 4);
    register(pac::CRC.result16(), base + 12, 2);
    register(pac::CRC.result32(), base + 12, 4);
    let _: Reg<pac::crc::regs::Dr8, RW> = pac::CRC.dr8();
    let _: Reg<pac::crc::regs::Dr16, RW> = pac::CRC.dr16();
    let _: Reg<pac::crc::regs::Dr32, RW> = pac::CRC.dr32();
    let _: Reg<pac::crc::regs::Result16, R> = pac::CRC.result16();
    let _: Reg<pac::crc::regs::Result32, R> = pac::CRC.result32();
}

#[test]
fn crc_alias_writes_touch_exact_width_in_aligned_ram() {
    use core::cell::UnsafeCell;
    let memory = UnsafeCell::new([0xa5a5_a5a5u32; 4]);
    let pointer = memory.get().cast::<u8>();
    let crc = unsafe { pac::crc::Crc::from_ptr(pointer.cast()) };
    crc.dr8().write(|v| v.0 = 0x12);
    assert_eq!(unsafe { pointer.add(8).read_volatile() }, 0x12);
    assert_eq!(unsafe { pointer.add(9).read_volatile() }, 0xa5);
    crc.dr16().write(|v| v.0 = 0x1234);
    for (index, expected) in 0x1234u16.to_ne_bytes().into_iter().enumerate() {
        assert_eq!(unsafe { pointer.add(8 + index).read_volatile() }, expected);
    }
    assert_eq!(unsafe { pointer.add(10).read_volatile() }, 0xa5);
    crc.dr32().write(|v| v.0 = 0x1234_5678);
    for (index, expected) in 0x1234_5678u32.to_ne_bytes().into_iter().enumerate() {
        assert_eq!(unsafe { pointer.add(8 + index).read_volatile() }, expected);
    }
    // Verify writes did not escape the shared data-register location.
    for index in [0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 15] {
        assert_eq!(unsafe { pointer.add(index).read_volatile() }, 0xa5);
    }
}

#[test]
fn uart_access_permissions_are_preserved() {
    let _: Reg<pac::uart::regs::Isr, R> = pac::UART1.isr();
    let _: Reg<pac::uart::regs::Rdr, R> = pac::UART1.rdr();
    let _: Reg<pac::uart::regs::Tdr, W> = pac::UART1.tdr();
    register(pac::UART1.rdr(), 0x4001_3824, 4);
    register(pac::UART1.tdr(), 0x4001_3828, 4);
}

#[test]
fn gpio_bit_setters_preserve_neighbors() {
    let mut dir = pac::gpio::regs::Dir(0x8000_8000);
    dir.set_pin0(true);
    dir.set_pin7(true);
    assert_eq!(dir.0, 0x8000_8081);
    assert!(dir.pin0());
    assert!(dir.pin7());
    assert!(dir.pin15());
    dir.set_pin7(false);
    assert_eq!(dir.0, 0x8000_8001);
}

#[test]
fn alternate_function_nibbles_have_correct_positions() {
    let mut af = pac::gpio::regs::Afrh(0xffff_ffff);
    af.set_afr8(0);
    af.set_afr15(5);
    assert_eq!(af.0, 0x5fff_fff0);
    assert_eq!(af.afr8(), 0);
    assert_eq!(af.afr15(), 5);
}

#[test]
fn gpio_clock_bits_match_vendor_header() {
    let mut en = pac::sysctrl::regs::Ahben(7);
    en.set_gpioa(true);
    en.set_gpiob(true);
    en.set_gpioc(true);
    en.set_gpiof(true);
    assert_eq!(en.0, 7 | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 9));
}

#[test]
fn interrupt_values_include_documented_fault_vector() {
    assert_eq!(pac::Interrupt::GPIOA as u16, 5);
    assert_eq!(pac::Interrupt::GPIOB as u16, 6);
    assert_eq!(pac::Interrupt::GPIOC as u16, 7);
    assert_eq!(pac::Interrupt::GPIOF as u16, 8);
    assert_eq!(pac::Interrupt::DMACH23 as u16, 10);
    assert_eq!(pac::Interrupt::UART1 as u16, 27);
    assert_eq!(pac::Interrupt::AWT as u16, 30);
    assert_eq!(pac::Interrupt::FAULT as u16, 31);
}

#[cfg(feature = "metadata")]
#[test]
fn metadata_has_corrected_core_and_no_guessed_generic_memory() {
    let metadata = &pac::metadata::METADATA;
    assert_eq!(metadata.name, "CW32F030");
    assert_eq!(metadata.nvic_priority_bits, Some(2));
    assert!(metadata.memory.is_empty());
    assert_eq!(metadata.peripherals.len(), 37);
    let mut irqs = metadata
        .interrupts
        .iter()
        .map(|i| i.number)
        .collect::<std::vec::Vec<_>>();
    irqs.sort_unstable();
    assert_eq!(irqs, (0..32).collect::<std::vec::Vec<_>>());
}

#[cfg(feature = "metadata")]
#[test]
fn metadata_register_block_references_resolve_exactly() {
    for peripheral in pac::metadata::METADATA.peripherals {
        let Some(registers) = &peripheral.registers else {
            continue;
        };
        assert!(
            registers
                .ir
                .blocks
                .iter()
                .any(|block| block.name == registers.block),
            "{} references missing register block {}",
            peripheral.name,
            registers.block,
        );
        for block in registers.ir.blocks {
            for item in block.items {
                if let pac::metadata::ir::BlockItemInner::Register(register) = &item.inner {
                    if let Some(fieldset) = register.fieldset {
                        assert!(
                            registers.ir.fieldsets.iter().any(|f| f.name == fieldset),
                            "{}.{} references missing fieldset {}",
                            block.name,
                            item.name,
                            fieldset
                        );
                    }
                }
            }
        }
    }
}
