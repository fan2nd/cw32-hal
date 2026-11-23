# Independent L012 ADC scans

Exact-part, ordinary Cortex-M0+ firmware examples for CW32L012C8T6 and
CW32L012C8U6. Build with one feature selected and target thumbv6m-none-eabi.
No firmware was executed by the source verification workflow.
Both exact selections have 64 KiB Flash and 8 KiB RAM. The generic `cw32l012`
alias has no package/memory declaration and is not an executable example feature.

- `owned_ordered`: ADC1 executes three caller-ordered external slots, including
  a repeated pin; ADC2 remains independently owned and executes its own two
  slots. Dropping ADC1 preserves ADC2 and the common analog resources.
- `borrowed_internal`: all eight ADC1 slots use borrowed external pins and
  owned internal handles, including repeated sources and different sample
  times. An independent ADC2 remains alive; subsequent single reads demonstrate
  that eight-slot scans do not change the single-read contract.

These examples declare a 3.0–3.6 V supply, VDDA=VDD, qualified HSI temperature
range, and HSI/8. Adjust the board declaration and source impedance/acquisition
assumptions before using on hardware. PA0, PA1 and PA8 must remain within
0..VDDA. Results are raw 12-bit counts; no average or calibration is applied.
The two binaries above use bounded polling. The async binaries below use
independent ADC1/ADC2 interrupt completion; DMA, external triggers and dual
coupling remain absent.

```
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/l012/adc-scan/Cargo.toml --no-default-features \
  --features cw32l012c8t6
```

## ADC1 and ADC2 async examples

- `async_single`: ordinary `join` polls independent ADC1 PA0 and ADC2 PA8
  single reads, each wrapped in `with_timeout`. The next loop reuses each owner
  after completion or successful cancellation. Direct DAC outputs on PB0/PB1
  remain active throughout both conversion waits.
- `async_scan_borrowed`: independent eight-slot scans mix ADC1 PA0/PA1 or ADC2
  PA8 with that converter's temperature/bandgap handles. Both sequences repeat
  handles and use per-slot sample times. `join` polls each scan's ordinary
  `select` deadline; the same owners, channels and output buffers are reused
  immediately through `with_timeout`, then for single reads. A canceled/error
  scan leaves every output element unchanged. Direct DAC outputs stay active.

Both async examples bind dedicated `ADC1` and shared `ADC2_DAC` exactly once;
the latter uses `adc::InterruptHandler<peripherals::ADC2>`. ADC2's existing
constructor checks actual DAC interrupt-enable bits in a valid clock/reset
domain. No DAC token/guard is passed to ADC2. The current direct-loading DAC API
preserves the checked interrupt-free condition, so PB0/PB1 remain usable while
ADC2 awaits. ADC2 never changes DAC control, data, output routing or sticky flags.
See the [shared-vector contract](../../../docs/adc-l012-adc2-async.md) for admission
errors, possible retained DAC progress when its gate is opened, and faults.

The selected generated routes are ADC1 IN0/IN1 on PA0/PA1, ADC2 IN5 on PA8 and
DAC OUT1/OUT2 on PB0/PB1. Both exact packages bond these pads at positions
10/11, 29 and 18/19 respectively; C8T6 is LQFP48 and C8U6 is QFN48. No pad is
shared between the two ADC owners and the DAC. No external loopback is required.
The DAC declaration is nominal 3.3 V within the board's declared 3.0–3.6 V range;
adjust all declarations to the actual board. Its unbuffered outputs retain the
[DAC load/startup contract](../../../docs/dac-opa.md): the 5 kΩ minimum resistive
load, 50 pF maximum capacitive load and typical startup figure have their stated
3.3 V conditions. Qualify loads, startup and every code change for the actual
board, supply and temperature. The pacing delay does not prove settling, and
ADC raw counts do not measure DAC performance.

Enable `async` to use the existing pinned Embassy executor, time and futures
revision. This feature reserves GTIM1 and its interrupt for the existing run-mode
time driver. Keep the declared HSI/8 clock and board conditions qualified, or
recalculate the ADC and time-driver requirements for your board.

```
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/l012/adc-scan/Cargo.toml --no-default-features \
  --features cw32l012c8t6,async
```

The same command supports `cw32l012c8u6`. `Config.timeout` bounds synchronous
poll attempts, not total await time. Caller deadlines need working timer/IRQ
service; neither timeout winning nor maximum IRQ latency is promised. A terminal
cleanup fault persists until chip reset. The example's `unwrap` makes such a
fault visible through its panic handler. See the complete
[finite async/source contract](../../../docs/adc-remaining-async.md) and
[time-driver contract](../../../docs/time-driver.md). Building these ordinary
firmware examples does not execute them or qualify silicon behavior.

Stage91 main produced five standalone ARM libraries and ten ordinary ELF links.
The initial `async_single` link exceeded the qualified 64 KiB Flash by 1,088 bytes.
A reviewed example-only release `opt-level = "s"` to `"z"` change, retaining LTO,
codegen-units=1 and the memory layout, allowed the ten-row ELF continuation to
pass. The five earlier library inputs were unchanged in their separate firmware
workspace. All 15 successful rows span 16 attempts and two source snapshots;
the failed attempt remains recorded. L010 library row L4 retains three existing
RCC dead-code warnings; no new warnings were reported. Both finite source/data
gates passed after the recorded navigation locator-key correction. At main
acceptance the final-source two-library/three-ELF clean replay and final54 report
binding had not run; final completion requires their separate receipts.
Compilation and source review do not establish hardware behavior.
