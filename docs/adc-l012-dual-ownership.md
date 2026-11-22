> Historical software-verification references: HAL tests and fixture harnesses cited below were deleted on 2026-10-08. Their reported results apply to the dated source snapshot only; source facts remain unchanged. See [current HAL layout and build scope](hal-production-layout.md).

# CW32L012 dual-ADC blocking implementation

Source-level implementation, 2026-10-08. No firmware was executed on a device;
no silicon/electrical qualification, calibration accuracy or power-loss claim
is made. This backend covers ADC1 and ADC2 on both exact L012 parts and their
package-intersection alias, through the actual `adc_cw32l012_v1` PAC.

## Ownership and shared resources

ADC1 and ADC2 each own their peripheral token. Both constructors acquire the
shared clock internally using the timeout in their ADC configuration.

```rust
let mut p = embassy_cw32::init(Default::default())?;
let mut adc1 = embassy_cw32::adc::Adc::new(
    p.ADC1, embassy_cw32::adc::Config::default())?;
let mut adc2 = embassy_cw32::adc::Adc::new(
    p.ADC2, embassy_cw32::adc::Config::default())?;
let a = adc1.blocking_read(&mut p.PA0, embassy_cw32::adc::SampleTime::Cycles518);
let b = adc2.blocking_read(&mut p.PA8, embassy_cw32::adc::SampleTime::Cycles518);
```

Driver initialization checks the initialized clock contract and a nonzero finite
poll budget. It rejects an asserted active-low shared reset without any write.
If APBEN1.ADC is already set, it performs no write; otherwise it enables that
one gate with the SYSCTRL `0x5a5a` key and a bounded readback wait, preserving
other gates. Common-clock acquisition can resume a previously gated converter;
each constructor therefore checks the group gate before accessing the ADC registers. It does not reset or reconfigure either converter.

No path clears APBEN1.ADC or changes APBRST1.ADC, including common acquisition
failure, individual constructor failure, conversion error, reconfiguration,
individual drop. A bootloader-configured sibling is not
considered disposable merely because there are no HAL instances yet.

Each ADC constructor validates timing and reads the sibling's SLAVE bit before
any ADC or BGR write. An inherited synchronized sibling is rejected. The same
check precedes reconfiguration and each conversion. It is deliberately
conservative even if the sibling is presently disabled: starting one converter
must not trigger the other. An independent inherited running sibling is left
untouched. No safe synchronization/slave recovery API is claimed; coordinated
whole-group reinitialization requires a separately reviewed API.

RM section 25.12.19 says BGREN, ADCEN, TSEN, VC1/2/3/4 and OPA1/2 enables start
the common BGR, which can only be disabled by software or POR. ADC startup may
therefore power BGR implicitly. Explicit BGR/TS operations only OR enable bits
under a critical section. They preserve every existing bit. BGR and temperature
remain enabled after use; no constructor/error/drop clears them. This preserves
inherited comparator/OPA/other-ADC dependencies and avoids a false ownership
claim over bootloader state.

This monotonic strategy deliberately retains power, even when all HAL ADCs are
gone. It is not a lowest-power power-management API. A future whole-analog owner
may reclaim resources only after accounting for all consumers and inherited
state. Raw users must not alter shared gate/reset/BGR/TS or owned ADC state
concurrently. Taking a HAL token does not authorize racing raw register access. Raw users of
an unowned sibling must also keep its SLAVE bit unchanged during HAL operations;
the preflight check is a snapshot, not synchronization with arbitrary raw code.

## Documented conversion subset

- One slot (`ENS=0`), single shot (`CONT=0`), software start, `SLAVE=0`.
- Right-aligned 12-bit RESULT0, masked to 0..4095.
- No scan, DMA, async, external-trigger, dual synchronization, analog watchdog,
  internal DAC-output channel, calibration command or physical-unit conversion.
- The sole reference is VDDA. The supported board contract requires VDDA=VDD,
  1.7..5.5 V. Inputs must remain within 0..VDDA and the pin electrical limits.
- PCLK/1, /2, /4 or /8; exact rational comparisons precede displayed Hz rounding.
- Supply declaration is the guaranteed minimum, including board tolerance.
  Clocks and waits are nominal; source oscillator tolerance is not modelled.

Own RM table25-3 and DS table7-28 together impose:

| Guaranteed minimum VDDA | Maximum ADCCLK | Maximum sample rate | Minimum external acquisition |
|---|---:|---:|---:|
| 1.7 to below1.8 V | 6 MHz | 200 kSPS | 1 us |
| 1.8 to below2.8 V | 12 MHz | 500 kSPS | 0.5 us |
| 2.8 to below3.3 V | 24 MHz | 1 MSPS | 0.25 us |
| 3.3 through5.5 V | 48 MHz | 1 MSPS | 0.125 us |

The datasheet 4 MHz minimum ADC clock is also enforced. For every candidate
divider, clock maximum, minimum, conversion-rate and acquisition duration are
checked independently using u64 arithmetic. This can make a requested sample
time impossible even when the ADC clock by itself is legal. Source impedance
can require longer acquisition than the tabulated floor; see the datasheet's
input-impedance formula.

Sampling codes0..15 are6,7,9,12,18,24,30,42,54,70,102,134,166,198,262,518 cycles,
followed by15 comparison cycles. The conservative initial acquisition is518
cycles. After EN the driver waits max(50 nominal us,15 ADCCLK cycles), covering
approximately1 us converter startup and approximately30 us automatic BGR
startup without inventing a READY flag.

Internal TS14/BGR15 are raw measurement channels, not selectable references.
Each internal read requires at least40 us acquisition, enables only the needed
shared-source bit, and waits50 nominal us even for an inherited enable that may
have occurred just before this read. Temperature startup is at most40 us in
the own datasheet. No factory trim is read or extrapolated into an accuracy
claim.

A conversion clears stale events, sets slot0 mux/sample, starts once, and polls
for EOC and EOS together with START cleared. Polls are bounded by `timeout`.
A failed conversion stops and disables only its own converter; a later valid
read retries initialization. Invalid configurations/reads and synchronized
sibling rejection leave ADC/common state unchanged. GPIO setup may occur before
read timing rejection, as documented by the channel API.

## Register protocol and source discrepancies

The actual L012 addresses are ADC1=0x40000000, ADC2=0x40000100 and shared
BGR_CR=0x400000FC. The PAC models the latter as BGR base0x400000FC, CR offset0;
it is not ADC1.CR and must not be treated as an ADCx offset.

Offsets: CR0x00, START0x04, AWDCR0x0c, TRIGGER0x10, SAMPLE0x18, SQRCFR0x20,
RESULT0x30, IER0x70, ICR0x74, ISR0x78. Only these real PAC accessors are used.
No instance layout cast or L010 offset reuse exists.

CR bits31:8 are reserved with reset0x100. A live read-modify-write changes only
bits7:0 and preserves all reserved bits, including reset bit8. Shutdown is
not a blanket CR=0 store.

ICR is R1W0. Own CN1.4 sections25.12.8–10 specify EOC0, EOS1, AWDL3 and AWDH4,
with bit2 reserved; ICR reset is0x0f. The driver preserves reserved bit2 at its
documented reset1, uses clearable mask0x1b, writes0x04 to clear all defined flags
and0x1c to clear EOC/EOS while preserving watchdog flags. It never reads or
writes a nonexistent READY/overrun bit and never writes ISR or RESULT.

The following discrepancies remain explicit, rather than silently merging
incompatible sources:

1. SDK `cw32l012_adc.h` ADC_IT_AWDL/AWDH macros are0x04/0x08, copied from the older
   contiguous map. Own manual and PAC register field definitions instead give
   bits3/4. HAL follows the own manual's detailed register table, preserving
   reserved bit2. The SDK interrupt-mask macros are not used.
2. DS table7-28 advertises96 MHz maximum while the own RM caps the highest supply
   band at48 MHz. HAL uses48 MHz; no96 MHz ADC operation is exposed.
3. RM table25-3's highest band caps1 MSPS even though48 MHz divided by the shortest
   21-cycle conversion could exceed2 MSPS. HAL enforces the listed1 MSPS too.
4. DS total-conversion maximum405 cycles conflicts with RM518 acquisition plus15
   comparison cycles. Own RM table25-2 and SDK `ADC_SampTime518Clk=15` agree;
   HAL exposes the documented518-cycle code, not L010/L011's390-cycle name.

## Primary source identities

- [L012 CN1.4 RM](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  SHA256 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
  Printed575–578 (clock/acquisition/rate/mux),590 (internal acquisition),
  589–599 (registers and common BGR).
- [L012 CN1.0 DS](https://www.whxy.com/uploads/files/20250717/CW32L012_DataSheet_CN_V1.0.pdf),
  SHA256 `08a605fe8843d1eefa2b1115474b5a74df2833068f0ad6f35225e4f39377da76`.
  Table7-4 (operating conditions), table7-28 printed60 (ADC), printed63 (TS).
- SDK `Libraries/inc/cw32l012_adc.h`, SHA256
  `13a189fef19ca32b64c6fa289c2e144069e8961ed4cf72bdbe62cd190266b045`.
- SDK `Libraries/src/cw32l012_adc.c`, SHA256
  `8b26760851f1ed6e83ce1d8e53b30fb436b51a924e0c69b2610a5e9db84bdd50`.
- SDK `Libraries/inc/cw32l012_sysctrl.h`, SHA256
  `3746c16b7a3fa72d1b43d42c080111e64faa36e8beb4b1f19c13daf3c71b993c`.

Own-source external routing and package evidence are separately documented in
`l012-adc-route-qualification.md`. ADC1/ADC2 each expose12 external routes.
Seven physical pads are shared with instance-specific mux meanings. Pins own
exclusive `Peri` borrows even when eligible for either converter. Internal
DAC mux12/13 never become GPIO traits.

## Validation and remaining acceptance barriers

The source test commands are:

- `cargo test -p embassy-cw32 --no-default-features --features cw32l012c8t6 --lib`
- The same host suite with `cw32l012c8u6` and the generic `cw32l012` alias
- `python3 tests/verify_l012_adc_routes.py --sources "$CW32_SOURCES"`
- `python3 tests/audit_l012_adc_sources.py --sources "$CW32_SOURCES"`
- `python3 tests/test_l012_adc_hal_contracts.py` (select any L012 profile with `CW32_ADC_TEST_CHIP`)
- `python3 tests/test_l012_adc_links.py`

Tests model two simultaneously owned converters, both construction/drop orders,
failed second construction, inherited active and synchronized siblings, bounded
polling/retry, raw internal channels, preserved BGR/TS, invalid-input nonmutation,
clock acquisition failures, exact timing boundaries and sparse R1W0 semantics.
RAM tests use the production PAC adapters and real `Adc::drop` with synthetic
register backing; they never touch actual device MMIO. ARM contracts include
shared-guard/token/borrow failures and converter-specific pin/internal-channel
erasure. ELF checks verify both exact parts' vectors and memory bounds without
running the image.

Real board testing, minimum/maximum voltage and clock tolerances, source
impedance, shared analog consumers, interrupt latency and energy use remain
acceptance barriers. This implementation deliberately retains analog power
rather than claiming an unverified low-power teardown.
