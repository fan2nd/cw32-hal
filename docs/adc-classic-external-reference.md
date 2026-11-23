# Classic ADC external reference qualification

The owned ExRef constructors support every classic ADC family, using each
family's own pin and REF definitions. They retain the same lifetime, validation,
reconfiguration and cancellation contract as [x030](adc-x030-external-reference.md).

| Family | ExRef pad | Own manual feature / voltage bands / REF register (printed; PDF +1) |
| --- | --- | --- |
| F002 CN1.4 | PB4 | §19.2 p300; table19-3 p303; §19.12.1 p326 |
| F003 CN2.3 | PB4 | §20.2 p360; table20-3 p363; §20.13.1 p387 |
| F020 CN1.4 | PB0 | §21.2 p371; table21-3 p374; §21.13.1 p398 |
| L031 CN1.6 | PB0 | p430; table22-3 p433; REF p458 |
| L052 CN1.5 | PB0 | p467; table23-3 p470; REF p496 |
| L083 CN2.0 | PB0 | p472; table23-3 p475; REF p500 |
| R031 CN1.3 | PB0 | p433; table22-3 p436; REF p460 |
| W031 CN1.4 | PB0 | p433; table22-3 p436; REF p461 |

The downloaded own manuals match the SHA-256 values in
`sources/evidence-sources.json`:

- F002: `e453b3f82d9d180a86cd5f7cb18d858d7f228afc8101d87c700ca9d5c02d5add`
- F003: `0fa58dac223add7f2ac1ee714a7df7db4e414e0f193601b80dda96a948bfc738`
- F020: `279521eaf7d32b241e614a9553044a4d99c1e7ca2566efa3b0fdd2b66c106eed`
- L031: `4288cfd97b56385059c5a283f69972047af4773ef8bbc4d8b51d8155fb17a760`
- L052: `4bac53df4db69a3b76c833cb14dd45b0e5c7f9884506c83a5cda6ea00a859f41`
- L083: `9930bf1755f3bbf8933163c2d0da57fd9a4f3250a358a4c0bfc75ed4eda3a0a3`
- R031: `fbee9b6942be9fa09f00c946705644d5356c4249f3e2280e3dfe5f0cb342eddb`
- W031: `b6973677946a9332b0e5b3e954119768aa40469d44140a73e18648e9419bedc9`

All listed manuals define REF=2 for ExRef, but their pads are not interchangeable.
F002/F003 PB4 is not an ADC conversion input route: no fictitious IN/mux entry
is added. The build script derives the sealed reference trait from the selected
family and bonded, safe GPIO inventory. Other pads and absent package pins
receive no trait. F002 PB4 is TSSOP20 pin12/QFN20 pin9 in the existing own
package metadata. F020 PB0 retains its separately qualified input route, but
reference ownership prevents concurrent use of that pad as a conversion input.

F002 exposes only supply and External references. Internal1V5/Internal2V5,
Temperature and VrefInt stay absent, and CR0 reserved bits5:4 are not treated as
TSEN/BGREN. Its own table explicitly shares VDD/ExRef clock bands, so the existing
supply bounds apply without inventing an internal-reference headroom rule.
F003/F020 retain their own internal-reference and BGR behavior. ExRef itself
never requests BGR; an independently requested internal channel still may.

`external_reference_f002` in `examples/shared/classic-adc-scan` uses F002F3P7 PB4 as
reference and PA0 as input. It declares a board minimum 3.3 V VDD and requires
a settled external reference, with input bounded by that reference. The example
switches to supply and back while keeping PB4 owned. It performs no calibration
or physical-voltage conversion. Other-family examples may use the same APIs
with their verified package pads.

No local compilation, tests, generation or board execution was performed.
Per-family availability, wrong/unbonded-pin rejection, borrowed-token exclusion,
F002 reserved-bit preservation, startup failure and cleanup remain pending
acceptance. L/R/W retain their own supply bounds and VDDA=VDD obligation. R031
starts at 2.2 V; W031's ADC supply policy does not qualify RF DCDC operation.
No shared BGR or ADC clock reset is introduced. Native L010/L011/L012 keep their
supply-only reference policy; their bandgap measurement inputs are not ExRef.
