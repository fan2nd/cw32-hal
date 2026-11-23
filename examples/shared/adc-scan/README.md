# L010/L011 software ADC reads and ordered scans

`scan_owned` owns PA0/PA1, samples different acquisition times and repeats PA0.
`scan_borrowed` uses all eight slots, borrows PA0/PA1, repeats the temperature and
bandgap sources, then performs a single conversion after the scan borrows end.
Both retain complete result order and timing via `black_box` for a debugger.

With the `async` feature, `async_single` binds the dedicated ADC interrupt and
performs EOS-driven single reads wrapped in `embassy_time::with_timeout`.
`async_scan_borrowed` uses the same eight mixed external/temperature/BGR slots as
the blocking example, races a scan against a short caller timer with
`embassy_futures::select`, then reuses the owner, channel handles and output for a
fresh scan. Either side of the race may win; it is an ordinary application example,
not a forced-interrupt or cancellation test. It also performs a single read after
the borrowed pin handles leave scope. A canceled scan leaves the caller's output
unchanged; a terminal cleanup failure is reported by subsequent HAL operations.

The examples declare actual VDD 3.0–3.6 V, ambient -40..85 °C, and HSI /8.
L011 requires VDDA=VDD. Adjust these declarations to the board and keep PA0/PA1
within the ADC reference/supply rails. The chosen acquisition times do not waive
source-impedance requirements. Internal results are raw counts, not calibrated
temperature or voltage. Shared BGR and its clock remain enabled after use.

Supported exact feature selections: cw32l010f8p6, cw32l010f8u6, cw32l010y8m6,
cw32l011k8t6 and cw32l011k8u6. The build script derives memory.x from selected
package metadata. For example:

    cargo build --locked --release --bins --target thumbv6m-none-eabi \
      --no-default-features --features cw32l011k8t6

To include both async bins, select the `async` feature as well:

    cargo build --locked --release --bins --target thumbv6m-none-eabi \
      --no-default-features --features cw32l011k8t6,async

Executor, time and futures use the repository's exact Embassy git revision
`f16efeffe37581092ec184718e6fdb1620393214`. The async example feature enables the
existing `time-driver-gtim1`, which reserves the whole GTIM1 and its IRQ. This is
an application choice for deadlines; the ADC async API itself needs no time driver.
The example's nominal clock and timers must remain running, interrupts enabled,
and the executor scheduled. Read [the time-driver contract](../../../docs/time-driver.md),
including its strict interrupt-blackout bound and unsupported debugger-halt,
clock-gating and deep-sleep cases. A caller deadline is observed only when polled;
synchronous setup/stop/finalization can delay observing it. `Config.timeout` is a
finite synchronous register-poll budget, not an async deadline.

No firmware was flashed or run during implementation. See
[the scan contract](../../../docs/adc-scan-sequences.md) for electrical, timeout,
ordering and unsupported-mode details, and [the async contract](../../../docs/adc-low-async.md)
for IRQ ownership, cancellation/reuse and terminal faults. The async contract
supersedes older statements that all L010/L011 IRQ scans are unimplemented.
