//! Software-controlled bandgap and startup witness for the l012 BGR IP.

use crate::{pac, peripherals, Peri};
use embedded_hal::delay::DelayNs;

/// Owns the software BGR control. Dropping it NEVER disables BGR: ADC/VC/OPA may
/// have automatically enabled it too. It provides a startup-delay witness.
///
/// Copyable marker values cannot grant hardware access.
pub struct Bandgap<'d> {
    _token: Peri<'d, peripherals::BGR>,
}
impl<'d> Bandgap<'d> {
    pub fn new(token: Peri<'d, peripherals::BGR>, delay: &mut impl DelayNs) -> Self {
        critical_section::with(|_| {
            // Preserve TSEN; BGR has no peripheral gate/reset. RM 25.12.19.
            pac::BGR.cr().modify(|w| w.set_bgren(true));
        });
        delay.delay_us(32); // RM says approximately 30us, including hardware enable.
        Self { _token: token }
    }
}
