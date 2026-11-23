# Direct HSE firmware

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

Real `thumbv6m-none-eabi` firmware for CW32F030C8T7, CW32A030C8T7 and
CW32F020C6U7, CW32L031C8T6, CW32R031C8U6.
Both binaries also cover every modeled exact L083 part: CW32L083RBT6,
CW32L083RCS6, CW32L083RCT6, CW32L083MCT6 and CW32L083VCT6.
Both also cover CW32L052C8T6, CW32L052R8T6 and CW32L052R8S6.
Exact features also cover CW32L010F8P6, CW32L010F8U6, CW32L010Y8M6,
CW32L011K8T6 and CW32L011K8U6 using the package wiring below.
The CI matrix compiles and links its enumerated production firmware; it never
executes it. Adding example features does not itself expand that matrix.
Source support is distinct from the build results recorded for a given snapshot.

Connect UART1 TX on PA8 to a 3.3 V serial receiver (115200 baud, 8N1), and an
LED with suitable resistor on PB0, **except for the L010/L011 routes below**.
UART timing comes from frozen PCLK. The LED busy delay is only visible activity
and is not a calibrated timebase.

- `crystal`: 16 MHz crystal/resonator between OSC_IN and OSC_OUT,
  including board-qualified load capacitors and drive level 2, with total
  frequency error bounded by ±30 ppm across the declared board conditions.
- `bypass`: externally powered 24 MHz digital clock on OSC_IN, with total error
  bounded by ±30 ppm across those conditions. OSC_OUT is free unless reserved
  by an incoming enabled crystal. Meet all input requirements: 40–60% duty,
  at least 15 ns high and low, no more than 20 ns rise
  or fall, and the own datasheet's input voltage thresholds. The source must
  already be stable and remain present. Nominal 24 MHz + positive tolerance
  requires Flash WAIT1 even though nominal frequency equals the WAIT0 ceiling.

OSC_IN/OSC_OUT are PF0/PF1 on the other listed families; L010/L011 use their
own pads in the table below. With default AHB/APB divisors of 1, the nominal
HCLK/PCLK is 16 MHz for `crystal` and 24 MHz for `bypass`. A serial receiver can
check baud timing against an independent reference; a build cannot prove the
declared source accuracy or startup behavior.

Both binaries declare MCU VDD 3.0–3.6 V and ambient −20–70°C. Adapt the source
frequency bounds and conditions to real oscillator characterization; the MCU
cannot verify these declarations. Include oscillator supply, load, temperature,
aging and short-term cycle variation in the bounds; a long-term-average ppm
rating alone does not establish every ClockBounds duration guarantee. The
independent HSI is retained with its own factory qualification. The inherited
LSI configuration must satisfy its own documented safe operating range; the
L011 discrepancy is called out below. RCC retains its trim. F020/x030 require
mandatory CCS; L031/R031 preserve configurable CCS and retain LSI when an enabled
external detector needs it. See `../../../docs/qualified-l031-hse.md`.
L083 also preserves configurable CCS; retained RTC or AUTOTRIM TIMER users
require exact ready HSE/pad reuse. Active AUTOTRIM calibration or automatic
mode is excluded, as is active ETR with an HSE request. Raw-HSI AUTOTRIM/LVD
owners block a required factory retrim. See `../../../docs/qualified-l083-hse.md`.

L052 uses PF0/PF1 physical pins5/6 on LQFP48 and both LQFP64 bodies.
Its pre-start and run fields use the same board-selected drive/range.
Configurable CCS fallback forces HSI/6 independently of the requested divider;
retained RTC/AUTOTRIM/LVD restrictions apply. See
`../../../docs/qualified-l052-hse.md`. VDDA must equal VDD.

## CW32L010/CW32L011 exact package wiring

These are the physical package positions from the own pin tables. UART1 PA6
on L010 and PA9 on L011 both use their own AF1 route. The example's safe pin
constructors select those routes; no raw PAC or debug-pin remap is needed.

| Chip feature | Package | OSC_IN | OSC_OUT | UART1 TX | LED |
| --- | --- | --- | --- | --- | --- |
| `cw32l010f8p6` | TSSOP20 | PA0, pin 5 | PA1, pin 6 | PA6, pin 16 | PA3, pin 13 |
| `cw32l010f8u6` | QFN20 | PA0, pin 2 | PA1, pin 3 | PA6, pin 13 | PA3, pin 10 |
| `cw32l010y8m6` | SOP16 | PA0, pin 4 | PA1, pin 5 | PA6, pin 14 | PA3, pin 11 |
| `cw32l011k8t6` | LQFP32 | PC13, pin 31 | PB7, pin 30 | PA9, pin 19 | PB0, pin 14 |
| `cw32l011k8u6` | QFN32 | PC13, pin 31 | PB7, pin 30 | PA9, pin 19 | PB0, pin 14 |

Sources: [L010 DS Table 5-2 and AF Table 5-3, PDF 24–25](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf#page=24),
[L011 DS Table 5-2 and AF Table 5-3, PDF 31](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf#page=31).
The repository's `cw32-data/pinouts/cw32l010.yaml`, `cw32l011.yaml` and respective
`cw32-data/af/*-serial.yaml` retain the package and route evidence. L010's PA8
is SWCLK and is deliberately not the example UART; PB0/PB1 remain free for
an inherited LSE. Neither family's chosen UART/LED conflicts with its HSE pads.
L011 requires VDDA=VDD. Match receiver logic levels and LED current limits to
the actual supply and own I/O ratings; use a common ground.

Crystal mode and L011 bypass admit actual 4–32 MHz. L010 bypass admits
3,908,159–32,000,000 Hz in authored metadata; the extension is pending acceptance.
L011 has conflicting RM 4 MHz and DS 1 MHz lower bounds. Factory HSIOSC remains 48 MHz ±2% on L010 or 96 MHz ±2% on L011, with
its independent bound. Inherited configurable CCS can force nominal 4 MHz
(actual 3.92–4.08 MHz) regardless of the requested HSI divider. The L011 RM
32.8 kHz ±10% safe LSI range conflicts with DS factory −10/+25%; detector
arithmetic uses 41 kHz conservatively without resolving that discrepancy.
See [the full L010/L011 qualification](../../../docs/qualified-l010-l011-hse.md)
for entry conditions, retained RTC/AWT and analog ownership, and failure limits.

## L010 low-frequency bypass

`bypass-low` requires `l010-low-hse` and exactly one of `cw32l010f8p6`,
`cw32l010f8u6` or `cw32l010y8m6`, with default features disabled. It uses the
L010 wiring above: external 3.95 MHz on PA0, UART1 PA6 at **9600 8N1**, LED PA3.
The source must remain within 3,949,881–3,950,119 Hz and the stated waveform and
3.0–3.6 V / −20–70°C conditions. `retained-hse` keeps HSI as SYSCLK.

The detector count is 2026. At the supported lower bound 3,908,159 Hz it is
2047; lower input bounds overflow the existing policy. No detector, startup,
CCS, retained-consumer or pin-ownership behavior is changed.

This example and range extension have not been generated, compiled or tested.
Fresh metadata/PAC generation is required before later authorized builds;
existing generated files retain the old limit. Local builds/tests remain prohibited.

## CW32L083 exact package wiring

L083 oscillator package pins, from its own DS Table5-2 PDF27:

| Chip feature | Package | PF0 OSC_IN | PF1 OSC_OUT |
| --- | --- | --- | --- |
| `cw32l083rbt6` | LQFP64 10×10 mm, 128 KiB Flash | 5 | 6 |
| `cw32l083rcs6` | LQFP64 7×7 mm, 256 KiB Flash | 5 | 6 |
| `cw32l083rct6` | LQFP64 10×10 mm, 256 KiB Flash | 5 | 6 |
| `cw32l083mct6` | LQFP80, 256 KiB Flash | 7 | 8 |
| `cw32l083vct6` | LQFP100, 256 KiB Flash | 12 | 13 |

The existing example uses each package's qualified UART1 PA8 route and PB0 LED.
L083 additionally requires VDDA=VDD; fit load components and drive for the actual
board. Selecting an exact feature supplies memory and bonded-pin metadata, not
evidence that an attached board has this oscillator or wiring.

## Build and failure limits

The own OSC_IN/OSC_OUT pads are unavailable through safe GPIO construction
while crystal HSE is reserved; bypass reserves only OSC_IN unless OSC_OUT was
already reserved by an incoming enabled crystal, which keeps its whole-boot
reservation. Raw PAC use can invalidate the clock contract. Startup failure
does not publish frozen clocks; reset before retrying.
No runtime clock-loss or deep-sleep recovery is implemented.

Build (choose one chip feature and one binary):

```sh
cargo build --release --target thumbv6m-none-eabi --no-default-features \
  --features cw32f030c8t7 --bin crystal

cargo build --release --target thumbv6m-none-eabi --no-default-features \
  --features cw32l083rct6 --bin bypass

cargo build --release --target thumbv6m-none-eabi --no-default-features \
  --features cw32l010y8m6 --bin crystal

cargo build --release --target thumbv6m-none-eabi --no-default-features \
  --features cw32l011k8u6 --bin bypass
```

For all five L010/L011 exact features, run from `examples/shared/hse-clock` after the
normal source/PAC generation step:

```sh
for chip in cw32l010f8p6 cw32l010f8u6 cw32l010y8m6 cw32l011k8t6 cw32l011k8u6; do
  cargo build --release --target thumbv6m-none-eabi --no-default-features \
    --features "$chip" --bins || exit
done
```

These commands are intended production ARM builds, not a record that they ran
or passed. Each binary has its own Cortex-M entry point and links with memory
from the selected exact part.

L012 supports both CW32L012C8T6/LQFP48 and CW32L012C8U6/QFN48 with PF0/OSC_IN
physical5 and PF1/OSC_OUT physical6. The examples use PB0 for observable GPIO
and PA9/UART1_TX, leaving HSE and inherited LSE pads alone. HSI defaults to8MHz;
its independent CCS fallback is4MHz. Start from reset or provide exactly matching
inherited HSE settings. The complete board/source and retained-owner contract
is [qualified-l012-hse.md](../../../docs/qualified-l012-hse.md).


## L012 retained HSE entry and Embassy time

The `retained-hse` feature keeps the same explicit `Some(Hse)` board declaration
and requests HSI SYSCLK instead. For an enabled incoming HSE, that declaration
requests exact readback-only reuse; it does not authorize retuning. These examples
do not create a bootloader state. Before entry, the bootloader must already leave
a legal stable source/bus/Flash state and the matching HSE enabled and stable,
with the declared source continuously present. The preserved HSI trim must yield a legal
90–100 MHz HSIOSC when the initializer enables it, even if HSI is disabled
on entry.
The HAL cannot measure either source or any board condition.

For the unmodified L012 example declarations, the incoming HSE must have both
drive fields at level 2, WAITCYCLE=262144 cycles, DIGFLT=0, and DETCNT=501 for the
16 MHz crystal or 334 for the 24 MHz bypass. MODE and the PF0/PF1 pad configuration
must match the declaration: no output driver, pull-up, edge interrupts, open
drain or filter; GPIO mux 0; analog input for crystal PF0/PF1, digital input for
bypass PF0. Crystal retains both pads; bypass retains PF0. The initializer checks
this state and refuses a mismatch before reconfiguring the retained source.

If RTC/AWT retains HSE, PSC1+1 must be at least 17 for the crystal bounds or 25 for
the bypass bounds, so the actual RTCCLKD upper bound stays at or below 1 MHz.
RTC START=0 does not release the source. The documented incoming RTC/HSI/LSE/LSI
and analog-owner constraints also apply; see the full qualification document.
PB0 and PA9 remain available for the LED and UART; safe construction of an
owned oscillator pin is rejected. No example changes those retained registers.

`hse-time` uses the same 24 MHz bypass declaration and PA9/PB0 L012 wiring with
the production GTIM1 Embassy time driver. Its async entry toggles the LED and
writes UART after 250 ms timer waits; it reads `Instant` and the source-qualified
tick bounds. With `retained-hse`, HSE stays declared while the timer uses nominal
8 MHz HSI instead. Source tolerance applies to wall-clock timing in both cases.
The continuous-run time-driver contract applies: GTIM1 interrupt latency must
stay below 32768 actual timer ticks (half a wrap). Debug stops, Flash stalls and
deep sleep are unqualified; source-only examples and ELF builds do not prove
runtime timing. See [the time-driver contract](../../../docs/time-driver.md).

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --features cw32l012c8t6,retained-hse --bin crystal
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --features cw32l012c8u6,retained-hse --bin bypass
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --features cw32l012c8t6,embassy-time --bin hse-time
```

These are build instructions, not claims of firmware execution or hardware
verification. The selected exact package supplies its own memory and vector table.
