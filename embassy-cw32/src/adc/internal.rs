//! Internal signals have a separate, single-conversion contract. They cannot be
//! erased into the external/OPA channel API or passed to a sequence or DMA read.

use embedded_hal::delay::DelayNs;

pub(crate) trait SealedInternalSource {
    fn channel(&self) -> u8;
    #[cfg(adc_l012)]
    fn minimum_sample_us(&self) -> u32 {
        0
    }
    fn setup(&mut self, delay: &mut impl DelayNs);
}

/// A sealed internal signal for `Adc::blocking_read_internal` and
/// `Adc::read_internal`. These operations exclusively borrow the ADC and this
/// source, activate the signal, wait for startup, and perform one conversion.
///
/// Internal sources deliberately do not implement [`super::AdcChannel`]: no
/// sequence, hardware trigger, watchdog, paired conversion or DMA is exposed.
#[allow(private_bounds)]
pub trait InternalSource: SealedInternalSource {}

#[cfg(all(adc_l012, bgr_l012))]
mod l012_reference {
    use super::*;
    use crate::{analog::Bandgap, pac};

    /// The nominal 1.2 V bandgap signal (IN15), borrowing its startup witness.
    /// This is a raw ADC input, not a calibrated reference-voltage measurement.
    pub struct VrefInt<'a, 'd> {
        _bandgap: &'a Bandgap<'d>,
    }
    impl<'a, 'd> VrefInt<'a, 'd> {
        pub fn new(bandgap: &'a Bandgap<'d>) -> Self {
            Self { _bandgap: bandgap }
        }
    }
    impl InternalSource for VrefInt<'_, '_> {}
    impl SealedInternalSource for VrefInt<'_, '_> {
        fn channel(&self) -> u8 {
            15
        }
        fn minimum_sample_us(&self) -> u32 {
            40
        }
        fn setup(&mut self, delay: &mut impl DelayNs) {
            critical_section::with(|_| pac::BGR.cr().modify(|w| w.set_bgren(true)));
            delay.delay_us(32);
        }
    }

    /// The temperature sensor signal (IN14), retaining the shared BGR owner.
    /// The returned ADC code is not degrees Celsius. Both ADCs may sample this
    /// source, and existing OPA/VC owners may retain the same bandgap witness.
    ///
    /// Reads enable TSEN and BGREN with a critical-section RMW. Neither bit is
    /// cleared on source drop: the other ADC may still be converting after a
    /// forgotten future. This intentionally trades shutdown for shared safety.
    pub struct Temperature<'a, 'd> {
        _bandgap: &'a Bandgap<'d>,
    }
    impl<'a, 'd> Temperature<'a, 'd> {
        pub fn new(bandgap: &'a Bandgap<'d>) -> Self {
            Self { _bandgap: bandgap }
        }
    }
    impl InternalSource for Temperature<'_, '_> {}
    impl SealedInternalSource for Temperature<'_, '_> {
        fn channel(&self) -> u8 {
            14
        }
        fn minimum_sample_us(&self) -> u32 {
            40
        }
        fn setup(&mut self, delay: &mut impl DelayNs) {
            critical_section::with(|_| {
                pac::BGR.cr().modify(|w| {
                    w.set_bgren(true);
                    w.set_tsen(true);
                })
            });
            // RM 25.9/25.12.19: about 30 us for TS and BGR startup.
            delay.delay_us(32);
        }
    }
}
#[cfg(all(adc_l012, bgr_l012))]
pub use l012_reference::{Temperature, VrefInt};

#[cfg(all(adc_l012, dac_l012))]
mod l012_dac {
    use super::*;
    use crate::analog::{DacInstance, DacSource};

    /// A stable internal DAC input: channel 1 maps to IN13, channel 2 to IN12.
    /// Build-time validation limits this bridge to the one audited L012 DAC
    /// shared by both ADCs; `D` retains that source's peripheral identity.
    ///
    /// Borrows the actual source mutably, so no shared `DacSource::set` writer
    /// or OPA/VC source borrow can coexist with this guard. It also retains the
    /// DAC channel, clock and any output pin. Drop ends this reservation without
    /// changing DAC output, route or enable state.
    pub struct DacInput<'a, 'd, D: DacInstance, const C: u8> {
        _source: &'a mut DacSource<'d, D, C>,
    }
    impl<'a, 'd, D: DacInstance, const C: u8> DacInput<'a, 'd, D, C> {
        pub fn new(source: &'a mut DacSource<'d, D, C>) -> Self {
            Self { _source: source }
        }
    }
    impl<D: DacInstance, const C: u8> InternalSource for DacInput<'_, '_, D, C> {}
    impl<D: DacInstance, const C: u8> SealedInternalSource for DacInput<'_, '_, D, C> {
        fn channel(&self) -> u8 {
            // Only Dac::split can produce DacSource, and only for C=1 or C=2.
            match C {
                1 => 13,
                2 => 12,
                _ => unreachable!(),
            }
        }
        fn setup(&mut self, delay: &mut impl DelayNs) {
            // Same conservative delay as Dac::new. Not a characterized maximum
            // settling time or a guarantee of analog accuracy at every load.
            delay.delay_us(10);
        }
    }
}
#[cfg(all(adc_l012, dac_l012))]
pub use l012_dac::DacInput;

#[cfg(adc_f030)]
mod f030 {
    use super::*;
    use crate::pac;

    // These values grant no register access or persistent resource ownership.
    // The dedicated ADC read owns activation and the full conversion lifetime.
    macro_rules! source {
        ($name:ident, $channel:literal, $description:literal) => {
            #[doc = $description]
            #[derive(Default)]
            pub struct $name {
                _private: (),
            }
            impl $name {
                pub const fn new() -> Self {
                    Self { _private: () }
                }
            }
            impl InternalSource for $name {}
            impl SealedInternalSource for $name {
                fn channel(&self) -> u8 {
                    $channel
                }
                fn setup(&mut self, delay: &mut impl DelayNs) {
                    if $channel != 13 {
                        critical_section::with(|_| {
                            pac::ADC.cr0().modify(|w| {
                                w.set_bgren(true);
                                if $channel == 14 {
                                    w.set_tsen(true);
                                }
                            })
                        });
                    }
                    // RM 23.3.1 gives ~20 us BGR startup; the ADC analog startup
                    // is ~40 us (22.4.1). There is no separate TS maximum in
                    // 22.10: this conservative wait does not certify accuracy.
                    delay.delay_us(40);
                }
            }
        };
    }
    source!(
        VrefInt,
        15,
        "The nominal 1.2 V bandgap signal, read against VDDA. Returns raw ADC codes."
    );
    source!(
        Temperature,
        14,
        "The temperature sensor signal. Returns raw ADC codes, not degrees Celsius."
    );
    source!(Vdda, 13, "The VDDA/3 signal. With VDDA as ADC reference this is ratiometric, not a supply-voltage measurement.");
}
#[cfg(adc_f030)]
pub use f030::{Temperature, Vdda, VrefInt};
