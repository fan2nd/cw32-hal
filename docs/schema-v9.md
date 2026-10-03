# Schema 9: OPA internal source connections

Version 9 adds optional `opa` connections to a peripheral instance. It records
the internal DAC source of an OPA independently of the reusable register IP.
Version 8 JSON must be regenerated; it is rejected by the PAC stage.

```yaml
- name: OPA1
  opa:
    dac:
      peripheral: DAC
      channel: 1
    source: CW32L012 RM1.4 section 29.3.1, Figure 29-1, printed p646
```

`channel` is the documented one-based DAC channel number. The maintained L012
data records DAC channel 1 → OPA1 and channel 2 → OPA2 from Figure 29-1's
`DAC_OUTx` connection to `OPAx.INP4`. The existing comparator connections continue
to record DAC1 → VC1/VC3 and DAC2 → VC2/VC4 separately. An omitted connection is
unmodeled; it never licenses a guessed connection or an independent owner.

Validation requires an OPA source, nonempty evidence, a distinct existing DAC
target and the modeled output/holding register for the specified channel.
The connection is serialized into chip JSON and rendered into PAC metadata.
HAL generation consumes that metadata to emit sealed DAC-source pairing traits;
it does not infer the pairing by taking an instance number modulo two.

The OPA output → ADC connection needs no duplicate topology field: it is the
intersection of the existing OPA `OUT` and ADC `INn` routes on the same physical
pad. The current OPA owner erases its output pin type, so the generator requires
one unambiguous output pad per supported OPA instance before emitting an
instance-level ADC channel trait. PB0 gives OPA1 → ADC1 channel 8 / ADC2 channel 3;
PB1 gives OPA2 → ADC1 channel 9 / ADC2 channel 4. The emitted capability retains
the OPA borrow; it does not construct another GPIO token.

The single `cw32-gen` crate still performs YAML → persisted normalized JSON →
re-read JSON → PAC. Register layouts, defaults, access semantics and DMA topology
are unchanged by this schema addition.

Source: [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
§29.3.1 Figure 29-1 and §29.3.2 Table 29-1, printed p646 (PDF p672);
§29.4, printed p647, explicitly describes ADC sampling of the OPA output.
