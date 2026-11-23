# Run-mode AWT, LPTIM and BTIM firmware examples

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

`polling` starts an owned periodic timer and polls its elapsed event. `interrupt` installs a typed interrupt binding and awaits events under the pinned Embassy thread executor. Neither changes GPIOs or enters deep sleep. These are real Cortex-M firmware binaries; compilation/linking is not hardware validation.

Build either binary with an exact package feature, for example:

```sh
cargo build --locked --manifest-path examples/shared/low-power-timers/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features --features cw32l010f8p6 --bin interrupt
```

The existing `polling` and `interrupt` binaries support the twelve exact-package features listed in Cargo.toml. The build script derives FLASH/RAM sizes from selected-chip metadata. These two binaries use HSIOSC for AWT on A030/F002/F003/F020/F030/L031/R031 and PCLK for LPTIM on L010/L011/L012/L052/L083. Their default reload/divider values intentionally produce different periods on different clock trees.

L052/L083 bind a vector shared with BTIM2. The existing `polling` and `interrupt` binaries never enable BTIM2; an application that does must bind and service its handler too. The new `btim_shared` binary below includes both owners and handlers. No constructor/handler/drop unpends or disables a shared NVIC line. Events coalesce. To recover from an LPTIM synchronization failure, stop using the faulted object and reconstruct through a reborrowed singleton, causing an independently owned LPTIM reset.

No pins need wiring for `polling` or `interrupt`. Observe the `nop` after an elapsed event with a debugger if desired, noting that default debug freeze settings affect timer progress. Do not interpret a successful link as proof of timer, clock-domain, or deep-sleep behavior on a board.

## BTIM overflow events

`btim_events` owns all three actual BTIM banks as `Timer<Async>` and binds every bank's overflow handler. It supports all twelve existing exact-package features without an additional marker. The build script reads the selected chip's own `GLOBAL` links and NVIC entries, checks IRQ20/21/22, and admits the actual dedicated or shared vector names. L052 uses `BTIM2_LPTIM`, L083 uses `BTIM2_LPTIM1`, and L012 uses `BTIM3_HALLTIM`.

```sh
cargo build --offline --locked --release --manifest-path examples/shared/low-power-timers/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features --features cw32f002f3p7 --bin btim_events
```

The example configures 62,500 ticks with divisors 256, 512 and 1024. Their nominal event rates are PCLK divided by 16,000,000, 32,000,000 and 64,000,000 respectively, so different clock trees give different periods. Each loop awaits BTIM1 and then consumes any retained BTIM2/BTIM3 events with `take_overflow()`. The `nop` sites are placeholders for periodic housekeeping; no GPIO or board wiring is required. The same metadata-derived `memory.x` is used for every binary, including the strict F002 16 KiB FLASH / 2 KiB RAM and F003 20 KiB FLASH / 3 KiB RAM maps.

Each event represents at least one overflow. Multiple overflows can coalesce in hardware or software; neither an await nor a successful take counts all periods or measures elapsed wall time. Cancelling a wait leaves counting, overflow requests and retained events intact. `start()` resumes the current counter without clearing an event; `stop()` retains events. Valid period changes intentionally clear old events and reset the counter. A stopped counting clock or unavailable IRQ service can leave a wait pending indefinitely. These run-mode examples provide no independent deadline, global Embassy clock, deep-sleep wake, or source-loss recovery.

### Shared vectors with real partner owners

`btim_shared` requires the nondefault `btim-shared` marker and one of `cw32l012c8t6`, `cw32l052c8t6` or `cw32l083mct6`. The build script rejects the marker for other selections. For example:

```sh
cargo build --offline --locked --release --manifest-path examples/shared/low-power-timers/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l052c8t6,btim-shared --bin btim_shared
```

- L052/L083 retain both BTIM2 Async and the existing PCLK-backed LPTIM Async owner. Their shared binding dispatches both real handlers on each vector entry. The loop awaits each owner's event in turn, so events may coalesce while the other wait is pending. No pins need wiring. LPTIM construction/start synchronization failures take the example's panic path.
- L012 retains BTIM3 Async and the existing polling `HallTim` owner. The Hall constructor masks its own interrupt/DMA requests before BTIM3 enables the shared vector; only the BTIM3 handler is bound. Connect push-pull Hall CH1/CH2/CH3 outputs to PA3/PA4/PA5 (AF9) and common ground, within the board's GPIO supply limits. Open-collector sensors need suitable board-specific pull-ups. Each BTIM event polls the latest Hall capture. WIDTH can be overwritten by intervening edges, input levels are live, and overflow remains sticky until an explicit Hall restart. This is observation, not lossless capture or motor commutation control. No motor outputs or power stage are driven. See [the Hall example](../../l012/halltim/README.md).

The BTIM-only binary does not enable a shared-vector partner. Applications that separately enable a partner's requests must add that partner's handler to the same dispatch; an unserviced foreign request can repeatedly enter the IRQ and starve foreground work. The BTIM handler never inspects or changes a partner bank. Dropping a BTIM owner stops and masks only its selected bank, preserves the shared BTIM group clock/reset, and never disables or unpends the NVIC line. Forgetting an owner skips shutdown and can leave counting and requests active.

Timer ownership does not acquire independently configured trigger or ADC recipients. Arrange the existing functional handover before takeover or reconfiguration when a retained recipient matters. On native BTIM, initialization/reconfiguration issues UG; `MMS=0` forwards UG onto TRGO, and `URS=1` restricts update IRQ/DMA causes without disconnecting that trigger output. The examples add no downstream routes or conversion of retained trigger owners. Compiling and linking these binaries checks software integration only; no IRQ delivery, event race, shared-source interaction, or silicon timing has been exercised on hardware.

## Owned LSE AWT events

`lse_events` is an additional run-mode firmware binary enabled only by the nondefault `lse-awt` marker. It configures a crystal source through RCC, acquires an owned `LseClock` using reborrowed SYSCTRL/PC14/PC15 tokens, and passes it to `Awt::new_lse`. Its first phase starts AWT and polls for at most 100,000 iterations, observing the retained event and source readiness separately. It records whether an event was seen and drops the Blocking owner regardless. This is an iteration bound, not a time deadline, proof that an event occurred, or a startup-duration guarantee.

The binary then reacquires `LseClock` through the original tokens and constructs `Awt::new_async_lse` with the typed AWT interrupt binding. It continues awaiting retained events under the pinned Embassy executor. It creates no independent RTC owner. A successful link checks types, target and linking; it does not execute polling, reconstruction, interrupts, source loss, or a silicon timing sequence.

Build one of the six own-family selections already present in this crate:

```sh
cargo build --offline --locked --release --manifest-path examples/shared/low-power-timers/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32f030c8t7,lse-awt --bin lse_events
```

| Existing feature | Own-family LSE qualification | PC14 / PC15 package pins |
| --- | --- | --- |
| `cw32f020c6u7` | F020 datasheet Rev1.3; 1650–5500 mV, −40–105 °C | QFN48: 3 / 4 |
| `cw32f030c8t7` | F030 datasheet Rev1.9; 1650–5500 mV, −40–105 °C | LQFP48: 3 / 4 |
| `cw32a030c8t7` | A030 datasheet Rev1.1; 1650–5500 mV, −40–105 °C | LQFP48: 3 / 4 |
| `cw32l031c8t6` | L031 datasheet Rev1.9; 1650–5500 mV, −40–85 °C | LQFP48: 3 / 4 |
| `cw32r031c8u6` | R031 datasheet Rev1.2; 2200–3600 mV, −40–85 °C | QFN48: 2 / 3 |

The five board declarations in `lse_events.rs` are illustrative, not measurements or board qualification: 3000–3600 mV, −20–70 °C, a 32766–32770 Hz per-cycle envelope, Strong drive, Normal amplitude, Cycles16384, and a 20,000,000-attempt startup budget. Replace them with actual qualified crystal, loading, drive, amplitude, supply, temperature, aging and per-cycle bounds before running on a board. Nominal 32768 Hz or an average ppm specification alone is insufficient. Connect the appropriately qualified crystal network to PC14/OSC32_IN and PC15/OSC32_OUT; there is no GPIO output or pin-toggle claim. See the [LSE board contract](../../../docs/qualified-lse.md) and [owned LSE AWT contract](../../../docs/lse-awt.md) for source authority and limitations.

The metadata admission is exactly F020C6U7, F030C8T7, A030C8T7, L031C8T6, L031C8U6, L031F8U6, R031C8U6. This crate exposes five existing representative features; the other two L031 packages have library-level admission. The build rejects `lse-awt` with any other selection. F002/F003 remain HSIOSC examples, and the default L010 all-bin path skips `lse_events` through its required feature.

### Events, source health and retained recipients

- `take_elapsed()` and `wait()` consume retained, coalesced AWT events, including events preceding an LSE fault. Neither reports current source health, an event count, or uninterrupted elapsed wall time. `is_clock_ready()` is a separate snapshot, not a measured frequency or guarantee of another edge. It does not acknowledge events or faults or inspect/open GPIO banks.
- LSE loss produces no driver wake. With no retained event, the async wait can remain pending forever. Checks before or after the wait cannot enforce a deadline; that needs an independently waking timeout path that remains valid under the application's failure assumptions. This example supplies no such path and makes no deep-sleep or recovery guarantee.
- Acquiring or reacquiring `LseClock` can temporarily open the whole GPIOC gate to inspect PC14/PC15. Retained bank sampling, filters and armed edges can advance during that window, including before an acquisition error. Restoring the gate does not undo that progress. The example's Drop/reacquire phase repeats this existing effect; AWT adds no scan of other recipient banks.
- On L031/R031, inherited GPIOA/B/C/F filters can use AWT underflow through shared `FILTER.FLTCLK=7`. Taking over AWT, changing its source/divider/reload, starting/stopping it, dropping it or losing LSE can change or stall these filters and delay or suppress edge/EXTI delivery. Owning AWT, SYSCTRL and oscillator pads does not own those recipients. Applications that need preserved filter timing must arrange a compatible functional handover. F020/F030/A030 have no such qualified FILTER7 route.
- Normal Drop stops and masks AWT while retaining LSE, monitoring LSI and oscillator-pad reservations. Reacquisition does not clear faults or recover the source and can fail. A successful replacement constructor clears old AWT retained events. Forgetting a running owner skips Drop and can leave counting/IRQs and recipient effects active; token lifetimes are not a shutdown guarantee.
