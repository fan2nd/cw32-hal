# Complementary ATIM external-pin firmware

These are ordinary Cortex-M0+ applications, built and linked but never flashed or
executed. Use a logic analyzer or oscilloscope on an isolated development board;
this example does not qualify a power stage. It requests 1 kHz, 25% main duty and
at least 500 ns symmetric dead time within the configured RCC envelope.

| Feature | Main CH1 | CH1N | External BK1 |
| --- | --- | --- | --- |
| cw32l010f8p6 | PB4, AF7, package 1 | PA4, AF7, package 14 | unavailable |
| cw32l011k8t6 | PA5, AF7, package 11 | PA7, AF7, package 13 | PA0, AF7, package 6 |
| cw32l012c8t6 | PA5, AF7, package 15 | PA7, AF7, package 17 | PA0, AF7, package 10 |

L011/L012 BK1 is active high without an internal pull. Fit an external pull-down
and use an appropriately rated external digital source, common ground and
board-specific electrical limits. These selected BK pads have no qualified
internal pull-down (L011 lacks that option; L012 restricts it to PF3).
An external break clears MOE in hardware; the application never re-enables it.
L010 has software break capability through the HAL but no qualified external BK
route because its BK pads require oscillator/debug ownership outside this subset.

Build with `cargo build --locked --offline --target thumbv6m-none-eabi
--no-default-features --features <feature>` from this directory. No runner or
flash command is provided. After any board-level experiment, fault recovery and
shutdown behavior require independent validation.
