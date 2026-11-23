# L012 HALLTIM asynchronous latest capture

`HallTim::wait_capture_async(&mut self, binding)` waits for one coalesced
capture and returns the unchanged `Capture` structure. It requires the timer
to be running; a stopped call returns `Error::Stopped` and retires any forgotten
wait. Existing blocking construction and polling methods remain available.

Own source: CW32L012 CN manual V1.4, 2026-06-03, SHA256
`a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
Section 18.8.2 printed p422 specifies CAPIE; table 5-1 p71 specifies IRQ22
BTIM3/HALLTIM. Existing capture, overflow and clear semantics remain those in
[the original source audit](halltim-evidence.md), not new hardware assertions.

Bind `halltim::InterruptHandler<peripherals::HALLTIM>` and the BTIM3 handler
in the same `BTIM3_HALLTIM` dispatch when both sources are enabled. The example
`examples/l012/halltim/src/bin/async_capture.rs` compiles this paired binding.
The handler checks RAM waiting state before register access; it masks only
CAPIE and wakes outside the critical section. It never clears CAPF/OVF or reads
WIDTH/STATE. The future delegates sampling to unchanged `try_capture`.

This remains a latest-observation API: CAPF coalesces, WIDTH can be overwritten,
STATE is sampled later and overflow has no provable temporal ordering. It is
not a queue, edge counter, atomic timestamp or motor-direction proof. Waiting
registration and CAPIE arming share a critical section. Cancellation disarms
CAPIE, preserving pending events and the running counter. Forgetting can leave
CAPIE armed but stores no caller pointer. Ordinary reconstruction, restart and
Drop retire waiting state before reconfiguration or clock/pin release. Stop
retains pending capture; it does not drain a physical pipeline.

Only existing L012 generic/C8T6/C8U6 support and qualified pins are admitted.
There is no DMA, extra trigger, filter, low-power or timing guarantee. Software
validation receipts are separate; no board execution is claimed.
