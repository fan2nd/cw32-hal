//! CW32F030 GTIM timekeeping without an atomic overflow/counter snapshot.
//!
//! All callers hold a critical section. ISR.OV is sticky and CNT is only 16 bits
//! (RM Rev 2.5 sections 14.8.6, 14.8.12–13). Read OV/CNT/OV and retry when OV
//! changed, so the sampled counter cannot be paired with the wrong epoch.
//! There must never be two overflows before the first one's acknowledgment:
//! every OV must be acknowledged strictly within one 65.536 ms counter period.
//! This includes time spent in critical sections, interrupt handlers and wakers.
pub(crate) const PERIOD: u64 = 1 << 16;
pub(crate) const UPDATE: u32 = 1;
pub(crate) const COMPARE: u32 = 1 << 3;
/// Implemented R1W0 flags (OV, TI, UD, CC1–4, DIRCHANGE).
pub(crate) const ICR_FLAGS: u32 = 0x027f;
/// RM 14.8.13's reset value. Keep reserved bits 8:7 at their reset value of one;
/// all implemented flags are written one except the specific flags being cleared.
pub(crate) const ICR_MASK: u32 = 0x03ff;

pub(crate) trait Hardware {
    fn flags(&mut self) -> u32;
    fn counter(&mut self) -> u16;
    fn clear(&mut self, flags: u32);
    fn compare_irq(&mut self, enabled: bool);
    fn compare(&mut self, value: u16);
    fn pend(&mut self);
}

/// CR0.PRS is log2(divider), unlike the L012's linear PSC register.
pub(crate) fn prescaler(divider: u32) -> Option<u32> {
    (divider.is_power_of_two() && divider <= 32768).then(|| divider.trailing_zeros())
}

#[derive(Default)]
pub(crate) struct Counter {
    epoch: u64,
}
impl Counter {
    pub(crate) const fn new() -> Self {
        Self { epoch: 0 }
    }

    /// Read-only: include a pending overflow without acknowledging it. The
    /// matching flag samples bracket the CNT read; a change means that the
    /// counter may have been sampled on either side of that overflow, so retry.
    /// A wrap after the final flag read leaves a valid slightly older snapshot.
    pub(crate) fn now(&self, hw: &mut impl Hardware) -> u64 {
        loop {
            let before = hw.flags() & UPDATE;
            let count = hw.counter();
            let after = hw.flags() & UPDATE;
            if before == after {
                return self
                    .epoch
                    .saturating_add(if after != 0 { PERIOD } else { 0 })
                    .saturating_add(u64::from(count));
            }
        }
    }

    /// Only IRQ service changes the epoch. Clear only an observed OV flag; a
    /// first overflow arriving after an OV=0 read is left pending for the next
    /// service. The service bound above prohibits a second wrap before this
    /// acknowledgment, which a one-bit flag fundamentally cannot distinguish.
    pub(crate) fn service_overflow(&mut self, hw: &mut impl Hardware) {
        if hw.flags() & UPDATE != 0 {
            self.epoch = self.epoch.saturating_add(PERIOD);
            hw.clear(UPDATE);
        }
    }

    pub(crate) fn arm(&self, hw: &mut impl Hardware, deadline: Option<u64>) {
        hw.compare_irq(false);
        // An R1W0 write must preserve OV even if it arrives during this write.
        hw.clear(COMPARE);
        let Some(deadline) = deadline else { return };
        let now = self.now(hw);
        if deadline <= now {
            hw.pend();
            return;
        }
        // Overflow IRQs revisit distant deadlines. Do not arm a low-16-bit
        // alias in an earlier cycle. A deadline exactly one period away is
        // likewise reconsidered on the next overflow.
        if deadline - now >= PERIOD {
            return;
        }
        hw.compare(deadline as u16);
        hw.compare_irq(true);
        // The target is the actual deadline, with no artificial minimum delay.
        // A match missed while programming is recovered by pending the real
        // IRQ. The queue still checks full-width deadlines before waking, so a
        // stale CC1 flag or a spurious IRQ never wakes an alarm early.
        if self.now(hw) >= deadline {
            hw.pend();
        }
    }
}
