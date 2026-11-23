# L012 analog output firmware

Four genuine bare-metal programs, for either exact L012 C8T6/C8U6 package:
- `dac_steps`: PB0 12-bit steps and PB1 8-bit steps, unbuffered outputs.
- `opa_buffer`: PA3 positive input, PB0 output, OPA1 follower.
- `opa_pga`: PA4 positive input, PB1 output, OPA2 nominal gain8. Keep multiplied voltage within the linear output range.
- `opa_standalone`: PA6 positive input, PA7 negative input, PB0 output. Provide a stable external feedback network before running.

These assume a declared 3.3V board with VDDA=VDD, sufficient loads, and safe external input voltages. They are compile/link validated only. Nothing was flashed. OPA's example pause is a bring-up margin, not a guaranteed maximum settling interval: qualify BGR startup, OPA startup and output settling on the actual board. The drivers do not calibrate.

`cargo build --locked --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6 --bins`

Use `cw32l012c8u6` for QFN48. See `../../../docs/dac-opa.md` for electrical limits, ownership and exact deferred functionality.

## L012 interrupt edge examples

- `lvd_edges`: observe either physical supply-threshold crossing; no reset or
  protection action is configured.
- `comparator_edges`: external pairs VC1 PA0/PA2, VC2 PA1/PA3, VC3 PA6/PA7,
  VC4 PA9/PA10, with both handlers bound on each shared VC13/VC24 vector.
- `comparator_divider_edges`: positive inputs PA0/PA1/PA6/PA9, borrowing VC12REF
  for VC1/2 and VC34REF for VC3/4. Default reference is half the declared supply.

The comparator examples declare 3300 mV; adjust both comparator and reference
configuration to the actual board. Respect legal analog input levels and qualify
settling separately. Early events can be startup transients. These builds do not
establish hardware correctness or low-power wakeup. See
[edge semantics and source evidence](../../../docs/l012-analog-edge-waits.md).

Build for either `cw32l012c8t6` or `cw32l012c8u6`, for example:
`cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6 --bin comparator_edges`.

DAC additions: `dac_split` demonstrates borrowed fixed-channel set/read handles;
`dac_software_trigger` stages and explicitly loads each channel. A completed
software trigger proves only digital loading, not analog settling or simultaneous
outputs. See [the ownership and loading contract](../../../docs/dac-channel-software-trigger.md).

`dac_owned` consumes the direct DAC into independently owned outputs, drops
channel 1, and continues writing channel 2. This new ownership path is
uncompiled and untested; see [owned-channel contract](../../../docs/dac-owned-channels.md).

`lvd_filtered` selects SYSCLK/32 with eight equal samples before reporting an
LVD edge. SYSCLK must remain running; analog startup, Vcore and voltage limits
still require board qualification. The new filtered path is uncompiled and
untested; see [filter contract](../../../docs/native-lvd-filter.md).
