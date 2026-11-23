# LVD monitoring and infrared modulation routing

This is bounded, source-qualified partial HAL support, not complete peripheral
or protocol support. No device has been flashed or measured.

## LVD scope and ownership

`lvd::LowVoltageMonitor` owns `Peri<LVD>` and, for `new_pin`, a lifetime-held
analog `Flex`. `new_supply` monitors the chip's named VDD/VDDA rail.
`Threshold::from_millivolts` accepts only an exact entry in generated own-family
metadata. It does not round, accept a raw selector, or claim measured accuracy.

Construction reads CR0/CR1 first and rejects an already enabled LVD, reset action,
interrupt enable, LEVEL/RISE/FALL trigger, or enabled filter. The low families
also reject SYSCTRL.CR2.LVDBRKEN, preventing a polling constructor from changing
an active ATIM protection source. Checks and configuration share a critical
section. Only CR0.SOURCE, CR0.VTH and CR0.EN are written; source/threshold/enable
are read back once. A failed write attempts to clear only the newly owned EN.
Drop clears only EN. Source/threshold writes are not rolled back after a write
failure; an attempted external input is left disconnected. Constructor tokens
are consumed on error unless the caller supplied a reborrow. Existing pending
flags are never acknowledged.

No CR1, SR, NVIC, BOR, reset flag, filter-clock, shared reference, divider or trim
register is written. There is no independent LVD controller gate in the reviewed
families, so no synthetic RCC_INFO or VC-gate ownership is introduced. GPIO
configuration uses the existing central GPIO RCC handling. Hardware may consume
its analog/reference resources while LVD is enabled; the driver does not promise
that a shared reference or oscillator is powered down on Drop.

`sample` reads the unfiltered, hysteretic comparator result. `wait_for` performs
at most the supplied nonzero number of SR reads, returning `Timeout` if the
requested state was not observed. The budget is not a duration, debounce or
analog-startup guarantee. The caller must qualify analog settling, tolerances,
pin limits, hysteresis and board behavior. L012's nominal thresholds assume
Vcore = 1.6 V; this driver neither changes nor validates Vcore. Reset/wakeup/IRQ,
filtering, output routing and asynchronous monitoring remain unimplemented.

| Family | Nominal threshold table | Supply | Reviewed external selectors |
|---|---|---|---|
| A030/F030/F020 | 2.00, 2.11, 2.22, 2.33, 2.44, 2.56, 2.67, 2.78, 2.89, 3.00, 3.11, 3.22, 3.33, 3.44, 3.56, 3.67 V | VDDA | 1:PA0, 2:PB0, 3:PB11 |
| F002/F003 | 1.8–3.3 V in 0.1-V steps | VDD | 1:PB6, 2:PB3, 3:PA0 |
| L010 | 1.8–4.6 V in 0.4-V steps | VDD | 1:PA3 |
| L011/L012 | 1.8–4.6 V in 0.4-V steps | VDDA | 1:PA0 |
| L031/L052/L083/W031 | 1.8–3.3 V in 0.1-V steps | VDDA | 1:PA0, 2:PB0, 3:PB11 |
| R031 | 1.8–3.3 V in 0.1-V steps | VDDA | 2:PB0, 3:PB11; selector 1 is omitted by its own manual |

All external-pin implementations are further filtered by the exact package or
family-alias intersection and the existing HAL safe-pin policy. F020's manual
supersedes its inconsistent SDK threshold constants. These values are nominal
facts, not an electrical safety threshold guarantee.

## IR scope, register differences and ownership

`ir::IrModulator::attach` takes the unique controller/subfunction token without
changing any control register. `set_mode` selects the exact destination-specific
PAC enum where qualified. `with_output` holds one source-reviewed, package-bonded
`Flex` and configures its AF as push-pull with no weak pulls. Replacing or dropping
the output disconnects the pin. Controller settings and externally owned signal
sources are preserved on Drop.

The real source timers/UARTs must be configured and kept alive separately. This
HAL never takes over their clocks, channels, baud rates, interrupts or DMA. Mode
changes affect live signals and may glitch; no glitch-free switching is promised.
IRSW is a boolean input, not a universal enable: false does not mean inactive in
OR modes. There is no automatic carrier generation, protocol encoding, receiving,
IrDA timing guarantee, or complete transmit engine.

| Families | Hardware control | Qualified combinations |
|---|---|---|
| A030/F030/F020/L031/L052/L083/R031/W031 | SYSCTRL.IRMOD, 0x40010074, MOD[3:0] | GTIM1/GTIM2 channels 1/2 or UART1/2 with the documented timer channel, AND/OR |
| F002/F003 | SYSCTRL.IRMOD, 0x40010074, MOD[3:0], IRSW[4] | GTIM channels 1/2, BTIM2/3 TOGP, UART1/2 or software input in the exact 16-entry destination table |
| L010 | IRMOD.CR, 0x40004080, MOD[3:0], IRSW[4], INV[5] | GTIM/ATIM channels 1/2 and UART1/2 combinations with IRSW |
| L011 | IRMOD.CR, 0x40004080, same field positions | GTIM1/GTIM2 channels 1/2 and UART1/2 combinations with IRSW; separate PAC version from L010 |
| L012 | IRMOD.CR, 0x40004080, same field positions | Source selector writes withheld; existing MOD preserved. Only attachment, output pin, IRSW and INV are exposed |

On classic families, metadata describes the IR subfunction on SYSCTRL, and the
HAL generator emits one `IR` ownership token if one is missing. F003's existing
`IR` alias token is reused; MMIO uses the fuller SYSCTRL.IRMOD view. No invented
register map is introduced, and no universal TriggerSource abstraction is used.
The low families retain their real `IRMOD` token. None has a separate IR RCC gate.

L012 is intentionally partial: CN V1.4 §24.4 p571 and EN V1.0 §24.4 p630 agree,
but the current SDK's `cw32l012_irmod.h` disagrees for the 16 mode encodings,
referring to GTIM1–4 and UART3. Neither side establishes the silicon truth. The
L012 register version remains untyped for MOD and the safe HAL omits `Mode`,
`set_mode` and `mode`; no arbitrary encoding is accepted instead.

Own-datasheet IR routes (before package and safe-pad filtering):

- A030/F030/F020/L031/W031: PA13 AF6, PB9 AF4
- F002: PA4 AF2, PB3 AF7, PC2 AF2; SDK-only PC4 omitted
- F003: PA4 AF2, PB3 AF7, PC2/PC4 AF2
- L010: PA2/PA3 AF5
- L011: PA8 AF3, PA13 AF6, PB1 AF4
- L012: PA8 AF3, PA13 AF6, PB1/PB9 AF4, PF3/BOOT AF7
- L052: PA13 AF6, PB9 AF4, PC4/PC11 AF3
- L083: PA13 AF6, PB9 AF4, PC4/PC11 AF3, PE2 AF4
- R031: PA13 AF6 only; SDK-only PB9 omitted

Current safe GPIO policy withholds debug PA13. Consequently R031 has owned
controller configuration but no safe HAL IR output route. Some family aliases or
small packages likewise have no safe IR output even when another package does.
No debug remapping, oscillator takeover, reset-pin takeover or unbonded pin is
silently enabled to advertise broader coverage.

## Verification and artifacts

- Own-family manuals, data sheets and selected SDK members are pinned by URL,
  SHA-256, version and page/field coordinates in `lvd-ir-evidence.json` and the
  canonical source lock. The source acquisition tool verifies actual bytes.
- IR output AFs were extracted from each own-family positioned PDF table,
  compared with SDK macros, and filtered by physical package metadata. R031's
  omitted PB9 and L012's PF3/BOOT row were additionally inspected as rendered
  pages. LVD threshold and source tables were read independently per family.
- Both generators run normally. All 54 chip/family features passed a normal
  `cargo build -p embassy-cw32 --target thumbv6m-none-eabi --features <chip>`.
- `examples/f030/basic/src/bin/lvd_ir_bursts.rs` links a real Cortex-M0+ ELF using
  the existing HAL timer drivers as separately owned signal sources, IR on PB9,
  and polling LVD supply indication on PB0. The link used explicit target and
  `-Tlink.x`; it is not a test harness and has not been executed on a board.
- Own-source static review checks all threshold and mode entries, permitted
  MMIO write fields, source-mode quarantine and every package projection.
  No HAL tests, fake register adapters, mock hardware or test harness were added.

The driver shape follows the pinned Embassy ownership boundary (`Peri`, sealed
pin capabilities, `Flex` lifetimes and direct PAC register access). STM32 voltage
protection or trigger semantics were not transferred to CW32.
