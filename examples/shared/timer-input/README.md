# Buffered timer input firmware

Two genuine `no_std` Cortex-M0+ firmware binaries use external input signals. They neither generate a waveform nor assume loopback. Compiling/linking is the only verification performed; no firmware was run or flashed.

- `polling_capture`: GTIM1 CH1 captures both edges with four-sample filtering. GTIM1 CH2 captures every second falling edge. ATIM's bonded qualified channels capture rising edges without filtering. Raw 16-bit timestamps are visible in `GTIM_TIMESTAMPS` and `ATIM_TIMESTAMPS`; they wrap and do not imply elapsed time or a sample count. `OVERRUN_OBSERVATIONS` counts polling observations only.
- `quadrature_encoder`: GTIM1 uses x4 with four-sample filtering. ATIM uses x2 on CH1 for L010, and x2 on CH2 for L011/L012. The raw modulo positions are visible in `GTIM_POSITION`/`ATIM_POSITION`. L010Y8M6 has no qualified ATIM CH1/2 pair, so that binary only constructs the GTIM decoder there.

| Family/package | GTIM1 CH1 / CH2 | ATIM capture CH1 / CH2 / CH3 / CH4 | ATIM encoder |
|---|---|---|---|
| L010F8P6, L010F8U6 | PA6 / PA5 | PB4 / PB2 / PA3 / omitted | PB4 + PB2 |
| L010Y8M6 | PA6 / PA5 | omitted / omitted / PA3 / omitted | unavailable |
| L011K8T6, L011K8U6 | PA6 / PA7 | PA8 / PA9 / PA10 / PA11 | PA8 + PA9 |
| L012C8T6, L012C8U6 | PA6 / PA7 | PA8 / PA9 / PA10 / PA11 | PA8 + PA9 |

GTIM pads use AF6; ATIM pads use AF7. Consult the [source matrix](../../../docs/timer-input-source-evidence.md) for physical package pin numbers. All pads use `Pull::None`; provide suitable external bias, common ground and clean signals. Keep input voltage/timing within the exact device's datasheet. Do not assume a 24 MHz external input limit unless the timer actually runs at 48 MHz; filtering reduces accepted bandwidth, and CPU polling can overrun at much lower rates.

Build one package from this directory:

```sh
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi --no-default-features --features cw32l011k8t6
```

The build script derives the linker memory map from the selected exact-package metadata. `ci/check-timer-input.sh` builds all supported package examples, plus the library feature/regression matrix.

With the optional local `time-driver` feature, GTIM1 belongs to Embassy time and these programs use only ATIM inputs. Encoder coexistence is unavailable on CW32L010Y8M6 because no qualified ATIM channel1/channel2 pair is bonded; its standalone capture program can still use ATIM channel3. Other six exact packages can link both programs with the reserved time driver.

`signed_position` consumes the same encoder owners into sampled signed-position
tracking. The example declares at most 1024 net counts between samples; the
board/application must actually guarantee that bound including interrupt delays.
Excess motion can alias undetected. Detected ambiguity/overflow panics instead
of silently resynchronizing. The new path is uncompiled and untested; see
[signed-position contract](../../../docs/qei-signed-position.md).
