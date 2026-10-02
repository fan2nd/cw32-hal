# 06: Embassy application composition

This independent package contains its own pure motor/protection source modules,
with the same algorithms as stage 05, an application telemetry scheduler, bounded mailbox and UART queue. It
has no library target or dependency on another example package. It has its
own board adapter and one real `embassy_executor::main` entrypoint. All modules
are loaded through ordinary local `mod` declarations, without cross-crate source imports.
Stage 05 exports its IRQ vectors only from its own binary, so this package
has a single motor interrupt domain.

`Board::new()` returns `(Board, Ui)`. The motor interrupt domain owns only motor
peripherals and pins. The Embassy task owns PC13, PA3, UART1, PB11 and PB12;
there is no retained `Option<Ui>` or UART forwarding in `Board`. It emits at
most one UART byte per millisecond wake. The command watchdog fails closed
at 100 ms and a single stale pressed-key sample cannot become a held key.

The application sends the original seven-byte frame on a debounced key event,
every 500 ms while powered, and on automatic power-off. The original unused
PI/PID algorithm is omitted.

From the workspace root:

```sh
cargo build --release --target thumbv6m-none-eabi \
  -p cw32-bldc-06-application
```

This package builds directly for the MCU. Default firmware keeps the motor
gates disarmed. `motor-output-enable` is an explicit opt-in permission and
still requires deliberate key commands. It performs the original six-tick
bootstrap sequence. This example is not an electrically qualified drive:
verify the schematic, gate polarity, supply, dead time, scope traces, worst-case
IRQ latency and current/voltage calibration before energizing a motor.
The supplied source does not establish a verified external hardware break.

The board retains 96 MHz HCLK (VDD at least 1.8 V), 48 MHz PCLK, 20 kHz ATIM,
8 MHz BTIM2/3 and 6 MHz ADC. ADC1's coherent four-slot sequence is 22 µs;
ADC2's five-slot sequence is about 444.167 µs every 5 ms. Those calculated
budgets do not prove sampling aperture quality or interrupt response on real
hardware. Panic, HardFault and detected stale/overlapping samples disconnect
outputs without automatically rearming.
