# Classic ordered ADC scans

`continuous` collects 32 MODE2 samples from the first generated input, then
attempts a single internal supply conversion. It uses the same exact-package
features below. Discard the output on error; interrupt latency can cause
overrun. See the [continuous contract](../../../docs/adc-classic-continuous.md).
`async_continuous` uses EOC/OVW interrupts, a caller deadline and the same
generated input. Select the `async-gtim` / `async-gtim1` feature described below.
Only success permits consuming the buffer; timeout and ADC errors discard it.

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

The blocking Cortex-M0+ firmware binaries demonstrate borrowed and owned pin
handles. Select one exact package feature. The build script obtains two bonded
analog pins and their actual mux encodings from reviewed metadata; see
`analog-input-routes.txt` in that build's output directory. R031 signal labels
and mux numbers are deliberately not equated.

Use a board with VDDA=VDD=3.3 V within its own tolerances and qualified HSI
conditions. Apply low-impedance, voltage-compatible signals to the two named
pins. Every full-length scan alternates these inputs, demonstrating preserved
slot order and repeated muxes. The internal supply/3 measurement uses the
existing single-read API after each scan. Debugger variables retain raw counts
and completion/failure counters. There is no signal loopback, fixture, synthetic
register image, HAL harness or firmware execution in the build verification.

Run with the official ARM target, for example:

    cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7 --bins

All scan slots use the same acquisition setting. Internal channels and follower
enabled external channels are intentionally limited to single reads. No DMA,
triggered or continuous scan is implemented. Finite async scans are described
below. Builds establish API and link compatibility; physical timing, analog
accuracy and source drive require
board validation.

## Finite async reads and scans

- `async_single`: EOC-driven external single reads with a caller deadline, then
  a raw internal supply/3 single read with its existing follower/rate limits.
- `async_scan_borrowed`: EOS-driven full-length external scans using repeated,
  alternating borrowed handles; `select` cancellation followed by immediate
  reuse with `with_timeout`. Successful completion commits all four/eight slots
  in order; a canceled/error call leaves the entire slice unchanged. Internal
  supply/3 stays a separate single read.

Both bind the dedicated ADC interrupt through `bind_interrupts!`, also observe
OVW as an error wake, and use the existing pinned executor/time/futures revision.
Select `async-gtim` on F002/F003 and `async-gtim1` on every other classic line.
These features imply `async` and reserve the named timer and IRQ for the existing
run-mode time driver. Do not combine the two timer selectors. The bare `async`
feature supplies the dependencies but requires an explicit compatible HAL time
driver feature before linking.

```
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/classic-adc-scan/Cargo.toml --no-default-features \
  --features cw32f030c8t7,async-gtim1

cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/classic-adc-scan/Cargo.toml --no-default-features \
  --features cw32f002f3p7,async-gtim
```

Representative selections for the ten lines are `cw32f030c8t7`, `cw32a030c8t7`,
`cw32f020c6u7`, `cw32f002f3p7`, `cw32f003e4p7`, `cw32l031c8t6`, `cw32r031c8u6`,
`cw32l052c8t6` and `cw32l083rct6`. All other exact-part features
in this package use the same examples and package-derived input routes.

The async binaries declare a 3.0–3.6 V board supply with VDDA=VDD and qualified
HSI conditions. Retain the default nominal 8 MHz PCLK compatible with the existing
time driver, or requalify the chosen clocks. Every classic scan uses one common
sample time and unbuffered external sources. `Config.timeout` covers synchronous
polls only; caller deadlines require working timer/IRQ service. Classic sources
document logical stop, not an explicit cursor-reset/drain guarantee; reuse
reinitializes the finite mode. Failed required cleanup latches `Faulted` until
chip reset, visible here via `unwrap` and the panic handler. See the
[async source contract](../../../docs/adc-remaining-async.md) and
[time-driver contract](../../../docs/time-driver.md). No firmware execution or
physical cancellation timing is claimed.
