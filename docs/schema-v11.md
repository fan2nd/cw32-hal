# Normalized schema 11

Schema 11 adds `Peripheral.timer_capture_mux`, an optional, evidenced
instance-level relationship for a capture selector outside the timer's own IP.
It preserves the single Rust generator and the enforced YAML → persisted JSON
→ reread/validate JSON → PAC boundary. Older schema versions and unknown keys
are rejected. The field/array/enum/clock contracts from [schema 10](schema-v10.md)
are unchanged.

The record contains `peripheral`, `register`, `index`, `field`,
`external_value`, and `source`. The register must be an indexed ordinary RW
register in the referenced controller; `index` chooses its physical array
element. The indexed field follows the timer's physical channel order.
`external_value` is the evidenced selector value for the external pin.

The validator rejects missing targets, a non-timer owner, self references,
empty evidence, scalar or non-RW selectors, read/write side effects, out-of-range
indices/values, and two timers claiming the same selector element. Generated
PAC metadata retains all six fields. The HAL independently checks the exact
supported controller IP and channel layout before generating register access.

CW32F030 GTIM1–4 explicitly reference SYSCTRL.GTIMCAP indices 0–3, field CH,
external value 0. RM2.5 §§4.7.20–23 and §14.3.3.2 Table14-5 establish these
connections. The build script does not derive a selector index by parsing the
instance name. L012 TISEL is part of each timer's own register block and is
handled in that version's capture backend. Output PWM routing remains
independent of capture-source selection.
