# Window watchdog

`wdg::WindowWatchdog::new(p.WWDT, WindowConfig::default())` validates the
configuration and acquires the actual peripheral without resetting or starting
it. `unleash()` starts it explicitly. `try_pet()` reads the live counter in a
critical section and writes a reload only inside the documented open window.
The shared audited `wwdt_l012` IP serves both chips. `WindowWatchdog<'d, T>`
uses a sealed `WindowInstance` whose register and clock association is generated
from metadata; the constructor infers T from the provided `Peri`.

The seven-bit counter uses PCLK / (4096 × 2^PRS). The configuration requires
0x40 ≤ window < reload ≤ 0x7f. The counter must reach `window` before a refresh,
and reaching 0x3f causes reset. `WindowConfig` returns the manual's nominal
closed-window and timeout formulas in PCLK cycles; `clock_frequency()` supplies
the actual configured nominal PCLK. Oscillator error, prescaler phase and
DeepSleep affect elapsed timing. WWDT stops counting in DeepSleep.

`TooEarly` and `TooLate` leave CR0 untouched. Hardware continues counting between
the read and write, so a deadline call can still reset the system. The critical
section excludes normal interrupts; it cannot exclude NMI, debug stalls or the
counter itself. Feed with margin inside the window.

EN cannot be cleared after start, and the pre-overflow interrupt-enable bit IE
also cannot be cleared once set. This driver implements reset-only operation:
`unleash()` rejects inherited EN or IE before changing any watchdog register.
It exposes no stop or early-warning IRQ operation. Dropping a started or
inherited running watchdog permanently retains the PCLK gate. Forgetting it
retains its clock lease. Neither operation removes reset protection.

The separate independent watchdog retains its existing API. It uses its own RC
source and has a documented CW32 stop command; those facts do not apply to WWDT.

The fixed [Embassy watchdog implementation](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/wdg/mod.rs)
provides an owned `WindowWatchdog` and direct refresh. CW32 keeps an explicit
start, actual register-level configuration and a checked refresh attempt because
its manual requires window < reload and because initialization must preserve an
inherited safety policy.

Evidence: [L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
§§20.3–20.6, printed pp438–444;
[F030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf)
§§17.3–17.6, printed pp320–327. No physical timing or reset test has been run.
