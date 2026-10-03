# Resource composition: first implementation stage

This is the historical v0.17 stage record. The subsequent [v0.18 clock/PWM/PAC stage](clock-pwm-pac-v0.18.0.md)
implements the next resource and data changes. The [v0.19 bus/CRC/IWDT stage](buses-v0.19.0.md)
records the current remaining scope; the list below describes what was open at v0.17.

This release addresses the first three concrete gaps from the v0.16.0 review,
plus reusable mutable DMA copy sources. It does not complete the entire remaining
HAL roadmap or establish physical board behavior. The exact checks are recorded
in [validation](validation-v0.17.0.md).

## Implemented resource paths

- **OPA → ADC:** `Opa::output()` produces a non-cloneable, lifetime-bound ADC
  source guard after checking enable/calibration/settling state. The four ADC
  route implementations come from matching real shared pads in metadata. ADC
  setup does not recreate a GPIO token or reconfigure the OPA output pin.
- **Independent DAC channels:** `Dac::split()` transfers the two channel owners.
  Each optional output pin and each channel's enable/route teardown remain
  independent. Shared register RMW uses critical sections and preserves the
  sibling. `channel.source()` reserves configuration and permits `source.set()`
  while OPA/VC retains its dependency. Only sourced physical pairings compile.
- **Finite ADC DMA:** a static-input `Sequence::read_dma()` consumes the sequence
  and a static `u32` destination. Its DMA channel already carries the real IRQ
  Binding in Async mode. L012 uses one EOS request and BULK to copy 1–8 native
  result slots. F030 supports one MODE0 conversion and rejects multi-slot DMA
  before start. Successful TC returns the sequence and destination; error,
  cancellation and timeout quarantine them. Persistent per-ADC/channel state
  also protects against forgotten guards and owner reconstruction.
- **Mutable copy sources:** `dma::Channel::copy_mut()` consumes static mutable
  source/destination buffers and returns both on clean completion or launch
  rejection. Failure/cancellation preserves the existing quarantine rule.

See [analog APIs](analog-resources.md), [DMA APIs and limitations](dma.md),
[hardware evidence](dma-hardware-evidence.md), and [schema9](schema-v9.md).
The user-requested unsafe motor operations remain available for the serialized
board domain. These new safe composition APIs do not change its interrupt
executor, sampling, commutation, timing parameters or fault algorithm.

The upstream comparison uses Embassy commit
[`b12a6d9efcd2711037abca1b63a661a9ef726444`](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444):
[OPA output channels](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/opamp.rs#L580-L595),
[DAC splitting](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dac/mod.rs#L1021-L1035),
and [ADC DMA composition](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc/mod.rs#L984-L1026).
CW does not have the same documented DMA abort/drain guarantee, so this release
aligns the resource relationship without adopting upstream's borrowed-buffer
cancellation promise.

## Remaining approved stages

These are open work, not claims that CW hardware lacks the feature:

1. RCC: extend sourced clock-tree configuration and per-peripheral frequency/
   shared-gate resource semantics. Shared gates require lifetime/reference
   accounting before driver Drop can turn them off. STOP/time recovery remains
   separate from the currently fixed running PCLK contract.
2. Independent PWM channel ownership: define split-owner lifetime, shared MOE,
   critical-section register updates and the full-scale duty endpoint policy.
   Current `SimplePwmChannel` still borrows the whole owner.
3. PAC semantic enums: put evidenced mode/selector values into actual data,
   including reserved-value handling. Counts, addresses and payloads remain
   integers. Current data still has no semantic enum entries.
4. Repeated field arrays: express actual regular/irregular bit positions, then
   migrate DMA flags and timer channel controls. Existing register/subblock
   arrays remain; heterogeneous fields should retain distinct semantics.
5. Additional real CW HAL modules: UART/SPI/I2C, CRC/Flash/RTC/watchdogs, ADC
   internal sources and timer capture/encoder are still unimplemented. Each
   module requires a bounded hardware/ownership/IRQ design and validation before
   being called supported. F030 internal ADC sources additionally require BUF=1
   and single-channel single-shot operation.

The order groups the remaining eight-gap audit into releases; it does not
silently remove those items or promise all STM32 features on CW32. General
peripheral endpoints, safe continuous DMA/ring buffers and low-power operation
are not supplied by this first resource-composition release.
