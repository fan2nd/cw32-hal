# F030/A030 owned ADC external reference

`Adc::new_with_external_reference` and `new_async_with_external_reference` consume
an ExRef pin and select `Reference::External` (CR0.REF=2). The sealed pin bound is
generated only for bonded PB0 on F030/A030, from the selected package's existing
analog route. The generator uses the selected family and bonded safe GPIO inventory.
[Other classic families](adc-classic-external-reference.md) have separate
own-source qualification, including F002/F003's different PB4 route.

The pin is configured as analog after software configuration/timing validation,
before ADC initialization. The owner retains it through reads, scans, cancellation,
and reference changes. Ordinary constructors reject External without a pin;
`set_config` likewise rejects it before stopping or changing ADC configuration.
An external-reference owner may switch to VDDA/internal references and back while
retaining PB0. Safe code cannot simultaneously use the same pin as ADC input or
GPIO. Tokens are consumed on constructor failure; reborrow when reuse is needed.
A clock/reset/inherited-state failure can follow GPIO preparation, leaving the
pin in the normal Flex teardown state.

External-reference timing uses the documented supply-reference voltage bands.
The caller declares the guaranteed minimum VDDA, not the ExRef voltage, in
`Config::vdda_mv`. No guessed ExRef startup delay or voltage measurement is added.
The board must supply a stable reference within its electrical ratings and keep
all sampled inputs (including internal sources) within 0..Vref. Noise, reference
source impedance, settling and absolute accuracy require board qualification.

External selection alone does not enable BGR; internal reference/temperature/
bandgap channels still request it as before. Inherited BGR remains preserved.
Shutdown/cancellation use existing trigger masking, stop/disable and flag cleanup;
async cleanup reverts REF to VDDA, and the next read reinitializes the configured
reference. Drop runs ADC cleanup before dropping the stored reference pin. No
new DMA, trigger, autonomous conversion or runtime source-settling guarantee is
introduced. Terminal cleanup faults retain their existing reset requirement.

Source evidence:

- CW32x030 UM CN2.5, SHA-256
  `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`:
  §22.2 p433/PDF434 identifies ExRef/PB00; §22.4.2 table22-3 p436/PDF437
  shares VDDA/ExRef clock bands; §22.5.1 p440/PDF441 requires analog configuration
  before external selection; §22.13.1 p460/PDF461 defines REF=2.
- F030 DS CN1.9 table5-1 pp23–25 and A030 DS CN1.1 table5-1 pp21–22 are pinned
  in `cw32-data/af/x030-analog.yaml`; package bonding is filtered before trait generation.
- Existing GPIO Flex analog setup/teardown and ADC lifecycle remain the owners
  of their register programming; no external reference is inferred on L010/L011/L012.

The `external_reference` binary in `examples/shared/classic-adc-scan` is restricted to
F030C8T7. It reads PA0 against a board-provided PB0 reference and demonstrates
reconfiguration without releasing the reference pin. No compilation, generation,
tests or board run was performed. Package gating, invalid-reference rejection,
borrow exclusion, read/scan reuse, cancellation and failure cleanup need acceptance
when the local execution restriction is lifted.
