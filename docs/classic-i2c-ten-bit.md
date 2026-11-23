# Classic I2C ten-bit master

The classic SI/STAT backend now accepts explicit `Address::SevenBit(u8)` and
`Address::TenBit(u16)` in blocking/async `transaction_with_address` methods.
Existing u8 methods remain seven-bit. Both ten-bit embedded-hal traits and
explicit-address retry methods use the same transfer engines. Values above
127/1023 are rejected even with an empty operation list; asynchronous owners
first retire forgotten work. Empty reads reject, empty writes probe and adjacent
same-direction operations merge as before.

The address sequence follows NXP UM10204 ten-bit addressing rules,
available as an [NXP-hosted specification](https://community.nxp.com/pwmxy87654/attachments/pwmxy87654/other/25158/1/UM10204%20Rev.7.pdf).
For address A, H = 0xf0 | ((A >> 8) << 1), L = A & 0xff:

| Run | Address bytes and STARTs | Successful SI/STAT observations |
| --- | --- | --- |
| Write | START H, L | 08/10, 18, 28 |
| Initial read | START H, L, repeated START H+1 | 08, 18, 28, 10, 40 |
| Read following write | repeated START H+1 | 10, 40 |
| Write following read | repeated START H, L | 10, 18, 28 |

Controller behavior is composed from the already audited DR/CR state machine in
[i2c-typed-register-evidence.json](i2c-typed-register-evidence.json), including each
classic family's own manual references. L010 CN1.2 §18.4.10.1 p473/PDF474 and
its p474/PDF475 flow show software-supplied SLA+W, arbitrary following data,
repeated START and the same ACK/NACK states. §18.7.6 p494/PDF495 marks ADDR0
as slave-only. These are software-generated address bytes, not a new hardware
address mode or ten-bit slave capability. The feature list's seven-bit matching
is not treated as evidence for ten-bit slave support. No L012 FIFO behavior is
copied into the classic controller.

The low address byte uses hardware data ACK/NACK states 28/30. Only in this
address phase is DataNack remapped to AddressNack. Payload NACK remains DataNack;
START, arbitration, timeout and bus errors retain their classifications. An
AddressNack retry may repeat earlier payload effects after a later direction
change; opt-in retry is never atomic. The same bounded address-phase description
is used by blocking and asynchronous engines.

START/DR/SI ordering, final-read NACK, final STOP, async cancellation guard,
forgotten-transfer retirement and permanent fault isolation retain their existing
paths. Read-start inserts an additional repeated START after the low byte; each
new wait is inside the existing guard. No pointers are installed in the ISR.
There is no new DMA, slave, GPIO recovery, automatic retry or timing guarantee.

`examples/shared/classic-i2c` adds `ten_bit_master` with the existing package-qualified
primary pin routes. It attempts one write/read at illustrative address 0x2a5 and
parks. The board must provide a matching ten-bit target and external pull-ups.
Source review only: no local compilation, tests, generation or board execution.
All-address encoding, NACK at each address byte, direction changes, cancellation,
seven-bit regression and per-family builds remain pending acceptance.
