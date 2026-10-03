# CW32L012C8 chip GPIO and FOC route evidence

## Scope

The data stores 40 chip GPIO capabilities directly in `chip.pins`, with no Package model or physical pad numbers. Vendor package drawings below are historical source evidence for signal names, not a maintained HAL package database. Board software must check the actual part and footprint. The reviewed drawings agree on these GPIO identities; this does not release SWD, oscillator or BOOT functions.

This original focused audit added **82 routes**: 28 ATIM CH1/CH1N/CH2/CH2N/CH3/CH3N/BK digital routes, 24 external ADC inputs, 12 OPA analog routes, 16 VC analog inputs, and two DAC analog outputs. Later work added timer routes documented in `cw32-data/sources/timer-pwm-routes.yaml` and **133 UART/SPI/I2C routes** in `cw32-data/sources/bus-routes.yaml`, giving 259 routes at that stage. The two dedicated LSE routes added in v0.20 bring the current L012 total to 261. Internal triggers and DMA selectors have separate evidence; this document is not a complete pinmux audit.

Sources:

- [CW32L012 datasheet v1.0](https://www.whxy.com/uploads/files/20260115/CW32L012_DataSheet_EN_V1.0.pdf): Figures 5-1 and 5-2, printed p.32 (physical PDF page 36); Table 5-2, pp.34–37; alternate-function Tables 5-3 onward, pp.38–40.
- [CW32L012 user manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf): GPIO §§9.3.4–9.3.5, Table 9-2 pp.128–129; ADC Table 25-4 p.578; VC Table 27-2 p.620; OPA Table 29-1 p.646.
- [SDK v1.0.5](https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip): `Libraries/inc/cw32l012_gpio.h` AF and ANALOG_ENABLE macros, `Libraries/inc/cw32l012_opa.h` pin descriptions, and `Libraries/src/cw32l012_dac.c` channel-to-pin declarations.

The pinout diagrams were rendered and visually inspected in addition to checking text/table extraction. In particular, pin 44 is labelled BOOT in both drawings; Table 5-2 explicitly identifies it as PF03/BOOT. No pad number was inferred from port numbering. The two top-view diagrams and pin table agree.

## All 48 numbered package pins

The pad number applies to **both** C8T6/LQFP48 and C8U6/QFN48. GPIO spelling in metadata drops leading zeroes: PA00 becomes PA0.

| Pad | Signal / GPIO token | Notes |
| --- | --- | --- |
| 1 | Vcore | Regulator output; dedicated, not GPIO |
| 2 | PC13 | Bonded GPIO |
| 3 | PC14 | OSC32_IN alternate use |
| 4 | PC15 | OSC32_OUT alternate use |
| 5 | PF0 | OSC_IN alternate use |
| 6 | PF1 | OSC_OUT alternate use |
| 7 | NRST | Dedicated reset input, not GPIO |
| 8 | VSSA | Analog ground, not GPIO |
| 9 | VDDA | Analog supply, not GPIO |
| 10 | PA0 | Bonded GPIO |
| 11 | PA1 | Bonded GPIO |
| 12 | PA2 | Bonded GPIO |
| 13 | PA3 | Bonded GPIO |
| 14 | PA4 | Bonded GPIO |
| 15 | PA5 | Bonded GPIO |
| 16 | PA6 | Bonded GPIO |
| 17 | PA7 | Bonded GPIO |
| 18 | PB0 | Bonded GPIO |
| 19 | PB1 | Bonded GPIO |
| 20 | PB2 | Bonded GPIO |
| 21 | PB10 | Bonded GPIO |
| 22 | PB11 | Bonded GPIO |
| 23 | VSS | Digital ground, not GPIO |
| 24 | VDD | Digital supply, not GPIO |
| 25 | PB12 | Bonded GPIO |
| 26 | PB13 | Bonded GPIO |
| 27 | PB14 | Bonded GPIO |
| 28 | PB15 | Bonded GPIO |
| 29 | PA8 | Bonded GPIO |
| 30 | PA9 | Bonded GPIO |
| 31 | PA10 | Bonded GPIO |
| 32 | PA11 | Bonded GPIO |
| 33 | PA12 | Bonded GPIO |
| 34 | PA13 | SWDIO by default; requires explicit debug-port release before GPIO use |
| 35 | PF6 | Bonded GPIO |
| 36 | PF7 | Bonded GPIO |
| 37 | PA14 | SWCLK by default; requires explicit debug-port release before GPIO use |
| 38 | PA15 | Bonded GPIO |
| 39 | PB3 | Bonded GPIO |
| 40 | PB4 | Bonded GPIO |
| 41 | PB5 | Bonded GPIO |
| 42 | PB6 | Bonded GPIO |
| 43 | PB7 | Bonded GPIO |
| 44 | PF3 | BOOT strap / PF3; sampled at reset |
| 45 | PB8 | Bonded GPIO |
| 46 | PB9 | Bonded GPIO |
| 47 | VSS | Digital ground, not GPIO |
| 48 | VDD | Digital supply, not GPIO |

This inventory covers the 48 numbered pads in the pinout, not mechanical land-pattern or exposed-pad assembly guidance. Consult the package drawing for PCB design. GPIO availability at runtime also depends on SWD, oscillator, BOOT and application configuration; bonding alone does not release those functions. Existing PF3-only pull-down and GPIO implemented masks are unchanged.

## Digital ATIM routes

Names intentionally match the HAL-facing contract: peripheral `ATIM`, signal `CH1`, `CH1N`, `CH2`, `CH2N`, `CH3`, `CH3N` or `BK`. `BK` corresponds to the manual's `ATIM_BK` and the SDK's `ATIMBKIN` macro spelling. It is not BK2 or BKOUT.

Every entry below is independently checked against manual Table 9-2 and the SDK macro that writes the same pin's AFRL/AFRH field. The datasheet alternate-function tables also agree. All main/complementary CH1–3 choices use AF7; BK has AF5/6/7 choices.

| ATIM signal | Pin | Pad, both packages | AF | SDK macro |
| --- | --- | --- | --- | --- |
| CH1 | PA5 | 15 | 7 | `PA05_AFx_ATIMCH1()` |
| CH1 | PA8 | 29 | 7 | `PA08_AFx_ATIMCH1()` |
| CH1 | PB2 | 20 | 7 | `PB02_AFx_ATIMCH1()` |
| CH1 | PB5 | 41 | 7 | `PB05_AFx_ATIMCH1()` |
| CH1N | PA7 | 17 | 7 | `PA07_AFx_ATIMCH1N()` |
| CH1N | PA15 | 38 | 7 | `PA15_AFx_ATIMCH1N()` |
| CH1N | PB13 | 26 | 7 | `PB13_AFx_ATIMCH1N()` |
| CH2 | PA9 | 30 | 7 | `PA09_AFx_ATIMCH2()` |
| CH2 | PB6 | 42 | 7 | `PB06_AFx_ATIMCH2()` |
| CH2 | PB10 | 21 | 7 | `PB10_AFx_ATIMCH2()` |
| CH2N | PA4 | 14 | 7 | `PA04_AFx_ATIMCH2N()` |
| CH2N | PB0 | 18 | 7 | `PB00_AFx_ATIMCH2N()` |
| CH2N | PB3 | 39 | 7 | `PB03_AFx_ATIMCH2N()` |
| CH2N | PB14 | 27 | 7 | `PB14_AFx_ATIMCH2N()` |
| CH3 | PA10 | 31 | 7 | `PA10_AFx_ATIMCH3()` |
| CH3 | PB7 | 43 | 7 | `PB07_AFx_ATIMCH3()` |
| CH3 | PB11 | 22 | 7 | `PB11_AFx_ATIMCH3()` |
| CH3N | PB1 | 19 | 7 | `PB01_AFx_ATIMCH3N()` |
| CH3N | PB4 | 40 | 7 | `PB04_AFx_ATIMCH3N()` |
| CH3N | PB15 | 28 | 7 | `PB15_AFx_ATIMCH3N()` |
| BK | PA0 | 10 | 7 | `PA00_AFx_ATIMBKIN()` |
| BK | PA6 | 16 | 7 | `PA06_AFx_ATIMBKIN()` |
| BK | PA12 | 33 | 5 | `PA12_AFx_ATIMBKIN()` |
| BK | PA13 | 34 | 5 | `PA13_AFx_ATIMBKIN()` |
| BK | PB5 | 41 | 5 | `PB05_AFx_ATIMBKIN()` |
| BK | PB9 | 46 | 7 | `PB09_AFx_ATIMBKIN()` |
| BK | PB12 | 25 | 7 | `PB12_AFx_ATIMBKIN()` |
| BK | PC13 | 2 | 6 | `PC13_AFx_ATIMBKIN()` |

PA13.BK is physically bonded but uses the default SWDIO pin. Drivers must not disable SWD silently. Selecting a legal route proves only pin/AF compatibility; it does not establish safe motor wiring, output polarity, dead time or fault response.

## Analog routes

Analog routes have `af: null`, not AF0. They use the GPIO ANALOG bit rather than an AFR selection. User manual §9.3.4 and the SDK `PxNN_ANALOG_ENABLE()` macros independently establish that mode choice. Every analog pin below has the corresponding SDK macro, and Table 5-2 lists each signal as analog.

The signal-to-pin mapping is cross-checked as follows:

- ADC: datasheet Table 5-2 and manual Table 25-4. IN0..IN11 are physical input numbers, not the eight sequence-slot numbers. IN12/13 are internal DAC paths, while temperature/reference channels are internal sources, so none is invented as a bonded external pin route.
- OPA: datasheet Table 5-2, manual Table 29-1 and SDK `cw32l012_opa.h`. INP4 is an internal DAC connection and has no external pin route.
- VC: datasheet Table 5-2 and manual Table 27-2. CH0..CH3 name the external mux inputs. Internal DAC/BGR/resistor-divider choices are not GPIO routes. This section does not add the separate digital VC OUT routes.
- DAC: datasheet Table 5-2 and the explicit `DAC_CHANNEL1 PB00` / `DAC_CHANNEL2 PB01` declarations in SDK `cw32l012_dac.c`. Analog GPIO mode and DAC CR1 output enable are distinct requirements; an `af: null` route does not enable a DAC output by itself.

| Peripheral | Signal | Pin | Pad, both packages | AF |
| --- | --- | --- | --- | --- |
| ADC1 | IN0 | PA0 | 10 | null (analog) |
| ADC1 | IN1 | PA1 | 11 | null (analog) |
| ADC1 | IN2 | PA2 | 12 | null (analog) |
| ADC1 | IN3 | PA3 | 13 | null (analog) |
| ADC1 | IN4 | PA4 | 14 | null (analog) |
| ADC1 | IN5 | PA5 | 15 | null (analog) |
| ADC1 | IN6 | PA6 | 16 | null (analog) |
| ADC1 | IN7 | PA7 | 17 | null (analog) |
| ADC1 | IN8 | PB0 | 18 | null (analog) |
| ADC1 | IN9 | PB1 | 19 | null (analog) |
| ADC1 | IN10 | PB10 | 21 | null (analog) |
| ADC1 | IN11 | PB2 | 20 | null (analog) |
| ADC2 | IN0 | PA5 | 15 | null (analog) |
| ADC2 | IN1 | PA6 | 16 | null (analog) |
| ADC2 | IN2 | PA7 | 17 | null (analog) |
| ADC2 | IN3 | PB0 | 18 | null (analog) |
| ADC2 | IN4 | PB1 | 19 | null (analog) |
| ADC2 | IN5 | PA8 | 29 | null (analog) |
| ADC2 | IN6 | PA9 | 30 | null (analog) |
| ADC2 | IN7 | PA10 | 31 | null (analog) |
| ADC2 | IN8 | PA11 | 32 | null (analog) |
| ADC2 | IN9 | PA12 | 33 | null (analog) |
| ADC2 | IN10 | PB10 | 21 | null (analog) |
| ADC2 | IN11 | PB2 | 20 | null (analog) |
| OPA1 | INP1 | PA3 | 13 | null (analog) |
| OPA1 | INP2 | PA6 | 16 | null (analog) |
| OPA1 | INP3 | PB1 | 19 | null (analog) |
| OPA1 | INN1 | PA4 | 14 | null (analog) |
| OPA1 | INN2 | PA7 | 17 | null (analog) |
| OPA1 | OUT | PB0 | 18 | null (analog) |
| OPA2 | INP1 | PA4 | 14 | null (analog) |
| OPA2 | INP2 | PA6 | 16 | null (analog) |
| OPA2 | INP3 | PB0 | 18 | null (analog) |
| OPA2 | INN1 | PA5 | 15 | null (analog) |
| OPA2 | INN2 | PA7 | 17 | null (analog) |
| OPA2 | OUT | PB1 | 19 | null (analog) |
| VC1 | CH0 | PA0 | 10 | null (analog) |
| VC1 | CH1 | PA2 | 12 | null (analog) |
| VC1 | CH2 | PA4 | 14 | null (analog) |
| VC1 | CH3 | PB0 | 18 | null (analog) |
| VC2 | CH0 | PA1 | 11 | null (analog) |
| VC2 | CH1 | PA3 | 13 | null (analog) |
| VC2 | CH2 | PA5 | 15 | null (analog) |
| VC2 | CH3 | PB1 | 19 | null (analog) |
| VC3 | CH0 | PA6 | 16 | null (analog) |
| VC3 | CH1 | PA7 | 17 | null (analog) |
| VC3 | CH2 | PB2 | 20 | null (analog) |
| VC3 | CH3 | PB0 | 18 | null (analog) |
| VC4 | CH0 | PA9 | 30 | null (analog) |
| VC4 | CH1 | PA10 | 31 | null (analog) |
| VC4 | CH2 | PB10 | 21 | null (analog) |
| VC4 | CH3 | PB1 | 19 | null (analog) |
| DAC | OUT1 | PB0 | 18 | null (analog) |
| DAC | OUT2 | PB1 | 19 | null (analog) |

The same pad can appear in several routes (for example PB0 is ADC1.IN8, ADC2.IN3, OPA1.OUT, OPA2.INP3, VC1.CH3, VC3.CH3 and DAC.OUT1). These are alternatives or explicitly configured analog connections, not independent physical pins. The HAL must retain single-pin ownership and avoid treating all routes as simultaneously allocatable.

## Validation and limits

- The chip has exactly 40 unique GPIO identities. No package file or pad-number field is generated.
- The full diagrams/table account for all 48 numbered pads: 40 GPIO plus eight dedicated pads.
- Every route pin exists in chip.pins and falls within the corresponding family implemented mask.
- All 28 ATIM signal/AF pairs were compared programmatically with the exact SDK macros and manually with the AF tables; analog routes have no AFR encoding.
- All 54 analog routes have a GPIO analog-enable macro; their signal-to-pin assignments were checked against the sources above.
- Chip memory, peripheral ownership and remap/quirk sections remain independent of board packaging. No omitted route is inferred.

The tables above record the audited pin and route evidence; compare changes against the pinned vendor GPIO header.

The normalized JSON contains a direct 40-pin chip capability list and all 82 routes, with no package maps. To regenerate it independently with the unified tool, use `cargo run --offline -p cw32-gen -- data cw32-data cw32l012c8 generated/data` in a fresh output directory. Workspace/PAC/HAL checks are separate from this data-only check.

No hardware test was performed. This data does not establish electrical suitability, simultaneous analog connectivity, pin current limits, motor-power safety, trigger timing or application board wiring.
