# Safe staged SPI DMA

Stage94 adds native L012 SPI1/SPI2/SPI3 on exactly CW32L012C8T6 and
CW32L012C8U6 to the existing classic profiles. The implementation scope is
finite full-duplex eight-bit master DMA with two exclusive static SRAM stages.
Stage94 main passed six standalone ARM libraries and fifteen ELF links in
21 accepted Cargo rows, all exiting zero, plus five finite source/declaration
commands. Independent runtime and metadata/source reviews accepted the bounded
implementation. At main acceptance the final-source two-library/six-ELF clean
replay had not run; final completion requires its separate receipt.
The first M07 ELF link failed with a 2,752-byte Flash overflow after all six
standalone libraries passed. The reviewed native companion-only release
profile correction from opt-level s to z preserves the 64 KiB Flash limit and
all five async methods. That failed attempt remains part of the record; the
21 accepted rows are not the total number of attempts. Native L012 builds
have a recorded new dead_code warning for new_admitted_pair; no warning-free
claim is made. No hardware execution or measurement is claimed. The Stage93 record below
retains its historical source and acceptance boundary.

Stage93 main software verification and independent runtime and metadata/source
reviews accepted the bounded F020/L031/L052/R031/W031 extension to the existing
F030/A030/L083 staged SPI DMA scope. Main produced thirteen standalone ARM
libraries and twenty-four linked ELFs in 37 actual Cargo invocations, all exiting
zero. The twenty-four dependent HAL archives retained with those ELFs are not
additional standalone library commands. Five finite source/data gates and a
separate source-navigation refresh passed. Main ran no canonical generation;
all 1,478 frozen source files, all 910 generated files and the recorded toolchain
were unchanged through the main run.

The first main attempt stopped while retaining M16 artifacts with ENOSPC after
Cargo had already linked successfully. Reviewed immutable-artifact deduplication
and recovery preserved the failed attempt and reused verified M01–M15. M16's
original outputs were retained without another Cargo invocation; only M17–M37
ran during the continuation. The accepted main result binds that recovered
37-row record, not a clean rerun or an erased failure. L010F8P6 and L011K8T6
library and async-PIO ELF rows each retained three existing unused RCC LSE
constants; the other eleven libraries and twenty-two ELFs were warning-free.

Main used the explicitly recorded Rust/Cargo 1.99.0 toolchain and the separate
project source and external vendor-evidence roots. At main acceptance final
capability reports and the separate final-source two-library/six-ELF clean replay
had not run; final-package completion requires their own receipts against the
final documented source. These main receipts remain bound to their frozen main
inputs. Earlier SPI and DMA receipts retain their dated scope; no hardware
result is claimed.

`Spi<'static, Async, Master>::new_with_dma` consumes whole static SPI,
SCK/MOSI/MISO, two admitted DMA-channel tokens, and two exclusive static SRAM
slices. Its ordinary borrowed `transfer`, `transfer_in_place`, `read`, `write`
and `flush` methods implement `embedded_hal_async::spi::SpiBus<u8>`. Caller
memory is never a DMA endpoint. Neither source evidence nor software validation
qualifies silicon, throughput or generic device-adapter cancellation behavior.

## Current scope and backend choice

The five-family extension is seven new family/controller pairs, fourteen exact
memory-qualified parts and twenty selected exact-part/controller routes. With
F030/A030/L083, Stage93 covered eight families and thirteen
family/controller pairs. Those classic routes use the existing CR1 request
gates and typed DR endpoint; each family's own sources supply the byte-DMA
qualification recorded in [the current evidence ledger](classic-spi-dma-evidence.json).

| New family | SPI instances | RX/TX selectors | Physical DMA channels / vectors |
| --- | --- | --- | --- |
| F020 | SPI1, SPI2 | SPI1 6/7; SPI2 8/9 | CH1/DMACH1, CH2/DMACH2 |
| L031, R031, W031 | SPI1 | 6/7 | CH1/DMACH1, CH2+CH3/DMACH23, CH4/DMACH4 |
| L052 | SPI1, SPI2 | SPI1 6/7; SPI2 8/9 | CH1/DMACH1, CH2+CH3/DMACH23, CH4/DMACH4 |

The exact parts are `cw32f020c6u7`, `cw32f020k6u7`, `cw32f020f6u7`,
`cw32l031c8t6`, `cw32l031c8u6`, `cw32l031f8p6`, `cw32l031f8u6`,
`cw32l031k8u6`, `cw32l031k8v6`, `cw32l052c8t6`, `cw32l052r8s6`,
`cw32l052r8t6`, `cw32r031c8u6` and `cw32w031r8u6`. All retain 8 KiB SRAM at
`[0x20000000,0x20002000)`; F020 retains 32 KiB Flash and the other eleven parts
64 KiB. Generic aliases retain `MissingMemoryMetadata` safe-staging/admission
rejection. No alias memory map or package route is invented.

The selected SPI1 triple is PA5/PA7/PA6 and SPI2 is PA2/PA1/PA0, in
SCK/MOSI/MISO order, all AF5. The ledger binds all twenty exact package routes;
this does not newly audit every alternative AF permutation. CS is separately
owned bonded GPIO. The representative DMA examples use PB0 for both binaries;
PB0 is absent on F020F6U7 and L031F8P6/F8U6, which require a bonded alternative
such as PB1 in a future application. The existing maximum SCK is 12 MHz, with
minimum divisor 2 on F020 and 4 on L031/L052/R031/W031.

F020 has only two DMA channels. One full-duplex DMA owner consumes both;
SPI1 and SPI2 DMA examples are separate firmware deployments. Clean Drop does
not restore one-time channel admission. PIO on another controller remains
useful while the pair is reserved. Four-channel shared-vector handlers preserve
each sibling's independent flags and waiter; no fifth channel is available.

F020/L031/L052/R031/W031 retain ordinary-borrowed `new_async`
[IRQ PIO construction](spi-async-pio.md) alongside static `new_with_dma`.
The constructor fixes the backend for that owner. One lifetime-generic async
method/trait surface dispatches to the installed backend; it does not switch
backends, relax static DMA ownership or admit two owners of one controller.
Different controllers may use different backends. Dedicated interrupt aliases
still resolve to `InterruptHandler<T>` and dispatch to separate PIO and DMA
states, each checking its active guard before MMIO. The DMA constructor now
requires `Binding<T::Interrupt, T::InterruptHandler>`. A downstream generic
wrapper constrained only by `Binding<T::Interrupt, InterruptHandler<T>>` may
need the associated-handler bound or an appropriate associated-type equality.
Preserved concrete classic bindings do not prove universal generic source
compatibility.

On the five Stage93 dual-capable families and now L012, the already `#[non_exhaustive]`
`ConfigError` and `Error` enums gain their existing DMA-gated variants:
`ConfigError::{DmaBuffer, DmaChannel, DmaQuarantined}` and
`Error::{Dma, DmaBusy, DmaQuarantined}`. Valid external matches already need a
fallback; in-crate exhaustive/cfg-specific matches and explicit DMA handling may
need updates. Preserved PIO signatures do not mean an unchanged public variant
set. F030/A030/L083 remain DMA-only; F002/F003/L010/L011 remain PIO-only.

A private owner of the original static RX allocation preserves automatic
`Send`/`Sync` derivation for existing PIO and blocking owners when DMA becomes
available. The same correction also permits existing F030/A030/L083 DMA and
blocking owners to satisfy these traits, subject to their other fields and
borrow lifetimes. Only the private storage token has explicit trait proofs;
`Spi` and `SpiDma` remain compiler checked. Moving an owner does not move its
static allocation or end a transfer, and shared access exposes no RX bytes.
CPU RX copying still needs exclusive access after both fenced retirements.
This does not make a borrowed owner static or promise that every operation
future is `Send`.

## Native L012 scope and shared interrupts

L012 adds one family, three family/controller pairs, two exact parts and six
selected part/controller routes, bringing the declared DMA cohort to nine
families and sixteen pairs. Both exact parts have 64 KiB Flash and 8 KiB SRAM
at `[0x20000000,0x20002000)`. Generic `cw32l012` exposes the compile surface but
retains `MissingMemoryMetadata` safe staging and clean-init admission rejection.
Compiling that alias does not execute or demonstrate the rejection.

| Controller | RX/TX selectors | Fixed DR address | SCK/MOSI/MISO | AFs | Separate example CS |
| --- | --- | --- | --- | --- | --- |
| SPI1 | 6/7 | `0x4000081c` | PA5/PA7/PA6 | 3/3/3 | PB0 |
| SPI2 | 8/9 | `0x4000041c` | PA2/PA1/PA0 | 8/8/8 | PB1 |
| SPI3 | 10/11 | `0x4000341c` | PA9/PA11/PA10 | 8/8/8 | PB2 |

Native DMA has four physical channels: CH1/2 share DMACH12 IRQ9 and CH3/4
share DMACH34 IRQ10; IRQ11 remains CORDIC. At most two full-duplex DMA owners
can coexist. A third controller may use borrowed PIO. Every owner retains its
pair; dropping one does not admit its channels for another owner. The
[native example deployments](../examples/l012/spi-dma/README.md) are separate
firmware images, preserve all constructed bus/CS owners and exercise both
mixed PIO/DMA peers and two simultaneous reserved DMA pairs.

SPI1 uses `InterruptHandler<SPI1>` on IRQ25. SPI2/SPI3 require the complete
`SharedInterruptHandler<interrupt::typelevel::SPI23>` on IRQ26 through
`T::InterruptHandler` for both PIO and DMA construction. A single-member
SPI23 handler does not satisfy this bound. The shared handler visits PIO2,
DMA2, PIO3 and DMA3, with each backend's active guard checked before MMIO.
PIO2+PIO3, DMA2+PIO3, PIO2+DMA3 and DMA2+DMA3 are supported owner combinations.
An inactive, unconstructed, blocking or dropped peer is skipped even when its
bank is gated. No owner teardown disables or unpends SPI23 or either shared
DMA vector, changes a sibling bank or acknowledges its flags. An externally
owned inactive peer must keep its own interrupt sources quiescent or handled.

Native request operations modify only CR2 DMARX/DMATX and preserve sampled EN
and unrelated bits; classic requests remain in CR1. This is an ordinary RMW,
not an atomicity guarantee against MODF autonomously clearing EN. Native DMA
uses TYPE1, BLOCK/TRANS1, SIZE0 and REPEAT1 reloaded for every chunk, with
RESTART/SRCLOAD/DSTLOAD false. SIZE selects byte SRAM elements; the fixed typed
DR address carried by a private pointer does not permit CPU byte MMIO or claim
byte peripheral accesses. CPU accesses remain typed. Native DR reads clear
RXNE and writes clear TXE. Sticky MODF/OV/SSERR/UD is observed without a live
DR read, ICR clear, FLUSH or disable; MODF may stop progress and quarantine.

The existing 24 MHz maximum and even divisors 2..256 are unchanged. Existing
whole-bank GPIO handover, package bonding and MISO pull-down rejection apply.
The [native source ledger](l012-spi-dma-evidence.json) binds L012's own normative
sources. No native SDK SPI DMA cookbook was identified; RX-first arm/request
order is a conservative engineering choice, not a quoted native master recipe.
L012 UART stays TX-only. Its separate RX/combined RC/ORE proof remains blocked.

R031 whole-GPIOA handover and W031 GPIOB/SPI1 ownership restrictions remain.
W031 external SPI is unavailable while internal RF uses SPI1. DMA and PA routing
provide no RF arbitration, coexistence, continuity or RF access.

## Chip select and cancellation

SPI owns SCK/MOSI/MISO, not device CS. Software NSS stays high and no physical
NSS pin is configured. Every successful awaited operation reaches wire idle.
Cancelling or forgetting an operation leaves its active chunk running in
private storage; the unstarted remainder is discarded. Keep the original
device selected and successfully await `bus.flush()` before deselecting,
selecting another device, releasing a device transaction, or changing mode or
frequency. The later flush discards that cancelled chunk's received bytes.

A generic `SpiDevice` adapter that drops CS/releases the lock on cancellation
can assert another device's CS before the next bus call has drained the old
chunk. Thus ordinary `SpiBus` compatibility does not establish cancellation-safe
device composition. No such adapter is implemented or certified here. On an
error the bus stays poisoned, and flush cannot prove that changing CS is safe.
Request-disable and SPI EN=0 are not adopted as an acknowledged early wire-stop
or outstanding-memory-access drain.

## Ownership and state

Both staging slices are validated independently: 1..=65535 bytes wholly inside
the generated `DMA_COPY_SRAM`. Their smaller length is the chunk capacity; safe
exclusive slice ownership establishes nonoverlap. RX ownership is retained as
a private pointer/capacity plus ownership marker after construction, avoiding
any destination reference while DMA may write. Only successful paired
retirement permits a temporary CPU read slice and caller-buffer copy. The
private unsafe `Channel::start_read` accepts a pointer/count for the same reason.

Channels use the existing one-time `Channel::new_admitted_pair` helper. Both
channel admission states are checked in one critical section before either is
claimed; rejection of either channel leaves both admission states unchanged.
This is pair-admission atomicity, not whole-constructor rollback: errors still
consume inputs, and a later SPI configuration failure does not undo successful
pair admission. No raw-channel upgrade, new request table, RCC layer or register
adapter is added. The existing generated
request routes and SPI GLOBAL interrupt facts supply selectors and bindings.
Normal reset/clean-runtime HAL initialization is required; dirty startup is
rejected, and arbitrary active bootloader handovers are unsupported.

The persistent state belongs to the driver, not an operation future:

- Idle permits staging access, guarded PIO and configuration.
- InFlight reserves both channels, both buffers, SPI configuration, pins and
  clocks before either DMA channel is enabled.
- Finishing means both channels reported clean TC plus Complete STATUS and
  EN/TCIE/TEIE read back clear, with DMARX/DMATX read back closed, but
  TXE=1/BUSY=0 still has to be observed.
- Quarantined never returns to Idle. SPI/DMA errors or unexpected launch failure
  retain the whole pair; one successful channel cannot be independently reused.

RX and TX use equal nonzero byte counts, fixed typed DR endpoints, incrementing
memory, REPEAT=1 and hardware BLOCK mode. RX is armed before TX with request
gates closed. DMARX is opened before DMATX after both descriptors are ready.
That RX-ready-first choice is a conservative inference. The classic manual's
example uses TX-channel, RX-channel, TX-request, RX-request order instead;
L012's native normative facts do not supply an SDK paired-master cookbook.

Both channels are polled even when either is pending, so both register owned
wakers and either DMA error is detected promptly. SPI error polling and the
SPI ISR check MODF/OV/SSERR/UD; the ISR latches the first error, masks owned
interrupt sources and wakes without DR access, flag clear, reset or disable.
Its active-clock guard and all state/IER operations share a critical section.
No ISR stores an owner/future/caller-buffer pointer. Sticky software and hardware
errors are checked after waker registration, including before success. The
final observation masks notification before rereading hardware/software status
in the same critical section. If wire idle is still pending, prior error
enables are restored; sticky flags raised in that interval can still wake it.

After both clean DMA results, requests close and Finishing persists through
wire completion. Failed channel or request-gate readback permanently quarantines
both channels and buffers. Request closure is also checked before launch, so a
failed preparation cannot arm either channel. BUSY has no interrupt: this tail cooperatively self-wakes
while BUSY is set or TXE is clear. Once no errors and wire idle are observed,
notification is masked, both DMA retirements fence the hardware results, and
Idle becomes visible. A transient Quarantined state protects sequential
retirement against panic. Only then can the current receive chunk be copied,
with no await between successful settlement and CPU copy.

Every public blocking method, including empty calls and both blocking-trait
transfer forms, makes a single nonblocking paired-settlement observation before
PIO. Pending reports DmaBusy; quarantine reports DmaQuarantined. Configuration
similarly returns Busy/quarantine before changing registers or pins. PIO's
existing destructive error recovery and word-width/RX drain run only after
this guard proves Idle. Before staged launch, eight-bit width is restored in
case a guarded blocking operation selected u16 or another supported word width.

Drop makes one observation, never a hidden unbounded wait. An unresolved or
poisoned pair masks SPI IER, deactivates the static SPI ISR guard, and forgets
both DMA/storage owners and all three AF pins, returning before SPI disable or
RCC gating. Static DMA handlers may still record/mask their own terminals.
Forgetting the driver also retains all static resources. Clean Drop follows
the original disable/gate/pin-disconnect path without restoring channel
admission. A PIO owner's Drop is destructive teardown without
DMA retention, and PIO errors use their existing destructive recovery; they do
not enter DMA quarantine. No public free/split/raw conversion, buffer recovery
or DMA-to-PIO conversion is offered.

## Data behavior and costs

`transfer` clocks max(read length, write length) bytes. TX is zero-padded;
excess RX is discarded. `read` transmits zeroes; `write` uses RX DMA as a private
sink. In-place transfer preserves each original chunk in TX staging before
launch and replaces the caller chunk only after successful settlement. Empty
methods still reap an outstanding pair, and all async trait methods forward
to these same inherent methods. Earlier completed chunks remain visible if a
later chunk fails or is cancelled; whole-buffer atomicity is not promised.

Costs include two SRAM buffers, two channel reservations, a TX CPU copy/fill,
and an RX CPU copy when requested. Both intra-chunk arbitration and arbitrary
scheduler gaps between wire-idle chunks affect device compatibility. A higher
priority RX channel helps contention but proves no sustained throughput;
excessive SCK or competing DMA traffic can cause overrun. Clock bounds do not
prove lossless reception. No bounded completion time or safe early abort exists.

## Primary evidence

Stage94's [native source ledger](l012-spi-dma-evidence.json) records own-source
facts separately from runtime and compiler/linker acceptance. Its main matrix
is six standalone libraries plus fifteen ELF links (21 accepted matrix rows),
with zero canonical generation. The final-source clean matrix is two native
libraries plus six native ELF links (eight Cargo invocations), preceded by one
existing generation bootstrap; all 910 canonical generated files must match
main by path and bytes. Dependent HAL archives do not add library commands.
Stage94 main and five finite commands passed, with independent runtime/source
acceptance as described above. At main acceptance the separate final-source
clean replay had not run; its own receipt remains required. These checks cannot
execute hardware cancellation, error drain, IRQ races, generic rejection, data
integrity or throughput; runtime/source review and hardware evidence remain
separate. Historical Stage86/89/93 proofs are not rewritten or reused as new
acceptance.

The [five-family current evidence ledger](classic-spi-dma-evidence.json) binds
the selected own manuals, SVD/header data and package datasheets for the
Stage93 extension. Each own manual supplies paired byte master DMA, DR-read
RXNE clearing, native request selectors and the terminal/request-close/BUSY-idle
sequence. Shared register layouts and historical UART DMA acceptance are not
substitutes for those SPI facts. Stage93 has its separate bounded main software
acceptance described above; its final-source clean replay had not run at main
acceptance. The following x030/L083 source history retains its original scope.

L083 SPI1/SPI2 use their own CN V2.0 manual paired byte-DMA sequence and error
semantics, as recorded in [L083 qualification](l083-peripheral-dma.md). The
existing L083 12 MHz maximum SCK and minimum divisor 4 remain unchanged. The
x030 sources below retain their original scope.

[CW32x030 User Manual CN V2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
SHA256 `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`.
Printed page numbers equal zero-based PDF indices (viewer page is one higher).

- §8.4.1 p127: descriptors before channel EN; no descriptor reset as abort.
- §8.4.4 pp130–132: request pacing/routes and hardware block triggering.
- §8.5 p134: matching byte widths, 1..65535 count and REPEAT reload.
- §8.6 p135 and §8.8.3 p139: TC means all data correctly transferred; Complete
  STATUS is 101. TE/STOP/EN-clear alone is not adopted as memory quiescence.
- §19.3.8 p375: RXNE data availability, BUSY covers queued/shift data, wait for
  zero before CS high.
- §19.3.9 p376: MODF disables SPI; overrun overwrites old RX data. Those errors
  can prevent the paired DMA counts completing, hence a separate error wake.
- §19.4 p377, §19.8.4 p391: interrupt sources; no BUSY-clear interrupt.
- §19.5 p378: independent TXE/RXNE DMA requests.
- §19.6.1.3 pp381–382, steps 5/12–13/16–44: eight-bit paired descriptors, both
  terminals, close both requests, wait BUSY zero, then SSI high.
- §19.8.1–7 pp388–393: SSM/SSI/EN, request gates, status, R1W0 ICR/FLUSH and DR.

[Official F030 SDK V2.2](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip),
SHA256 `7c431df43d7075b817a51d818ea4c9aba55f83b7c9c77bf0065976758954b780`.
`Libraries/src/cw32f030_spi.c:51–68,326–338` corroborates typed DR and request
bit operations; those operations provide no drain handshake. The unchanged
`Examples/SPI/SPI_DMA/USER/src/main.c:237–263` corroborates paired byte/fixed-DR/
incrementing-memory descriptors. That SDK example is polling master with DMA
slave; the manual's paired-master sequence is the master evidence here.

Authored `cw32-data/registers/spi_v1.yaml`, `cw32-data/dma/`, package metadata and
existing build hooks produce typed PAC endpoints, DMA routes and GLOBAL IRQs.
Generated PAC/chip outputs are build evidence, excluded from source archives.
The initial x030 implementation/source identities and source locators are recorded in
`docs/spi-dma-evidence.json`; external run receipts separately identify the
frozen build inputs and actual firmware ELFs. The real example is
[examples/shared/spi-dma](../examples/shared/spi-dma/README.md). No firmware was executed.
