# x030 finite continuous acquisition

`Adc<Blocking>::blocking_read_continuous` starts MODE2 once and collects a
nonempty buffer from one unbuffered external channel. F030/A030 are
covered by this source audit; the [family audit](adc-classic-continuous.md)
extends the API to the other classic families. Existing voltage, reference, clock and sealed-pin constraints apply.
Internal and buffered inputs are rejected before ADC writes; GPIO preparation
can precede rejection. The timeout budget is per sample. An error invalidates
the caller buffer, which may already contain a prefix.

Evidence is CW32x030 User Manual CN2.5, pinned in
`sources/evidence-sources.json`, SHA-256
`1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`.
Page references below are printed / PDF:

| Contract | Pages |
| --- | --- |
| MODE2 repeats CHMUX conversions until START=0; each completion sets EOC and writes RESULT0 | 443 / 444 |
| MODE=2 encoding | 461 / 462 |
| DISCARD=0 replaces unread old results; right alignment | 462 / 463 |
| OVW means a new conversion completed before the previous result was read | 467 / 468 |
| ICR is R1W0, RESULT0 is read-only | 468 / 469 |

The collector observes EOC, clears only EOC, reads RESULT0, then checks OVW
before publishing a sample. OVW is never acknowledged during collection.
Clearing EOC after reading would risk erasing the next sample's notification.
With the chosen order, a completion between EOC clearing and RESULT reading
encounters unread old data and raises OVW under the documented semantics.
A completion after the read remains notified for the next iteration. A
completion before acknowledgment also encounters unread old data and fails
the sticky-OVW check. There is no software critical section assumption about
ADC progress. Exact simultaneous bus/conversion behavior remains a silicon
acceptance item; no uninterrupted-service guarantee is made.

Both success and acquisition error use the existing bounded cleanup protocol:
mask completion requests, disable triggers, request START=0, read back controls,
disable EN, drain and clear completion state. Cleanup failure takes priority
and latches `Faulted` until reset. This is control retirement, not an analog
abort-latency proof. Shared BGR and inherited clock ownership are retained.
Later single/scan/continuous reads initialize afresh. Results arriving after
the requested prefix has been collected are discarded during retirement.

`examples/shared/classic-adc-scan/src/bin/continuous.rs` collects 32 samples from the
first package-qualified generated input and consumes them only on success.
It then attempts a single internal supply read after continuous retirement.
The family audit also describes `read_continuous`, the IRQ-driven equivalent.
Continuous scans, hardware triggers and native L010/L011/L012 remain excluded.

Only source review and diff whitespace checks were performed. PAC regeneration,
compilation, tests and board acceptance remain pending under the local execution
ban. The authored register enum gains CONTINUOUS=2; stale generated PACs do not
contain that variant and must be regenerated when execution is permitted.
