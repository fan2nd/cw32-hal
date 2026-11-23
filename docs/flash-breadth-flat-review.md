# FLASH breadth flattened Stage12 rebase: independent review

Reviewed 2026-10-08T14:58:44Z. Verdict: **pass; no blocking finding**. This reviewer wrote only this report and its JSON companion. No main-tree changes, HAL tests, test hooks or harnesses were created.

## Boundary result

The 65-file Stage12 HAL snapshot is preserved except for exactly four authorized files:

- `embassy-cw32/build.rs`
- `embassy-cw32/src/lib.rs`
- `embassy-cw32/src/flash/mod.rs`
- `embassy-cw32/src/flash/backend.rs`

All 61 other HAL files are byte-identical to the snapshot; none are added, deleted or moved. In particular, the single `src/gpio.rs` remains unchanged, and no nested `src/gpio/` directory exists. HAL Cargo.toml and all unrelated RCC, ADC, SPI, I2C, timer, watchdog, RTC and interrupt modules are untouched.

Snapshot SHA-256: `484c4f258dac6d9ae0413ccfdd7e5618636d634457c9fe2a71cf44a9776f74fa`. Accepted packet manifest SHA-256: `61a7aca9e6fd2836359afd6be33c9604cb1ab67edfdaea77b7308fa98b53a113`.

## Behavior equivalence

- The accepted `flash/backend/mod.rs` is byte-identical to the rebased `flash/backend.rs`. The old nested backend directory is absent; existing `mod backend;` resolves the single flat sibling file.
- The rebased `flash/mod.rs` differs from the accepted module only in three documentation blocks. All noncomment lines are identical. Comments now distinguish generic profiles without memory metadata from F030 compatibility aliases whose memory is verified but is not qualified for this storage API. No alias metadata or runtime policy was changed.
- The build hook is identical to the accepted Flash hook after comment-only alias wording is removed. Against current Stage12, the only edits are five Flash cfg-name additions and replacing the old three-family Flash block with the accepted nine-family version checks and exact 25-part capacity allowlist. Every byte outside these bounded edits matches Stage12. Existing capacity-constant emission is retained.
- The only lib.rs edit is changing the public Flash module gate from `flash_cw32l031_v1` to `flash`; every other byte matches Stage12.
- The four real-example source/configuration files remain byte-identical to the accepted packet. Cargo.lock subsequently received the reviewed additive Stage12 build-dependency closure refresh described below; exact-part features and linker reservation logic are unchanged.

Consequently the accepted safety behavior is retained: exclusive unsafe region ownership; exact capacities; electrical/HCLK/WAIT checks; legal lock masks and groups; keyed MODE replacement; W0C flags; original WAIT preservation through cache disable/restore; Read-mode verification before cache restoration; blocking BUSY completion; and fail-closed cleanup. No unsafe new storage qualification or excluded-family backend was introduced.

## Reviewed file hashes

| Path | SHA-256 |
|---|---|
| `embassy-cw32/build.rs` | `f49ca09fecf4eb381868e2abf0a970c01f52e1b97cf8feb81dbc4f5cb8b9944b` |
| `embassy-cw32/src/lib.rs` | `9e469381e4806768a5df280a793a31722932969cc560471f838bbeb79b0aa0e9` |
| `embassy-cw32/src/flash/mod.rs` | `ec2283f73295c5d748cf2ee616300a617c3943a44b6cdae35f8ad1fe381dea1e` |
| `embassy-cw32/src/flash/backend.rs` | `98b8bd62175292c6786ff8cf3189fb6515b0e116d27f045ec587f927ce8d953b` |
| `examples/shared/flash-storage/Cargo.toml` | `aec81dcfcf68c7f9d52a8c3e00b8a4ee569c39a2cc4a3f0e29b6204993dcdafa` |
| `examples/shared/flash-storage/Cargo.lock` | `0323ed756061e14bd2e78e96c161f6b8ec1d5232d1637f2ed650d9fe3175b6fc` |
| `examples/shared/flash-storage/build.rs` | `ce62aa478924fb8456687ea71f32c368071c3944a3e21ec065e87af689dfe0da` |
| `examples/shared/flash-storage/src/main.rs` | `b58073ad0f191c865af70eff5c353fe27263ac51ec94796627875980323b6e6a` |
| `examples/shared/flash-storage/README.md` | `54daf746bb33d48c06d12a15bf468bbfc875102793669cc4f2c618e5b64c5082` |

## Verification boundary

This pass independently compares accepted production behavior, the Stage12 snapshot, shared hook diffs and file hashes. No HAL test markers or test-only hooks were found. Normal ARM builds and real-example links for this flattened candidate are being performed by the implementation task; earlier nested-path build results do not count as new flat-candidate build results. Nothing was flashed or run on silicon.

## Scoped GPIO/ATIM overlap addendum

Independently checked on 2026-10-08T15:05:46Z. The frozen Flash production patch has SHA-256 `2ebe7cad107e87cddb70d8ff3191c514211a83b1430cec07791a7b69419e7997`. Its only overlapping production path with the ATIM shared patch is `embassy-cw32/build.rs`; Flash does not touch GPIO or timer source.

For the narrow composition check, only ATIM's build.rs diff was extracted from its shared patch (full patch SHA-256 `22d4a74f4595404bf9ad7df37d69d6f789d9a750761b4b8c06a54e30f8b950b3`), then applied with the complete four-file Flash production patch to temporary copies of the pinned Stage12 baseline. Both orders succeed with `patch --fuzz=0`; line offsets are accepted, context fuzz is not. Both yield identical combined build.rs SHA-256 `9f539831ec50860ce54ee05f6c4a89f8e738c38ac069b86f1639256486fdff7d`. The copied GPIO remains unchanged. This verifies shared-hook composition only, without claiming a fresh review or application of the entire ATIM packet. Main was untouched.

The example Cargo.lock refresh adds serde_json plus its six additional closure packages (itoa, memchr, serde, serde_core, serde_derive and zmij) because current Stage12 already declares serde_json as a HAL build dependency. No existing package version, source or checksum changed; the only modified existing package record adds serde_json to embassy-cw32's dependencies. Refreshed lock SHA-256: `0323ed756061e14bd2e78e96c161f6b8ec1d5232d1637f2ed650d9fe3175b6fc`. All four runtime hashes, four other example files and all 61 unrelated HAL files remain unchanged. The producer is rebuilding the 25 real examples with the refreshed lock. No blocker or test harness was introduced.
