# Embassy Timer firmware

This uses the real pinned Embassy executor, integrated timer queue and
`embassy_time::Timer`, with concurrent 1 ms, 33 ms and 100 ms waits. It has no
board-specific pin wiring. Select one exact package feature; default is F030C8T7.

```sh
cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6
cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6,generic-queue
```

The second command lets the application select the pinned generic queue with
capacity eight. Neither command executes firmware. Defaults use a nominal 1 MHz
GTIM tick clock. The selected whole timer and IRQ are reserved, and default RCC
clocks must remain running. Read [the driver contract](../../../docs/time-driver.md)
for the strict interrupt-blackout bound, HSI error, Flash/debug/deep-sleep limits,
and the difference between monotonicity and elapsed-time accuracy.
