# CW32L012 full peripheral semantic review

## Scope, evidence, and limits

This is a **targeted manual semantic review of all 28 peripheral register-block
classes** represented by the pinned vendor SVD, covering its 50 named peripheral
views. It is not a claim that every register encoding, reset value, operating
mode, electrical characteristic, or HAL implementation is correct or tested.
In particular, raw SVD/header import and successful Rust compilation are not a
semantic audit or silicon validation.

Sources used in this review:

- WHXY **CW32L012 User Manual CN V1.4**, June 2026 (abbreviated RM below),
  [manufacturer PDF](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf).
- WHXY **CW32L012 StandardPeripheralLib V1.0.5**, 2026-07-01,
  [manufacturer archive](https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip).
  SDK paths below are relative to this pinned archive, principally
  `Libraries/src/cw32l012_<ip>.c` and `Libraries/inc/cw32l012_<ip>.h`.
- Repository `vendor/cw32l012.h`, version 1.2, 2026-06-24, and
  `vendor/CW32L012.svd` from the SDK MDK pack. Hashes are recorded in
  `cw32-data/sources/provenance.yaml`. No newer source or internet inference was
  substituted for these pinned inputs.

The review checked register lists, access notation, status/clear mechanisms,
command/data ports, keys, lock behavior, aliases, and the matching SDK operations.
The register-level recommendations are also in
[`peripheral-side-effects.yaml`](peripheral-side-effects.yaml). That file is a
semantic overlay, not a replacement for access permissions, bit masks, field-level
rules, reserved-bit requirements, peripheral ownership, or operational sequencing.
Its `mixed` category deliberately prevents treating a heterogeneous word as a
plain read-modify-write value. An omitted register is **not** a proof that every
possible read/write is harmless.

## Blocking findings and conservative decisions

| Finding | Evidence | Required treatment |
| --- | --- | --- |
| Interrupt-clear registers are generally **R1W0**, not W1C or ordinary RW | RM §1.2 and the individual ICR tables listed below; SDK clear routines | Preserve unselected clear bits as ones. A zero-initialized field writer can acknowledge every other implemented interrupt. Do not derive a normal `modify` contract from SVD `read-write`. |
| ADC ISR is falsely writable at word level; header additionally calls AWDH writable | SVD ADC1.ISR is RW but all its fields are RO; CMSIS word is `__IOM`, AWDH field `__IOM`; RM §25.12.9 says all status fields RO | Override ADC ISR and all fields to RO, including ADC2 through its shared block. |
| DMA current-count registers are falsely writable | RM §8.8.8 says CCNT and CREPEAT RO; SVD/header describe DMA.CCNT1–4 and DMACHANNEL.CCNT as RW | Override word and fields to RO in **both** overlapping views. |
| GTIM ISR access conflicts with manual | RM §16.10.5 prints RW; header/SVD are RO; RM §16.7 and §16.10.6 define separate ICR; SDK `GTIM_ClearITPendingBit` writes ICR | Keep ISR RO; clear only through established ICR/capture-read behavior. Do not invent direct ISR writes. |
| I2C RXWATER/RXCNT width conflicts | RM §§23.8.9–10 print bits 27:16; header/SVD define 17:16 | Keep the narrower 2-bit vendor definition. Do not expose bits 27:18 pending manufacturer clarification. RM §23.2.1 and SDK `FSL_FEATURE_I2C_FIFO_SIZEn(x)` both describe a one-entry master FIFO. The wider table does not prove a deep FIFO. |
| VC reference DIV width conflicts | RM §§27.7.1–2 print 3:0; header/SVD define 2:0; SDK `VC_DIV_0`…`VC_DIV_7` define eight settings | Keep 3-bit DIV and publicly retain this limitation. Do not claim the field is independently established for values 8–15. |
| I2C RDMO access label contradicts its function | RM §23.8.3 says R0W1 but explicitly describes both enable/disable transitions; §23.4.1.5 and SDK `I2C_MasterRxDataMatch` require clearing it | Preserve a documented conflict. Treat MCR1 as mixed/guarded, not a proven write-one-only strobe. Do not silently resolve this by copying the access label. |
| SDK RTC lock macro conflicts with manual and another SDK function | `cw32l012_rtc.h:310–313`: unlock CA,53; `RTC_LOCK()` writes 55,55. RM §§13.3.5, 13.5.1 require CA then non-53. `RTC_DeInit` writes CA,55 | Use the documented CA,55 lock sequence in any implementation; do not copy `RTC_LOCK()` blindly. |
| SDK DMA clear helper uses the wrong bit mapping | `cw32l012_dma.c:134–136` shifts `interrupt_type << channel`; channels are 0–3; public TC/TE masks are CSR bits 1/2, while ICR TC/TE are bits 0/1 at stride 4 | Explicitly map selected event kind to ICR bit `4*channel + {0,1}`. SDK `DMA_DeInit` already uses stride 4. Do not reuse `DMA_ClearInterruptFlag` arithmetic. |
| SDK BTIM preload helper writes a reserved bit | `BTIM_AutoReloadPreloadConfig` writes `BTIM_ARR_PRELOAD_ENABLE=0x80`; RM §14.10.1 reserves CR1[10:4]; no corresponding SVD/header field | Do not invent BTIM ARPE at bit 7 or copy the helper. GTIM/ATIM preload semantics do not automatically apply to BTIM. |
| CORDIC advertised iteration range conflicts with register table | RM §12.2 says 6–66, whereas §12.6.1 describes four-bit ITER encodings for 2–32; SDK `cordic_iter_t` is 2–32 | Retain the register-width/SDK-supported encoding only; do not promise 66 iterations. The table prose “n means n iterations” also conflicts with its explicit encoding. |
| CORDIC result-read acknowledgement is incompletely specified | RM §12.6.1 says reading results clears EOC, but does not define exact multi-result read ordering | Mark X/Y/Z reads conservatively side-effectful. A driver needs a deliberate result-reading protocol; no neutral register dump. |

Additional pre-existing manual inconsistencies remain: HSI reset value versus
reserved/TRIM text (§4.7.4), and the boot-ROM range/size mismatch (table 2-1).
ADC ICR reset value `0x0000000F` (§25.12.10) also does not match its implemented
R1W0 mask `0x1B`: bit 4 is omitted while reserved bit 2 is included. Do not use
that reset literal as a selective-clear initializer. HALLTIM ISR reset `0xFFFF`
(§18.8.7) spans reserved bits and needs caution as well.

## General access rules

RM §1.2 distinguishes:

- **R1W0:** reads return one; writing zero performs the clear; one has no effect.
  This is not a status read. Vendor C `ICR &= ~mask` takes advantage of this read
  behavior, but a direct clear-mask write is easier to reason about.
- **RW0:** reads report current state; zero clears and one preserves. In a
  mixed/live status word, a generic read-modify-write may acknowledge a flag
  that arrived between the read and write.
- **R0W1:** reads return zero; writing one performs an action; zero has no effect.
  GPIO reset/set/toggle and software strobes belong here. It is not evidence of
  write-one-to-clear interrupt status.
- **RW1:** only writes of one take effect, such as WWDT enable. It must not be
  exposed as a freely reversible boolean.

This review found no W1C interrupt-clear register among these 28 blocks. GPIO
BRR writes one to reset **output data**, a different operation. Do not confuse
these with ARM NVIC pending-clear registers, which are outside this SVD set.

For a selective R1W0 acknowledgement, write zero only to requested implemented
clear bits and one to other implemented clear bits. Several SDK routines use
`~mask` as a whole word; this is evidence of the intended clear polarity, not a
blanket waiver of each register's reserved-bit specification. In particular,
SPI ICR.FLUSH must stay one when merely clearing another flag. Neither register
reset values nor all-zero fieldset defaults are universally safe write values.

## Alias and ownership review

- **DMA / DMACHANNEL1–4:** DMA exposes CSR/CNT/SRCADDR/DSTADDR/TRIG/CCNT/
  CSRCADDR/CDSTADDR at `base + 0x20, 0x40, 0x60, 0x80`; the four channel views
  expose the same words at their own offset zero. RM §8.7 and §§8.8.3–10 agree.
  These are five views of one DMA engine, not independently ownable resources.
- **Timer CCMR CAP/CMP:** GTIM CCMR1CAP/CMP and CCMR2CAP/CMP alias `+0x18`
  and `+0x1C`. ATIM additionally aliases CCMR3CAP/CMP at `+0x50`. Interpretations
  depend on channel mode. Preserve the aliases or an explicit union; do not
  renumber them to remove an apparent collision.
- **I2C MCR2/SCR2:** both are at `+0x24`. RM §23.8.19 explicitly confirms this.
  Slave PINCFG shares storage with master PINCFG and other master fields;
  writing an all-zero slave fieldset can destroy a master configuration.
- **VC base convention:** RM §27.6 uses VC1 base `0x40000080` with CR0 at `+4`.
  Header/SVD use `0x40000084` with CR0 at `+0`. Similarly for VC2–4. Absolute
  register addresses agree. VC12REF/VC34REF are shared analog reference controls,
  not duplicate VC1/VC3 CR0 registers.
- **BGR base convention:** RM §25.12.19 uses offset `0xFC` in the ADC area;
  header/SVD give BGR its own base `0x400000FC`, CR offset zero. Absolute address
  agrees. BGR is shared across ADC, VC and OPA users.
- **IRMOD:** absolute address `0x40004080` (RM §24.4), adjacent to SYSCTRL but
  a separate SVD view. Do not append it to SYSCTRL and also create a second
  independent owner of the same register.
- Preserve manufacturer instance names while normalizing reusable block kinds.
  Examples of field aliases, not extra hardware, are DAC DOR `DMAUDR1/2` →
  `DMAUDR`, `DACC1DOR/2DOR` → `DATA`, DAC output-enable names → `C1OUT/C2OUT`,
  and LPTIM `CH1INPUTSOURCE/CH2INPUTSOURCE` → `CH1SRC/CH2SRC`.

## Per-block manual review (28/28)

### 1. ADC (`ADC1`, `ADC2`)

RM §§25.4–25.12, especially §§25.12.1–2, 25.12.8–18. CR contains ordinary
configuration plus hardware state; START is a conversion command: one starts,
zero stops and resets the sequence channel. Completion behavior is mode-dependent.
ISR is RO and ICR is R1W0. RESULT0–7 are 12-bit RO conversion results; the
review found no documented read-clear effect on these result words. Eight result
slots are not a receive FIFO. Header result comments saying `[15..0]` must not
override the actual 12-bit field and manual.

SDK: `ADC_Init`, `ADC_Enable`, `ADC_SoftwareStartConvCmd`, `ADC_GetITStatus`,
`ADC_ClearITPendingAll`, `ADC_ClearITPendingBit`, `ADC_GetConversionValue`,
`ADC_DMACmd`. ADC1/ADC2 share their RCC gate/reset, and `ADC_DeInit` resets the
shared ADC domain. This requires coordination, not two independent reset calls.
External-trigger and slave coupling need operating-mode-specific review; the
presence of TRIGGER fields alone does not establish a safe synchronized driver.

### 2. ATIM (`ATIM`)

RM §§17.3–17.10; ICR §17.10.6 is R1W0. EGR §17.10.7 generates real update,
capture, COM, trigger and brake events; hardware clears the command bits.
A brake event clears MOE. CCR1–6 reads clear their capture flag in input-capture
mode (§17.4 and the §17.7 interrupt table); capture overrun is separate.
CAP/CMP CCMR views alias. CNT includes RO UIFCPY mixed with the writable count.

BDTR §17.10.25 has a once-programmable LOCK field with level-dependent protection
of BDTR, CR2, CCER, CCMR, DTR2 and AF registers. It is not a whole-word write-once
register: other controls, including MOE, still have their own specified behavior.
Configure the protected set before choosing a lock level; do not silently lock
inside a generic constructor. Preloads, repetition counter, update events and
COM events have different timing. Clearing a brake flag does not establish that
external braking conditions have disappeared or that output is safe.

SDK: `ATIM_ClearITPendingBit`, `ATIM_CtrlPWMOutputs`, `ATIM_SetPWMDeadtime`,
`ATIM_Brake1Config`, `ATIM_Brake2Config`. These corroborate operation types;
they do not replace a motor-output safety design or bench validation.

### 3. BGR (`BGR`)

RM §§25.8–25.9 and §25.12.19. CR is a shared analog enable control, not an
interrupt port. ADC/TS/VC/OPA enabling can automatically start BGR; the manual
says BGR is disabled only by software or POR. BGR/TS need about 30 µs stabilization,
and BGR/TS ADC sampling must last at least 40 µs (§25.12.3). Do not disable this
resource when another analog consumer still uses it. Factory BGR calibration is
at `0x001007D2`; it is not an MMIO register in this block.

SDK: `ADC_SetTs`, `ADC_BgrResult2Avcc`, BGR use in analog examples. No independent
BGR peripheral IRQ or FIFO is described. Ordinary RW storage alone does not
capture shared-resource lifetime or startup timing.

### 4. BTIM (`BTIM1`–`BTIM3`)

RM §§14.3, 14.4 and 14.10. ICR is R1W0; EGR is WO action storage and generates
updates/triggers. CNT.UIFCPY is RO within a writable word. An update can reload
preloads and produce interrupt/DMA activity depending on URS/UDIS; software UG
is not a neutral way to “apply settings.” BTIM ONESHOT and EN differ in naming
from GTIM/ATIM OPM/CEN and should not be merged solely by familiar field names.

SDK: `BTIM_ClearITPendingBit`, `BTIM_Cmd`, event-generation routines and
`BTIM_ConfigUpdateRemap`. Reject the reserved-bit preload helper described above.

### 5. CORDIC (`CORDIC`)

RM §§12.3–12.6. Operand-port writes start computation according to FUNC; for a
two-input operation, Y is the final/start operand. Results reuse X/Y/Z and
reading results clears EOC (§12.6.1), with multi-result ordering not fully specified.
BUSY and EOC are RO; DMAIDLE means BUSY=0 **and** EOC=0. Clearing/consuming EOC
can therefore affect DMA request behavior. No separate start register exists.

SDK: `CORDIC_Init`, `CORDIC_GetStatus`, `CORDIC_SetIter`, `CORDIC_StartComputation`.
The last function is intentionally empty. The input format, convergence domain,
function-dependent output selection and 2–32 iteration encoding require explicit
validation. No generic RMW or debugger dump of result ports is safe to assume.

### 6. CRC (`CRC`)

RM §§10.3–10.6. CR.MODE selects/initializes a calculation; DR accepts one 8-bit
input value per write and advances the running CRC. RESULT is a 16-bit RO result.
The word-level RW tag on DR is not a meaningful “read, edit, write” operation.
No interrupt-clear or FIFO register is present.

SDK: `CRC16_Calc_8bit` writes CR before each calculation, feeds DR byte-by-byte,
then reads RESULT. Do not infer 16/32-bit packed input streaming from the MMIO
word width. The algorithm mode table and data ordering determine the result.

### 7. DAC (`DAC`)

RM §§26.5–26.10. Data-holding registers provide several format/channel views.
A write can update an output without an external trigger when TEN is disabled;
with triggering enabled it feeds the next conversion. SWTRGR bits are write-one
commands and self-clear after the load. DOR1/2 are mixed: RO DATA[11:0] and RW0
DMAUDR bit 15. The latter is cleared by zero, not W1C. No separate DAC ICR exists.

SDK: `DAC_SoftwareTrigger`, `DAC_SetChannel1Data`, `DAC_SetChannel2Data`,
`DAC_SetDualChannelData`, `DAC_ClearFlag`. A DAC channel and an OPA output share
physical output paths; §26.10.14 explicitly forbids simultaneously enabling
conflicting DAC/OPA pin drivers. Sharing one DAC word between two channel drivers
also requires coordinated ownership.

### 8. DMA (`DMA`)

RM §§8.4–8.8. ISR is RO, ICR R1W0, with TC/TE at bit `4*n`/`4*n+1` for zero-based
channel n. CSR mixes hardware TC/TE, transfer state, enables and configuration.
TRIG.SOFTSRC is an action: one starts, zero does nothing, reads report activity.
Do not replay it through generic RMW. RESTART enables repeated transfers;
REPEAT must not be zero, and count semantics are CNT × REPEAT (§8.8.5).

SDK: `DMA_DeInit`, `DMA_Init`, `DMA_StartTransfer`, `DMA_ConfigChannel`,
`DMA_ConfigTriggerSource`. Reject the faulty clear helper mapping. Current count
CCNT1–4 must be RO. Source/destination addresses may not point to DMA, FLASH
controller or RAM controller registers (§§8.8.6–7); this is separate from RAM
and Flash memory regions. Raw register availability does not establish safe DMA
buffer ownership, memory lifetimes, alignment, transfer sizes or bus permissions.

### 9. DMA channel view (`DMACHANNEL1`–`DMACHANNEL4`)

RM §§8.8.3–10 and the header `DMACHANNEL_TypeDef`. These eight-word blocks alias
DMA's numbered channel registers exactly. CCNT is RO even though the vendor access
tag says RW. TRIG has the SOFTSRC action described above. CSRCADDR/CDSTADDR are
RO current addresses. A split-channel HAL must retain one shared owner for global
ISR/ICR and serialize channel configuration against its DMA view. This class is
counted separately only because the vendor SVD supplies a distinct register view.

### 10. EAU (`EAU`)

RM §§11.4–11.7. Division starts on DIVISOR write after DIVIDEND. Square-root
starts on DIVIDEND write; DIVISOR is irrelevant in that mode. Mode/operand writes
while BUSY are restricted/ignored. Only 32-bit accesses are supported; smaller
accesses are ignored. QUOTIENT/REMAINDER are RO and read as zero while busy, not
as a completion indication. ZERO/OVR describe the last operation and reset for a
new operation; they are not software-clear flags.

SDK: `EAU_SetMode`, `EAU_StartOperation`, `EAU_GetStatus`, `EAU_GetQuotient`,
`EAU_GetRemainder`. No IRQ or FIFO is described. Validate divide-by-zero and signed
overflow before treating results as meaningful.

### 11. FLASH (`FLASH`)

RM §§7.5–7.10. CR1, CR2 and PAGELOCK require KEY `0x5A5A` in their upper halfword
on each write. PAGELOCK zero means locked, one unlocked. ICR is R1W0. CR2
CACHEINVALID requires one then zero and is not automatically self-clearing.
CR1 operation mode changes the effect of **memory writes**: page erase/program
is triggered by accesses to the target Flash memory, not only controller writes.
ISR.BUSY and error conditions must be respected; keying is not a sufficient safety
check. SDKCFR is RO metadata about the protected library area.

SDK: `FLASH_SetLatency`, `FLASH_UnlockAllPages`, `FLASH_LockAllPages`, erase/program
routines, `FLASH_ClearITPendingBit`. Security-level helpers also use undocumented
addresses outside this controller view; their existence does not authorize adding
an unreviewed secure-control block or a safe API. OTP and security changes need a
separate irreversible-operation design and are not certified by this review.

### 12. GPIO (`GPIOA`, `GPIOB`, `GPIOC`, `GPIOF`)

RM §§9.3, 9.6. ISR/IDR are RO, ICR R1W0. BRR, BSRR and TOG are R0W1 action
registers, with reset, set/reset halfwords and toggle semantics respectively.
ODR is a latch; IDR is the sampled input. GPIOC/F are not full 16-pin ports, and
PDR is implemented only for PF03 despite inherited vendor bitfields for all pins.
AF register position alone does not prove a legal peripheral/pin combination.

SDK: `GPIO_WritePin`, `GPIO_Write`, `GPIO_TogglePin`, `GPIO_SetAlternateFunction`
and pin macros. SWD pins PA13/PA14 need deliberate policy; CR2.SWDIO changes debug
availability. Preload output latches before enabling output drivers, and serialize
shared configuration-word RMW across pins.

### 13. GTIM (`GTIM1`–`GTIM4`)

RM §§16.3–16.10. ICR R1W0; EGR action strobes; mode-dependent CAP/CMP aliases;
CCR1–4 reads clear CCxIF in input-capture mode. CCR writes are not supported as
capture programming and can be preload writes in output mode. CNT contains RO
UIFCPY. Preserve ISR as RO despite the manual's conflicting access labels.

SDK: `GTIM_ClearITPendingBit`, initialization/capture/compare functions in
`cw32l012_gtim.c`. The four instances share a layout, not a common live register
bank. Timer update, compare and trigger sources, clock prescaling and preload
transfer timing remain distinct runtime concerns.

### 14. HALLTIM (`HALLTIM`)

RM §§18.3–18.8. CR.SOFTCAP is R0W1, while other CR fields are configuration.
ICR is R1W0; ISR, WIDTH and STATE are RO. CNT.DATA is explicitly **RW0** in
§18.8.3, despite RW in header/SVD: reads give the live 24-bit counter, but arbitrary
preload writes are not established. Keep zero-clearing semantics conservative.
The manual's ISR reset literal also needs the caution noted above.

SDK: `HALLTIM_GenerateSoftTrigger`, `HALLTIM_GetCounterValue`,
`HALLTIM_GetPulseWidth`, `HALLTIM_ClearStatus`. WIDTH reads have no documented
read-clear effect in the reviewed table; do not infer the GTIM capture behavior
merely from a similar use case.

### 15. I2C (`I2C1`, `I2C2`; SDK file name `lpi2c`)

RM §§23.4–23.8. MICR/SICR are R1W0. MCR0/SCR0 FIFO-reset bits are R0W1,
whereas RESET holds the module in reset until cleared. MTDR is a WO command/data
port; STDR is a WO slave transmit port. MRDR/SRDR consume buffered receive data;
read DATA and EMPTY/SOF from a single access. SASR reads acknowledge address state
and are only valid in the documented ADDR condition (§23.4.1.6). Neither the
slave nor master register set can be blindly dumped by a debugger.

SDK: `I2C_MasterClearFlag`, `I2C_SlaveClearFlag`, `I2C_MasterReceive`,
`I2C_SlaveReceive`, `I2C_MasterSend`, `I2C_SlaveSend`, FIFO reset helpers,
`I2C_MasterRxDataMatch`. Keep MCR2/SCR2 as aliases. RDMO access and RXWATER/RXCNT
width conflicts remain unresolved. The SDK receive helper's command parameter
packing should not be used to claim arbitrary-length command support: MTDR's
receive count is one byte plus one; larger operations need multiple commands.

### 16. IRMOD (`IRMOD`)

RM §§24.3–24.4. One CR with ordinary MOD/IRSW/INV controls; no IRQ-clear, FIFO
or key register is described. It combines live GTIM/UART outputs and a software
level. Changing it can affect an already-driving output immediately; “ordinary RW”
does not mean no external effect. Source peripherals, mux and GPIO configuration
must agree. There is no on-chip IR receive demodulator (§24.3.3).

SDK: `IRMOD_Config`, `IRMOD_IRSW_SetHigh`, `IRMOD_IRSW_SetLow`. Keep the explicit
IRMOD register identity instead of accidentally extending SYSCTRL twice.

### 17. IWDT (`IWDT`)

RM §§19.3–19.6. KR is WO command/key storage: `5555` unlocks, other values lock;
`CCCC` starts, `AAAA` reloads, `5A5A` then `A5A5` stops. These commands also
relock. CR/ARR/WINR require unlocking and update synchronization; a WINR change
reloads the counter. SR.OV is RW0 while the remaining status bits are RO. A valid
CNT observation requires two consecutive equal reads (§19.6.4).

SDK: `IWDT_Init`, `IWDT_Unlock`, `IWDT_Lock`, `IWDT_ClearOVFlag`,
`IWDT_GetCounterValue`, `__IWDT_*` macros. Feed timing outside the allowed window
can cause a reset. Importing key values does not justify an unrestricted safe
watchdog write API.

### 18. LPTIM (`LPTIM`)

RM §§15.3–15.7. ICR R1W0; ISR/CNT RO. CR0 is especially mixed: ARST/SRST are
**write-zero reset commands** with completion readback; CNTSTART/SNGSTART are
write-one start commands; EN is normal control. Write a reset only after its
readback is one. CFGR and IER can be configured only while disabled. ARR/CMP
cross a clock domain; wait for ARROK/CMPOK before another write (§15.3 update
protocol). A zero-initialized CR0 write inadvertently requests both resets.

SDK: `LPTIM_Init`, `LPTIM_SetAutoreload`, `LPTIM_PWMStart`, `LPTIM_ClearFlag`,
`LPTIM_ClearITPendingBit`. Read CNT twice until equal (§15.7.4); clock-domain
synchronization is not a read-clear operation or a guarantee of coherent sampling
with other registers.

### 19. LVD (`LVD`)

RM §§28.3–28.7. CR0/CR1 configure analog detection and interrupt/reset behavior;
SR.INTF is RW0, FLTV RO. ACTION chooses interrupt versus system reset, so this is
not merely an event source. Level-triggered conditions can persist after a flag
acknowledgement. LSI can be hardware-enabled by the filter path independently of
a software LSI enable indication.

SDK: `LVD_Init`, `LVD_GetIrqStatus`, `LVD_ClearIrq` and filter/configuration helpers
in `cw32l012_lvd.c`. Thresholds, hysteresis and board supply behavior require analog
validation; no writable FLTV setter should be generated.

### 20. OPA (`OPA1`, `OPA2`)

RM §§29.3–29.6. CR contains analog routing/enabling. CAL mixes configuration,
RO AZRUN, the SOFTTRIG write-one action and START level-calibration control.
AZRUN is meaningful for triggered calibration only; level calibration requires an
explicit elapsed interval and stop. Calibration temporarily changes analog
operation. Shared BGR and DAC/OPA output conflicts must be coordinated.

SDK: `OPA_Init`, `OPA_Start`, `OPA_Stop`, `OPA_AutoZeroSoft_Start`,
`OPA_AutoZeroSoft_Stop`, `OPA_AutoZeroTrig_Config`, `OPA_AutoZeroTrig_SoftEvent`.
SDK delay comments and loop counts are not a calibrated timing proof. No OPA
interrupt/FIFO register is present in these two-word blocks.

### 21. RAM (`RAM`)

RM §§6.4–6.6. ADDR reports the parity-fault address, ISR.PARITY is RO and ICR
PARITY is R1W0. IER controls the error interrupt. This block is a parity controller,
not SRAM data storage. Its ADDR value is fault context, not a general address
register to program.

SDK: `RAM_ITConfig`, `RAM_GetITStatus`, `RAM_ClearITPendingBit`. The reviewed
manual does not establish a FIFO of multiple faults or a read-clear behavior for
ADDR. SYSCTRL also has a RAM-fault brake path; clearing the interrupt is separate
from resolving the corrupt data or braking policy.

### 22. RTC (`RTC`)

RM §§13.3.5–6 and 13.5. KEY is WO; ICR is unprotected R1W0. Other writable
registers need the CA,53 unlock sequence and documented relock. While running,
DATE/TIME/AWTARR require WAIT synchronization. Date/time values use documented
BCD formats. Read SSCNT using the repeated-equal and nonzero-low-counter procedure
in §13.3.6; a single MMIO read is not a certified timestamp. Timestamp registers
and counters remain RO after unlocking.

SDK: `RTC_DeInit`, `RTC_Init`, `RTC_SetTime`, `RTC_SetDate`, `RTC_GetTime`,
`RTC_GetDate`, interrupt-clear and timestamp functions. Reject the conflicting
RTC_LOCK macro. Do not classify KEY as a real stored password or assume all RTC
registers have identical reset persistence.

### 23. SPI (`SPI1`–`SPI3`)

RM §§22.3.8–9 and 22.7. DR read receives a value **and clears RXNE**; a DR write
transmits data. ICR fields are R1W0. Its bit 0 FLUSH is also zero-active and clears
the transmit buffer and shift register, setting TXE. Clearing unrelated flags
with a zero-filled word can thus cancel an active transmission. ISR is RO.

SDK: `SPI_SendData`, `SPI_ReceiveData`, `SPI_ClearITPendingBit`, `SPI_ClearFlag`.
Unlike some unrelated SPI IPs, do not import a “read SR then read DR” error-clear
ritual or W1C status model without local evidence. Data width, duplex mode and CS
ownership determine which transfers are valid.

### 24. SYSCTRL (`SYSCTRL`)

RM §§4.2–4.7. CR0/CR1/CR2/IER/AHBEN/APBEN1/APBEN2 need embedded `0x5A5A` keys
on every write. Reset registers are unkeyed and active-low. ICR is R1W0;
RESETFLAG is RW0 actual cause state. CR1.LSELOCK and LSE.PINLOCK are POR-sticky
controls; they are not reversible booleans. HSI/LSI/HSE/LSE combine RO stability
with writable configuration. Enabling a consumer can start LSI without setting
CR1.LSIEN (§4.7.2), so a software enable bit alone is not actual oscillator state.

SDK: `SYSCTRL_ClearITPendingBit`, `SYSCTRL_ClearRstFlag`, clock/reset macros and
`system_cw32l012.c`. Frequency switching, flash latency, clock monitoring and
SWD/brake settings need ordered policy. Existing HSI reset-value conflict must
remain recorded; factory trim and documented masks are preferable to writing a
conflicted whole-register reset literal.

### 25. UART (`UART1`–`UART3`)

RM §§21.3–21.9. TDR is a WO data/break/idle port; writing it changes transmit
availability. RDR is RO data but **reading it does not clear RC**. RC needs an
explicit zero to ICR.RC; in synchronous receiving the clear permits the next
reception. ICR fields are R1W0. ISR.TXE is hardware-managed through TDR activity,
not an ICR bit. Do not transfer SPI receive-read acknowledgement semantics here.

SDK: `UART_SendData`, `UART_ReceiveData`, `UART_LINSendBreak`, `UART_ClearFlag`
(the explicit RC/RDR warning is at `cw32l012_uart.c:815–819`). The SDK receive
helper clears RC before reading RDR; a driver must consider mode-specific data
lifetime/races rather than assuming those two actions are atomic.

### 26. VC (`VC1`–`VC4`)

RM §§27.3–27.7. CR0/1/2 configure analog selection, filtering, blanking and
interrupt conditions. SR mixes RO FLTV and RW0 INTF. Clearing INTF acknowledges
an event; it does not change the comparator output or remove a persistent level
condition. Shared reference controls and BGR are separate resources. The manual
base convention differs by four bytes, but absolute addresses match header/SVD.

SDK: `VC_Init`, `VC_GetInterruptFlag`, `VC_ClearInterruptFlag`, reference and
blanking helpers in `cw32l012_vc.c`. Do not make four independent exclusive owners
of two shared reference ladders, or silently alter a pair's reference when
constructing the second comparator.

### 27. VC reference (`VC12REF`, `VC34REF`)

RM §§27.7.1–2. REF selects source, enable and resistor-divider setting. It is an
ordinary analog configuration register, with no IRQ/FIFO/key mechanism described.
VC1/VC2 share one output; VC3/VC4 share the other. The DIV width conflict is
unresolved: use the header/SVD 3-bit field and SDK settings 0–7, explicitly keeping
this limitation instead of claiming a corrected 4-bit interpretation.

SDK: `VC_SetRefVoltage` and `VC_DIV_0`…`VC_DIV_7` in `cw32l012_vc.h`.
Reference voltage changes affect both associated comparators and can trigger
threshold-related behavior elsewhere.

### 28. WWDT (`WWDT`)

RM §§20.3–20.6. CR0.EN is RW1: once enabled it cannot be disabled by writing zero.
WCNT reads the live counter and writes feed it. CR1.IE is labelled RW, but §20.6.2
explicitly says it cannot be cleared once set. SR.POV is RW0. Counter writes
outside the valid window can cause reset; an ordinary-looking setter is an
operationally consequential command.

SDK: `WWDT_Init` and `WWDT_Feed`. No stop command equivalent to IWDT's two-key
sequence is established. Keep the two watchdog kinds distinct.

## Validation boundaries and follow-on requirements

1. The SVD inventory has 50 views and 28 non-derived layouts; every layout is
   represented above. DMA aliases mean this is not a count of 50 independent
   ownership resources.
2. The machine-readable overlay uses existing normalized block/register names;
   all entries must resolve before generation. Access corrections for ADC ISR
   and DMA/DMACHANNEL CCNT must apply to fields as well as words.
3. Generic readable/writable capability must not imply generic safe `modify`.
   Every action, key, mixed word, or read-consuming register needs a deliberate
   API, normally a direct raw operation behind an unsafe boundary or a reviewed
   driver. Read-clear ports need protection from accidental reads/debug display.
4. The unresolved width/access/reset inconsistencies above are documented,
   conservative source selections, **not** manufacturer errata confirmations.
5. PWR, DBG, core NVIC/SysTick and digital signatures appear elsewhere in the
   manual/SDK but are not additional independent blocks in this 50-view SVD.
   PWR uses core SCB/SCR and SYSCTRL; DBG uses SYSCTRL.DEBUG and core debug;
   signature/factory-trim addresses are memory facts. Do not claim their omission
   means the device lacks those facilities or fabricate peripheral instances.
6. This review does not certify complete enums, DMA request routing legality,
   all AF/package mappings, every timer mode, all silicon revisions, interrupt
   concurrency, analog safety, timing or behavior on real hardware. Those require
   explicit metadata validation and subsystem/board tests.
