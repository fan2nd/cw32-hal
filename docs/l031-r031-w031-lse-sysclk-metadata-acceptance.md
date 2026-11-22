# Stage58 independent metadata, timing-contract and compatibility review

**Main-scope verdict: accept the frozen metadata addition. No blocking source, projection, factory-reference or timing-contract discrepancy was found.** This is a source/metadata review, not full runtime or publication acceptance. Publication remains HOLD. The separate CI validation adjustment will receive a supplement; it does not replace this conclusion about the original frozen inputs.

## Reviewed inputs and integrity

- Authoritative design-v2 SHA-256: `6661bb5461df1607f261af120ae589bf0d778fbdc87e4a781546a94602dd43ca`; independent design report SHA-256: `77658f9ac665df31c541690972b9748d1f588b0a42df2c1f4238c1919332d962`.
- Authored data-manifest SHA-256: `74d9a63329df7ad3957362a6d147df139655e443103fac771083f5e48fe45ea4`; complete eleven-file manifest SHA-256: `42565834a1c7e360938538947fffdace450a236856a29fd4f107aea9462e2ed0`. All listed sizes and hashes independently match the candidate.
- Stage57 source ZIP SHA-256: `4399316e13463450c2429ac8b830c6e1af8f3f46215f61f1631dcb8104d61671`. All 1,229 archived originals independently byte-match the Stage57 baseline used for comparison.
- The three shared models independently byte-match that ZIP: `cw32-data-serde/src/lib.rs` (`f36ce5da85172080d963347d8c9e25985b02d92e1a1227df78850cf7dd1ea9b1`), `cw32-metapac-gen/src/data.rs` (`227095b7b073825f26e34d2fbfe745c0938b8861dd53f68f00cf788668f6fc3c`), and `cw32-metapac-gen/res/src/metadata.rs` (`d5a1ab60f97e6a097e1970bd3c32667828563035442fb8fa299c8270147d516c`). No schema, layout or identity change is needed.
- Source authority, catalog, layout history and the original RTC calendar facts are unchanged. Regenerated source-lock, vendor-sources and coverage are also byte-identical to Stage57. The reference-index naturally adds the new evidence document and new hashes for the two changed source inputs; it does not change original source identity.

## Exact package and generated projection

The YAML adds only `sysclk_detector = {lse_edges:128, lsi_cycles:256, margin_lse_edges:1}` to CW32L031C8T6, CW32L031C8U6, CW32L031F8U6, CW32R031C8U6 and CW32W031R8U6. Removing exactly that field from those five existing records and removing the one new qualification-policy digest reproduces the complete Stage57 YAML object. All 23 existing active-LSE records retain their other fields and evidence policies.

Independent comparisons cover all 54 generated chip JSON profiles and all 54 expanded PAC metadata profiles. Exactly these five change; their only difference is the detector field. The detector roster is now exactly classic3 plus these five (eight total). Other packages, family aliases, all prior RTC facts, pins, register layouts, route data and operating facts remain unchanged. JSON absence is treated as absence, not normalized to null.

The build allowlist distinguishes the classic3, which still require their `lsi_sysclk` facts, from the new5, which explicitly require `lsi_sysclk` to be absent and validate the own RTC factory reference. Generator validation additionally requires the own six-source roster, selected authority identity, matching family scope, exact reviewed pages and matching own RTC source/rate/conditions. Both stage allowlists are exact package names; no family-wide capability is inferred from the PAC selector.

I inspected the actual five production HAL build-script outputs, locating each through its candidate-bound JSON build event rather than choosing arbitrary files from the shared target directory. Every one emits detector constants 128/256/1, nominal LSE 32768, RTC factory address 1051138 (`0x00100A02`), and RTC rates 32800/31816/33784. They have `rcc_lse` and no `rcc_lsi_sysclk`. The generated supply/temperature constants match each family's own qualification. Source snapshots and build-event links are recorded in `docs/l031-r031-w031-lse-sysclk-generated-constants.json`.

## Own original evidence

All six original PDFs were freshly hashed and their selected physical pages were freshly extracted. Each hash, source-authority identity, selected status, family scope, page count and physical/printed page correspondence matches the new qualification document. The three own factory-LSI tables were also independently rendered and visually inspected. This review reused no other family's electrical qualification.

| Family | Own manual hash | Own datasheet hash |
|---|---|---|
| L031 | `4288cfd97b56385059c5a283f69972047af4773ef8bbc4d8b51d8155fb17a760` | `90525f4085d00e9d586a991c24f2e4398d92a41e6cc1402c413e963423a35855` |
| R031 | `fbee9b6942be9fa09f00c946705644d5356c4249f3e2280e3dfe5f0cb342eddb` | `88759314fa4cf8b6caf7098df4829489179e27aa3752a13de85ce955b29c5805` |
| W031 | `b6973677946a9332b0e5b3e954119768aa40469d44140a73e18648e9419bedc9` | `45ec43e6370956d09f9b9aa4c0f6c83fb0661f576d2d2bf64e0a6d7c4e203d3c` |

All page numbers below are one-based physical PDF pages; printed pages are one lower.

- L031 RM1.6: calibration56, detector59, CR0/CR1 67/68, native LSI71, ISR75, Flash107. DS1.9: package26, general38, bypass45, crystal46, factory LSI47.
- R031 RM1.3: calibration58, detector61, CR0/CR1 69/70, native LSI73, ISR77, Flash109. DS1.2: package29, general42, bypass52, crystal53, factory LSI54.
- W031 RM1.4: calibration57, detector60, CR0/CR1 68/69, native LSI72, ISR76, Flash108. DS1.3: package30, general41, bypass51, crystal52, factory LSI53.

Each own manual establishes the LSE selector4, AHB /1,/2,/4,/8,/16,/32,/64,/128 and APB /1,/2,/4,/8; LSE detection uses 128 edges over 256 LSI cycles and requires LSI enabled. Configurable CLKCCS and HSECCS remain distinct from LSECCS. Each own calibration section gives the halfword at `0x00100A02`; each native LSI register gives ten-bit TRIM, WAIT at11:10 and STABLE at15. Factory LSI nominal32.8kHz with ±3% at −40…85°C produces exactly31816…33784Hz.

The retained LSE/monitor supply limits are L0311650…5500mV, R0312200…3600mV and W0312000…3600mV. W031's last range is conservative across the documented supply modes; no RF behavior is qualified. L031 permits24MHz buses below1.8V and48MHz at/above1.8V; R031/W031 permit48MHz. The Flash WAIT0/1/2 thresholds24/48/72MHz do not enlarge the bus ceiling.

Own package tables confirm LSE PC14/PC15 at L031C8T6/C8U6 pins3/4, L031F8U6 pins1/2, R031C8U6 pins2/3 and W031R8U6 pins61/62. L031F8U6 has no bonded HSE pair, so the existing optional-HSE rejection remains applicable. Bypass source limits and crystal tables do not establish arbitrary nominal source support or a maximum crystal startup: nominal remains32768Hz and the1.50s crystal startup entry is typical with no specified maximum.

## Timing, source identity and pre-token rejection

`Lse::bounds` retains its existing every-cycle board declaration and uses `ClockBounds::external`; the new L031 detector restriction is only in the `Sysclk::LSE` branch of this backend's `frequencies()`. General auxiliary LSE does not inherit the new target margin. Missing source fails with `LseNotConfigured`. The selected source is the same already-computed `(Lse, ClockBounds)` tuple that is retained in `Clocks.lse`; factory RTC/LSI bounds are not constructed as the CPU source.

The margin is strictly `256 × minimum_LSE > (128+1) × 33784`. Independently computed RHS is4358136;17023Hz fails by248 and17024Hz passes by8. Hardware threshold remains128; one extra edge is software policy. Own factory LSI rate accuracy is not converted into an every-cycle LSI guarantee, an all-window detection claim, a fault deadline or CPU progress guarantee. Board LSE strict cycle bounds remain distinct from the rate-only detector arithmetic.

Source inspection proves all32 bus-divider choices reject with the fixed time driver before singleton acquisition/MMIO. Nominal PCLK is exactly32768/(AHB×APB), ranging32768…32Hz. In the unchanged `exact_divisor_for`, target denominator is1000000×AHB×APB, always greater than32768, so no nonzero exact divisor exists. `validate_clock` returns `UnsupportedTimeDriverClock`. `init` order is `frequencies()` → `validate_clock()` → `Peripherals::take()` → `rcc::init()`. This argument does not depend on a rounded or widened monitor rate and applies to every one of the32 combinations. Any other invalid configuration may reject still earlier.

Retained HSI is qualified under final buses even when inherited CLKCCS is disabled. The existing exact upper-envelope arithmetic therefore rejects nominal HSI/2 at AHB/1 in the24MHz low-voltage band (upper24.48MHz), and HSI/1 at AHB/1 even in the48MHz band (upper48.96MHz). This is a source-level compatibility and arithmetic check; runtime sequence acceptance belongs to the independent runtime review.

## Method and limits

I did not run Cargo, mutate the candidate, or operate hardware. Generation and ordinary HAL compilation were performed separately; I independently inspected their candidate-bound outputs and receipts. `external-evidence/stage58/l031-metadata-review/audit_metadata.py` is a read-only independent JSON/YAML/PAC projection and arithmetic checker, with output in `docs/l031-r031-w031-lse-sysclk-metadata-correspondence.json`.

For complete disclosure, one external host rustc probe of the bounds/time-divisor functions was run. Further HAL probes were excluded from this review. It did not enter the candidate, project, CI or delivery. It will not be expanded or rerun, is a non-deliverable external scratch asset, and is not an acceptance basis in this report. This report's timing conclusion is established by the explicit source/control-flow and arithmetic argument above.

Source-bound negative guard conditions were inspected, not mutation-tested by this reviewer. This review does not establish hardware startup, arbitrary asynchronous loss behavior, maximum detection/fallback latency, runtime switching, Sleep/DeepSleep, wake restoration, RF, calendar continuity after fault, or publication readiness.
