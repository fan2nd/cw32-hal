# EAU instance ownership

EAU now follows the generic peripheral-instance structure used by the pinned
Embassy [CORDIC driver](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/cordic/mod.rs).
The owner is `Eau<'d, T: Instance>` and its constructor consumes `Peri<'d, T>`.
The public `Instance` trait is sealed and requires the real peripheral-token and
generated clock traits. Register access resolves through `T::regs()` and clock
acquisition through `T::acquire()`. There is no concrete-type default, public
adapter, dynamic dispatch, or separate software accelerator implementation.

Previously, `Eau<'d>` stored `Peri<'d, peripherals::EAU>`, acquired that concrete
token's clock, and accessed `pac::EAU` directly. Ordinary inferred construction
still reads `let eau = Eau::new(p.EAU);`. Explicit owner types now name their
instance, for example `Eau<'static, peripherals::EAU>`. Generic callers can use
`fn accelerator<'d, T: eau::Instance>(p: Peri<'d, T>) -> Eau<'d, T>` and return
`Eau::new(p)`. `release()` returns the same `Peri<'d, T>` that construction owned.

## Generated identity

`build.rs` writes `_generated_eau.rs` from the selected PAC metadata's
standalone peripherals whose block kind is `eau`. It checks the supported `l012`
IP version and implements the private register association and public `Instance`
for each actual peripheral name. The current L012 metadata contributes exactly
`EAU`; the F030 metadata contributes none and does not enable the EAU module.
The existing clock generator associates the token with its actual AHB gate and
reset resource. The driver does not invent another instance or infer a clock
from a chip-family name.

## Preserved hardware behavior

This change only selects the peripheral through its generated instance type.
Division still writes CSR, DIVIDEND, then DIVISOR; square root still starts on
the DIVIDEND write and never writes DIVISOR. All operands and results use the
same typed 32-bit PAC accesses. Signed division retains truncation toward zero
and signed remainder conversion. The zero-divisor and signed-overflow checks,
zero poll-budget rejection, initial BUSY rejection, finite status polling,
hardware ZERO/OVR errors, and result-read ordering are unchanged.

Timeout still leaves a potentially running operation with its owner. `reset()`
still uses the retained clock guard's exclusive-resource check, and `release()`
still attempts that reset before returning the token. The counted clock guard
still gates the accelerator only when its final owner drops. EAU has no IRQ,
async cancellation path, or DMA operation; none is added by the instance trait.

## Related constructor audit

The same pinned upstream revision has generic CORDIC and generic DAC
constructors, but its [CRC v1](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/crc/v1.rs)
and [CRC v2/v3](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/crc/v2v3.rs)
constructors consume a concrete CRC token. A single physical instance therefore
does not by itself require or prohibit a generic owner. EAU's instance boundary
here is an explicit API choice grounded in generated hardware identity.

The read-only audit identified these related constructor boundaries:

- CW32 CORDIC previously used concrete CORDIC tokens, registers and IRQ identity
  in both blocking and async owners. The accompanying CORDIC change carries
  the instance through its interrupt handler, static completion state and
  cancellation guard as well as its constructor.
- CW32 DAC previously consumed the concrete DAC token and retained that
  identity in its split channel owners, output-pin bounds and borrowed source
  guards. The accompanying DAC change carries its generated instance through
  those resources and their internal connections. The
  upstream [DAC](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dac/mod.rs)
  constructors accept `T: Instance`.
- BGR consumes a concrete BGR token and acts as a shared analog startup witness.
  It has no peripheral gate or reset; a generic version must not add a clock
  requirement that the hardware cannot satisfy.
- ADC, OPA, comparator and comparator-reference owners already consume generic
  `Peri` tokens constrained by sealed, metadata-generated instance traits.
  Their registers and clocks resolve through those instances. The comparator
  reference associations and DAC consumer routes retain their actual topology.

These observations are constructor/identity findings, not a claim of complete
upstream API parity. Board-level arithmetic, reset timing and power behavior
remain unverified.

## Focused verification

Rust 1.99.0 release builds for the real `thumbv6m-none-eabi` target pass for
L012 with all optional HAL features and F030 with minimum features. L012
full-feature rustdoc also passes with `-D warnings`. External
ARM compilation accepts generic and concrete EAU construction, all arithmetic
methods, reset, release, and token reuse after an ended reborrow. Five negative
probes reject an externally forged instance, a CORDIC token, a moved token,
overlapping token reborrows, and use of an owner after release. F030 rejects an
EAU-module import.

An external Rust register-model executable exercises ten protocol/MMIO groups
using the production operation logic, production generic hardware backend and
actual generated PAC register types. It checks trigger order, status/error
paths, bounded polling, signed word forwarding, square-root operand access,
and full-width register reads/writes with adjacent sentinel words. The
operation/error/polling implementation also compares byte-for-byte with its
pre-refactor source. These checks validate software protocol and ownership;
they do not emulate the accelerator's arithmetic or prove physical timing.
No validation executable or probe scaffolding is added to the source package.
