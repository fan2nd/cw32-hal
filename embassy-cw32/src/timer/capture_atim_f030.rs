//! CW32x030 RM2.5 chapter 15. Physical channels are A1/B1/A2/B2/A3/B3.
use super::{Capture, CaptureRegisters, Edge, Filter, QeiMode};
use crate::pac::atim::{regs, Atim};

fn channel_control(r: Atim, channel: usize) -> regs::Chcr {
    let mut v = r.chcr(channel / 2).read();
    // CHCR mixes configuration with software capture commands. Never replay
    // either self-clearing command when changing a gate or interrupt enable.
    v.set_ccga(false);
    v.set_ccgb(false);
    v
}
impl CaptureRegisters for Atim {
    fn capture_channels(self) -> usize {
        6
    }
    fn capture_filter(self, filter: Filter) -> Option<u8> {
        match filter {
            Filter::None => Some(0),
            Filter::PclkSamples3 => Some(4),
            _ => None,
        }
    }
    fn configure_capture(self, channel: usize, filter: u8) {
        self.capture_edge(channel, None);
        let mut v = channel_control(self, channel);
        if channel % 2 == 0 {
            v.set_csa(true);
            v.set_bufea(false);
        } else {
            v.set_csb(true);
            v.set_bufeb(false);
        }
        self.chcr(channel / 2).write_value(v);
        self.fltr().modify(|v| {
            if channel % 2 == 0 {
                v.set_ocmflta(channel / 2, filter);
                v.set_ccpa(channel / 2, false);
            } else {
                v.set_ocmfltb(channel / 2, filter);
                v.set_ccpb(channel / 2, false);
            }
        });
        self.mscr().modify(|v| {
            v.set_ia1s(false);
            v.set_ib1s(false);
        });
    }
    fn capture_edge(self, channel: usize, edge: Option<Edge>) {
        let edge = match edge {
            None => 0,
            Some(Edge::Rising) => 1,
            Some(Edge::Falling) => 2,
            Some(Edge::Both) => 3,
        };
        let mut v = channel_control(self, channel);
        if channel % 2 == 0 {
            v.set_bksa(edge);
        } else {
            v.set_bksb(edge);
        }
        self.chcr(channel / 2).write_value(v);
    }
    fn capture_interrupt(self, channel: usize, enabled: bool) {
        let mut v = channel_control(self, channel);
        if channel % 2 == 0 {
            v.set_ciea(enabled);
        } else {
            v.set_cieb(enabled);
        }
        self.chcr(channel / 2).write_value(v);
    }
    fn capture_interrupt_enabled(self, channel: usize) -> bool {
        let v = self.chcr(channel / 2).read();
        if channel % 2 == 0 {
            v.ciea()
        } else {
            v.cieb()
        }
    }
    fn capture_pending(self, channel: usize) -> bool {
        let v = self.isr().read();
        if channel % 2 == 0 {
            v.caf(channel / 2)
        } else {
            v.cbf(channel / 2)
        }
    }
    fn capture_clear(self, channel: usize) {
        self.icr().write(|v| {
            if channel % 2 == 0 {
                v.set_caf(channel / 2, false);
                v.set_cae(channel / 2, false);
            } else {
                v.set_cbf(channel / 2, false);
                v.set_cbe(channel / 2, false);
            }
        });
    }
    fn capture_read(self, channel: usize) -> Capture {
        let status = self.isr().read();
        let (count, overcapture) = if channel % 2 == 0 {
            (self.ccra(channel / 2).read().ccr(), status.cae(channel / 2))
        } else {
            (self.ccrb(channel / 2).read().ccr(), status.cbe(channel / 2))
        };
        self.capture_clear(channel);
        Capture {
            count,
            overcapture: Some(overcapture),
        }
    }
    fn configure_encoder(
        self,
        mode: QeiMode,
        first_filter: u8,
        second_filter: u8,
        invert_first: bool,
        invert_second: bool,
    ) {
        self.configure_capture(0, first_filter);
        self.configure_capture(1, second_filter);
        self.fltr().modify(|v| {
            v.set_ccpa(0, invert_first);
            v.set_ccpb(0, invert_second);
        });
        self.mscr().modify(|v| v.set_sms(mode as u8 + 3));
        // Encoder interface taps filtered A1/B1 upstream of capture gates.
        // BKS stays 00: RM15.3.2.1 capture could otherwise run even with EN=0.
    }
    fn encoder_downcounting(self) -> bool {
        self.cr().read().dir()
    }
}
