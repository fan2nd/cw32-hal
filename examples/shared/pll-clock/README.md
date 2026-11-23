# HSI- and HSE-fed PLL examples

Normal standalone ARM firmware using the HAL's one-time clock ownership, UART1 TX on PA8 and a GPIO activity output on PB0. Connect a UART receiver that tolerates the board's I/O voltage and an appropriate PB0 LED circuit. The unchanged HSI defaults require no external oscillator. Use one exact package feature; the three F020/F030/A030 selections below and all five modeled L083 packages expose these pads.

`pll-uart` on L083 uses factory HSI /6 ×7, nominal 56 MHz SYSCLK/HCLK/PCLK with 54.88–57.12 MHz rate bounds. The declared board envelope is 3.0–3.6 V and −20…70°C. `fractional` selects HSI /10 ×12, nominal 57.6 MHz with 56.448–58.752 MHz bounds. `low-voltage` declares 1.65–1.79 V and adds AHB /4, giving nominal 14 MHz (or 14.4 MHz with fractional); the independent raw PLL limit still applies. Do not choose a declaration the actual board cannot guarantee.

For `cw32f020c6u7`, `cw32f030c8t7` and `cw32a030c8t7`, the default is factory HSI /6 ×4, nominal 32 MHz. F020 has 30.4–33.6 MHz bounds; F030/A030 have 31.36–32.64 MHz bounds. `fractional` selects /10 ×9, nominal 43.2 MHz: F020 41.04–45.36 MHz, F030/A030 42.336–44.064 MHz. `low-voltage` keeps the raw PLL configuration and adds AHB /4, giving nominal 8 MHz (or 10.8 MHz fractional). Flash latency uses the actual HCLK maximum. The F020 raw PLL cap is 48 MHz; downstream division cannot legalize an excessive raw output.

## HSE crystal and bypass choices

Select `hse-crystal` or `hse-bypass` with any one of the supported package features. Both use `PllSource::HSE` and the existing `Config.hse` declaration: nominal 8 MHz, actual 7,999,600–8,000,400 Hz (±50 ppm), multiplied by four. SYSCLK/HCLK/PCLK are nominal 32 MHz with rate bounds 31,998,400–32,001,600 Hz. The reference fits the 6–12 MHz input bin and the result fits the 24–36 MHz output bin on all four families. `low-voltage` adds AHB /4 for nominal 8 MHz HCLK/PCLK with bounds 7,999,600–8,000,400 Hz; the raw PLL remains 32 MHz. Retained HSI stays at /6 and is checked independently. `fractional` is HSI-only; combining it with either HSE feature, or selecting both HSE modes, is a compile-time error.

These are required board declarations, not measurements or a component recommendation. Qualify the complete source interval across tolerance, temperature, aging, loading, source supply and short-term cycle variation throughout the declared −20…70°C and 3.0–3.6 V envelope, or 1.65–1.79 V with `low-voltage`. VDDA must equal VDD. Do not select low voltage unless the actual oscillator and board meet the same frequency and waveform contract there.

- `hse-crystal`: connect a suitable 8 MHz resonator and qualified load network to PF0/OSC_IN and PF1/OSC_OUT. Qualify the board layout, resonator characteristics, startup and the example's `HseDrive::Level2`; adapt the drive and declaration if the hardware requires it. Both pads are reserved. Crystal-to-PLL admission follows the vendor-documented internal composition. The PLL's 40–60% input-duty condition is not waived, but neither this declaration nor STABLE independently certifies hidden internal duty, and no extra duty assertion is supplied. The typical 2 ms crystal startup is not a maximum.
- `hse-bypass`: provide a stable continuous 8 MHz digital source on PF0/OSC_IN before initialization. At that pin, require 40–60% duty, high 0.7×VDDIOx…VDDIOx, low VSS…0.3×VDDIOx, each high/low pulse at least 15 ns and each rise/fall at most 20 ns, with all applicable I/O limits met together. Qualify startup, loading and the external source's supply. PF0 is reserved; on L083, an inherited crystal can retain PF1's boot reservation too. Bypass voltage/edge rules do not apply to the analog crystal mode.

Both choices keep HSE filtering disabled and the longest HSE/PLL startup-count settings. The finite poll budget is not a wall-time deadline, and STABLE does not measure duty or continuous lock. The source and board conditions must continue to satisfy their contract while these frozen rates are used. Neither mode adds guaranteed fallback, surviving CPU progress or valid PLL rates after reference loss.

## Embassy time and builds

`pll-time`, enabled by `embassy-time`, drives the same output through Embassy timers and owns GTIM1. The L083 default 56 MHz and low-voltage 14 MHz choices exactly divide to nominal 1 MHz. Classic F020/F030/A030 GTIM requires a power-of-two prescaler, so their 32 MHz and low-voltage 8 MHz defaults satisfy that separate constraint. Fractional choices fail the existing exact time-driver admission before acquiring the device. Tick bounds represent clock-rate qualification, not precision timekeeping or a cycle-jitter guarantee.

Both HSE modes also satisfy nominal 1 MHz divisibility: divide 32 normally, or divide 8 with `low-voltage`. The source tolerance and PLL rate-only qualification still apply; a nominal tick is not a guaranteed microsecond.

Example commands, after `./d gen-all` from the repository root:

- `cargo build --locked --manifest-path examples/shared/pll-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083rct6 --bin pll-uart`
- Select `cw32f020c6u7`, `cw32f030c8t7` or `cw32a030c8t7` in place of the L083 feature for the classic examples.
- Add `,fractional` or `,low-voltage` for the other UART clock configurations.
- Select `cw32l083rct6,embassy-time` with `--bin pll-time` for Embassy timing.
- Add `,hse-crystal` or `,hse-bypass` to select the matching physical HSE board source, optionally with `,low-voltage`.
- For example: `cargo build --locked --manifest-path examples/shared/pll-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7,hse-crystal,embassy-time --bin pll-time`.

Initialization requires stable, legal incoming clocks/Flash, quiescent DMA/peripheral/interrupt/MCO/PLL_OUT consumers and continuous availability of the active source. It may stop an inherited PLL. On failure, reset before retrying; no requested clocks are published. This example does not enter DeepSleep or perform runtime retuning. Every PLL output and subsequent division remains rate-only, including both HSE choices. Classic ADC strict acquisition/duration APIs reject the PLL rate-only source before ADC/RCC writes. F030/A030 complementary PWM also rejects it before timing synthesis or ATIM/RCC enable, including zero dead time; previously constructed pin wrappers still retain their normal GPIO initialization/drop effects. Direct HSI/HSE ADC behavior is unchanged. See [L083 qualification](../../../docs/l083-hsi-pll.md) and [F020/x030 qualification](../../../docs/f020-x030-hsi-pll.md) for original source pages and limitations.

Compilation and strong linked ARM ELF symbols can verify source/build integration only. These examples were not executed on silicon, and the declarations do not electrically qualify any board.
