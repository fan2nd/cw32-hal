//! Application entrypoint for CW32L012C8.
#![no_std]
#![no_main]

pub mod board;
pub mod board_contract;
pub mod control;
pub mod frame_queue;
pub mod mailbox;
pub mod protection;
pub mod protocol;
use crate::board::Board;
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, KEY_DEBOUNCE_MS};
use crate::mailbox::{UiCommand, UiFeedback};
use crate::protection::{Adc2Sample, Fault};
use crate::protocol::telemetry_frame;
use core::cell::RefCell;
use critical_section::Mutex;

/// Logical requests are not a claim about physical gate levels. Default
/// firmware remains disarmed even when this controller requests commutation.
#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    pub adc1: [u16; 4],
    pub adc2: [u16; 5],
    pub milliseconds: u32,
    pub adc1_sequences: u32,
    pub adc2_sequences: u32,
    pub logical_bridge: Bridge,
    pub outputs_armed: bool,
    pub state: u8,
    pub fault: u8,
}

const TELEMETRY_INTERVAL_MS: u16 = 500;

// Installed once before NVIC unmask. Only the P1 motor IRQs access this state;
// equal-priority Cortex-M handlers cannot preempt each other. Foreground never
// borrows it after installation. NMI/HardFault only stop hardware, then diverge.
// No other handler may access it or change these interrupt priorities.
struct MotorState {
    board: Board,
    controller: MotorController,
    key_hold_ms: u16,
    telemetry_ms: u16,
    diagnostics: Diagnostics,
    bootstrap_ms: u8,
}
static mut MOTOR: Option<MotorState> = None;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let (mut board, mut ui) = Board::new().expect("board reset/clock initialization failed");

    let controller = MotorController::new(board.calibration_mv());
    // The feature only permits arming. Reset is powered off; deliberate
    // key presses are still required to wake and request positive speed.
    let armed = board.arm_outputs();
    if armed {
        board.apply_bridge(Bridge::bootstrap());
    }

    let mut state = MotorState {
        board,
        controller,
        key_hold_ms: 0,
        telemetry_ms: 0,
        diagnostics: Diagnostics {
            adc1: [0; 4],
            adc2: [0; 5],
            milliseconds: 0,
            adc1_sequences: 0,
            adc2_sequences: 0,
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
        let feedback = next_ui_tick().await;
        // Only this task accesses PC13/PA3/UART1/PB11/PB12 after initialization.
        let pressed = ui.button_pressed();
        critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().command.update(pressed));
        if let Some(on) = feedback.led_on {
            ui.set_led(on);
        }
        if let Some(frame) = feedback.telemetry {
            ui.queue_frame(frame);
        }
        ui.service_uart();
    }
}

fn apply_actions(
    board: &mut Board,
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    mut motor: Actions,
    telemetry: Option<[u8; 7]>,
) {
    critical_section::with(|cs| {
        UI_LINK.borrow(cs).borrow_mut().feedback.merge(UiFeedback {
            led_on: motor.led_on.take(),
            telemetry,
        });
    });
    board.apply(motor);

    diagnostics.outputs_armed = board.outputs_armed();
    diagnostics.logical_bridge = controller.bridge();
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
        let motor = controller.on_adc1(Adc1Sample::from(raw), board.step_ticks());
        apply_actions(board, controller, diagnostics, motor, None);
    }
    if board.sampling_fault() {
        let motor = controller.report_fault(Fault::AdcStale);
        apply_actions(board, controller, diagnostics, motor, None);
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
        key_hold_ms,
        telemetry_ms,
        diagnostics,
        bootstrap_ms,
        ..
    } = state;

    if !board.ms_irq() {
        return;
    }
    diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
    publish_ui_tick();
    if (*bootstrap_ms) > 0 {
        (*bootstrap_ms) -= 1;
        if (*bootstrap_ms) == 0 {
            board.apply_bridge(Bridge::off());
        }
        return;
    }
    let command = critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.command.tick_1ms();
        link.command
    });
    if command.requires_stop(controller.motor_enabled(), controller.speed_percent()) {
        let motor = controller.report_fault(Fault::UiStale);
        apply_actions(board, controller, diagnostics, motor, None);
    }
    let key_pressed = command.key_pressed();
    let step_ticks = board.step_ticks();

    let (motor, telemetry) = {
        (*key_hold_ms) = if key_pressed {
            (*key_hold_ms).saturating_add(1)
        } else {
            0
        };
        let key_event = (*key_hold_ms) == KEY_DEBOUNCE_MS;
        let was_off = controller.powered_off();
        // In the source, immediate key telemetry precedes this tick's 100 ms
        // measurement update. A periodic frame on the same tick supersedes it.
        let key_voltage = controller.measurements().bus_decivolts;
        let motor = controller.tick_1ms(key_pressed, step_ticks);
        let mut telemetry = key_event.then(|| {
            telemetry_frame(
                controller.speed_level(),
                key_voltage,
                controller.powered_off(),
            )
        });
        if was_off && !controller.powered_off() {
            (*telemetry_ms) = 0;
        }
        if controller.powered_off() {
            if !was_off {
                // Final off frame wins even when auto-off and cadence coincide.
                telemetry = Some(telemetry_frame(
                    controller.speed_level(),
                    controller.measurements().bus_decivolts,
                    controller.powered_off(),
                ));
            }
        } else {
            (*telemetry_ms) += 1;
            if (*telemetry_ms) >= TELEMETRY_INTERVAL_MS {
                (*telemetry_ms) = 0;
                telemetry = Some(telemetry_frame(
                    controller.speed_level(),
                    controller.measurements().bus_decivolts,
                    controller.powered_off(),
                ));
            }
        }
        (motor, telemetry)
    };
    apply_actions(board, controller, diagnostics, motor, telemetry);
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
        let motor = controller.on_sensorless_timer(board.step_ticks());
        apply_actions(board, controller, diagnostics, motor, None);
    }
    // Retain debug observations without adding a foreground state accessor.
    core::hint::black_box(&*diagnostics);
}

struct UiLink {
    command: UiCommand,
    feedback: UiFeedback,
    pending_tick: bool,
    waker: Option<core::task::Waker>,
}
static UI_LINK: Mutex<RefCell<UiLink>> = Mutex::new(RefCell::new(UiLink {
    command: UiCommand::default_const(),
    feedback: UiFeedback {
        led_on: None,
        telemetry: None,
    },
    pending_tick: false,
    waker: None,
}));

fn publish_ui_tick() {
    let waker = critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.pending_tick = true;
        link.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

async fn next_ui_tick() -> UiFeedback {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut link = UI_LINK.borrow(cs).borrow_mut();
            if link.pending_tick {
                link.pending_tick = false;
                core::task::Poll::Ready(link.feedback.take())
            } else {
                if !link
                    .waker
                    .as_ref()
                    .is_some_and(|old| old.will_wake(cx.waker()))
                {
                    link.waker = Some(cx.waker().clone());
                }
                core::task::Poll::Pending
            }
        })
    })
    .await
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
