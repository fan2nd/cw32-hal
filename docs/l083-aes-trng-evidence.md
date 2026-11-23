# CW32L083 AES and TRNG: source-qualified blocking access

This batch implements only L083 (family feature and all five exact packages).
It is an original chip-specific implementation using the pinned Embassy
`Peri`, borrowed-key lifetime, sealed instance and central `RCC_INFO` concepts.
No STM32 register layout, reset sequence, RNG health flags or AES key-derivation
sequence is assumed. No hardware was executed or flashed.

## Authorities and semantics

Exact URL, revision, SHA-256, SDK member chains and page references are recorded
in `l083-aes-trng-evidence.json`, with the canonical pins in
`sources/evidence-sources.json`. The own manual is CN Rev2.0, datasheet CN
Rev1.9, SDK V2.2. Consumed SDK members are byte-compared to the locked archive.
Official PDFs, extracts and SDK originals are not bundled; redistribution rights
for these originals remain unverified.

- AES: manual sections 28.1–28.5, printed pp565–570; datasheet 4.21 p22.
  The engine processes 128-bit ECB blocks with 128/192/256-bit keys. MODE0
  encrypts, MODE1 decrypts. KEYSIZE0/1/2 select those lengths; encoding3 aliases
  128. The driver uses only the canonical encodings.
- TRNG: manual sections 26.1–26.6, printed pp536–541; datasheet 4.20 p21.
  SOURCE0 is linear feedback alone, SOURCE1 analog source, SOURCE2 analog XOR
  feedback. SHIFT1..6 correspond to 8,16,32,64,128,256 shifts. Other encodings
  are undocumented and rejected by the HAL. Source0 is deterministic and is
  never offered as randomness.
- Both START fields: write1 submits; read1 means active, read0 means completed.
  The tables do not specify write0 abort. The driver never uses zero to abort.
- Both have independent HCLK gates, AHBEN AES bit11/TRNG bit10, and active-low
  AHBRST fields with the same bits (manual4.7.12 p85;4.7.15 p89). The documented
  peripheral reset restores registers, state machines and control logic
  (4.2 p48); this bounded first API never pulses reset, including on timeout.
- Neither has an interrupt in table5-1, pp104–105. DMA source and destination
  restrictions explicitly exclude both, sections8.8.6–7 pp149–150; AES repeats
  the DMA exclusion in section28.4 p567. Async/IRQ/DMA are not implemented.

## AES ordering and key lifetime

The only promised representation is documented register-word order: input[0]
maps to DATA0 bits31:0, through input[3] to DATA3 bits127:96. Key words map KEY0
bits31:0 upward to KEY7 bits255:224. Unused high key words are cleared.

The official example encrypts and then decrypts, and only checks recovery of the
original words. It contains no independently known ciphertext. Its byte-looking
constants do not establish interoperability with a standard byte-array API.
This driver therefore exposes words only. No byte serialization, standard
known-answer validation, padding, chaining or authentication is implied.
Hardware ECB by itself is not a safe message encryption protocol.

The caller's key is borrowed throughout the driver's lifetime and is never
cloned into a persistent software buffer. Key has no Debug implementation.
Each idle operation selects direction/key length, reloads the original key,
writes one block, starts and waits. Encryption and decryption use the same
original key as the own manual prescribes. The driver never erases caller RAM.

Idle Drop writes zeros through all eight volatile key registers and four data
registers, then disables the dedicated gate. This statement covers only those
writable registers, not undocumented internal key expansion state, physical
remanence, stack temporaries or certified erasure. Busy Drop deliberately leaves
hardware key/data and gate intact. Keep the driver and successfully call
`wait_idle` before Drop when register clearing after timeout is required.

## TRNG validity and security boundary

Generation enables the analog source, configures one of the two analog-bearing
sources and a documented shift count, verifies configuration/enable readback,
submits START and waits. Only completion allows reads of DATA0 and DATA1.
After those reads it disables analog power, as the manual recommends.
A prior timed-out result can be discarded only after the engine is idle.

The complete TRNG chapter specifies no separate analog-ready/stabilization
interval, clock/seed error flag, health status, repetition test, guaranteed
entropy amount or cryptographic certification. The driver cannot detect an
analog source failure that the hardware does not report. `Sample` is explicitly
raw, unassessed hardware output. All valid SHIFT values remain hardware controls,
not minimum-entropy promises. Default analog-only/256 shifts is a policy choice,
not a claim of stronger entropy. No CryptoRng/RngCore adapter, statistical test,
software fallback, or security suitability guarantee is supplied. Security uses
need an independently qualified entropy/conditioning and failure policy.

## Ownership and bounded failure behavior

Both drivers hold a `Peri` and use the actual generated `RccPeripheral::RCC_INFO`.
Constructor rejects held reset before enabling, verifies the gate, and rejects
active hardware without touching operands/configuration. TRNG also rejects a
pre-enabled analog source at construction. Constructor failure consumes the
passed ownership lease; use `reborrow` when recovery/retry ownership is needed.
A busy constructor retains the gate so pre-existing work is not frozen.

Zero budgets submit nothing. At most `polls` status reads follow submission;
no wrapping counter or elapsed-time claim is used. This intentionally does not
copy the SDK uint16 post-decrement timeout loops, which can wrap at zero.
Timeout never reads or returns result registers, resets, disables analog power,
or gates the engine. Busy subsequent calls write nothing. Drop is nonblocking:
idle cleanup only, otherwise retention. `wait_idle` observes completion without
collecting old output or pretending that an operation was canceled.

The safe ownership contract assumes no concurrent raw PAC/unsafe access or
external clock/reset reconfiguration. Bus/voltage clock qualification comes from
the HAL RCC initialization; this API adds no undocumented clock source or rate.

## Authored/generated boundary and verification

Register enums and START command/status descriptions are in the authored AES and
TRNG YAML. Whole-IR fingerprints are updated in `register-reuse.json`. Geometry
and qualified capabilities live in `cw32-data/crypto.yaml`; the build script
validates the selected metadata/RCC relationships and emits the geometry used by
the drivers. No raw register adapters, shadow bit masks, path attributes, HAL
cfg prefixes, or one-file module directories are introduced. Existing shared
schema types and fixtures are unchanged.

`tests/verify_l083_crypto_evidence.py` validates only source, typed IR, member
provenance and metadata. It neither runs nor models HAL hardware. Ordinary
optimized thumbv6m library builds and genuine firmware examples provide compile
and link evidence. `examples/l083/crypto` contains a hardware word-round-trip
application and a raw TRNG sampling application. Source audits, compilation and
linking do not establish silicon correctness, byte interoperability or entropy.
