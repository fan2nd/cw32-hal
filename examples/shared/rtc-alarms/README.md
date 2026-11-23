# RTC alarm firmware

`poll_alarm` attaches to a previously provisioned calendar, configures Alarm A
for second 10 of every minute on every weekday, acknowledges its old flag and
enables it. It polls the sticky flag and toggles PA4 after acknowledgment.
`interrupt_alarm` performs the same work with Embassy's executor and the actual
RTC vector on CW32L010/L011/L012. Both leave RTC source/calibration unchanged.

Provision the RTC first with `examples/shared/rtc-calendar`, and choose the correct
exact package. PA4 is only a demonstration output; check your board connection.
Retained RTC interrupt enables must already be inactive. The examples fail on
unexpected retained interrupt ownership instead of resetting the RTC.

From the repository root:

```sh
cargo build --locked --manifest-path examples/shared/rtc-alarms/Cargo.toml --release --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7 --bin poll_alarm
cargo build --locked --manifest-path examples/shared/rtc-alarms/Cargo.toml --release --target thumbv6m-none-eabi --no-default-features --features cw32l011k8t6 --bin interrupt_alarm
```

No firmware has been flashed by this work. Runtime timing, cancellation and
rollover need board QA. Async waiting preserves the hardware flag on completion
and cancellation; acknowledge explicitly. Repeated events coalesce. The executor
wait is run-mode notification using the existing HSI source, not a deep-sleep or
precision wall-clock guarantee. Alarm B mask programming remains excluded
because its own manuals and SDK contradict each other. See
[the source-qualified boundaries](../../../docs/rtc-alarms.md).
