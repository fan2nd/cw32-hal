# Safe staged UART RX and split full-duplex DMA

F030/A030/L083 and fourteen exact F020/L031/L052/R031/W031 parts expose
`UartRx::new_with_dma` and `Uart::new_with_dma`. The classic extension uses the
existing receive algorithm and [each family's own byte-RDR qualification](classic-uart-dma-rx.md).
L012 remains TX-only for UART DMA. Stage92 main passed eight standalone ARM
libraries and 21 ELF links without warnings; independent runtime and metadata/source reviews accepted the bounded main result.
At main acceptance final capability reports and the separate two-library/three-ELF
clean replay had not run. Prior copy/TX receipts do not verify this extension. The existing
`UartRx` owns the receive lease, while the combined constructor prepares the
UART once and builds its existing TX/RX halves. `split` and `split_ref` remain
the concurrent interface. Standalone and combined constructors require static
UART/pin/channel tokens, exact-part qualified SRAM and exclusive static staging
of 1..=65535 bytes per direction. Both combined buffers and UART configuration
are validated before atomic admission of both channel capabilities, then UART
preparation. Rejected inputs are consumed. Normal reset/clean-runtime startup
and permanent exclusion after raw DMA takeover retain the Stage33 contract.

Ordinary `read(&mut [u8]).await` accepts a local caller buffer. DMA only writes
the consumed private SRAM allocation. Its pointer and capacity are retained
without forming references while DMA may write. Clean chunks are synchronously
copied into the caller. There is no borrowed zero-copy receive API.

## Persistent receive ownership

- Idle: no unresolved DMA. A chunk can be armed or CPU reception used.
- InFlight: stores programmed length, highest observed line error and whether
  that error was reported. DMA owns staging and RDR; the CPU reads neither.
- Ready: recorded TC without TE and current Complete status were accepted,
  EN/TCIE/TEIE read back clear, DMARX closed and read back clear, RX flags
  sampled and acknowledged, then DMA fenced and retired. Failed channel or
  request-gate readback permanently quarantines the lease before CPU copying.
  Only this state permits staging copies. Partial consumption keeps the tail.
- Quarantined: DMA TE or ambiguous completion permanently retains resources.
  Later TC never restores staging, channel, pin or clock availability.

Before launch, DMARX is off and only RX flags are initialized; preexisting
PE/FE is returned as an error. InFlight is published before DMA EN. The private
`Channel::start_read` uses the common DMA plan/start path, fixed typed RDR source,
incrementing byte destination, REPEAT=1 and generated request metadata. DMA EN
precedes DMARX, matching the own-manual sequence. Unexpected launch failures
conservatively quarantine. No caller-buffer pointer reaches DMA.

A polled read that is cancelled or forgotten leaves its current chunk running.
A later inherent read settles that chunk, retains a clean Ready tail and can
then arm more chunks. Already copied caller prefixes remain; incomplete chunks
never touch caller memory. Empty inherent reads may await old work but do not
consume Ready bytes. They are not abort operations.

## Line errors, completion and reads

Every DMA await registers UART and DMA wakers before final status observations.
Only PE/FE waiter interrupts are armed; RC remains masked. The UART ISR only
masks/wakes, leaving RDR and status intact. A line error returns promptly even
if the peer stops before completing the DMA count. The request remains running,
the invalid chunk is retained, and its error is reported once. A later inherent
read can await the old count and discard it only after clean DMA completion.
If the peer sends no more bytes this recovery can wait indefinitely.

DMA TE/ambiguous completion takes precedence and quarantines. Otherwise parity
precedes framing and both precede accepting data. FE/PE is sampled again after
clean DMA completion and DMARX closure. An invalid clean chunk retires safely
but is discarded; an unreported error is returned. A new error after the final
snapshot remains for the next reception. There are no per-byte error tags.

Each qualified family's own SDK confirms that reading RDR does not clear RC.
The five classic families expose FE/PE/RC without ORE/NE; their own-source
locators and selected typed ICR no-op values are recorded in
[classic RX evidence](classic-uart-dma-rx.md#positive-own-source-receive-authority).
Therefore stale RC after DMA is acknowledged with the generated R1W0 no-op command and never read as an
extra byte. Selected RX acknowledgments preserve TX/CTS and reserved fields.
No line-error path treats a UART flag, EN=0, cleared DMARX, zero count, TE or a
stop-request status as proof that DMA bus accesses drained.

`blocking_read` first makes one nonblocking selected-channel observation,
servicing flags even with CPU interrupts masked. Pending yields DmaBusy and
fault/quarantine yields an error. After clean completion it copies Ready bytes
and uses CPU RDR reads only after the DMA lease is idle. It starts no new DMA.
The `embedded_io` and `embedded_io_async` Read implementations retain one-byte
short reads. A previously abandoned pending chunk yields DmaBusy rather than
secretly waiting for a longer count. Empty trait reads make at most one such
nonblocking observation and return zero or an ordinary error. They never await
recovery. The accepted immediate empty async Write trait behavior is preserved.

## Split ownership and Drop

UART state tracks live and retained TX/RX ownership separately under a critical
section. Shared CR2 and IER read-modify-writes use that same protocol. An abandoned
TX cannot disable live RX dispatch or erase its DMARX/interrupt updates, and an
abandoned RX cannot disable live TX. Dropping either direction masks only that
direction's waiters. Clean/idle halves disable only their TXEN/CTS or RXEN/RTS
and release pins. Unresolved/quarantined halves forget private DMA/staging/pins
and preserve requests, UART configuration and clocks. RX Drop checks its phase,
not merely whether the last result was a line error: a clean invalid chunk is
safe to discard, an unresolved invalid chunk is still retained.

When neither half is live, the UART dispatch guard becomes inactive. UART clocks
remain on if either direction is retained. No late DMA completion resurrects an
abandoned owner. Only when neither direction is live or retained is the UART
clock gated. L083 keeps its existing shared-vector rule: never disable or unpend
a partner's vector, and skip inactive partners before clock-gated MMIO. The DMA
controller clock and other channels are never reset/gated by this lifecycle.

This is finite explicitly armed reception, not continuous/circular/ring/idle-line
RX. Frames may be overwritten or lost during executor, copy and rearm gaps.
These qualified UARTs have no overrun indication, so no lossless, throughput,
bounded-liveness or early-abort claim is made. No safe return of consumed static resources
or RTS/CTS DMA is introduced. L083 UART1–6 are independently qualified in
[the own-family source record](l083-peripheral-dma.md), including the shared
UART vectors and all five exact parts.

## Own-source evidence

The five classic families have independent positive byte-RDR, software RC-clear,
FE/PE and package evidence in [classic RX](classic-uart-dma-rx.md) and its
[source ledger](classic-uart-dma-rx-evidence.json). Existing pin exclusions
remain: F020F6U7/L031F8P6 have no safe UART1 direction, while L031F8U6 has
UART1 RX-only on PA9/AF2 or PC15/AF6, with no safe UART1 TX.

L083 RX and shared UART interrupt evidence is documented separately in
[L083 staged peripheral DMA](l083-peripheral-dma.md); the following x030
references do not substitute for that own-family qualification.

The authoritative shared F030/A030 manual is
[CW32x030 User Manual CN V2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
cover 2024-09 / revision history 2024-07-24, selected in
`sources/evidence-sources.json`. Rechecked PDF SHA-256:
`1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`;
extracted text SHA-256:
`23935a54e976c25acfacd689818f272db88b579fc8f15363c2732c12da8773fe`.
Printed page equals zero-based PDF index; viewer page is printed page plus one.

| Manual location | Evidence and implementation consequence |
| --- | --- |
| §8.4.1 p127 | Descriptor, trigger, increments, widths and count precede EN. |
| §8.4.4 pp130–132, Table 8-2 | BLOCK per request and UART RX selectors; implementation consumes existing generated routes. |
| §8.5 p134 | Equal width, 1–65535 count and rewriting REPEAT=1. |
| §8.6 p135; §8.8.3 p139 | TC means correctly completed data; Complete STATUS is 101; TE/EN do not document drain. |
| §8.8.2 p138 | DMA R1W0 ICR and 0xffffffff no-op seed. |
| §18.3.3.4 p338 | FE/PE/RC events; unread RDR can be overwritten, no overrun report. |
| §18.6 p344 | DMARX handshakes RDR to memory until the count completes; DMA TC follows. |
| §18.7.1.6 p350, steps 9–25 | Explicit uint8_t SRAM RX sequence: fixed RDR, incrementing SRAM, byte BLOCK, REPEAT=1, EN then DMARX, TC then DMARX off; initialize flags again per chunk. |
| §18.9.2 p356; §18.9.6 p357 | Typed DMARX control and RDR endpoint. |
| §18.9.7 p358; §18.9.8 p359; §18.9.9 p360 | Independent FE/PE/RC enables/status and R1W0 UART ICR with 0xff no-op seed. |

The locked [F030 SDK V2.2](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip)
`Libraries/src/cw32f030_uart.c:316–318` explicitly says RC is cleared only by
software, not by reading RDR. Rechecked member SHA-256:
`69b062544d4e5fb16927d6388412a0ca0ddb752d17821e35dfcf3f23d982cd0d`.
Existing RX request metadata retains its EN V1.0 page references; the CN V2.5
locations above are an independent check, not relabeled metadata citations.

Pinned Embassy `f16efeffe37581092ec184718e6fdb1620393214` provides the constructor,
ordinary borrowed method and split shape. Its STM32 borrowed DMA/OnDrop/channel
reset behavior does not prove CW32 early stopping. See the actual
[embedded-io Read contract](https://docs.rs/embedded-io/0.7.1/embedded_io/trait.Read.html)
and [async Read contract](https://docs.rs/embedded-io-async/0.7.0/embedded_io_async/trait.Read.html)
for immediate empty and short-read requirements.

[Real full-duplex and standalone RX firmware](../examples/shared/uart-dma/README.md)
uses exact F030C8T7/A030C8T7, all five L083 selections and the fourteen
classic selections above. Their separate [Stage92 main results](classic-uart-dma-rx.md#application-and-verification-boundary)
cover fourteen combined and five selected standalone classic images. Earlier ARM
compile/link receipts retain only their recorded scope, not hardware
execution or error injection. No HAL tests, synthetic register models or
negative harnesses are added.
