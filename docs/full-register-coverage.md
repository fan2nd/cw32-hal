# CW32L012C8 complete register model coverage

The source inventory is **50 vendor peripheral instances, 28 reusable IP models,
306 logical register views and 1713 retained fields**. Logical counts include
same-address timer capture/compare and I2C controller/target views, and the DMA
aggregate/channel views. They are not 306 distinct physical words.

The register inventory was audited against the pinned CMSIS header and SVD. Source discrepancies and selected resolutions are recorded below.

## Explicit source discrepancies and resolution

- ATIM +0x0C: header union is `IER`; header mask macros, SVD and RM §17 use
  `DIER`. The maintained register is DIER. Only this exact header spelling is
  normalized for evidence comparison.
- ADC.ISR: both header and SVD word tags are incorrectly RW; the SVD fields
  and RM §25.12.9 ISR table are RO. Header AWDH child also incorrectly uses
  __IOM. Override the entire register and every field to RO.
- DAC.DOR1/2.DATA: header child bitfields incorrectly have `__IOM`; enclosing
  registers, SVD and RM §26.10.12–13 are RO. Retain RO.
- GPIO.PDR: SVD and macro catalog contain PIN0..15, but actual header bitfield
  and RM §9 implement only PF03. Retain PIN3 only (15 false fields excluded).
  The shared model's field does not imply a pull-down exists on another port;
  each GPIO instance retains its implemented/pulldown mask.
- VC12REF SVD `headerStructName=VC2REF` is a naming defect. CMSIS declares
  `VCREF_TypeDef`, used by VC12REF/VC34REF. Its REF register macros are named
  `VCREF_DIV_*`; normalize this prefix only. The three fields match the header
  bitfields and SVD exactly. See semantics audit for unresolved DIV width prose.
- WWDT header uniquely omits address-offset comments. Audit the exact order and
  access of its three consecutive u32 unions CR0/CR1/SR and compare offsets
  0/4/8 with SVD and RM §20.6.
- Timer CCMR capture/compare views and I2C MCR2/SCR2 are same-address aliases,
  explicitly represented by `alias_of`; distinct fields are mode-dependent.
- DMA channel instances are views into DMA, not independent ownership tokens.

These are evidence-backed normalizations, not silent acceptance of arbitrary
vendor disagreements. Vendor hashes remain pinned by the original audit tests.
Other disagreements, manual/SDK hazards and action-register semantics are
recorded in `full-peripheral-semantics.md`.

## Limits

Register/field layout coverage does not certify every raw encoding, electrical
behavior, DMA request route, silicon erratum or a working hardware driver.
Multi-bit values without an individually verified enumeration remain raw.
Access direction does not imply that read-modify-write is safe. Side-effect
metadata and unsafe PAC contracts must be observed. All hardware validation is
still outstanding. The original partial-package claim is superseded only when
the separately audited pin maps and route evidence are committed.

DMA.CCNT1–4 and DMACHANNEL.CCNT are intentionally more restrictive than the
header/SVD: RM §8.8.8 marks both current-count fields RO. Both register and field
APIs are RO. The source audit first checks the two vendor artifacts agree on
the old RW annotation, then asserts the exact conservative RO correction.

## Remaining reference conflicts

- I2C MFIFOCR.RXWATER / MFIFOSR.RXCNT: RM tables say bits 27:16; header
  declarations, SVD and masks say 17:16. Retain two bits and do not fabricate a
  12-bit field. Hardware/vendor clarification is required before expanding it.
- VCREF.REF.DIV: RM describes 3:0, but header/SVD encode 2:0 and SDK known
  values are 0..7. Retain three bits; no invented encoding for bit 3.
- GTIM.ISR: RM access tag says RW while header/SVD say RO. Dedicated ICR and
  SDK clearing code agree on status/clear separation; retain RO.
- These conservative choices deliberately block unsupported encodings or
  writes. Complete layout coverage does not mean these source contradictions
  have been resolved by silicon measurements.

All registers are 32-bit aligned and SVD provides explicit individual names;
there are no SVD `dim` arrays or clusters to lose. Repeated registers and
channels are explicitly enumerated and every resulting address is checked.
