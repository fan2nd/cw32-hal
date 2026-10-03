//! L012 external-feedback current-sense front end, with board-selected inputs.
use crate::pac;
#[derive(Clone, Copy, Debug)]
pub enum PositiveInput {
    Inp1,
    Inp2,
    Inp3,
    Inp4,
}
#[derive(Clone, Copy, Debug)]
pub enum NegativeInput {
    Inn1,
    Inn2,
}
/// Configure BGR plus OPA1 external feedback with calibration disabled.
/// No output pin routing, settling delay or gain compensation is implicit.
///
/// # Safety
/// Caller exclusively controls OPA1 and coordinates the shared BGR; no safe
/// analog owner, IRQ or DMA may reconfigure them. Pins must already be analog,
/// external feedback must be physically present, and the bridge must be off.
/// Wait the required analog startup time before using conversion results.
pub unsafe fn configure_current_sense(positive: PositiveInput, negative: NegativeInput) {
    let mut clock = <crate::peripherals::OPA1 as crate::rcc::PeripheralClock>::acquire_no_reset();
    clock.pin();
    pac::BGR.cr().modify(|w| w.set_bgren(true));
    // Keep the audited reset bias (BIAS=7), as the prior PAC write closures did.
    let mut control = pac::opa::regs::Cr::default();
    match positive {
        PositiveInput::Inp1 => control.set_inp1en(true),
        PositiveInput::Inp2 => control.set_inp2en(true),
        PositiveInput::Inp3 => control.set_inp3en(true),
        PositiveInput::Inp4 => control.set_inp4en(true),
    }
    match negative {
        NegativeInput::Inn1 => control.set_inn1en(true),
        NegativeInput::Inn2 => control.set_inn2en(true),
    }
    pac::OPA1.cr().write_value(control);
    pac::OPA1.cal().write(|_| {});
    control.set_en(true);
    pac::OPA1.cr().write_value(control);
}
