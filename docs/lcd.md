# CW32L052 / CW32L083 LCD

This segment-LCD driver provides blocking and asynchronous frame waits,
generated physical pin traits and source-qualified PAC selectors. It implements internal resistor bias,
LSI or held LSE operation, software pixel buffering, direct display writes, checked
frame waits, refresh-interval waits, hardware blinking and contrast changes.
See `lcd-clock-blink.md` for the held-source and event-interval extension contract. It has not been flashed or verified on hardware.

## Public API and wiring

`lcd::Lcd::new` consumes `Peri<LCD>`, a fixed array of `LcdPin` values, a shared
`AUTOTRIM_LCD` type-level interrupt binding and an explicit configuration.
`LcdPin::com::<N>` and `LcdPin::segment::<N>` require generated, sealed traits.
An exact part exposes only physically bonded routes; family aliases retain the
existing common-package-intersection policy. Every pad physically wired to the
panel must be supplied, even when its pixels are intended to remain blank.

All required COM0 through COM(duty-1) must be supplied exactly once. The driver
rejects absent COMs, duplicate physical or logical pins and SEG pads reserved by
1/6 or 1/8 duty. For 1/6 duty, SEG30/31 become COM5/4. For 1/8 duty,
SEG28–31 become COM7–4. Static duty requires BIAS=0 (the `Third` selector).
The constructor checks configuration before changing pins or LCD registers.
GPIO is disconnected into analog mode before the owned LCD pad is enabled.
SEG24–27 set MUX while internal bias keeps their SEG function. External VLCD
roles are retained as metadata but do not gain a usable HAL constructor.

`set_pixel(common, segment, on)` edits an owned crosspoint in the software buffer;
`fill` touches every owned pixel. `write_display` writes the buffer directly.
`present` first waits for a fresh frame event, then writes. `pixel` reads the
software buffer. `set_contrast` accepts the documented 0–15 domain, with 0 the
strongest and 15 the weakest contrast.

## Clocks, events and update semantics

The LSI enable path uses typed SYSCTRL fields, preserves LSI trim/startup settings
and never disables LSI, including on failure or drop. The LCD and GPIO gates use
the existing central `RCC_INFO` policy. The driver does not reset LCD, alter a
foreign asserted reset, touch ADC/BGR/reference configuration or disable shared
clock gates. A failed LSI wait leaves the shared oscillator requested.
The sequence explicitly writes keyed CR1.LSIEN=1 and polls both its readback
and SYSCTRL_LSI.STABLE; it does not assume LCD automatically enables LSI.
The SDK normally reloads factory TRIM and WAITCYCLE, but this shared-use path
retains them because both manuals forbid changing oscillator parameters after
startup. RTC, AUTOTRIM automatic-wakeup and clock-security consumers remain
untouched. IWDT uses the distinct RC10K oscillator on these two families.

Scan selections are named after the manual's nominal 128/256/512/64 Hz labels.
The LSI datasheet gives 32.8 kHz as typical and separate factory-calibration
accuracy conditions. This driver retains existing trim and does not establish
those factory conditions, so it does not publish a guaranteed frequency interval.
Actual scan/frame rates depend on LSI; there is no invented clock bound or
wall-clock timeout. The caller supplies a nonzero register-poll budget.

Both manuals expose a frame INTF event, not STM32's UDR/UDD or voltage-ready
handshake. The rendered tables do not establish independent INTF latching with
IE=0, so frame waits enable LCD IE for their duration. The bound handler checks
only an actively awaited LCD source, clears its W0C command via the generated
no-op seed, latches the observation and disables LCD IE. A foreground poll can
also observe INTF. The shared NVIC vector is enabled, never cleared or disabled;
other enabled AUTOTRIM sources require their own bound handler.

RAM is live and not atomically latched. `present` is a best-effort frame-boundary
write; interrupt latency and multiword writes may produce a mixed frame. A
successful frame wait establishes an event, not analogue settling or completion
of scanning all newly written pixels. On timeout IE is disabled and the driver
remains usable. No pump/voltage-ready maximum is asserted.

## Electrical scope

The user explicitly supplies maximum board VDD including tolerance/transients,
and the glass manufacturer's allowed peak drive. The maximum LCD waveform is
bounded by VDD in both datasheets. The driver requires the declared panel limit
to tolerate the entire declared VDD because neither source provides a qualified
contrast-code-to-voltage function. These are declarations, not measurements.
Contrast and resistor strength still require board/glass selection and hardware
characterization. The constructor does not choose either automatically.

Only internal resistor strengths 4/2/1/7 are accepted; BUMP is always disabled.
External resistor division, external capacitive division/charge pump, LSE setup,
DMA and low-power resume are not implemented. RCC performs LSE setup; LCD can
retain an existing LSE lease. Hardware blinking is supported as described in
`lcd-clock-blink.md`.
The manuals require INRS=0 and board networks for external division, and require
BUMP=1 only for external capacitive division (recommended capacitors 100 nF).
Those modes need an explicit future circuitry/pin-ownership API; unsupported
internal configurations cannot accidentally turn on a pump.

## L052 interface correction

CW32L052 UM V1.5 Table 26-9, printed p550/PDF p551, and §§26.7.6–15 list only
RAM0–8 (+0x40–0x60) and RAM13 (+0x74). The vendor SVD/SDK mistakenly expose
RAM9–12 (+0x64–0x70). The L052 authored PAC removes those four unsupported
registers and their fieldsets, using an evidence-bearing register-removal
manifest and independent SVD parity audit. This is a deliberate correction to
the previously generated Stage14 interface. It does not claim that undocumented
addresses have known silicon behavior.

The remaining ten L052 RAM words each retain their documented 32 crosspoint
bits; there is no added blanket bit mask. The missing addresses correspond to
undocumented SEG36–51. Package omissions and duty-dependent COM repurposing are
separate HAL ownership masks. L083 retains its own version with all RAM0–13 and
SEG0–55. Typed access generation follows each version's real pin-field layout:
L052 has per-pin fields, while L083 groups SEGX, SEG_VBIAS, SEG_COM and COMY.
No raw register facade or copied family driver is added.

## Evidence and validation

`lcd-evidence.json` pins both official manuals, datasheets, LCD SDK header/source
and example members, including URLs, SHA-256 and section references. The
canonical acquisition lock includes new members; originals remain outside the
source deliverable. `cw32-data/lcd.yaml` records exact analog-column cells and
package positions. `tests/verify_lcd_evidence.py` re-extracts those cells from
pinned PDFs and checks all ten family/exact-chip projections, RAM domains,
source locks and normalized register-reuse fingerprints. Schema fixtures and
independent generated-vs-SVD parity cover the new optional facts and removals.

The ordinary `examples/shared/lcd` firmware walks COM/SEG crosspoints on eight explicitly
wired pads. It requires compatible glass and board voltage assumptions, links
against generated exact-package memory maps, and is not a HAL test harness.
Detailed compiler/link outcomes are in `lcd-receipts`; the main integration task
owns the later full feature matrix and final combined provenance review.

## Asynchronous frame waits

`wait_next_frame_async().await` waits for a fresh LCD frame event without
busy-polling. `present_async().await` waits in the same way, then writes the
software pixel buffer. Both require an exclusive mutable borrow of the display,
so safe code cannot run overlapping waits or drop the owner during a live wait.
An unpolled future changes nothing. Both async frame methods now return Result,
and reject a blink/event interval other than one with EventIntervalNotOne.
Use event-named waits/presentation for longer intervals.

The same existing LCD IE/INTF protocol is used for blocking and async waits.
The handler checks the active waiter before reading LCD state, disables only
LCD IE on completion, and wakes the task outside the critical section. It never
disables/unpends AUTOTRIM_LCD or acknowledges AUTOTRIM. A coexisting enabled
AUTOTRIM source still needs its own bound handler. Registering the waker and
checking readiness are serialized with interrupt handling.

Cancellation disables LCD IE, clears the frame observation and drops the stored
waker, without changing display RAM, LSI or the shared IRQ. Forgetting a polled
future can leave a waiter armed; a later blocking/async wait, successful
reconstruction or owner Drop retires it. No future or caller-memory pointer is
stored by the interrupt handler.

Async waits have no built-in timeout: the configuration's poll budget still
applies only to synchronous startup/frame polling. Source-clock loss can leave
an async wait pending indefinitely; an application can select against its own
timeout and drop the wait. Completion still proves only observation of a frame,
not atomic pixel presentation or analogue settling. No hardware validation is
claimed. `examples/shared/lcd/src/bin/async.rs` demonstrates these APIs on the same
qualified package routes and explicit board/glass assumptions as the blocking
example.
