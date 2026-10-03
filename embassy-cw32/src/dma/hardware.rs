//! The audited register subset is identical on dma/dmachannel l012 and f030.
use super::engine::Io;
use crate::pac;
use core::sync::atomic::{compiler_fence, fence, Ordering};

pub(super) struct Hardware(pub usize);
impl Io for Hardware {
    fn csr(&mut self) -> u32 {
        pac::DMA.ch(self.0).csr().read().bits()
    }
    fn status(&mut self) -> u32 {
        let flags = pac::DMA.isr().read();
        u32::from(flags.tc(self.0)) | (u32::from(flags.te(self.0)) << 1)
    }
    fn control(&mut self, value: u32) {
        use pac::dmachannel::{
            regs::Csr,
            vals::{CsrSize, CsrTrans},
        };
        // The engine supplies only the audited writable control subset. Build
        // from the reset word so read-only terminal flags are never replayed.
        let mut control = Csr::default();
        control.set_size(CsrSize::from_bits((value >> 6) as u8));
        control.set_dstinc(value & (1 << 5) != 0);
        control.set_srcinc(value & (1 << 4) != 0);
        control.set_trans(CsrTrans::from_bits((value >> 3) as u8));
        control.set_teie(value & (1 << 2) != 0);
        control.set_tcie(value & (1 << 1) != 0);
        control.set_en(value & 1 != 0);
        #[cfg(dmachannel_l012)]
        control.set_restart(value & (1 << 11) != 0);
        pac::DMA.ch(self.0).csr().write_value(control);
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
        use pac::dmachannel::{
            regs::Trig,
            vals::{TrigHardsrc, TrigType},
        };
        let mut trigger = Trig::default();
        trigger.set_hardsrc(TrigHardsrc::from_bits((value >> 2) as u8));
        trigger.set_softsrc(value & (1 << 1) != 0);
        trigger.set_type(TrigType::from_bits(value as u8));
        pac::DMA.ch(self.0).trig().write_value(trigger);
    }
    fn clear(&mut self) {
        // ICR reset/reserved value is all ones on both variants. R1W0: zero
        // clears selected flags, one preserves all peers and reserved bits.
        let mut clear = pac::dma::regs::Icr::default();
        clear.set_tc(self.0, false);
        clear.set_te(self.0, false);
        pac::DMA.icr().write_value(clear);
    }
    fn fence(&mut self) {
        // Rust emits a CPU DMB on ARMv6-M. No Atomic RMW or CAS is required.
        // Fences order memory/MMIO; they are never used as a bus-drain proof.
        compiler_fence(Ordering::SeqCst);
        fence(Ordering::SeqCst);
    }
}
