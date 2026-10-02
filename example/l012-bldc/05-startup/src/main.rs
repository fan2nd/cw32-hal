#![no_std]
#![no_main]

pub mod board;
pub mod board_contract;
pub mod control;
pub mod protection;
use crate::board::Board;
use crate::control::{Actions, Adc1Sample, Bridge, MotorController};
use crate::protection::{Adc2Sample, Fault};

/// IRQ-owned debugger data. Logical bridge requests are not physical outputs.
/// Read while halted with power disconnected; a live debugger read can tear.
#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    pub adc1: [u16; 4],
    pub adc2: [u16; 5],
    pub milliseconds: u32,
    pub adc1_sequences: u32,
    pub adc2_sequences: u32,
    pub sector: u8,
    pub logical_bridge: Bridge,
    pub outputs_armed: bool,
    pub state: u8,
    pub fault: u8,
}

// Installed once before NVIC unmask. Only the P1 motor IRQs access this state;
// equal-priority Cortex-M handlers cannot preempt each other. Foreground never
// borrows it after installation. NMI/HardFault only stop hardware, then diverge.
// No other handler may access it or change these interrupt priorities.
struct MotorState {
    board: Board,
    controller: MotorController,
    diagnostics: Diagnostics,
    bootstrap_ms: u8,
}
static mut MOTOR: Option<MotorState> = None;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = Board::new().expect("board reset/clock initialization failed");

    let controller = MotorController::new(board.calibration_mv());
    // Feature permits output only; the controller still requires deliberate
    // key presses before selecting speed. Reset/feature-off remain disarmed.
    let armed = board.arm_outputs();
    if armed {
        board.apply_bridge(Bridge::bootstrap());
    }

    let mut state = MotorState {
        board,
        controller,
        diagnostics: Diagnostics {
            adc1: [0; 4],
            adc2: [0; 5],
            milliseconds: 0,
            adc1_sequences: 0,
            adc2_sequences: 0,
            sector: 0,
            logical_bridge: Bridge::off(),
            outputs_armed: armed,
            state: 0,
            fault: 0,
        },
        bootstrap_ms: if armed { 6 } else { 0 },
    };
    // Preparation leaves NVIC lines masked. End the board borrow and move the
    // complete state before unmask; foreground never accesses it again.
    unsafe {
        state.board.prepare_interrupts();
        core::ptr::addr_of_mut!(MOTOR).write(Some(state));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
        board::enable_interrupts();
    }

    loop {
        cortex_m::asm::wfi();
    }
}

fn apply_actions(
    board: &mut Board,
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    actions: Actions,
) {
    board.apply(actions);
    diagnostics.outputs_armed = board.outputs_armed();
    diagnostics.logical_bridge = controller.bridge();
    diagnostics.sector = controller.sector();
    diagnostics.state = controller.state() as u8;
    diagnostics.fault = controller.fault().map_or(0, |fault| fault as u8);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC1() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap() };
    let MotorState {
        board,
        controller,
        diagnostics,
        ..
    } = state;

    if let Some(raw) = board.adc1_irq() {
        diagnostics.adc1 = raw;
        diagnostics.adc1_sequences = diagnostics.adc1_sequences.wrapping_add(1);
        let actions = controller.on_adc1(Adc1Sample::from(raw), board.step_ticks());
        apply_actions(board, controller, diagnostics, actions);
    }
    if board.sampling_fault() {
        let fault = controller.report_fault(Fault::AdcStale);
        apply_actions(board, controller, diagnostics, fault);
    }
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC2_DAC() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap() };
    let MotorState {
        board,
        controller,
        diagnostics,
        ..
    } = state;

    if let Some(raw) = board.adc2_irq() {
        diagnostics.adc2 = raw;
        diagnostics.adc2_sequences = diagnostics.adc2_sequences.wrapping_add(1);
        controller.update_adc2(Adc2Sample {
            current: raw[0],
            bus_voltage: raw[1],
            potentiometer: raw[2],
            temperature: raw[3],
            reference: raw[4],
        });
    }
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap() };
    let MotorState {
        board,
        controller,
        diagnostics,
        bootstrap_ms,
        ..
    } = state;

    if !board.ms_irq() {
        return;
    }
    diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
    if (*bootstrap_ms) > 0 {
        (*bootstrap_ms) -= 1;
        if (*bootstrap_ms) == 0 {
            board.apply_bridge(Bridge::off());
        }
        return;
    }
    let actions = controller.tick_1ms(board.button_pressed(), board.step_ticks());
    apply_actions(board, controller, diagnostics, actions);
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM3_HALLTIM() {
    // SAFETY: initialized before unmask; every accessor is a non-nesting P1 IRQ.
    let state = unsafe { (&mut *core::ptr::addr_of_mut!(MOTOR)).as_mut().unwrap() };
    let MotorState {
        board,
        controller,
        diagnostics,
        ..
    } = state;

    if board.commutation_irq() {
        let actions = controller.on_sensorless_timer(board.step_ticks());
        apply_actions(board, controller, diagnostics, actions);
    }
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
