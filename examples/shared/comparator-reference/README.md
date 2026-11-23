# Shared comparator-reference polling

This is a real `no_std` Cortex-M firmware image. It is compile/link checked only;
no MCU execution or analog measurement is claimed.

Select one exact part:

- `cw32l010f8p6`: VC1 positive PB5 and VC2 positive PB6, both borrowing VCDIV.
- `cw32l011k8t6`: VC1 positive PA0 and VC2 positive PA1, both borrowing VCDIV.
- `cw32l012c8t6`: VC1 PA0 / VC2 PA1 borrow VC12REF; VC3 PA6 / VC4 PA9 borrow VC34REF.

The first bank uses half the declared 3.3 V analog supply. L012's second bank
uses one quarter of nominal Vcore (about 1.6 V, not an exact calibrated source).
The board must supply 3.3 V to VDD and VDDA where present, and keep positive
inputs within analog ground and the analog supply. These declarations are not
voltage measurements. Check exact package pin positions in the own datasheet.

Outputs always retain `Readiness::Unknown` on these families: qualify reference,
comparator startup, response/overdrive and changing-input settling on the board
before making decisions from levels. No fixed software delay is claimed adequate.

Build from this directory, after source generation:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --features cw32l012c8t6
```

See `../../../docs/comparator-reference.md` for ownership, retained analog power,
source evidence and the L012 DIV field disagreement.
