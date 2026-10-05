# CRC and independent watchdog

Both drivers own their real peripheral token and a counted RCC configuration-clock lease. Their synchronous operations do not require an interrupt binding.

## CRC

`crc::Crc::new(p.CRC, crc::Algorithm::Ccitt)` initializes the selected algorithm. `feed_byte` / `feed_bytes` continue the current calculation, `read` observes it, and `reset` restarts it. `set_algorithm` explicitly starts a new calculation; rewriting MODE is a hardware initialization command.

L012 implements eight CRC16 algorithms. Its 32-bit DR access consumes the low byte. F030 adds CRC32 and CRC32/MPEG-2 and implements `feed_halfword(s)` and `feed_word(s)` using the actual 16/32-bit PAC aliases. Multi-byte inputs are consumed low byte first. CRC32 variants and wide access methods are absent from the L012 API, selected by evidenced register capabilities.

`Ccitt` is the vendor's reflected, initial-zero CRC16_CCITT (also called KERMIT); `CcittFalse` uses initial 0xffff without reflection. The names do not promise arbitrary polynomial/initial-value configuration. Drop releases the clock; there is no asynchronous operation or outstanding memory access.

Evidence: CW32L012 RM1.4 §§10.3–10.6, printed pp141–145; CW32x030 RM2.5 §§10.3–10.6, pp160–164. Manual URLs and SHA-256 values are listed in [DMA hardware evidence](dma-hardware-evidence.md#primary-sources-and-pins). F030 p161's final 32-bit example has a transposed hex word; the explicit low-byte-first rule and preceding word list establish the implemented ordering.

## Independent watchdog

`wdg::IndependentWatchdog::new(p.IWDT, config)` acquires without reset or start. `unleash` starts and configures it, `pet` reloads it, and CW32's documented `stop` is explicit. The driver implements reset-on-expiry, disables the early-feed window and IRQs, and leaves counting enabled during DeepSleep. IWDT window/IRQ modes remain later work. The separate [window watchdog](window-watchdog.md) is implemented from v0.20.

The counter uses a dedicated, approximate 10 kHz RC oscillator. Configuration exposes the actual /4…/512 prescaler and 12-bit reload value; `nominal_timeout_us` assumes precisely 10 kHz and is not a deadline guarantee. Default settings are nominally 1 second. Applications need margin based on their chip's oscillator tolerance. `poll_limit` bounds status reads for each synchronization, rather than elapsed time.

CW32 requires starting before protected configuration writes. The driver waits for RUN, unlocks, waits each CR/ARR/WINR update, disables the window with 0xfff, then reloads and relocks. A start/configuration timeout may leave an active watchdog with partially applied configuration; `pet` rejects an incomplete start, while explicit `stop` remains available. Starting over an inherited running watchdog returns `AlreadyRunning`; no constructor silently disables or resets it.

Drop and forgetting never stop a running watchdog. Drop permanently retains its configuration-clock resource when running or after an uncertain start, so releasing the Rust owner does not disable protection. Only successful explicit stop allows normal release. The application is responsible for feeding or accepting the eventual reset after it gives up the owner.

Evidence: L012 RM1.4 §§19.3.2–19.4, pp427–431 and §19.6, pp433–435; F030 RM2.5 §§16.3.2–16.4, pp312–315. KR 0xcccc starts, 0x5555 unlocks, 0xaaaa reloads, and sequential 0x5a5a/0xa5a5 stops. A key other than 0x5555 also locks protected registers.

## Upstream comparison

The fixed Embassy source is [CRC](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/crc/v2v3.rs) and [watchdog](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/wdg/mod.rs). CRC follows its owned stateful feed/read/reset pattern and width-specific methods. Watchdog follows explicit `unleash`/`pet`; configuration moves into `unleash` because CW32 requires an already-running watchdog. The CW32 stop command is a real additional hardware capability, not an assumed STM32 behavior.

The watchdog owner is generic over the generated sealed `wdg::Instance`
(v0.20), and its registers and clock are obtained through that same identity.
`WindowWatchdog` similarly uses `WindowInstance`. Ordinary constructor calls
infer the peripheral type from their token; no trait object or copied singleton
is involved. CRC remains concrete, matching the fixed upstream CRC driver.
