# HAL selection: chip metadata, IP versions and capabilities

The comparison is pinned to Embassy commit
[b12a6d9efcd2711037abca1b63a661a9ef726444](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs)
and its [shared ADC layer](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc/mod.rs).

## Selection boundaries

1. Cargo chip feature selects one PAC metadata set. The HAL build checks that
   its selection agrees with that metadata; it does not derive driver behavior
   from the chip name or family. No `cw32l012`, `cw32f030`, or exact-chip cfg is
   emitted. The family/name remain descriptive metadata.
2. Peripheral presence enables `adc`, `gpio`, `atim`, `gtim`, etc. Each instance
   independently enables its kind/version, such as `adc_l012` or `gtim_f030`.
   `peri_adc1`, `peri_adc2`, and other instance-presence cfgs come from actual
   instances. They gate real paired-resource APIs, not whole chip families.
3. `l012` and `f030` are retained as the names of audited IP contracts. They
   identify register versions first used in those families. They are not global
   aliases for selecting every driver. Unknown versions of implemented drivers
   are rejected rather than silently compiled with a guessed compatible backend.
4. Shared GPIO code uses independently derived `gpio_has_speed`,
   `gpio_has_drive_strength`, `gpio_has_level_interrupts` and
   `gpio_pulldown_indexed`. Metadata must contain the exact supported ordinary
   RW register/field shape; malformed capabilities fail generation. Pin-specific
   pull-down masks still decide whether an individual pin supports that pull.
5. Bus drivers independently select `uart_l012/f030`, `spi_l012/f030` and
   `i2c_l012/f030`. UART/SPI/I2C pin traits use the actual peripheral/signal/AF
   routes and physical GLOBAL vector, including SPI23 sharing. F030 I2C has no
   peripheral interrupt mask, so generation requires its independently owned
   vector before enabling the backend's NVIC masking strategy.
6. CRC32 algorithms and wide data feeds use `crc_has_crc32` and
   `crc_has_wide_data`, derived from sourced MODE values and 8/16/32-bit data
   register views. IWDT uses the same audited `iwdt_l012` IP on both chips.

ATIM and GTIM backends are independently selected, including mixed IP versions
or only one timer kind. There is no `all(atim_l012,gtim_l012)` family bundle.
Analog BGR, DAC, VCREF, OPA and VC are separate IP modules; optional reference/DAC
APIs are conditionally available without hiding unrelated analog drivers.
The L012 OPA/VC constructors still require a real BGR startup witness. That
explicit dependency is retained; no fake reference resource is manufactured.
ADC-to-ATIM convenience methods require the compatible trigger contract, while
ordinary ADC remains available without ATIM. These combinations describe checked
software capability, not a claim that a synthetic mixed chip exists.

VC reference associated types now consume explicit, sourced family connections,
not VC-name or IRQ-group inference. VC pin traits consume the audited pin routes, with
register selector-width checks. No family-wide “four or eight inputs” guess
selects their layout. ADC local reset suppression uses the explicit
[reset cross-effects](schema-v6.md), alongside shared reset-bit ownership.

## What is genuinely shared

- GPIO ownership, mode transitions and interrupt Wait use one implementation.
- ADC channel ownership and shared lifecycle/trigger-disarm/completion/guarded
  cancellation use one workflow. L012 EOS and F030 single EOC/scan EOS remain
  register-backend hooks, including the single-channel watchdog rule.
- Comparator handler publication, wait/arm/recheck/cancel and lifetime guard
  use one implementation. Register masks and startup/reference topology remain
  in each VC IP backend.
- Generic Timer/SimplePwm algorithms are shared; ATIM/GTIM adapters are selected
  independently. Verified L012 common register operations are reused without
  casting distinct PAC block types.
- Embassy time State/Driver/queue servicing and interrupt lifecycle are shared.
  L012 atomic UIFCPY and F030 OV/CNT/OV consistency algorithms remain distinct.
- ATIM event-owner/wait/cancellation control flow is shared; break flags,
  deadtime, protection and duty-update semantics remain version-specific.
- UART ownership, split halves and one-shot IRQ waits are shared. SPI transfer
  flow is shared while divisors/register setup remain IP-specific. I2C shares
  buffer traversal and operation lifecycle; L012 command/FIFO and F030 SI/status
  state transitions retain separate engines.

This is not a claim that every similar-looking register file can be merged.
After excluding descriptions/provenance, all 17 kinds with both versions still
have real layout, access, width, field or reset differences. IWDT/WWDT already
reuse the same data IP version across the two chips. PAC Rust and register metadata modules are now emitted once per kind/version;
chip roots select them within the same PAC crate. Known reset specializations
are aggregated across the selected chip set without inventing unknown values.

## Remaining boundaries

Some public configuration/owner declarations remain IP-specific where their
channels, startup, source dependencies or error contracts differ. This version
does not implement all upstream peripheral drivers, a generic register adapter
framework, dynamic clocking or unverified hardware combinations. Source-only
archives contain no test scaffolding. Temporary equivalence, negative and
mixed/absent-IP probes accompany the real two-chip release builds; see
[v0.13.1 validation](validation-v0.13.1.md). No silicon was tested.

## v0.13.2 source layout

Metadata associations, supported-IP checks and capability/cfg generation now live
as private functions in `embassy-cw32/build.rs`. The shared single-owner event
latch is crate-private in the existing `interrupt` module. The separate
`build_support.rs` and `async_support.rs` files were removed. The operations,
critical sections, cancellation ordering and generated public tables are
unchanged; see [v0.13.2 validation](validation-v0.13.2.md).

## v0.20 peripheral additions

RTC and Flash select their real IP layouts independently. WWDT reuses the
audited shared l012 IP on both chips. LSE pin traits come from sourced SYSCTRL
oscillator routes; LSI startup does not require GPIO. The early LSI off-state
audit enumerates actual requester instances and gate addresses from metadata,
while source-selector meaning is checked against each supported hardware
contract. Flash capacity comes from the chip memory region and is rejected if
it exceeds the geometry audited by the driver.

## v0.21 timer and DMA extensions

Capture/QEI adapters select each ATIM or GTIM IP independently. Concrete timer,
physical capture channel, pin AF and actual IRQ associations are generated from
metadata; F030 GTIM's external mux register/index is an explicit schema11
instance relationship. Complementary capability is generated only for audited
ATIM main/N or A/B pairs and BK routes. UART/SPI DMA capabilities pair each
actual instance and TX/RX request with channels from the controller topology.
There is no new chip-name cfg, runtime registration layer or fabricated common
STM32 register layout.
