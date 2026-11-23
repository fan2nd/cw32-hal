# Staged SPI DMA firmware

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

The primary Cortex-M0+ application, `cw32-spi-dma-example`, supports
CW32F030C8T7, CW32A030C8T7, the five exact L083 packages RBT6, RCS6, RCT6,
MCT6 and VCT6, and these four retained Stage93 representatives: CW32F020C6U7,
CW32L031C8T6, CW32L052C8T6, CW32R031C8U6. It owns SPI1
(PA5 SCK, PA7 MOSI, PA6 MISO, all AF5) and PB0 active-low chip select.

| SPI1 selection | TX / RX channels | DMA handlers | CS speed |
| --- | --- | --- | --- |
| F020C6U7 | CH2 / CH1 | DMACH2 / DMACH1 | `Speed::High` |
| L031C8T6, L052C8T6, R031C8U6 | CH3 / CH2 | Both on DMACH23 | `Speed::Default` |
| F030C8T7, A030C8T7 | CH3 / CH2 | Both on DMACH23 | `Speed::High` |
| Existing L083 exact packages | CH3 / CH2 | Both on DMACH23 | `Speed::Default` |

RX has higher hardware priority than TX in each selection. The SPI1 error
handler is bound alongside the actual DMA vectors. Normal reset/clean-runtime
`hal::init` is required.

The two singleton SRAM buffers have different capacities (16/24). Ordinary
stack slices and a flash slice exercise unequal-length transfers, in-place
replacement, reads, writes, empty calls and multi-chunk transfers through all
five `embedded_hal_async::spi::SpiBus<u8>` methods. RX beyond the caller's read
length is discarded; missing TX is zero. No expected-device-data assertion is
made. Arbitrary byte traffic is suitable only for a device/fixture selected and
configured for this example; review its protocol and electrical limits before
flashing. An ELF link does not prove a physical transaction worked.

The `spi2_flash_id` application supports F020C6U7, L052C8T6 and all five
existing L083 exact packages. It uses SPI2 (PA2 SCK, PA1 MOSI, PA0 MISO, all
AF5) and PB0 active-low CS, with the independent SPI2 error handler.

| SPI2 selection | TX / RX channels | DMA handlers | CS speed |
| --- | --- | --- | --- |
| F020C6U7 | CH2 / CH1 | DMACH2 / DMACH1 | `Speed::High` |
| L052C8T6 | CH4 / CH1 | DMACH4 / DMACH1 | `Speed::Default` |
| Existing L083 exact packages | CH5 / CH4 | Both on DMACH45 | `Speed::Default` |

All signal and CS pins are bonded on the selected exact packages. F020 has
only two DMA channels, so its SPI1 and SPI2 DMA examples are separate firmware
deployments. DMA admission is permanent even after clean Drop; these examples
do not demonstrate simultaneous paired DMA owners on both F020 controllers.
Both channel admissions are checked before either is claimed. Channel rejection
leaves both admission states unchanged, but constructor inputs remain consumed
and later SPI configuration failure does not roll back successful admission.

Connect a voltage-compatible SPI NOR flash supporting the mode-0 JEDEC ID
command `0x9f` and status-register-1 command `0x05`, with common ground and any
required HOLD/WP wiring. Check the device's datasheet and board schematic first.
The default requested SCK is 1 MHz. The application reads three ID bytes with
an in-place transfer, then repeatedly writes a status command and reads one
byte under the same CS selection. It never issues a flash program, erase or
write-enable command. ID/status bytes pass through `black_box` for inspection;
their values are not asserted. Both logical writes and reads use paired byte
DMA, including receive discard and zero dummy transmission. The 16/24-byte
private buffers remain owned throughout. L083 retains its existing maximum
12 MHz SCK policy and supported divisors 4 through 128.

The four retained extension families also retain IRQ PIO constructors. Each owner selects its
backend at construction: `new_with_dma` requires static peripheral, pin and
channel owners, interrupt proof and exclusive static staging; `new_async` retains borrowed PIO ownership. Common async methods keep that
installed backend. PIO on the other F020 controller remains useful while the
DMA pair is reserved. No mixed-backend hardware workload is exercised here.

Keep the original CS selection through any cancellation, then await successful
`bus.flush()` before deselecting or selecting another device. A generic device
adapter that drops CS or releases its bus lock on cancellation cannot infer this
property from `SpiBus`. The example does not cancel operations; on error it
halts while retaining the selection and poisoned bus. Completed chunks may
already be visible before later cancellation/error. Chunk boundaries can pause
arbitrarily; DMA arbitration can still cause receive overrun.

These representative firmware selections do not cover all thirteen retained
source-qualified exact parts. F020F6U7 and L031F8P6/F8U6 lack PB0 and need a
separately chosen bonded software CS, such as PB1, in a future application.
R031 retains its whole-GPIOA handover implications.

Build from the repository root with the project's Rust/Cargo toolchain:

```sh
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/spi-dma/Cargo.toml --no-default-features --features cw32f030c8t7
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/spi-dma/Cargo.toml --no-default-features --features cw32a030c8t7
for chip in cw32f020c6u7 cw32l031c8t6 cw32l052c8t6 cw32r031c8u6; do
  cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
    --manifest-path examples/shared/spi-dma/Cargo.toml --no-default-features --features "$chip"
done
for chip in cw32l083rbt6 cw32l083rcs6 cw32l083rct6 cw32l083mct6 cw32l083vct6; do
  cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
    --manifest-path examples/shared/spi-dma/Cargo.toml --no-default-features --features "$chip"
done
```

The exact F020C6U7, L052C8T6 and all five L083 features automatically enable
`spi2-example`, so `--bins` includes `spi2_flash_id`. The separate `l083`
marker identifies only the L083 family. Select one exact chip, not either
marker alone. Generic aliases lack exact qualified SRAM extents and are not
offered.

The build script checks the actual bonded signal/AF routes and PB0 CS for every
enabled binary, derives FLASH/RAM only from the selected exact package metadata,
and links with the real Cortex-M runtime. Dependencies and the Embassy revision
remain locked. Source checks and ELF links do not establish interrupt wakeups,
cancellation races, DMA bus retirement, wire timing, throughput, power or RF
behavior. See [source and lifecycle evidence](../../../docs/spi-dma.md).
