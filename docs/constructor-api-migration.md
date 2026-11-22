# Constructor API migration

Fallible peripheral constructors use `new`, `new_async`, `new_blocking`,
`new3` or `new_with_config` directly and return `Result`. The redundant
public `try_new*` names and corresponding panic-on-error convenience wrappers
are removed. Applications choose how to handle construction errors:

```rust
let adc = embassy_cw32::adc::Adc::new(
    p.ADC1, embassy_cw32::adc::Config::default(),
)?;
```

Replace `Type::try_new*(...)` with `Type::new*(...)`, preserving the existing
`?`, `match`, `unwrap` or other error handling. Previously infallible `new*`
calls need explicit error handling. The included bare-metal examples use
`unwrap()` and their existing panic handlers. ADC `new_blocking` remains a fallible alias of `Adc::new`; both return
`Result` for Blocking construction.
Private shared initialization is named `new_inner` where it is actually shared.

The affected APIs are ADC (including L012 and BTIM-triggered
ADC), SPI, I2C, timer/capture/QEI/PWM, timer triggers, CRC and watchdog
constructors. USART already uses fallible `new` and is unchanged. GPIO and
pin wrappers, DMA constructors, and non-constructor
`try_*` methods are unchanged. This is an API cleanup, not new peripheral
qualification, altered DMA ownership, changed UART buffering or hardware
validation.

Global HAL initialization now uses `init(config) -> Result<Peripherals, rcc::Error>`. Use `init(config)?` to propagate clock errors.

## Historical source evidence

Dated validation reports and hash-pinned source-audit documents retain their
original text and API names. In particular,
[the L012 ADC ownership audit](adc-l012-dual-ownership.md) is an immutable
input to electrical-data verification. Its historical `try_new` and
panic-wrapper examples do not describe the current constructor API; use the
migration above and the current examples instead. The hardware ownership and
acquisition evidence in that audit is unchanged.
