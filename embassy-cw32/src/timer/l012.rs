//! CW32L012 RM1.4 chapters 16/17: operations audited as identical on the
//! distinct ATIM and GTIM PAC types. Initialization and update-trigger gating
//! stay in the respective adapters because their register sets differ.

macro_rules! l012_common {
    () => {
        fn prescalers(self) -> super::Prescalers {
            super::Prescalers::Linear
        }
        fn set_running(self, on: bool) {
            self.cr1().modify(|v| v.set_cen(on));
        }
        fn is_running(self) -> bool {
            self.cr1().read().cen()
        }
        fn counter(self) -> u16 {
            self.cnt().read().cnt()
        }
        fn set_counter(self, value: u16) {
            self.cnt().write(|v| v.set_cnt(value));
        }
        fn write_timing(self, c: super::low_level::Config) {
            self.psc()
                .write(|v| v.set_psc((c.prescaler_divisor - 1) as u16));
            self.arr().write(|v| v.set_arr((c.period_ticks - 1) as u16));
            self.cnt().write(|v| v.set_cnt(0));
        }
        fn configure_pwm(self) {
            for n in 0..2 {
                self.ccmr_cmp(n).write(|v| {
                    v.set_ocm(0, vals::CcmrCmpOcm::PWM1);
                    v.set_ocm(1, vals::CcmrCmpOcm::PWM1);
                    v.set_ocmh(0, false);
                    v.set_ocmh(1, false);
                    v.set_ocpe(0, true);
                    v.set_ocpe(1, true);
                });
            }
        }
        fn write_compare(self, channel: usize, duty: u32) {
            // 65536 is represented by forced-active mode, never truncated into CCR.
            let ccr = if duty == 65536 { 0 } else { duty as u16 };
            self.ccr(channel).write(|v| v.set_ccr(ccr));
        }
        fn output(self, channel: usize, enabled: bool, duty: u32) {
            // Disable before any mode write. This also makes endpoint changes
            // safe when their requested duty differs from the active mode.
            if !enabled {
                self.ccer().modify(|v| v.set_cce(channel, false));
            } else {
                // CCxNP/CCxP/CCxNE remain zero: only main active-high outputs.
                self.ccmr_cmp(channel / 2).modify(|v| {
                    // These named modes use the base OCMH=0 encoding.
                    v.set_ocmh(channel % 2, false);
                    v.set_ocm(
                        channel % 2,
                        if duty == 65536 {
                            vals::CcmrCmpOcm::FORCE_ACTIVE
                        } else {
                            vals::CcmrCmpOcm::PWM1
                        },
                    );
                });
                self.ccer().modify(|v| v.set_cce(channel, true));
            }
        }
    };
}
