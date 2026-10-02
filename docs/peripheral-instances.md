# CW32L012 peripheral instances, clocks and interrupts

## Scope and evidence

The family YAML enumerates all **50 peripheral views** exposed by the pinned vendor header and SVD, using **28 shared register kinds**, plus the existing **32 physical external IRQs**. Core peripherals (NVIC, SysTick, SCB) and memory regions are outside this peripheral-view list. This is register/metadata coverage; it does not add peripheral HAL drivers or prove hardware operation.

Evidence used in this audit:

- [Pinned CMSIS header](../vendor/cw32l012.h), `*_BASE`, `CW_*` pointer casts, `*_TypeDef`, IRQn_Type, and SYSCTRL bit-position/mask macros.
- [Pinned SVD](../vendor/CW32L012.svd), peripheral baseAddress, headerStructName, derivedFrom, register layouts and interrupt declarations. The header was generated from a vendor SVD; agreement is a consistency check between shipped artifacts, not two independent hardware measurements.
- [CW32L012 user manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf), §§4.7.11–4.7.16 (printed pp.56–63), Table 5-1 (pp.70–71), §25.11 (p.588), and §27.6 (p.626).
- [SDK v1.0.5](https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip), `Libraries/inc/cw32l012_sysctrl.h` clock/reset macros and `Libraries/src/cw32l012_vc.c`.

The YAML is the maintained source; vendor files are audit evidence, not runtime inputs. Pinned source hashes and versions are in [provenance](../cw32-data/sources/provenance.yaml).

## Complete instance inventory

Every address below was checked against both the header's matching `NAME_BASE` and the SVD's `baseAddress`. All 50 agree. The `block` column is the actual CMSIS structure type in lowercase, with the documented VCREF naming correction below. All select version `l012`.

| Instance | Address | block | SVD layout source |
| --- | --- | --- | --- |
| ADC1 | `0x40000000` | `adc` | ADC1 |
| ADC2 | `0x40000100` | `adc` | ADC1 (`derivedFrom`) |
| ATIM | `0x40001400` | `atim` | ATIM |
| BGR | `0x400000FC` | `bgr` | BGR |
| BTIM1 | `0x40004800` | `btim` | BTIM1 |
| BTIM2 | `0x40004840` | `btim` | BTIM1 (`derivedFrom`) |
| BTIM3 | `0x40004880` | `btim` | BTIM1 (`derivedFrom`) |
| CORDIC | `0x40023400` | `cordic` | CORDIC |
| CRC | `0x40023000` | `crc` | CRC |
| DAC | `0x400000C0` | `dac` | DAC |
| DMA | `0x40020000` | `dma` | DMA |
| DMACHANNEL1 | `0x40020020` | `dmachannel` | DMACHANNEL1 |
| DMACHANNEL2 | `0x40020040` | `dmachannel` | DMACHANNEL1 (`derivedFrom`) |
| DMACHANNEL3 | `0x40020060` | `dmachannel` | DMACHANNEL1 (`derivedFrom`) |
| DMACHANNEL4 | `0x40020080` | `dmachannel` | DMACHANNEL1 (`derivedFrom`) |
| EAU | `0x40023200` | `eau` | EAU |
| FLASH | `0x40022000` | `flash` | FLASH |
| GPIOA | `0x48000000` | `gpio` | GPIOA |
| GPIOB | `0x48000100` | `gpio` | GPIOA (`derivedFrom`) |
| GPIOC | `0x48000200` | `gpio` | GPIOA (`derivedFrom`) |
| GPIOF | `0x48000300` | `gpio` | GPIOA (`derivedFrom`) |
| GTIM1 | `0x40001800` | `gtim` | GTIM1 |
| GTIM2 | `0x40001C00` | `gtim` | GTIM1 (`derivedFrom`) |
| GTIM3 | `0x40002400` | `gtim` | GTIM1 (`derivedFrom`) |
| GTIM4 | `0x40002800` | `gtim` | GTIM1 (`derivedFrom`) |
| HALLTIM | `0x40006400` | `halltim` | HALLTIM |
| I2C1 | `0x40005800` | `i2c` | I2C1 |
| I2C2 | `0x40005C00` | `i2c` | I2C1 (`derivedFrom`) |
| IRMOD | `0x40004080` | `irmod` | IRMOD |
| IWDT | `0x40005000` | `iwdt` | IWDT |
| LPTIM | `0x40006000` | `lptim` | LPTIM |
| LVD | `0x400000A4` | `lvd` | LVD |
| OPA1 | `0x400000B0` | `opa` | OPA1 |
| OPA2 | `0x400000B8` | `opa` | OPA1 (`derivedFrom`) |
| RAM | `0x40022400` | `ram` | RAM |
| RTC | `0x40004400` | `rtc` | RTC |
| SPI1 | `0x40000800` | `spi` | SPI1 |
| SPI2 | `0x40000400` | `spi` | SPI1 (`derivedFrom`) |
| SPI3 | `0x40003400` | `spi` | SPI1 (`derivedFrom`) |
| SYSCTRL | `0x40004000` | `sysctrl` | SYSCTRL |
| UART1 | `0x40000C00` | `uart` | UART1 |
| UART2 | `0x40001000` | `uart` | UART1 (`derivedFrom`) |
| UART3 | `0x40002000` | `uart` | UART1 (`derivedFrom`) |
| VC1 | `0x40000084` | `vc` | VC1 |
| VC12REF | `0x40000080` | `vcref` | VC12REF |
| VC2 | `0x40000094` | `vc` | VC1 (`derivedFrom`) |
| VC3 | `0x40000184` | `vc` | VC1 (`derivedFrom`) |
| VC34REF | `0x40000180` | `vcref` | VC12REF (`derivedFrom`) |
| VC4 | `0x40000194` | `vc` | VC1 (`derivedFrom`) |
| WWDT | `0x40005400` | `wwdt` | WWDT |

### Address conventions and source differences

- `BGR` has header/SVD base `0x400000FC` and `CR` offset 0. The manual §25.11 instead describes an analog region base of `0x40000000` plus offset `0xFC`. Both identify the same absolute register address; they must not be added together twice.
- `VC1/2/3/4` have header/SVD bases `0x40000084/94/184/194`, with `CR0` at offset 0. Manual §27.6 uses bases `0x40000080/90/180/190` with `CR0` at offset 4. Again, the absolute register addresses agree.
- SVD `VC12REF.headerStructName` is `VC2REF`; the actual header declares `VCREF_TypeDef` and uses it for both `CW_VC12REF` and `CW_VC34REF`. The normalized kind is therefore `vcref`. SVD `VC34REF` derives from `VC12REF`, so both use the same one-register layout. This is a vendor naming discrepancy, not a second incompatible IP version.

## Shared IP validation

The following groups share the exact vendor C typedef, and all non-root instances in the SVD use `derivedFrom` without local register overrides. Thus reuse is based on an explicit layout contract, not similar-looking peripheral names. Single-instance kinds are listed in the inventory and need no cross-instance equivalence assumption.

| Shared kind | Instances | SVD root / header type |
| --- | --- | --- |
| `adc/l012` | ADC1, ADC2 | ADC1 / `ADC_TypeDef` |
| `btim/l012` | BTIM1, BTIM2, BTIM3 | BTIM1 / `BTIM_TypeDef` |
| `dmachannel/l012` | DMACHANNEL1, DMACHANNEL2, DMACHANNEL3, DMACHANNEL4 | DMACHANNEL1 / `DMACHANNEL_TypeDef` |
| `gpio/l012` | GPIOA, GPIOB, GPIOC, GPIOF | GPIOA / `GPIO_TypeDef` |
| `gtim/l012` | GTIM1, GTIM2, GTIM3, GTIM4 | GTIM1 / `GTIM_TypeDef` |
| `i2c/l012` | I2C1, I2C2 | I2C1 / `I2C_TypeDef` |
| `opa/l012` | OPA1, OPA2 | OPA1 / `OPA_TypeDef` |
| `spi/l012` | SPI1, SPI2, SPI3 | SPI1 / `SPI_TypeDef` |
| `uart/l012` | UART1, UART2, UART3 | UART1 / `UART_TypeDef` |
| `vc/l012` | VC1, VC2, VC3, VC4 | VC1 / `VC_TypeDef` |
| `vcref/l012` | VC12REF, VC34REF | VC12REF / `VCREF_TypeDef` |

GPIOA/B/C/F retain their existing implemented/pulldown masks: shared layout does not imply identical bonded or implemented pins. ADC/GTIM and other shared layouts likewise do not by themselves establish identical channel capabilities, pin routes or safe ownership.

### DMA overlapping register views

`DMA` spans `0x40020000..0x4002009F`. Each `DMACHANNELn` is a 0x20-byte view inside that same controller, at `DMA_BASE + n * 0x20` for n=1..4. Its eight registers alias `DMA.CSRn`, `CNTn`, `SRCADDRn`, `DSTADDRn`, `TRIGn`, `CCNTn`, `CSRCADDRn`, and `CDSTADDRn`, respectively. The audit compared all 32 alias pairs and found identical relative offsets, sizes, access modes and fields in the SVD, with matching header layout.

Each channel records `ownership_parent: DMA`. This is an overlap/ownership relationship, not an independently owned hardware block or an extra DMA engine. A future safe HAL must partition the parent deliberately and coordinate shared IRQ/clock/reset state; it must not hand out an unrestricted DMA owner alongside unrestricted channel owners. PAC raw views remain available for each vendor instance. Merely emitting ownership metadata does not create such a safe HAL API.

In v0.7, HAL build.rs enforces this boundary by omitting `ownership_parent` children from its safe singleton set. The remaining peripheral and audited pin fields are official Embassy `Peri<'static, T>` values, and drivers preserve their exclusive lifetimes. GPIO port and SYSCTRL resources are also withheld from the public singleton set because the HAL manages their shared registers. This does not constitute a DMA partitioning driver.

## Clock-enable and reset associations

The `clock_gate` and `reset` mappings identify `{peripheral, register, field, bit}`. All non-null mappings below refer to peripheral `SYSCTRL`. They record control locations, not a full clock tree, selected clock source, clock frequency, enable sequence, or register-write recipe. Existing GPIO `clock_bit` values remain solely for compatibility with the HAL's AHBEN GPIO path; other peripherals do not reuse that ambiguous field.

| Clock register | Offset | Reset register | Offset | Evidence |
| --- | --- | --- | --- | --- |
| AHBEN | 0x30 | AHBRST | 0x40 | RM §§4.7.11, 4.7.14 |
| APBEN1 | 0x38 | APBRST1 | 0x48 | RM §§4.7.12, 4.7.15 |
| APBEN2 | 0x34 | APBRST2 | 0x44 | RM §§4.7.13, 4.7.16 |

For every row, the clock and reset bit positions were checked against header `SYSCTRL_<register>_<field>_Pos` and SVD fields; the manual establishes their meaning:

- Clock controls are enabled by 1. AHBEN/APBEN1/APBEN2 writes require KEY[31:16] = `0x5A5A`.
- Reset controls are **active low**: 0 holds the module in reset; 1 releases reset. These reset registers do not use the clock registers' KEY field. Do not apply a generic active-high reset helper.
- Shared gate/reset rows affect every listed instance. Resetting one can disrupt the others; mappings must not be interpreted as per-instance ownership or independent reference counts.
- “Configuration clock” is not interchangeable with a functional clock. FLASH has a configuration gate; VC, UART, DAC, OPA, RTC, IWDT, I2C and LPTIM also require attention to their documented independent functional/selected clocks. This metadata intentionally does not claim to model those complete dependencies.

| Instances | Clock field [bit] | Reset field [bit] |
| --- | --- | --- |
| DMA, DMACHANNEL1, DMACHANNEL2, DMACHANNEL3, DMACHANNEL4 | `AHBEN.DMA[0]` | `AHBRST.DMA[0]` |
| FLASH | `AHBEN.FLASH[1]` | `AHBRST.FLASH[1]` |
| CRC | `AHBEN.CRC[2]` | `AHBRST.CRC[2]` |
| EAU | `AHBEN.EAU[3]` | `AHBRST.EAU[3]` |
| GPIOA | `AHBEN.GPIOA[4]` | `AHBRST.GPIOA[4]` |
| GPIOB | `AHBEN.GPIOB[5]` | `AHBRST.GPIOB[5]` |
| GPIOC | `AHBEN.GPIOC[6]` | `AHBRST.GPIOC[6]` |
| GPIOF | `AHBEN.GPIOF[7]` | `AHBRST.GPIOF[7]` |
| CORDIC | `AHBEN.CORDIC[8]` | `AHBRST.CORDIC[8]` |
| ADC1, ADC2 | `APBEN1.ADC[0]` | `APBRST1.ADC[0]` |
| VC1, VC12REF, VC2, VC3, VC34REF, VC4 | `APBEN1.VC[1]` | `APBRST1.VC[1]` |
| SPI1 | `APBEN1.SPI1[2]` | `APBRST1.SPI1[2]` |
| UART1 | `APBEN1.UART1[3]` | `APBRST1.UART1[3]` |
| UART2 | `APBEN1.UART2[4]` | `APBRST1.UART2[4]` |
| ATIM | `APBEN1.ATIM[5]` | `APBRST1.ATIM[5]` |
| GTIM1 | `APBEN1.GTIM1[6]` | `APBRST1.GTIM1[6]` |
| GTIM2 | `APBEN1.GTIM2[7]` | `APBRST1.GTIM2[7]` |
| UART3 | `APBEN1.UART3[8]` | `APBRST1.UART3[8]` |
| GTIM3 | `APBEN1.GTIM3[11]` | `APBRST1.GTIM3[11]` |
| GTIM4 | `APBEN1.GTIM4[12]` | `APBRST1.GTIM4[12]` |
| SPI2 | `APBEN1.SPI2[13]` | `APBRST1.SPI2[13]` |
| SPI3 | `APBEN1.SPI3[14]` | `APBRST1.SPI3[14]` |
| RTC | `APBEN2.RTC[1]` | `APBRST2.RTC[1]` |
| BTIM1, BTIM2, BTIM3 | `APBEN2.BTIM123[2]` | `APBRST2.BTIM123[2]` |
| IWDT | `APBEN2.IWDT[4]` | `APBRST2.IWDT[4]` |
| WWDT | `APBEN2.WWDT[5]` | `APBRST2.WWDT[5]` |
| I2C1 | `APBEN2.I2C1[6]` | `APBRST2.I2C1[6]` |
| LPTIM | `APBEN2.LPTIM[7]` | `APBRST2.LPTIM[7]` |
| OPA1, OPA2 | `APBEN2.OPA[9]` | `APBRST2.OPA[9]` |
| DAC | `APBEN2.DAC[10]` | `APBRST2.DAC[10]` |
| I2C2 | `APBEN2.I2C2[11]` | `APBRST2.I2C2[11]` |
| HALLTIM | `APBEN2.HALLTIM[12]` | `APBRST2.HALLTIM[12]` |

The manual abbreviates APBEN2/APBRST2 bit 2 as `BTIM`; the header and SVD field name is `BTIM123`. YAML uses that canonical field name. The manual explicitly says BTIM1/2/3 share this bit (§14 programming examples as well as §§4.7.13/16).

The VC gate includes VC12REF and VC34REF: manual §27.6 places both reference registers in the VC module; SDK `VC_Init` enables `__SYSCTRL_VC_CLK_ENABLE()` before `VC_SetRefVoltage` accesses either reference register. The reference divider is shared by a comparator pair. The gate/reset control is shared by the complete VC module, not independently controlled by the two reference instances.

### Explicit nulls and boundaries

The following have `clock_gate: null` and `reset: null`. Null means no asserted SYSCTRL gate/reset association in this data; it must not automatically be interpreted as “always on,” “no clock needed,” or “safe to reset independently.”

| Instance | Audited boundary |
| --- | --- |
| BGR | No BGR field in SYSCTRL gate/reset registers. Manual §§3.1, 25.11/25.12.19 describes the shared analog reference and local TSEN. SDK ADC code accesses BGR, but does not establish a separate BGR bus-gate/reset contract. No ADC gate is guessed for it. |
| IRMOD | No IRMOD field in SYSCTRL gate/reset registers. Manual §24.3.2 requires enabling the chosen carrier/data timers or UART and output GPIO, then configuring IRMOD. Its inputs are runtime selections, so no one fixed gate is assigned. |
| LVD | No LVD field in SYSCTRL gate/reset registers. Manual §§28.3.3–28.5 uses LVD local enable and SYSCLK/LSI filter-clock selection, which are not SYSCTRL peripheral bus-gate bits. |
| RAM | No RAM field in SYSCTRL gate/reset registers. Manual §2.7 explicitly treats SRAM separately from the initially disabled peripherals. The RAM control block is not assigned FLASH's configuration gate merely because it shares FLASHRAM IRQ. |
| SYSCTRL | No self-gate/reset field in the three SYSCTRL bus-gate/reset registers. Oscillator controls and system resets are separate mechanisms. |

## IRQ topology

`GLOBAL` denotes the aggregate interrupt output documented for an instance, not a separate interrupt per status bit. `DMA.CH1`–`CH4` identify channel outputs in the aggregate DMA view; `DMACHANNELn.GLOBAL` describes the same source through the alias view. `SYSCTRL.CLKFAULT` is the separate running-clock-failure output. These are normalized metadata labels, not names of physical pins or generated ISR implementations.

| Physical IRQ | Number | Peripheral signal bindings |
| --- | --- | --- |
| WDT | 0 | IWDT.GLOBAL, WWDT.GLOBAL |
| LVD | 1 | LVD.GLOBAL |
| RTC | 2 | RTC.GLOBAL |
| FLASHRAM | 3 | FLASH.GLOBAL, RAM.GLOBAL |
| SYSCTRL | 4 | SYSCTRL.GLOBAL |
| GPIOA | 5 | GPIOA.GLOBAL |
| GPIOB | 6 | GPIOB.GLOBAL |
| GPIOC | 7 | GPIOC.GLOBAL |
| GPIOF | 8 | GPIOF.GLOBAL |
| DMACH12 | 9 | DMA.CH1, DMA.CH2, DMACHANNEL1.GLOBAL, DMACHANNEL2.GLOBAL |
| DMACH34 | 10 | DMA.CH3, DMA.CH4, DMACHANNEL3.GLOBAL, DMACHANNEL4.GLOBAL |
| CORDIC | 11 | CORDIC.GLOBAL |
| ADC1 | 12 | ADC1.GLOBAL |
| ATIM | 13 | ATIM.GLOBAL |
| VC13 | 14 | VC1.GLOBAL, VC3.GLOBAL |
| VC24 | 15 | VC2.GLOBAL, VC4.GLOBAL |
| GTIM1 | 16 | GTIM1.GLOBAL |
| GTIM2 | 17 | GTIM2.GLOBAL |
| GTIM34 | 18 | GTIM3.GLOBAL, GTIM4.GLOBAL |
| LPTIM | 19 | LPTIM.GLOBAL |
| BTIM1 | 20 | BTIM1.GLOBAL |
| BTIM2 | 21 | BTIM2.GLOBAL |
| BTIM3_HALLTIM | 22 | BTIM3.GLOBAL, HALLTIM.GLOBAL |
| I2C1 | 23 | I2C1.GLOBAL |
| I2C2 | 24 | I2C2.GLOBAL |
| SPI1 | 25 | SPI1.GLOBAL |
| SPI23 | 26 | SPI2.GLOBAL, SPI3.GLOBAL |
| UART1 | 27 | UART1.GLOBAL |
| UART2 | 28 | UART2.GLOBAL |
| UART3 | 29 | UART3.GLOBAL |
| ADC2_DAC | 30 | ADC2.GLOBAL, DAC.GLOBAL |
| CLKFAULT | 31 | SYSCTRL.CLKFAULT |

The physical vector table stays unique. Multiple endpoints refer to the same IRQ for WDT, FLASHRAM, DMACH12, DMACH34, VC13, VC24, GTIM34, BTIM3_HALLTIM, SPI23 and ADC2_DAC. Handlers must inspect the relevant status flags to demultiplex sources; the association table is not a type-level binding proof.

### v0.7 runtime connection

The PAC generator emits `rt.rs` and `device.x` from this physical IRQ inventory. With `rt`, `cortex-m-rt` uses the generated external vector array and linker-provided default handler aliases. Vector positions follow IRQ numbers; if a generated device has gaps, reserved slots stay zero instead of compacting subsequent IRQs. The current real chip's external IRQ range is contiguous 0–31.

The HAL uses the official Embassy `interrupt_mod!` to generate sealed type-level IRQ identities, priority/NVIC operations, and Handler/Binding contracts. Its `bind_interrupts!` macro requires `rt` and emits actual ISR calls plus proofs for precisely those handler/IRQ pairs. A shared-vector handler list is called synchronously in declaration order on every IRQ occurrence. This is dispatch infrastructure; handlers still own pending-flag checks, acknowledgement, wakeups and correct interrupt enabling. The existing FOC drivers remain polling drivers and do not automatically install these handlers. A custom bootloader or replacement vector table must preserve the dispatch contract.

The public HAL association tables require `metadata`; HAL build-time metadata remains enabled independently. Raw PAC re-export through the HAL requires `unstable-pac`. The [Embassy comparison](embassy-api-alignment.md) distinguishes these API changes from missing DMA and asynchronous peripheral drivers. Runtime compilation/linking evidence and any limitations belong to the [validation report](validation.md); no silicon interrupt timing is claimed here.

Two vendor SVD omissions are completed using the header plus manual Table 5-1:

1. The SVD DMA controller lists DMACH12 only, although channels 3/4 declare DMACH34. DMA's aggregate view is associated with both physical vectors through all four channel signals.
2. The SVD SYSCTRL instance lists SYSCTRL/IRQ4 only and the SVD lacks IRQ31 altogether. Header `CLKFAULT_IRQn = 31` and manual Table 5-1 establish SYSCTRL's separate HSE/LSE running-failure interrupt. The manual calls IRQ4 `RCC` and IRQ31 `FAULT`; YAML retains the header's canonical names `SYSCTRL` and `CLKFAULT`. IRQ4 covers ready/start-up-failure events; IRQ31 covers HSE/LSE failures while running.

No peripheral-specific IRQ is invented for BGR, CRC, EAU, IRMOD, OPA1/2 or VC12REF/34REF. Local analog reference use or shared gate membership does not create an additional IRQ source.

## Audit results and remaining limits

- 50/50 header pointer instances and SVD peripheral names match the family inventory.
- 50/50 absolute peripheral bases agree between header and SVD.
- All shared layouts are explicit header typedef reuse and SVD derivation; VCREF spelling is documented above.
- All 32 physical external IRQ identities are retained, with aggregate instance associations including all shared vectors.
- 45 views have an audited SYSCTRL gate/reset association, including aliases/shared references; five explicit nulls retain the limitations above.
- All 32 DMA alias register pairs agree in offset/size/access/fields.

This audit covers instance wiring, not package bonding, pin multiplexing, DMA request muxes, complete functional clock trees, HAL drivers, safe DMA ownership APIs, or silicon validation. Those claims require their own data and tests.
