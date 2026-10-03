//! Quadrature decoding with an owned timer and a metadata-proved input pair.
//! The fixed range 0..=65535 respects the F030 GTIM encoder ARR requirement.
pub use super::input_capture::Error;
use super::{
    input_capture::{CaptureChannel, CaptureInput, CapturePin, Filter},
    low_level::Timer,
};
use crate::{gpio::Pull, Peri};

pub(crate) mod sealed {
    pub trait Instance {}
}
/// Timer with an audited encoder input pair. F030 ATIM uses A1/B1; other
/// supported timer IPs use CH1/CH2. Swapping arbitrary channels is not accepted.
#[allow(private_bounds)]
pub trait Instance: super::input_capture::Instance + sealed::Instance {
    type First: CaptureChannel;
    type Second: CaptureChannel;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QeiMode {
    /// Count both edges of the first input, using the second for direction (x2).
    Mode1 = 1,
    /// Count both edges of the second input, using the first for direction (x2).
    Mode2 = 2,
    /// Count both edges of both inputs (x4).
    Mode3 = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Upcounting,
    Downcounting,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub mode: QeiMode,
    pub first_pull: Pull,
    pub second_pull: Pull,
    pub first_filter: Filter,
    pub second_filter: Filter,
    pub invert_first: bool,
    pub invert_second: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            mode: QeiMode::Mode3,
            first_pull: Pull::None,
            second_pull: Pull::None,
            first_filter: Filter::None,
            second_filter: Filter::None,
            invert_first: false,
            invert_second: false,
        }
    }
}
/// A hardware quadrature decoder. Construction starts counting external edges.
/// No capture/DMA/IRQ, index reset, or output is enabled. Reading count and
/// direction are separate observations; they are not an atomic combined sample.
/// Keep the count delta within half the 16-bit range to infer a signed movement.
pub struct Qei<'d, T: Instance> {
    timer: Timer<'d, T>,
    first: CaptureInput<'d, T, T::First>,
    second: CaptureInput<'d, T, T::Second>,
}
impl<'d, T: Instance> Qei<'d, T> {
    pub fn new<P1: CapturePin<T, T::First>, P2: CapturePin<T, T::Second>>(
        timer: Peri<'d, T>,
        first: Peri<'d, P1>,
        second: Peri<'d, P2>,
        config: Config,
    ) -> Result<Self, Error> {
        let first_filter = T::regs()
            .capture_filter(config.first_filter)
            .ok_or(Error::UnsupportedFilter)?;
        let second_filter = T::regs()
            .capture_filter(config.second_filter)
            .ok_or(Error::UnsupportedFilter)?;
        let first = CaptureInput::from_pin(first, config.first_pull);
        let second = CaptureInput::from_pin(second, config.second_pull);
        let mut timer = Timer::new(timer);
        critical_section::with(|_| {
            T::select_external_input(T::First::INDEX);
            T::select_external_input(T::Second::INDEX);
            T::regs().configure_encoder(
                config.mode,
                first_filter,
                second_filter,
                config.invert_first,
                config.invert_second,
            );
        });
        timer.start();
        Ok(Self {
            timer,
            first,
            second,
        })
    }
    pub fn count(&self) -> u16 {
        self.timer.counter()
    }
    pub fn read_direction(&self) -> Direction {
        if T::regs().encoder_downcounting() {
            Direction::Downcounting
        } else {
            Direction::Upcounting
        }
    }
    /// Stop counting. Incoming transitions during this interval are lost.
    pub fn stop(&mut self) {
        self.timer.stop();
    }
    pub fn start(&mut self) {
        self.timer.start();
    }
    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
    /// Set count without an update event; stops briefly, preserving run state.
    /// Transitions while stopped are lost. No capture, ADC, or DMA is triggered.
    pub fn set_count(&mut self, count: u16) {
        let running = self.timer.is_running();
        self.timer.stop();
        // The fixed full 16-bit range accepts every u16.
        let _ = self.timer.set_counter(count);
        if running {
            self.timer.start();
        }
    }
    pub fn reset(&mut self) {
        self.set_count(0);
    }
}
impl<T: Instance> Drop for Qei<'_, T> {
    fn drop(&mut self) {
        self.timer.stop();
        self.first.pin.set_as_disconnected();
        self.second.pin.set_as_disconnected();
    }
}
