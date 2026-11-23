# UART RTS/CTS hardware flow control

This bounded extension exposes the hardware flow-control signals already
qualified in each family's official manual, SDK and serial AF metadata. It
changes two existing authored files: the UART module and the HAL build script.
No authored register, generated PAC, chip metadata, RCC, IRQ or clock data changes
are needed. RF functionality remains deferred.

This document records the original ordinary flow-control extension and its
verification scope. Its constructors and examples remain unbuffered. The
separate Stage83 [buffered receive extension](uart-buffered-rx.md) reuses these
qualified RTS/CTS roles; its accepted main results and final-source clean
requirement are recorded separately from the historical build results below.

## API and ownership

Following embassy-stm32 at commit
[`f16efeffe37581092ec184718e6fdb1620393214`](https://github.com/embassy-rs/embassy/blob/f16efeffe37581092ec184718e6fdb1620393214/embassy-stm32/src/usart/mod.rs),
the driver adds sealed `CtsPin<T>` / `RtsPin<T>` roles and these constructor pairs:

- `Uart::new_with_rtscts` / `Uart::new_blocking_with_rtscts`
- `UartTx::new_with_cts` / `UartTx::new_blocking_with_cts`
- `UartRx::new_with_rts` / `UartRx::new_blocking_with_rts`

The argument ordering follows the existing CW32 UART API, with RTS/CTS pins
before the async interrupt binding and configuration. These remain byte-IRQ
async drivers; no DMA arguments or DMA capability are implied.

`Config::cts_pull` defaults to `Pull::None`, like the pinned STM32 API. An
application can choose `Pull::Up` to pause an undriven CTS input. Unsupported
CTS pull-down is rejected as `ConfigError::UnsupportedCtsPull` before any RCC or
pin change, using the existing generated GPIO pull-capability masks. An unused
CTS pull setting does not affect constructors without CTS.

Pin conversion into an owned, erased `Peri<AnyPin>` performs no MMIO. Baud/pull
preflight completes before reset or `Flex` construction. CTS is configured as
an AF input; RTS as an AF push-pull output with no pull. Only the existing
metadata-selected CR2 CTSEN/RTSEN fields are written. The existing configuration
clears CR3.DEM on L010/L011/L012, whose RS485 function otherwise overrides RTS.

TX owns its optional CTS `Flex`, and RX owns its optional RTS `Flex`. Splitting
keeps each signal with the correct direction. Dropping one half masks only its
existing event sources, disables that direction and its own flow-control field,
and disconnects only its own pads. The other half retains its enable, pin,
clock and interrupt. Final shutdown still uses the central RCC_INFO and the
existing instance-active guard. L083 shared-vector enable/disable/unpend and
partner dispatch behavior are unchanged.

## Hardware semantics and supported route boundary

All seven reviewed UART register revisions use CR2 bit 2 for CTSEN and bit 3
for RTSEN. Every one of the 13 currently supported families has qualified
hardware and at least one reviewed route. The existing build script now emits
CTS/RTS pin roles from authored, package-filtered metadata and applies its
existing safe-pad filter. Register shape alone never grants a pin route.

Family aliases expose only their existing common-package intersection. An
instance can therefore have RTS without a usable CTS or vice versa. For
example, F030/F020 family-alias UART1 has RTS but no CTS, L010 family-alias UART1
has neither, and L083 UART5 has neither in any reviewed package. Exact packages
may expose more routes than an alias. Constructors requiring an absent pin role
cannot be formed with safe singleton pins. No debug/reset pad is newly exposed.

CTS is active low. A high CTS pauses subsequent frames after any current frame
finishes; queued data remains queued. Hardware TXE/TC events resume the existing
wait paths when transmission proceeds, so the CTS-change interrupt stays masked.
No new interrupt source is introduced.

RTS is active low when RC=0 and rises when a frame sets RC. Existing receive
code reads RDR and then acknowledges only the observed RC/error flags through
`Icr::write_noop()` and the existing typed R1W0 writes. That acknowledgement
permits another frame. Ordinary no-ring read cancellation keeps unread hardware
data and RC intact, so RTS remains controlled by the hardware receive state.
The ordinary path retains its receive readiness, one-shot event masking and
cancel/drop semantics; ISR wakes now occur outside the lifecycle critical
section. A buffered owner continues ISR collection and acknowledgment between
reads and through cancellation. Its RTS still follows hardware RC, not software
ring occupancy, even while the ring is full or a pending error causes discards.

## Source and verification scope

[`uart-flow-control-sources.json`](uart-flow-control-sources.json) records all
13 families' official manual/SDK URLs, revisions, exact PDF pages, artifact and
SDK-member SHA-256 values, existing AF inputs and pinned Embassy source hash.
The manuals sometimes abbreviate the prose register as `UARTx_CR.CTSEN`; their
CR2 register tables and each own SDK establish CR2 bit 2 unambiguously. A030
retains its existing official shared-x030-manual/F030-SDK qualification alongside
its own datasheet; no separate A030 SDK is claimed.

Run the read-only source/data validator with:

```sh
python3 tests/verify_uart_flow_control_sources.py --sources /path/to/cw32-sources
```

The production ARM matrix covers all 54 selections with `rt` and `rt,defmt`.
Six genuine firmware applications in `examples/shared/uart-flow-control` link on seven
exact packages, covering each UART register revision and the L083 shared-vector
binding. Their echo variants split ownership; sender/receiver variants use the
new independent constructors. The source-data audits recheck original official
bytes and serial AF/package evidence. No HAL tests, mocks, model harnesses,
firmware execution, flashing or silicon/electrical validation are claimed.

## Remaining limitations

Hardware flow control has one receive register; these ordinary constructors
provide no software queue. It cannot promise lossless reception if a peer
ignores RTS, sends additional in-flight
frames, or violates the hardware's timing. The earlier lack-of-overrun-reporting
limits on non-L010/L011/L012 UARTs still apply. CTS can stall blocking or async
writes/flush indefinitely. Cancelling a write/flush does not retract queued
bytes. Dropping TX can truncate transmission; flush first. Dropped RTS becomes
disconnected; a peer needing a defined pause level must arrange it electrically.

DMA, buffered/idle reception, half duplex, synchronous mode, nine-bit data,
address matching, LIN, RS485 automatic direction, configurable signal inversion,
CTS-status/event APIs, baud detection and low-speed/deep-sleep UART remain outside
this extension. Existing ClockBounds/source qualification and baud-rounding
limitations remain unchanged.
