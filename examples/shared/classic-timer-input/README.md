# Classic GTIM external inputs

Select one exact package feature from Cargo.toml. The build selects two distinct, already-qualified CAP1/CAP2 pads on one owned GTIM and saves the exact pad and AF assignments in `external-input-routes.txt` beside its generated constructors. The verification receipts retain those assignments for every linked firmware.

Connect voltage-compatible external signals to those two pads and a shared ground. `polling_capture` observes real external edges and stores latest-value snapshots in `CAPTURE_TICKS`; it deliberately pauses each capture channel during read/acknowledgement. `quadrature` accepts external quadrature phases and stores independent position, direction and wrap-flag observations. Neither program generates input edges, loops outputs back, forces flags, or models peripheral behavior.

All classic encoder configurations use the own-manual-required ARR=0xffff. No tests, lossless capture throughput, debounce, edge timing or silicon validation is implied. See [driver limits](../../../docs/classic-timer-input.md).

The optional `time-driver` feature reserves GTIM1 and selects a different available timer for external input. F002/F003 have only one GTIM, so that feature is not a usable external-input example on them; the library's reservation remains compile-verified separately.
