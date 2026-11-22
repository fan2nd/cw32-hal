//! Owned HCLK/PCLK output on PA8 for exact F030C8T7/K8T7/K8U7 and A030C8T7.
//!
//! Admission uses the initialized bus's exact, cycle-qualified envelope and an
//! explicit board declaration. The fixed 20 MHz, SPEED1, <=50 pF policy is
//! conditional: the datasheet calls its electrical values design/simulation
//! data, not measurements. Its fmax definition additionally requires 45..55%
//! duty and (tr + tf) <= 2T/3 under load. Neither this rate check nor integer
//! division proves those waveform conditions. Qualify the actual loaded
//! waveform and receiver, supply tolerance, routing, VOH/VOL/current, inherited
//! DRIVER setting, and power/thermal limits separately. Without that qualification
//! this is an observation output, not a fully qualified receiver clock.
//!
//! Opening GPIOA's shared work gate may resume other pins' sampling, filters,
//! edge/level detection and pending interrupts, including on a later error.
//! The gate stays enabled. Only PA8's lock bit is cleared; other low lock bits
//! are retained. PA8's former function is disconnected and its edge/level enables
//! are cleared, without clearing pending status or restoring its earlier mode.
//! The board and prior/downstream users must tolerate these functional effects.
//! These requirements are not hidden Rust memory-safety prerequisites.
//!
//! Register checks are snapshots, not measurements or fault recovery. Normal
//! interrupt-driven register updates are excluded by a critical section; DMA,
//! NMI, debugger/raw writes and physical clock faults are not. The original RCC
//! source and board contracts must continue to hold. There is no oscillator,
//! bus-divider, low-power restoration or clock-fault management here.

use embassy_hal_internal::Peri;

use super::{ClockBounds, OperatingConditions, try_clocks};
use crate::gpio::{Flex, Pull};
use crate::{pac, peripherals};

/// Already initialized bus selected for output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McoSource {
    /// Published HCLK envelope.
    Hclk,
    /// Published PCLK envelope.
    Pclk,
}

/// Integer MCO divider. Each variant selects a build-validated metadata entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McoPrescaler {
    /// Divide by 1.
    Div1,
    /// Divide by 2.
    Div2,
    /// Divide by 8.
    Div8,
    /// Divide by 64.
    Div64,
    /// Divide by 128.
    Div128,
    /// Divide by 256.
    Div256,
    /// Divide by 512.
    Div512,
    /// Divide by 1024.
    Div1024,
}

impl McoPrescaler {
    fn entry(self) -> (u8, u16) {
        crate::MCO_DIVISORS[match self {
            Self::Div1 => 0,
            Self::Div2 => 1,
            Self::Div8 => 2,
            Self::Div64 => 3,
            Self::Div128 => 4,
            Self::Div256 => 5,
            Self::Div512 => 6,
            Self::Div1024 => 7,
        }]
    }
}

/// Explicit guarantees for the board throughout output use, not measurements.
///
/// The supply voltage must describe the actual PA8 I/O supply. The declaration
/// must truthfully describe the same board as RCC initialization. Frozen clocks
/// retain the source qualification range, not the earlier board declaration;
/// compatibility with that range cannot prove consistency with the earlier one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct McoBoardConditions {
    /// Guaranteed VDD and ordinary ambient TA intervals, including tolerance.
    pub operating_conditions: OperatingConditions,
    /// Conservative total PA8 load, including receiver, routing and probes.
    ///
    /// Must be nonzero and at most 50 pF. Rejecting zero requires a complete
    /// declaration; it does not assert a silicon minimum capacitance. The <=50
    /// pF policy is a conservative interpretation of the CL=50 pF table row.
    pub max_load_pf: u16,
}

/// Admission or fixed readback failure. Cleanup flags are observed snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum McoError {
    /// No successful RCC initialization has published clocks.
    ClocksNotInitialized,
    /// The selected bus has rate-only bounds, such as a PLL/LSI-rooted bus.
    SourceNotCycleQualified,
    /// Reversed condition intervals or an unspecified zero total load.
    InvalidBoardConditions,
    /// Declared conditions exceed the selected source's qualification range.
    BoardOutsideSourceRange,
    /// Declared conditions or load exceed the fixed MCO GPIO policy.
    BoardOutsideGpioRange,
    /// The exact divided maximum exceeds the conditional output ceiling.
    OutputTooFast,
    /// A non-disabled inherited selector is never taken over.
    OutputAlreadyEnabled,
    /// The inherited register has nonzero reserved bits.
    ReservedMcoState,
    /// GPIOA is held in reset; the shared reset is never released here.
    GpioResetAsserted,
    /// Initial GPIO setup failed; MCO was not written.
    GpioReadback {
        /// Full disconnected PA8 state observed after the local Flex destructor.
        pin_disconnected_readback: bool,
    },
    /// MCO source/divider did not match after programming or AF attachment.
    McoReadback {
        /// Disabled SOURCE observed after the single cleanup attempt.
        source_disabled_readback: bool,
        /// Full disconnected PA8 state observed after the local Flex destructor.
        pin_disconnected_readback: bool,
    },
    /// Final PA8 configuration did not match after successful MCO readback.
    PinReadback {
        /// Disabled SOURCE observed after the single cleanup attempt.
        source_disabled_readback: bool,
        /// Full disconnected PA8 state observed after the local Flex destructor.
        pin_disconnected_readback: bool,
    },
}

/// Owns the real SYSCTRL token and PA8 for an immutable bus-clock output.
///
/// Holding SYSCTRL excludes simultaneous live safe RTC clock ownership. Normal
/// Drop first disconnects PA8, then disables only MCO SOURCE, retaining DIV and
/// the shared GPIOA gate. Flex subsequently repeats the disconnection. Shutdown
/// is bounded best effort and cannot report a failed write or promise a final
/// level, glitch-free pulses, duty, or phase continuity. PA8 remains unlocked;
/// AF4/SPEED1, ODR and DRIVER are retained after normal successful use.
///
/// `mem::forget` skips shutdown and may leave the output active indefinitely.
/// With reborrowed tokens the original tokens can later be reused; another MCO
/// construction still rejects an active selector, while ordinary PA8 reuse can
/// disconnect the orphaned output. Token exclusion is not permanent hardware
/// exclusion after forgetting. No moved singleton tokens are reconstructed.
pub struct Mco<'d> {
    _sysctrl: Peri<'d, peripherals::SYSCTRL>,
    pin: Flex<'d>,
    bounds: ClockBounds,
}

impl<'d> Mco<'d> {
    /// Acquire the idle MCO selector and PA8 using published initialized clocks.
    ///
    /// Pure admission precedes every write. An early error consumes moved tokens
    /// without touching GPIO/MCO; reborrowed original tokens become usable when
    /// their failed borrow ends. After Flex acquisition, cleanup drops that local
    /// Flex before reporting disconnection. Once MCO programming was attempted,
    /// cleanup additionally attempts SOURCE disable once, preserving DIV.
    /// Cleanup booleans report only observed register state, not a lasting
    /// electrical guarantee. Bus faults, aborts, reset and broken internal
    /// generated invariants are not recoverable constructor errors.
    pub fn new(
        sysctrl: Peri<'d, peripherals::SYSCTRL>,
        pin: Peri<'d, peripherals::PA8>,
        source: McoSource,
        prescaler: McoPrescaler,
        board: McoBoardConditions,
    ) -> Result<Self, McoError> {
        let clocks = try_clocks().ok_or(McoError::ClocksNotInitialized)?;
        let (bounds, source) = match source {
            McoSource::Hclk => (clocks.hclk_bounds(), crate::MCO_HCLK_SOURCE),
            McoSource::Pclk => (clocks.pclk_bounds(), crate::MCO_PCLK_SOURCE),
        };
        if !bounds.has_cycle_timing_bounds() {
            return Err(McoError::SourceNotCycleQualified);
        }
        let conditions = board.operating_conditions;
        if conditions.min_temperature_c > conditions.max_temperature_c
            || board.max_load_pf == 0
        {
            return Err(McoError::InvalidBoardConditions);
        }
        if !conditions_fit(
            conditions,
            bounds.supply_range_mv(),
            bounds.temperature_range_c(),
        ) {
            return Err(McoError::BoardOutsideSourceRange);
        }
        if !conditions_fit(conditions, crate::MCO_SUPPLY_MV, crate::MCO_TEMPERATURE_C)
            || board.max_load_pf > crate::MCO_MAXIMUM_LOAD_PF
        {
            return Err(McoError::BoardOutsideGpioRange);
        }
        let (div, divisor) = prescaler.entry();
        let bounds = bounds.divided_by(u32::from(divisor));
        if bounds.maximum_exceeds(crate::MCO_MAXIMUM_OUTPUT_HZ) {
            return Err(McoError::OutputTooFast);
        }
        critical_section::with(|_| {
            let inherited = pac::SYSCTRL.mco().read();
            if inherited.source() != crate::MCO_DISABLED_SOURCE {
                return Err(McoError::OutputAlreadyEnabled);
            }
            // Reconstruct only the defined fields for a reserved-bit comparison.
            // This default image is never written to hardware.
            let mut defined = pac::sysctrl::regs::Mco::default();
            defined.set_source(inherited.source());
            defined.set_div(inherited.div());
            if inherited.0 != defined.0 {
                return Err(McoError::ReservedMcoState);
            }
            if crate::gpio_rcc(0).reset_asserted() {
                return Err(McoError::GpioResetAsserted);
            }
            // Exact classic GPIO enable policy is a read barrier (Once), not
            // acknowledgement polling. Verify the actual state ourselves.
            let mut pin = Flex::new(pin);
            if !pin_disconnected() {
                drop(pin);
                return Err(McoError::GpioReadback {
                    pin_disconnected_readback: pin_disconnected(),
                });
            }
            pac::SYSCTRL.mco().modify(|r| {
                r.set_source(source);
                r.set_div(div);
            });
            if !mco_matches(source, div) {
                let (source_disabled_readback, pin_disconnected_readback) = cleanup(pin);
                return Err(McoError::McoReadback {
                    source_disabled_readback,
                    pin_disconnected_readback,
                });
            }
            pin.set_as_af(crate::MCO_AF, true, Pull::None);
            let pin_matches = pin_configured();
            // Always re-read MCO, with its failure taking priority over the pin.
            if !mco_matches(source, div) {
                let (source_disabled_readback, pin_disconnected_readback) = cleanup(pin);
                return Err(McoError::McoReadback {
                    source_disabled_readback,
                    pin_disconnected_readback,
                });
            }
            if !pin_matches {
                let (source_disabled_readback, pin_disconnected_readback) = cleanup(pin);
                return Err(McoError::PinReadback {
                    source_disabled_readback,
                    pin_disconnected_readback,
                });
            }
            Ok(Self {
                _sysctrl: sysctrl,
                pin,
                bounds,
            })
        })
    }

    /// Exact divided logical-clock envelope retained at successful acquisition.
    ///
    /// Conditional on the source and board contracts continuing to hold. This
    /// is not a measurement, live health check, pad rise/fall or duty bound, or
    /// a guarantee of pin pulses during startup/shutdown. Whole-hertz rounding
    /// can report zero for valid sub-hertz outputs. Duration getters explicitly
    /// panic if a rounded result exceeds `u64::MAX` nanoseconds.
    pub fn clock_bounds(&self) -> ClockBounds {
        self.bounds
    }
}

impl Drop for Mco<'_> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            self.pin.set_as_disconnected();
            pac::SYSCTRL
                .mco()
                .modify(|r| r.set_source(crate::MCO_DISABLED_SOURCE));
            let _ = pac::SYSCTRL.mco().read();
        });
    }
}

fn conditions_fit(c: OperatingConditions, supply: (u16, u16), temperature: (i16, i16)) -> bool {
    c.supply_mv >= supply.0
        && c.supply_mv <= supply.1
        && c.min_temperature_c >= temperature.0
        && c.max_temperature_c <= temperature.1
}

fn mco_matches(source: u8, div: u8) -> bool {
    let r = pac::SYSCTRL.mco().read();
    r.source() == source && r.div() == div
}

// Called only inside acquisition's critical section, after programming MCO.
fn cleanup(pin: Flex<'_>) -> (bool, bool) {
    drop(pin);
    pac::SYSCTRL
        .mco()
        .modify(|r| r.set_source(crate::MCO_DISABLED_SOURCE));
    let source_disabled = pac::SYSCTRL.mco().read().source() == crate::MCO_DISABLED_SOURCE;
    (source_disabled, pin_disconnected())
}

// Every predicate performs its full bounded set of reads, including when an
// earlier read disagrees. Only PA8 fields are compared, never whole GPIO banks.
fn pin_common() -> bool {
    let gate = crate::gpio_rcc(0).is_enabled();
    let reset = crate::gpio_rcc(0).reset_asserted();
    let r = pac::GPIOA;
    let locked = r.lock().read().pin8();
    let pull_up = r.pur().read().pin8();
    let pull_down = r.pdr().read().pin8();
    let open_drain = r.opendrain().read().pin8();
    let rising = r.riseie().read().pin8();
    let falling = r.fallie().read().pin8();
    let high = r.highie().read().pin8();
    let low = r.lowie().read().pin8();
    gate && !reset
        && !locked
        && !pull_up
        && !pull_down
        && !open_drain
        && !rising
        && !falling
        && !high
        && !low
}

fn pin_disconnected() -> bool {
    let common = pin_common();
    let input = pac::GPIOA.dir().read().pin8();
    let analog = pac::GPIOA.analog().read().pin8();
    common && input && analog
}

fn pin_configured() -> bool {
    let common = pin_common();
    let r = pac::GPIOA;
    let input = r.dir().read().pin8();
    let analog = r.analog().read().pin8();
    let af = r.afrh().read().afr8();
    let speed = r.speed().read().pin8();
    common && !input && !analog && af == crate::MCO_AF && u8::from(speed) == crate::MCO_GPIO_SPEED
}
