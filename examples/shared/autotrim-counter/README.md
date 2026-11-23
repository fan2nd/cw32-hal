# AUTOTRIM periodic counter

A normal bare-metal polling application for CW32L052/CW32L083 exact packages.
Select a package feature and build for `thumbv6m-none-eabi`. It uses the frozen
factory HSIOSC with MD=3/AUTO=0, then performs periodic work after consuming UD.
No interrupt binding or external pin is needed. The 49,152,000-source-cycle
period is bounded by the qualified ±2% factory oscillator envelope, and is not
an exact one-second delay. Polling coalesces missed periods. The finite poll
budget is iterations, not microseconds. This example has been linked, not flashed.

Example build from this directory:
`cargo build --locked --target thumbv6m-none-eabi --no-default-features --features cw32l052c8t6`

The `async` binary uses `AutotrimTimer::new_async` and awaits coalesced
underflows. Its shared-vector binding also includes the LCD handler, which
skips MMIO when no LCD waiter is active. Canceling a wait preserves the
pending underflow and counter. See [the async contract](../../../docs/autotrim-async.md).
