# Qualified LSE calendar examples

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

These two firmware examples target CW32F030C8T7/CW32A030C8T7 LQFP48
CW32F020C6U7 QFN48, the four exact L031/R031 parts, three exact L052 parts
and five exact L083 parts listed below, for fifteen qualified selections in total.
Other F020 packages lack the oscillator pair.
Crystal uses PC14/PC15; bypass consumes PC14 and leaves PC15 available.
RCC init requests the source before peripheral tokens are exposed. LseClock
subsequently acquires SYSCTRL and the pins without reconfiguring the oscillator.

The example declares nominal 32768 Hz with **every-cycle** bounds32766..32770Hz
at3.0..3.6V and−20..70°C. Replace these declarations and drive/amplitude settings
with board-qualified data; average oscillator ppm alone is insufficient.
Qualify crystal load, drive, startup and the own datasheet bypass input levels,
duty, pulse widths and edges. Nothing here measures or validates a real board.

The explicit20000000poll budget counts attempts, not milliseconds. Crystal
startup is slow; the published1.5s is typical, not an upper bound. A timeout
retains EN and the permanent pad reservation. LSE controls survive ordinary reset; a POR may be needed before retrying.

Cold provisioning deliberately writes2026-10-09 12:00:00. Replace it with the
intended time, or use Rtc::attach_preserving_state for a retained calendar.
PA4 toggles on observed second transitions. LSE faults make operations fail;
there is no automatic RTC LSI replacement or post-fault elapsed-time guarantee.

Build with cargo build --locked --release --bins --target thumbv6m-none-eabi
--manifest-path examples/shared/lse-clock/Cargo.toml --no-default-features
--features cw32f030c8t7 (or cw32a030c8t7 or cw32f020c6u7). No firmware execution is claimed.


The same normal binaries also select cw32l031c8t6, cw32l031c8u6,
cw32l031f8u6, cw32r031c8u6. L031F8U6 has both LSE
pads but neither direct-output route. These families use a monitored LSE
policy and qualify factory LSI trim only for a repeatedly verified stopped,
unused source; inherited calibrated LSI is preserved. The declarations are
board assumptions, not measurements. See ../../../docs/qualified-lse.md.

The same binaries select cw32l052c8t6, cw32l052r8s6 and cw32l052r8t6.
L052 requires independent startup_drive and startup_amplitude in addition to
running drive/amplitude. All four must be qualified for the actual board;
the example settings are declarations only. Both banks are programmed before
enable, with no assumed precise bank-switch timing. Its separate AUTOTRIM,
LPTIM and LCD consumers and the own ALARMA reset value are admitted through
the native source facts. No power-mode behavior or live calibration is offered.

The same crystal/bypass calendar examples also select the five exact L083 parts:
`cw32l083rbt6`, `cw32l083rct6`, `cw32l083rcs6`, `cw32l083mct6`,
`cw32l083vct6`. L083 has only one analog bank and therefore no startup-drive
fields. The example's frequency and board declarations require replacement
with qualified board data; linked firmware has not been executed on silicon.
