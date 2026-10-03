# Generic timer/PWM output route evidence

Ninety routes below are represented in chip YAML and feed generated timer pin capabilities. No hardware execution or electrical validation has been performed.

## Result

- F030: 46 GTIM CH1-4 routes (GTIM1 8, GTIM2 12, GTIM3 13, GTIM4 13).
- L012: 41 GTIM CH1-4 routes (GTIM1 11, GTIM2 14, GTIM3 8, GTIM4 8), plus 3 ATIM CH4 routes.
- Every endpoint is present in the respective current chip.pins set; no inferred pin tokens.
- Output capability is proved by timer output sections, independently of the AF tables. All listed routes select digital output and a stated AF; no SYSCTRL output remap is required.

## Main-output route table

AF values appear after /; leading zeros are normalized to the existing chip-token convention.

### cw32f030c8

| Peripheral | Signal | Pin / AF |
| --- | --- | --- |
| GTIM1 | CH1 | PA6/6, PB4/6 |
| GTIM1 | CH2 | PA7/6, PB5/6 |
| GTIM1 | CH3 | PB0/6, PB8/6 |
| GTIM1 | CH4 | PB1/6, PB9/6 |
| GTIM2 | CH1 | PA0/6, PA5/6, PA15/2, PB14/1 |
| GTIM2 | CH2 | PA1/6, PA3/3, PB3/2, PB15/1 |
| GTIM2 | CH3 | PA2/6, PB10/6 |
| GTIM2 | CH4 | PA3/6, PB11/6 |
| GTIM3 | CH1 | PA6/1, PA9/6, PB8/2, PC14/7 |
| GTIM3 | CH2 | PA10/6, PB7/2, PC15/7 |
| GTIM3 | CH3 | PA11/6, PB6/2, PF0/7 |
| GTIM3 | CH4 | PA12/6, PB5/2, PF1/7 |
| GTIM4 | CH1 | PA7/1, PB9/2, PB15/2, PF1/2 |
| GTIM4 | CH2 | PB8/4, PB14/2, PF0/2 |
| GTIM4 | CH3 | PB7/4, PB13/2, PC15/2 |
| GTIM4 | CH4 | PB6/4, PB12/2, PC14/2 |

### cw32l012c8

| Peripheral | Signal | Pin / AF |
| --- | --- | --- |
| ATIM | CH4 | PA3/7, PA11/7, PB9/5 |
| GTIM1 | CH1 | PA6/6, PB4/6 |
| GTIM1 | CH2 | PA7/6, PB5/6 |
| GTIM1 | CH3 | PA5/6, PB0/6, PB6/6, PB8/6 |
| GTIM1 | CH4 | PB1/6, PB7/6, PB9/6 |
| GTIM2 | CH1 | PA0/6, PA15/5, PB14/6 |
| GTIM2 | CH2 | PA1/6, PA11/6, PB3/5, PB15/6 |
| GTIM2 | CH3 | PA2/6, PA12/6, PB6/5, PB10/6 |
| GTIM2 | CH4 | PA3/6, PB7/5, PB11/6 |
| GTIM3 | CH1 | PA4/8, PA15/8 |
| GTIM3 | CH2 | PA5/8, PB3/8 |
| GTIM3 | CH3 | PA6/8, PB4/8 |
| GTIM3 | CH4 | PA7/8, PB5/8 |
| GTIM4 | CH1 | PA9/9, PC14/9 |
| GTIM4 | CH2 | PA10/9, PC15/9 |
| GTIM4 | CH3 | PA11/9, PF0/9 |
| GTIM4 | CH4 | PA12/9, PF1/9 |

## Evidence and constraints

- F030 RM2.5 Table9-2 printed pp146-147 (PDF pages147-148) and DS1.9 Tables5-3/5-4/5-5/5-6 pp28-29 agree on every route. The RM AF tables were rendered and inspected. Section14.3.4.1 / Table14-6 p229 explicitly covers GTIMx_CHy with x,y=1..4 as compare/PWM output. CMMR CCyM=0xE yields high for CNT>=CCR; 0xF yields high for CNT<CCR. Do not substitute TOGP/TOGN outputs or use L012 CCER semantics. SYSCTRL_GTIMxCAP is capture-source selection (pp89-90, p228), not output remapping.
- L012 RM1.4 Table9-2 printed pp128-129 (PDF pages154-155) was rendered and inspected. All 44 selected routes were extracted geometrically and exactly compared with SDK1.0.5 cw32l012_gpio.h AF macros. The JSON records each macro and header line. GTIM section16.3.4/Table16-9 p262, section16.3.4.3/Table16-10 p264, and external-output example section16.8.8 p282 establish output behavior. Set CCyS=0 while CCyE=0; configure OCyM=6/7, CCyP and CCyE; CCyNP must remain zero in output mode (pp302-303). TISEL is capture-input selection.
- L012 ATIM CH4 has PA3/AF7, PA11/AF7 and PB9/AF5. Section17.3.1.10 p327 explicitly establishes CH1-6 independent outputs; section17.3.4 p341 describes external CHy/CHyN. CH4 output requires CC4S=0, appropriate OC4M/OC4MH and CC4E; output remains subject to BDTR.MOE, break/idle configuration and CC4NE (pp391, 394-395, 402).
- Both chips: GPIO ANALOG=0 and DIR=0 for output, stated AFR selected. See F030 sections9.4.1/9.4.4 p150 (also unlock GPIO lock), L012 sections9.4.1/9.4.4 p131. Pin compatibility does not establish safe electrical wiring. Oscillator-shared PC14/PC15/PF0/PF1 require board/clock configuration review; this audit does not silently reclaim an active oscillator.

## Exceptions and boundaries

- F030 ATIM CH4 is explicitly internal-only, no external pin, compare-only, not capture: RM2.5 section15.3.1.6 p264. Thus no F030 CH4 pin route is exposed.
- L012 CH4N is separately evidenced at PA2/AF7 and PB8/AF5 but is complementary, outside the requested main-output list. CH5/6, triggers, ETR and F030 toggle outputs remain out of this bounded list. Their omission is not a lack-of-hardware assertion.
- L012 Table9-1 stops its enumeration at AF7, but Table9-2 explicitly names AF8/AF9 routes and the SDK macros independently write 8/9 into four-bit AFR fields. These are verified, not inferred.

## Source integration

The complete per-route evidence is maintained in `cw32-data/sources/timer-pwm-routes.yaml`; chip YAML contains the concise pin/peripheral/signal/AF/remap records. Safe timer pin traits deliberately omit PA13/PA14 until a reviewed SWD-release ownership API exists. Route facts remain available in metadata; no SYSCTRL capture remap is performed as a substitute for output AF selection.

## Official source links

- f030_rm: https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf; SHA-256 1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2
- f030_ds: https://www.whxy.com/uploads/files/20251229/CW32F030_DataSheet_CN_V1.9.pdf; SHA-256 04ef91434320e5d05a3b6690fead0fedb31a7d9bad28c96b655a6e13a22e46b2
- l012_rm: https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf; SHA-256 a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340
- l012_gpio: SDK https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip; Libraries/inc/cw32l012_gpio.h header SHA-256 45f356e8028475c2e25d135c5fdd0b75babb6b1c83f1d5534038e9355822d717
