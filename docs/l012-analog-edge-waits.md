# L012 LVD and comparator edge waits

## Source and qualified scope

Source: CW32L012 User Manual CN V1.4, published 2026-06-03,
https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf
SHA256 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
Printed page numbers are used below (PDF page = printed + 26).

- §28.4 p638 and §28.7 pp642–643: LVD ACTION=0 selects interrupt rather than
  reset; CR1 IE gates the request, RISE/FALL select physical monitored-voltage
  threshold crossings. INTF is write-zero-to-clear; FLTV is read-only. IRQ1 is
  LVD (§5 p70). RISE is a voltage rising above threshold, **not** FLTV rising.
- §27.4 p624 and §27.7.6 p632: comparator CR0 IE gates requests; CR1 RISEIE/FALLIE
  select post-polarity output edges. HIGHIE is disabled. SR INTF is W0C. VC1/VC3
  share IRQ14 (VC13), VC2/VC4 share IRQ15 (VC24), §5 p71.

Only `cfg(cw32l012)` exposes these APIs. Both exact supported L012 packages and
its existing family alias have the same qualified interrupt/register layout.
No similarly named L010/L011 or classic register layout implies IRQ support.

## Public API and ownership

`LowVoltageMonitor::wait_for_edge(lvd::Edge, binding)` uses the existing supply
or qualified external-input monitor. `Comparator::wait_for_edge(comparator::Edge,
binding)` uses existing external/external or external/bank-qualified-divider
configuration. Both return `()` asynchronously. Edge choices are Rising,
Falling and Either; no level waits are added.

Bind LVD to `lvd::InterruptHandler`. Bind each used comparator to
`comparator::InterruptHandler<VCn>`. If VC1 and VC3 are active, both handlers must
be bound to VC13; likewise VC2/VC4 to VC24. Dispatching an inactive channel does
not touch peripheral registers. Neither driver disables or unpends an NVIC
vector, resets a shared analog block, disables a reference, nor gates VC clocks.

Each future borrows its owner mutably. On first poll it retires an abandoned
request, masks IE, selects edges, clears INTF, publishes active state and then
registers a waker and enables the request. A wait that is never polled has no
effect. An already pending event is deliberately discarded: this is a fresh
observation window, not a persistent event queue. A boundary transition can be
coalesced or discarded by the initial clear. Completion means a sticky event
was observed, not a current level, direction history, edge count or timestamp.

The handler masks only its own IE and takes the waker, leaving INTF for the
future. Completion and cancellation disable the owned edge selectors/request,
clear its pending INTF and retire software state. EN remains on until owner
Drop. Forgetting the future retains no pointer into its storage: a next wait or
owner Drop retires it. Forgetting the owner can retain enabled analog hardware;
it is not an implicit power-off. Comparator reconstruction retires its old
waiter before reconfiguration; LVD reconstruction instead preserves its existing
no-write `AlreadyConfigured` rejection while inherited EN/policy is active.
Waker clone/drop/wake callbacks execute outside the critical section and outside
state RefCell borrows.

## Analog and safety limitations

All existing source, threshold, divider borrow, polarity, speed and hysteresis
rules remain unchanged. No filter, blanking, window, output route, BGR/DAC source,
reset/protection policy, stop-mode wake, calibrated threshold, startup maximum,
analog readiness or real-time latency guarantee is added. L012 comparator
`Readiness` remains `Unknown`. Qualify settling and the board's voltage limits
separately. LVD thresholds retain the documented Vcore=1.6 V assumption.

Examples `examples/l012/analog/src/bin/{lvd_edges,comparator_edges,
comparator_divider_edges}.rs` exercise the API and both shared comparator
vectors. The comparator examples use four concurrent waits, then sample output
separately; this is not a simultaneous voltage snapshot. Startup transients may
produce events. These examples are not safety controls or hardware qualification.

## Validation record

Source changes initially passed L012C8T6 library checking and all three new
example release links. Final matrix and source-extracted lifecycle checks are
recorded separately by the batch validator; no hardware execution, threshold
accuracy, settling, interrupt latency or low-power wake test is claimed here.
