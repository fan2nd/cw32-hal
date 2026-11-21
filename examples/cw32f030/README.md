# CW32F030C8T7 firmware examples

Four standalone, fully linked Cortex-M0+ binaries demonstrate the public HAL:

- `blocking`: GPIO output, blocking UART output, and blocking SPI loopback.
- `dma_owned_copy`: owned static SRAM copies, concurrent shared-vector channels and resource reuse.
- `lvd_ir_bursts`: timer-driven infrared bursts with a polling voltage monitor.
- `async_gpio_edge`: a real Embassy thread-mode executor sleeping between GPIO
  interrupts and toggling an output for each falling input edge.

These examples target **CW32F030C8T7 (LQFP48) only**. There is no assumed board or
on-board LED, no flashing command, and no hardware-validation claim.

## Build

With Rust and `thumbv6m-none-eabi` installed, run from **this directory**:

```sh
cargo build --locked --release --bins
```

Running Cargo here loads `.cargo/config.toml`, including the target and
`-Tlink.x`. Running `cargo check`, or using only `--manifest-path` from the
repository root, is not a substitute for this firmware link step. The complete
repository HAL check also builds these binaries:

```sh
# From the repository root:
./ci/check-hal.sh
```

The directory is its own Cargo workspace with a committed lockfile. Its
`embassy-executor` uses exactly the HAL's pinned Embassy revision
`f16efeffe37581092ec184718e6fdb1620393214` and the verified feature names
`platform-cortex-m` and `executor-thread`. Cortex-M's
`critical-section-single-core` supplies the actual critical-section
implementation for this single-core bare-metal target. `cortex-m-rt`, the PAC
runtime vectors, the linker script, and a local halt-on-panic handler are linked;
no debugger logging library is required.

The linker memory map is 64 KiB FLASH at `0x00000000` and 8 KiB RAM at
`0x20000000`. It comes from the verified C8T7 entry in
[`parts.json`](../../cw32-data/parts.json) and
[generated chip metadata](../../cw32-data/data/chips/CW32F030C8T7.json),
cross-checked against the official
[CW32F030 datasheet CN V1.9](https://www.whxy.com/uploads/files/20251229/CW32F030_DataSheet_CN_V1.9.pdf)
and DFP memory description. Do not apply this map to an F6 part, another package,
or firmware placed after a bootloader.

## Common wiring and setup

Use a properly powered CW32F030C8T7 board. Follow its schematic and the datasheet
for supply voltage, decoupling, VDDA/VSSA, reset, and boot strapping. Connect all
external equipment grounds to board ground and use GPIO-compatible signal
levels. Do not connect RS-232 voltage levels or assume a USB adapter's supply
voltage is the right board supply. Pin numbers below are **chip LQFP48 lead
numbers**, not board header numbers.

Both examples use PB0 (lead 18) as an active-high output. A possible indicator is
PB0 to a current-limiting resistor to an LED anode, then LED cathode to ground;
choose its current and resistance from your supply and the datasheet. A logic
analyzer can replace the LED. The examples leave SWDIO/SWCLK (PA13/PA14) alone.
Default HAL initialization selects nominal 8 MHz HSI/HCLK/PCLK; no external
crystal is used. RCC's default declared envelope covers the factory-HSI-qualified
VDD and ambient TA intervals; the slow default clocks remain valid throughout.
If changing clocks, set `config.rcc.operating_conditions` to the board's guaranteed
minimum/maximum supply and ambient range. This is a declaration, not a sensor.
Frequencies and final Flash latency are checked against the actual HSI envelope;
a nominal boundary frequency alone is insufficient. See
[`rcc-operating-envelope.md`](../../docs/rcc-operating-envelope.md).

## `blocking`

- UART1 TX: PA8 (lead 29), to a logic-level USB-UART adapter RX.
- UART1 RX: PA9 (lead 30), to adapter TX if bidirectional wiring is desired.
  This example sends only and does not wait for received UART data.
- Terminal: 115200 baud, 8 data bits, no parity, 1 stop bit, no flow control.
- SPI1 SCK: PA5 (lead 15), optional logic analyzer input.
- SPI1 MOSI: PA7 (lead 17), connect directly to MISO PA6 (lead 16) for loopback.
- Software chip select: PA4 (lead 14), optional analyzer input. It is not needed
  for the loopback jumper; no SPI slave should drive the loopback signal.
- SPI is master mode 0, MSB first, 8-bit words, requested 1 MHz SCK.

The example transfers `A5 5A 00 FF`, checks the returned bytes, writes a status
line to UART, and toggles PB0. Chip select is raised again even if the transfer
returns an error. A Cortex-M busy delay spaces the transfers; it is deliberately
not a calibrated delay or a time driver. Missing loopback wiring causes a
reported mismatch, not an infinite UART receive wait. HAL peripheral operations
can still block if hardware does not make progress.

## `async_gpio_edge`

Connect PA0 (lead 10) to ground through a normally open switch, or drive it with
a suitable external digital signal. Its internal pull-up is enabled. Each
falling edge toggles PB0. The `GPIOA` vector is bound to the typed HAL EXTI
handler; that interrupt wakes the genuine Embassy executor. While idle the
executor uses Cortex-M `WFE`/`SEV`, rather than a busy-poll executor.

There is no `embassy-time` dependency and no invented timer implementation.
Mechanical switches may bounce and produce several toggles. Edges while the
wait is not armed can be missed; this is an interrupt-wait demonstration, not a
debouncer or pulse counter. A clean pulse generator is preferable when checking
single-edge behavior.

## `lvd_ir_bursts`

PB9 carries IR_OUT and PB0 indicates a nominal below-3.00-V VDDA sample. Use
appropriate external transistor, LED and current-limiting circuitry, not a bare
IR LED on a GPIO. Two independently owned GTIM PWM drivers supply nominal 38-kHz
carrier and 1-kHz burst gating internally, with no physical PWM output pins.
This emits continuous bursts rather than a receiver protocol. The LVD monitor
refuses an existing reset/interrupt/filter configuration and never changes BOR.
Analog startup, hysteresis and voltage tolerances need board qualification.

An explicit build from the repository root is:

```sh
RUSTFLAGS='-C link-arg=-Tlink.x' cargo build --locked \
  --manifest-path examples/cw32f030/Cargo.toml \
  --target thumbv6m-none-eabi --bin lvd_ir_bursts
```

See `docs/lvd-ir.md` for precise per-family source and ownership boundaries.

## Boundaries

Linking checks software integration and memory placement, not electrical timing,
clock accuracy, interrupt delivery on silicon, power consumption, or run-time
stack headroom. Nothing here has been flashed or measured on a physical board.

The DMA example below uses explicitly unsafe transfer construction,
and safe early cancellation is not available: the manuals do not establish that
clearing enable drains every in-flight bus access. Dropping a running transfer
waits for completion/error and can block forever if hardware requests stop.
There is no safe cancellable UART/SPI DMA wrapper implied by these examples.

## `dma_owned_copy`

This firmware uses owned static SRAM buffers for software-triggered DMA copies.
Its unsafe construction blocks justify exclusive hardware control: no raw PAC
access, controller reset or DMA clock changes, and disjoint buffers on each
channel. That obligation persists for forgotten or error-quarantined transfers.
It first copies halfwords on channel 1 with an explicit blocking wait, then
starts byte and word copies on channels 2 and 3 before awaiting either. Both
handlers are bound to their shared `DMACH23` vector. It checks the copied words,
recovers and reuses the owners after clean completion, and toggles PB0 after each
successful pair. No external DMA peripheral or request wiring is required.

The buffers and channels move into each future. Forgetting a copy leaks them;
dropping it waits for completion/error and discards them. A DMA error permanently
consumes its resources because the manual does not prove bus quiescence on TE.
Drop can hang on stalled hardware. This example is compiled and linked, not
flashed or claimed to validate silicon. See `docs/dma-owned-copy-evidence.md`.

Motor control examples are in [f030-motor](../f030-motor/README.md).
