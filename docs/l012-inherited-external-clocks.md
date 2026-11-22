# L012 inherited external-clock entry restriction

Historical Stage41 correction. The direct-HSE candidate preserves this no-write
refusal for undeclared enabled HSE and adds only explicit exact reuse or a fresh
disabled-source start. See [current qualification](qualified-l012-hse.md).

This is a bounded correction to the HSI-only L012 initializer. It does not add
HSE initialization or change RTC ownership. Current common RCC initialization
also provides [inherited LSE pad protection](inherited-lse-pads.md), independently
of this L012 HSE restriction.
It applies to `cw32l012`, `cw32l012c8t6`, and `cw32l012c8u6`.

## Corrected behavior

After pure configuration/frequency validation, the existing L012 backend reads
`pac::SYSCTRL.cr1().read().hseen()`. If HSE is enabled, it returns
`Error::InheritedHseEnabled` before any MMIO write, including the RTC and FLASH
configuration gates. This checks the enable state regardless of SYSCLK, HSE
mode, STABLE, or whether the RTC calendar is running. An enabled HSE may belong
to RTC/AWT even when SYSCLK is already HSI.

The error publishes no clocks and returns no peripheral tokens. `init`
already took the singleton set before entering RCC, so reset before retrying. The correction never disables HSE, rewrites
RTC, clears oscillator faults, or changes the user's clock-security policy to
make entry succeed. The remaining HSI initialization path is unchanged.

Enter with HSE disabled, for example after a clean reset whose startup and
bootloader code leave HSE disabled. Do not enable HSE or establish an HSE-backed
retained RTC/AWT before calling this initializer. An application that must keep
that inherited source needs a separately qualified ownership implementation;
this patch intentionally rejects it. Do not disable a source supporting a
retained consumer merely to bypass the check.

For previously delivered firmware without this correction, avoiding PF0/PF1
GPIO and other peripheral use while HSE is enabled is an immediate mitigation:
reserve PF0 for bypass and both PF0/PF1 for a crystal. Avoiding both is the
simpler conservative rule. A GPIO input constructor also reconfigures the pad;
this warning is not limited to driving an output.

## Confirmed earlier call chain and impact

In Stage39 and the unchanged L012 code in the Stage40 HSE candidate:

1. `init` takes the peripheral singleton set, then calls RCC `init`.
2. L012 `configure` accepts incoming SYSCLK encodings 0/1/3/4, preserves HSE,
   inspects only the retained RTC HSIOSC owner, and returns frozen HSI clocks.
3. `build.rs::safe_pin` exposes PF0/PF1. L012 `clock_limits.hse` is absent, so
   `build.rs` does not emit `rcc_hse` for this family.
4. `gpio::Output::new` or `Input::new` enters `Flex::new`. The HSE reservation
   check exists only under `cfg(rcc_hse)`, so it is absent for L012.
5. GPIO construction disconnects/reconfigures the pad through the real GPIO
   DIR/ANALOG/pull/interrupt controls; output selection also selects GPIO AF0
   and enables digital output.

The own datasheet maps PF0 to OSC_IN and PF1 to OSC_OUT on both modeled
48-pad packages. These legal safe-API calls can therefore disrupt retained
external-clock operation, including RTC/AWT timing. This is a concrete
ownership and functional-clock defect, not a newly demonstrated memory-safety
violation. No silicon experiment or memory-corruption proof is claimed.

Stage39 also lacked the corresponding HSE reservation on L010/L011. The
separate Stage40 candidate qualifies their own HSE paths and records inherited
enabled HSE mode in frozen clocks, so their existing safe GPIO check reserves
the actual pads. This L012 patch must be applied on that candidate rather than
replacing it with an older low-family backend.

Across the 13 non-RF electrical profiles, the current candidate has HSE
qualification on A030/F020/F030, L010/L011, L031/R031/W031, and L052/L083. The
remaining profiles are L012 and F002/F003. F002/F003 use their distinct digital
HEX hardware and `hex_pin_reserved` guard, including retained AWT inputs; they
are not an additional missing-`rcc_hse` case. This inventory is limited to the
reported qualification/guard gap, not a new complete audit of every driver.

## Separate retained-LSE protection

The Stage41 HSE-only correction left an LSE ownership gap: L012 could preserve
LSE while returning PC14/PC15 tokens without a GPIO reservation. The current
common RCC capture now protects inherited LSE on all eleven LSE-bearing families.
It reserves both pads in crystal mode, only OSC32_IN in bypass, and both when the
family's documented pad lock applies, even with LSE disabled. It neither changes
these locks nor relies on hardware blocking every write through a fault.

L010's own pads and PINLOCK policy differ from L011/L012; the implementation uses
reviewed metadata rather than inferring compatibility. See the
[all-family LSE ownership contract](inherited-lse-pads.md) for the boot-retained
reservation, package limits and direct-PAC/quiescent-entry boundary. LSE setup
and source qualification remain separate unsupported work.

## Own-source evidence

- [L012 RM CN1.4, §4.7.2, printed p45 / PDF71](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf#page=71):
  SYSCTRL.CR1 offset 0x04, reset 0x00000001; HSEEN bit1, 0=disabled, 1=enabled.
  This is the native PAC field read by the correction. LSEEN bit4 and LSELOCK
  bit5 establish the distinct low-frequency enable/retention behavior.
- [L012 DS CN1.0, Table5-2, printed p32 / PDF35](https://www.whxy.com/uploads/files/20250717/CW32L012_DataSheet_CN_V1.0.pdf#page=35):
  both LQFP48/QFN48 map PF0/PF1 to OSC_IN/OUT at package positions5/6 and
  PC14/PC15 to OSC32_IN/OUT at positions3/4. CR1 and this pin table were visually
  checked. The copied HSE setup example in the RM names different pins; the
  positively corroborated own DS table is the pad authority.
- [L012 RM §13.3.2 and §13.5.3, printed pp165/176 / PDF191/202](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf#page=202):
  RTC.CR1.SOURCE[10:8]=1 selects HSE, 0 selects LSE; the first prescaler also
  feeds AWT. SYSCLK alone cannot establish external-oscillator ownership.
- [L012 RM §4.3.5 and §4.7.7, printed pp30/51 / PDF56/77](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf#page=77):
  LSE crystal/bypass pad requirements and PINLOCK bit17 conditions.
- L010 own DS CN1.3 Table5-2 p23/PDF24 and RM CN1.2 §4.7.7 p73/PDF74;
  L011 own DS CN1.1 Table5-2 p26/PDF29 and RM CN1.1 §4.7.7 p71/PDF72:
  distinct LSE pad mappings and source-specific pin-lock controls.

Exact L012 source SHA-256 identities:

| Input | SHA-256 |
|---|---|
| RM CN1.4 PDF | `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340` |
| DS CN1.0 PDF | `08a605fe8843d1eefa2b1115474b5a74df2833068f0ad6f35225e4f39377da76` |
| SDK1.0.5 archive | `8b0a4c0ee865642d08f353aec98906c900a8b10f672120e649e6b1bf5e29c18f` |
| SDK `Libraries/inc/cw32l012.h` | `3758779c7b9e60fda1aa80dfd2848b6986074d91a6763076fa6777d55b1ad000` |
| SDK `CW32L012.svd` | `fda08355c30ed60204cd53b2c1ca33e6ee2233530e6cb2448522158dfc302942` |

The SDK header corroborates CR1.HSEEN position1/mask0x2 and LSEEN position4/
mask0x10. The existing source-derived register YAML/PAC needs no modification.
No adapter, cfg, software model, metadata schema, or HAL test is added.
Validation receipts for the exact candidate distinguish source review and ARM
compilation from hardware execution. No hardware execution is claimed.
