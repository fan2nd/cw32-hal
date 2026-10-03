# v0.14.0 data model and source audit

This review compared the maintained data with the pinned original headers/SVDs,
manual reset headings, IRQ/gate/reset definitions and pin-route tables. It found
no new register-layout or route correction beyond the already documented source
conflicts. It did fix data-layer validation and move comparator source wiring
out of HAL name inference into sourced family-instance data.

Coverage: 87 instances, 46 IP models, 520 expanded register views, 158 clock/reset
references, two 32-entry IRQ tables, 79 pin identities and 233 routes. There are
463 distinct scalar/array-element reset evidence records: 442 known full words
and 21 unknown/context-dependent words. DMA subblock reuse adds 57 known views,
giving 499 known and 21 unknown expanded views. These counts describe different
levels of reuse, not lost records.

The 233 routes are 72 digital + 54 analog for L012 and 78 digital + 29 analog
for F030. Digital routes were checked against L012 SDK AF macros and F030 RM9-2;
analog routes against the corresponding ADC/VC/OPA/DAC tables. Chip capability
does not establish package bonding or board wiring. F030 PF3 remains withheld.

## Every existing IP model

Counts are expanded register views/fields within each reusable block, including DMA nested banks. Reset values count logical expanded views. The same watchdog definition is reused by both chips.

| Model | Views / fields | Known / unknown reset views | Primary RM sections | Finding / retained hardware difference |
|---|---:|---:|---|---|
| adc/f030 | 16 / 70 | 16 / 0 | F030 RM2.5 §§22.13 | Genuine different CR0/CR1/CR2, four result slots and START/OVW/DISCARD semantics; result reads are latch-consuming. |
| adc/l012 | 18 / 94 | 18 / 0 | L012 RM1.4 §§25.12 | Retain manual-backed ISR RO correction; two ADC instances share layout, not external channel routing. Eight result slots are explicit indexed elements. |
| atim/f030 | 21 / 150 | 21 / 0 | F030 RM2.5 §§15.7 | Genuine CHx A/B control/event layout, not L012 CCMR/CCER/BDTR; commands remain mixed-effect. |
| atim/l012 | 31 / 266 | 31 / 0 | L012 RM1.4 §§17.10 | Retain IER→DIER header-name normalization; capture/compare aliases, sparse CCR groups and read-clear capture values remain explicit. |
| awt/f030 | 6 / 9 | 6 / 0 | F030 RM2.5 §§11.8 | Genuine F030 standalone wakeup timer; L012 has no separate corresponding instance. |
| bgr/l012 | 1 / 2 | 1 / 0 | L012 RM1.4 §§25.12 | Genuine separately addressable analog bandgap resource at ADC1_BASE+0xFC; no invented clock/reset gate. |
| btim/f030 | 8 / 22 | 8 / 0 | F030 RM2.5 §§13.7 | Genuine ARR/BCR/ACR layout; BTIM1/2/3 share gate/reset. |
| btim/l012 | 10 / 28 | 10 / 0 | L012 RM1.4 §§14.10 | Genuine CR1/SMCR/DIER/EGR layout; BTIM1/2/3 share gate/reset. |
| cordic/l012 | 4 / 13 | 4 / 0 | L012 RM1.4 §§12.6 | Operand writes may trigger calculation; result reads clear READY. No F030 instance. |
| crc/f030 | 6 / 6 | 6 / 0 | F030 RM2.5 §§10.6 | Genuine 8/16/32-bit input and 16/32-bit result aliases retain bus widths; no false ordinary RMW. |
| crc/l012 | 3 / 3 | 3 / 0 | L012 RM1.4 §§10.6 | Writes initialize/feed engine; 32-bit data view retained. |
| dac/l012 | 14 / 34 | 14 / 0 | L012 RM1.4 §§26.10 | DOR DATA is RO despite erroneous header child; underrun flag remains W0C. Two hardware channels modeled, internal VC connection now explicit. |
| dma/f030 | 27 / 95 | 27 / 0 | F030 RM2.5 §§8.8 | Five nested channel banks reuse dmachannel/f030; 25 channel views plus two global registers match. |
| dma/l012 | 34 / 112 | 34 / 0 | L012 RM1.4 §§8.8 | Four nested channel banks reuse dmachannel/l012; 32 channel views plus two global registers match. |
| dmachannel/f030 | 5 / 15 | 5 / 0 | F030 RM2.5 §§8.8 | Five-word channel view; STATUS remains conservative RO where RM and vendor access conflict. |
| dmachannel/l012 | 8 / 24 | 8 / 0 | L012 RM1.4 §§8.8 | Retain manual-backed CCNT RO correction; current count/address words exist here and not F030. |
| eau/l012 | 5 / 8 | 5 / 0 | L012 RM1.4 §§11.7 | Operand writes start computation; status/result words preserve source semantics; no F030 instance. |
| flash/f030 | 6 / 35 | 5 / 1 | F030 RM2.5 §§7.9 | Distinct busy/security bits and no SDKCFR; protection reset remains unknown. |
| flash/l012 | 7 / 43 | 5 / 2 | L012 RM1.4 §§7.10 | Keyed controls; CR1 protection and SDKCFR values remain unknown/reset-domain-dependent. |
| gpio/f030 | 24 / 356 | 17 / 7 | F030 RM2.5 §§9.6 | Genuine level IRQ/LOCK/drive controls and pulldown support. GPIOC/F subsets remain instance constraints; IDR RO manual correction retained. |
| gpio/l012 | 17 / 258 | 12 / 5 | L012 RM1.4 §§9.6 | PIN arrays and reversed physical AFR index offsets match source. PDR only PIN3. Instance resets and contradictory ANALOG/ICR headings stay explicit. |
| gtim/f030 | 14 / 61 | 14 / 0 | F030 RM2.5 §§14.8 | Register bank begins at +0x300; CR0 prescaler, no PSC/UIFCPY; retain ISR field RO correction. |
| gtim/l012 | 23 / 141 | 23 / 0 | L012 RM1.4 §§16.10 | Capture/compare aliases and read-clear CCR; dedicated PSC and UIFCPY are genuine layout differences. |
| halltim/l012 | 9 / 29 | 9 / 0 | L012 RM1.4 §§18.8 | CNT is clear-on-zero; software commands and BTIM3 shared IRQ remain explicit; no F030 instance. |
| i2c/f030 | 9 / 17 | 9 / 0 | F030 RM2.5 §§20.7 | SI-driven state machine rather than L012 FIFO IP; DR writes and SI advancement are commands. |
| i2c/l012 | 29 / 129 | 29 / 0 | L012 RM1.4 §§23.8 | Master/slave FIFO and overlapping MCR2/SCR2 views; FIFO/address reads and command/clear writes remain nonordinary. |
| irmod/l012 | 1 / 3 | 1 / 0 | L012 RM1.4 §§24.4 | Separate source register; F030 IR modulation is inside SYSCTRL, not a copied instance. |
| iwdt/l012 | 6 / 14 | 6 / 0 | L012 RM1.4 §§19.6; F030 RM2.5 §§16.3–16.6 | Full layout/access/key/command policy shared with F030; genuine source clock differences stay in family/HAL layer. |
| lptim/l012 | 9 / 46 | 9 / 0 | L012 RM1.4 §§15.7 | Separate low-power timer with mixed CR0 command bits; no F030 instance. |
| lvd/f030 | 3 / 13 | 3 / 0 | F030 RM2.5 §§24.7 | Distinct source/threshold/filter layout; W0C status retained. |
| lvd/l012 | 3 / 12 | 3 / 0 | L012 RM1.4 §§28.7 | W0C status; threshold/filter encoding differs from F030 and remains raw. |
| opa/l012 | 2 / 35 | 2 / 0 | L012 RM1.4 §§29.6 | Mixed calibration command/status; two instances share layout, not pin connections; no F030 instance. |
| ram/f030 | 4 / 5 | 4 / 0 | F030 RM2.5 §§6.6 | Retain SVD/manual IER.EN RO correction, despite header RW child. |
| ram/l012 | 4 / 4 | 4 / 0 | L012 RM1.4 §§6.6 | Parity status/clear and fault address preserved; EN/PARITY positions differ from F030. |
| rtc/f030 | 15 / 66 | 15 / 0 | F030 RM2.5 §§12.5 | Distinct ACCESS/WINDOW/COMPEN; retain TAMPDATE/TAMPTIME RO and HOUREN typo corrections. |
| rtc/l012 | 18 / 70 | 18 / 0 | L012 RM1.4 §§13.5 | Keyed write window, two alarm array elements and L012 PSC/SSCNT; backup/reset-domain notes retained. |
| spi/f030 | 7 / 42 | 7 / 0 | F030 RM2.5 §§19.8 | Different CR/IER/DR offsets; preserve read-clear RXNE and zero-triggered FLUSH semantics. |
| spi/l012 | 8 / 44 | 8 / 0 | L012 RM1.4 §§22.7 | DR reads clear RXNE and writes transmit; ICR zero is not a neutral command. |
| sysctrl/f030 | 28 / 176 | 25 / 3 | F030 RM2.5 §§4.7, 21.4 | AHB/APB gates are unkeyed; genuine 48 MHz HSI / 6 reset clock source and reset field map; full clock tree unmodeled. |
| sysctrl/l012 | 19 / 160 | 16 / 3 | L012 RM1.4 §§4.7 | Keyed clock enables, active-low resets and reset-source uncertainty preserved. Full clock tree remains unmodeled. |
| uart/f030 | 11 / 42 | 11 / 0 | F030 RM2.5 §§18.9 | Distinct framing/clock fields; RDR read does not itself clear RC. |
| uart/l012 | 15 / 83 | 15 / 0 | L012 RM1.4 §§21.9 | TDR transmit command, separate clear register; framing/clock fields differ from F030. |
| vc/f030 | 4 / 26 | 4 / 0 | F030 RM2.5 §§23.7 | Internal divider/BGR sharing is a genuine F030 relationship; no fictitious independent VCREF/DAC instance added. |
| vc/l012 | 4 / 49 | 4 / 0 | L012 RM1.4 §§27.7 | Register layout unchanged. Fixed topology placement: four instance-level reference/DAC connections replace HAL name inference. |
| vcref/l012 | 1 / 3 | 1 / 0 | L012 RM1.4 §§27.7 | VC2REF headerStructName/VCREF_DIV macro spelling discrepancy normalized; DIV remains narrower documented 3-bit source model. |
| wwdt/l012 | 3 / 6 | 3 / 0 | L012 RM1.4 §§20.6; F030 RM2.5 §§17.3–17.6 | Full layout/access/sticky-enable/clear policy shared with F030; watchdog semantics are not ordinary RMW. |

## Corrected data-layer responsibilities

- Schema 7 places comparator reference/DAC source connections on the peripheral
  instance, with required evidence and validated target kinds/channels.
- Chip/family, core/target and generated namespace/filename checks fail closed.
- The 32-bit address-space upper boundary accepts a valid final byte without
  relaxing overflow, alignment, alias or overlap checks.
- Perimap/fixes remain explicit, auditable transformations. Source comments and
  provenance now describe the implemented fields and both chips accurately.

Current multi-bit hardware fields remain raw: there are no populated hardware
enum tables in this dataset. Enum generation support does not mean all semantic
encodings have been audited. The existing single-kind/version-per-chip and
inline-fieldset model also lacks general inheritance and fragmented fields.
Those capabilities are not needed by the current maintained data.

## Retained uncertainty

- L012 VCREF DIV is modeled as 3 bits from header/SVD/SDK and valid values 0–7;
  the manual register table prints 3:0. I2C RXWATER/RXCNT retain the conservative
  2-bit header/SVD shape rather than the conflicting 12-bit manual table.
- L012 GPIOA/B ANALOG/ICR descriptions conflict with reset tables/bit access;
  those disputed instance defaults remain unavailable. HSI reset headings and
  TRIM/reserved-bit prose also conflict.
- Oscillator/reset-cause full words, persistent flash security/SDKCFR and generic
  GPIO full defaults are not manufactured. Verified GPIO instance overrides
  remain separate from the reusable layout; ODR and its narrow views stay unknown.
- L012 boot-ROM endpoint/size disagreement is not used for executable memory.
  Both maintained C8 chips retain unambiguous 64 KiB Flash and 8 KiB RAM.

## Primary sources and upstream comparison

Pinned vendor files and hashes are in `vendor/manifest.json` and the F030 vendor
manifest. Manual sources are [L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
and [F030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf).
Each maintained reset carries its exact section/page/source in YAML. L012
comparator wiring is explicit in §§27.7.1–3, independently of IRQ grouping.

The actual pinned [stm32-data schema](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-serde/src/lib.rs),
[register loader](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-gen/src/registers.rs)
and [perimap](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-gen/src/perimap.rs)
separate reusable IP contracts from instance connections. CW32 preserves that
separation while keeping one generator crate and its real persisted JSON stage.

The data-lock IR comparison uses [chiptool be1bff3e](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/ir.rs), as pinned by that stm32-data Cargo.lock; the later access/render comparison uses the separate exact pin in the PAC audit.

Whole-dataset comparisons and reset-heading checks do not prove every raw field
encoding, undocumented silicon behavior, DMA request map or electrical limit.
See [validation](validation-v0.14.0.md) and [schema 7](schema-v7.md).
