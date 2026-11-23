# LCD crosspoint walk

Ordinary bare-metal firmware, not a HAL test harness. Select the actual exact
package feature. It links to that part's generated FLASH/RAM memory map.

Wire a compatible 1/4-duty, 1/3-bias passive panel as described in `src/main.rs`.
The example explicitly assumes VDD ≤ 3.6 V and panel peak-drive tolerance ≥ 3.6 V.
Adjust those declarations, drive strength and contrast to the board and glass
specifications before use. Do not wire an external resistor/capacitor bias network.
Unused wired panel segments must still be owned by the constructor; extend its
pin list to include them before using a larger panel. The four listed segment
pads are a minimal four-segment example, not a board-vendor pinout claim.

Build: `cargo build --manifest-path examples/shared/lcd/Cargo.toml --no-default-features
--features cw32l052c8t6 --target thumbv6m-none-eabi` (one shell command).
Use `cw32l083vct6` for L083 LQFP100; the same example wiring is valid there.
The polling budget counts polls, not microseconds. A timeout preserves LSI and
shared gates; the error should be handled according to the application's policy.

No hardware flashing or display verification has been performed.

The `async` binary uses Embassy and `present_async` / `wait_next_frame_async`
with the same wiring and electrical assumptions. It also owns an AUTOTRIM
async timer, binding both handlers to AUTOTRIM_LCD to demonstrate shared-vector
coexistence without shared-source acknowledgement. Add `--bin async` to the build
command to select it, or `--bin cw32-lcd-example` for the original blocking
binary. Async waits yield rather than consume the polling budget and have no
built-in timeout. Dropping a pending wait disables only LCD's interrupt source;
no atomic display update or hardware-tested timing is implied.

The `lse_blink` binary additionally requires a board-qualified crystal on PC14/PC15.
Its example frequency envelope and supply/temperature assumptions must be replaced
with the actual board data. It consumes the unique LSE lease, configures a shared
32-frame blink/refresh interval, and uses `present_on_event` / `wait_next_event`.
Frame-named methods reject intervals other than one; their async variants now
return Result. Neither interval phase nor a full visible/blank cycle is promised.
Build all binaries with `--release --bins` and the actual exact package feature.
See `docs/lcd-clock-blink.md` for the contract and limits.
