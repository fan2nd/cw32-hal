# Native finite UART receive to observed idle

Main verification passed six actual ARM libraries, five ELF links and four
finite source/data gates. At main acceptance clean had not run; final-package
completion requires a separate final-source two-library/two-ELF clean receipt.
These are the historical Stage82 main results, not verification of the Stage83
[buffered receive extension](uart-buffered-rx.md), whose separate main results
and final-source clean requirement are recorded in its own verification boundary.

`UartRx<Async>::read_until_idle` and the combined `Uart<Async>` method are available
on the existing L010/L011/L012 UART register versions. All ten current selections
retain their existing pin, clock and peripheral restrictions. Other families,
DMA reception and programmable TIMOV receive timers are outside this API.
These methods remain on ordinary owners; `BufferedUartRx` and `BufferedUart`
expose no idle method or idle-marker queue.

## Operation and boundary

The result is a received byte count when the caller slice fills or native RXIDLE
is observed. An empty slice returns zero without MMIO or arming. First nonempty
poll masks the previous RX wait group and clears only stale RXIDLE once; it
preserves unread RC/RDR and receive errors. This starts a new observation window,
not a flush or recovery of an earlier packet boundary.

One task-context status snapshot governs RDR eligibility, acknowledgments, error
selection and idle completion. Observed RC permits one RDR read. The command
uses the existing typed R1W0 no-op seed and clears only observed receive events.
ORE, parity, framing and noise errors take priority in that order; a sampled
error byte is not appended. Earlier successful caller bytes remain on error or
cancellation, but an error result does not provide a partial count.

With RC and RXIDLE together, the observed byte is consumed before returning the
count. The flags provide no timestamp assigning that byte to a preceding or
following burst. Boundary assignment is best-effort under service delay, and
repeated already-set flags can coalesce while being acknowledged. A critical
section does not stop incoming UART hardware events.

Idle detection starts only after a completed character. If an old idle is cleared
while its RC byte remains, consuming that byte does not guarantee another idle
for it. More traffic, a full buffer or cancellation may be needed. A nonempty
slice may return zero on an idle-only observation in the new window. No software
timer, exact framing, continuous collection, lossless reception or bounded wait
is promised. Prompt per-byte executor service remains necessary.

## Ownership and cancellation

For these ordinary owners without a ring, the ISR only samples status/enables,
masks the pending direction and wakes its task. That no-ring path never reads
RDR or acknowledges ICR, and holds no caller-buffer pointer. The separate
buffered path collects and acknowledges observed receive data in the ISR;
both paths wake outside the lifecycle critical section.
Wait registration precedes arming and status recheck. Dropping a pending wait
masks RX/error/idle enables while retaining unread data and flags. Forgetting can
leave enables and a task waker registered; a subsequent enabled RX event masks
the whole group, or a later ordinary read, nonempty idle read or RX-owner Drop
retires it. Forgetting does not promise boundary recovery or notification.

Existing native ordinary async, blocking and trait reads retire old idle enables,
including empty ordinary reads, without clearing status or changing exact-length
semantics. An empty new idle call intentionally has no cleanup side effect.
RX teardown preserves a live TX half, and TX teardown preserves live RX. Existing
split ownership, IRQ guards, CTS/RTS, clock gates and DMA retention stay intact.

## Source and examples

Own L010 CN1.2 printed398–400/425–428, L011 selected CN1.1 printed400–402/427–430,
and L012 CN1.4 printed455–457/486–489 establish idle, overwrite/error and native
R1W0 fields. Each own SDK states RDR reads do not clear RC. RXIDLE bit3 differs
from TIMOV bit6; the programmable timer formula is not an idle timeout.
Source identities and page pairs are in [the source evidence](uart-idle-evidence.json).

The [ordinary examples](../examples/shared/uart-idle/README.md) use combined receive on
three exact packages and split concurrent I/O on L012. Compilation and linkage
establish their software integration only, not hardware event order,
throughput, injected errors, cancellation execution or waveform behavior.

## Verification boundary

Main verification passed six real ARM library builds: L010F8P6, L011K8T6,
L012C8T6, F030C8T7, L031C8T6 and L083RCT6. Five ELF links passed: native
`uart-idle/receive` on L010F8P6/L011K8T6/L012C8T6, L012C8T6 `uart-idle/split`,
and the unchanged L083RCT6 `uart-dma/shared_vector` regression example.
All eleven Cargo commands exited zero. The four finite source/data gates
also passed: native idle source verification, provenance, functional-coverage
consistency and module layout. Source inputs and all 905 generated data/PAC
files stayed unchanged during the main matrix; no main generation ran.

The three existing unused RCC constants `RCC_LSE_SYSCLK_LSE_EDGES`,
`RCC_LSE_SYSCLK_LSI_CYCLES` and `RCC_LSE_SYSCLK_MARGIN_LSE_EDGES` remain as
warnings in each L010/L011 library and receive build. The other main rows
are warning-free. These are software compile/link and source/data results;
no hardware execution, timing, injected error or cancellation run is claimed.

At main acceptance, final-source clean replay had not run. Final-package
completion requires a separate receipt for one bootstrap generation,
L011K8T6/L012C8T6 libraries, their two receive ELFs and the same four gates.
The regenerated 905-file hardware projection must match the accepted main
projection. Main results do not substitute for that clean receipt.
