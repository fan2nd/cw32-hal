//! Bounded messages between the motor interrupt domain and the UI task.
//! No peripheral handle, register pointer or controller reference crosses here.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiFeedback {
    pub led_on: Option<bool>,
    pub telemetry: Option<[u8; 7]>,
}

impl UiFeedback {
    /// Latest complete message wins while a consumer is delayed. A frame is
    /// copied as seven bytes, never interleaved with an older partial frame.
    pub fn merge(&mut self, newer: Self) {
        if newer.led_on.is_some() {
            self.led_on = newer.led_on;
        }
        if newer.telemetry.is_some() {
            self.telemetry = newer.telemetry;
        }
    }
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiCommand {
    key_pressed: bool,
    age_ms: u16,
}

impl UiCommand {
    pub const fn default_const() -> Self {
        Self {
            key_pressed: false,
            age_ms: 0,
        }
    }
    pub fn update(&mut self, key_pressed: bool) {
        self.key_pressed = key_pressed;
        self.age_ms = 0;
    }
    pub fn tick_1ms(&mut self) {
        self.age_ms = self.age_ms.saturating_add(1);
    }
    // Debounce requires a stream of fresh observations. One pressed sample
    // followed by a stalled task must never masquerade as a 60 ms key hold.
    pub const fn key_pressed(&self) -> bool {
        self.key_pressed && self.age_ms < 2
    }
}

use crate::frame_queue::FrameQueue;
use core::cell::RefCell;
use critical_section::Mutex;
use embassy_cw32::{
    gpio::{Input, Level, Output, Pull},
    uart::{ClockSource, Config, Uart},
};

pub async fn run(
    led: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PC13>,
    key: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA3>,
    uart: embassy_cw32::Peri<'static, embassy_cw32::peripherals::UART1>,
    tx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB12>,
    rx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB11>,
) {
    let mut led = Output::new(led, Level::High);
    let key = Input::new(key, Pull::Up);
    // Keep the source application's 96 MHz PCLK / (16 * 52 + 1), 8N1,
    // PB12 TX / PB11 RX routing and polling cadence. UART2 is still the motor
    // software executor's vector; UART1 does not enable any interrupt here.
    let mut config = Config::default();
    config.baudrate = 115_200;
    config.clock_source = ClockSource::PclkAlt;
    config.rx_pull = Pull::Up;
    let mut uart = Uart::new_blocking(uart, tx_pin, rx_pin, config).unwrap();
    let mut tx = FrameQueue::new();
    let mut last_fault = None;
    loop {
        let (feedback, snapshot) = next_ui_tick().await;
        let pressed = key.is_low();
        critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().command.update(pressed));
        if let Some(on) = feedback.led_on {
            // PC13 LED is active-low; SET/CLR does not race motor GPIO mux RMW.

            if on {
                led.set_low();
            } else {
                led.set_high();
            }
        }
        if let Some(frame) = feedback.telemetry {
            tx.push(frame);
        }
        // Nonblocking, at most one UART byte per task wake.
        if uart.is_write_ready() {
            if let Some(byte) = tx.pop_byte() {
                let _ = uart.try_write(byte).unwrap();
            }
        }
        // The snapshot holds copied values only. RTT runs outside UI_LINK's
        // critical section, never in ADC/commutation/the motor executor IRQ.
        if let Some(snapshot) = snapshot {
            snapshot.report(last_fault);
            last_fault = snapshot.fault;
        }
        core::hint::black_box((&led, &key, &uart));
    }
}

struct UiLink {
    command: UiCommand,
    feedback: UiFeedback,
    diagnostics: Option<crate::logging::Snapshot>,
    pending_tick: bool,
    waker: Option<core::task::Waker>,
}
static UI_LINK: Mutex<RefCell<UiLink>> = Mutex::new(RefCell::new(UiLink {
    command: UiCommand::default_const(),
    feedback: UiFeedback {
        led_on: None,
        telemetry: None,
    },
    diagnostics: None,
    pending_tick: false,
    waker: None,
}));

pub fn publish_ui_tick() {
    let waker = critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.pending_tick = true;
        link.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

async fn next_ui_tick() -> (UiFeedback, Option<crate::logging::Snapshot>) {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut link = UI_LINK.borrow(cs).borrow_mut();
            if link.pending_tick {
                link.pending_tick = false;
                core::task::Poll::Ready((link.feedback.take(), link.diagnostics.take()))
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

pub fn key_sample() -> bool {
    critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.command.tick_1ms();
        link.command.key_pressed()
    })
}

pub fn publish_status(feedback: UiFeedback) {
    critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().feedback.merge(feedback));
}

/// Same latest-value exchange as LED/UART feedback; no event queue or live references.
pub fn publish_diagnostics(snapshot: crate::logging::Snapshot) {
    critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().diagnostics = Some(snapshot));
}
