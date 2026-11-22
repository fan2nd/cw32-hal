> Historical software-verification references: HAL tests and fixture harnesses cited below were deleted on 2026-10-08. Their reported results apply to the dated source snapshot only; source facts remain unchanged. See [current HAL layout and build scope](hal-production-layout.md).

# CW32L052 and CW32L083 GPIO/HSI HAL backends

## Implemented scope

These are explicitly selected family backends for blocking GPIO and a
factory-calibrated HSI final clock tree. They provide the existing Embassy-style
`Peri`, `Config`, `init`, frozen `Clocks`, `Input`, `Output`,
`OutputOpenDrain`, `Flex`, and embedded-hal digital interfaces. UART/SPI/I2C,
EXTI, ADC, timers, DMA, CRC, LCD and other peripheral HALs remain unavailable on
these families. A generated peripheral singleton or PAC block is not a HAL driver.

Ten feature selections are covered:

- L052 family profile, C8T6, R8S6, R8T6
- L083 family profile, MCT6, RBT6, RCS6, RCT6, VCT6

Use an exact orderable-part feature to get its package's pin set and memory map.
The family profiles retain their existing conservative metadata pin sets; neither
is a promise that every union pad is bonded on an unknown package.

## Primary sources

- [L052 UM CN V1.5](https://www.whxy.com/uploads/files/20240920/CW32L052_UserManual_CN_V1.5.pdf),
  especially §§4.3.4, 4.5.2, 4.7.1–4.7.4, 4.7.11, 7.4, 7.9.2, 9.4–9.6.
- [L083 UM CN V2.0](https://www.whxy.com/uploads/files/20240920/CW32L083_UserManual_CN_V2.0.pdf),
  especially §§4.3.4, 4.5.2, 4.5.7, 4.7.1–4.7.4, 4.7.8, 4.7.12,
  7.4, 7.9.2, 9.4–9.6.
- [L052 DS CN V1.3](https://www.whxy.com/uploads/files/20251229/CW32L052_DataSheet_CN_V1.3.pdf),
  Table 5-2 pin types/bonding and §7.3.1 electrical operating limits.
- [L083 DS CN V1.9](https://www.whxy.com/uploads/files/20251229/CW32L083_DataSheet_CN_V1.9.pdf),
  Table 5-2 pin types/bonding and §7.3.1 electrical operating limits.
- [L052 SDK V1.4](https://www.whxy.com/uploads/files/20260309/CW32L052_StandardPeripheralLib_V1.4.zip)
  and [L083 SDK V2.2](https://www.whxy.com/uploads/files/20240821/CW32L083_StandardPeripheralLib_V2.2.zip):
  each CMSIS header, RCC header/source, GPIO source and FLASH source.

`l052-l083-hal-evidence.json` records and rehashes 18 source artifacts, explicit
variant selection, independent register offsets, pin unions and clock encodings.
Reviewed `cw32-data/clock/` and `cw32-data/pinouts/` sidecars supply additional
source-backed metadata, not a substitute for the behavioral manual review.

## GPIO semantics and package contracts

Both variants have DIR=1 input, ANALOG=1 digital path disabled, OPENDRAIN=1 open
drain, normal PUR/PDR controls on output-capable GPIO, four-bit AFRL/AFRH fields,
and atomic BRR/BSRR/TOG latches. They have edge AND level interrupt enables.
There is no SPEED register: only `Speed::Default` exists and +0x08 is untouched.

The real configuration lock at +0x3c requires `0x5a5a` in bits 31:16. Clearing
only the owned pin's low-half bit unlocks it; other locks are retained. Curated
PACs name that register `lock()` on L052 and `lckr()` on L083. Both names refer
to the same audited manual semantics. The SDK's `GPIO_LockPin` helper omits the
key, so its writes are not copied as behavioral authority.

A mode change disconnects first, selects GPIO by clearing the complete four-bit
AF field, configures pulls/open-drain, enables the digital path, then enables an
output last. The public constructor primes the latch first. Disconnect disables
all four interrupt enables for the owned pad and its weak pulls/output; it
retains ODR, pending interrupt flags, shared FILTER, neighboring pins and locks.
Initialization neither resets a shared port nor remaps debug/reset pads.

AHBEN is **unkeyed**, unlike L010/L011/L012. A/B/C/D/E/F gates occupy bits
4/5/6/7/8/9; L052 has no E port. High AHBEN bits and unrelated gates are retained.
Port bases are `0x48000000 + port_number * 0x400`.

Family-wide bidirectional-pad masks, A through F:

- L052: `ffff ffff ffff 0004 0000 00f3`
- L083: `ffff ffff ffff ffff ffff 07f7`

The masks include unsafe-steal-capable SWD pins. Safe singleton generation
excludes PA13/PA14 (default SWD) and PF3/BOOT (input-only). NRST is a dedicated
input and never becomes a GPIO. `AnyPin::steal` remains unsafe and checks only
the family union; its caller must verify bonding, exclusive ownership and any
required SWD/oscillator remapping. Oscillator pads cannot be used as GPIO while
owned by an active oscillator circuit.

Exact-package masks are independently asserted by the compile-contract test.
In particular L083 LQFP100/VCT6 has PF mask `074f`: PF4/PF5/PF7 are **absent**,
although LQFP80 and LQFP64 expose them. LQFP80/MCT6 has sparse D=`0f1f`,
E=`f000`, F=`07ff`; LQFP64 has D2 only, no E, F=`00fb`. Larger packages are not
assumed to be supersets. L052 LQFP48 has C=`e000`, no D and F=`00cb`;
LQFP64 has C=`ffff`, D2, F=`00fb`.

## Clock behavior and safe handover

Both families have 48 MHz HSIOSC, an 11-bit trim halfword at `0x00100a00`,
STABLE bit 15 and DIV at 14:11. Exactly nine manual encodings are supported:
5→/6, 6→/1, 8→/2, 9→/4, 11→/8, 12→/10, 13→/12, 14→/14, 15→/16.
The default HSI/HCLK/PCLK is 8 MHz. Undocumented entry divider encodings,
reserved SYSCLK selections, erased calibration and timeout=0 fail before writes.
L052 has no PLL: SYSCLK=2 is rejected and no PLL register is even accessible to
its compiled implementation. L083 accepts an incoming PLL tree but never exposes
a configurable final PLL tree.

HSI oscillator parameters must be set while stopped (§4.3.4); only divider-only
live changes are allowed by §4.5.2. The SDK's combined live trim/divider writes
are deliberately not copied. Initialization:

1. Enables and verifies the unkeyed FLASH configuration gate.
2. Raises FLASH WAIT to 2, preserving every reserved bit; it verifies readback.
3. Stages HCLK at least /4 (retaining slower entry settings) and PCLK /8
   before any oscillator/divider changes. L083's legal maximum 64 MHz entry
   is bounded to 16 MHz by /4. This
   cannot speed up a legal incoming clock and protects low-voltage handovers.
4. An incoming L083 PLL source first switches to verified, unchanged HSI.
   Direct PLL↔LSI/LSE transitions are forbidden by §4.5. If trim differs,
   enables existing-parameter LSI, verifies stability and
   switches to LSI. L083 also uses this bridge whenever PLL is enabled/selected.
5. On L083, disables PLL and verifies both enable clear and STABLE clear before
   touching HSI. PLL configuration is unchanged. HSE/LSE stay as they were.
6. If trim differs, disables HSI, verifies enable/STABLE clear, then loads and
   verifies factory trim. It does not invent the HSI.WAITCYCLE field mentioned
   in generic manual prose: that field is absent from these register tables.
7. Changes only the HSI divider, preserving trim; enables and verifies HSI,
   switches to it, then rechecks trim/divider/STABLE before final bus dividers.
   This catches an external-fault fallback changing L052's HSI divider in the
   narrow interval between initial HSI verification and source selection.
8. Restores the previous software LSI enable bit, preserving LSI trim/wait
   settings and hardware-dependent users; finally lowers WAIT for final HCLK.

CR0/CR1/FLASH writes carry the documented `0x5a5a0000` key. CR1 clock-security,
LSE lock and other oscillator bits are preserved; §4.4.3.3 limits automatic
external-fault fallback to currently selected HSE/LSE, so LSI is a safe bridge; reset controls, debug/wakeup
CR2, interrupt flags, oscillator configuration and unrelated gates are not
changed. FLASH WAIT is 0 through 24 MHz, 1 through 48 MHz; the documented
conservative WAIT=2 supports flash accesses through 72 MHz but does not enlarge
any device frequency rating. Neither family has FLASH CACHE/FETCH fields.

All hardware waits are bounded iteration budgets, including accepted writes,
oscillator start/stop, source selection and final divider readback. The final
budget iteration may succeed without underflow. Frequencies are published only
after complete success. Failure can leave temporary LSI selected/enabled, staged
bus dividers, or L083 PLL disabled. Reset before retrying; do not use drivers
on an assumption that requested clocks took effect. Call init before peripheral,
DMA, NMI or interrupt code can depend on the affected clocks.

### Board voltage and entry-state contract

Both datasheets §7.3.1 require HCLK/PCLK ≤24 MHz for 1.65 V ≤ VDD <1.8 V.
At VDD ≥1.8 V, L052 permits 48 MHz and L083 permits 64 MHz, but this HSI-only
backend never requests more than 48 MHz. VDD/VDDA, temperature, pad voltages,
loading and all other datasheet limits remain the board application's duty.
Initialization does not measure VDD or validate an already-overclocked bootloader.
The incoming state must be legal for the board and the requested final HCLK/PCLK
must satisfy its supply voltage. Temporary bus prescalers prevent a final
24 MHz low-voltage configuration from briefly executing at 48 MHz with old /1
prescalers. Oscillator tolerance still applies; reported frequencies are nominal.

## Verification and limits

- Production generated-PAC accesses are exercised against aligned RAM for
  register offsets, pin neighbors, lock keys, unkeyed gates and untouched
  speed/reserved/filter/reset/debug registers.
- Host protocol models cover all nine HSI × eight AHB × four APB combinations,
  all supported entry sources, differing/matching trim, LSI restoration, PLL
  shutdown on L083, every effectful-write rejection, timeout paths and the
  final allowed readiness poll. Low-voltage transition traces remain ≤24 MHz
  when both the input and requested final HCLK are ≤24 MHz.
- `tests/test_l052_l083_hal_contracts.py` builds all safe selected pads and
  digital/ownership/RCC APIs on `thumbv6m-none-eabi`, with and without rt+defmt.
  Negative controls reject every unbonded A–F candidate, SWD/BOOT/NRST singletons,
  missing HALs, unsupported speeds/dividers and a conflicting pin reborrow.
- `tests/test_l052_l083_example_links.py` links all eight exact parts with rt,
  both with and without defmt. It inspects ELF32/ARM identity, entry/stack,
  vector count and each load segment against the exact part's memory map.

These checks establish source-level software contracts and linkability only.
No firmware is run on a board; no oscillator timing, electrical behavior,
physical lock operation, SWD operation or silicon bootloader handover is claimed.

### Observed final verification (2026-10-08)

After the PLL-route, temporary-divider and external-fault-race corrections:

- 10/10 host selections passed: 25 unit tests per L052 feature and 28 per L083
  feature, plus one compile-fail doctest for each selection.
- 10/10 ARM release API controls and 10/10 rt+defmt variants passed, covering
  543 safe-pad selections in aggregate. All 547 intended negative controls
  failed for their specifically checked diagnostic and target symbol.
- 16/16 ARM release linked ELFs passed exact memory/vector inspections. L052
  uses 4,496–4,514 bytes FLASH and L083 5,080–5,090 bytes FLASH for the
  tiny init/GPIO fixture; every fixture uses 24 bytes of static RAM, excluding
  stack. Exact sizes are in `verification-logs/l052-l083/arm-links.log`.
- Rustfmt checks passed for all four added Rust files. No warning/error appeared
  in successful host/build logs. Clippy was not run: the installed stable
  toolchain lacks the clippy component.

Logs and a source snapshot manifest are retained under
`verification-logs/l052-l083/`. Firmware was linked and inspected, not executed.
