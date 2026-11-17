# CW32L012 JD4631 FOC example

This example ports the supplied C project to the current Embassy-style HAL. It
uses the L012 buffered ATIM complementary PWM owner, blocking ADC1/ADC2 reads,
blocking SPI3 for the SSI encoder, and the CORDIC for sine/cosine. The control
path follows the C split: ATIM CH4 starts an ADC1 PB0/PB1 scan, the ADC1 EOS
interrupt runs the 16 kHz current PI/Clarke/Park/SVPWM path, and the foreground
loop performs the SSI read plus the 1 ms speed loop. The hardware loop is split
into board conversion helpers, an arming/state module, and the allocation-free
FOC kernel.

ATIM is programmed with the C project's center-aligned mode and CH4 compare
trigger near the end of the up-count; ADC1 scans PB0/PB1 with 30-cycle sample
windows and raises EOS for each current-loop update.
The six PWM routes match the supplied board:

| Function | Pin |
| --- | --- |
| ATIM CH1/CH1N | PB5 / PA15 |
| ATIM CH2/CH2N | PB6 / PB3 |
| ATIM CH3/CH3N | PB7 / PB4 |
| ADC1 current A/B | PB0 / PB1 |
| ADC2 bus voltage / speed pot | PA8 / PA10 |
| MT6701 SSI SCK/MISO/CS | PB8 / PC14 / PF0 |
| Status LED (active-low) | PC13 |

The motor constants are taken from `JD-4621.jpg`: 22 poles (11 pole pairs),
6.7 Ω phase resistance, 1.75 mH phase inductance, 12 V nominal bus and 1.1 A
rated current. The C board uses 5 V ADC reference, a 1:10 bus divider, and
OPA1/OPA2 external current amplifiers from PA6/PA7 and PA4/PA5 into PB0/PB1.
The image names a 5048A encoder; the supplied C project uses a 14-bit MT6701
SSI frame, so the example keeps the encoder electrical protocol in the board
layer and uses the 14-bit frame currently present in the project.

The bridge is held off for ADC offset calibration and at power-on. The firmware
will not align or enable the bridge until the speed potentiometer on ADC2 PA10
has stayed at zero for 100 ms. After that arming step, the potentiometer maps to
0..240 rpm with a quadratic potentiometer curve and a 1 kHz speed PI generates the torque-current command; returning
the potentiometer to zero commands a stop. Immediately after alignment, the
firmware applies a short, low-current positive-Iq probe and uses the encoder
angle delta to compensate motor/encoder direction automatically. The ADC
current scale and amplifier gain must be calibrated for the actual shunt/OPA
circuit before applying power.

The C project's LED state indications are retained: PC13 blinks while waiting
for the zero-pot arming condition, stays on during alignment, and toggles as a
run heartbeat. Startup, alignment, direction probing, speed telemetry, and bus
faults are emitted through `defmt-rtt`; `probe-rs run` displays them over the
connected SWD debugger.

Build without flashing or executing hardware:

```sh
rustc --edition 2024 --test examples/l012-foc/src/foc.rs -o /tmp/cw32-foc-math-tests \
  && /tmp/cw32-foc-math-tests
cargo build --manifest-path examples/l012-foc/speed-loop/Cargo.toml --release \
  --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6 --bin foc --offline
```

工程内的 `.cargo/config.toml` 已配置 `probe-rs` runner。连接调试器并确认
电源、电流采样和电位器安全后，可在本目录执行：

```sh
cargo run --release --bin foc
```

该配置只属于此独立 example，不使用 Cargo workspace。

算法验证拆成了同级独立 Cargo crate，均使用 `foc-algorithms` 的同一份
实现：

底层算法本身位于 `foc-algorithms/` 独立 crate，包含电流 PID、速度 PID、
Clarke/Park 变换、SVPWM 和电机参数。完整速度闭环与耗时 crate 都通过
这个 crate 使用同一份实现。

| crate | 用途 |
| --- | --- |
| `algorithm-timing/` | 用 BTIM1 统计电流环和速度环耗时 |
| `speed-loop/` | 完整速度闭环 FOC 应用，包含 PWM/ADC/编码器板级代码 |

进入对应目录后执行 `cargo run --release --offline`。这些目录各自有
`Cargo.toml`，没有加入 workspace。

## 项目约束

FOC 相关代码禁止使用 64 位整数类型和转换。固定点、速度和相位计算必须
保持在 32 位范围内，必要时采用分段乘除或商余数计算。ADC1 采样前必须启用
前置运放：OPA1 为 PA6/PA7 → PB0，OPA2 为 PA4/PA5 → PB1。
