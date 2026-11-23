# Native comparator PCLK filtering

L010/L011/L012 `comparator::Config::filter` accepts fifteen `DigitalFilter`
settings. `None` preserves unfiltered behavior. Both external-input and shared
reference constructors configure FLTCLK=PCLK and FLTTIME while CR0 is disabled,
then enable the comparator through the existing path. Requested filtering
requires initialized RCC clocks before any MMIO or pin write. Filter selection
is read back before enable; a mismatch requests bypass and returns
`FilterWriteFailure`, releasing the constructor's pin locals. Shared clock,
reference and sibling state are retained. Drop retains the existing disable
and IRQ retirement behavior; it does not stop PCLK or reset filter hardware.

The names specify PCLK division and consecutive equal samples, not a guaranteed
analog settling time. `output()` reads FLTV, so enabled filtering affects both
polling and the native edge waits. Readiness remains Unknown. Initial
filter history, input propagation, clock phase and board noise need application
qualification. An edge wait clears the sticky event, not filter history. No LSI,
low-power wake, output routing, blanking or protection policy is introduced.

Own-manual evidence, hashes pinned in `sources/evidence-sources.json`:

| Part/manual | Filter behavior | FLTCLK/FLTTIME table | Initialization |
| --- | --- | --- | --- |
| L010 CN1.2 | §21.3.5 p530/PDF531 | §21.7.3 p538/PDF539 | §21.5 p533/PDF534 |
| L011 CN1.1 | §21.3.5 p532/PDF533 | §21.7.3 p540/PDF541 | §21.5 p535/PDF536 |
| L012 CN1.4 | §27.3.5 p621/PDF647 | §27.7.4 p629/PDF655 | §27.5 p625/PDF651 |

Each own table assigns 1..15 to /1×2, /1×4, /1×8, /2×6, /2×8,
/4×6, /4×8, /8×6, /8×8, /16×5, /16×6, /16×8, /32×5, /32×6,
/32×8 respectively. FLTCLK=1 selects PCLK. Tables were read and visually
reviewed; canonical per-family register layouts remain unchanged.

`examples/l012/analog/src/bin/comparator_filtered.rs` uses VC1 PA0/PA2 with
/32×8 filtering. Local compilation and tests remain prohibited; register
readback failure, shared-resource behavior and board timing acceptance are pending.
