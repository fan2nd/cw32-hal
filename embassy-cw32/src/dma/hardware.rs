//! The audited register subset is identical on dma/dmachannel l012 and f030.
use super::engine::Io;
use crate::pac;
use core::sync::atomic::{compiler_fence, fence, Ordering};

pub(super) struct Hardware(pub usize);
impl Io for Hardware {
    fn csr(&mut self) -> u32 {
        pac::DMA.ch(self.0).csr().read().0
    }
    fn status(&mut self) -> u32 {
        pac::DMA.isr().read().0 >> (self.0 * 4)
    }
    fn control(&mut self, value: u32) {
        pac::DMA
            .ch(self.0)
            .csr()
            .write_value(pac::dmachannel::regs::Csr(value));
    }
    fn count(&mut self, value: u32) {
        pac::DMA
            .ch(self.0)
            .cnt()
            .write_value(pac::dmachannel::regs::Cnt(value));
    }
    fn source(&mut self, value: u32) {
        pac::DMA
            .ch(self.0)
            .srcaddr()
            .write_value(pac::dmachannel::regs::Srcaddr(value));
    }
    fn destination(&mut self, value: u32) {
        pac::DMA
            .ch(self.0)
            .dstaddr()
            .write_value(pac::dmachannel::regs::Dstaddr(value));
    }
    fn trigger(&mut self, value: u32) {
        pac::DMA
            .ch(self.0)
            .trig()
            .write_value(pac::dmachannel::regs::Trig(value));
    }
    fn clear(&mut self) {
        // ICR reset/reserved value is all ones on both variants. R1W0: zero
        // clears selected flags, one preserves all peers and reserved bits.
        pac::DMA
            .icr()
            .write_value(pac::dma::regs::Icr(!(3 << (self.0 * 4))));
    }
    fn fence(&mut self) {
        // Rust emits a CPU DMB on ARMv6-M. No Atomic RMW or CAS is required.
        // Fences order memory/MMIO; they are never used as a bus-drain proof.
        compiler_fence(Ordering::SeqCst);
        fence(Ordering::SeqCst);
    }
}
