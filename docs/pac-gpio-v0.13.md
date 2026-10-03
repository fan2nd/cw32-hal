# v0.13.0 PAC and GPIO access

This document describes PAC/indexed registers and GPIO. ADC/comparator owner/mode/channel APIs and GPIO interrupt waits are implemented and described in [async contracts](async-api.md); generic timer/PWM is described in [timer-pwm.md](timer-pwm.md). Compilation alone is not evidence of API or hardware parity.

## Actual upstream shape, CW32 register facts

Reference: chiptool [`common.rs`](https://github.com/embassy-rs/chiptool/blob/bcf538a2e7b8584ae874ee9ab72efb1576fc6152/src/generate/common.rs), its [IR](https://github.com/embassy-rs/chiptool/blob/bcf538a2e7b8584ae874ee9ab72efb1576fc6152/src/ir.rs), and pinned stm32-metapac [GPIO](https://github.com/embassy-rs/stm32-data-generated/blob/e463add8cc54375f61c6f5f83d6b589e7fc68be2/stm32-metapac/src/peripherals/gpio_v2.rs).

The namespace is `pac::GPIOA`, not an added `peripher` wrapper. CW32 uses its real `PIN`, `BSS`, `BRR` fields and BSRR offset `0x5c`; no STM32 offsets or invented bit-proxy API are substituted.

```rust
pac::GPIOA.odr().modify(|w| w.set_pin(5, false));
let high = pac::GPIOA.idr().read().pin(5);
pac::GPIOA.bsrr().write(|w| w.set_bss(5, true)); // set pin5 high
pac::GPIOA.bsrr().write(|w| w.set_brr(5, true)); // reset pin5 low
```

BSRR commands are not read-modify-write. A false BSS bit performs no action. Ordinary direction changes can use `dir().modify`; unsafe clock/pin sharing and reserved-bit semantics are still the caller's responsibility when using PAC directly.

```rust
let sample = pac::ADC1.result(3).read().result();
pac::ATIM.ccr(2).write(|w| w.set_ccr(1200));
pac::ATIM.ccmr_cmp(1).modify(|w| w.set_ocpe(0, true));
pac::DMA.ch(0).cnt().write(|w| w.set_cnt(4));
pac::GPIOA.afr(0).modify(|w| w.set_afr(5, 0));
```

These examples show L012 access shape, not complete peripheral initialization. Index0 maps the first documented hardware member. ATIM `ccr(0..3)` covers CCR1–4; `ccr_group(0..1)` covers the different complete CCR5/6 fieldset. `ccmr_cmp(2)` reaches offset0x50. GPIO `afr(0)` is AFRL at0x18 and `afr(1)` is AFRH at0x14. DMA channel accesses reuse the actual channel layout and do not confer independent ownership.

`write_value` accepts the fieldset, for example `pac::ADC1.cr().write_value(pac::adc::regs::Cr(0x100))`. `read()` already returns that fieldset. Legacy `fields::...write(word, value)`, raw free MMIO helpers, numeric `write_value` and `read_value`/`modify_value` shims were removed. HAL and board examples use the new surface.

## Defaults stay register-accurate

ADC.CR's known reset0x100 still contains reserved bit8. Raw typed writes never OR reset bits back on. Field setters can clear ordinary reset-one bits deliberately. Unknown defaults and disputed persistent security/oscillator/GPIO values remain unknown.

F030 GTIM `ccr(n)` seeds `[0xffff,0,0xffff,0]`; L012 RTC `alarm(n)` seeds `[0x02121000,0x04120000]`. This is selected by the register handle's index, not a false shared fieldset Default. All original reset evidence survives in array element metadata. See [reset evidence](register-reset-defaults.md).

## Real GPIO drivers

The fixed [Embassy GPIO source](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/gpio.rs) is the structural reference.

`Flex` owns `Peri<AnyPin>` and implements real mode transitions. `Input`, `Output` and `OutputOpenDrain` delegate to it. Inherent APIs include `is_high/is_low/level`, `set_high/set_low/set_level/toggle`, `is_set_high/is_set_low/output_level`, plus embedded-hal1.0 adapters. The inherent input/latch queries return bool; callers no longer append Result handling such as `.unwrap_or(false)`.

```rust
// L012 has no programmable output-speed register.
let mut led = gpio::Output::new(p.PC13, gpio::Level::High);
led.set_low();
let key = gpio::Input::new(p.PA3, gpio::Pull::Up);
let pressed = key.is_low();
let mut pin = gpio::Flex::new(p.PA4);
pin.set_as_input(gpio::Pull::None);
```

- Both chips support real open-drain through OPENDRAIN.
- F030 exposes the real Low/High SPEED values and a separate drive-strength setting. SPEED0=Low/1=High; DRIVER0=High/1=Low. Mode changes preserve DRIVER unless explicitly changed.
- L012 has no `Speed` type/argument. `Pull::Down` is accepted only for PF03; unsupported pins reject it before MMIO. F030 uses its actual pull-down capability mask.
- Output latches are preloaded before enabling output direction. Drop disconnects only the owned pin to floating digital input, preserving its latch and shared clock. Explicit `release` transfers the token while retaining the current mode/latch, preserving the prior ownership-transfer behavior.
- Pin identity/erasure uses upstream `Peri::into`, sealed `Pin::pin/port`, and encoded unsafe `AnyPin::steal`; there is no invented `degrade` facade.
- CW GPIO interrupt topology is per-port. `InterruptInput` implements real `embedded_hal_async::digital::Wait` with pin-specific Binding/state and shared-vector isolation, without fake STM32 EXTI channel tokens.

RCC `Clocks` fields are no longer publicly constructible; public getters return a verified initialization snapshot. Runtime clock reconfiguration is not supported by this HAL. ADC construction uses the validated RCC frequency internally; ADC/comparator owners use sealed blocking/async modes.
