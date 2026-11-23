# Owned BTIM overflow events

This source-qualified contract covers BTIM1, BTIM2 and BTIM3 on all thirteen
supported families and all 54 existing normalized selections. It adds typed
Async observation of internal-PCLK, continuous-upcounter ARR-overflow events.
Main verification passed 16 actual ARM library builds and 23 ELF links, with
all 39 commands exiting zero. Four finite source/data gates passed separately.
Independent main [runtime/ownership](btim-overflow-irq-runtime-review.json) and
[metadata/source](btim-overflow-irq-metadata-review.json) reviews accepted the
frozen implementation. At main acceptance clean had not run; final-package
completion requires a separate final-source 2-library/6-ELF receipt. Software
results do not establish hardware behavior; the verification history and limits
are recorded below.

## Cohort and own-source facts

The [current IRQ evidence](btim-overflow-irq-evidence.json) binds each family's
own manual, SDK and existing metadata. A030 explicitly shares the x030 manual
and admitted F030 SDK authority; other families use their own sources. The
54 selections include aliases, not 54 distinct physical devices. Each selection
has exactly three actual BTIM banks, giving 162 selection/instance associations.
This internal-clock function adds no package or pin qualification.

| Families | Canonical BTIM version | Overflow enable / status |
| --- | --- | --- |
| F030, A030, F020 | `v1` | `IER.OV` / `ISR.OV` |
| F002, F003 | `cw32f002_v1` | `IER.OV` / `ISR.OV` |
| L031, R031, W031, L083 | `cw32l031_v1` | `IER.OV` / `ISR.OV` |
| L052 | `cw32l052_v1` | `IER.OV` / `ISR.OV` |
| L010, L011 | `cw32l010_v1` | `IER.UIE` / `ISR.UIF` |
| L012 | `cw32l012_v1` | `DIER.UIE` / `ISR.UIF` |

Every overflow enable/status field is bit0. BTIM1, BTIM2 and BTIM3 use external
IRQ20, IRQ21 and IRQ22 respectively. Their typed associations come from the
selected peripheral's real `GLOBAL` metadata route. The three exceptional
shared-vector names are listed below; other admitted routes are dedicated.
Classic banks are at `0x40014800`, `0x40014900`, `0x40014A00`; native
L010/L011/L012 banks are at `0x40004800`, `0x40004840`, `0x40004880`.

ISR flags are read-only and ICR uses R1W0 semantics. Servicing overflow directly
writes `0x06` on classic-style OV/TI/TOP banks or `0x40` on native UIF/TIF banks,
preserving unrelated flags with reserved bits zero. It never writes ISR or
read-modify-writes ICR. Native URS=1 and UDIS=0 restrict the enabled update
request to counter overflow. ARR overflow is distinct from classic TOP wrap.
The existing prescaler/period limits, nominal-PCLK frequency selection and lack
of an APB timer multiplier remain in force.

## Public API and ownership

`timer::low_level::Timer<'d, T, M = Blocking>` retains its real `Peri<'d, T>`.
`Timer::new` returns `Result<Self, ConfigError>` in Blocking mode;
`Timer<'d, T>` annotations retain the default Blocking mode. Construction starts stopped, with 65536 ticks and no
prescaling. Blocking construction leaves requests disabled.

Only BTIM implements the sealed `timer::OverflowInstance` capability.
`Timer::<T, Async>::new_async(peripheral, binding)` requires
`Binding<T::Interrupt, timer::OverflowInterruptHandler<T>>`. It starts stopped
with only the selected bank's overflow request enabled. Call `start()` to count.
There is no mode-switch, peripheral extraction or GTIM/ATIM Async construction
path through this API.

- `wait_overflow(&mut self)` on Async BTIM returns `()` after consuming one
  coalesced event. Only one wait can hold the mutable borrow at a time.
- `take_overflow(&mut self) -> bool` on either BTIM mode consumes the software
  latch or hardware flag in one critical section. Hardware is acknowledged only
  when its overflow flag was observed.
- `is_overflow_pending()` observes software-latched or hardware overflow without
  consuming it. `clear_overflow()` intentionally discards both representations,
  preserving unrelated flags.
- `start()` resumes current CNT without clearing events. `stop()` preserves CNT,
  retained events and Async overflow enable; an existing event remains consumable.
- `set_counter()` retains its existing bounds and running behavior without
  silently clearing events. Valid period/frequency changes clear old software
  and hardware overflow together, reset CNT and preserve running/stopped state.
  Invalid setters change neither registers nor the software latch.

The ISR owns only per-instance static state and an owned registered Waker; it
keeps no pointer into the Timer or future. Each dispatch first checks active
ownership before any bank read, then requires that bank's overflow enable and
pending flag. It acknowledges overflow, latches an event and wakes the task.
The source remains enabled across events and waits. A wait registers its waker
before consuming retained software/hardware state.

Canceling an unpolled or pending wait changes no hardware or retained event
state. Forgetting a future likewise has no cleanup duty. A previously registered
task may receive a harmless later wake; the next wait replaces the waker.
Forgetting an Async owner skips shutdown and may leave counting, requests and
wakes active indefinitely. Static state makes this independent of the old
owner's memory, without promising power savings or forward progress.

Every successful BTIM construction, including the old Blocking constructors
and construction inside retained trigger owners, retires prior static active
state before initializing that bank. It stops and masks the owned bank, clears
its old hardware/software events, and only then permits Async activation.
Thus safe reborrow reconstruction after a forgotten Async owner cannot leave
the old ISR ownership active. Pure validation fails before this transaction
and does not retire that prior state. A failed constructor consumes its token
unless the caller used `reborrow()`; existing clock-gate failure still panics,
including through `new_async`. No new RCC recovery behavior is promised.

Drop in either mode retires active state, stops the selected bank, disables
its requests and clears its software latch in one critical section. Inherited
DMA requests in that bank are disabled during whole-bank initialization and
final cleanup, including L012 UDE/TDE. A pending dispatch after Drop returns
before MMIO. Successful reconstruction discards old event identity. The group
clock remains enabled and the shared BTIM reset is unchanged. Other BTIM banks
are untouched; existing GTIM/ATIM ownership and critical-section scope remain
separate.

## Coalescing and progress

Hardware provides a sticky bit and software retains one boolean event. Several
overflows before observation can merge, including an overflow between a positive
flag read and its acknowledgment. A true result means an event was observed;
it provides no count, precise timestamp or lossless history. An overflow after
a negative check is not blindly acknowledged. Task consumption, ISR latching,
reconfiguration and event clearing use the same critical-section boundary.
A stale NVIC pending entry simply rechecks the live source.

A stopped counter, ceased PCLK or unavailable IRQ service can leave a wait
pending indefinitely. A caller-composed deadline needs an independently useful
wake source. Very short periods can consume substantial IRQ time despite
coalesced task-visible events. This API adds no elapsed-time reconstruction,
global Embassy timebase, clock-loss recovery, deep-sleep timing or wake guarantee.

## Shared-vector dispatch and retained recipients

Use the existing `bind_interrupts!` multiple-handler dispatch when independent
owners enable sources on the same vector:

| Family / vector | Required dispatch when the named owners coexist |
| --- | --- |
| L052 `BTIM2_LPTIM` | `OverflowInterruptHandler<BTIM2>` and `lptim::InterruptHandler<LPTIM>` for Async BTIM2 and Async LPTIM |
| L083 `BTIM2_LPTIM1` | The same two handlers; the actual peripheral token is `LPTIM`, not `LPTIM1` |
| L012 `BTIM3_HALLTIM` | `OverflowInterruptHandler<BTIM3>`; the existing polling `HallTim` owner keeps its own requests disabled |

An application separately enabling HALLTIM requests must supply its own matching
handler in that dispatch; this addition provides no Hall IRQ API. Each handler
services only its own bank. BTIM never probes a partner, opens its gate, changes
its reset, masks its requests, acknowledges its flags or requires its singleton.
LPTIM and HALLTIM have separate gates/resets from the BTIM group even though
their IRQs are shared. LPTIM's stopped-only IER rule is not a BTIM restriction.

Async construction enables the associated NVIC line after state initialization.
No constructor, wait cancellation, reconfiguration or Drop disables/unpends
that line or changes its priority. Enabling a vector with an already-enabled
foreign request can expose that request immediately; omitting its dispatch can
cause repeated interrupts and starve foreground work. This is an application
dispatch/liveness responsibility, not a hidden memory-safety precondition or a
reason for the BTIM owner to alter another source.

Native CR2.MMS=0 forwards software EGR.UG onto TRGO. URS=1 limits update IRQ/DMA
causes but does not disconnect TRGO. Existing initialization and reconfiguration
still generate UG, so independently retained trigger recipients can react.
Ordinary Timer ownership does not acquire those recipients. Their functional
handover must allow source start/stop/reconfiguration effects or detach them;
the driver neither scans nor rewrites other owners' routes to imply isolation.

Existing L010/L011 `Btim1Update`, `Btim2Cascade` and `Btim1Adc` retain their static
tokens, Blocking Timer fields and destination-before-source sequencing. A live
trigger owner and Async Timer cannot simultaneously own the same BTIM singleton.
Forgetting a static trigger owner does not create a safe reuse path. This feature
adds no general triggers, ADC integration, capture, PWM, DMA or one-shot mode.

## Verification boundary

The current [source evidence](btim-overflow-irq-evidence.json) records own-family
manual/SDK identities and semantic register, IRQ and shared-bank facts. The
historical [polling evidence](btim-counter-evidence.md) and its
[JSON receipt](btim-counter-evidence.json) keep their original scope and results.
No hardware schema or canonical register change is needed for these existing
metadata routes.

The final main matrix produced 16 actual ARM libraries and 23 linked firmware
ELFs, with all 39 commands exiting zero. It comprises thirteen family libraries
plus three feature-boundary libraries; thirteen `btim_events`, three shared-owner
`btim_shared`, and seven existing compatibility links for trigger/ADC, PWM,
polling, capture, QEI and complementary PWM owners. The new programs live in
[the existing example crate](../examples/shared/low-power-timers/README.md). The matrix
ran serially using the existing shared target, retaining actual artifacts and
HAL OUT_DIR output per row. Its 1,353 source files and all 905 retained generated
files stayed unchanged. This is an unchanged-output comparison; no main
regeneration was performed.

Four separately recorded source/data commands passed: the extended BTIM
own-evidence check, source provenance, module layout and canonical validation.
Independent main runtime/ownership and metadata/source implementation reviews
accepted the frozen implementation and actual main artifacts. These finite
checks do not expand the cumulative HAL recipe. Their main-time document hashes
identify historical snapshots; later administrative status updates do not change
which executable source and artifacts were reviewed.

The first main attempt was deliberately stopped before row 26 after 25
successful commands, yielding 16 libraries and nine ELFs. It had no Cargo
failure, but eight introduced `private_interfaces` warnings per compile required
a visibility-only correction to `State` in the private BTIM module. The partial
attempt remains recorded and is not the accepted full matrix. After the reviewed
correction, all 39 main commands ran again against the revised, unchanged source.
The introduced warnings are absent from the final run. Three existing native
L010/L011 `RCC_LSE_SYSCLK_*` `dead_code` warnings remain in each applicable row;
the final matrix is not warning-free.

The first provenance invocation failed only because the derived
`build/provenance/reference-index.json` was stale. Refreshing that view with the
existing provenance tool's `--write` operation preceded the passed final four
gates. It did not replace original source identities or authorize new source
facts. The failed invocation and partial main attempt remain distinct from the
successful final receipts.

The original recorder mislabeled the memory-summary fields in the sixteen new
BTIM ELF receipts with bank names, after its range checks had already used the
correct Flash/RAM objects. A separately recorded read-only reinspection of those
same binaries corrected only the labels. Independent reviews confirmed all
sixteen corrected maps and all 23 ELF load bounds; original receipts and
binaries remain unchanged. This correction required no new Cargo invocation
or device-memory change.

At main acceptance clean had not run. Final-package completion
requires a separate final-source two-library/six-ELF clean receipt covering all
six BTIM IP versions and all three shared-vector cases. Main results and
administrative status updates cannot substitute for that replay or establish
publication. The source-only final package requires its own ordinary clean
generation before replay; those generated projections must match the unchanged
905-file baseline. No clean result is claimed here.

Compilation, linkage, emitted-association inspection and source/data comparisons
can establish their respective software boundaries. They do not execute or
measure cancellation, reconstruction, interrupt delivery, overflow ordering,
retained-recipient effects or silicon timing. No firmware execution or hardware
validation is claimed.
