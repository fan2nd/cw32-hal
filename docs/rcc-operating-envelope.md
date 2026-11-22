> Historical software-verification references: HAL tests and fixture harnesses cited below were deleted on 2026-10-08. Their reported results apply to the dated source snapshot only; source facts remain unchanged. See [current HAL layout and build scope](hal-production-layout.md).

# Declared RCC operating envelope and actual-clock Flash latency

This correction builds on the source-qualified `ClockBounds` implementation. It validates the board declaration before hardware access and chooses final Flash latency from the actual upper HCLK bound. Existing RTC/AWT retained-HSI protection, temporary-clock guards, factory calibration, ready/readback polling and failure cleanup are preserved. No hardware execution is claimed.

## Board declaration

Every RCC backend's non-exhaustive `Config` gains `operating_conditions: rcc::OperatingConditions`:

- `supply_mv`: board VDD in millivolts.
- `min_temperature_c`, `max_temperature_c`: guaranteed ambient TA interval in Celsius, not junction temperature.

`Config::new()` and `Config::default()` remain available. The field defaults to the selected family's lowest factory-HSI-qualified supply voltage and full qualified ambient interval. Existing slow default clock configurations remain valid. A board using faster clocks should set its supply voltage, for example `supply_mv = 3_300` for a 3.3 V rail. The rail must sustain the voltage required by the selected clocks throughout use.

These values are assertions by the board/application, not measurements. The HAL cannot detect a board outside its declaration. Other thermal, analog-supply, regulator-mode, source, peripheral and pin requirements remain the caller's responsibility. In particular, W031 RF DCDC operation separately needs at least 2.0 V; the source's 1.8 V floor does not waive it.

Before MMIO, `Config::frequencies()` and initialization reject reversed temperature intervals, supply values outside `HSI_BOUND_SUPPLY_MV`, and ambient ranges outside `HSI_BOUND_TEMPERATURE_C`. This includes unsupported hot tails, even where general device operating tables allow a hotter low-dissipation condition. Declaring a narrower temperature range, including 25 C, deliberately does not reduce the full characterized HSI error envelope.

Additive errors are `InvalidTemperatureRange`, `SupplyOutsideQualifiedRange`, `TemperatureOutsideQualifiedRange`, `HclkTooHigh`, and `PclkTooHigh`. The public HAL `init()` already calls `frequencies()` before taking peripheral tokens; every RCC backend calls it before its first register read/write, including retained RTC preflight. Failed validation therefore does not consume HAL tokens or modify peripheral state.

## Actual HCLK and PCLK limits

The conservative ceiling comes from the declared supply voltage and each family's own datasheet Table 7-4:

| Families | Below 1.8 V | At least 1.8 V |
| --- | --- | --- |
| F030, A030, L083 | 24 MHz | 64 MHz |
| L011, L012 | 24 MHz | 96 MHz |
| F002, F003, F020, L031, L052, L010 | 24 MHz | 48 MHz |
| R031, W031 | Unsupported by their supply range | 48 MHz |

Both bus domains are checked using exact rational upper bounds, before whole-hertz display rounding. The current topology implies PCLK ≤ HCLK, but both checks are retained explicitly. A fast HSI/SysClk may feed legal divided buses. No independent SysClk electrical ceiling is invented from a bus-table limit.

A nominal 24 MHz HCLK below 1.8 V is rejected because the qualified positive error can exceed 24 MHz. At a sufficiently high declared voltage it is legal and requires WAIT1. Nominal 48 MHz on 48 MHz-rated families, and nominal 96 MHz on L011/L012, are rejected with AHB /1 under this conservative envelope policy. The source's /1 configuration remains supported with a suitable AHB divider.

This is a HAL worst-case admission policy, not a claim that the vendor's nominal maximum configuration is universally unusable or that every physical part reaches the error endpoint. No automatic clock modification or guessed extra tolerance is used. Applications needing another qualified source or a measured/calibrated narrower envelope require a separately supported configuration.

## Flash WAIT and transitions

Final WAIT is selected from the actual upper HCLK endpoint: WAIT0 through 24 MHz, WAIT1 through 48 MHz, WAIT2 through 72 MHz, and, on L011/L012, WAIT3 through 96 MHz. L010 exposes only agreed WAIT0/1; classic families use agreed WAIT0/1/2. Exact boundary-side arithmetic is preserved by rounding the upper frequency outward before this integer threshold selection.

At nominal 24 MHz, the source envelope reaches 24.48 MHz (25.2 MHz on F002/F020), so WAIT1 replaces the old WAIT0. At a legal nominal 48 MHz on F030/A030/L083, WAIT2 replaces WAIT1.

Initial conservative waits remain unchanged: classic 2, L010 1, L011/L012 3. Their documented ceilings cover every *legal actual incoming HCLK*. Initialization still requires a valid incoming source, available/stable clock, legal incoming bus frequencies and suitable existing Flash latency. It does not recover arbitrary malformed bootloader clock or asynchronous source-loss states.

L012's clock-chapter WAIT4 prose conflicts with its Flash table/register descriptions and 96 MHz rated bus ceiling. This HAL uses the agreed WAIT0..3 and rejects actual HCLK above 96 MHz. A raw oscillator tuning target of 100 MHz is not proof of legal 100 MHz HCLK. Existing divider guards and RTC/AWT HSI reservations are preserved.

## Sources and verification

`rcc-operating-envelope.json` records the independent source audit's 13 own-family bus tables, agreed Flash table/register values, 25 original PDF hashes and distinct printed/PDF page numbers. Its reviewed code hashes identify the preceding ADC-only baseline, not the corrected implementation. `verify_rcc_operating_envelope.py` rehashes the official sources, reads the relevant pages and checks runtime policy/validation placement. `adc-clock-source-bounds.json` remains the canonical factory-HSI accuracy authority.

`ci/check-rcc-operating-envelope.sh` runs all 13 host HAL suites with the accepted SPI/I2C implementation dependencies, ARM public-API controls, source verifiers and representative exact-package firmware links. Host protocol tests explicitly classify every supported divider combination at voltage boundaries as accepted or rejected; rejected configurations must produce the expected error with zero register access. Temperature tails/reversed ranges, nominal boundaries, Flash readbacks, retained RTC and prior failure-injection paths are covered.

SPI's production-RCC property test declares a qualified high-voltage board and counts rejected absolute-ceiling trees instead of blindly unwrapping every nominal tree. I2C's direct bounded-rate sweeps are labeled arithmetic-domain cases, including rates that RCC would reject as board configurations. No production SPI/I2C bytes change.

Runnable L012 and L052/L083 GPIO link fixtures now use explicit HSI /6, valid across the default declared supply range. ADC link fixtures retain their earlier qualified HSI /10 setup. The F030 firmware examples keep the valid default 8 MHz tree. Type-only API controls that deliberately inspect `Result` values do not promise successful board initialization.
