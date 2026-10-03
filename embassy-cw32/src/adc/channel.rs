//! Borrowed, type-erased ADC channels. Only metadata-verified external pins implement this API.
use core::marker::PhantomData;

use super::{ChannelPin, Instance};
use crate::{gpio::AnyPin, Peri};

pub(crate) trait SealedAdcChannel<I> {
    fn setup(&mut self);
    fn channel(&self) -> u8;
}

/// A channel belonging to one ADC instance.
///
/// Implemented for verified typed GPIO `Peri` tokens. Internal sources are not
/// exposed without their required reference-resource ownership and settling policy.
#[allow(private_bounds)]
pub trait AdcChannel<'d, I: Instance>: SealedAdcChannel<I> + Sized {
    /// Consume this channel token and configure the pin for analog use.
    /// The returned token retains the original exclusive lifetime.
    fn degrade_adc(mut self) -> BorrowedAdcChannel<'d, I> {
        self.setup();
        BorrowedAdcChannel {
            channel: self.channel(),
            _borrow: PhantomData,
        }
    }

    /// Configure and exclusively borrow this channel for an operation or sequence.
    fn reborrow_adc<'a>(&'a mut self) -> BorrowedAdcChannel<'a, I> {
        self.setup();
        BorrowedAdcChannel {
            channel: self.channel(),
            _borrow: PhantomData,
        }
    }
}

impl<'d, I: Instance, P: ChannelPin<I>> AdcChannel<'d, I> for Peri<'d, P> {}
impl<I: Instance, P: ChannelPin<I>> SealedAdcChannel<I> for Peri<'_, P> {
    fn setup(&mut self) {
        let pin: Peri<'_, AnyPin> = self.reborrow().into();
        pin.configure_analog();
    }
    fn channel(&self) -> u8 {
        P::CHANNEL
    }
}

/// A non-cloneable channel token retaining an exclusive pin borrow.
///
/// It cannot be constructed from an integer. Dropping it ends the borrow, but
/// leaves the pin in analog mode, like the upstream Embassy borrowed-channel API.
pub struct BorrowedAdcChannel<'a, I: Instance> {
    channel: u8,
    _borrow: PhantomData<&'a mut I>,
}
impl<I: Instance> BorrowedAdcChannel<'_, I> {
    /// Hardware channel number, for diagnostics only.
    pub fn get_hw_channel(&self) -> u8 {
        self.channel
    }
}
impl<I: Instance> SealedAdcChannel<I> for BorrowedAdcChannel<'_, I> {
    fn setup(&mut self) {}
    fn channel(&self) -> u8 {
        self.channel
    }
}
impl<'a, I: Instance> AdcChannel<'a, I> for BorrowedAdcChannel<'a, I> {
    fn degrade_adc(self) -> Self {
        self
    }
}

pub(crate) trait SealedBorrowedChannel<'a, I: Instance> {
    fn into_channel(self) -> BorrowedAdcChannel<'a, I>;
}
/// An exclusive channel reference or an already type-erased channel borrow.
#[allow(private_bounds)]
pub trait BorrowedChannel<'a, I: Instance>: SealedBorrowedChannel<'a, I> {}
impl<'a, I: Instance, C: SealedBorrowedChannel<'a, I>> BorrowedChannel<'a, I> for C {}
impl<'a, 'd, I: Instance, C: AdcChannel<'d, I>> SealedBorrowedChannel<'a, I> for &'a mut C {
    fn into_channel(self) -> BorrowedAdcChannel<'a, I> {
        self.reborrow_adc()
    }
}
impl<'a, I: Instance> SealedBorrowedChannel<'a, I> for BorrowedAdcChannel<'a, I> {
    fn into_channel(self) -> BorrowedAdcChannel<'a, I> {
        self
    }
}
