# HSE and PLL startup clocks

The clock source is selected once, during checked direct-reset startup. The
default remains HSI /24 = 4 MHz on CW32L012C8 and HSI /6 = 8 MHz on CW32F030C8.
The existing L012 motor profile remains HSI /1, AHB /1, APB /1 = 96 MHz.
All previously documented HSI and bus divider choices remain available.

## Audited hardware limits

| Capability | CW32L012C8 | CW32F030C8 |
| --- | --- | --- |
| HSE crystal/resonator | 4–32 MHz | 4–32 MHz |
| HSE external clock input | 4–32 MHz, 40–60% duty | 4–32 MHz, 40–60% duty |
| HSE drive | 0–7; separate startup drive 0–7 | 0–3 |
| HSE pins | PF0 OSC_IN, PF1 OSC_OUT | PF0 OSC_IN, PF1 OSC_OUT |
| PLL | No PLL in this device's clock tree | Divided HSI or HSE, multiplier 2–12 |
| Supported PLL input/output | Not applicable | Input 4–24 MHz; output 12–64 MHz |
| HCLK/PCLK at VDD ≥1.8 V | Up to 96 MHz | Up to 64 MHz |
| HCLK/PCLK below 1.8 V | Up to 24 MHz at VDD ≥1.7 V | Up to 24 MHz at VDD ≥1.65 V |

The maximum allowed supply in either configuration is 5.5 V. Both datasheets
require VDDA = VDD. The caller must satisfy all other datasheet conditions,
including temperature, input levels, clock waveform and peripheral-specific
limits. `supply_voltage_mv` is the caller's assertion of the minimum board VDD,
not a measurement or voltage-regulator setting. Its default is 1800 mV, retaining
the supply requirement for previously available full-speed profiles.

F030's reference manual defines a PLL output band extending to 72 MHz, while
the F030 datasheet limits PLL output to 64 MHz. The HAL rejects frequencies above
64 MHz even if an AHB divider would reduce HCLK. Conversely, the datasheet lists
PLL output from 8 MHz, but the lowest documented PLL configuration band starts
at 12 MHz. This implementation supports their intersection, 12–64 MHz. F030 has
no separately configurable PLL pre-divider or post-divider: the HSI divider
applies before an HSI-fed PLL, and AHB/APB dividers apply after SYSCLK selection.
L012 exposes no `PllConfig` or PLL clock-source variant.

## Selecting a source

Both chips accept a crystal or an externally driven OSC_IN signal:

```rust,ignore
let mut config = embassy_cw32::Config::default();
let mut hse = embassy_cw32::rcc::HseConfig::crystal(16_000_000);
hse.drive = 2; // Select from the board's crystal/load requirements.
config.rcc.source = embassy_cw32::rcc::ClockSource::Hse(hse);
config.rcc.supply_voltage_mv = 3300;
let p = embassy_cw32::try_init(config)?;
// p.PF0 and p.PF1 are None: both are permanently reserved by HSE.
```

Use `HseConfig::bypass(frequency_hz)` for an external clock. PF0 is reserved as a
digital input; PF1 remains `Some` because both manuals explicitly permit
OSC_OUT to serve as GPIO in this mode. HSI-only initialization leaves both pad
fields `Some`. These two fields are `Option<Peri<...>>` in the generated
singleton set, so ordinary GPIO use takes the `Some` token rather than obtaining
an alias of an oscillator pad. Pins are configured and read back before HSE is
enabled. No used pin is returned on success or after a startup error.

On F030, this example selects an 8 MHz crystal multiplied to 64 MHz:

```rust,ignore
use embassy_cw32::rcc::{ClockSource, HseConfig, PllConfig, PllSource};
let mut config = embassy_cw32::Config::default();
config.rcc.source = ClockSource::Pll(PllConfig::new(
    PllSource::Hse(HseConfig::crystal(8_000_000)),
    8,
));
config.rcc.supply_voltage_mv = 3300;
let p = embassy_cw32::try_init(config)?;
```

`PllSource::Hsi` instead uses `config.rcc.hsi_divider`. For example, the default
48 MHz /6 HSI input multiplied by 6 gives 48 MHz. An HSE-derived source leaves
HSI at its reset divider; the otherwise unused `hsi_divider` does not retune it.
HSI stays enabled throughout startup and afterward.

`Config::validate()` returns the requested clocks or a specific `ClockError`
without accessing hardware. `Config::clocks()` retains its previous return type
and panics for invalid configurations; use `validate()` when handling errors.
`rcc::clocks()` is published only after successful HAL initialization.

HSI-derived direct SYSCLK retains its historical nominal integer-Hz floor for
nonintegral divider ratios. Voltage and Flash limits use the exact rational
frequency or its ceiling, never that floor. New HSE and PLL profiles require
integral SYSCLK, HCLK and PCLK; for example, 8,000,001 Hz HSE followed by AHB /2
is rejected instead of being reported as 4,000,000 Hz. This also prevents a
rounded frequency from incorrectly passing the fixed 1 MHz Embassy timebase
check. With `time-driver-gtim1`, L012 needs an integral positive PCLK/1 MHz ratio
up to 65536 and F030 needs a power-of-two ratio up to 32768. These checks occur
before RCC MMIO. CW32 timer clocks are PCLK with no APB clock multiplier.

## Startup ordering and failure behavior

1. Validate all requested numerical limits and timebase compatibility before
   touching registers. Verify reset HSI source/divider and undivided buses, HSI
   enable, and disabled/unready HSE and PLL. Bootloader clock adoption and runtime
   source changes are unsupported.
2. Apply factory calibration, retaining existing LSI trim whenever a documented
   autonomous or retained LSI consumer may already be active. Poll HSI readiness
   within its configured limit and verify calibration readback.
3. Apply bus dividers first and verify them. Set Flash wait states for the
   largest reset, intermediate and final HCLK before increasing a source
   frequency. L012 uses its SYSCTRL.CR2 alias; F030 retains the FLASH configuration
   gate and writes FLASH.CR2. RMW preserves SWD, cache/prefetch and reserved bits.
4. Configure and reserve the actual HSE pads, write all HSE parameters while
   disabled, verify readback, enable, verify enable/control readback, and poll
   STABLE within the requested limit. Digital filtering stays disabled. DETCNT
   uses the manual's 8000/fHSE(MHz) rule, rounded up to avoid shortening the
   detection interval.
5. For F030 PLL, configure its source, multiplier, frequency bands and wait count
   while disabled/unready, preserving the debug-control nibble. Verify, enable
   and poll lock within its configured limit. HSE-fed PLL source encoding matches
   crystal or bypass mode. An already enabled or still-stable PLL is rejected;
   this API does not reconfigure it.
6. Verify source readiness, select SYSCLK, and verify source selection, bus
   dividers, enables, oscillator parameters and STABLE again. CR0 exposes no
   separate switch acknowledgement; its source readback plus enable/STABLE checks
   are the available startup evidence.

Poll limits are iteration counts, not elapsed-time guarantees. Zero bounds are
invalid. HSE wait selectors 0–3 correspond to 8192, 32768, 131072 and 262144 HSE
cycles. F030 PLL selectors 0–7 correspond to 128 through 16384 PLL cycles. Board
startup time depends on the crystal, load and environment; a default poll bound
is not a promise that every board will start within it.

An error after writes leaves completed settings in place, including enabled
oscillators, GPIO reservation and any changed Flash/divider configuration. No
rollback is claimed, no clock snapshot is published, and safe global acquisition
cannot be retried to recover oscillator pads as GPIO. Reset the device to retry
startup. Numerical/timebase errors occur before MMIO and do not consume global
ownership.

F030 CR1 CLKCCS/HSECCS/LSECCS are written as one as required by RM Rev2.5, while
other enables and lock bits are preserved. No new LSI request is issued by this
clock-source path. On L012, existing monitor settings are preserved. Clock fault
monitoring and retained LSI consumers can cause automatic fallback to HSI after
an HSE failure. Returned frequencies are a verified startup snapshot, valid only
while the source remains healthy; there is no runtime failure handling, automatic
recalculation, STOP/DeepSleep recovery or hardware-tested failover guarantee.

## Primary evidence and differences from the vendor examples

- [CW32L012 RM 1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  §§4.3.1–4.3.3, 4.4.3, 4.7.1–4.7.6 and 7.4: clock tree, HSE modes/limits,
  monitoring, HSI/bus dividers and Flash wait rules. The clock tree and SYSCLK
  selector document four sources and no PLL.
- [CW32x030 RM Rev2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
  §§4.3.3, 4.3.7, 4.7.1–4.7.8, 7.4 and 7.9.2: HSE, PLL, mandatory CR1 bits,
  preservation of PLL debug bits, FLASH clock prerequisite and wait states.
- [CW32L012 datasheet EN 1.0](https://en.whxy.com/uploads/files/20260115/CW32L012_DataSheet_EN_V1.0.pdf),
  Table 5-2 and §7.3.1 Table 7-4: actual package pins and voltage/frequency limits.
- [CW32F030 datasheet CN 1.9](https://www.whxy.com/uploads/files/20251229/CW32F030_DataSheet_CN_V1.9.pdf),
  pin tables, §7.3.1 Table 7-4 and §7.3.9 Table 7-21: supply/bus and PLL limits.
- L012 SDK 1.0.5 `cw32l012_sysctrl.h` HSE_PIN_PORT/HSE_PIN_IN/HSE_PIN_OUT definitions
  independently identify PF0/PF1. The L012 RM §4.5.4 PB7/PC13 example conflicts
  with its datasheet and SDK pin definitions; it is not used as pin evidence.
  The SDK's bypass routine also hardcodes an inconsistent GPIOA access. The HAL
  uses the generated, audited PF0/PF1 associations. RM's 0–7 L012 HSE drive range
  takes precedence over SDK macros offering additional undocumented values.
- F030 SDK 2.2 `cw32f030_rcc.c` confirms band endpoints: 8/16/24 MHz HSE,
  6/12/20 MHz PLL input, and 18/24/36/48 MHz PLL output use the upper band.
  The HAL preserves PLL debug bits instead of copying the SDK's whole-word write.
- The actual Embassy revision
  [b12a6d9efcd2711037abca1b63a661a9ef726444 RCC backend](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rcc/f013.rs)
  was inspected alongside its `rcc/mod.rs`. The source/PLL configuration,
  dependency ordering, Flash-before-switch principle, and source-ready/readback
  checks are architectural references. CW32 keeps its own register model,
  direct-reset-only contract, bounded error returns, pad reservation and counted
  peripheral resources. STM32 PLL dividers, bus timer doubling and switch-status
  fields are not transplanted.

External host probes exercise actual-source configuration/MMIO models and
negative APIs; target builds check both generated ARM interfaces. These are
software validation only. No hardware, destructive Flash operation, board
oscillator startup or electrical validation has been performed.
