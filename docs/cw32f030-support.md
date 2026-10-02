# CW32F030C8 support (v0.9.1)

## Chip identity and build

The public chip selector is `cw32f030c8`. `cw32l012c8` remains supported.
Select exactly one; no chip, both chips, and package-suffixed selectors are
rejected. Chip identity carries the memory variant, not the package or
operating-temperature suffix. Both C8 variants have 64 KiB Flash at address
0 and 8 KiB SRAM at 0x20000000. Applications remain responsible for board
bonding and wiring. The maintained data has no `packages` layer.

```sh
cargo run -p xtask -- regenerate
cargo check -p embassy-cw32 --features cw32f030c8
```

Generation is still one Rust `cw32-gen` crate, two real file stages:
YAML -> schema-3 normalized JSON -> read/validate JSON -> PAC/metadata.
Normal PAC builds consume those pre-generated artifacts and do not parse
YAML or JSON. Generated trees and Cargo.lock are ignored and not shipped.

## Actual hardware support

| Layer | CW32F030C8 support and limits |
| --- | --- |
| Register data/PAC | All 37 official peripheral views, normalized to 20 IP models and 32 IRQs; not all have safe HAL drivers |
| GPIO | 39 conservative die GPIO identities, digital input/output, typed audited routes; board must verify bonding; PF3/BOOT is withheld due to conflicting vendor descriptions |
| Clocks | Checked direct-reset HSI 48 MHz /6 = nominal 8 MHz, vendor factory HSI/LSI trim; no dynamic clock switching or inherited 4 MHz assumption |
| Embassy time | Reserved GTIM1, true IRQ16, 1 MHz hardware count, direct deadline compare; independent F030 backend, no SysTick and no ATIM reservation |
| ADC | One real ADC, 1..4 sequence slots, one shared sample-time setting, conservative default 500 kHz ADC clock at 8 MHz PCLK, software or ATIM-update trigger, blocking and real EOS IRQ async |
| VC | Two comparators, 0..7 external inputs on either mux, READY handshake, true VC1/VC2 IRQ waits, typed pin ownership; guarded comparator-to-ATIM brake route |
| ATIM | True A/B complementary output pairs, F030 prescaler/deadtime/brake model, update/break IRQ futures; conservative strict three-phase duty update API as described below |
| Unsupported HAL drivers | UART/SPI/I2C/DMA/RTC/watchdogs/CRC and other peripherals remain PAC/metadata only; route inventory is deliberately FOC-focused |
| Absent hardware | No fabricated ADC2, OPA, DAC, CORDIC, EAU, or standalone VCREF/BGR singleton |

IRQ waits use actual `Binding` proofs, instance-specific hardware flags and
critical-section-protected event latches. Futures cancel their own subscription
and can be rearmed. These are one-shot event APIs, not lossless edge/sample
streams. Startup readiness polls and blocking constructors remain explicitly
bounded blocking operations.

### Shared analog resources

F030 VC1_DIV and VC2_DIV name the same physical divider; ADC.CR0.BGREN also
controls a bandgap source used by internal comparator references. This version
therefore does **not** expose an independent divider or bandgap token, and the
safe comparator API is external-input-only. ADC initialization and Drop preserve
BGREN, TSEN, bias and reserved fields, and its constructor does not pulse the ADC
reset. VC shared reset is not pulsed when creating either comparator.

The comparator brake guard borrows the comparator and PWM owner; establishing
it requires disabled outputs. Dropping it disables outputs before disconnecting
the route. It is not a safety-certified protection system: confirm voltage range,
polarity, gate-driver wiring, comparator response and actual fault latency on
hardware. Async wait cancellation is not a general emergency-stop function.

### ATIM update boundary

F030 hardware **does** support compare preload/shadow registers and UEV transfer
(RM Rev2.5 sections 15.3.1.2 and 15.3.3.2). Its SDK exposes separate compare writes.
That alone does not prove that three independent bus writes cannot straddle a
hardware UEV. The hardware has no audited equivalent of L012's UDIS in this
implementation; an unproven global inhibit sequence is not silently substituted.

Accordingly this version's strict `set_duty([u16; 3])` returns `Busy` while outputs
are enabled. With outputs disabled it stops the counter, temporarily gates ADC
triggering, writes the three preloads, issues one UG to load them, clears that
software update and restores the prior trigger/counter settings. This can change
PWM phase and ADC trigger spacing. It is **not** a disturbance-free running FOC
control loop. Ordinary single-channel buffered writes remain a hardware/PAC
capability, but are not advertised as an atomic three-phase commit.

F030 deadtime encoding includes two extra timer ticks; the minimum nonzero
enabled encoding is two ticks and the maximum is 1010 ticks. Zero requested
deadtime disables deadtime rather than pretending encoding zero means no delay.

### Timebase boundary

F030 GTIM has neither UIFREMAP/UIFCPY nor a linear PSC register. The backend
uses CR0.PRS=3 to divide 8 MHz PCLK by 8, latched when EN rises. It obtains a
consistent timestamp by reading OV, CNT, OV and retrying if OV changed, while
the interrupt service advances the software epoch. Compare channel 1 uses the
internal compare-event mode and no GPIO pin. ICR W0 flags are cleared without
RMW; reserved reset bits are preserved.

From **each first unserviced overflow until its corresponding acknowledgement**
the elapsed time must be strictly less than 65.536 ms. This includes interrupt
masking, higher-priority handlers, flash stalls, synchronous waker work and debug
pauses. One OV bit cannot recover two missed wraps. Average service rate is not
a substitute for that bound. No STOP-continuous time or clock changes are
supported. Nominal 1 microsecond ticks do not promise measured oscillator
accuracy, 1-microsecond task wakeup or hardware real-time certification.

There is no fixed artificial alarm lead/delay. The requested deadline is written
directly; a missed programming deadline is detected by a fresh timestamp and
pend of the real GTIM1 IRQ. The queue checks `now >= deadline`, including stale
compare aliases across long horizons.

## Shared IP and access widths

IWDT and WWDT have independently verified equivalent register and side-effect
contracts and reuse existing `l012`. The other 18 F030 models use `f030` selected
through explicit chip/instance/vendor `perimap` rules; no L012 `l012` register model
was rewritten to fit F030.

CRC DR8/DR16/DR32 select different hardware feed widths. GPIO also provides byte
views inside its ODR word. Register `bit_size` and generated typed volatile
accesses preserve 8/16/32-bit bus operations, as do field masks/values. Explicit
aliases must fit wholly inside their canonical register byte extent. A narrow
register is never represented as a fake 32-bit access merely because the address
matches. The generated API preserves this distinction.

## Evidence and verification

Official source versions are SDK V2.2, CW32x030 RM Rev2.5 and CW32F030 datasheet
Rev1.9. Fixed hashes, header/SVD discrepancies and register side effects are
preserved in `vendor/cw32f030/`; signal provenance is in
[cw32f030-pins.md](cw32f030-pins.md). The SDK audit compares both directions, so
missing/extra registers or fields cannot pass by merely checking an existing
subset. Current verification uses ARM build/link checks.

See [validation.md](validation.md) for the final executed test record. No board
was connected: compile, simulated race tests and ELF vectors do not establish
actual silicon timing, motor safety or electrical correctness.
