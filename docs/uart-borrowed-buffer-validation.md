# UART borrowed receive buffer validation

Date: 2026-10-14. Base: `10f646ff1aa9666fc589df008848520235a9b1df`.
This receipt covers only the explicit borrowed IRQ receive-ring API. Earlier
constructor, SPI/UART and BufRead receipts remain historical records.

## Change and safety boundary

Four explicitly unsafe constructors are added:

- `BufferedUartRx::new_borrowed` and `new_borrowed_with_rts`.
- `BufferedUart::new_borrowed` and `new_borrowed_with_rtscts`.

They borrow `&'d mut [u8]`; the RX owner carries its lifetime marker, while the
static ISR state holds only a raw storage descriptor. Existing safe static
constructors remain available with unchanged signatures. There is no explicit
buffer-return method. Standalone RX Drop permits ordinary buffer reuse.
Combined split halves share `'d`, so a TX half still in use can keep the buffer
borrow alive after RX Drop; dropping both halves permits reuse.

This aligns normal borrowed-buffer/Drop usage with
[embassy-stm32 at the checked revision](https://github.com/embassy-rs/embassy/blob/ad0d0dbd2dd4339ec17867e01d380323f52d6591/embassy-stm32/src/usart/buffered.rs),
but does not copy its safe nonstatic constructor signature. A forgotten RX
owner or containing future can bypass Drop while the collector remains enabled.
The unsafe caller must keep the ring allocated, stationary and exclusively
reserved until reception stops and the ISR ring is detached. Without shutdown,
that reservation lasts forever. Merely adding `PhantomData` would not establish
this guarantee.

No TX software buffering, DMA staging changes, SPI changes, hardware throughput
claims or new permanent test framework are included. Temporary host/public-API
harnesses are not shipped.

## Executed validation

- 113/113 ARM release library commands passed: 54 chip selections with `rt` and
  `rt,defmt`, plus five time-driver configurations, on Rust 1.99.0.
- All eight UART example build groups linked. The new synchronous F002
  `borrowed` binary additionally linked explicitly at the final source revision.
  It constructs RX over a local array, drops RX, then overwrites the same array;
  its panic handler disables interrupts and never resumes execution.
- 15/15 actual public-API compile expectations passed against the final source:
  five positive cases cover standalone/combined Drop reuse, disjoint
  `split_ref`/`fill_buf`, and RTS/RTSCTS construction. Ten negative cases reject
  live-buffer reuse, owner/prefix escape, missing `unsafe`, local storage passed
  to a safe static constructor, consuming while a prefix is live, and buffer
  reuse with a still-used combined TX half. Negative cases are expected compiler
  rejections, not runtime tests.
- Actual ring-source host tests passed 5/5. The same five-test regression suite
  passed Miri Stacked Borrows 5/5 and Tree Borrows 5/5.
- A new actual-source nonstatic stack-buffer test passed Miri Stacked Borrows
  1/1 and Tree Borrows 1/1. It appends while a disjoint occupied prefix is held,
  detaches/drops the raw ring descriptor, then reuses the local array.
- Exact source-body comparison against the base confirmed `shutdown`, ring
  `observe`, `advance`, `read`, `readable` and `consume` unchanged. Independent
  review confirmed RX IRQ masking, RX/RTS disable and pointer detachment remain
  ordered within one critical section, before buffer reuse.
- `git diff --check` passed.

Miri used Rust `1.101.0-nightly (822182db4 2026-10-13)`, with aliasing
checks enabled. Extracted helper differences from the final source were
comments only; noncomment tokens matched.

The ARM matrix ran while constructor rustdoc wording was refined; executable
logic was unchanged. Public-API checks and the explicit borrowed example were
rerun at the final source hash. Existing generated RCC LSE warnings remain;
temporary host extraction can also emit synthetic cfg warnings. Neither is
claimed as a warning-free build.

Final source SHA-256:

- `embassy-cw32/src/usart.rs`:
  `2207f9830bbf436952a3a194dc53b7fe54d7f01d8919141cc852c56dfb5e879d`
- `examples/shared/uart-buffered/src/bin/borrowed.rs`:
  `dbd650c3fbda1e5021b871f29f248940564af3f0c0b1cadbc896934a39cbee44`

## Not validated

No hardware UART traffic, interrupt timing, cancellation on a real executor,
register side effects, loss/throughput behavior or physical teardown was tested.
Miri exercises extracted actual ring helpers, not the real interrupt controller
or complete hardware driver. Compile checks establish borrow/API constraints,
not the caller's unsafe no-leak promise. No remote CI pass is asserted.
