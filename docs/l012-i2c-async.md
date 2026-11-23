# Owned CW32L012 command/FIFO I2C masters

Main execution: all six standalone ARM libraries, six linked ordinary ELFs and
six finite source/data commands passed. Independent-review outcomes are recorded
separately; these execution results alone are not an acceptance verdict. At main
acceptance clean had not run; final-package completion requires a separate
final-source three-library/three-ELF clean receipt.

The matrix retained all 1435 authored files, all 910 generated files, the toolchain
and Cargo configuration unchanged. L6 (`cw32l011k8t6,rt`) retains the three existing
`RCC_LSE_SYSCLK_*` dead-code warnings. No hardware execution or electrical result
is claimed. Both modes now share the L012 retirement and constructor fence:
MEN-only disable precedes bounded drain, FIFO reset follows confirmed local idle,
and unconfirmed drain falls back to held master reset. This corrects the prior
sequence's source-qualification gap; no reproduced silicon defect is asserted.

CW32L012 I2C1 and I2C2 provide owned seven-bit blocking and asynchronous master
transactions. This is the L012 command/FIFO engine, selected by
`i2c_cw32l012_v1`; classic SI/STAT/STO behavior does not apply. The current
source-level contract is described here. Build receipts and independent review
outcomes are separate and do not establish silicon or electrical validation.

## API, ownership and qualified clocks

`I2c<'d, Async>::new_async` takes the peripheral, typed SCL/SDA
pins, a verified instance interrupt binding, and the existing `Config`. The
owned driver exposes `read`, `write`, `write_read`, `transaction` and the ordinary
seven-bit `embedded_hal_async::i2c::I2c` trait. Existing blocking constructors,
`blocking_*` methods and the embedded-hal trait remain available. No unsafe
constructor, caller-buffer pointer in static state, or public recovery escape
hatch is introduced. I2C1 and I2C2 use their own dedicated IRQ23 and IRQ24.

All new transfers use frozen PCLK source0 and the existing timing solver.
`Config` retains its frequency ceiling, internal pull-up choices, poll limit,
SCL/SDA rise-time bounds and filter-cycle settings. Default 100 ns rise bounds
are board assumptions, not measurements. External pull-ups and qualification of
actual voltage, loading, rise/fall time and noise remain necessary. Filters are
disabled by default; spike suppression is not promised in that setting. The
[existing clock-bound contract](i2c-clock-bounds.md) and waveform inequalities
remain applicable and unchanged.

The family alias is a library selection with no exact memory/package map.
[Ordinary executable examples](../examples/l012/i2c/README.md) admit exactly
CW32L012C8T6 (LQFP48) and CW32L012C8U6 (QFN48), using each selected part's 64 KiB
Flash and 8 KiB RAM. Both binaries use I2C1 PA4/PA5 AF2 and I2C2 PB0/PB1 AF8;
both packages bond those pairs at positions 14/15 and 18/19. This qualifies the
selected example routes, not compilation of all promoted AF combinations.

## Transactions and results

Every nonempty transaction has ordinary address phases, repeated START at
direction changes, and one final STOP. Adjacent writes share one address phase.
An empty write retains its address phase. Adjacent reads share exactly one
receive command and total 1..=256 bytes; buffer boundaries do not add START,
receive commands or NACK. A read of 256 bytes encodes count 255. The hardware
NACKs the last received byte when no additional receive command is queued.
That transmitted final NACK is distinct from an unexpected received NACK flag.

Address, empty-read and complete read-run preflight occurs before starting new
bus activity. An empty operations array starts no transaction. A forgotten
active future is retired at the next mutating driver entry before a new request
is admitted, even when that new request is invalid. Preflight cannot erase the
need to stop previously abandoned work.

TXE means command/TX FIFO capacity, not address ACK or wire completion. Each
received byte is read once together with its EMPTY state; an EMPTY observation
stores no byte. The disputed RXCNT field is never used. Both modes check errors
before acting on readiness. Async command/data paths take another error-first
snapshot after the await, before enqueue or consumption. An unexpected NACK maps
to `NoAcknowledge(Unknown)` because enqueue position cannot
establish address versus data phase. No late NACK is ignored after the last
write or while STOP is pending.

Success requires the durable normal-STOP marker and observed error-free STOP,
TXE and local `!MSTBUSY`. BUSBUSY need not be clear because another master may
start after this master's STOP. PACKET is not transaction completion. Errors
can follow partial target effects; there is no automatic retry or rollback.

## Interrupt waits and finite synchronous work

Only TXE, RXNE and the initial final-STOP event are interrupt waits. Each wait
registers its waker before arming, enables only its phase event plus the four
error events, and rechecks status before returning Pending. Other phases do not
leave a level TXE request enabled. The ISR masks only its own dedicated vector
and wakes an armed task. It never accesses I2C MMIO, clears status, queues a
command, reads data, or accesses caller buffers. A delayed invocation therefore
does not touch a clock-gated instance. Static state owns only a waker and flags.

These suspended waits have no internal deadline. Initial bus-idle sampling,
final STOP/idle confirmation, drain, reset, configuration readback and Drop use
finite synchronous sample budgets. `Config::poll_limit` counts samples, not
elapsed time or whole-transaction latency. The existing central-reset release
helper has its separate fixed 32-sample gate check; it is not an async deadline.
An independently working application timeout can cancel a suspended future but
cannot preempt synchronous polling or a destructor.

## Construction and inherited state

Pure timing and configuration validation precedes peripheral/pin mutation.
Construction fences its dedicated interrupt and rejects a sticky instance fault
before touching I2C or pins. No I2C register access is permitted until the gate
is observed enabled and the central reset is observed released. An already
asserted central reset is handled by the existing helper followed by bounded
gate/reset readback; the helper itself is not a new checked API. An inherited
released controller is not blindly pulsed through central reset.

The independent slave logic must already be disabled and idle, including its
interrupt/DMA requests. A foreign enabled slave owner is rejected as Busy;
master RESET does not retire the slave engine. An inherited reserved clock
selector is also refused before master control or pin changes. Other inherited
ordinary clock/debug/sleep fields are preserved while stopping the controller;
they do not qualify those clocks for new transfers.

A zero complete watermark register is required before inherited TXE/RXNE can
prove FIFO-empty state. Otherwise construction uses held-master-reset isolation.
Only after master RESET=1/MEN=0 is confirmed may the configuration switch to
PCLK0. Master reset release and complete timing/input/master/slave-disable
restoration are checked with MEN=0 before pin setup and enabling MEN. Successful
construction leaves interrupt/DMA requests disabled and its vector masked until
an actual asynchronous wait. The blocking constructor uses the same fence.

## Cancellation, retirement and reuse

A durable driver marker is set before the first command. Dropping an unpolled
future does nothing; dropping a polled one retires the active controller.
Forgetting a future leaves the marker for the next mutating entry. Neither the
ISR nor static state retains a pointer to the forgotten future or buffers.

Retirement first masks requests and the dedicated vector. It then clears MEN
alone while preserving live ordinary clock/debug/sleep fields and without FIFO
reset strobes. MEN-disable is a drain operation: active and queued work may
still reach the target, and received bytes may never reach caller buffers. It
is not an immediate abort. Only after bounded confirmation of MEN=0, TXE and
local master idle are the FIFOs reset in a separate write. Empty FIFOs and
local idle are rechecked before observed flags are cleared. TX reset precedes
clearing a FIFO error.

If drain or empty-state confirmation fails, the driver asserts master RESET
with MEN=0, verifies that held state and retains it. No explicit cleanup STOP
is ever enqueued, including after arbitration loss or a failed normal STOP.
Neither an old ownership marker nor arbitration state authorizes a new STOP.
Reset isolates local logic; it cannot make a target or another master release
SDA/SCL and does not promise an absence of electrical edges.

Confirmed retirement leaves Disabled or ResetHeld, not automatically re-enabled.
Before a later transaction, the driver requires both lines high and checked
master reset/full configuration restoration. No abandoned transaction is
retransmitted. `set_config` also retires abandoned Active state; it publishes
new cached timing/configuration/pull choices only after successful restoration.
An invalid candidate leaves the prior cache and any confirmed stopped state.

Unconfirmed shutdown, reset, restoration or gate readback latches a sticky
per-instance Faulted condition. `RecoveryFailed` takes precedence when safe
reuse was not established. Constructors, configuration changes and subsequent
transfers cannot clear it; system reset is the boundary. Faulted code may mask
the dedicated vector and disconnect owned pins, but does not touch I2C, pulse
reset or gate its clock. Drop gates only after fresh confirmed local idle or
held reset, and verifies the gate-off result. A failed gate disable becomes
terminal and is not blindly retried.

Cancellation returns Rust memory ownership but can leave partial target effects,
truncate progress or leave a low bus line. There is no bus repair, transaction
rollback or multi-master noninterference guarantee. Forgetting an entire driver
can leave hardware work outstanding until a replacement constructor applies
the same source-qualified retirement and static-fault fence.

## Source authority and scope limits

The original L012 manual CN V1.4, SDK V1.0.5, datasheet CN V1.0 and their exact
identities are retained in [typed I2C evidence](i2c-typed-register-evidence.json).
The separate `command_async_master` evidence binds the L012 IRQ inventory,
register fields/access, gate/reset facts and exact package routes. The existing
source/PAC checker inspects original evidence and generated data, not a HAL
behavior model. Source facts and ARM compilation/linking have distinct limits.

RXWATER/RXCNT width remains disputed; the driver uses only qualified zero
watermarks and RXNE/MRDR.EMPTY. Source3's HSI/LSI conflict remains unresolved;
only PCLK0 is admitted for new transfers. Master reset is distinct from central
RCC reset and from the independent slave engine. The SDK's late-NACK suppression
and FIFO-clear-before-reset sequence are not adopted.

DMA, slave and ten-bit APIs, automatic retries, GPIO bus-unwedge pulses, new
clock sources, low-power guarantees and RF work are outside this addition.
Hardware execution, waveform quality, cancellation on silicon and electrical
bus recovery remain unverified. The [historical blocking document](l012-command-i2c.md)
retains prior dated timing/source work; its old combined MEN-clear/FIFO-reset
retirement sequence and deleted model/fixture checks are not current acceptance.
