# Native LIN paired ownership

`LinUart` consumes one qualified UART plus TX/RX pins. Scope is L010/L011 UART1
and L012 UART1/2, using the same fixed-baud 8N1 validation as the independent
owners. It prepares the peripheral once, constructs both direction guards while
disabled, programs LIN/BRKL, then enables TX and RX together. No ordinary-UART
receive window or second reset is introduced. The async constructor enables the
bound IRQ only after both owners are active.

`transmitter`/`receiver` borrow a direction; `split_ref` permits concurrent polling;
`split` transfers both LIN-specific owners. Neither exposes ordinary UART or a
mode change. Each half retains its existing cancellation contract. Its Drop masks
only its requests, disables its direction and disconnects its pin. Existing UART
lifecycle accounting keeps the clock and IRQ usable for the other live half.
Forgetting a half retains its lifecycle ownership and can keep the clock enabled.
A queued TX may be truncated on Drop. There is no combined atomic transaction.

RX remains enabled during transmission. Echo may cause bytes, break events or
overrun; writes do not drain it and reads do not suppress it. Concurrent polling
can reduce service gaps but cannot guarantee ordering or lossless reception.
External transceiver controls, arbitration, collision handling, response contents,
oscillator tolerance and deadlines belong to the application. UART flush proves
neither external bus release nor LIN response timing.

`examples/shared/uart-lin/src/bin/respond.rs` uses UART1 PA1/PA0 on L010F8P6 and PA2/PA3
on L011K8T6/L012C8T6/L012C8U6. A deliberately configured test network must assign
IDs 0x12 (two bytes) and 0x13 (three bytes), both with enhanced checksum,
exclusively to this publisher. The example calls `respond_once`, which waits
for Break/sync/validated PID, writes the selected response, then waits
for another Break. It does not compare echoed bytes, detect collisions or enforce
header/response deadlines; delayed header bytes may therefore still cause a reply.
It is not a production scheduled responder. UART errors abandon header state.

`LinResponder` borrows an immutable response table and rejects duplicate IDs
before retaining it. The table may be empty. A new Break replaces any partial
header, Sync must be 0x55, and PID validation uses the existing identifier
decoder. Unknown IDs and trailing bytes are ignored until the next Break.
Selection returns an owned response copy, so later parsing cannot alter bytes
already chosen for transmission. Errors reset the header. Direct selector users
must call `reset` on timeout, UART errors or known event loss.

`LinUart<Async>::respond_once` combines this selector with RX, response TX and
flush. Duplicate tables fail before UART access. It first flushes earlier queued
TX, then waits for one matching header, sends the selected payload/checksum and
waits for UART idle. The parser is local to that call: cancellation or forgetting
cannot cause the next call to reuse a partial header. Transmission cancellation
can still leave a queued prefix; no automatic resend is performed. Header or
UART failure is returned explicitly. An empty table never selects a response
and can wait indefinitely. A caller deadline must cover header reception and
transmission if the application needs a bound.

RX is not drained or compared during this operation's TX waits. Echo may cause
overrun and an error on the next attempt. Success denotes UART completion only,
not peer receipt, collision freedom, LIN timing compliance or diagnostic session
validity. Table entries must be exclusively assigned to this publishing node;
validated PID/checksum bytes do not authorize the node's network role.

`respond_once_with_echo` is an opt-in variant for transceivers that return each
transmitted byte on RX. It shares header/table admission with `respond_once`,
queues one response byte, then waits for its matching receive event before
queuing the next byte. The checksum is included. A differing byte reports
`EchoMismatch` with zero-based offset, expected byte and received byte; a Break
reports `EchoBreak`. UART errors remain UART errors. Each failure returns before
another response byte is queued. The already queued byte or prefix can still
finish transmitting. A Break consumed by echo checking is not silently reused
as another request; a subsequent attempt requires a fresh parsed header.

No RX events are drained as presumed echo, and an empty/missing echo does not
become success. A caller deadline is needed for finite waiting. Matching data
cannot prove its sender, detect two publishers sending identical bits, or
establish that no electrical collision occurred. This is byte comparison,
not full collision arbitration or recovery. The per-byte await adds scheduling
gaps; the existing baud setting does not bound response space or inter-byte
latency. The `respond_with_echo` example requires an echoing transceiver and
parks on errors so it cannot automatically retry a potentially transmitted
prefix. It intentionally has no built-in timer and can wait forever if echo
is missing. Deployment requires an application deadline and schedule policy.

`timed_respond_with_echo` demonstrates deadline composition with the optional
`timed` example feature. It enables the existing GTIM1 time driver and uses the
same pinned Embassy revision as the executor. Select `timed` together with one
supported exact chip feature. GTIM1/its IRQ are reserved by the time driver;
default HSI/PCLK and the documented operating conditions must be retained.

One 100 ms application deadline covers the whole attempt: initial TX retirement,
header selection, payload/checksum echoes and final flush. Unknown IDs or byte
progress do not restart it. Missing headers and missing echoes therefore have
a caller cancellation path when the timer and executor make progress. After
completion, error or timeout, the example separately bounds a TX flush to 10 ms
and parks without retrying a possibly transmitted prefix. The UART owner is
retained while parked; a failed/timed-out flush is not claimed as retirement.

These values demonstrate API use, not a LIN response-space, inter-byte or network
schedule specification. A timeout future is cooperative: it cannot preempt a
running poll or recover a failed clock/IRQ/executor. Deadline observation races
must not be interpreted as wire-level timestamps. Source review covered the
fixed Embassy version's operation-first timeout polling; an already-ready UART
operation may win a simultaneous timeout. Source review also covered the
existing receive-wait cancellation (request masking without draining RDR) and
TX semantics (queued prefix can continue; flush waits for UART completion).
No local build, dependency resolution, cancellation test or board run was done.

Hardware evidence is the same own-manual LIN format, receive and send chapters
listed in [TX](uart-native-lin-tx.md) and [RX](uart-native-lin-rx.md); no new register
encoding is introduced. Ownership/interrupt behavior was inspected against
`Info::prepare`, `Info::shutdown`, `UartTx::from_pin`, `UartRx::from_pin` and the
one-shot IRQ dispatcher. Compilation, tests, paired cancellation/Drop acceptance
and board timing remain pending under the local execution restriction.
