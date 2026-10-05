# CW32F030C8 die pins and motor-control signal audit

## Chip-level identity and audit scope

The HAL selects **CW32F030C8** (feature `cw32f030c8`), with 64 KiB Flash
and 8 KiB SRAM. Package and temperature suffixes are deliberately not part
of the HAL identity. Chip YAML contains die-level GPIO identities and routes;
there is no packages data layer, package intersection or package-specific
metadata. The board author must verify which signals are bonded and wired.

The original physical evidence includes CW32F030C8T7 LQFP48 (temperature
suffix 7: -40 to +105 degrees Celsius); the drawing below is evidence only,
not a package selector. Its 39 GPIOs match the conservatively exposed C8
set. PF3/BOOT is excluded because the latest datasheet marks it dedicated,
even though the older manual/header expose its register bit. PA13/PA14
remain die identities; applications must preserve debug unless deliberately
reclaiming those pins.

The original route audit covered the external **ADC, ATIM and VC1/VC2** signals from
the pin/AF tables: 13 ADC inputs, 26 ATIM routes, 16 comparator inputs and
6 comparator outputs, totaling **61 routes**. Later timer work added GTIM
routes, and v0.19 adds **116 UART/SPI/I2C routes**, giving 223 routes at that stage. v0.20 adds the two dedicated LSE routes,
bringing the total to 225 at that stage; v0.22 adds the two HSE routes for a current total of 227.
See [timer/PWM route evidence](timer-pwm-route-evidence.md) and the
[bus implementation stage](buses-v0.19.0.md) for the later route coverage. BTIM,
clock-output and LVD routes remain outside this focused audit. Dedicated LSE/HSE pads are documented in [RTC](rtc.md) and [external clocks](external-clocks.md);
absence does not imply that the silicon lacks those functions. The F030
does not gain L012-only ADC2, OPA, DAC or CORDIC capabilities from this data.

## Official evidence

The following official documents were independently cross-checked on
2026-10-02. Page references below use the printed page number, where given.

- [CW32F030 datasheet, Rev 1.9](https://www.whxy.com/uploads/files/20251229/CW32F030_DataSheet_CN_V1.9.pdf):
  Figure 5-1 (p20) visually inspected; Table 5-2 (pp23-27) for pad assignment
  and analog functions; Tables 5-3/5-4 (pp28-29) for GPIOA/GPIOB AF numbers;
  Tables 3-1 and 9-1 for capacity, package and order code.
- [CW32x030 user manual, Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf):
  Table 9-2 for AF assignments, section 15.3.1.6 and Table 15-3 for ATIM
  channels, and section 23.3.2/Table 23-2 for comparator inputs and outputs.
- [CW32F030 Standard Peripheral Library, V2.2](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip):
  `Libraries/inc/cw32f030_gpio.h`, particularly the `PAxx_AFx_*` and
  `PBxx_AFx_*` macros, provides an independent AF encoding cross-check.

Pinned SHA-256 values:

| Source | SHA-256 |
| --- | --- |
| Datasheet PDF | `04ef91434320e5d05a3b6690fead0fedb31a7d9bad28c96b655a6e13a22e46b2` |
| User manual PDF | `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2` |
| SDK ZIP | `7c431df43d7075b817a51d818ea4c9aba55f83b7c9c77bf0065976758954b780` |
| SDK GPIO header | `3ffe458a615f2dd099c6a14283e0438cfece2260444f4167a4c05c62930abe2e` |

## Complete LQFP48 bonding

Signal names drop the vendor's leading zero: `PA00` becomes `PA0`.

| Physical pad | Signal | Physical pad | Signal | Physical pad | Signal |
| ---: | --- | ---: | --- | ---: | --- |
| 1 | VDD | 17 | PA7 | 33 | PA12 |
| 2 | PC13 | 18 | PB0 | 34 | PA13 / SWDIO |
| 3 | PC14 | 19 | PB1 | 35 | PF6 |
| 4 | PC15 | 20 | PB2 | 36 | PF7 |
| 5 | PF0 | 21 | PB10 | 37 | PA14 / SWCLK |
| 6 | PF1 | 22 | PB11 | 38 | PA15 |
| 7 | NRST | 23 | VSS | 39 | PB3 |
| 8 | VSSA | 24 | VDD | 40 | PB4 |
| 9 | VDDA | 25 | PB12 | 41 | PB5 |
| 10 | PA0 | 26 | PB13 | 42 | PB6 |
| 11 | PA1 | 27 | PB14 | 43 | PB7 |
| 12 | PA2 | 28 | PB15 | 44 | BOOT |
| 13 | PA3 | 29 | PA8 | 45 | PB8 |
| 14 | PA4 | 30 | PA9 | 46 | PB9 |
| 15 | PA5 | 31 | PA10 | 47 | VSS |
| 16 | PA6 | 32 | PA11 | 48 | VDD |

The nine non-GPIO pads are 1, 7, 8, 9, 23, 24, 44, 47 and 48. The
datasheet names pad 44 `PF03/BOOT` but marks its type **I**, structure **B**,
and lists no GPIO alternate or analog functions. Figure 5-1 labels it BOOT.
The 39-GPIO count also excludes it. Consequently it is not represented as
an ordinary output-capable `PF3` token; the L012 GPIO PF3 must not leak into
the F030 package. PF0/PF1 and PC14/PC15 retain GPIO ownership despite their
optional oscillator functions.

## ATIM routes

All routes below select **AF7**. The official `A`/`B` signal suffixes are
preserved. They identify F030 channel outputs/capture inputs, not an
assumption that the peripheral is register-compatible with L012 `CHx/CHxN`.

| ATIM signal | GPIOs |
| --- | --- |
| CH1A | PA5, PA8, PB2, PB5 |
| CH1B | PA7, PA15, PB13 |
| CH2A | PA4, PA9, PB6, PB10 |
| CH2B | PB0, PB3, PB14 |
| CH3A | PA3, PA10, PB7, PB11 |
| CH3B | PB1, PB4, PB15 |
| BK | PA6, PB9, PB12 |
| ETR | PA12, PB8 |

CH4 is internal only (manual section 15.3.1.6), so no external CH4 route is
invented. ATIM GATE is excluded for the evidence conflict described below.

## ADC and comparator routes

These are analog input routes (`af: null`), not AF0. Analog mode requires
the corresponding GPIO ANALOG bit. The F030 has one ADC instance named
`ADC`; its 13 external inputs map in order as follows:

| ADC input | GPIO | ADC input | GPIO |
| --- | --- | --- | --- |
| IN0 | PA0 | IN7 | PA7 |
| IN1 | PA1 | IN8 | PB0 |
| IN2 | PA2 | IN9 | PB1 |
| IN3 | PA3 | IN10 | PB2 |
| IN4 | PA4 | IN11 | PB10 |
| IN5 | PA5 | IN12 | PB11 |
| IN6 | PA6 | | |

Internal ADC temperature/reference/supply channels have no external GPIO
route. PB0's external-reference function is not another ADC input channel.

| Comparator channel | VC1 GPIO | VC2 GPIO |
| --- | --- | --- |
| CH0 | PA0 | PA5 |
| CH1 | PA1 | PA6 |
| CH2 | PA2 | PA7 |
| CH3 | PA3 | PB0 |
| CH4 | PA4 | PB1 |
| CH5 | PA5 | PB2 |
| CH6 | PA6 | PB10 |
| CH7 | PA7 | PB11 |

Comparator digital output routes are separate from analog input routes:

| Signal | GPIO / AF |
| --- | --- |
| VC1 OUT | PA6 / AF3, PA0 / AF4, PA11 / AF4 |
| VC2 OUT | PA7 / AF3, PA2 / AF4, PA12 / AF4 |

## Conflicting evidence and conservative resolutions

1. **ATIM CH2A PA4 versus PA5.** User manual Table 15-3 lists PA05 as the
   first CH2A pin, overlapping CH1A. Datasheet Tables 5-2/5-3, user manual
   Table 9-2, and the SDK `PA04_AFx_ATIMCH2A()` macro agree on PA04/AF7.
   This dataset uses PA4/AF7 for CH2A and PA5/AF7 for CH1A, treating the
   isolated Table 15-3 entry as a documentation error.
2. **SDK-only ATIM GATE.** The SDK defines `PA11_AFx_ATIMGATE()` as AF7,
   but datasheet Table 5-3 and user manual Table 9-2 leave PA11/AF7 empty,
   and the datasheet pad-function table does not list it. The dataset
   does not publish this uncorroborated route.
3. **BOOT naming.** A `PF03` label or generic GPIO register description is
   insufficient evidence of an ordinary bonded GPIO. The specific F030
   pad type, drawing and total GPIO count take precedence, as described
   above. The dedicated boot input is never handed to an ordinary output
   constructor.

No undisclosed source correction or inference is used to add a peripheral
capability. When official sources disagree, the selected value needs
multiple independent detailed references, or the disputed route remains
excluded.
