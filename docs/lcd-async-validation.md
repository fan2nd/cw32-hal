# LCD asynchronous frame wait validation

Date: 2026-10-14. Base: `35dc4d8131dcc84a61c6a5a431f5a0a7bf2faad8`.

## Scope

`Lcd::wait_next_frame_async` and `present_async` extend the existing qualified
L052/L083 frame observation protocol. No new clock, pin, bias, pump or display
latch capability is introduced. A stored task waker is taken by the existing
LCD interrupt handler and woken outside the lifecycle critical section.
Blocking and asynchronous frame waits share a cleanup guard. Completion,
cancellation and owner Drop retire the local waiter without disabling or
unpending the shared AUTOTRIM_LCD vector. A subsequent wait or successful
reconstruction also retires a forgotten waiter.

## Executed software checks

- All eight existing LCD exact-chip selections linked both blocking and async
  binaries with the final LCD/AUTOTRIM coexistence example. These are eight
  Cargo build commands, not eight emitted ELFs. The wider batch also passed
  113 ARM library and 91 example build commands; see the [batch receipt](unblocked-hal-extensions-validation.md)
  for distinct command accounting, warnings and exclusions.

- Independent source review found no blocking lifecycle issue: an event before
  waker registration stays latched, registration/readiness checking serialize
  with the handler, and cancellation changes only LCD's own request/status.
- Nine actual-source host tests passed. The temporary harness extracts State,
  the interrupt-handler body, FrameWait, the async wait body and owner Drop;
  only the hardware register interface and surrounding type signatures are
  replaced with host stand-ins. It covers pending cancellation, an unpolled
  future preserving a preexisting hardware flag, IRQ wake/completion, event
  before registration, event observed before handler dispatch, replacement of
  a registered waker, forgotten wait followed by restart, forgotten wait
  followed by owner Drop, and inactive handler preserving an unrelated event.
  A sentinel representing the other shared-vector client remains unchanged.
- The final host suite was also compiled directly with Rust 1.99.0 against the
  same critical-section dependency and passed 9/9. This bypassed a shared Cargo
  cache lock; it did not change the tested source.

- Actual production-API ARM borrow checks on L052 passed all three expected
  outcomes: sequential async waits compile, two simultaneously live wait futures
  are rejected with E0499, and dropping the display while its future is later
  used is rejected with E0505. These are compile-time ownership checks, not
  runtime or hardware tests.

Source SHA-256:

- `embassy-cw32/src/lcd.rs`:
  `0a96df0d3d2634b7a3988d15e6ccf1db404be4cafd0494e830804373386edc8c`
- `examples/shared/lcd/src/bin/async.rs`:
  `b52960cabb4aa157fb4a0862f5560e08993f3c44a3e9e0e4c602fed4cd89a5d3`

The async LCD example also constructs the qualified AUTOTRIM async counter
and binds both handlers to AUTOTRIM_LCD. This demonstrates the intended
shared-vector composition; it is not a measured simultaneous-IRQ result.

## Limits

The host harness is not the silicon, interrupt controller or an Embassy
executor. No board was flashed, and no electrical timing, display waveform,
clock-loss wakeup or atomic display update was tested or promised. Async waits
have no built-in timeout. The original polling budget still applies only to
synchronous startup/frame waits. Hardware/source qualification remains as
recorded in `lcd.md` and `lcd-evidence.json`.
