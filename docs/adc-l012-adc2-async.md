# L012 ADC2 finite asynchronous conversion with DAC coexistence

Implementation scope, 2026-10-12. L012 ADC2 extends the existing finite software
single-read and ordered 1–8-slot scan API to shared IRQ30, `ADC2_DAC`. ADC1 keeps
its dedicated IRQ12 policy. Both converters support blocking operation and manage their shared clock internally. Stage91 main verification produced five standalone
ARM libraries and ten ordinary ELFs; its independent reviews are recorded below.
At main acceptance clean replay and final54 report binding had not run.
No hardware execution or analog qualification is claimed.

## Ordinary API and usable peer

```rust,ignore
bind_interrupts!(struct Irqs {
    ADC1 => adc::InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => adc::InterruptHandler<peripherals::ADC2>;
});
let mut adc1 = adc::Adc::new_async(p.ADC1, Irqs, config)?;
let mut adc2 = adc::Adc::new_async(p.ADC2, Irqs, config)?;
let sample = adc2.read(&mut input, sample_time).await?;
let timing = adc2.read_sequence(&sequence, &mut output).await?;
```

The checked `new_async` constructor performs peer admission itself and returns
`Result<Self, Error>`. ADC2 implements the sealed `AsyncInstance`
capability; there is no separate unchecked constructor, public quiet-peer lease,
DAC-token reservation or DAC interrupt handler. `Dac::new`, `new_ch1` and
`new_ch2`, ordinary direct-loading writes, reads, drop and reconstruction remain
usable during an ADC2 async lifetime. ADC2 never prepares DAC pins.

This is a closed contract over the current safe driver API. DAC preparation and
start write both interrupt-enable bits as zero; set/read leave them untouched;
Drop preserves them while disabling outputs. OPA reads DAC and retains its gate,
RCC mutation is private/init-only, and safe DMA destinations do not include DAC
controls. An actual checked zero therefore stays zero under these supported
operations. A future safe DAC interrupt API must first replace this proof with
compatible shared dispatch and fault handling. DAC trigger, waveform and DMA
APIs are not added here.

## Admission and retained clock effects

Inside a critical section, after the existing ADC configuration and inherited
source checks, ADC2:

1. Checks that DAC's active-low reset is released before opening its gate or
   reading DAC MMIO. It never releases reset on the caller's behalf.
2. Enables a closed DAC gate using the existing keyed RCC helper and finite
   `Config.timeout` budget. Any acquired gate remains enabled, including after
   timeout or later constructor rejection.
3. Verifies gate-on and reset-released state, reads actual DAC CR0, then rechecks
   the domain before interpreting that sample.
4. Requires both CR0.DMAUDRIE1 and DMAUDRIE2 to be zero before creating the ADC
   RAII owner, writing ADC2 configuration/flags or activating IRQ30.

`Error::DacInReset` reports an asserted DAC reset; `Error::DacClockUnavailable`
reports unavailable gate/readback admission; `Error::DacInterruptEnabled`
reports either enabled DAC interrupt source. A peer-admission rejection before
local ADC writes is not a terminal ADC2 fault. These checks do not promise to
detect arbitrary hardware faults or recover from bus faults.

DAC uses APBEN2/APBRST2 bit10; ADC1/ADC2 share APBEN1/APBRST1 bit0. No gate is
disabled, no reset is asserted/released and no BGR/TS enable is removed. Opening
a previously closed DAC clock may advance retained DHR-to-DOR transfer or other
clock-dependent activity before admission succeeds or fails. This is a
conservative functional warning: the manual's clock notes occur under channel
EN fields and do not establish a particular APBEN2-to-DOR transfer event or
timing. Restoring the gate would not undo progress, and is not attempted. An
ordinary live HAL DAC already retains its gate.

ADC2 writes no DAC CR0, CR1, SWTRGR, DHR or DOR register. It neither disconnects
outputs nor acknowledges peer status. Both zero IEs suffice to mask DAC's two
documented DMA-underrun IRQ sources even if sticky DMAUDR is already set. EN,
OUT, TEN, WAVE and DMAEN are not substitute IRQ masks and are not rejected
solely for interrupt admission. Retaining an unsupported inherited trigger/DMA
configuration does not adopt, stop, drain or qualify an external DMA writer.

## Completion and output ownership

Both converters reuse the [L012 finite sequence and timing rules](adc-l012-scans.md).
Every operation has one software start and one to eight caller-ordered slots;
repeated typed channels and per-slot acquisition times are supported. Internal
temperature and bandgap sources keep their settling/acquisition constraints.
Output length must exactly match sequence length.

The ADC2 handler first checks per-instance static active state before MMIO.
It requires enabled local EOS and sticky EOS, masks only that local completion
enable, checks its readback and wakes the static `AtomicWaker`. The handler
never reads DAC registers, samples or caller buffers. A stale NVIC pending
exception with inactive ownership returns before ADC MMIO; normal handling,
construction, cancellation and drop never clear NVIC pending.

The future registers its waker before its final event check. After EOS it
confirms EOC and START-clear with finite synchronous polls, stages all samples
locally, verifies local completion masking and selective acknowledgment, then
commits the whole requested output without another await. Error or cancellation
leaves every caller output element unchanged. EOC/EOS clear through the existing
typed ICR no-op image, preserving watchdog flags and reserved bit2. Result/ISR
registers remain read-only; no DAC flag is cleared.

`Config.timeout` bounds synchronous poll attempts, not elapsed await time.
Use ordinary `with_timeout`/`select` with working clocks, IRQs and a waking time
source for caller deadlines. Eventual executor service is required; no maximum
IRQ latency, analog abort latency or completion time is promised.

## Cancellation, faults and replacement

Dropping a pending conversion masks local EOC/EOS, requests documented START=0
stop/slot-cursor reset, disables local EN/CONT/SLAVE and selectively clears
completion with bounded readback. Successful cleanup permits the next operation
to initialize afresh. Ordinary cancellation and successful owner drop leave
IRQ30 enabled and preserve the sibling converter, DAC and common analog power.

Failed required cleanup/configuration readback latches per-instance
`Error::Faulted` until chip reset. ADC2's terminal containment is a nonrecursive
leaf: it latches fault, attempts local EOC/EOS masking with bounded readback,
then retires static dispatch. If masking is verified, a stop/configuration/clear
fault does not disable the shared vector. Only failure to verify the local mask
permits whole-vector disable under the admitted zero-DAC-IE contract. The ISR
uses a single immediate EOS-mask check and can contain its known failure
directly; it does not run task-side polling. Direct DAC outputs remain usable.
ADC1 retains its existing dedicated-vector containment.

Forgetting a future skips cleanup and can leave the finite conversion active
or complete. The ISR holds static state and an owned waker clone, with no pointer
to a future, channel, peer guard or caller buffer. The next operation explicitly
establishes stopped/cleared state before reuse. Forgetting the ADC2 owner can
similarly leave static dispatch active after a reborrow ends. Async replacement
rechecks peer admission and retires old dispatch before local initialization;
blocking replacement retires abandoned async state before its existing blocking
initialization. Both preserve any sticky fault. A never-async blocking owner
keeps its existing path.

A forgotten DAC can leave outputs physically active; token availability is not
proof of quiet hardware. Actual IE-zero admission remains necessary, and later
ordinary DAC construction or drop still preserves that checked invariant.
Raw/PAC code, external DMA or debugger writes that alter the owned ADC controls,
DAC IRQ enables, relevant gate/reset domains or vector are functional interference
outside this HAL contract. They require explicit coordination with the raw
driver, not a caller assertion passed to this constructor.

## Examples, evidence and acceptance

The existing [ordinary L012 firmware examples](../examples/l012/adc-scan/README.md)
use both async ADCs concurrently with direct DAC outputs. `async_single` reads
ADC1 PA0 and ADC2 PA8; `async_scan_borrowed` uses full eight-slot external/internal
sequences, repeated channels, select cancellation and immediate reuse. PB0/PB1
are independently owned DAC outputs. C8T6/LQFP48 and C8U6/QFN48 both have qualified
64 KiB Flash and 8 KiB SRAM; the generic L012 alias remains a library selection
with no package/memory claim. The optional existing `async` feature reserves
GTIM1 for the run-mode time driver. Old blocking and DAC binaries remain intact.

The immutable [own-source fact record](adc-l012-adc2-async-evidence.json) uses
the existing locked CN RM1.4, current EN RM1.0, DS1.0 and SDK1.0.5 identities.
See [the source catalog](../sources/SOURCES.md) for original links and page
locators. Hardware evidence is separate from the current safe-API closure and
software acceptance; it carries no build/publication status.

Main verification passed five standalone ARM library commands and ten ordinary
ELF links. The successful rows span sixteen attempts: initial E1 `async_single`
overflowed the qualified 64 KiB Flash by 1,088 bytes, then the reviewed
example-only release optimization changed from `s` to `z` with LTO, codegen1,
source behavior and memory unchanged. The ten ELF rows completed on that final
profile. The five successful libraries retain their original actual inputs in
the separate firmware workspace. The failed attempt and two source snapshots
remain recorded; this was not one uninterrupted same-tree run. L4, the L010
regression library, retains three existing RCC dead-code warnings; there were
no new warnings.

Both existing finite L012 data and provenance gates passed. An earlier
navigation refresh failed on the new evidence locator-key spelling; the narrowly
reviewed correction and successful refresh remain distinct receipts. Main ran
no hardware generation, and all 910 canonical generated files stayed unchanged.
The isolated example profile correction did not change source/data/provenance
facts; its inverse correspondence binds the earlier successful gate snapshot.

The independent [runtime/example review](adc-l012-adc2-async-runtime-review.json)
and [metadata/source review](adc-l012-adc2-async-source-review.json) accepted
the frozen main implementation and its actual software receipts. Their source
identities, reviewed corrections and acceptance boundaries remain separate
from the immutable hardware evidence record.

At main acceptance the final-source clean replay had not run. Final completion
requires its separate two-library/three-ELF receipt, the same finite gates and
one bootstrap's correspondence with all 910 accepted canonical generated
paths/bytes. The 54-selection semantic source/data review was completed against
its recorded inputs, while binding the final source and deterministic capability
reports remained outstanding. That separate final54 receipt is required for the
overall baseline exit; it is not a new 54-library build matrix.

No hardware execution, accuracy, calibration, settling, lossless streaming,
continuous/periodic scanning, general hardware-triggered scan, safe ADC DMA,
synchronized converters, DAC IRQ dispatch, low-power or RF support follows.
