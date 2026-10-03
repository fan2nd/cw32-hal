# PAC semantic values and field-array audit, v0.18.0

All 46 reusable IP/version definitions were reviewed for repeated fields, including existing arrays, numeric-suffix candidates and role-related fields without a numeric suffix. The result contains **156 field arrays (95 added here, including 9 irregular layouts)** and **50 semantic enum fields**. Register/subblock arrays are unchanged. Every implemented group has `array_source` evidence and ordered `elements` in source YAML and persisted JSON; every enum has `values_source`. The audit does not claim that every hardware selector is already enumerated.

The primary sources are [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf) and [CW32x030 RM Rev2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf), checked against the pinned vendor field layouts. Exact section/page citations are retained per group below and in YAML. Grouping changes neither MMIO width nor register access/read/write behavior or reset values. No generator-time name inference is used.

## All 46 IP decisions

| IP/version | Arrays | Enums | Decision and intentionally unmerged candidates |
| --- | ---: | ---: | --- |
| adc/f030 | 6 | 0 | SQR slots and per-instance trigger gates are indexed. CR0/CR1 control roles and watchdog low/high thresholds stay distinct; counts/results remain numeric. |
| adc/l012 | 7 | 1 | Sequence slots, watchdog input mask and trigger gates are indexed. GTIM_OCREF has explicit gaps between timer banks; ATIM TRGO/TRGO2 remain distinct signals. |
| atim/f030 | 14 | 0 | Channels 1–3 flags/filter/mode words are grouped. C4AF remains scalar: channel 4 is compare-only with distinct controls. A/B paths remain distinct groups. |
| atim/l012 | 27 | 6 | CCER, interrupt/event bits, idle states and comparator break routes are grouped. TISEL inputs keep distinct mux meanings; BRK/BRK2 controls remain separate. CCR5/6 grouping bits stay outside CCR1–4. |
| awt/f030 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| bgr/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| btim/f030 | 0 | 0 | No repeated field bank. Mode, divider, count and event controls have different roles. |
| btim/l012 | 0 | 0 | No repeated field bank. CR1/CR2 names identify different control words, not repeated fields. |
| cordic/l012 | 0 | 0 | X/Y operands and sine/cosine/angle results have operation-specific roles; no homogeneous repeated fields. |
| crc/f030 | 0 | 0 | DR/RESULT width suffixes identify different bus access widths. Polynomial/data/count fields remain raw. |
| crc/l012 | 0 | 0 | CRC input, polynomial, initial value and result have distinct roles; no repeated field bank. |
| dac/l012 | 12 | 2 | All two-channel control/data pairs are indexed, preserving each bus word and raw sample width. WAVE/TSEL gain semantic enums; MAMP remains numeric encoding with mode-dependent meaning. |
| dma/f030 | 4 | 0 | Five TC and TE flag/clear slots grouped separately. Channel bank still has five elements; no status/control mixing. |
| dma/l012 | 4 | 0 | Four TC and TE flag/clear slots grouped separately. Channel bank still has four elements; R1W0 seed remains all ones. |
| dmachannel/f030 | 0 | 5 | Single channel view has no repeated fields; CSR modes/status and TRIG selectors use enums. Addresses/counts stay raw. |
| dmachannel/l012 | 0 | 5 | Single channel view has no repeated fields; CSR modes/status and TRIG selectors use enums. Current counts/reload control remain distinct. |
| eau/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| flash/f030 | 1 | 0 | Sixteen PAGELOCK groups are indexed. Key, erase/program, wait-state and flags retain distinct roles and write policies. |
| flash/l012 | 1 | 0 | Sixteen PAGELOCK groups are indexed. Key, erase/program, wait-state and flags retain distinct roles and write policies. |
| gpio/f030 | 22 | 0 | Existing 22 field arrays now retain element identity and evidence. Set/reset halfwords remain separate; lock key is scalar. |
| gpio/l012 | 16 | 0 | Existing 16 field arrays now retain element identity and evidence. Set/reset halfwords remain separate; lock key is scalar. |
| gtim/f030 | 7 | 2 | CCM, IRQ/DMA channel bits and input polarity/filter groups are indexed. Trigger, overflow, direction and encoder status retain distinct roles. |
| gtim/l012 | 19 | 6 | CCER, interrupt/event and CCMR fields are indexed. TI1–TI4 muxes are kept scalar because their selections differ by input; split extended OCMH is not merged with OCM. |
| halltim/l012 | 2 | 0 | Three pre-filter and three post-filter input states form separate groups. Hall phase states are not timer counters. |
| i2c/f030 | 1 | 0 | Three address-match flags are indexed. Address-0 general-call control stays distinct from secondary addresses. |
| i2c/l012 | 2 | 3 | Address-match flags are indexed. MMATCH0/1 are asymmetric under MATCFG (MATCH1 can be a mask); SAMR0/1 have different widths/roles under ADDRCFG and can be ordered range bounds. SDA/SCL timing and TX/RX FIFO controls remain distinct. |
| irmod/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| iwdt/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| lptim/l012 | 0 | 0 | CH1SRC and CH2SRC are source-specific selector tables, not one uniform enum bank. Count/compare/period have distinct roles. |
| lvd/f030 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| lvd/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| opa/l012 | 3 | 0 | Unambiguous GTIM TRGO, ATIM OC4–6 and ADC START trigger gates are indexed. OC1-named fields are retained: RM1.4 29.6.2 descriptions call them OC2, so they need evidence reconciliation. INP1–3 external inputs, INP4 internal DAC and INN paths retain named topology switches. |
| ram/f030 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| ram/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| rtc/f030 | 0 | 0 | Calendar components, alarm controls and subsecond values differ in range and meaning; no homogeneous field array. |
| rtc/l012 | 0 | 0 | PSC1/PSC2 and SSCNT1/SSCNT0 are unequal-width cascaded stages, not interchangeable repeated fields. Calendar and alarm components retain distinct roles. |
| spi/f030 | 0 | 2 | No repeated channel fields. MODE and discrete BR have enums; WIDTH, transfer data and arithmetic quantities remain raw. |
| spi/l012 | 0 | 1 | No repeated channel fields. MODE has an enum; arithmetic BR, GAP, WIDTH and transfer data remain raw. |
| sysctrl/f030 | 2 | 4 | GTIM capture selectors and DEBUG freeze bits are indexed. Clock/reset gate bits retain scalar resource identities used by instance metadata; SYSCLK and HSI/AHB/APB divisor selections have audited enums. GTIMETR/TIMITR muxes retain source-specific names and tables. |
| sysctrl/l012 | 1 | 4 | DEBUG GTIM freeze bits are indexed. Clock/reset gate bits retain scalar resource identities used by instance metadata; SYSCLK and HSI/AHB/APB divisor selections now have audited enums. |
| uart/f030 | 0 | 4 | No repeated channel fields. Oversampling, stop, parity and clock selectors have enums. Baud/count/address/data stay raw. |
| uart/l012 | 0 | 5 | No repeated channel fields. Oversampling, stop, parity, clock and RX-source selectors have enums. Baud/count/address/data stay raw. |
| vc/f030 | 1 | 0 | Three ATIM channel-B blanking gates are indexed. Positive/negative source and reference fields stay separate. |
| vc/l012 | 4 | 0 | Repeated timer blanking gates are indexed, including explicit GTIM gaps. Positive/negative source and reference controls stay separate. |
| vcref/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |
| wwdt/l012 | 0 | 0 | No homogeneous repeated field bank: the implemented control, status, count/data and calibration fields have distinct functions. Numbered register names alone do not establish field equivalence. |

## New field groups and physical positions

Indices are zero-based in the order shown. Bit offsets below are absolute in the containing register. Repeated register fieldsets record their representative first-register field identities; the outer register elements retain the actual register names and manual references. In particular, CCMR slots represent channel pairs, GPIO AFR slots are local to a bank, and CCR_GROUP retains CCR5/CCR6 separately from CCR1–4.

| IP/version | Register.field | Source fields in index order | Bit offsets | Evidence |
| --- | --- | --- | --- | --- |
| dma/l012 | ISR.TC | TC1, TC2, TC3, TC4 | 0, 4, 8, 12 | CW32L012 RM1.4 section 8.8.1, printed p118 (PDF p144) |
| dma/l012 | ISR.TE | TE1, TE2, TE3, TE4 | 1, 5, 9, 13 | CW32L012 RM1.4 section 8.8.1, printed p118 (PDF p144) |
| dma/l012 | ICR.TC | TC1, TC2, TC3, TC4 | 0, 4, 8, 12 | CW32L012 RM1.4 section 8.8.2, printed p119 (PDF p145) |
| dma/l012 | ICR.TE | TE1, TE2, TE3, TE4 | 1, 5, 9, 13 | CW32L012 RM1.4 section 8.8.2, printed p119 (PDF p145) |
| dma/f030 | ISR.TC | TC1, TC2, TC3, TC4, TC5 | 0, 4, 8, 12, 16 | CW32x030 RM Rev2.5 section 8.8.1, printed p137 (PDF p138) |
| dma/f030 | ISR.TE | TE1, TE2, TE3, TE4, TE5 | 1, 5, 9, 13, 17 | CW32x030 RM Rev2.5 section 8.8.1, printed p137 (PDF p138) |
| dma/f030 | ICR.TC | TC1, TC2, TC3, TC4, TC5 | 0, 4, 8, 12, 16 | CW32x030 RM Rev2.5 section 8.8.2, printed p138 (PDF p139) |
| dma/f030 | ICR.TE | TE1, TE2, TE3, TE4, TE5 | 1, 5, 9, 13, 17 | CW32x030 RM Rev2.5 section 8.8.2, printed p138 (PDF p139) |
| gtim/l012 | IER.CCIE | CC1IE, CC2IE, CC3IE, CC4IE | 1, 2, 3, 4 | CW32L012 RM1.4 section 16.10.4, printed p290 (PDF p316) |
| gtim/l012 | IER.CCDE | CC1DE, CC2DE, CC3DE, CC4DE | 9, 10, 11, 12 | CW32L012 RM1.4 section 16.10.4, printed p290 (PDF p316) |
| gtim/l012 | ISR.CCIF | CC1IF, CC2IF, CC3IF, CC4IF | 1, 2, 3, 4 | CW32L012 RM1.4 section 16.10.5, printed p292 (PDF p318) |
| gtim/l012 | ISR.CCOF | CC1OF, CC2OF, CC3OF, CC4OF | 9, 10, 11, 12 | CW32L012 RM1.4 section 16.10.5, printed p292 (PDF p318) |
| gtim/l012 | ICR.CCIF | CC1IF, CC2IF, CC3IF, CC4IF | 1, 2, 3, 4 | CW32L012 RM1.4 section 16.10.6, printed p294 (PDF p320) |
| gtim/l012 | ICR.CCOF | CC1OF, CC2OF, CC3OF, CC4OF | 9, 10, 11, 12 | CW32L012 RM1.4 section 16.10.6, printed p294 (PDF p320) |
| gtim/l012 | EGR.CCG | CC1G, CC2G, CC3G, CC4G | 1, 2, 3, 4 | CW32L012 RM1.4 section 16.10.7, printed p295 (PDF p321) |
| gtim/l012 | CCER.CCE | CC1E, CC2E, CC3E, CC4E | 0, 4, 8, 12 | CW32L012 RM1.4 section 16.10.12, printed p302 (PDF p328) |
| gtim/l012 | CCER.CCP | CC1P, CC2P, CC3P, CC4P | 1, 5, 9, 13 | CW32L012 RM1.4 section 16.10.12, printed p302 (PDF p328) |
| gtim/l012 | CCER.CCNP | CC1NP, CC2NP, CC3NP, CC4NP | 3, 7, 11, 15 | CW32L012 RM1.4 section 16.10.12, printed p302 (PDF p328) |
| atim/l012 | DIER.CCIE | CC1IE, CC2IE, CC3IE, CC4IE, CC5IE, CC6IE | 1, 2, 3, 4, 16, 17 | CW32L012 RM1.4 section 17.10.4, printed p378 (PDF p404) |
| atim/l012 | DIER.CCDE | CC1DE, CC2DE, CC3DE, CC4DE, CC5DE, CC6DE | 9, 10, 11, 12, 18, 19 | CW32L012 RM1.4 section 17.10.4, printed p378 (PDF p404) |
| atim/l012 | ISR.CCIF | CC1IF, CC2IF, CC3IF, CC4IF, CC5IF, CC6IF | 1, 2, 3, 4, 16, 17 | CW32L012 RM1.4 section 17.10.5, printed p380 (PDF p406) |
| atim/l012 | ISR.CCOF | CC1OF, CC2OF, CC3OF, CC4OF, CC5OF, CC6OF | 9, 10, 11, 12, 18, 19 | CW32L012 RM1.4 section 17.10.5, printed p380 (PDF p406) |
| atim/l012 | ICR.CCIF | CC1IF, CC2IF, CC3IF, CC4IF, CC5IF, CC6IF | 1, 2, 3, 4, 16, 17 | CW32L012 RM1.4 section 17.10.6, printed p383 (PDF p409) |
| atim/l012 | ICR.CCOF | CC1OF, CC2OF, CC3OF, CC4OF, CC5OF, CC6OF | 9, 10, 11, 12, 18, 19 | CW32L012 RM1.4 section 17.10.6, printed p383 (PDF p409) |
| atim/l012 | EGR.CCG | CC1G, CC2G, CC3G, CC4G, CC5G, CC6G | 1, 2, 3, 4, 16, 17 | CW32L012 RM1.4 section 17.10.7, printed p384 (PDF p410) |
| atim/l012 | CCER.CCE | CC1E, CC2E, CC3E, CC4E, CC5E, CC6E | 0, 4, 8, 12, 16, 20 | CW32L012 RM1.4 section 17.10.14, printed p394 (PDF p420) |
| atim/l012 | CCER.CCP | CC1P, CC2P, CC3P, CC4P, CC5P, CC6P | 1, 5, 9, 13, 17, 21 | CW32L012 RM1.4 section 17.10.14, printed p394 (PDF p420) |
| atim/l012 | CCER.CCNP | CC1NP, CC2NP, CC3NP, CC4NP, CC5NP, CC6NP | 3, 7, 11, 15, 19, 23 | CW32L012 RM1.4 section 17.10.14, printed p394 (PDF p420) |
| atim/l012 | CCER.CCNE | CC1NE, CC2NE, CC3NE, CC4NE, CC5NE, CC6NE | 2, 6, 10, 14, 18, 22 | CW32L012 RM1.4 section 17.10.14, printed p394 (PDF p420) |
| atim/l012 | CR2.OIS | OIS1, OIS2, OIS3, OIS4, OIS5, OIS6 | 8, 10, 12, 14, 16, 18 | CW32L012 RM1.4 section 17.10.2, printed p372 (PDF p398) |
| atim/l012 | CR2.OISN | OIS1N, OIS2N, OIS3N, OIS4N, OIS5N, OIS6N | 9, 11, 13, 15, 17, 19 | CW32L012 RM1.4 section 17.10.2, printed p372 (PDF p398) |
| atim/l012 | AF1.BKVCE | BKVC1E, BKVC2E, BKVC3E, BKVC4E | 1, 2, 3, 4 | CW32L012 RM1.4 section 17.10.30, printed p408 (PDF p434) |
| atim/l012 | AF1.BKVCP | BKVC1P, BKVC2P, BKVC3P, BKVC4P | 10, 11, 12, 13 | CW32L012 RM1.4 section 17.10.30, printed p408 (PDF p434) |
| atim/l012 | AF2.BK2VCE | BK2VC1E, BK2VC2E, BK2VC3E, BK2VC4E | 1, 2, 3, 4 | CW32L012 RM1.4 section 17.10.31, printed p410 (PDF p436) |
| atim/l012 | AF2.BK2VCP | BK2VC1P, BK2VC2P, BK2VC3P, BK2VC4P | 10, 11, 12, 13 | CW32L012 RM1.4 section 17.10.31, printed p410 (PDF p436) |
| gtim/f030 | IER.CC | CC1, CC2, CC3, CC4 | 3, 4, 5, 6 | CW32x030 RM Rev2.5 section 14.8.11, printed p248 (PDF p249) |
| gtim/f030 | ISR.CC | CC1, CC2, CC3, CC4 | 3, 4, 5, 6 | CW32x030 RM Rev2.5 section 14.8.12, printed p249 (PDF p250) |
| gtim/f030 | ICR.CC | CC1, CC2, CC3, CC4 | 3, 4, 5, 6 | CW32x030 RM Rev2.5 section 14.8.13, printed p250 (PDF p251) |
| gtim/f030 | DMA.CC | CC1, CC2, CC3, CC4 | 2, 3, 4, 5 | CW32x030 RM Rev2.5 section 14.8.14, printed p251 (PDF p252) |
| gtim/f030 | CR1.CHPOL | CH1POL, CH2POL, CH3POL, CH4POL | 3, 7, 11, 15 | CW32x030 RM Rev2.5 section 14.8.2, printed p245 (PDF p246) |
| gtim/f030 | CR1.CHFLT | CH1FLT, CH2FLT, CH3FLT, CH4FLT | 0, 4, 8, 12 | CW32x030 RM Rev2.5 section 14.8.2, printed p245 (PDF p246) |
| gtim/f030 | CMMR.CCM | CC1M, CC2M, CC3M, CC4M | 0, 4, 8, 12 | CW32x030 RM Rev2.5 section 14.8.4, printed p246 (PDF p247) |
| atim/f030 | ISR.CAF | C1AF, C2AF, C3AF | 2, 3, 4 | CW32x030 RM Rev2.5 section 15.7.4, printed p298 (PDF p299) |
| atim/f030 | ISR.CBF | C1BF, C2BF, C3BF | 5, 6, 7 | CW32x030 RM Rev2.5 section 15.7.4, printed p298 (PDF p299) |
| atim/f030 | ISR.CAE | C1AE, C2AE, C3AE | 8, 9, 10 | CW32x030 RM Rev2.5 section 15.7.4, printed p298 (PDF p299) |
| atim/f030 | ISR.CBE | C1BE, C2BE, C3BE | 11, 12, 13 | CW32x030 RM Rev2.5 section 15.7.4, printed p298 (PDF p299) |
| atim/f030 | ICR.CAF | C1AF, C2AF, C3AF | 2, 3, 4 | CW32x030 RM Rev2.5 section 15.7.5, printed p300 (PDF p301) |
| atim/f030 | ICR.CBF | C1BF, C2BF, C3BF | 5, 6, 7 | CW32x030 RM Rev2.5 section 15.7.5, printed p300 (PDF p301) |
| atim/f030 | ICR.CAE | C1AE, C2AE, C3AE | 8, 9, 10 | CW32x030 RM Rev2.5 section 15.7.5, printed p300 (PDF p301) |
| atim/f030 | ICR.CBE | C1BE, C2BE, C3BE | 11, 12, 13 | CW32x030 RM Rev2.5 section 15.7.5, printed p300 (PDF p301) |
| atim/f030 | TRIG.CMEA | CM1AE, CM2AE, CM3AE | 1, 2, 3 | CW32x030 RM Rev2.5 section 15.7.8, printed p304 (PDF p305) |
| atim/f030 | FLTR.CCPA | CCP1A, CCP2A, CCP3A | 3, 11, 19 | CW32x030 RM Rev2.5 section 15.7.7, printed p302 (PDF p303) |
| atim/f030 | FLTR.OCMFLTA | OCM1AFLT1A, OCM2AFLT2A, OCM3AFLT3A | 0, 8, 16 | CW32x030 RM Rev2.5 section 15.7.7, printed p302 (PDF p303) |
| atim/f030 | TRIG.CMEB | CM1BE, CM2BE, CM3BE | 4, 5, 6 | CW32x030 RM Rev2.5 section 15.7.8, printed p304 (PDF p305) |
| atim/f030 | FLTR.CCPB | CCP1B, CCP2B, CCP3B | 7, 15, 23 | CW32x030 RM Rev2.5 section 15.7.7, printed p302 (PDF p303) |
| atim/f030 | FLTR.OCMFLTB | OCM1BFLT1B, OCM2BFLT2B, OCM3BFLT3B | 4, 12, 20 | CW32x030 RM Rev2.5 section 15.7.7, printed p302 (PDF p303) |
| dac/l012 | CR0.DMAUDRIE | DMAUDRIE1, DMAUDRIE2 | 13, 29 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.DMAEN | DMAEN1, DMAEN2 | 12, 28 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.MAMP | MAMP1, MAMP2 | 8, 24 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.WAVE | WAVE1, WAVE2 | 6, 22 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.TSEL | TSEL1, TSEL2 | 2, 18 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.TEN | TEN1, TEN2 | 1, 17 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.EN | EN1, EN2 | 0, 16 | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR1.OUT | C1OUT, C2OUT | 0, 1 | CW32L012 RM1.4 section 26.10.14, printed p616 (PDF p642) |
| dac/l012 | SWTRGR.SWTRIG | SWTRIG1, SWTRIG2 | 0, 1 | CW32L012 RM1.4 section 26.10.2, printed p612 (PDF p638) |
| dac/l012 | DHR12RD.DATA | C1DATA, C2DATA | 0, 16 | CW32L012 RM1.4 section 26.10.9, printed p614 (PDF p640) |
| dac/l012 | DHR12LD.DATA | C1DATA, C2DATA | 4, 20 | CW32L012 RM1.4 section 26.10.10, printed p614 (PDF p640) |
| dac/l012 | DHR8RD.DATA | C1DATA, C2DATA | 0, 8 | CW32L012 RM1.4 section 26.10.11, printed p614 (PDF p640) |
| adc/l012 | AWDCR.IN | IN0, IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13, IN14, IN15 | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 | CW32L012 RM1.4 section 25.12.6, printed p592 (PDF p618) |
| adc/l012 | TRIGGER.ATIM_OCREF | ATIMOC1REFC, ATIMOC2REFC, ATIMOC3REFC, ATIMOC4REFC, ATIMOC5REFC, ATIMOC6REFC | 2, 3, 4, 5, 6, 7 | CW32L012 RM1.4 section 25.12.7, printed p593 (PDF p619) |
| adc/l012 | TRIGGER.GTIM_OCREF | GTIM1OC1REFC, GTIM1OC2REFC, GTIM1OC3REFC, GTIM1OC4REFC, GTIM2OC1REFC, GTIM2OC2REFC, GTIM2OC3REFC, GTIM2OC4REFC, GTIM3OC1REFC, GTIM3OC2REFC, GTIM3OC3REFC, GTIM3OC4REFC, GTIM4OC1REFC, GTIM4OC2REFC, GTIM4OC3REFC, GTIM4OC4REFC | 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 24, 25, 26, 27 | CW32L012 RM1.4 section 25.12.7, printed p593 (PDF p619) |
| adc/l012 | TRIGGER.GTIM_TRGO | GTIM1TRGO, GTIM2TRGO, GTIM3TRGO, GTIM4TRGO | 8, 13, 18, 23 | CW32L012 RM1.4 section 25.12.7, printed p593 (PDF p619) |
| adc/l012 | TRIGGER.BTIM_TRGO | BTIM1TRGO, BTIM2TRGO, BTIM3TRGO | 28, 29, 30 | CW32L012 RM1.4 section 25.12.7, printed p593 (PDF p619) |
| vc/l012 | CR2.ATIM_OCREF | ATIMOC1REFC, ATIMOC2REFC, ATIMOC3REFC, ATIMOC4REFC, ATIMOC5REFC, ATIMOC6REFC | 2, 3, 4, 5, 6, 7 | CW32L012 RM1.4 section 27.7.5, printed p630 (PDF p656) |
| vc/l012 | CR2.GTIM_OCREF | GTIM1OC1REFC, GTIM1OC2REFC, GTIM1OC3REFC, GTIM1OC4REFC, GTIM2OC1REFC, GTIM2OC2REFC, GTIM2OC3REFC, GTIM2OC4REFC, GTIM3OC1REFC, GTIM3OC2REFC, GTIM3OC3REFC, GTIM3OC4REFC, GTIM4OC1REFC, GTIM4OC2REFC, GTIM4OC3REFC, GTIM4OC4REFC | 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 24, 25, 26, 27 | CW32L012 RM1.4 section 27.7.5, printed p630 (PDF p656) |
| vc/l012 | CR2.GTIM_TRGO | GTIM1TRGO, GTIM2TRGO, GTIM3TRGO, GTIM4TRGO | 8, 13, 18, 23 | CW32L012 RM1.4 section 27.7.5, printed p630 (PDF p656) |
| vc/l012 | CR2.BTIM_TRGO | BTIM1TRGO, BTIM2TRGO, BTIM3TRGO | 28, 29, 30 | CW32L012 RM1.4 section 27.7.5, printed p630 (PDF p656) |
| flash/l012 | PAGELOCK.LOCK | LOCK0, LOCK1, LOCK2, LOCK3, LOCK4, LOCK5, LOCK6, LOCK7, LOCK8, LOCK9, LOCK10, LOCK11, LOCK12, LOCK13, LOCK14, LOCK15 | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 | CW32L012 RM1.4 section 7.10.3, printed p102 (PDF p128) |
| flash/f030 | PAGELOCK.LOCK | LOCK0, LOCK1, LOCK2, LOCK3, LOCK4, LOCK5, LOCK6, LOCK7, LOCK8, LOCK9, LOCK10, LOCK11, LOCK12, LOCK13, LOCK14, LOCK15 | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 | CW32x030 RM Rev2.5 section 7.9.3, printed p121 (PDF p122) |
| halltim/l012 | STATE.CHS | CH1S, CH2S, CH3S | 4, 5, 6 | CW32L012 RM1.4 section 18.8.9, printed p424 (PDF p450) |
| halltim/l012 | STATE.CHF | CH1F, CH2F, CH3F | 0, 1, 2 | CW32L012 RM1.4 section 18.8.9, printed p424 (PDF p450) |
| i2c/l012 | SIER.AM | AM0, AM1 | 12, 13 | CW32L012 RM1.4 section 23.8.27, printed p562 (PDF p588) |
| i2c/l012 | SISR.AM | AM0, AM1 | 12, 13 | CW32L012 RM1.4 section 23.8.28, printed p564 (PDF p590) |
| i2c/f030 | MATCH.ADDR | ADDR0, ADDR1, ADDR2 | 0, 1, 2 | CW32x030 RM Rev2.5 section 20.7.9, printed p427 (PDF p428) |
| sysctrl/l012 | DEBUG.GTIM | GTIM1, GTIM2, GTIM3, GTIM4 | 1, 2, 3, 4 | CW32L012 RM1.4 section 4.7.18, printed p65 (PDF p91) |
| sysctrl/f030 | DEBUG.GTIM | GTIM1, GTIM2, GTIM3, GTIM4 | 1, 2, 3, 4 | CW32x030 RM Rev2.5 section 4.7.19, printed p88 (PDF p89) |
| adc/f030 | TRIGGER.I2C | I2C1, I2C2 | 13, 14 | CW32x030 RM Rev2.5 section 22.13.7, printed p464 (PDF p465) |
| adc/f030 | TRIGGER.SPI | SPI1, SPI2 | 11, 12 | CW32x030 RM Rev2.5 section 22.13.7, printed p464 (PDF p465) |
| adc/f030 | TRIGGER.UART | UART1, UART2, UART3 | 8, 9, 10 | CW32x030 RM Rev2.5 section 22.13.7, printed p464 (PDF p465) |
| adc/f030 | TRIGGER.BTIM | BTIM1, BTIM2, BTIM3 | 5, 6, 7 | CW32x030 RM Rev2.5 section 22.13.7, printed p464 (PDF p465) |
| adc/f030 | TRIGGER.GTIM | GTIM1, GTIM2, GTIM3, GTIM4 | 1, 2, 3, 4 | CW32x030 RM Rev2.5 section 22.13.7, printed p464 (PDF p465) |
| vc/f030 | CR1.BLANKCHB | BLANKCH1B, BLANKCH2B, BLANKCH3B | 10, 11, 12 | CW32x030 RM Rev2.5 section 23.7.3, printed p482 (PDF p483) |
| opa/l012 | CAL.GTIM_TRGO | GTIM1TRGO, GTIM2TRGO, GTIM3TRGO, GTIM4TRGO | 20, 23, 26, 29 | CW32L012 RM1.4 section 29.6.2, printed p650 (PDF p676) |
| opa/l012 | CAL.ATIM_OCREF | ATIM1OC4, ATIM1OC5, ATIM1OC6 | 17, 18, 19 | CW32L012 RM1.4 section 29.6.2, printed p650 (PDF p676) |
| opa/l012 | CAL.ADC_START | ADC1START1, ADC2START1 | 14, 15 | CW32L012 RM1.4 section 29.6.2, printed p650 (PDF p676) |

## Semantic enum fields

The encoding names below correspond only to documented meanings. Other encodings remain representable as `_RESERVED_<hex>` variants. Count, address, payload, numeric threshold and arithmetic divider fields remain raw integers unless a discrete, audited selection was added. Boolean gates remain booleans.

| IP/version | Register.field | Encodings | Evidence |
| --- | --- | --- | --- |
| adc/l012 | CR.CLK | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8 | CW32L012 RM1.4 section 25.12.1, printed p589 (PDF p615) |
| atim/l012 | CR1.CKD | 0=DIV1, 1=DIV2, 2=DIV4 | CW32L012 RM1.4 section 17.10.1, printed p370 (PDF p396) |
| atim/l012 | CR1.CMS | 0=EDGE_ALIGNED, 1=CENTER_DOWN, 2=CENTER_UP, 3=CENTER_BOTH | CW32L012 RM1.4 section 17.10.1, printed p370 (PDF p396) |
| atim/l012 | CCMR_CAP.CCS | 0=OUTPUT, 1=DIRECT_TI, 2=INDIRECT_TI, 3=TRC | CW32L012 RM1.4 section 17.10.8, printed p386 (PDF p412) |
| atim/l012 | CCMR_CAP.ICPSC | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8 | CW32L012 RM1.4 section 17.10.8, printed p386 (PDF p412) |
| atim/l012 | CCMR_CMP.CCS | 0=OUTPUT, 1=DIRECT_TI, 2=INDIRECT_TI, 3=TRC | CW32L012 RM1.4 section 17.10.9, printed p387 (PDF p413) |
| atim/l012 | CCMR_CMP.OCM | 0=FROZEN, 1=ACTIVE_ON_MATCH, 2=INACTIVE_ON_MATCH, 3=TOGGLE, 4=FORCE_INACTIVE, 5=FORCE_ACTIVE, 6=PWM1, 7=PWM2 | CW32L012 RM1.4 section 17.10.9, printed p387 (PDF p413) |
| dac/l012 | CR0.WAVE | 0=DISABLED, 1=NOISE, 2=TRIANGLE, 3=TRIANGLE_ALT | CW32L012 RM1.4 section 26.10.1, printed p609 (PDF p635) |
| dac/l012 | CR0.TSEL | 0=SOFTWARE, 1=BTIM1_TRGO, 2=BTIM2_TRGO, 3=BTIM3_TRGO, 4=GTIM1_TRGO, 5=GTIM2_TRGO, 6=GTIM3_TRGO, 7=GTIM4_TRGO, 8=ATIM_TRGO, 9=LPTIM_OV, 10=BTIM1_ETR, 11=BTIM2_ETR, 12=BTIM3_ETR, 13=GTIM1_ETR, 14=GTIM2_ETR, 15=GTIM3_ETR | CW32L012 RM1.4 table 26-1, printed p605 |
| dmachannel/f030 | CSR.STATUS | 0=INITIAL, 1=ADDRESS_OUT_OF_RANGE, 2=TRANSFER_STOP, 3=SOURCE_ACCESS_ERROR, 4=DESTINATION_ACCESS_ERROR, 5=COMPLETE | CW32x030 RM Rev2.5 section 8.8.3, printed p139 (PDF p140) |
| dmachannel/f030 | CSR.SIZE | 0=BITS8, 1=BITS16, 2=BITS32 | CW32x030 RM Rev2.5 section 8.8.3, printed p139 (PDF p140) |
| dmachannel/f030 | CSR.TRANS | 0=BULK, 1=BLOCK | CW32x030 RM Rev2.5 section 8.8.3, printed p139 (PDF p140) |
| dmachannel/f030 | TRIG.HARDSRC | 0=UART1_RX, 1=UART1_TX, 2=UART2_RX, 3=UART2_TX, 4=UART3_RX, 5=UART3_TX, 6=SPI1_RX, 7=SPI1_TX, 8=SPI2_RX, 9=SPI2_TX, 10=ADC_CONVERSION, 11=BTIM1_UPDATE, 12=BTIM1_TRIGGER, 13=BTIM2_UPDATE, 14=BTIM2_TRIGGER, 15=BTIM3_UPDATE, 16=BTIM3_TRIGGER, 17=ATIM_SHARED_A, 18=ATIM_SHARED_B, 19=GTIM1_UPDATE, 20=GTIM1_TRIGGER, 21=GTIM1_CH1, 22=GTIM1_CH2, 23=GTIM1_CH3, 24=GTIM1_CH4, 25=GTIM2_UPDATE, 26=GTIM2_TRIGGER, 27=GTIM2_CH1, 28=GTIM2_CH2, 29=GTIM2_CH3, 30=GTIM2_CH4, 31=GTIM3_UPDATE, 32=GTIM3_TRIGGER, 33=GTIM3_CH1, 34=GTIM3_CH2, 35=GTIM3_CH3, 36=GTIM3_CH4, 37=GTIM4_UPDATE, 38=GTIM4_TRIGGER, 39=GTIM4_CH1, 40=GTIM4_CH2, 41=GTIM4_CH3, 42=GTIM4_CH4 | CW32x030 RM Rev2.5 sections 8.2, 8.4.4, 8.7, 8.8.4 and 15.3.6 (printed pp124,130-131,136,140-141,283), table 5-1 |
| dmachannel/f030 | TRIG.TYPE | 0=SOFTWARE, 1=HARDWARE | CW32x030 RM Rev2.5 section 8.8.4, printed p139 (PDF p140) |
| dmachannel/l012 | CSR.STATUS | 0=INITIAL, 1=ADDRESS_OUT_OF_RANGE, 2=TRANSFER_STOP, 3=SOURCE_ACCESS_ERROR, 4=DESTINATION_ACCESS_ERROR, 5=COMPLETE | CW32L012 RM1.4 section 8.8.3, printed p119 (PDF p145) |
| dmachannel/l012 | CSR.SIZE | 0=BITS8, 1=BITS16, 2=BITS32 | CW32L012 RM1.4 section 8.8.3, printed p119 (PDF p145) |
| dmachannel/l012 | CSR.TRANS | 0=BULK, 1=BLOCK | CW32L012 RM1.4 section 8.8.3, printed p119 (PDF p145) |
| dmachannel/l012 | TRIG.HARDSRC | 0=UART1_RX, 1=UART1_TX, 2=UART2_RX, 3=UART2_TX, 4=UART3_RX, 5=UART3_TX, 6=SPI1_RX, 7=SPI1_TX, 8=SPI2_RX, 9=SPI2_TX, 10=SPI3_RX, 11=SPI3_TX, 12=ADC1_SEQUENCE, 13=ADC1_SINGLE, 14=ADC2_SEQUENCE, 15=ADC2_SINGLE, 16=DAC_DHR1_UNDERRUN, 17=DAC_DHR2_UNDERRUN, 18=HALLTIM_EVENT, 19=BTIM1_UPDATE, 20=BTIM1_TRIGGER, 21=BTIM2_UPDATE, 22=BTIM2_TRIGGER, 23=BTIM3_UPDATE, 24=BTIM3_TRIGGER, 25=GTIM1_TRIGGER, 26=GTIM1_UPDATE, 27=GTIM1_CH1, 28=GTIM1_CH2, 29=GTIM1_CH3, 30=GTIM1_CH4, 31=GTIM2_TRIGGER, 32=GTIM2_UPDATE, 33=GTIM2_CH1, 34=GTIM2_CH2, 35=GTIM2_CH3, 36=GTIM2_CH4, 37=GTIM3_TRIGGER, 38=GTIM3_UPDATE, 39=GTIM3_CH1, 40=GTIM3_CH2, 41=GTIM3_CH3, 42=GTIM3_CH4, 43=GTIM4_TRIGGER, 44=GTIM4_UPDATE, 45=GTIM4_CH1, 46=GTIM4_CH2, 47=GTIM4_CH3, 48=GTIM4_CH4, 49=ATIM_UPDATE, 50=ATIM_CH1, 51=ATIM_CH2, 52=ATIM_CH3, 53=ATIM_CH4, 54=ATIM_CH5, 55=ATIM_CH6, 56=ATIM_COM, 57=ATIM_TRIGGER, 58=CORDIC_IDLE, 59=CORDIC_EOC, 60=I2C1_TX, 61=I2C1_RX, 62=I2C2_TX, 63=I2C2_RX | CW32L012 RM1.4 sections 8.2, 8.7 and 8.8.4 (printed pp107,117,121), table 5-1 |
| dmachannel/l012 | TRIG.TYPE | 0=SOFTWARE, 1=HARDWARE | CW32L012 RM1.4 section 8.8.4, printed p121 (PDF p147) |
| gtim/f030 | CMMR.CCM | 0=DISABLED, 1=RISING_EDGE, 2=FALLING_EDGE, 3=BOTH_EDGES, 8=FORCE_LOW, 9=FORCE_HIGH, 10=LOW_ON_MATCH, 11=HIGH_ON_MATCH, 14=PWM_FORWARD, 15=PWM_REVERSE | CW32x030 RM Rev2.5 section 14.8.4, printed p246 (PDF p247) |
| gtim/f030 | CR0.MODE | 0=TIMER, 1=COUNTER, 2=TRIGGER_START, 3=GATED | CW32x030 RM Rev2.5 section 14.8.1, printed p243 (PDF p244) |
| gtim/l012 | CR1.CKD | 0=DIV1, 1=DIV2, 2=DIV4 | CW32L012 RM1.4 section 16.10.1, printed p284 (PDF p310) |
| gtim/l012 | CR1.CMS | 0=EDGE_ALIGNED, 1=CENTER_DOWN, 2=CENTER_UP, 3=CENTER_BOTH | CW32L012 RM1.4 section 16.10.1, printed p284 (PDF p310) |
| gtim/l012 | CCMR_CAP.CCS | 0=OUTPUT, 1=DIRECT_TI, 2=INDIRECT_TI, 3=TRC | CW32L012 RM1.4 section 16.10.8, printed p296 (PDF p322) |
| gtim/l012 | CCMR_CAP.ICPSC | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8 | CW32L012 RM1.4 section 16.10.8, printed p296 (PDF p322) |
| gtim/l012 | CCMR_CMP.CCS | 0=OUTPUT, 1=DIRECT_TI, 2=INDIRECT_TI, 3=TRC | CW32L012 RM1.4 section 16.10.9, printed p297 (PDF p323) |
| gtim/l012 | CCMR_CMP.OCM | 0=FROZEN, 1=ACTIVE_ON_MATCH, 2=INACTIVE_ON_MATCH, 3=TOGGLE, 4=FORCE_INACTIVE, 5=FORCE_ACTIVE, 6=PWM1, 7=PWM2 | CW32L012 RM1.4 section 16.10.9, printed p297 (PDF p323) |
| i2c/l012 | MCR0.CLKSRC | 0=PCLK, 2=LSE, 3=HSI | CW32L012 RM1.4 section 23.8.2, printed p547 (PDF p573) |
| i2c/l012 | MTDR.CMD | 0=TRANSMIT, 1=RECEIVE, 2=STOP, 3=RECEIVE_DISCARD, 4=START_EXPECT_ACK, 5=START_EXPECT_NACK | CW32L012 RM1.4 section 23.8.11, printed p552 (PDF p578) |
| i2c/l012 | SCR0.CLKSRC | 0=PCLK, 2=LSE, 3=LSI | CW32L012 RM1.4 section 23.8.17, printed p557 (PDF p583) |
| spi/f030 | CR1.MODE | 0=FULL_DUPLEX, 1=TRANSMIT_ONLY, 2=RECEIVE_ONLY, 3=HALF_DUPLEX | CW32x030 RM Rev2.5 section 19.8.1, printed p388 (PDF p389) |
| spi/f030 | CR1.BR | 0=DIV2, 1=DIV4, 2=DIV8, 3=DIV16, 4=DIV32, 5=DIV64, 6=DIV128 | CW32x030 RM Rev2.5 section 19.8.1, printed p388 (PDF p389) |
| spi/l012 | CR1.MODE | 0=FULL_DUPLEX, 1=TRANSMIT_ONLY, 2=RECEIVE_ONLY, 3=HALF_DUPLEX | CW32L012 RM1.4 section 22.7.1, printed p516 (PDF p542) |
| sysctrl/f030 | CR0.HCLKPRS | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8, 4=DIV16, 5=DIV32, 6=DIV64, 7=DIV128 | CW32x030 RM Rev2.5 section 4.7.1, printed p69 (PDF p70) |
| sysctrl/f030 | CR0.PCLKPRS | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8 | CW32x030 RM Rev2.5 section 4.7.1, printed p69 (PDF p70) |
| sysctrl/f030 | CR0.SYSCLK | 0=HSI, 1=HSE, 2=PLL, 3=LSI, 4=LSE | CW32x030 RM Rev2.5 section 4.7.1, printed p69 (PDF p70) |
| sysctrl/f030 | HSI.DIV | 5=DIV6, 6=DIV1, 8=DIV2, 9=DIV4, 11=DIV8, 12=DIV10, 13=DIV12, 14=DIV14, 15=DIV16 | CW32x030 RM Rev2.5 section 4.7.4, printed p72 (PDF p73) |
| sysctrl/l012 | CR0.HCLKPRS | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8, 4=DIV16, 5=DIV32, 6=DIV64, 7=DIV128 | CW32L012 RM1.4 section 4.7.1, printed p44 (PDF p70) |
| sysctrl/l012 | CR0.PCLKPRS | 0=DIV1, 1=DIV2, 2=DIV4, 3=DIV8 | CW32L012 RM1.4 section 4.7.1, printed p44 (PDF p70) |
| sysctrl/l012 | CR0.SYSCLK | 0=HSI, 1=HSE, 3=LSI, 4=LSE | CW32L012 RM1.4 section 4.7.1, printed p44 (PDF p70) |
| sysctrl/l012 | HSI.DIV | 0=DIV32, 1=DIV1, 2=DIV2, 3=DIV3, 4=DIV4, 5=DIV5, 6=DIV6, 7=DIV7, 8=DIV8, 9=DIV9, 10=DIV10, 11=DIV12, 12=DIV16, 13=DIV20, 14=DIV24, 15=DIV28 | CW32L012 RM1.4 section 4.7.4, printed p48 (PDF p74) |
| uart/f030 | CR1.OVER | 0=OVERSAMPLE16, 1=OVERSAMPLE8, 2=OVERSAMPLE4, 3=LOW_FREQUENCY | CW32x030 RM Rev2.5 section 18.9.1, printed p355 (PDF p356) |
| uart/f030 | CR1.STOP | 0=STOP1, 1=STOP1P5, 2=STOP2 | CW32x030 RM Rev2.5 section 18.9.1, printed p355 (PDF p356) |
| uart/f030 | CR1.PARITY | 0=NONE, 1=CUSTOM, 2=EVEN, 3=ODD | CW32x030 RM Rev2.5 section 18.9.1, printed p355 (PDF p356) |
| uart/f030 | CR2.SOURCE | 0=PCLK, 1=PCLK_ALT, 2=LSE, 3=LSI | CW32x030 RM Rev2.5 section 18.9.2, printed p356 (PDF p357) |
| uart/l012 | CR1.SOURCE | 0=PCLK, 1=PCLK_ALT, 2=LSE, 3=LSI | CW32L012 RM1.4 section 21.9.1, printed p480 (PDF p506) |
| uart/l012 | CR1.OVER | 0=OVERSAMPLE16, 1=OVERSAMPLE8, 2=OVERSAMPLE4, 3=LOW_FREQUENCY | CW32L012 RM1.4 section 21.9.1, printed p480 (PDF p506) |
| uart/l012 | CR1.STOP | 0=STOP1, 1=STOP1P5, 2=STOP2 | CW32L012 RM1.4 section 21.9.1, printed p480 (PDF p506) |
| uart/l012 | CR1.PARITY | 0=EVEN, 1=ODD | CW32L012 RM1.4 section 21.9.1, printed p480 (PDF p506) |
| uart/l012 | CR2.RXSRC | 0=RXD_PIN, 1=VC1, 2=VC2, 3=VC3, 4=VC4 | CW32L012 RM1.4 section 21.9.2, printed p482 (PDF p508) |

Two qualifications are essential. L012 `CCMR_CMP.OCM` covers the low three bits only: its base-mode names require `OCMH = 0`; `OCMH = 1` selects a separately documented extended bank. `UART.SOURCE` encodings 0 and 1 both select PCLK, while `DAC.WAVE` encodings 2 and 3 both select triangle generation. These hardware aliases are named explicitly rather than mislabeled reserved. F030 SPI BR encoding 7 is reserved; there is no fabricated divide-by-256 choice. L012 I2C master CLKSRC encoding 3 is HSI, while slave encoding 3 is LSI.

## Upstream value representation

The implementation follows the actual [chiptool enum renderer at bcf538a2](https://github.com/embassy-rs/chiptool/blob/bcf538a2e7b8584ae874ee9ab72efb1576fc6152/src/generate/enumm.rs) and checked the [generated STM32 SPI values at stm32-data-caa36afd](https://github.com/embassy-rs/stm32-data-generated/blob/stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83/stm32-metapac/src/peripherals/spi_v1.rs). It selects u8/u16/u32 from field width, emits every reserved discriminant for dense enums, and uses a transparent newtype only when at least 100 encodings and at least half of all encodings are reserved. `from_bits` masks and returns a value directly; `to_bits` and both `From` directions round-trip every field encoding. A reserved value represents observed bits, not permission to write a hardware-reserved encoding.

The DMA hardware adapter now reads indexed TC/TE bits, builds CSR/TRIG with typed modes/selectors, and clears flags from the audited all-ones ICR reset seed. This preserves peer/reserved clear bits and avoids replaying read-only terminal flags. The engine's hardware completion and abort restrictions are unchanged.

An expanded comparison with the frozen v0.17 source found zero offset, bus-width, access, reset-word, read/write-policy or field-layout/access differences across 463 reusable register views and 2,748 scalar field slots. Enum names are supported by the cited manual encodings; raw payload/count/address encodings stay intact.

Validation/build evidence is recorded with the release verification. No hardware execution, electrical measurements or new safe peripheral-DMA endpoint claims are made by this PAC work.

## Independent validation

- Both chip PACs compile with metadata enabled, and `xtask regenerate --check` confirms byte-for-byte YAML → JSON → PAC reproducibility.
- Every one of the 50 production enum fields was checked across all 256 u8 inputs for masked round-trip behavior and reserved-value Debug output.
- All 156 production field arrays, containing 1,075 elements, passed exact-offset, bit-preservation and bounds checks.
- External schema probes exercised 29 cases: 25 intended rejections and four valid layouts. Valid cases included descending field offsets and 12-/32-bit sparse enum representations; generated descending/sparse accessors compiled and ran.
- Comparison with v0.17 confirmed existing scalar field layouts and all 61 earlier arrays were preserved. Eight earlier indexed raw field kinds intentionally became audited enums. No test harnesses or generated artifacts are source deliverables.
