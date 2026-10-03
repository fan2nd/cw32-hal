# Normalized schema v6: reset cross-effects

Schema 6 retains the [schema 5 indexed register model](schema-v5.md), full reset
values/provenance, explicit aliases and ownership. It adds one physical
relationship to `Peripheral`: `reset_effects`.

```yaml
reset_effects:
- peripheral: VC1
  description: Resetting ADC clears the bandgap enable used by the comparator.
  source: An exact reference-manual section and official document URL.
```

Each effect names another existing peripheral, with nonempty description and
source. Repeated targets, self references, absent targets and effects without a
reset control are rejected. The normalized list is deterministically sorted.
Omitted source YAML lists default to empty; absence is not a proof that all
possible undocumented cross-effects have been ruled out.

This is different from two instances owning the same reset bit. The generated
`RegisterBit.shared` still represents shared-bit ownership. Metadata separately
preserves `Peripheral.reset_effects`; a HAL constructor may pulse a local reset
only when the bit is private **and** no cross-effects are declared.

The real F030 ADC reset affects VC1/VC2 reference circuitry through ADC_CR0.BGREN.
These effects are recorded in family YAML with RM2.5 §§4.7.17,22.13.1,23.3.1.
A reset therefore remains suppressed even if a chip/family name changes or the
same ADC/VC combination is used by another chip. There is no hardcoded family
exception in the HAL generator.

The path remains one `cw32-gen` crate: YAML → validated persisted JSON → reload
and validate JSON → PAC/metadata. Older normalized schemas are rejected and
must be regenerated. Source archives exclude generated artifacts.

HAL capability cfgs do not require duplicated booleans in YAML. Presence and
IP version are already modeled; GPIO speed, drive, level-IRQ and pull-down
layout are derived from their actual validated register fields. A field layout
is not a package-bonding claim or proof of an unreviewed semantic encoding.
See [HAL cfg layering](hal-cfg-layering.md).
