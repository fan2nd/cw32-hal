#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
//! Experimental CW32 HAL using Embassy singleton and type-level interrupt APIs.
//!
//! Select exactly one supported chip feature. The coverage manifest distinguishes
//! register descriptions from implemented drivers. Initialization configures the
//! verified selected clock path; asynchronous drivers require typed IRQ bindings.

#[cfg(adc)]
pub mod adc;
#[cfg(aes_cw32l083_v1)]
pub mod aes;
#[cfg(autotrim)]
pub mod autotrim;
#[cfg(awt)]
pub mod awt;
#[cfg(vc)]
pub mod comparator;
#[cfg(cordic_cw32l012_v1)]
pub mod cordic;
#[cfg(crc)]
pub mod crc;
#[cfg(dac_cw32l012_v1)]
pub mod dac;
#[cfg(any(dma_v1, dma_cw32f020_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
pub mod dma;
#[cfg(eau_cw32l012_v1)]
pub mod eau;
#[cfg(gpio_exti)]
pub mod exti;
#[cfg(flash)]
pub mod flash;
pub mod gpio;
#[cfg(halltim_cw32l012_v1)]
pub mod halltim;
#[cfg(i2c)]
pub mod i2c;
pub mod ir;
#[cfg(lcd)]
pub mod lcd;
#[cfg(lptim)]
pub mod lptim;
pub mod lvd;
mod macros;
#[cfg(opa_cw32l012_v1)]
pub mod opamp;
pub mod rcc;
#[cfg(rtc)]
pub mod rtc;
#[cfg(spi)]
pub mod spi;
pub mod time;
#[cfg(feature = "_time-driver")]
pub mod time_driver;
#[cfg(trng_cw32l083_v1)]
pub mod trng;
#[cfg(aes_cw32l083_v1)]
mod crypto_geometry {
    include!(concat!(env!("OUT_DIR"), "/_crypto.rs"));
}
#[cfg(any(gtim_classic, btim))]
pub mod timer;
#[cfg(uart)]
pub mod usart;
#[cfg(vref)]
pub mod vref;
#[cfg(iwdt)]
pub mod wdg;
/// Raw register access, outside the HAL ownership boundary.
///
/// Like the pinned chiptool PAC, register operations use safe Rust syntax but
/// are technically unsafe hardware integration: they can violate memory safety
/// and HAL ownership. In particular, never change a HAL-owned DMA channel,
/// flags, trigger, shared reset or clock, including after a forgotten/error copy.
/// Use ordinary HAL drivers for safe resource operations. Direct PAC takeover
/// must establish quiescence and preserve all other owners independently.
pub use cw32_metapac as pac;
pub use embassy_hal_internal::{Peri, PeripheralType};
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

/// HAL initialization options, following Embassy's clock configuration boundary.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct Config {
    /// Verified system and bus clock configuration for the selected backend.
    pub rcc: rcc::Config,
}

/// Initialize clocks and acquire all peripheral singletons exactly once.
///
/// Supported firmware starts after hardware reset, or a low-level runtime/boot
/// handover that has already left all bus masters quiescent before Rust starts
/// using application memory: no outstanding accesses, armed/gated transfers or
/// pending requests that can resume when clocks are enabled. Initialization is
/// not an active-DMA bootloader
/// recovery routine: inherited accesses could already corrupt memory before it
/// runs. Runtime integration and direct PAC access must preserve HAL ownership.
/// This is the platform entry model, not a per-driver unsafe caller obligation.
///
/// On CW32L010F8P6/F8U6/Y8M6 only, selecting factory LSI SYSCLK reuses an
/// already software-requested, stable, factory-matching source observed at
/// original entry. Ordinary POR, stopped startup and automatic-only LSI requests
/// are refused. LSI TRIM/WAIT and the continuous software request are preserved;
/// no IWDT register, gate, reset or feed operation establishes admission.
/// `Config.lse=Some(_)` is refused for this target before hardware changes.
/// Inherited LSE and existing RTC/HSI/HSE/analog ownership remain protected.
///
/// Running IWDT, ready/fault observers, LSI-selected timers, UARTs, filters and
/// downstream or external recipients may continue progressing, including toward
/// reset, before an error and afterward. Necessary existing GPIO inspection
/// windows run the whole bank and may advance unrelated sampling/events;
/// restoring a gate cannot undo that progress. No dormant work-bank scan proves
/// idleness. The functional handover must permit these effects. This adds no
/// hidden memory-safety duty to safe callers and does not replace the pre-Rust
/// bus-master/ownership boundary above.
///
/// Final bus dividers and Flash latency are established under verified factory
/// HSI before the last source write selects LSI. Errors publish no new clocks;
/// partial hardware state can remain and requires reset before retry. A lost
/// CPU source can prevent an error return. LSI system/bus bounds are rate-only;
/// native ADC and buffered complementary PWM reject their strict timing use,
/// and the fixed 1 MHz time driver refuses before singleton acquisition/MMIO.
/// The existing HsiOscClock/RTC source keeps independent raw-HSI cycle bounds.
/// No normal-reset boot, watchdog survival, oscillator continuity or hardware
/// validation is claimed. See docs/l010-live-factory-lsi-sysclk.md.
///
/// On CW32L011K8T6/K8U6 only (64 KiB Flash / 6 KiB SRAM), factory LSI SYSCLK
/// requires original software LSIEN, STABLE and own ten-bit factory equality.
/// Raw factory 0xffff is erased; zero is valid. Config.hse and Config.lse must
/// both be None; original HSEEN/HSECCS/LSECCS must be zero, with HSI or LSI
/// entry. Inherited unmonitored LSE remains protected. Ordinary POR, stopped
/// startup and automatic-only requests fail; no LSI parameter/request or IWDT
/// operation obtains admission. Own 29,520..41,000 Hz bounds at 1.7..5.5 V,
/// VDDA=VDD and -40..85 C are rate-only. The DS -10/+25% factory envelope does
/// not withdraw the RM +/-10% calibration-adjustment warning or qualify cycles.
///
/// Actual RTC/ADC/VC configuration windows check gate/reset access and restore
/// borrowed gates before buffered semantic refusals. Gate enable/restore errors
/// retain both diagnostics. RTC SOURCE1 refuses even with START/AWT disabled;
/// SOURCE3 requires original factory-ready raw HSI and actual PSC1 raw >=97
/// (divisor >=98, widening before +1). SOURCE4..7 refuse. No RTC migration or
/// profile-divisor assumption establishes admission; native RTC stays raw
/// 96 MHz HSIOSC. Necessary windows may advance work before refusal, and
/// restoration cannot undo it. No HSE GPIO or dormant UART/LPTIM/GPIO/IWDT
/// inspection is added. PB0 AF3 raw-HSI and downstream observers must permit
/// HSI-preparation effects. These functional limits add no hidden memory-safety
/// obligation to the existing platform entry boundary.
///
/// FLASH stays enabled; its actual gate/reset and preserved non-WAIT state are
/// checked around each access, with coherent WAIT/mirror acknowledgments.
/// Final buses and latency are established under factory HSI, covering its
/// requested rate and full 4.08 MHz fallback without divider credit. Final LSI
/// selection is followed only by read-only checks before clock publication.
/// Hardware refusals occur after singleton acquisition, publish no RCC clocks,
/// may leave partial state and require reset before retry even without a clock
/// write. Pure configuration/time-driver errors precede acquisition and MMIO.
/// No continuity, watchdog survival, recovery or hardware validation is claimed.
/// See docs/l011-live-factory-lsi-sysclk.md for scope and verification status.
///
/// On CW32L012C8T6/C8U6 only, factory LSI SYSCLK likewise requires original
/// software LSIEN, STABLE and own nine-bit factory-TRIM equality. Ordinary
/// POR, stopped startup and automatic-only requests are refused; no later
/// observation upgrades admission. Raw erased calibration is rejected before
/// masking; zero is valid. No LSI parameter register, explicit LSIEN setter,
/// IWDT register, gate, reset, command, feed or pause operation prepares entry.
/// `Config.lse=Some(_)` is refused in pure validation for this target only.
/// Inherited LSE, HSE ownership and all previous target contracts are retained.
/// Own rate-only LSI bounds are 29,520..36,080 Hz at 1.7..5.5 V and -40..85 C;
/// board declarations require actual qualification and are not measurements.
///
/// Native L012 ADC1/ADC2 share a configuration-and-work gate: inspecting EN
/// can resume conversions or triggers before refusal. Required HSE GPIOF
/// windows affect the whole bank, including PF3 LSI_OUT, filters and events.
/// Both PB11 AF4 and PF3 AF2 outputs, PB0 raw-HSI output, native timer/cascade
/// alternatives and downstream/external recipients need functional handover
/// that permits progression or disconnects them. Closed UART3/timer/output-bank
/// gates are not opened merely to prove idleness. Restoring borrowed gates
/// cannot undo sampling, pin pulses, IRQs, capture/brake flags or IWDT progress.
/// The RCC automatic-LSI-request clause and the IWDT counter's separate RC10K
/// source both remain applicable. These are functional limits within the
/// existing pre-Rust bus-master/memory-ownership boundary above, not a new
/// hidden memory-safety duty of safe callers.
///
/// Exact L012 preserves independent HSI owners, actual retained RTC PSC1 and
/// both Flash latency interfaces. Final bus divisors and Flash WAIT are set
/// under factory HSI before the last source/divider/Flash mutation selects LSI.
/// Borrowed GPIO gate restoration and complete read-only checks precede clock
/// publication. RCC errors publish no new clocks and can leave partial state;
/// reset is required before retry. Poll limits count CPU iterations, and source
/// loss can prevent an error return or invalidate frozen timing. Native ADC
/// and buffered complementary PWM reject rate-only cycle timing; the fixed
/// 1 MHz time driver rejects selected LSI before singleton acquisition/MMIO.
/// `HsiOscClock` and native RTC retain raw 96 MHz HSIOSC with 94.08..97.92 MHz
/// strict bounds and existing calendar source variants. No native LSI calendar,
/// normal-POR success, recovery or hardware validation is claimed. See
/// docs/l012-live-factory-lsi-sysclk.md.
///
/// On CW32F002F3P7/F3U7 only, selecting factory LSI SYSCLK can briefly run
/// each whole GPIOA/B/C bank to inspect retained selectors. Sampling, filters
/// and armed events may advance before an error; restored gates cannot undo
/// progress. Configuration and locks are preserved. Software does not clear
/// flags, including LSIRDY, but flags may change naturally. Cold admission
/// also checks AWT, UART1/2, MCO and RCC ready/NVIC observers; F002 has no RTC.
/// A failed transition may leave attempted TRIM, an enabled gate, conservative
/// Flash/bus guards or a permanent LSI request. HSI-calibration failure may
/// leave execution on LSI with HSI stopped or incompletely restarted. Reset
/// before retrying hardware initialization. ADC rejects LSI rate-only timing;
/// the fixed 1 MHz time driver rejects selected LSI before singleton acquisition.
/// This functional handover adds no hardware-validation or recovery guarantee.
/// See docs/f002-factory-lsi-sysclk.md for the complete exact-package contract.
///
/// On CW32F003F4P7/F4U7/E4P7 only, factory LSI SYSCLK has the same
/// whole-GPIOA/B/C functional handover and partial-state failure boundary,
/// using its own 31,816..33,784 Hz factory rate qualification. Cold admission
/// also checks AWT, UART1/2, MCO and RCC ready/NVIC observers; there is no
/// RTC API. ATIM/IR dependencies require no additional inspection gates;
/// unrelated gate/reset bits, including ATIM, are preserved. ADC rejects
/// LSI rate-only timing; the fixed 1 MHz time driver rejects selected LSI
/// before singleton acquisition or RCC MMIO. AWT keeps its independent
/// HSIOSC timing. Generic F003 is excluded. Reset before retrying hardware
/// initialization; no rollback, recovery or hardware validation is promised.
/// See docs/f003-factory-lsi-sysclk.md for the complete exact-package contract.
///
/// On CW32L031C8T6/C8U6/F8U6 only, selecting factory LSI SYSCLK may run
/// each whole GPIOA/B/C/F bank. Sampling, filters and armed events may advance
/// even before an error; restoring gates cannot undo that progress. Cold
/// admission checks RTC, AWT, UART1/2/3, all four FILTER selectors, MCO, PB11 AF
/// and ready/NVIC observers even when factory TRIM already matches. PB11 is
/// unbonded on F8U6; its register check is conservative software policy.
/// Documented AWT-overflow FILTER7 is also conservatively refused. A live,
/// factory-matching LSI is retained without TRIM/WAIT writes. Configurable
/// CCS/LSELOCK and inherited HSE/LSE ownership remain preserved except the
/// existing fresh-LSE enable/monitor addition. Source loss or concurrent
/// clock/pad/consumer changes are outside this bounded functional handover.
/// Failure can leave partial clock, pad, gate or calibration state and publishes
/// no RCC clocks; reset before retry. This adds no memory-safety precondition,
/// rollback or recovery promise. RTC LSI bounds become rate-only under every
/// SYSCLK on these three parts; ADC and the fixed 1 MHz time driver reject
/// selected LSI. See docs/l031-factory-lsi-sysclk.md for the complete contract.
///
/// On CW32R031C8U6 only, factory LSI SYSCLK reuses that native sequence with
/// own 31,816..33,784 Hz bounds at 2.2..3.6 V and -40..85 C. Whole GPIOA
/// inspection can affect PA00..PA03 RF host activity through PCLK despite the
/// RFCLK root's independent dedicated 16 MHz oscillator. Finish/quiet host
/// transfers and permit the complete inspection interval. Keep inherited RF
/// XTAL_OCLK available if it supplies HSE bypass. Gate restoration or failure
/// does not guarantee unchanged RF signals, packets or source continuity.
/// These are existing functional handover limits, not hidden Rust memory-safety
/// preconditions. Initialization neither reads nor writes RF state. Exact-part
/// RTC LSI aliases become rate-only under every SYSCLK; generic R031 remains
/// excluded from LSI SYSCLK. The fixed 1 MHz time driver refuses
/// selected LSI before singleton acquisition and any RCC MMIO. Reset before
/// retrying a hardware failure. See docs/qualified-r031-lsi-sysclk.md.
///
/// On CW32L052C8T6/R8S6/R8T6 only, factory LSI SYSCLK uses own 31,816..33,784 Hz
/// rate bounds at 1.65..5.5 V, VDDA=VDD and -40..85 C. Admission may temporarily
/// run each whole GPIOA/B/C/D/F bank, including unrelated pins. Sampling,
/// filters and armed edge/level/event paths can advance before failure;
/// restoring gates cannot undo that activity. The functional handover must
/// permit the whole-bank interval and external pin loopbacks. Other application
/// clock-dependent clients must remain quiescent during initialization.
/// Off LCD/LPTIM work gates stay off and their local registers are not read.
/// Cold admission includes RTC, AUTOTRIM, all three UARTs, both work gates,
/// GPIO FILTER words, MCO, PC4 AF and ready/NVIC observers even for matching
/// cold TRIM. PC4/AF6 is conservatively checked on unbonded C8T6 without a pin
/// token. Live factory-matching LSI is neither stopped nor retrimmed. Full
/// consumer and source identity is retained; no IRQ/event clear, timer stop,
/// LCD pump manipulation or peripheral reset manufactures admission.
/// The incoming source/buses/Flash must already be legal for actual voltage.
/// Configured factory HSI must also be legal with final bus divisors. Failure
/// publishes no clocks and can leave partial source, Flash, pad or gate state;
/// reset, or power reset for POR-retained LSE controls, before another attempt.
/// Poll budgets are not elapsed deadlines and clock loss may prevent return.
/// All exact-L052 RTC LSI aliases become rate-only under every SYSCLK, including
/// HSI/HSE/LSE. Strict duration consumers refuse these bounds; ADC timing
/// refuses rate-only PCLK before peripheral gate/reset writes. The fixed 1 MHz
/// time driver refuses selected LSI before singleton acquisition and RCC work.
/// This functional handover adds no hidden Rust memory-safety precondition,
/// rollback, source-loss recovery, sleep/wake or hardware-validation guarantee.
/// See docs/l052-factory-lsi-sysclk.md for the complete contract.
///
/// On CW32L083RBT6/RCT6/RCS6/MCT6/VCT6 only, factory LSI SYSCLK has own
/// 31,816..33,784 Hz rate bounds at 1.65..5.5 V, VDDA=VDD and -40..85 C.
/// Admission can run each entire GPIOA/B/C/D/E/F bank, including unrelated
/// pins, filters, armed edge/level events and external pin feedback before
/// failure. Functional handover permits every such window and quiesces all
/// clock-dependent clients; restoring a gate cannot undo elapsed activity.
/// Full snapshots cover six UARTs, RTC/AUTOTRIM/LVD, tagged LCD/LPTIM, MCO,
/// all bonded LSI/PLL output routes and conservative unbonded AF0 guards.
/// Off LCD/LPTIM work gates remain unread and off. Matching cold trim never
/// skips full ownership/observer proof; live factory-matching LSI is retained
/// without stop/retrim. No consumer reset/stop or IRQ/event clear gains admission.
/// An inherited enabled PLL requires actual-reference bounds and full output
/// ownership. If selected, first use a proven HSI/HSE bridge; direct PLL-to-LSI
/// is forbidden. PLL enable and both ready observations must reach zero before
/// reference-affecting HSI changes. Flash WAIT2 remains on success. Configured
/// factory HSI must fit at final bus divisors. Potential HSE/LSE fallback uses
/// the undivided 48.96 MHz HSI bound without HSI/AHB/APB divider credit, requiring
/// VDD at least 1.8 V; no L052 fixed-/6 escape or 48 MHz limit is imported.
/// Once established, LSI readiness remains invariant after permanent request. Final source-only
/// LSI selection is the last source, divider, Flash, pad or policy write; temporary
/// read-only gate inspection/restoration may follow. The final CR0 observation
/// precedes publication. Failed RCC initialization publishes no clocks/tokens and
/// can leave partial state; reset, or power reset for retained LSE, before retry.
/// Poll budgets are attempts, and loss of an execution clock can prevent return.
/// Exact-L083 RTC LSI aliases become rate-only under HSI/HSE/PLL/LSE/LSI, with
/// unchanged numeric bounds and 32800/32768 nominal calendar fraction. Strict
/// duration assertions and ADC timing refusal retain their existing order;
/// the fixed 1 MHz time driver refuses selected LSI before singleton/RCC work.
/// These functional limits add no hidden Rust memory-safety precondition,
/// rollback, source-loss recovery, sleep/wake or hardware-validation guarantee.
/// See docs/l083-factory-lsi-sysclk.md for the complete source contract.
///
/// On F020/F030/A030, selecting factory LSI SYSCLK (or LSE SYSCLK on the
/// three qualified packages) may briefly open each
/// entire GPIOA/B/C/F bank to inspect retained source selectors. Sampling,
/// filters and armed events can advance, including before a failure. GPIO
/// configuration and flags are preserved; gate restoration cannot undo events.
/// This is an explicit functional handover under the entry model above.
///
/// On the four qualified CW32L031/R031 packages, LSE SYSCLK may also
/// open GPIO banks for source/pad inspection. Sampling, filters and armed
/// events can advance, including before an error. A previously disabled LSI
/// whose factory TRIM already matches is requested without a parameter write
/// or trim-owner proof. This can resume parked AWT SOURCE1, UART SOURCE3,
/// GPIO FLTCLK5, MCO SOURCE4 and bonded PB11 AF1 consumers. The functional
/// handover must permit that progress; restored gates do not undo it. Normal
/// clock-sensitive peripheral/interrupt work remains excluded during init.
/// Factory-mismatching LSI retains its separate two-pass consumer admission.
///
/// On CW32L052C8T6, CW32L052R8S6 and CW32L052R8T6, LSE SYSCLK admission
/// can briefly run each entire GPIOA/B/C/D/F bank. Sampling, filters and armed
/// events can advance even before an error; preserving registers and restoring
/// gates cannot undo that progress. Starting factory-matching stopped LSI
/// leaves TRIM/WAIT unchanged but can resume parked UART1..3 SOURCE3 (native
/// SORCE), permitted manual AUTOTRIM timer SRC1, GPIO FLTCLK5, MCO SOURCE4,
/// and PC4 AF6 on the two R8 packages only. Enabled, already work-ungated
/// LPTIM ICLKSRC3 and LCD CLKCS0 can also resume; closed work gates stay closed.
/// The admitted LSI/LSE combinations do not newly resume RTC SOURCE2: cold
/// LSE requires the full RTC reset record with SOURCE0, and reused LSE already
/// requires a ready monitor. AUTOTRIM admission precedes any factory-match
/// shortcut and rejects automatic/active calibration. The functional handover
/// must permit this progress and initialization's changes to bus/output timing;
/// normal clock-sensitive peripheral/interrupt work remains excluded. This
/// does not change the pre-Rust bus-master/memory-ownership boundary above.
/// Configured HSI must be legal at the final AHB/APB dividers even without HSE
/// or enabled CLKCCS. Separately, fallback electrical coverage uses fixed
/// HSI /6 escape's 8.16 MHz upper bound before any bus-divider credit.
/// Frozen healthy LSE clocks are invalid after source loss/fallback. This LSE
/// path grants no factory-LSI SYSCLK qualification, recovery, continuity or
/// silicon-validation guarantee; the separate LSI contract is documented above.
///
/// On the five qualified CW32L083 RBT6/RCT6/RCS6/MCT6/VCT6 packages,
/// LSE SYSCLK uses a separate init-only transition. The unchanged HSI used to
/// escape an inherited source must already be in its documented legal
/// calibration regime, even when idle at entry; STABLE/DIV do not prove its
/// rate. Factory bounds are applied only after factory trim is established.
/// WAIT2 and ready unchanged HSI precede the first explicit HSI mux write,
/// which also installs monotonic bus guards. An inherited PLL then stops
/// before its post-DIV HSI or HSE reference can change. PLL/MCO/dedicated
/// output recipients must permit the resulting interruption of those outputs.
/// Source/pad inspection can run whole bonded GPIO banks. Starting matching
/// stopped LSI can resume parked UART1..6 SOURCE3, permitted manual AUTOTRIM
/// SRC1, GPIO FLTCLK5, MCO SOURCE4 and bonded PC4 AF6, PF2 AF4 or PD5 AF6
/// outputs, plus already work-ungated LPTIM/LCD consumers. The handover must
/// permit this progress; restored gates do not undo it. Cold-LSE RTC reset
/// admission, existing ready-LSE reuse and closed work-gate preservation stay
/// unchanged. The full stopped-mismatch LSI consumer check occurs after PLL
/// stop; live detector contradictions are rejected before source mutation.
/// Conservative fallback admission gives no HSI/AHB/APB divisor credit:
/// raw factory HSI can reach 48.96 MHz, requiring VDD >= 1.8 V and retained
/// WAIT2 regardless of dividers or CLKCCS. This is a bounded software policy,
/// not a claim that hardware resets its dividers. No CR0 write follows final
/// LSE selection. Source loss invalidates frozen timing; no recovery, public
/// LSI SYSCLK, low-power restoration or RTC migration is added. The fixed
/// 1 MHz time driver remains incompatible with this 32768 Hz system tree.
/// See docs/l083-lse-sysclk.md for the complete own-source contract.
///
/// On CW32L010, starting a previously disabled LSE while RTC selects LSE
/// additionally requires a handover with no dependent RTC_OUT or RTC_1Hz
/// observer. Disconnect or leave inactive PB04/PB06 RTC digital output pads
/// and their external users, BTIM RTC trigger/reset paths, and GTIM/ATIM RTC
/// input capture or trigger paths. LPTIM and directly visible RTC output
/// conflicts are also checked. Dormant timer clocks are not enabled to prove
/// this condition. Ordinary reset entry with no intervening setup satisfies
/// the reset-route condition; register similarity is not proof of reset and
/// a firmware jump must establish it independently. Root disconnection also
/// excludes downstream timer/ADC-trigger and GPIO-filter cascades; interrupt
/// masking alone does not disconnect these observers.
///
/// L010 LSE pad admission may temporarily run the whole GPIOB bank. Input
/// sampling, filtering and armed edge capture may advance, including before
/// a later initialization error. The handover must not depend on the bank
/// remaining paused or on absence of those effects, including retained
/// LSI/LPTIM-filtered or asynchronous alternate-function participants.
/// Unrelated controls and flags are preserved; reopening the gate is not a
/// side-effect-free read. No particular unrelated output-level change is
/// implied by gate opening.
///
/// On CW32L010F8P6, CW32L010F8U6 and CW32L010Y8M6, selecting LSE SYSCLK
/// retains factory-calibrated, enabled HSI. Preparing HSI may temporarily request
/// unchanged LSI, whose inherited TRIM/WAIT and actual frequency must already
/// satisfy the own legal 32.8 kHz ±10% regime, including after genuine reset.
/// An entry-nonstable first request rejects RTC SOURCE = 2 even with START = 0,
/// UART1/2 SOURCE = 3 even with RX/TX disabled, enabled LPTIM LSI, MCO SOURCE = 4,
/// enabled LSIRDY, and enabled LSI-filtered VC/LVD even with zero filter count.
/// This entry classification is retained through a second admission at the
/// request edge; later STABLE cannot turn it into inherited monitored readiness.
/// Stable legal automatically requested LSI with LSIEN = 0 remains supported.
/// A needed HSI start/restart also rejects raw-HSIOSC MCO SOURCE = 3 and enabled
/// HSIRDY; RTC SOURCE = 3 retains its existing factory-ready ownership rule.
///
/// These checks do not establish universal LSI idleness. The handover must
/// permit or disconnect retained GTIM/ATIM LSI_OUT selector 9, whole-bank GPIO
/// LSI filtering, IWDT and downstream timer/ADC/GPIO or external participants
/// across temporary LSI start and restoration of the software LSI request.
/// Their dormant working gates are not opened to inspect them. Reset-looking
/// selectors, missing tokens, closed gates and interrupt masking cannot prove
/// actual reset history or disconnect observers.
///
/// StartupOnly SYSCLK can stop the CPU on source loss without a fault or error
/// return, even with inherited CLKCCS enabled. MonitoredExistingRoutes requires
/// an already stable legal LSI before any LSI enable and preserves existing
/// fault/IRQ/brake routes; faults may affect outputs or observers before error.
/// CLKCCS is preserved. Documented HSI 4 MHz fallback is qualified up to 4.08 MHz
/// without bus-divider credit, but gives no register-state or progress promise.
/// All frozen LSE timings become invalid on loss/fallback. Dividers are installed
/// under calibrated HSI before the final LSE mux write; no later CR0 write is
/// made. Flash planning also covers requested HSI and fixed fallback. Final
/// source, policy, monitor, mux/divider and pad checks precede publication.
/// Errors may leave partial source/gate/Flash state and monotonic reservations;
/// reset is required before retry and ordinary reset may retain LSE ownership.
/// Poll budgets count CPU iterations only while execution continues. The fixed
/// 1 MHz time driver rejects this 32768 Hz tree before tokens or RCC MMIO.
/// See docs/l010-lse-sysclk.md for the complete own-source target contract.
///
/// On CW32L011K8T6 and CW32L011K8U6, LSE SYSCLK retains enabled,
/// factory-calibrated 96 MHz HSIOSC through the requested divider (default /24).
/// Ordinary legal cold entry uses the existing guarded unchanged-LSI bridge
/// when HSI trim needs preparation; StartupOnly does not require factory LSI.
/// Incoming source/bus/Flash state, HSI trim and unchanged LSI must already be
/// electrically legal. The own RM legal LSI adjustment condition and the
/// 41000 Hz factory-monitor envelope remain separate prerequisites; the latter
/// does not make arbitrary inherited rates below that ceiling legal.
/// MonitoredExistingRoutes additionally requires stable, non-erased,
/// factory-matching ten-bit LSI TRIM at 0x001007C2 before configuration-gate
/// writes. Later bridge readiness cannot manufacture that entry fact. LSI
/// TRIM/WAIT and the original software LSIEN request remain unchanged/restored.
///
/// An entry-nonstable first LSI request rejects RTC SOURCE = 2/reserved selections,
/// UART1/2/3 SOURCE = 3 even with RX/TX disabled, enabled LPTIM LSI, MCO SOURCE = 4,
/// enabled LSIRDY and enabled LSI-filtered VC/LVD even with zero filter count.
/// Held reset rejects inspection; each configuration gate restores separately,
/// preserving enable/restore failure distinctions. The same admission repeats
/// immediately before LSIEN, using the entry classification despite later
/// STABLE. Stable legal automatic LSI clients with LSIEN clear remain admitted.
/// RTC SOURCE = 3 requires already factory-ready HSIOSC; necessary HSI start or
/// retrim also checks raw-HSI MCO SOURCE = 3 and enabled HSIRDY. The handover must
/// additionally permit or disconnect PB0 AF3 HSIOSC_OUT observers across HSI
/// interruption, and residual GTIM/ATIM LSI_OUT selector 9, whole-bank GPIO LSI filters,
/// IWDT and downstream timer/ADC/GPIO or external observers across the bridge.
/// Dormant work gates are not opened to establish universal idleness.
///
/// The native PC14/PC15, whole-GPIOC, RTC_OUT/RTC_1Hz and raw PINLOCK rules
/// below still apply, including exact source reuse and final pad inspection.
/// StartupOnly loss can stop the CPU without a fault/error return. Monitored
/// mode preserves existing IRQ/brake routes and CLKCCS; real faults can affect
/// observers before an error. Documented effective HSI 4 MHz fallback receives
/// its full 4.08 MHz bound without bus-divider credit, independently of configured
/// HSI at final divisors. This promises no register retention or clock continuity.
/// WAIT3 precedes guarded transitions. Final dividers are verified under factory
/// HSI before the last CR0 write selects LSE; no later CR0 write occurs. Final
/// WAIT0/1/2/3 covers LSE, configured HSI and fallback. Source/policy/monitor,
/// mux/dividers, pads and authoritative Flash WAIT are checked before publication.
/// Existing HSIOSC/LSE calendar capabilities remain; no RTC/AWT migration occurs.
/// Errors publish no clocks, can leave partial state and require reset before
/// retry; ordinary reset may retain LSE. Frozen timings are invalid after loss.
/// Poll budgets require continuing CPU execution; the fixed 1 MHz time driver
/// rejects this tree before tokens or RCC MMIO. Generic-family targets
/// remain excluded. See docs/l011-lse-sysclk.md for the own-source contract.
///
/// On CW32L012C8T6/C8U6, direct LSE SYSCLK uses the existing native LSE
/// declaration. Factory HSIOSC stays enabled: configured HSI defaults to /12
/// (8 MHz, 7.84–8.16 MHz), distinct from reset/effective failure fallback /24
/// (4 MHz, 3.92–4.08 MHz after factory trim). There is no PLL or public LSI
/// SYSCLK. LSE and configured HSI at the final dividers, plus the full fallback
/// without bus-divider credit, must independently fit the declared bus limits.
/// LSE bounds cover every cycle and the complete declared board envelope;
/// average ppm is insufficient. No frequency or board condition is measured.
///
/// MonitoredExistingRoutes requires entry-stable, non-erased, factory-matching
/// native nine-bit LSI TRIM at 0x001007C2, with unchanged TRIM/WAIT. The own
/// factory maximum is 36,080 Hz. The detector's extra edge is an engineering
/// margin, not a measured jitter or continuity guarantee. Later bridge startup
/// cannot manufacture entry monitoring. StartupOnly STABLE is a startup latch:
/// later LSE loss can halt the CPU without an error return. Existing fault,
/// IRQ, brake, CLKCCS and output-recovery policy remain in effect.
///
/// A needed entry-nonstable first LSI request rejects RTC/AWT SOURCE2 or
/// reserved sources, UART1/2 SOURCE3 even when RX/TX is disabled, operational
/// UART3 SOURCE3, enabled LPTIM ICLKSRC3, MCO SOURCE4, enabled LSIRDY, and
/// enabled LSI-filtered LVD or any of the four VCs even at zero filter count.
/// Both I2Cs' master and slave raw CLKSRC1/3 are conservatively refused for
/// first LSI request or necessary HSI start/retrim; the source3 conflict is
/// unresolved. HSI start/retrim also rejects MCO SOURCE3 and enabled HSIRDY.
/// Factory-ready divider-only changes leave raw HSIOSC unchanged. RTC/AWT
/// source3 requires already factory-ready HSIOSC; source1 requires declared,
/// exactly reused enabled HSE. Retained RTC PSC1 must satisfy its 1 MHz ceiling
/// even when START is clear. Enabled inherited HSE is never stopped or retuned.
///
/// The shared ADC1/ADC2 configuration-and-work gate may be opened to read EN.
/// That can resume conversion, triggering or other ADC-domain progress before
/// an enabled-ADC refusal. The supported functional handover must already
/// permit this progress; restoring the gate cannot undo it. The separate
/// platform bus-master/memory-ownership boundary above still applies. This is
/// not a new hidden memory-safety obligation on safe Rust callers. The same
/// whole-GPIOC operational handover and native LSE/RTC/pad rules below apply.
///
/// The functional handover must permit or disconnect PB0 AF3 HSIOSC_OUT,
/// PB11 AF4/PF3 AF2 LSI_OUT, inaccessible UART3, direct timer inputs and
/// cascades, whole-bank GPIO LSI filters, IWDT and downstream timer/ADC/GPIO
/// or external observers across temporary oscillator requests and restoration.
/// Dormant timer, output/GPIO or disputed UART3 work gates are not opened to
/// prove universal idleness. Native timer/source mapping disagreements remain
/// unresolved; all documented alternatives must be inactive or disconnected.
///
/// WAIT3 precedes guarded HSI handover. Final dividers and WAIT0/1/2/3 are
/// established while factory HSI is verified, covering LSE, configured HSI and
/// the full fallback. Owned Flash readback completes only when both interfaces
/// show the target WAIT. Matching old values may only continue bounded polling;
/// mixed observations fail even if caused by a benign read race. The final LSE mux write is the last CR0 write; no CR0 or
/// FLASH write follows it, including every failure and pad/gate cleanup path.
/// No rollback, retry write or stale LSE reselection overrides a fallback.
///
/// HSIOSC/LSE calendar capabilities remain; no RTC/AWT source, time, prescaler,
/// flag or alarm migration occurs. Frozen clocks are published only after all
/// source, fault, route, monitor, divider, pad and authoritative WAIT checks.
/// Failure consumes initialization ownership, publishes no clocks and requires
/// reset before retry; ordinary reset may retain LSE. CPU poll budgets require
/// continuing execution. The fixed 1 MHz time driver rejects this tree before
/// tokens or RCC MMIO. See docs/l012-lse-sysclk.md for the own-source contract.
///
/// CW32L011/L012 have the same functional requirement when a new LSE feeds
/// RTC SOURCE0, and again when an RTC owner later activates or changes the
/// calendar source: no independent RTC_OUT/RTC_1Hz recipient may depend on
/// the transition unless its effects are within that RTC owner's scope. This
/// includes all output pads/external wiring, BTIM trigger/reset, GTIM/ATIM
/// inputs and their cascades, and LPTIM events. L011 routes include PA01/PA03,
/// BTIM1..3 code6, GTIM/ATIM TI code8 and LPTIM triggers1..4. L012 includes
/// PA01/PA03/PB14/PB15/PC13 and LPTIM triggers1..5. Its BTIM/ATIM mapping
/// conflicts remain unresolved; all documented alternatives must be inactive
/// or disconnected. Reserved RTC1HZ0 and a reset-looking selector are not
/// proof that a root is constant or disconnected.
///
/// On L012, newly starting LSE also requires no independent direct LSE_OUT
/// recipients on PB12/PF01/PF03, and no inaccessible inherited UART3 LSE
/// owner. The two current manuals disagree about UART3's gate operation.
/// Only already-open output banks and UART3 are inspected; closed gates do
/// not prove absence of users. These conditions are not fully checked by the
/// HAL. Dormant output/timer banks are never opened merely to inspect them.
///
/// L011/L012 pad admission temporarily runs the whole GPIOC bank, including
/// during exact source reuse. Sampling, filters and armed edge capture can
/// advance before an error. The handover must permit that progress; restoring
/// the incoming gate cannot undo it. PC13, unrelated controls, shared FLTCLK
/// and flags are preserved. No particular unrelated output glitch is implied.
/// MonitoredExistingRoutes intentionally permits existing asynchronous fault
/// capture, enabled IRQ and brake/PWM effects before an error and afterward;
/// preserving routes does not freeze their observers during a real fault.
///
/// These are functional limits on supported hardware handovers. They do not
/// replace the existing pre-Rust bus-master/memory-ownership boundary or give
/// safe Rust callers a hidden memory-safety obligation.
///
/// On x030/L083/F020/L031/L052/R031, safe static-copy admission is recorded
/// once after clock init.
/// An already enabled/reset-held or non-default DMA controller is rejected for
/// safe copies without resetting it, clearing flags or treating EN=0 as a drain
/// acknowledgment. Rejection leaves other peripherals usable; acquiring a
/// `dma::CopyChannel` reports the reason. A non-default-register rejection is
/// detected after clock enable and leaves that gate enabled; it is diagnostics
/// within the supported entry model, not arbitrary-handover sanitization.
/// Generic profiles without qualified SRAM cannot acquire this capability.
/// The shared DMA gate remains enabled
/// once admitted; no safe DMA operation resets or disables it.
///
/// Returns clock or timer initialization errors. Panics if peripheral
/// singletons have already been taken. Hardware waits are bounded.
///
/// Invalid configuration is rejected before taking ownership. A hardware timeout
/// can leave clocks partially changed and consumes the singleton set; reset the
/// device before retrying. RCC errors publish no new clocks. A later time-driver
/// initialization error may occur after RCC has published verified clocks; it
/// still returns no peripheral tokens.
/// With a time-driver feature, exact nominal 1 MHz divisibility is also checked
/// before taking ownership; the whole selected timer/IRQ is then initialized
/// before returning the remaining peripherals. See the time-driver module for the
/// continuous-clock and strict IRQ blackout contract.
pub fn init(config: Config) -> Result<Peripherals, rcc::Error> {
    let clocks = config.rcc.frequencies()?;
    #[cfg(feature = "_time-driver")]
    let timer_divisor = time_driver::validate_clock(clocks.pclk_bounds())?;
    #[cfg(not(feature = "_time-driver"))]
    let _ = clocks;
    let peripherals = Peripherals::take();
    unsafe {
        rcc::init(config.rcc)?;
    }
    #[cfg(feature = "_time-driver")]
    time_driver::init(timer_divisor)?;
    #[cfg(any(dma_v1, dma_cw32f020_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
    dma::init(config.rcc.timeout);
    Ok(peripherals)
}

/// Driver operating modes, matching Embassy naming.
pub mod mode;

/// RAM parity diagnostics with exclusive controller ownership.
pub mod ram;
