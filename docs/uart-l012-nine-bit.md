# L012 ordinary UART nine-bit payloads

This document records the L012 qualification. Current L010/L011 support and
its pending acceptance are described in [native UART scope](uart-native-word-half-duplex.md).


This delta qualifies L012 UART1/2/3 ordinary blocking and IRQ-assisted PIO for
nine data bits without parity. Existing pin, package, clock and interrupt
qualification remains unchanged. `Config::data_bits` defaults to
`DataBits::DataBits8`; select `DataBits::DataBits9` explicitly.

## API and boundaries

- L012 ordinary `Uart`, `UartTx` and `UartRx` provide `write_u16`, `read_u16`
  and their `blocking_` counterparts. These use u16 containers for low eight or
  nine payload bits according to the fixed owner configuration. Higher TX bits
  are ignored; unused RX bits are masked to zero. Read lengths count complete
  frames, not bytes. Eight-bit owners can also use these PIO methods.
- Nine data bits plus parity are rejected with `ConfigError::InvalidParity`.
  Other families reject nine-bit configuration with `UnsupportedDataBits`, and
  do not expose these u16 methods. A similarly named custom-parity register on
  another IP is not treated as qualification.
- Buffered UART and staged DMA remain byte-only. Their constructors reject nine
  bits before channel admission, ring installation, clock/pin changes or UART
  MMIO. Inputs are still consumed on constructor failure under the existing
  contract; this does not return peripheral tokens or buffer ownership.
- On a nine-bit ordinary owner, byte reads/writes, byte `read_until_idle`, and
  byte embedded-io operations return `Error::WordSizeMismatch` before MMIO,
  waiter registration or interrupt retirement, even for empty data operations.
  They never silently truncate bit8. Flush remains width-independent.
- On a DMA-selected owner, u16-container operations return
  `Error::UnsupportedWordTransfer` before inspecting/settling its lease or
  touching registers, even for empty slices. There is no implicit CPU fallback.
  A rejected call does not retire earlier work; explicit flush keeps its role.
- There is no nine-bit buffered ring, DMA transfer, idle-read variant, address
  filtering, half-duplex, LIN or RS485-direction implementation in this delta.
  In particular the L012 DMA RX RC/ORE evidence blocker remains unresolved.

The pinned [Embassy STM32 USART implementation](https://github.com/embassy-rs/embassy/blob/f16efeffe37581092ec184718e6fdb1620393214/embassy-stm32/src/usart/mod.rs)
provides DataBits9 and these u16-container method names. This aligns the public
shape, not STM32's DMA/register semantics. CW32 deliberately rejects byte API
use with nine-bit owners instead of allowing silent payload truncation.

## Exact source qualification

The official [L012 CN user manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
is 18,034,510 bytes; SHA256:
`a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
It was downloaded and matched the existing source authority on 2026-10-14;
no historical source identity was changed.

- §21.3.3.1 p450 table21-2 maps CHLEN=1/PARITYEN=0 to nine payload bits and
  CHLEN=1/PARITYEN=1 to eight payload bits plus parity.
- CR1 pp480–481 confirms CHLEN bit6 and independent PARITYEN bit3. CHLEN=0
  selects eight bits and disables parity. Existing eight-bit parity setup is
  unchanged; nine-bit setup sets CHLEN and leaves PARITYEN clear.
- TDR p485 and RDR p486 have nine-bit payload fields. Typed 32-bit register
  accesses are retained. TDR IDLE/BREAK command bits are not set by data writes.
- ISR pp487–488 specifies RC, ORE, PE, FE, NE, TXE, TC and TXBUSY per frame.
  ICR p489 specifies the existing observed-event R1W0 acknowledgment protocol.
  These facts do not prove a new DMA handshake or lossless reception.

Official SDK v1.0.5 archive SHA256:
`8b0a4c0ee865642d08f353aec98906c900a8b10f672120e649e6b1bf5e29c18f`.
`Libraries/src/cw32l012_uart.c` SHA256:
`49199142bb1e564d0edc297a08bf5e2b138b96111b9fc42b81539419c051c065`.
Its UART_SendData/UART_ReceiveData explicitly describe eight/nine-bit transfers
and use uint16_t. Its initializer alone does not expose nine-data-bit/no-parity
configuration, so the manual is the configuration authority. Its RC-before-RDR
receive ordering is not copied; the existing HAL RDR-before-ack protocol stays.

### Page extraction receipts

Extracted with pdftotext25.03.0 `-layout` using UTF-8. Hash each exact segment
of the full output split on form feed, excluding that separator. PDF page
numbers are one-based; printed pages are26 less. These are source-reading
receipts, not hardware or runtime test results.

| Printed / PDF page | Extracted-page SHA256 |
| --- | --- |
| 450 / 476 | `568350913b7a65a6cc3facef61b10e2f5afb41e7baf1b83e720a5bbd9572b100` |
| 480 / 506 | `b4f73d70f6d277d9b4299f1b56156ce7c2e19a7e99f11908728af2fd3ef9888d` |
| 481 / 507 | `4ee159132796aab07fc8e2cbaccd6395d44828e178e2231c78714178de623401` |
| 485 / 511 | `0b91092d2039c3197c1e18b4d8cd1f8005b2016b39fa77687655604a0fd9b212` |
| 486 / 512 | `c05634d4470a229c4bef1a1eb851102b6d151eb8ef68968d363c210a4e80a884` |
| 487 / 513 | `7879f9248a76ff2a296396fe27485109fafe0151c01e9234eeb61580219f9860` |
| 488 / 514 | `9dabcbb9543d3bf62c9b71f189a22937007763e88a51c69ed55bfab0d9d699a4` |
| 489 / 515 | `e2ef5789c775487c64ed838ab30b82d9a36deb69f0a637ccd1ccb2f688683e62` |

## Lifecycle and error contract

Each ordinary half retains its configured data-bit count through split and
split_ref. No runtime width mutation is introduced. New u16 operations hold
caller buffers only inside their own operation future; no address is installed
in ISR state. The ordinary one-shot waits, cancellation, forgotten-wait retirement
and Drop lifecycle are unchanged. A canceled read/write may have completed a
prefix; no lossless, timed-completion or transactional rollback claim follows.

The full-width receive helper preserves one status observation, an RC-gated RDR
read before one ICR write, and acknowledgment of only observed events. Error
precedence remains ORE > PE > FE > NE. Eight-bit callers use a wrapper which
narrows only after the same observation/error result. No parity error is ignored
just because parity was configured off. RX flags can coalesce under delayed
service, as before. An empty supported u16 read retires an old receive wait but
does not consume RDR; rejected operations perform no retirement.

TX still waits for TXE and clears the old TC indication before writing TDR.
Flush still waits for the buffer and shifter to empty via TXBUSY/TC, not TXE
alone. Dropping without flush can truncate outstanding transmission. Existing
shared-vector dispatch on other families is untouched; L012 UART1/2/3 themselves
have distinct UART1/UART2/UART3 vectors, not a fabricated shared UART23 vector.

## Software checks

Rust1.99 actual-source helper tests on 2026-10-14 passed:

- Three native configuration/payload tests: eight-bit parity preservation,
  nine-bit/no-parity acceptance, nine-bit/parity and byte-backend refusal,
  byte-operation mismatch, and eight/nine-bit payload masks.
- Two non-L012 tests: nine-bit configuration rejection and byte mismatch.
- Two receive tests:512 combinations of flags, idle acknowledgment and payload
  compare the old byte helper and new byte wrapper result/register trace;
  separate nine-bit/error tests preserve bit8, error priority and RDR-before-ICR.

The narrow register-trace harness runs extracted production helper bodies against
recording register stand-ins. It is not a silicon simulator or IRQ timing test.
The constructor refusal order and IRQ/ownership behavior also require source
review; helper success alone does not certify those paths.

`examples/shared/uart-ninebit` contains separate ordinary blocking and async split-owner
loopback firmware for C8T6/C8U6, UART1 PA2(TX)/PA3(RX). Build/link and public API
matrix results are recorded in the batch validation receipt after execution.
No hardware loopback, physical frame timing, sustained-throughput or electrical
validation is claimed.
