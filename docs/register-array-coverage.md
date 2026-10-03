# Indexed register coverage: both CW32 chips

All46 shared IP definitions and520 original physical register views were reviewed. Implemented26 groups:24 register arrays and2 repeated DMA subblocks. No generator-side register-name inference or scalar compatibility aliases were added.

Each grouped element retains its original name/reset evidence. Width/access/sideeffects and complete field layouts were compared before grouping. The index is zero-based even when hardware numbering begins at1.

## Implemented groups

| IP | Accessor | Original members | Byte offsets |
| --- | --- | --- | --- |
| adc/l012 | result(n) | RESULT0, RESULT1, RESULT2, RESULT3, RESULT4, RESULT5, RESULT6, RESULT7 | 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c |
| adc/f030 | result(n) | RESULT0, RESULT1, RESULT2, RESULT3 | 0x20, 0x24, 0x28, 0x2c |
| dma/l012 | ch(n) | CSR1, CSR2, CSR3, CSR4 | 0x20, 0x40, 0x60, 0x80 |
| dma/f030 | ch(n) | CSR1, CSR2, CSR3, CSR4, CSR5 | 0x20, 0x40, 0x60, 0x80, 0xa0 |
| gtim/l012 | ccr(n) | CCR1, CCR2, CCR3, CCR4 | 0x34, 0x38, 0x3c, 0x40 |
| gtim/f030 | ccr(n) | CCR1, CCR2, CCR3, CCR4 | 0x320, 0x324, 0x328, 0x32c |
| gtim/l012 | ccmr_cap(n) | CCMR1CAP, CCMR2CAP | 0x18, 0x1c |
| gtim/l012 | ccmr_cmp(n) | CCMR1CMP, CCMR2CMP | 0x18, 0x1c |
| atim/l012 | ccmr_cap(n) | CCMR1CAP, CCMR2CAP, CCMR3CAP | 0x18, 0x1c, 0x50 |
| atim/l012 | ccmr_cmp(n) | CCMR1CMP, CCMR2CMP, CCMR3CMP | 0x18, 0x1c, 0x50 |
| atim/l012 | ccr(n) | CCR1, CCR2, CCR3, CCR4 | 0x34, 0x38, 0x3c, 0x40 |
| atim/l012 | ccr_group(n) | CCR5, CCR6 | 0x48, 0x4c |
| atim/f030 | chcr(n) | CH1CR, CH2CR, CH3CR | 0x24, 0x28, 0x2c |
| atim/f030 | ccra(n) | CH1CCRA, CH2CCRA, CH3CCRA | 0x3c, 0x44, 0x4c |
| atim/f030 | ccrb(n) | CH1CCRB, CH2CCRB, CH3CCRB | 0x40, 0x48, 0x50 |
| dac/l012 | dhr12r(n) | DHR12R1, DHR12R2 | 0x8, 0x14 |
| dac/l012 | dhr12l(n) | DHR12L1, DHR12L2 | 0xc, 0x18 |
| dac/l012 | dhr8r(n) | DHR8R1, DHR8R2 | 0x10, 0x1c |
| dac/l012 | dor(n) | DOR1, DOR2 | 0x2c, 0x30 |
| gpio/l012 | afr(n) | AFRL, AFRH | 0x18, 0x14 |
| gpio/f030 | afr(n) | AFRL, AFRH | 0x18, 0x14 |
| gpio/f030 | odr_byte(n) | ODRLOWBYTE, ODRHIGHBYTE | 0x54, 0x55 |
| i2c/f030 | secondary_addr(n) | ADDR1, ADDR2 | 0x20, 0x24 |
| rtc/l012 | alarm(n) | ALARMA, ALARMB | 0x1c, 0x20 |
| rtc/f030 | alarm(n) | ALARMA, ALARMB | 0x1c, 0x20 |
| sysctrl/f030 | gtimcap(n) | GTIM1CAP, GTIM2CAP, GTIM3CAP, GTIM4CAP | 0x50, 0x54, 0x58, 0x5c |

## All model decisions

| Model | Indexed accessors | Other reviewed candidates / reason |
| --- | --- | --- |
| adc/f030 | result(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| adc/l012 | result(n) | Numbered DMA channel registers are covered by the indexed child block. |
| atim/f030 | chcr(n), ccra(n), ccrb(n) | chcc(n): Three contiguous A/B compare pairs at0x3c+8*n; child ccra at0 and ccrb at4. This is an alternative nesting of ccra/ccrb arrays.; all_channel_control(n): CH4 differs in fields, reset(0 vs0x3000), write behavior(Ordinary vsMixed). Do not flatten into one full fieldset.; full_channel(n): Whole channels are not uniform translated subblocks: CR offsets step4 while paired CCRA/B offsets step8. A pointer-only chiptool block cannot expose both with fixed child offsets. Use chcr and ccra/ccrb or chcc arrays. |
| atim/l012 | ccmr_cap(n), ccmr_cmp(n), ccr(n), ccr_group(n) | ccr_all(n): Rejected by implementation design: preserve homogeneous full types rather than weaker common-subset views. Irregular six-element array possible only with explicit common low16 Ccr fieldset. Keep named/extended CCR5/6 access so GC controls are not erased; never manufacture GC fields on CCR1..4.; excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; excluded_tiseln: TISEL1 has four input selectors; TISEL2 has two. Use indexed field arrays with respective lengths; no full homogeneous register array.; excluded_afn: AF1 ETRSEL and AF2 OCRSEL have different hardware roles; BK and BK2 routes are related but not identical semantics. Equal bit widths do not justify one full typed array. |
| awt/f030 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| bgr/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| btim/f030 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| btim/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| cordic/l012 | None | coordinates(n): Named operands/results carry operation- and order-dependent semantics, including read-clear result completion. Numeric data width equality does not establish a generic repeated channel block. |
| crc/f030 | None | excluded_drn: Suffix numbers are bus-access widths, not array indices. Same-address8/16/32 aliases must retain distinct transaction types.; excluded_resultn: Suffix numbers are bus-access widths, not array indices. Same-address8/16/32 aliases must retain distinct transaction types. |
| crc/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| dac/l012 | dhr12r(n), dhr12l(n), dhr8r(n), dor(n) | holding(n): Repeated three-register holding group at0x08+0x0c*n with 12R/12L/8R views. DORs are not at a fixed translated offset in this same group; retain separate dor(n).; excluded_crn: Broad numeric match confuses resolution/alignment with channel indices. Only the explicit same-format channel pairs above are valid arrays.; excluded_dhrnrn: Broad numeric match confuses resolution/alignment with channel indices. Only the explicit same-format channel pairs above are valid arrays.; excluded_dhrnrd: Broad numeric match confuses resolution/alignment with channel indices. Only the explicit same-format channel pairs above are valid arrays. |
| dma/f030 | ch(n) | Numbered DMA channel registers are covered by the indexed child block. |
| dma/l012 | ch(n) | Numbered DMA channel registers are covered by the indexed child block. |
| dmachannel/f030 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| dmachannel/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| eau/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| flash/f030 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| flash/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| gpio/f030 | afr(n), odr_byte(n) | Numbered DMA channel registers are covered by the indexed child block. |
| gpio/l012 | afr(n) | Numbered DMA channel registers are covered by the indexed child block. |
| gtim/f030 | ccr(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| gtim/l012 | ccr(n), ccmr_cap(n), ccmr_cmp(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; excluded_afn: Numbered names do not establish interchangeable slots. Different fields/roles, widths or semantics require distinct register types. |
| halltim/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| i2c/f030 | secondary_addr(n) | addr(n): Rejected by implementation design: preserve homogeneous full types rather than weaker common-subset views. Irregular offsets [0x10,0x20,0x24]. A common view can expose only address bits1..7; retain ADDR0-specific GC typed view. Do not expose a GC setter at indices1/2. |
| i2c/l012 | None | excluded_mcrn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; excluded_scrn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; master_slave: Master/slave domains have distinct fields, roles and irregular offsets; SCR2 even aliases MCR2 at0x24. Do not assume uniform translated blocks. |
| irmod/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| iwdt/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| lptim/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| lvd/f030 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| lvd/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| opa/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| ram/f030 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| ram/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| rtc/f030 | alarm(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; date_capture_domains: Live writable DATE and read-only captured TAMPDATE differ access and domain; not an interchangeable slot array. |
| rtc/l012 | alarm(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; date_capture_domains: Live writable DATE and read-only captured TAMPDATE differ access and domain; not an interchangeable slot array. |
| spi/f030 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| spi/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| sysctrl/f030 | gtimcap(n) | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; excluded_apbenn: APB1 and APB2 gate/reset words control different peripherals and have different fields/full defaults. A common semantic fieldset is invalid.; excluded_apbrstn: APB1 and APB2 gate/reset words control different peripherals and have different fields/full defaults. A common semantic fieldset is invalid. |
| sysctrl/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ.; excluded_apbenn: APB1 and APB2 gate/reset words control different peripherals and have different fields/full defaults. A common semantic fieldset is invalid.; excluded_apbrstn: APB1 and APB2 gate/reset words control different peripherals and have different fields/full defaults. A common semantic fieldset is invalid. |
| uart/f030 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| uart/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| vc/f030 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| vc/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |
| vcref/l012 | None | Reviewed all register names, offsets, field layouts and roles; no repeated register or repeated subblock candidates. Separate peripheral instances are not register arrays within this block. |
| wwdt/l012 | None | excluded_crn: Numbered control/configuration words are distinct functional registers, not repeated identical channel/register instances; field layouts differ. |

## Differences kept explicit

- ATIM L012 CCR5/6 contain extra grouping bits; they use a separate ccr_group array, not a weak shared subset.
- ATIM F030 CH4 has a different complete layout and remains separate from the CH1–3 arrays.
- F030 I2C ADDR0 has the general-call bit; only ADDR1/2 form secondary_addr(n).
- I2C master/slave, oscillator controls, CR0/1/2 and similar numbered but semantically distinct registers were not falsely treated as one array.
- GPIO byte aliases preserve8-bit MMIO inside the scalar ODR word. Irregular and descending offsets are explicit data.
- L012 RTC alarm and F030 GTIM CCR retain different per-element reset seeds. All read-clear/keyed/mixed/command behavior remains typed.

Reference architecture: [pinned chiptool IR](https://github.com/embassy-rs/chiptool/blob/bcf538a2e7b8584ae874ee9ab72efb1576fc6152/src/ir.rs). Hardware offsets, fields and resets remain sourced from the per-register CW32 official evidence in YAML.
