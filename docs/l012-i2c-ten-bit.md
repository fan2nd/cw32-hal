# L012 ten-bit I2C master addressing

## Scope and API

The L012 command/FIFO I2C backend supports explicit seven-bit or ten-bit target
addresses on its existing I2C1/I2C2 blocking and async owners. This is not a
classic SI/STAT backend change, slave mode, DMA, automatic retry or GPIO bus
recovery feature. Timing, the 256-byte contiguous read-run limit, pin ownership,
interrupt bindings and terminal-fault isolation remain unchanged.

Existing inherent methods still take `u8` seven-bit addresses. New
`blocking_transaction_with_address(Address, operations)` and async
`transaction_with_address(Address, operations)` accept `Address::SevenBit(u8)`
or `Address::TenBit(u16)`. Both `embedded_hal::i2c::I2c<TenBitAddress>` for the
blocking owner and `embedded_hal_async::i2c::I2c<TenBitAddress>` for the async
owner are implemented. A ten-bit address with numerical value below 128 is
still ten-bit; address width is never guessed from its value.

Preflight rejects seven-bit addresses above 127 and ten-bit addresses above
1023 before any *new* transaction activity, including an empty operation list.
A previously forgotten transfer is retired first, so invalid/empty arguments
are not a zero-MMIO or no-bus-side-effect guarantee. Empty writes are address
probes; empty reads are rejected; an empty operation list emits no new START or
STOP. Adjacent operations in the same direction still share their address phase.

## Source qualification and command sequences

The verified [CW32L012 CN user manual V1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
(18,034,510 bytes, SHA-256
`a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`)
§23.4.1.4, printed page 531, explicitly specifies ten-bit master addressing as
a START/address command followed by transmission of the second address byte.
No new register primitive is needed. Protocol sequencing follows
[NXP UM10204 §3.1.11](https://www.nxp.com/docs/en/user-guide/UM10204.pdf).

For ten-bit address `a`, `H = 0xf0 | (((a >> 8) & 3) << 1)` and `L = a & 0xff`:

- Write run: START H (write), transmit L, transmit payload.
- Initial read run: START H (write), transmit L, repeated START H|1 (read),
  receive payload.
- Read after a write run in the same transaction: repeated START H|1, receive.
  The earlier full address selection remains in effect.
- A subsequent write run sends the full H/write and L sequence again.
- Adjacent read buffers use one receive command and one final read NACK.
  The whole transaction ends with one STOP and existing completion checks.

The same private address-command helper is used by blocking and async engines.
On any enqueue/transfer error the existing retirement path runs; a low-address
byte sent with a Transmit command is still part of addressing, but the FIFO
cannot identify which command received NACK. `Error::Nack` therefore continues
to report `NoAcknowledgeSource::Unknown`, not a guessed address/data category.

### SDK discrepancy

The pinned SDK V1.0.5 `Libraries/src/cw32l012_lpi2c.c` explicitly provides
`I2C_MasterSTART_Addr10b` and `I2C_MasterReSTART_Addr10b`. Its generic transfer
paths around lines 2016 and 2431 set the first address header's direction from
the requested direction when no subaddress is supplied. In the pure-read case
that produces a read header before the low address byte rather than the
required initial write selection and repeated read START. Its address-width
heuristic also cannot express a ten-bit target whose numeric address is below
128. Those paths were not copied. This is a source-level sequence discrepancy,
not a claim of a reproduced silicon fault.

## Cancellation and validation

No caller pointer is added to static state. The existing durable Active marker
and async guard precede commands; cancellation, forgotten transfers, bounded
local retirement, reset fallback and permanent fault policy are unchanged.
Cancellation can complete queued address/payload bytes and is not rollback.
No wall-clock deadline, target recovery or electrical behavior is inferred.

Temporary host checks extract the actual address helper, preflight, blocking
run, async run and cancellation guard. They exhaust all 1024 ten-bit encodings,
compare command sequences for both engines, inject cancellation and NACK at
every command boundary, and check explicit low-valued ten-bit addresses,
out-of-range addresses, empty reads/writes and the 256-byte read limit. Mock
backend checks do not validate registers, interrupt timing or silicon.

`examples/l012/i2c/src/bin/ten_bit_master.rs` demonstrates both traits and both
typed transaction methods on separate I2C buses. Address 0x2a5 is an example,
not an assertion that a particular device exists there; configure suitable
hardware before running. Existing seven-bit binaries remain unchanged.
Final ARM compilation/linking results belong to the current batch validation
receipt, not earlier seven-bit acceptance records. No hardware test was run.
