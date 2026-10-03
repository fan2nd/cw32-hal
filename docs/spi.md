# SPI controller buses

`embassy_cw32::spi::Spi<'d, I, M>` owns a generated SPI instance, SCK/MOSI/MISO pin tokens and its counted peripheral clock. `new_blocking` selects `Blocking`; `new` additionally requires a checked `Binding<I::Interrupt, InterruptHandler<I>>` and selects `Async`. Pin traits and alternate functions come from the selected chip metadata. The driver configures push-pull SCK/MOSI and floating MISO. Pin availability still depends on the package and board; debug pins without an explicit debug-release API are excluded from safe bus routes.

The driver is a full-duplex controller bus. Both modes implement `embedded_hal::spi::SpiBus<u8>` and `SpiBus<u16>`; `Async` also implements the corresponding `embedded_hal_async` traits. A `u8` operation uses eight-bit frames and a `u16` operation uses sixteen-bit frames. The manuals document other widths from four to sixteen bits, but this version exposes only these two word sizes. All four standard SPI modes and both bit orders are supported. There is no target, half-duplex, hardware-CS, CRC, FIFO, or DMA interface in this API.

## Construction and device chip select

For SPI1 on either supported chip, PA5/PA7/PA6 are documented SCK/MOSI/MISO routes. The generated driver selects each chip's actual AF value.

```rust,ignore
use embassy_cw32::{bind_interrupts, peripherals, spi};

bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<peripherals::SPI1>;
});

let p = embassy_cw32::init(Default::default());
let mut config = spi::Config::default();
config.frequency = 1_000_000;
config.mode = spi::MODE_0;
let mut bus = spi::Spi::new(p.SPI1, p.PA5, p.PA7, p.PA6, Irqs, config)?;
let mut words = [0x9fu8, 0, 0, 0];
bus.transfer_in_place(&mut words).await?;
```

For blocking access, use `Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, config)` and `blocking_transfer_in_place`. A blocking bus does not need an IRQ binding. Bindings via `bind_interrupts!` require the HAL `rt` feature.

Chip select is an independently owned GPIO. Wrap the bus and a GPIO output in an appropriate `SpiDevice` adapter, such as the exclusive or shared bus adapters in the standard embedded-hal ecosystem, so command/data phases stay in one transaction. The HAL uses internal `SSM=1` and never claims or toggles a physical SPI CS pin. It is the device adapter's job to assert/deassert CS and provide any device-required timing. A direct `SpiBus` call alone does not select a device.

L012 SPI2 and SPI3 share `SPI23`. An application using both async buses must list both `InterruptHandler` types in that vector's `bind_interrupts!` entry. Each handler only services its own peripheral. The driver never disables or clears the shared NVIC vector on cancellation or drop.

## Frequency and configuration

The SPI kernel is the generated PCLK source, with no timer-style clock multiplier. `Config::frequency` is a maximum nominal SCK rate. Selection uses exact integer comparisons so fractional division cannot silently overspeed that maximum:

- SPI IP `l012`: SCK = PCLK / (2 × (BR + 1)), BR 0..127, divisors 2, 4, 6, …, 256.
- SPI IP `f030`: SCK = PCLK / 2^(BR + 1), BR 0..6, divisors 2, 4, 8, …, 128. Encoding 7 is reserved and is never selected.

A request above PCLK/2 selects PCLK/2. Zero frequency or a request below the slowest representable rate fails before hardware/pin configuration. `frequency()` reports the selected nominal rate rounded down to integer Hz; `kernel_clock_hz()` and `clock_divider()` expose its exact ratio. Oscillator tolerance and electrical limits remain board-level constraints.

`set_config` validates first, then stops any outstanding operation before changing control registers. External chip selects must already be inactive. On `spi_l012`, all CR1 writes occur with CR2.EN clear, as required by the manual. Slew-rate configuration exists only when generated `gpio_has_speed` is present. Delayed sampling, input filtering, interframe gaps, and target-only options are left off.

## Completion, errors, cancellation and bounds

An exchange transmits one frame, waits for RXNE, and reads DR before queuing another frame. Reads transmit zeros. Writes consume and discard the received words. Unequal slice transfers send `max(read.len(), write.len())` frames, padding a shorter write slice with zeros and discarding any receive tail. An empty transfer sends no frames.

Successful methods finish with TXE=1 and BUSY=0. `flush` verifies this condition; it does not invoke the destructive hardware `ICR.FLUSH` command. Each blocking wait and the final BUSY drain has `Config::poll_limit` register observations (default 100,000). This allowance is finite and is not an elapsed-time timeout. Zero is rejected. A budget that is too short can abort an otherwise healthy slow transfer.

Async TXE/RXNE waits register a waker, check completion and then enable the relevant peripheral interrupt under a critical section. The interrupt masks the level source and wakes the task; it never retains buffer pointers. There is no self-waking polling future. Async frame waits have no built-in deadline and can be cancelled with an application timeout. CW32 has no BUSY-clear interrupt, so after receiving the last word the final clock-edge tail uses the same bounded synchronous drain as blocking mode. A successful operation and a normal subsequent `flush` leave no background transfer.

Hardware mode fault, receive overrun, underrun, select error, and exhausted polling budgets are returned as errors. Any error or dropped in-progress future disables peripheral interrupts and EN, then uses the documented W0C `ICR.FLUSH=0` command to clear the transmit buffer and shift register; receive/error state is discarded too. No pending word continues after cancellation. The next operation restores the saved controller configuration. Cancellation may truncate a frame already visible on the wire, so the device transaction must be abandoned and its CS restored by its owner.

Forgetting an in-progress future cannot expose a borrowed buffer to an ISR or DMA. At most one frame is outstanding; a later operation aborts that frame before restarting, and `flush` can drain it. Dropping the driver stops hardware while both clock and pin ownership remain live, disconnects the pins, then releases the counted clock. Interrupt dispatch after the last clock release checks the clock resource before any MMIO. There is no runtime RCC switching support.

## Audit basis and validation limits

The API/ownership shape was compared with the actual [Embassy STM32 SPI source at b12a6d9efcd2711037abca1b63a661a9ef726444](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/spi/mod.rs). Its STM32 register layouts, /256 encoding and DMA workflow were not treated as CW32 hardware evidence.

The backend register sequences, clock formulas, GPIO direction, completion and flag semantics follow the official [CW32L012 manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf), sections 22.3.1–22.3.4, 22.3.8–22.3.9, 22.6.1 and 22.7 (printed pp.493–498, 506–508, 510–512, 516–522), and [CW32x030 manual Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf), sections 19.3.1–19.3.4, 19.3.8–19.3.9, 19.6.1 and 19.8 (printed pp.362–369, 375–376, 379–382, 388–393), with the pinned vendor CMSIS headers for per-IP offsets. In particular, ICR writes zero to clear and bit 0 clears the shift register; ordinary flag clearing must never accidentally issue that command.

Compile checks and external register/interrupt probes can establish software contracts, but do not establish board-level signal integrity, maximum reliable SCK, or physical wire timing. Those require a connected board and logic-analyzer verification.

The additional [finite typed DMA API](bus-dma.md) consumes complete static owners
and buffers. It has distinct completion and permanent-quarantine rules; these
CPU-driven operations retain their existing borrowed-buffer behavior.
