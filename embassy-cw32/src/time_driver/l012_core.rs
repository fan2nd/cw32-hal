//! Hardware-independent GTIM timekeeping. All operations run with interrupts
//! masked: UIFCPY and CNT are one atomic hardware snapshot, not two reads.
pub(crate) const PERIOD: u64 = 1 << 16;
pub(crate) const UIF_COPY: u32 = 1 << 31;
pub(crate) const UPDATE: u32 = 1;
pub(crate) const COMPARE: u32 = 2;
// RM 16.10.6 reset value: preserve reserved bits at zero, all other R1W0
// flags at one. Never RMW ISR/ICR and never acknowledge uncaptured flags.
pub(crate) const ICR_MASK: u32 = 0x00f0_1e5f;

pub(crate) trait Hardware {
    fn counter(&mut self) -> u32;
    fn clear(&mut self, flags: u32);
    fn compare_irq(&mut self, enabled: bool);
    fn compare(&mut self, value: u16);
    fn pend(&mut self);
}
#[derive(Default)]
pub(crate) struct Counter {
    epoch: u64,
}
impl Counter {
    pub(crate) const fn new() -> Self {
        Self { epoch: 0 }
    }
    /// Read-only: the pending overflow remains accounted for until IRQ service.
    pub(crate) fn now(&self, hw: &mut impl Hardware) -> u64 {
        let snapshot = hw.counter();
        self.epoch
            .saturating_add(if snapshot & UIF_COPY != 0 { PERIOD } else { 0 })
            .saturating_add(u64::from(snapshot as u16))
    }
    /// Only IRQ service advances the epoch. A pending UIF must be acknowledged
    /// strictly before the NEXT hardware overflow; no one-bit flag counts two.
    pub(crate) fn service_overflow(&mut self, hw: &mut impl Hardware) {
        if hw.counter() & UIF_COPY != 0 {
            self.epoch = self.epoch.saturating_add(PERIOD);
            hw.clear(UPDATE);
        }
    }
    pub(crate) fn arm(&self, hw: &mut impl Hardware, deadline: Option<u64>) {
        hw.compare_irq(false);
        hw.clear(COMPARE);
        let Some(deadline) = deadline else { return };
        let now = self.now(hw);
        if deadline <= now {
            hw.pend();
            return;
        }
        // For distant alarms, overflow interrupts revisit the queue. Avoid a
        // low-16-bit alias waking in an earlier counter cycle.
        if deadline - now >= PERIOD {
            return;
        }
        let target = deadline;
        hw.compare(target as u16);
        hw.compare_irq(true);
        // Programming can race the match, including crossing an overflow.
        // Pend the real NVIC vector rather than wait an entire extra wrap.
        if self.now(hw) >= target {
            hw.pend();
        }
    }
}
