# Reserved internal flash data

`flash::Flash` is a blocking driver for one explicitly reserved data partition.
It performs actual main-flash byte programming and 512-byte page erasure on
CW32L012C8 and CW32F030C8. It is not a running-firmware updater, an option-byte
writer, or a mechanism for changing read protection.

## Establishing ownership

`Flash::new_blocking(p.FLASH, start..end)` is **unsafe**. `start` and `end` are
absolute flash addresses; `end` is exclusive. Both must be 512-byte aligned and
the partition must be nonempty. The peripheral singleton only owns controller
registers. It cannot prove that arbitrary flash addresses are not code or live
Rust data.

Before construction, reserve the whole range in the application linker layout.
Do not place code, vector tables, static references, constants, literal pools,
unwind/return paths, or RAM initialization load images there. No Rust allocation
or Rust reference may overlap any byte of the partition during the driver's
lifetime. This remains true if the driver is forgotten. Dropping it does not
make erased contents executable or restore old data. The driver never returns
a reference or memory-mapped slice into its partition.

No other code or bus master may access the partition or change its FLASH
controller, clocks, remapping or protection while this owner exists. In
particular, acquiring `FLASH` does not reserve memory used by another HAL driver
or a DMA source. The caller must coordinate those users before construction and
keep that coordination valid for every operation.

All read/write/erase method offsets are **relative to the partition start**.
`capacity()` reports its size; `base_address()` reports its absolute start.

```rust,ignore
// Application-specific example only. Its linker script must reserve exactly
// this range and the system must satisfy the constructor's DMA/IRQ contract.
let mut data = unsafe {
    embassy_cw32::flash::Flash::new_blocking(p.FLASH, 0xe000..0xf000)?
};
data.blocking_erase(0, 512)?;
data.blocking_write(1, &[0x12, 0x34, 0x56])?;
let mut readback = [0; 3];
data.blocking_read(1, &mut readback)?;
```

The example is not executed by this project. The generated default `memory.x`
describes all physical flash; it does **not** reserve the example's partition.
An application using a data partition must provide its own appropriate linker
layout instead of assuming that unused addresses will stay unused in a later
firmware build. Verify the final ELF's VMA and LMA ranges, including `.data`'s
flash load image, after linking with the actual optimization and LTO settings.

## Geometry and excluded regions

| Property | CW32L012C8 | CW32F030C8 |
|---|---|---|
| Physical main flash | `0x0000_0000..0x0001_0000` | same |
| Erase size | 512 bytes, 128 physical pages | same |
| Program access used | 8-bit store, alignment 1 | same |
| Read access used | 8-bit volatile load, alignment 1 | same |
| Erased value | `0xff` | same |
| Unlock granularity | 8 pages / 4 KiB per `PAGELOCK.LOCK[n]` | same |
| Busy status | `ISR.BUSY` | `CR1.BUSY` |
| Additional exclusion | page 127 and configured secure library | none within main flash |

Both RMs also support aligned 16-bit and 32-bit accesses. This implementation
deliberately uses the actual minimum 1-byte program unit, so source-buffer and
offset alignment need no special handling. An erase request must have both
endpoints aligned to 512 bytes. Bounds use checked arithmetic before touching
hardware. Reversed, overflowing, out-of-range and unaligned erase requests are
rejected. Empty reads/writes at `capacity()` and aligned empty erases succeed
without hardware access.

On L012, `0xfff0..0x10000` contains secure-library activation controls, including
the key, page indexes, magic values and CRC. The **entire** containing erase page,
`0xfe00..0x10000`, is excluded even if secure library protection is disabled.
When `SDKCFR.START <= SDKCFR.END`, the accepted partition must end before the
start page. The documented secure-library layout ends at page 127. Thus no
operation can read, overwrite or erase its configured contents or controls.
`SDKCFR` is only read. F030 does not expose this L012 security-control layout.

OTP, boot ROM, ISP protection-change registers and other option/security
registers are outside the accepted main-flash range. The driver never accesses
them, never requests chip erase and never changes `SECURITY` or a protection
level. Temporary page-group unlocking is unavoidable hardware granularity;
only the group containing the current byte/page is added to the original unlock
mask, and the exact original mask is restored afterward. It does not assume
that the caller originally had every group locked.

## Execution, busy state and interrupts

The manuals explicitly support executing from flash during a page erase or
program: the CPU's next flash instruction fetch stalls until the operation
completes. RAM execution continues, so RAM code must poll the busy bit. The
driver polls the correct per-family busy bit in both cases.

There is **no wall-clock timeout** and no documented flash abort sequence. A
counter loop located in flash cannot bound an instruction-fetch stall. After
launch, this implementation retains the controller clock and critical section
until it observes idle. It does not restore mode/cache, return an error, drop
the owner, or release normal interrupts while the operation remains busy. A
hardware fault that leaves busy asserted can therefore hang the caller. An
application requiring a guaranteed deadline or in-flight cancellation cannot
use this driver to supply that guarantee.

An operation already busy on entry returns `Error::Busy` without changing its
mode, cache, page locks or flags. The configuration clock is pinned in this
unexpected state so that dropping the driver, or a failed constructor, cannot
gate a still-active controller. The driver never resets FLASH as part of
acquisition or recovery. Normal operations always finish before returning;
there is no outstanding transfer for `Drop` to cancel.

The constructor requires all DMA and other non-CPU bus masters to avoid **all
flash reads during a write/erase**, including reads outside the data partition.
The RMs document CPU instruction-fetch stalls, not a general safe concurrent
DMA-read protocol, so DMA behavior is not inferred from CPU behavior. Merely
placing the destination partition outside code does not establish this
exclusion. Source bytes used by the blocking write may be in other flash: the
CPU loads each byte before its program store and waits before loading the next.

Normal interrupts stay masked for the whole nonempty write/erase request,
including checks and verification. This can exceed a timer, UART, DMA service,
motor-control or watchdog deadline. The caller must choose request sizes and
system operating conditions accordingly. There is no async interface or IRQ
binding: these FLASH interrupts report errors rather than a completion event.
The shared FLASHRAM NVIC pending state is never cleared globally, since it also
belongs to RAM error handling.

In particular, the configured GTIM1 time driver requires overflow IRQ service
within **less than 65.536 ms**. One whole write/erase batch, including hardware
flash stalls, may exceed that budget and lose timekeeping wraps. The application
must bound request sizes and actual elapsed time to meet it, or avoid concurrent
time-driver/motor activity. Flash does not preserve real-time scheduling merely
because its address range is separate from the application. Existing motor
examples do not call this driver; their sampling paths are unchanged.

NMI and HardFault are not masked by an ordinary Cortex-M critical section. The
caller must ensure those paths obey the exclusion contract. If any such path
must execute during busy, its vector entries, handler, called functions,
literal pools and accessed data must be in SRAM and must avoid flash/controller
access. The same applies to a future complete SRAM updater: annotate-and-copy
one Rust function alone does not place its callees, constants, vectors or
compiler-generated helpers in RAM. Verify final linked addresses and startup
copying. This driver does not emit a `.ramfunc` section or linker/startup
scaffolding, and does not claim arbitrary self-update safety.

## Hardware sequence and error behavior

The controller must initially be idle and in read mode. L012 must not be left
holding `CACHEINVALID=1`. The driver acquires the configuration clock without a
peripheral reset, preserving startup wait states and F030 `STANDBY`.

For each request it saves CR2, the original page-group lock mask and FLASH IER;
disables FLASH error interrupts; writes keyed CR2 with `CACHE=FETCH=0`; and
checks cache/prefetch/wait-state readback. L012 receives its explicit
`CACHEINVALID=1` then `0` pulse. Implemented error flags are cleared by writing
zero, with reserved ICR bits preserved from the register read. These are not
write-one-to-clear flags.

A write prechecks **every destination byte** for `0xff` with the cache disabled
before it programs any byte. CW32 hardware rejects a non-erased program unit
even if the requested data would only make additional 1-to-0 transitions.
The driver sets program mode, unlocks and reads back the current page group,
stores one byte, waits for idle, checks hardware flags, and verifies the byte.
An erase similarly selects page-erase mode, unlocks the group, triggers with a
byte store, waits/checks, and verifies every byte in the page is `0xff`.

Completion and error paths restore read mode, the exact old page-lock mask,
CR2/cache/prefetch/wait state, and FLASH IER. L012 cache invalidation is repeated
before normal cache use resumes. Captured operation errors are cleared before
restoring IER. Mode, lock and CR2 restoration are read back; rejected writes
report `Configuration`. F030's `STANDBY` and both devices' read-only protection
state are preserved. Program, locked-page, current-PC-page, secure-library and
cache errors have distinct variants. A partial write/erase can remain after
an error; readback and bounds checking do not provide transactionality or
power-fail atomicity.

## Storage traits

`embedded-storage` 0.3.1 `ErrorType` and `ReadNorFlash` are implemented with
`READ_SIZE=1` and partition capacity. Byte program/page erase are available
through the inherent blocking methods, with `WRITE_SIZE=1` and `ERASE_SIZE=512`.

`NorFlash` and `MultiwriteNorFlash` are deliberately **not** implemented.
The actual [`NorFlash::write` contract](https://docs.rs/embedded-storage/0.3.1/embedded_storage/nor_flash/trait.NorFlash.html)
promises that power loss cannot change the rest of the page outside the written
words. The audited CW32 manuals and SDKs establish byte programming behavior
during normal operation but do not establish that power-loss containment
guarantee. Deferring the writable trait avoids making that stronger promise.
Repeated programming is independently disallowed by both CW32 manuals.

## Reference comparison and evidence

The implementation was compared with the actual
[Embassy Flash module](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/flash/mod.rs)
and [common blocking implementation](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/flash/common.rs)
at `b12a6d9efcd2711037abca1b63a661a9ef726444`: owned peripheral, relative
blocking methods, storage error kinds and geometric bounds are useful
precedents. CW32 register sequences and the unsafe partition contract are not
assumed from STM32.

Primary hardware sources:

- [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf):
  7.3–7.5, printed pp88–92 (geometry/access/program/erase); 7.6.1–7.6.4,
  pp94–96 (group locks, PC protection, secure library); 7.8, pp98–99
  (instruction stalls and RAM polling); 7.10, pp100–105 (keys, busy, flags,
  cache invalidation and SDKCFR).
- [CW32x030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf):
  7.3–7.5, pp111–114; 7.6, pp115–117; 7.7, pp118–119;
  7.9, pp120–123. F030 busy is in CR1 and has no L012 CACHEINVALID/SDK flags.
- Vendor SDK `cw32l012_flash.c` V1.0.5 and `cw32f030_flash.c` V2.2 corroborate
  byte triggers, keyed writes and 4 KiB group masks. The L012 SDK erase helper
  omits the cache/prefetch disable mandated by the RM; the HAL follows the RM.
  The SDK's unchecked halfword/word byte-count arithmetic is not copied.

No erase/program operation on a physical device was performed for this release.
Compile/API checks and external protocol probes cannot establish voltage,
endurance, power-loss behavior or silicon timing.


## L012 read acceleration

`flash::enable_read_acceleration(&mut p.FLASH)` is a narrow L012-only startup
operation. It enables instruction prefetch and read caching with a keyed CR2
read-modify-write, preserving the RCC-selected wait states. It rejects busy,
non-read-mode or active-invalidation states and verifies readback. The return
value reports the wait-state count and previous prefetch/cache enables.
It does not reset, erase, program, unlock or change protection. Borrowing the
singleton prevents concurrent use by `Flash` through safe HAL ownership.
The temporary AHB clock guard controls the configuration interface only.

Example 07 invokes this before PWM, preserves three wait states at 96 MHz,
performs DSB/ISB and retains the singleton without Flash mutations. Examples
01–06 do not opt in. Cache/prefetch do not establish a real-time execution bound;
Flash mutation still requires the full cache and concurrency rules above.
