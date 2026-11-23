# L012 Hall capture

`capture` is normal Cortex-M firmware, linked for CW32L012C8T6 or CW32L012C8U6.
Connect Hall sensor CH1/CH2/CH3 to PA3/PA4/PA5 and common ground, within the board's
GPIO supply limits. This example assumes push-pull sensor outputs. Open-collector
sensors need suitable pull-ups; choose these according to the actual sensor and
board. No motor outputs or power stage are controlled.

The driver polls a latest-sample register, not a FIFO. A new edge overwrites WIDTH
and resets the 24-bit counter. Input state is live and not atomic with WIDTH.
The transition description is a difference between software observations, not a
verified physical edge sequence. Overflows remain sticky until `restart`.

Build with `cargo build --offline --locked --target thumbv6m-none-eabi
--no-default-features --features cw32l012c8t6 --bin capture` from this directory.
Replace the feature with `cw32l012c8u6` for QFN48. Do not flash without checking
board wiring. Compiling/linking does not establish silicon validation.

`async_capture` awaits `wait_capture_async` using the shared BTIM3/HALLTIM
handler pairing. It retains the same non-atomic latest-sample semantics;
see [the async contract](../../../docs/halltim-async.md).

`history` uses the eight-entry IRQ observation queue. It drains oldest records,
handles software overflow explicitly, and restarts collection after counter
overflow. It is not a complete physical edge log; the new path is uncompiled
and untested. See [history contract](../../../docs/halltim-history.md).
