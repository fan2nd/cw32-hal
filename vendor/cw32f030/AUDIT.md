# CW32F030 register evidence

The unmodified SDK V2.2 header and SVD are pinned by SHA-256 in manifest.json.
The importer verifies their hashes before interpreting either source. Manual and
datasheet PDF hashes and official download URLs are recorded in the same manifest.
Run `cargo run -p cw32-gen --example audit_import_f030` to audit, or append
`-- --write` for explicit regeneration. Normal data/PAC generation does not import
vendor files. No Python or additional crate is involved.

## Inventory and model decisions

The SDK/SVD contains 37 peripheral instances, 22 distinct vendor register structs,
268 register declarations and 1404 fields. GPIOC/F are strict pin subsets of the
canonical GPIO layout after the documented IDR access correction. The canonical
set has 20 kinds and 223 register declarations, including explicit CRC aliases.

| Kind | Model | F030 difference from L012 |
|---|---|---|
| adc | v_f030 | CR0/CR1/CR2 and four-slot sequencer, distinct trigger/result/flag layouts |
| atim | v_f030 | CR, CHxCR, A/B channels and event-command bits; not the L012 timer IP |
| awt | v_f030 | Separate automatic wakeup timer, absent as a standalone L012 block |
| btim | v_f030 | ARR at 0, BCR/ACR control and distinct flags |
| crc | v_f030 | 8/16/32-bit input bus access aliases and 16/32-bit results |
| dma | v_f030 | Five channels, no L012 current-count/address registers |
| dmachannel | v_f030 | Five-register channel view, distinct CSR/trigger semantics |
| flash | v_f030 | Distinct security/busy bits, no L012 SDK configuration register |
| gpio | v_f030 | Distinct AFR names, high/low interrupts and full implemented-pin pulldown |
| gtim | v_f030 | Registers at 0x300+, CR0 prescaler, no PSC or UIFCPY |
| i2c | v_f030 | SI-driven state machine, not the L012 master/slave FIFO IP |
| iwdt | v1 | Entire register/field/access and command/key/clear policy matches |
| lvd | v_f030 | Distinct source, threshold, filter and interrupt field layout |
| ram | v_f030 | IER.EN bit 0 and PARITY bit 1 |
| rtc | v_f030 | ACCESS/WINDOW handshake and COMPEN; no L012 PSC/SSCNT |
| spi | v_f030 | Different CR/IER/DR offsets and control fields |
| sysctrl | v_f030 | Different clock tree/gates/reset bits; AHB/APB gates are not keyed |
| uart | v_f030 | Different framing, clock and interrupt fields |
| vc | v_f030 | Different mux/filter controls; reference divider sharing |
| wwdt | v1 | Entire register/field/access and sticky-enable/clear policy matches |

F030 has no CORDIC, EAU, OPA, DAC, HALLTIM, LPTIM, or standalone BGR/VCREF/IRMOD
peripheral instances. IR modulation configuration is inside SYSCTRL. No such
missing peripheral is copied from L012. The importer never writes a v1 file.

IWDT/WWDT reuse is proven at the register and access-policy level by an independent
F030 manual review (sections 16.3–16.6 and 17.3–17.6), then a structural comparison
against v1. This does not imply an identical peripheral clock: F030 IWDT uses its
nominal RC10K source. The L012 source descriptions remain unchanged; F030-specific
provenance is in side-effects.yaml and this document.

## Explicit source disagreements

- GPIO IDR: header marks all views RW, SVD marks A/B RO and C/F RW. Manual 9.6.18
  specifies all IDRs and their input fields RO. Canonical GPIO is RO.
- GPIO names: manual calls the keyed lock register LCKR, but header/SVD call it
  LOCK. The PAC retains the official SDK/SVD name. Section 9.6.16 supplies the key.
- GPIOC/F: C only has pins 13–15 and no AFRL; F only has raw fields for pins
  0,1,3,6,7 and no AFRH. Shared raw PAC offsets do not prove a register exists on
  every port. Die pin metadata and the safe HAL constrain available pins.
- PF03/BOOT: header/SVD and manual GPIO table describe PIN3, but datasheet 1.9
  Table 3-1 classifies PF03/BOOT as dedicated BOOT rather than general I/O. The
  raw family mask remains 0x00cb; CW32F030C8 pins exclude PF3. Unsafe raw access
  must not infer pad output capability from the family mask.
- RAM.IER.EN: header bitfield is RW; SVD and manual 6.6.2 say RO. Keep RO.
- GTIM.ISR.UD/TI/OV: header bitfields are RW; SVD and manual 14.8.12 say RO.
- RTC.ALARMA.HOUREN: header bitfield has typo HOURRN; macro, SVD and manual
  12.5.8 agree on HOUREN. Normalize this one spelling explicitly.
- RTC.TAMPDATE/TAMPTIME: header and SVD register access are RW, while SVD fields
  and manual 12.5.10–11 are RO. Both register and fields are represented RO.
- DMA.CSR.STATUS: manual 8.8.3 labels RW; header and SVD agree RO. Preserve the
  conservative RO API; no software write semantics are assumed.
- SVD reports three NVIC priority bits, while SDK CMSIS header reports two.
  The register importer does not turn SVD CPU metadata into a core configuration.
  HAL/Cortex configuration must follow separately audited device settings.

No generic ignore rule accepts a source mismatch: individual discrepancies are
asserted, and every remaining header offset, width, field mask, access and field
count must match the SVD. Pins and chip suffixes are outside register import.

## Side effects and ownership

side-effects.yaml is a separate F030 manual-derived overlay, never an import of
L012 metadata. It records write-zero clears, GPIO commands, keyed configuration,
CRC width-sensitive data feeds, timer/ADC software commands, DMA SOFTSRC, I2C SI
state-machine advancement, watchdog behavior and RTC access windows. SPI DR reads
clear RXNE; UART RDR reads do not clear RC. ADC RESULT0–3 reads consume the
unread-result state used by overrun/DISCARD logic (the latch read classification),
but do not automatically clear EOC, EOS or OVW. SPI ICR.FLUSH is a zero-triggered
transmit flush, so a zero-initialized write is not an innocuous flag clear.

DMA channel views explicitly belong to DMA. Their IRQ associations are
DMACH1, DMACH23 and DMACH45. FLASH/RAM share FLASHRAM; the watchdogs share WDT.
SYSCTRL includes RCC and FAULT. The CMSIS IRQ enum is the 32-vector authority,
checked against every explicit SVD association. All gate/reset references are
verified against SYSCTRL field macros. Shared BTIM and VC gates stay shared.
VC reference division and the ADC bandgap are shared resources despite separate
VC address windows; raw views do not grant independent ownership.

This is a source audit and generated API test, not silicon validation or a claim
that every field value, reset value, electrical mode or erratum is modeled.
