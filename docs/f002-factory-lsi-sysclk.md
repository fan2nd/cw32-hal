# Init-only factory LSI SYSCLK on CW32F002F3P7/F3U7

For F002, `rcc::Sysclk::LSI` is available only on CW32F002F3P7 (TSSOP20)
and CW32F002F3U7 (QFN20), through their qualified metadata. The generic
CW32F002 alias and all unlisted F002 parts remain excluded. F003's three
exact packages have a [separate contract](f003-factory-lsi-sysclk.md) and
independent qualification; sharing this backend does not grant capability.
HSI remains the default. F002 has no RTC or dedicated LSI_OUT pad: this addition
provides no `LsiClock`, `CalendarClock` or RTC API. The existing
[classic factory-LSI contract](factory-lsi-sysclk.md) remains separate.

Set `config.rcc.sys = rcc::Sysclk::LSI` before one-time `init(config)`.
Existing `operating_conditions`, `hsi`, `hex`, AHB/APB prescalers and nonzero
`timeout` remain the complete configuration. There is no public arbitrary LSI
frequency, user trim, WAIT adjustment, runtime switching, sleep/wake restoration
or clock-loss recovery. No hardware validation is claimed.

## Rates and board conditions

The own factory qualification is nominal 32,800 Hz, bounded by 31,160–34,440 Hz
over VDD 1.65–5.5 V and ambient −40–105°C. This is the full-range factory row;
the RM's ±10% adjustment range and the datasheet's 25°C accuracy row are
separate facts. Board declarations must cover actual supply tolerance and
ambient conditions. They are not measured or established by initialization.

Default HSIOSC /6 remains nominal 8 MHz, and AHB/APB defaults remain /1. Factory
HSI stays enabled on success and retains its independent bounds. Independent
`hex: Some(...)` configuration and retained AWT input ownership keep their
existing contracts; `hex: None` preserves an inherited HEX source.

LSI SYSCLK/HCLK/PCLK bounds are rate-only: `has_cycle_timing_bounds()` is false.
Exact source numerators and cumulative divisors are retained; public whole-Hz
endpoints round outward. For AHB /128 and APB /8, displayed PCLK is nominal
32 Hz, minimum 30 Hz, maximum 34 Hz, with exact divisor 1024 still used for
comparisons. No absolute per-cycle or jitter guarantee is inferred.

The existing ADC timing guard rejects this rate-only clock before peripheral
startup. The fixed 1 MHz Embassy time driver cannot divide this source at any
supported AHB/APB setting and rejects it before singleton acquisition or RCC
MMIO. F002 uses the `time-driver-gtim` feature; the demonstration firmware enables
no time driver. AWT retains its independent HSIOSC timing qualification. Other
peripheral constructors keep their own rate and representability limits; a low
SYSCLK does not establish that every peripheral or baud rate works.

## Functional handover and retained consumers

The existing platform entry model still applies: bus masters must already be
quiescent before Rust uses application memory, and no debugger, DMA or exception
handler may race HAL-owned register writes. A critical section does not freeze
peripheral state machines or external edges. Incoming executing clocks must be
continuous and electrically legal.

Selecting LSI permits bounded inspection windows that run the entire GPIOA,
GPIOB and GPIOC banks, even for unbonded pins and even if initialization later
fails. Input sampling, filters and armed events can advance. GPIO configuration
and locks are preserved; software does not clear flags, but flags may change
naturally. Restoring a gate cannot undo this progress. This is a functional
handover condition of the mode, not an additional unsafe constructor or risk
confirmation flag. Bus guards and the system-clock change also affect existing
HCLK/PCLK consumers and direct outputs.

Cold admission checks six gated consumers: AWT, UART1/2 and GPIOA/B/C. AWT/UART
gates provide configuration access; GPIO gates also enable bank operation. MCO
is inspected without a gate. Each window verifies reset and readability before
and after selector reads, then restores the original gate with bounded readback.
Enable failure still attempts restoration; a restoration failure is reported
rather than hidden by a consumer rejection. No reset, FIFO/data access,
consumer-selector write or interrupt acknowledgement creates admission.

Cold-source allowed selector sets are AWT SRC [0,2,3,4], UART1/2 SOURCE [0,1],
GPIOA/B/C FLTCLK [0,1,2,3,4,6] and MCO SOURCE [0,1,2,3,5,8,9]. Reserved values
are rejected. Disabled function enables, masked interrupts, unused pads and
closed gates do not prove a retained selector harmless. There is no invented
RTC, UART3, GPIOF or PB11 direct-output check. IWDT uses independent RC10K.

## Admission and source ordering

An already-requested or selected factory-matching source with both stable
indications set is reused without TRIM/WAIT writes. A requested, unselected
matching source may finish startup within the poll budget. A selected source
must already be stable. Contradictory dual status, unowned stable state, or a
live/in-flight nonmatching trim is rejected; no stop/retrim manufactures reuse.
The initial classification is preserved through the transition.

A genuinely stopped source needs the complete cold proof even if TRIM already
matches factory. HSI or HEX must be the legal selected source, LSIEN and both
LSI stability indications must be clear, and IER.LSIRDY, ISR.LSIRDY and NVIC RCC
IRQ4 pending must be clear. A legal continuous HEX entry is permitted; F002 has
no classic CCS controls to write or reserved CCS fields to borrow.

Two full consumer/observer passes must agree before a possible TRIM-only write.
The aligned factory halfword is read at 0x001007BA. Raw 0xFFFF is conservatively
rejected as “factory halfword reads all ones”; it is not claimed to prove erased
or defective silicon. Zero is valid. Only the low ten bits become TRIM; high
factory bits are not an additional rejection condition. All four inherited WAIT
encodings and other LSI bits are preserved. Readback and a third complete
use-edge pass use the original snapshot advanced only to the expected TRIM.

Conservative Flash/bus guards precede source changes. Cold preparation finishes
before the first CR1 source write, which requests HSI and permanently requests
LSI together. LSI request and both stable indications are checked before source
handover. Subsequent HSI calibration and independent HEX processing retain the
permanent LSI request and all existing AWT/LVD ownership checks. Necessary HSI
calibration can temporarily execute on this qualified LSI bridge.

After a successful independent HEX pad configuration, only the expected GPIOB
gate is advanced to enabled to account for that existing owned operation;
retained selectors, other gates and reset checks are not resampled or relaxed.

LSI is selected and read back before final bus divisors are relaxed. Final Flash
WAIT covers both target LSI and retained factory HSI at those divisors; low LSI
rate alone does not imply WAIT0. Source, parameters, requests, dividers, retained
HSI/HEX ownership and Flash are verified before frozen clocks are published.
No ready flag is cleared to obtain admission or after a successful start. A new
LSI start may naturally set LSIRDY and leave it set.

## Failure boundaries

Pure configuration/time-driver rejection occurs before taking singletons and
writes no hardware. Later errors can consume singletons and leave conservative
Flash/bus guards or GPIO event progress. A trim/readback/use-edge failure can
leave attempted TRIM even before source request. Gate restoration failure can
leave a gate enabled.

After the enable attempt, no cleanup stops or retrims a possibly started LSI;
the permanent request can remain active and become ready after timeout. HSI
calibration failure can leave execution on LSI with HSI stopped or incompletely
restarted. HEX can be partially configured. No hardware error promises rollback
or a ready HSI. RCC errors publish no clocks and return no peripheral tokens;
reset before retrying hardware initialization.

Timeouts count polls, not microseconds; LSI execution makes them slower. STABLE
is a latched startup observation and does not detect running clock loss. If the
executing clock stops, software may be unable to return an error.

## Metadata compatibility, sources and verification

`lsi_output_pin` changes from `String` to `Option<String>` in the owned data
models and from `&'static str` to `Option<&'static str>` in static metadata.
F002 authors explicit null with an empty AF set and no RTC source set; existing
classic parts retain their real output pin and complete RTC/consumer checks.
Rust code constructing or consuming these metadata fields must handle the
option. Authored policy validation, rather than Serde alone, enforces explicit
presence and the family-specific absence rules. The generated F002 source
roster has seven selectors and six gates; classic retains eleven selectors and
nine gates.

Own-source bindings are in `cw32-data/lsi-sysclk-qualified.yaml`. F002 RM Rev1.4
PDF pp44–60 (printed pp43–59) covers system/LSI fields and status; DS Rev1.2
PDF p37 (printed p36), table 7-14 gives the factory LSI rate. The RM SHA-256 is
`e453b3f82d9d180a86cd5f7cb18d858d7f228afc8101d87c700ca9d5c02d5add`; the DS
SHA-256 is `6d0c5c37d069e5b4e33d394b0be6c53938868e9375e7b4bbbbdd40d62bb9d506`.
The SDK `cw32f002_rcc.h:133` and `cw32f002_rcc.c:301–306` corroborate the factory
halfword; the SDK's whole-register/WAIT assignment is not copied.

The [ordinary example](../examples/lsi-clock/README.md) uses the exact packages'
16 KiB Flash and 2 KiB SRAM from own DS tables 3-1 and 6-1, release LTO and an
explicit `-Tlink.x`. Generic aliases are not firmware targets. Ordinary
compilation/linking, generated-data assertions and static source review do not
establish startup, electrical rate, runtime failure handling or any other
hardware behavior. No HAL tests, probes, simulator/model or harness are added.
