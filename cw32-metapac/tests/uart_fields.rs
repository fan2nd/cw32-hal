//! Pure generated UART value tests. No HAL, MMIO or hardware simulation.
use cw32_metapac::uart::{regs, vals};

#[test]
fn field_types_keep_word_transaction_width() {
    assert_eq!(core::mem::size_of::<regs::Icr>(), 4);
    assert_eq!(core::mem::size_of::<regs::Ier>(), 4);
    assert_eq!(core::mem::size_of::<regs::Isr>(), 4);
    assert_eq!(core::mem::size_of::<regs::Rdr>(), 4);
    assert_eq!(core::mem::size_of::<regs::Tdr>(), 4);
}

#[test]
fn manual_configuration_encodings() {
    for (over, bits) in [
        (vals::Over::Over16, 0),
        (vals::Over::Over8, 1),
        (vals::Over::Over4, 2),
    ] {
        let mut value = regs::Cr1::default();
        value.set_over(over);
        assert_eq!(value.0, bits << 9);
        assert_eq!(value.over(), over);
    }
    for (stop, bits) in [
        (vals::Stop::Stop1, 0),
        (vals::Stop::Stop1p5, 1),
        (vals::Stop::Stop2, 2),
    ] {
        let mut value = regs::Cr1::default();
        value.set_stop(stop);
        assert_eq!(value.0, bits << 4);
        assert_eq!(value.stop(), stop);
    }
}

#[test]
fn no_op_seeds_preserve_every_unselected_and_reserved_bit() {
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    let (reset, domain) = (0x1fff, 0x1ffe);
    #[cfg(any(uart_cw32l031_v1, uart_cw32l052_v1))]
    let (reset, domain) = (0x0fff, 0x0e5e);
    #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1, uart_cw32l031_v1, uart_cw32l052_v1)))]
    let (reset, domain) = (0x00ff, 0x005e);
    assert_eq!(regs::Icr::reset_value().0, reset);
    assert_eq!(regs::Icr::write_noop().0, reset);
    assert_eq!(
        regs::Icr::default().0,
        0,
        "command seeds must not change zero Default"
    );
    let mut fields: Vec<(u32, fn(&mut regs::Icr, bool))> =
        vec![(1 << 1, regs::Icr::set_tc), (1 << 2, regs::Icr::set_rc)];
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    fields.extend([
        (1 << 3, regs::Icr::set_rxidle as fn(&mut regs::Icr, bool)),
        (1 << 4, regs::Icr::set_rxbrk),
        (1 << 5, regs::Icr::set_baud),
        (1 << 6, regs::Icr::set_timov),
        (1 << 7, regs::Icr::set_cts),
        (1 << 8, regs::Icr::set_fe),
        (1 << 9, regs::Icr::set_pe),
        (1 << 10, regs::Icr::set_ne),
        (1 << 11, regs::Icr::set_ore),
        (1 << 12, regs::Icr::set_rxmatch),
    ]);
    #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
    fields.extend([
        (1 << 3, regs::Icr::set_fe as fn(&mut regs::Icr, bool)),
        (1 << 4, regs::Icr::set_pe),
        (1 << 6, regs::Icr::set_cts),
    ]);
    #[cfg(any(uart_cw32l031_v1, uart_cw32l052_v1))]
    fields.extend([
        (1 << 9, regs::Icr::set_timov as fn(&mut regs::Icr, bool)),
        (1 << 10, regs::Icr::set_baud),
        (1 << 11, regs::Icr::set_rxbrk),
    ]);
    assert_eq!(fields.iter().fold(0, |mask, (bit, _)| mask | bit), domain);
    for selected in 0..(1 << fields.len()) {
        let mut value = regs::Icr::write_noop();
        let mut cleared = 0;
        for (index, (bit, setter)) in fields.iter().enumerate() {
            if selected & (1 << index) != 0 {
                setter(&mut value, false);
                cleared |= bit;
            }
        }
        assert_eq!(value.0, reset & !cleared);
        assert_eq!(value.0 & !domain, reset & !domain);
    }
}
