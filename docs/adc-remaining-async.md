# Finite software ADC interrupts on L012 and classic families

Source-qualified scope, updated 2026-10-12. Both L012 ADCs and all ten classic lines expose
`Adc<'d, T, Async>` for single reads and one finite ordered scan. Their existing
blocking APIs remain available. L012 ADC2 uses the separate
[checked DAC-peer and shared-vector contract](adc-l012-adc2-async.md);
its Stage91 main produced five libraries and ten ELFs. At main acceptance the
separate clean replay and final54 report binding had not run; final completion
requires their own receipts.
This extends the [L010/L011 async scope](adc-low-async.md); each backend retains
its own register, electrical, source and cancellation contract.

## API and ownership

Classic families bind their dedicated ADC interrupt:

```rust,ignore
bind_interrupts!(struct Irqs {
    ADC => adc::InterruptHandler<peripherals::ADC>;
});
let mut adc = adc::Adc::new_async(p.ADC, Irqs, config)?;
let sample = adc.read(&mut pin, sample_time).await?;
let timing = adc.read_sequence(&sequence, &mut output).await?;
```

L012 retains the common analog owner. Both converters implement the sealed async
instance capability with distinct dedicated/shared-vector policies:

```rust,ignore
bind_interrupts!(struct Irqs {
    ADC1 => adc::InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => adc::InterruptHandler<peripherals::ADC2>;
});
let mut adc1 = adc::Adc::new_async(p.ADC1, Irqs, config)?;
let mut adc2 = adc::Adc::new_async(p.ADC2, Irqs, config)?;
```

`new_async` returns `Result<Self, Error>`; callers handle construction failures explicitly. `read` returns raw 12-bit
counts and `read_sequence` returns `SequenceTiming`. The existing sealed channel
handles, borrowed/owned erasure and package-qualified pins are unchanged. A
sequence references existing handles, so repeated channels do not duplicate pin
ownership. Output length must equal sequence length. The future borrows the
converter, channels, sequence and output until completion or drop.

F030/A030/F020/F002/F003 have up to four slots; L031/R031/W031/L052/L083 and L012
have eight. Classic scans require one common sample time and unbuffered external
channels for more than one slot. Internal/follower-enabled sources keep their
single-read qualification. L012 allows distinct sample times and qualified
internal sources in an ordered scan. Existing voltage/reference bounds, weak
source restrictions, HSI qualification and exact rational `ClockBounds` apply.
No additional route, internal reference, calibration or electrical range follows
from enabling interrupts.

## Completion and output commit

L012 uses EOS-only interrupts for singles and scans. The future verifies EOS,
EOC and `START=false`. Classic MODE0 singles use EOC; MODE4 scans use EOS, with
OVW enabled as an error wake in both modes. Classic singles require EOC and
`START=false`; scans require EOS and `START=false`. OVW wins over apparent success.
READY remains a
bounded startup poll because it has no enable or clear field.

The handler filters active ownership and the enabled pending owned source,
masks owned completion/error enables, retains status and wakes the per-instance
upstream `AtomicWaker`. The future registers its waker before its final status
check. Retained completion covers an event that arrives before registration.
Once an event has been observed, terminal readback uses a finite synchronous
poll; the task cannot depend on a second interrupt after the sole wake was masked.
The ISR never holds or writes a caller buffer pointer.

Results are read into a local array only after terminal observations. The driver
then verifies owned masking and selective acknowledgment and commits the whole
requested output slice without an intervening await. An error or cancellation
leaves every element unchanged. With all hardware triggers disabled and a finite
software mode, delayed collection cannot be overwritten by a subsequent automatic
scan. This does not detect arbitrary raw register interference.

L012's typed ICR write starts from its no-op value and clears only EOC/EOS,
preserving AWDL/AWDH and reserved bit2 (effective value 0x1c). Classic clears
EOC/EOS/OVW while preserving EOA/WDTL/WDTH/WDTR (effective value 0x3c). ISR and
RESULT are read-only; reading a result is not acknowledgment. No classic READY
clear or invented L012 READY/OVW/BUSY field is used.

## Cancellation, inheritance and faults

Async acquisition rejects inherited triggers, watchdog monitoring/interrupts and
DMA request enables before adopting the converter. Classic also rejects EOA
interrupt and accumulation; L012 rejects synchronized local or sibling SLAVE
state. An independent L012 sibling can remain owned and operating. Rejection
preserves those inherited configurations and flags. Common or local clock-gate
acquisition needed before readable preflight is a separate explicit operation.

Every new read first establishes stopped, disabled and cleared owned state,
including after a forgotten future. A drop guard masks owned events, requests
START zero, disables owned conversion state and selectively clears owned flags
with finite readbacks. Successful cancellation permits fresh initialization.
Failure of a required mask, stop, clear or configuration readback latches
`Error::Faulted` per instance until chip reset, including replacement owners and
switching between blocking and async. Reconfiguration does not clear the latch.
Normal cancellation does not disable NVIC. Dedicated-vector backends retain
their existing terminal-fault containment. ADC2 disables the shared ADC2_DAC
vector only if owned completion masking cannot be verified, under its checked
DAC-interrupt-free contract. Other ADC2 readback faults still latch `Faulted`
but leave the shared vector enabled when the local completion mask is verified.

L012 explicitly documents START zero as stop plus sequence-cursor reset.
Classic documents stop, and separately documents MODE4 as one complete selected
sequence per start. Classic reuse is a stated inference: establish stop, disable
and reinitialize EN, reprogram the full owned mode/slot configuration, clear owned
flags, then start a fresh finite sequence. This is not an explicit classic cursor
reset guarantee. Neither backend supplies independent drain acknowledgment,
maximum analog abort latency or instantaneous analog sampling cutoff. A readback
of START is the documented control state, not an invented BUSY handshake.

Forgetting a future skips its destructor and promises no quiescence. Finite
hardware writes only peripheral result registers; no ISR/DMA pointer survives into
caller memory. The next operation repairs the peripheral explicitly.

L012 never resets or disables the ADC group gate or clears shared BGR/TS enables; the sibling converter, comparator, OPA and DAC registers
are preserved. ADC2's separate retained DAC-gate acquisition is described below. On
F030/A030/F020/F003, live BGREN and its shared ADC gate are retained. F002 reserves
BGREN/TSEN, while L031/R031/W031/L052/L083 reserve BGREN; no fabricated enable is
written. Their existing ADC reference/temperature ownership limits still apply.
Retained shared resources and unresolved fault state can retain power.

ADC2 admission requires actual DAC CR0.DMAUDRIE1/2 both zero in a clocked,
reset-released domain. It retains the DAC gate and does not write DAC registers;
opening that gate can allow retained DAC activity to progress, including before
a rejected admission. Ordinary direct-loading DAC outputs coexist because the
current safe DAC/OPA/RCC/DMA API preserves both interrupt enables at zero.
There is no public lease, DAC-token reservation or DAC interrupt dispatcher.
Raw users must coordinate owned ADC/DAC interrupt controls and the vector.
Forgotten ADC2 async ownership is retired before blocking or async replacement
initializes the converter; the sticky fault latch survives replacement.

## Deadlines and examples

`Config.timeout` counts finite synchronous poll attempts during setup, stop and
finalization. It does not bound time pending in `.await`. Applications use the
ordinary pinned `embassy_time::with_timeout` or `embassy_futures::select` with a
working time/wake source; losing futures are dropped. Progress needs running
clocks, enabled interrupts, a correct typed binding and eventual executor service.
There is no maximum IRQ, scheduling or whole-operation latency guarantee.

[The L012 examples](../examples/l012-adc-scan/README.md) include independent
ADC1/ADC2 single reads and eight-slot mixed-source scans, select/timeout
cancellation and reuse, and concurrent direct-loading DAC output. The existing
blocking binaries remain separate. [The classic examples](../examples/classic-adc-scan/README.md)
cover all ten lines using generated package routes, full-length ordered scans,
internal supply single reads and cancellation/reuse. Their optional `async`
feature uses the existing executor/time/futures pin and family-appropriate GTIM
or GTIM1 time driver; it reserves that timer and interrupt. The separate
[run-mode time-driver contract](time-driver.md) applies.

These ordinary firmware examples are source deliverables, not HAL tests,
register models or evidence of execution. Compilation and silicon validation
must be reported separately. DMA, periodic/continuous acquisition, externally
triggered scans, synchronized ADCs, RF, deep sleep and lossless streaming remain
outside this scope.

## Own-source evidence

The [evidence record](adc-remaining-async-evidence.json) cites only existing
canonical manual/header source IDs. Exact URLs, revisions, hashes and archive
chains remain in [the single source authority](../sources/evidence-sources.json).
No new vendor artifact or SDK example is a qualification dependency. A030 uses
the explicitly shared CW32x030 manual and existing F030 register-source alias;
there is no claimed separate A030 SDK.

Page numbers are printed / one-based PDF pages. Classic IRQ tables are §5.4;
each own manual and CMSIS header independently identifies dedicated ADC IRQ12.

| Family | Slots | IRQ | MODE0 single | MODE4 scan | START/IER | ISR | ICR |
| --- | ---: | --- | --- | --- | --- | --- | --- |
| F030/A030 | 4 | 95/96 | 439/440 | 446/447 | 466/467 | 467/468 | 468/469 |
| F020 | 4 | 93/94 | 377/378 | 384/385 | 404/405 | 405/406 | 406/407 |
| F002 | 4 | 71/72 | 306/307 | 313/314 | 331/332 | 332/333 | 333/334 |
| F003 | 4 | 73/74 | 366/367 | 373/374 | 393/394 | 394/395 | 395/396 |
| L031 | 8 | 90/91 | 436/437 | 443/444 | 465/466 | 466/467 | 467/468 |
| R031 | 8 | 92/93 | 439/440 | 446/447 | 467/468 | 468/469 | 469/470 |
| W031 | 8 | 91/92 | 439/440 | 446/447 | 468/469 | 469/470 | 470/471 |
| L052 | 8 | 95/96 | 473/474 | 480/481 | 505/506 | 506/507 | 507/508 |
| L083 | 8 | 104/105 | 478/479 | 485/486 | 507/508 | 508/509 | 509/510 |

L012 CN1.4: IRQ tables70–71/96–97; finite conversion §25.5.2 at581–582/607–608;
stop/reset §25.5.1 at579/605 and START §25.12.2 at590/616; interrupt table
§25.10 at587/613; IER596/622 and ISR/ICR597/623; results598–599/624–625;
shared BGR599/625; DAC shared interrupt sources607/633.

F002's manual calls TRIGGER bit3 PA02 while its SDK/PAC calls it PA32. The bit
location and disable polarity agree, which suffices for software-only rejection;
no new GPIO trigger route is inferred. L031/R031/W031 SPI/I2C versus SPI1/I2C1
labels likewise retain their recorded alias scope. Existing voltage, sample-time
and reference conflicts remain in the electrical and scan source documents.

The existing Embassy pin `f16efeffe37581092ec184718e6fdb1620393214` supplies the
waker, binding, guard and caller-composition structure. It supplies no CW32
hardware or electrical evidence.

The additional [ADC2 source record](adc-l012-adc2-async-evidence.json) binds
the own L012 DAC request enables, IRQ30 owner set and distinct gate/reset
domains. Its hardware facts are separate from the current safe-API closure
and the accepted main 5-library/10-ELF receipts. At main acceptance clean had
not run; final completion requires separate final-source 2-library/3-ELF and
final54 binding receipts, not a reinterpretation of the main results.
