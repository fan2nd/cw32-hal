# v0.13.2 support-module consolidation

2026-10-03; Rust/Cargo 1.99.0; `thumbv6m-none-eabi`. This is a source-layout
cleanup of v0.13.1. It adds no driver functionality, dependency, test framework,
or public API. No chip was flashed or electrical/timing behavior measured.

## Changes and preserved contracts

- Metadata association generation, IP validation, GPIO capability checks and
  cfg emission are private functions in `embassy-cw32/build.rs`. The separate
  `build_support.rs` file and its module/rerun declarations are removed.
- The existing `EventState` lives crate-private in `interrupt.rs`.
  `async_support.rs` and its module declaration are removed. ADC, GPIO, VC,
  ATIM and CORDIC references, including generated instance implementations,
  use the new location. No replacement utility/support module was introduced.
- The event latch's struct and implementation are byte-identical to v0.13.1.
  Pending event bits, register-before-check, waker clone/drop ordering,
  cancellation/rearm serialization and wake-after-service behavior are retained.
  No AtomicWaker substitution or synchronization redesign is part of this release.
- Moved build functions/constants are byte-identical after making the functions
  private and updating one documentation comment. Unknown-IP rejection and
  audited register/field-layout assertions remain intact.
- All ten existing workspace package versions advance together to 0.13.2.
  The dependency requirements and public driver contracts do not change.

## Equivalence and production checks

Four pre-change ARM release builds captured both chips with minimum and full
valid feature sets. Matching post-change builds preserve:

- The exact enabled cfgs, environment exports and link-library declarations.
- Cargo build instructions, including check-cfg declarations, after removing
  the obsolete support-file rerun line and normalizing only the output path.
- Every generated HAL file, after changing only the EventState module path in
  the generated GPIO, ADC and comparator implementations. Association tables
  are byte-identical without normalization.

Independent source review checks the moved bodies and all affected callers;
this is not a claim of new on-device race testing. Existing v0.13.1 race and
ownership checks remain documented in [the earlier validation](validation-v0.13.1.md).

The production matrix passes:

- Workspace formatting, regeneration and read-only byte-drift check.
- Both-chip PAC metadata/runtime ARM release builds.
- Both-chip minimum/full HAL ARM release builds and strict documentation
  (`RUSTDOCFLAGS="-D warnings"`).
- All six board executables in ARM release, plus 05/06 with explicit
  `motor-output-enable`. Default output policy is unchanged.

## Clean source-only archive

The final source-only archive is extracted into a fresh directory and repeats
that complete matrix using a new Cargo target directory. Its maintained files
match the working source byte-for-byte. Regenerated normalized JSON and PAC
outputs match the working tree. The archive contains no generated output,
Cargo.lock, target directory, tests, validation scripts, logs or download cache.

Validation scripts and snapshots stay outside the repository. No Clippy or
silicon result is claimed. The existing proc-macro-error2 future-compatibility
notice is unrelated to this cleanup.
