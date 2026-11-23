# CW32W031 supported-target retirement validation

Date: 2026-10-15. Baseline: `029e90be138a5f536b363b5bf3c90deddd379646`.

## Scope and compatibility boundary

The supported generic `cw32w031` and exact `cw32w031r8u6` Cargo targets are removed. The official vendor identifies CW32W031 as a CW32L031 + PAN3028 SiP; this does **not** establish package, pinout, voltage or RF interchangeability with a standalone L031. There is no feature alias and no automatic firmware migration. CW32R031 remains supported. See [the retirement note](w031-target-retirement.md) for the official source and migration limits.

Normal data generation and direct PAC generation reject retired target admission, including stale externally supplied retired chip metadata. Historical source-qualified W031 facts remain available for bounded source audits, not for supported target generation. The historical reader permits exactly two hash-pinned snapshots, requires absence of live retired output, and is not a generic missing-file fallback. Rust historical helpers are test-only.

## Executed software acceptance

- Exact roster: baseline 54 HAL/PAC features and metadata targets minus exactly the two retired targets equals 52. No survivor was added, removed, aliased or silently skipped.
- All 52 retained chip JSON files and 875 retained generated chip/register/peripheral/access files are byte-identical to baseline. Exactly 11 W031-only outputs were removed, including two unreferenced deduplicated metadata files. Only three generated PAC indexes changed: Cargo features, all-chips index and chip-peripheral versions. The final 899-file generated inventory is byte-identical to the inventory used for the successful builds.
- 109 retained-target ARM release library configurations passed: 52 targets with `rt`, the same 52 with `rt,defmt`, and five existing Embassy time-driver configurations. Two additional exact L031/R031 time-driver builds passed.
- 83 retained L031/R031 example build/link selections across all 21 affected example manifests passed. These are software link results, not board runs.
- 25 removed-feature negative checks passed by failing with the expected Cargo missing-feature diagnostic: generic/exact HAL and PAC features, plus the removed feature in each affected example. Failure for the intended diagnostic is not an arbitrary compilation-error acceptance.
- Full `./d test` exited zero after the explicit retirement adapters and fixture updates. It includes new public-generator/PAC retirement tests, survivor metadata contracts, register access controls and historical source checks. Full `./d lint` and `./d check` passed on final source; generation comparison and zero/multiple-chip admission controls remained active. `git diff --check` passed.
- All 72 broad source/audit commands in the source-audit suite passed on their accepted final attempts. This includes 53 provenance tests, 10 transactional-generation tests, source-qualified clock/RTC/UART/I2C/timer/ADC/DMA checks, and 12 discovery-scope tests. Historical W031 source checks in these logs do not mean W031 is still an active target.
- Clean staged source packaging passed the unchanged distribution/provenance gate. The stage contains only tracked source plus an explicit new-file whitelist; no vendor source tree, build output, validation scratch directory, or temporary API workspace is distributed.

## Preserved history and reviewed current-registry transition

All 308 other tracked documentation/source JSON artifacts are byte-identical to baseline. The single current `docs/upstream-file-provenance.json` change is independently reviewed: only the serde library row's replacement-review tip changes, and its complete prior review is nested unchanged. Every other registry value is identical. The previous serde source body is byte-identical; the sole addition exposes the new project-authored support module. The new [review receipt](w031-support-provenance-review.json) does not claim legal clearance. Original historical acceptance receipts, source hashes and vendor provenance are not rewritten.

Source validators distinguish exact current-target projection from immutable historical evidence. Survivor checks retain exact equality; there is no broad subset acceptance, missing-file catch-and-ignore, source-cohort removal or relaxed provenance guard. The separate pre-existing discovery expectation of 553 outputs was reproduced at baseline against the same pinned inventory, which already enumerated 567; the repair checks the independently enumerated exact scoped set, duplicates, omissions and same-count substitutions rather than merely changing a magic count. The pre-existing SPI example-bin assertion was also corrected to include the already-existing `async_words` bin.

## Failed attempts and reruns

Initial compiler/generation checks hit ENOSPC. Only regenerable compiler output under `firmware/target` was cleared after the affected processes terminated, freeing 9.2 GB. Sources, vendor evidence and logs were not removed. Failed/remaining commands were resumed; previously successful commands retain their original logs and are not represented as newly rerun.

The audit caught stale current-versus-historical target counts and reads in RTC, I2C, UART, BTIM, flash, DMA and metadata fixtures. Each was repaired with exact retirement projection or the bounded historical snapshot path, then rerun. The initial packaging rejection of the changed serde replacement hash was resolved by the independent additive review and one-row current-registry transition, not by weakening the validator. Provenance views became stale while source adapters were edited; their unsuccessful attempts remain recorded, and final views/tests were regenerated after source freeze.

Two validation-harness setup corrections are separate from product fixes: the baseline parity harness initially treated two W031-only deduplicated metadata files as survivors, and the first RTC alarm audit used an unavailable default evidence path. The accepted harness verifies no survivor references the two removed metadata files and supplies the explicit official-source directory.

## Limits

No hardware, RF, low-power, electrical-equivalence, Miri, or remote CI run is claimed. Existing hardware-evidence blockers remain unchanged. This acceptance concerns supported-target retirement, strict historical evidence preservation, reproducible generation, compilation, linking and source packaging only.

## Audit artifact fingerprints

- `d-test-final2.log`: SHA-256 `5a299a3a44192d5ebd1be4edde9dc43bf5cee97604c6d64fdaa880bd94acc85d`.
- `matrix-results.json`: SHA-256 `5996da28d272da328717776d90ea7cde9edee6a420ca1cf0d54be2dafd67a526`.
- `examples-results.json`: SHA-256 `b9c864bfe256492133b27a37b5151efa13e44b705fa1fc11280db60c60826af4`.
- `negative-features-results.json`: SHA-256 `c0ec87324046e7e53f73bc310001b8227ef44a323fbd12030ac3af89cde57ff8`.
- `quality-final-results.json`: SHA-256 `f5a8ca65139cbb5eab4c59d0b7970a5e89f68e8d6f72c223eb0d847a66ec8b61`.
- `audit-sources-results.json`: SHA-256 `548d2c0b7c60496c066345b27dd6130f2029a779c92cdc82a47c547e5d13d55b`.
- `parity-results.json`: SHA-256 `f0c842693af77b248aca84686c0ca2abea1b25036a1eaf91233198089ebacaf5`.

## Final documentation-only integration and package check

On 2026-10-15, the already software-validated candidate was integrated for
publication on parent `5446f201b5d16e78775f2dffdd63c4b12b698b74`. That parent
differs from the validation baseline only in `todo.md`; its exact Git-object
bytes are preserved in the final source package and expected publication tree.
The validation worktree's original `todo.md` was not overwritten.

The only post-validation candidate edits are current-count corrections in
`README.md`, `examples/shared/four-channel-dma/README.md`,
`examples/shared/low-power-timers/README.md`,
`examples/shared/lse-sysclk/README.md`, and `examples/shared/spi-dma/README.md`, plus this
append-only integration note. All other 141 publication paths and every
non-documentation staged source/test input retain their validated SHA-256
fingerprints. The original 147-path publication inventory, receipt, package,
package log and stage were preserved before the final package was rebuilt.

Final checks reverified the unchanged generated-output snapshot, the retained
historical JSON artifacts and the exact one-row current-provenance transition;
checked relative documentation links and whitespace; and rebuilt the package
from a clean stage using the unchanged source-distribution/provenance gate.
Every archive member's bytes and executable mode were compared with that final
stage. Only the explicit source inventory is included: no vendor source tree,
generated build output, validation scratch files or temporary API workspace.

The software acceptance above remains evidence from the unchanged validated
code, not a claim that Cargo builds, tests or firmware links were rerun after
these documentation-only changes. This final integration adds documentation,
packaging and byte-identity checks only; it does not add hardware or remote-CI
validation.
