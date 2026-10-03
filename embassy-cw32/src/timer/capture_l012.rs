//! CW32L012 RM1.4 chapters 16/17: direct TI capture and quadrature modes 1–3.
macro_rules! capture_l012 {
    ($ier:ident, $tisel:ident) => {
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
            self.$tisel().modify(|v| match channel {
                0 => v.set_ti1sel(0),
                1 => v.set_ti2sel(0),
                2 => v.set_ti3sel(0),
                3 => v.set_ti4sel(0),
                _ => unreachable!(),
            });
            self.ccmr_cap(channel / 2).modify(|v| {
                v.set_ccs(channel % 2, vals::CcmrCapCcs::DIRECT_TI);
                v.set_icpsc(channel % 2, vals::CcmrCapIcpsc::DIV1);
                v.set_icf(channel % 2, filter);
            });
        }
        fn capture_edge(self, channel: usize, edge: Option<Edge>) {
            // Disable before modifying input polarity, including both-edge mode.
            self.ccer().modify(|v| v.set_cce(channel, false));
            if let Some(edge) = edge {
                self.ccer().modify(|v| {
                    v.set_ccp(channel, edge != Edge::Rising);
                    v.set_ccnp(channel, edge == Edge::Both);
                    v.set_cce(channel, true);
                });
            }
        }
        fn capture_interrupt(self, channel: usize, enabled: bool) {
            self.$ier().modify(|v| v.set_ccie(channel, enabled));
        }
        fn capture_interrupt_enabled(self, channel: usize) -> bool {
            self.$ier().read().ccie(channel)
        }
        fn capture_pending(self, channel: usize) -> bool {
            self.isr().read().ccif(channel)
        }
        fn capture_clear(self, channel: usize) {
            // PAC write() seeds R1W0 fields with ones: unrelated flags survive.
            self.icr().write(|v| {
                v.set_ccif(channel, false);
                v.set_ccof(channel, false);
            });
        }
        fn capture_read(self, channel: usize) -> Capture {
            // Caller gated the channel. CCR read clears CCIF, but not CCOF.
            let overcapture = self.isr().read().ccof(channel);
            let count = self.ccr(channel).read().ccr();
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
            // Disable optional index/direction-clock features even after a shared
            // RCC resource could not reset this individual peripheral.
            self.ecr().write_value(regs::Ecr(0));
            self.ccer().modify(|v| {
                v.set_ccp(0, invert_first);
                v.set_ccnp(0, false);
                v.set_ccp(1, invert_second);
                v.set_ccnp(1, false);
            });
            self.smcr().modify(|v| {
                v.set_smsh(false);
                v.set_sms(mode as u8);
            });
            // Encoder uses TI1FP1/TI2FP2 before capture gates. CCxE remains off;
            // encoder counting does not require generating capture events.
        }
        fn encoder_downcounting(self) -> bool {
            self.cr1().read().dir()
        }
    };
}
