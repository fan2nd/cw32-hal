# I2C controller HAL

The v0.19 driver provides seven-bit controller transfers on both I2C instances of the maintained CW32L012C8 and CW32F030C8 datasets. It owns the peripheral, its clock lease, SCL and SDA. Both `embedded_hal::i2c::I2c` and, for `Async`, `embedded_hal_async::i2c::I2c` use the same transaction traversal. The register backends are distinct: L012 is a command/FIFO controller, F030 is a byte/status controller. The selection uses generated `i2c_l012` / `i2c_f030` IP configuration, not chip-name branches.

This is an experimental, source-audited implementation. Host models and target compilation cannot establish electrical behavior; board and logic-analyzer validation remain necessary. Target/slave mode, ten-bit addressing, DMA and Fast-mode Plus are not exposed.

## Ownership and use

```rust
use embassy_cw32::{bind_interrupts, i2c, peripherals};

bind_interrupts!(struct Irqs {
    I2C1 => i2c::InterruptHandler<peripherals::I2C1>;
});

let p = embassy_cw32::init(Default::default());
let mut bus = i2c::I2c::new(
    p.I2C1, p.PB6, p.PB7, Irqs, i2c::Config::default(),
)?;
let mut reply = [0; 2];
bus.write_read(0x48, &[0x00], &mut reply).await?;
```

The binding macro requires `rt`. `new_blocking(peripheral, scl, sda, config)` constructs `I2c<'d, T, Blocking>` without an IRQ binding. Blocking methods also exist on an async driver. The fallible constructors validate timing before touching registers. `new` returns `I2c<'d, T, Async>` and requires the selected instance's physical `Binding<Interrupt, InterruptHandler<T>>`.

Only generated `SclPin<T>` and `SdaPin<T>` routes satisfy the constructors. This rejects swapped signals, unrelated pins and another instance's routes at compile time. Tokens can be moved in or reborrowed with Embassy's `Peri` lifetime rules; the driver retains exclusive pin ownership. Pin existence and routing are die-level metadata: confirm package bonding, board wiring, oscillator use and debug use separately. PB6/PB7 are the example I2C1 route; the source data and generated traits are the complete route authority.

Both pins are configured as open-drain alternate-function outputs with their output latches preloaded high. F030 uses the available high slew-rate setting; L012 has no GPIO speed register. `internal_pullups` optionally enables the documented weak pull-ups on both pins. There is no pull-down choice. Use suitable external pull-ups and verify actual rise/fall times; internal weak pulls alone do not prove I2C electrical compliance. Dropping the driver disconnects both pins to floating GPIO input and releases its peripheral clock lease.

## Transactions

- An initial START and address precede the first operation.
- Consecutive operations of the same direction are concatenated without extra START or STOP, including reads split across buffers.
- A direction change emits repeated START and the same seven-bit address with the new direction.
- Reads ACK every nonfinal byte in a contiguous read group and NACK its final byte, including one-byte reads and read-to-write transitions.
- Successful return is fenced by final STOP completion, not merely an empty transmit FIFO.
- An empty operation slice does not access the wire. Empty writes are allowed, including an address-only probe. Empty reads anywhere are rejected before starting; addresses above `0x7f` and aggregate length overflow are also rejected before starting.
- No automatic retry occurs after an error. Retrying a partially accepted write is a device/application decision.

On L012, each receive command encodes 1–256 bytes as `DATA = count - 1`. The hardware sends a NACK at the end of a receive command unless another receive command is already queued. The driver calculates the length of the entire contiguous read group, uses 256-byte nonfinal chunks, and queues the following receive command **before consuming any RX data in the current chunk**. The documented one-byte RX FIFO then provides backpressure long before the current 256-byte command can finish. This permits arbitrary finite read lengths without depending on interrupt latency and without inserting a NACK at buffer or 256-byte boundaries. The last receive command is not followed by another receive command, so hardware generates the final NACK. The `STAR.TXNACK` slave register is not used to control master ACKs.

F030 explicitly follows the status protocol: `08/10` START/repeated START, `18/40` address ACK, `28` data ACK, and `50/58` received byte with ACK/NACK. It prepares DR and AA before clearing SI, since clearing SI advances the state machine. SI is handled with complete, intentional CR writes, not generic read-modify-write.

## Errors, waiting and recovery

`Error::NoAcknowledge` maps to embedded-hal's `NoAcknowledgeSource`. F030 reports `Address` for status `20/48` and `Data` for `30`. L012 reports `Unknown`: its one sticky NACK bit does not identify which pipelined address/data byte was rejected, and TXE reports FIFO occupancy, not completed acknowledgement. Address and data failures are both caught, including a rejected final write byte. The driver does not invent a source or suppress final-byte NACKs.

Arbitration loss maps to `ArbitrationLoss`. F030 status `00`, unexpected controller states and L012 command/FIFO errors map to `Bus`; an inconsistent empty RX data read maps to `Overrun`. Arbitration loss does not enqueue a software STOP that could disrupt the winning controller. L012 has no separate F030-style bus-error state; errors retain the distinctions its hardware actually provides.

`poll_limit` bounds consecutive unsuccessful register observations in blocking transfers. It resets after meaningful software progress. `recovery_poll_limit` bounds each STOP/abort polling phase. Neither field is a wall-clock duration; CPU and bus speeds change their elapsed time. A blocked target therefore produces a finite blocking failure.

Async byte and L012 STOP waits arm actual hardware interrupts and sleep on the registered waker. Handlers mask the event and wake the owning task, preserving sticky status/data until that task services it. F030 has no peripheral interrupt-enable register, so its dedicated NVIC vector is masked while SI is consumed. F030 also has no STOP-complete IRQ: its final self-clearing STO handshake uses bounded polling. The async implementation does not repeatedly self-wake to poll hardware.

There is no implicit elapsed-time deadline for an async wait and no mandatory Embassy time-driver dependency. Wrap the future in the application's timer/select for a deadline. Dropping a polled future attempts bounded STOP cleanup, resets the local controller and restores its timing. L012 flushes queued commands before requesting STOP because clearing MEN alone would continue draining them. F030 requests STOP only after observing a master state; F8 is not evidence of bus ownership. Both avoid issuing STOP after observed arbitration loss. If reset cannot be obtained exclusively, the driver becomes poisoned and rejects further transfers until a successful explicit `recover()`.

Interrupt state never contains a pointer to a transaction's buffers. Forgetting a future can leave a transfer unfinished but cannot authorize later ISR access to those buffers; a retained driver `in_flight` marker forces recovery before its next transfer. Ordinary completion clears that marker only after STOP; cancellation clears it through recovery. A forgotten driver retains its pin tokens and clock lease under the usual ownership rules.

`recover()` and drop provide finite **local controller** cleanup. They do not prove that external SDA/SCL is high, send GPIO bus-clearing clocks, repair a target stuck in a protocol state, or guarantee rollback of accepted bytes. A target holding the wires low may need board-specific recovery or power control before another transaction can complete.

## Clock and timing

`Config::frequency` is a maximum requested rate in `1..=400_000` Hz. `frequency()` reports the configured nominal upper bound, and `kernel_frequency()` reports the selected input clock. Rounding and conservative timing margins can make the actual rate lower. Unsupported clock/rate/edge-time combinations return `InvalidConfig`; rates are never silently rounded upward.

F030's input is fixed PCLK, with `fSCL = fPCLK / (8 * (BRR + 1))`. BRR must be 1–255. The manual specifies the total period but does not explicitly establish its high/low split. The divider adds margin using an **assumed symmetric nominal split**, the requested standard/fast mode's high/low minima and configured maximum edge times. This model does not establish silicon tLOW/tHIGH, data setup or data hold compliance; verify those on the board. FLT follows the manual's rule: simple filtering when BRR <= 9, advanced filtering otherwise.

L012 explicitly selects `MCR0.CLKSRC = 00` (PCLK) while the controller is disabled; slave logic and its interrupts/DMA remain disabled. Its globally configurable I2C kernel clock intentionally has no unconditional `rcc::KernelClock` implementation. This driver derives its real source from its own mux selection and `PeripheralClock::bus_frequency()`. It writes MCCR only with MEN clear. The timing search checks register widths, prescaler limits, the at-least-eight-times kernel/bus ratio, minimum SCL/start/stop/data setup/hold requirements and the manual's DATAVD/filter constraints. Maximum rise time is used for the upper latency/window bounds; minimum data setup and the reported rate use zero extra rise latency, so a faster physical edge cannot invalidate the minimum setup check. BUSIDLE timeout is disabled to avoid declaring another controller's long high phase idle. PINLOW timeout is also disabled; applications choose their async elapsed deadline.

`rise_time_ns` and `fall_time_ns` describe the board's maximum edge times; defaults are 300 ns and 100 ns. Standard mode allows rise <=1000 ns, fast mode rise <=300 ns, and both allow fall <=300 ns. No software timing calculation verifies capacitance, pull-up resistance, oscillator tolerance or actual pad waveforms.

## Source evidence and deliberate differences

- [CW32L012 User Manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf): §23.2.1 one-entry TX/RX FIFOs; §23.4.1 clock/reset and receive-command NACK behavior; §23.4.3 timing equations and limits; §23.4.4 stretching/disable behavior; §23.4.5 errors; §23.8 registers.
- [CW32L012 SDK v1.0.5](https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip): `Libraries/inc/cw32l012_lpi2c.h` defines FIFO size 1; `Libraries/src/cw32l012_lpi2c.c` confirms command encodings, count-minus-one and FIFO flush during abort. Archive SHA-256: `8b0a4c0ee865642d08f353aec98906c900a8b10f672120e649e6b1bf5e29c18f`.
- [CW32x030 User Manual Rev2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf): §20.4.2–3 divider and filter, §20.4.10 state codes, §20.5 examples, §20.7 CR.SI/STA/STO and register semantics.
- [CW32F030 SDK v2.2](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip): `Libraries/src/cw32f030_i2c.c` confirms divider/filter selection and status-driven transfers. Archive SHA-256: `7c431df43d7075b817a51d818ea4c9aba55f83b7c9c77bf0065976758954b780`.
- [Embassy STM32 I2C at the fixed audit revision](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/i2c/mod.rs) and [v2 backend](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/i2c/v2.rs): reference for owned pins, blocking/async modes, binding proofs, IRQ masking/waking and contiguous operation grouping. STM32 register sequences and DMA requirements are not copied to CW32.

The L012 manual's §23.4.4.2 master-read narrative mentions the slave STAR.TXNACK register and an unadjusted receive count. This conflicts with §23.4.1.4 and the explicit MTDR register table/SDK. The driver follows the command-engine description and DATA+1 register definition. The SDK also limits some receive APIs to 256 bytes and suppresses certain last-byte NACK flags in its IRQ path. Those restrictions/heuristics are not copied: safe RX-backpressure chaining supplies long reads, and the manual's unexpected-ACK/NACK status definition governs error reporting. Hardware verification should explicitly check final NACK and rejected-last-write behavior.
