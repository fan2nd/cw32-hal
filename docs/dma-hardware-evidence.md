# DMA hardware and generated-route evidence

This audit covers CW32L012C8 and CW32F030C8 independently. It establishes documented programming interfaces, not measured silicon behavior. No STM32 register, interrupt-clear, disable, circular-buffer, priority, or request-mux behavior is imported.

## Primary sources and pins

- [CW32L012 user manual 1.4, June 2026](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf): chapter 8, printed pp106–123 (PDF pp132–149); interrupt table 5-1. SHA-256 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
- [CW32x030 user manual Rev2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf): chapter 8, printed pp123–141 (PDF pp124–142); interrupt table 5-1; ATIM §15.3.6, p283. SHA-256 `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`.
- Pinned [L012 header](../vendor/cw32l012.h), [L012 SVD](../vendor/CW32L012.svd), [F030 header](../vendor/cw32f030/cw32f030.h), and [F030 SVD](../vendor/cw32f030/CW32F030.svd) corroborate addresses and external IRQ numbers. Vendor artifacts are not independent hardware measurements.
- [Embassy STM32 DMA at b12a6d9](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dma), particularly `mod.rs` and `dma_bdma/mod.rs`, is a design reference only. Its channel ownership, IRQ-bound instances, per-channel state, register-before-check waker sequence, transfer guard, and memory fences are useful architectural precedents. Its EN polling and Drop reset sequences depend on different hardware and are not CW32 evidence.

The manuals were fetched and their hashes listed above checked during this audit. A targeted official-site errata search on 2026-10-03 found no separately published CW32 DMA erratum that resolves the stop/drain question; this is not a claim that no errata exist.

## Channels, addresses, clocks and IRQs

Both controllers are at `0x40020000`. ISR is `+0x00`, ICR `+0x04`. Channel hardware number `y` has its bank at `+0x20*y`; PAC `DMA.ch(index)` uses `index=y-1`. L012 has four banks; F030 has five. This follows each device's chapter 8 register list, not a family heuristic.

| Chip | Channel alias | CH index | Address | External IRQ |
| --- | --- | ---: | --- | --- |
| L012 | DMACHANNEL1 | 0 | 0x40020020 | DMACH12 = 9 |
| L012 | DMACHANNEL2 | 1 | 0x40020040 | DMACH12 = 9 |
| L012 | DMACHANNEL3 | 2 | 0x40020060 | DMACH34 = 10 |
| L012 | DMACHANNEL4 | 3 | 0x40020080 | DMACH34 = 10 |
| F030 | DMACHANNEL1 | 0 | 0x40020020 | DMACH1 = 9 |
| F030 | DMACHANNEL2 | 1 | 0x40020040 | DMACH23 = 10 |
| F030 | DMACHANNEL3 | 2 | 0x40020060 | DMACH23 = 10 |
| F030 | DMACHANNEL4 | 3 | 0x40020080 | DMACH45 = 11 |
| F030 | DMACHANNEL5 | 4 | 0x400200a0 | DMACH45 = 11 |

Each bank has CSR `+0`, CNT `+4`, SRCADDR `+8`, DSTADDR `+12`, TRIG `+16`. L012 additionally has read-only CCNT `+20`, CSRCADDR `+24`, CDSTADDR `+28`; these three registers are not modeled for F030. All channels share the controller's `SYSCTRL.AHBEN.DMA[0]` gate and active-low `SYSCTRL.AHBRST.DMA[0]` reset. Resetting a channel through that bit would reset every channel. Existing alias views retain `ownership_parent: DMA`; they do not become independent controller owners.

For index `i`, TC is ISR bit `4*i` and TE is bit `4*i+1`. ICR has the corresponding R1W0 fields: zero clears, one preserves. Reserved ICR bits have reset value one and must remain at that default. A shared-vector ISR must inspect each participating channel and clear only flags it handles. It must not disable the shared NVIC vector merely because one channel completed, nor indiscriminately clear its peer's flags. There is no documented half-transfer interrupt.

## Transfer behavior

| Property | L012 | F030 |
| --- | --- | --- |
| Width encoding | SIZE 0/1/2 = 8/16/32 bits; 3 undocumented | Same encodings |
| Increment | Separate SRCINC/DSTINC; step 1/2/4 selected by SIZE | Separate SRCINC/DSTINC; matching-width examples |
| Width qualification | §8.8.3 says width affects Flash/SRAM memory; increment affects all peripherals | §8.5.1 says source and destination widths must match |
| Count | CNT 1..65535 blocks; CCNT reports remaining blocks | CNT 1..65535 items and decrements during transfer |
| Block size | REPEAT nonzero, four bits; total CNT×REPEAT; paced endpoints require 1 | §8.8.5 requires REPEAT=1 |
| Conservative common setup | REPEAT=1 | REPEAT=1 |
| Priority | Fixed by hardware channel number, lowest number highest | Same, five channels |
| Automatic restart | CSR.RESTART exists | No restart/circular field modeled |
| Per-block address reload | CSR.SRCLOAD/DSTLOAD | Not modeled |

L012 §8.5.2 says REPEAT becomes zero at completion, while §8.8.5 plus the separate current-count registers describe configured and current values separately. Do not build restart or residual-count guarantees from that ambiguous prose. Reprogram the count/block size for each one-shot operation. The common implementation can deliberately use only single-item blocks.

TYPE=0 selects software trigger, TYPE=1 hardware trigger. TRANS=1 selects BLOCK; TRANS=0 selects BULK. Software BLOCK needs one SOFTSRC write to run the entire configured count, inserting arbitration gaps after each block. Hardware BLOCK transfers one block for each hardware request. BULK completes the entire configured count after one trigger without arbitration gaps and can starve CPU or higher-priority channels targeting the same bus device. BLOCK is the conservative common mode. The priority is fixed, not software configurable.

Source and destination may be memory or peripheral addresses; memory-to-memory is explicitly supported by each overview. The source/destination register notes prohibit DMA, Flash-controller, and RAM-controller **registers**, not the Flash/SRAM memory arrays. The nominal 32-bit address field does not make every address valid, writable, correctly aligned, or appropriate for a chosen transfer width. Peripheral FIFOs/data registers require device-specific access width, request-generation and error/stop coordination.

## Completion, errors and the stop boundary

Both manuals document these CSR.STATUS values:

| Value | Meaning |
| ---: | --- |
| 0 | Initial state |
| 1 | Address outside addressing range |
| 2 | Transfer aborted by a stop request |
| 3 | Source-address access error |
| 4 | Destination-address access error |
| 5 | Transfer completed |

STATUS is a diagnostic value, not a BUSY flag. The manuals call it RW but give no useful software-write protocol. L012's header/SVD agree with RW. F030's SVD calls STATUS RO; the pre-existing conservative RO access is retained and the discrepancy is explicit. HAL logic should read it, not attempt to clear or drive a state machine by writing it.

TC means the configured data transfer completed correctly. TE can report the error causes above. Clearing TC/TE does not itself prove a channel is idle. TRIG.SOFTSRC reads zero on completion, reads one during an ongoing software transfer, starts on writing one, and ignores a write of zero. TRIG therefore remains a mixed configuration/action register; ordinary read-modify-write could replay a trigger.

Crucially, CSR.EN is documented only as disable/enable. Neither pinned manual describes an acknowledgement delay, EN self-clear, a BUSY register, a software abort sequence, completion of an outstanding bus write before EN reads zero, or terminal-error draining. The presence of STATUS=2 does not identify a software-generated stop protocol. A compiler fence, DMB or DSB does not establish that another bus master's writes have drained.

Consequences for a Rust DMA interface:

- Normal one-shot TC, with automatic restart disabled and no independently armed retrigger, is the documented successful-completion boundary.
- Clearing EN alone must not authorize reclaiming a borrowed destination or source, or restarting the channel over another buffer. The same restriction applies to timeout/cancel/error paths without additional silicon evidence.
- A safe static-owned-buffer design can quarantine the buffer and channel on incomplete cancellation/error, preserving ownership even if a future is forgotten. Static lifetime alone is insufficient if the API hands the mutable buffer back prematurely.
- A raw peripheral-transfer API needs an explicit unsafe caller contract for endpoint validity, width, request generation, rearming, and post-cancel lifetime. Merely exposing typed request selectors does not discharge that contract.
- L012 RESTART is a repeat facility, not evidence for an Embassy-style safe streaming ring. It has no documented half-transfer event or independent producer cursor. F030 does not gain circular support by analogy.

No safe drain, bounded stop latency, restart-after-error, circular ring, or hardware-validation claim is made by this audit.

### Finite ADC endpoint proof

L012 RM 25.5.2 says CONT=0 performs exactly the configured sequence, stores each
conversion in its matching RESULT0–RESULT7 slot, sets EOS after the final slot,
and clears START. RM 25.12.8 gives independent DMAEOS and DMAEOC request enables;
the HAL uses only DMAEOS with the metadata-derived ADCx_SEQUENCE selector.
RESULT slots are at offsets 0x30 + 4*n (RM 25.12.11–25.12.18). One EOS can
therefore trigger a finite BULK transfer with 32-bit source/destination increments
over the stable result bank. External triggers, CONT and DMAEOC are disabled.

F030 RM 22.5.1 says MODE=0 performs one conversion to RESULT0 and clears START.
RM 22.13.2 says DMAEN requests DMA after each conversion; it does not provide an
EOS request or promise queuing of a complete sequence's requests. The initial
safe endpoint therefore uses exactly one BLOCK word from RESULT0 (offset 0x20,
RM 22.13.12), with source increment disabled. MODE=4 multi-slot DMA is rejected;
its four-byte result stride is not emulated as a fixed-address packed stream.

Both endpoints use native 32-bit register reads and static owned destinations.
The ADC request is enabled only after DMA validation/programming and publication
of a persistent ADC/channel lease. Clean DMA TC establishes endpoint completion;
error, timeout, Drop or controller reset without TC retains static inputs,
destination and ADC enable, and permanently poisons both drivers. These software
contracts do not claim measured conversion timing or outstanding-read draining.

## Hardware requests and selector identity

HARDSRC is six bits in each channel's TRIG. Each manual specifies one list for every `y` in its supported channel range; there is no per-channel mux restriction in these sources. Consequently the controller metadata's requests are uniformly available on all its listed channels. Selector zero is UART1_RX, not “no request.” Software trigger selection is TYPE=0, independently of HARDSRC.

The generated names below use the maintained peripheral instance name plus a signal. DAC channel requests use the single `DAC` peripheral; F030 ATIM_SHARED_A/B intentionally describe shared sources. F030 §8.4.4 table 8-2 abbreviates their update behavior inconsistently, but §8.8.4 and §15.3.6 agree: source A combines CH1A/2A/3A/4, source B CH1B/2B/3B, and CCDS selects compare versus update triggering. They must not be represented as independent channel or update request selectors.

Peripheral request enables, FIFO state, timers' DMA-enables/CCDS, ADC conversion mode and source event acknowledgement still belong to each peripheral's driver. Selecting HARDSRC neither enables those generators nor establishes safe buffer pacing.

### CW32L012C8 request table

| HARDSRC | Generated signal |
| ---: | --- |
| 0 | UART1_RX |
| 1 | UART1_TX |
| 2 | UART2_RX |
| 3 | UART2_TX |
| 4 | UART3_RX |
| 5 | UART3_TX |
| 6 | SPI1_RX |
| 7 | SPI1_TX |
| 8 | SPI2_RX |
| 9 | SPI2_TX |
| 10 | SPI3_RX |
| 11 | SPI3_TX |
| 12 | ADC1_SEQUENCE |
| 13 | ADC1_SINGLE |
| 14 | ADC2_SEQUENCE |
| 15 | ADC2_SINGLE |
| 16 | DAC_DHR1_UNDERRUN |
| 17 | DAC_DHR2_UNDERRUN |
| 18 | HALLTIM_EVENT |
| 19 | BTIM1_UPDATE |
| 20 | BTIM1_TRIGGER |
| 21 | BTIM2_UPDATE |
| 22 | BTIM2_TRIGGER |
| 23 | BTIM3_UPDATE |
| 24 | BTIM3_TRIGGER |
| 25 | GTIM1_TRIGGER |
| 26 | GTIM1_UPDATE |
| 27 | GTIM1_CH1 |
| 28 | GTIM1_CH2 |
| 29 | GTIM1_CH3 |
| 30 | GTIM1_CH4 |
| 31 | GTIM2_TRIGGER |
| 32 | GTIM2_UPDATE |
| 33 | GTIM2_CH1 |
| 34 | GTIM2_CH2 |
| 35 | GTIM2_CH3 |
| 36 | GTIM2_CH4 |
| 37 | GTIM3_TRIGGER |
| 38 | GTIM3_UPDATE |
| 39 | GTIM3_CH1 |
| 40 | GTIM3_CH2 |
| 41 | GTIM3_CH3 |
| 42 | GTIM3_CH4 |
| 43 | GTIM4_TRIGGER |
| 44 | GTIM4_UPDATE |
| 45 | GTIM4_CH1 |
| 46 | GTIM4_CH2 |
| 47 | GTIM4_CH3 |
| 48 | GTIM4_CH4 |
| 49 | ATIM_UPDATE |
| 50 | ATIM_CH1 |
| 51 | ATIM_CH2 |
| 52 | ATIM_CH3 |
| 53 | ATIM_CH4 |
| 54 | ATIM_CH5 |
| 55 | ATIM_CH6 |
| 56 | ATIM_COM |
| 57 | ATIM_TRIGGER |
| 58 | CORDIC_IDLE |
| 59 | CORDIC_EOC |
| 60 | I2C1_TX |
| 61 | I2C1_RX |
| 62 | I2C2_TX |
| 63 | I2C2_RX |

### CW32F030C8 request table

| HARDSRC | Generated signal |
| ---: | --- |
| 0 | UART1_RX |
| 1 | UART1_TX |
| 2 | UART2_RX |
| 3 | UART2_TX |
| 4 | UART3_RX |
| 5 | UART3_TX |
| 6 | SPI1_RX |
| 7 | SPI1_TX |
| 8 | SPI2_RX |
| 9 | SPI2_TX |
| 10 | ADC_CONVERSION |
| 11 | BTIM1_UPDATE |
| 12 | BTIM1_TRIGGER |
| 13 | BTIM2_UPDATE |
| 14 | BTIM2_TRIGGER |
| 15 | BTIM3_UPDATE |
| 16 | BTIM3_TRIGGER |
| 17 | ATIM_SHARED_A |
| 18 | ATIM_SHARED_B |
| 19 | GTIM1_UPDATE |
| 20 | GTIM1_TRIGGER |
| 21 | GTIM1_CH1 |
| 22 | GTIM1_CH2 |
| 23 | GTIM1_CH3 |
| 24 | GTIM1_CH4 |
| 25 | GTIM2_UPDATE |
| 26 | GTIM2_TRIGGER |
| 27 | GTIM2_CH1 |
| 28 | GTIM2_CH2 |
| 29 | GTIM2_CH3 |
| 30 | GTIM2_CH4 |
| 31 | GTIM3_UPDATE |
| 32 | GTIM3_TRIGGER |
| 33 | GTIM3_CH1 |
| 34 | GTIM3_CH2 |
| 35 | GTIM3_CH3 |
| 36 | GTIM3_CH4 |
| 37 | GTIM4_UPDATE |
| 38 | GTIM4_TRIGGER |
| 39 | GTIM4_CH1 |
| 40 | GTIM4_CH2 |
| 41 | GTIM4_CH3 |
| 42 | GTIM4_CH4 |

## Data flow and validation

The maintained family YAML owns channels and request routes. DMA metadata was introduced in schema version 8; current schema version 9 preserves `Peripheral.dma` in the persisted chip JSON alongside the analog resource topology. The PAC renderer reads that JSON to emit `DmaController`, `DmaChannel`, and `DmaRequest` metadata. The HAL consumes that metadata; there is no separate handwritten Rust chip/request table. Earlier-schema JSON must be regenerated.

Validation checks the controller kind and ownership, evidence text, nonempty tables, complete CH-array coverage, unique channel numbers/indices/aliases, parent-child ownership, physical bank address/version equality, controller and channel IRQ agreement, matching TC/TE bit positions, existing request peripheral identities, generated-name collisions, unique selector values, and the actual HARDSRC field width. Shared IRQ names remain legal because sharing is the hardware arrangement.

Regeneration and external negative probes cover both devices; temporary probes and generated outputs are not maintained source files. Host generation or cross-compilation demonstrates software consistency, not real DMA transfers or bus-drain timing.
