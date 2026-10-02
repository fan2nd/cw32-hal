//! Stage 02: sampling. Physical bridge gate pins are untouched.
#![no_std]
#![no_main]

pub mod board;
pub mod sampling;

use crate::board::SamplingBoard;
use crate::sampling::{SamplingClock, SamplingDiagnostics};

// Installed once before NVIC unmask. Only the P1 sampling IRQs access this state;
// equal-priority Cortex-M handlers cannot preempt each other. Foreground never
// borrows it after installation. NMI/HardFault only stop hardware, then diverge.
// No other handler may access it or change these interrupt priorities.
struct SamplingState {
    board: SamplingBoard,
    clock: SamplingClock,
    diagnostics: SamplingDiagnostics,
}
static mut SAMPLING: Option<SamplingState> = None;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = SamplingBoard::new().expect("sampling board reset/clock initialization failed");
    let mut state = SamplingState {
        board,
        clock: SamplingClock::default(),
        diagnostics: SamplingDiagnostics::default(),
    };
    // Preparation leaves NVIC lines masked. End the board borrow and move the
    // complete state before unmask; foreground never accesses it again.
    unsafe {
        state.board.prepare_interrupts();
        core::ptr::addr_of_mut!(SAMPLING).write(Some(state));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
        board::enable_interrupts();
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC1() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(SAMPLING)).as_mut().unwrap() };
    let SamplingState {
        board, diagnostics, ..
    } = state;

    if let Some(raw) = board.adc1_irq() {
        diagnostics.record_adc1(raw);
    }
    diagnostics.sampling_fault = board.sampling_fault();
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC2_DAC() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(SAMPLING)).as_mut().unwrap() };
    let SamplingState {
        board, diagnostics, ..
    } = state;

    if let Some(raw) = board.adc2_irq() {
        diagnostics.record_adc2(raw);
    }
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(SAMPLING)).as_mut().unwrap() };
    let SamplingState {
        board,
        clock,
        diagnostics,
        ..
    } = state;

    if !board.ms_irq() {
        return;
    }
    diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
    if clock.tick_1ms() {
        board.start_adc2();
    }
    board.set_led(board.button_pressed());
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    board::emergency_stop();
    loop {
        cortex_m::asm::wfi();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn NonMaskableInt() -> ! {
    fatal()
}

#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) {
    fatal()
}
