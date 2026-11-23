# Native comparator edge waits

L010/L011 VC1 and VC2 expose the existing `wait_for_edge` API with dedicated
interrupt bindings. L012 retains its VC13/VC24 shared-vector bindings. Build
metadata must supply VC1 IRQ14 and VC2 IRQ15 on L010/L011; a mismatch is rejected
by the build script. No generated data or PAC was regenerated for this change.

The owner accepts rising, falling or either output edge. First poll retires any
forgotten prior wait, disables local IE, selects edges with HIGHIE disabled and
clears INTF by writing zero. The ISR checks its active owner, IE and INTF, masks
only local IE and wakes outside the critical section. Poll reads the sticky flag;
completion/cancellation retires local selectors, IE and INTF. NVIC pending state,
sibling registers, reference ownership, filter settings and clock gates remain
untouched. No caller memory pointer is stored in the ISR state.

An event is coalesced, not an edge count, timestamp or current level. Startup
and analog settling remain Unknown. With PCLK filtering enabled the output edge
is filtered; clearing INTF does not reset filter history. There is no timeout,
LSI qualification or low-power wake guarantee. Only one wait can borrow an owner.

Own-manual evidence (exact hashes in `sources/evidence-sources.json`):

| Part/manual | IRQ mapping | Event/selector behavior | W0C status |
| --- | --- | --- | --- |
| L010 CN1.2 | §5.4 p88/PDF89 | §21.4–21.5 p533/PDF534 | §21.7.4 p538/PDF539 |
| L011 CN1.1 | §5.4 p86/PDF87 | §21.4–21.5 p535/PDF536 | §21.7.4 p540/PDF541 |

Both approved SDK device headers independently name VC1_IRQn=14 and VC2_IRQn=15.
The existing canonical native CR0/CR1/SR field layouts provide typed IE,
RISEIE/FALLIE/HIGHIE and W0C INTF access. The L012 protocol remains as described
in [analog edge waits](l012-analog-edge-waits.md).

The three-feature example is `examples/shared/comparator-reference/src/bin/edges.rs`.
It keeps the existing qualified divider/pin routes and waits on both comparators.
Local builds and tests were not run. IRQ races, cancellation/forgetting, sibling
independence and actual board timing remain pending acceptance.
