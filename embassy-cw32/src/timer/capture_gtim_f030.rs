//! CW32x030 RM2.5 chapter 14. Capture has no overflow/lost-edge status.
use super::{Capture, CaptureRegisters, Edge, Filter, QeiMode};
use crate::pac::gtim::{vals, Gtim};
impl CaptureRegisters for Gtim {
    fn capture_channels(self) -> usize {
        4
    }
    fn capture_filter(self, filter: Filter) -> Option<u8> {
        match filter {
            Filter::None => Some(0),
            Filter::PclkSamples2 => Some(1),
            Filter::PclkSamples4 => Some(2),
            Filter::PclkSamples3 => None,
        }
    }
    fn configure_capture(self, channel: usize, filter: u8) {
        self.capture_edge(channel, None);
        self.cr1().modify(|v| {
            v.set_chflt(channel, filter);
            v.set_chpol(channel, false);
        });
    }
    fn capture_edge(self, channel: usize, edge: Option<Edge>) {
        self.cmmr().modify(|v| {
            v.set_ccm(
                channel,
                vals::CmmrCcm::from_bits(match edge {
                    None => 0,
                    Some(Edge::Rising) => 1,
                    Some(Edge::Falling) => 2,
                    Some(Edge::Both) => 3,
                }),
            )
        });
    }
    fn capture_interrupt(self, channel: usize, enabled: bool) {
        self.ier().modify(|v| v.set_cc(channel, enabled));
    }
    fn capture_interrupt_enabled(self, channel: usize) -> bool {
        self.ier().read().cc(channel)
    }
    fn capture_pending(self, channel: usize) -> bool {
        self.isr().read().cc(channel)
    }
    fn capture_clear(self, channel: usize) {
        self.icr().write(|v| v.set_cc(channel, false));
    }
    fn capture_read(self, channel: usize) -> Capture {
        let count = self.ccr(channel).read().ccr();
        self.capture_clear(channel);
        Capture {
            count,
            overcapture: None,
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
        self.cr1().modify(|v| {
            v.set_chpol(0, invert_first);
            v.set_chpol(1, invert_second);
        });
        self.cr0().modify(|v| {
            v.set_encmode(mode as u8);
            v.set_encreset(0);
            v.set_encreload(0);
        });
        // RM14.8.5 requires ARR=0xffff in encoder mode; Qei fixes that range.
    }
    fn encoder_downcounting(self) -> bool {
        self.isr().read().dir()
    }
}
