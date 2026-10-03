> Historical schema. Current analog-source topology is documented in [schema9](schema-v9.md), extending [schema8 DMA topology](schema-v8.md).

# Normalized schema v7: instance analog connections

Schema 7 retains the [indexed register model](schema-v5.md) and
[reset cross-effects](schema-v6.md). It adds sourced comparator connections to
`Peripheral`, rather than attaching chip-instance wiring to a reusable IP.

```yaml
comparator:
  reference: VC12REF
  dac:
    peripheral: DAC
    channel: 1
  source: CW32L012 RM1.4 sections 27.7.1–3, official source URL
```

`reference` and `dac` are optional; source evidence is required. Validation
requires a comparator owner, a real VCREF target for reference connections, and
a real DAC/channel for DAC connections. Structurally invalid, self/dangling, wrong-kind or nonexistent-channel
connections are rejected. Physical wiring still requires primary-source audit;
a schema validator cannot prove that a different structurally valid pairing
is the actual silicon connection. DAC channel validation uses the actual
modeled DOR and DHR12R channel elements rather than assuming a universal count.

The four L012 records describe VC1/2→VC12REF, VC3/4→VC34REF, VC1/3→DAC output1
and VC2/4→DAC output2. HAL `VcInstance::Reference` consumes that association;
it no longer infers reference pairing from VC names or IRQ grouping. F030's
shared internal divider/BGR does not gain a fictitious separate VCREF/DAC token.

Source names and generated namespaces/path keys are also validated before
output. Chip/family and Cortex-M0+/Thumb target consistency are checked, and
32-bit physical address bounds retain the valid final address without allowing
overflow. Descriptions, field kinds and enum values survive into Rust metadata.

The interface remains YAML → **persisted normalized JSON** → reload and validate
→ PAC/metadata inside one `cw32-gen` crate. Old normalized schema versions are
rejected and must be regenerated. No generated files, helper scripts or test
framework belong in the source archive.
