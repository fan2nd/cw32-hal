# L012 EAU and CORDIC: source and operation review

This implementation is a source-reviewed blocking HAL, not a silicon validation.
Only `cw32l012`, `cw32l012c8t6`, and `cw32l012c8u6` expose these modules. The generic
selection supports peripheral arithmetic but cannot link an application without
an independently known memory map. The two exact parts have genuine firmware
examples under `examples/l012/math`.

## Authorities

- CW32L012 User Manual CN Rev 1.4, dated 2026-06-03; EAU printed pp.146–153,
  CORDIC pp.154–161; AHB gate/reset pp.56,60; revision note p.672.
  PDF physical pages are printed page +26. Tables 12-1 on physical pp.182–183
  were rendered and visually inspected because PDF text extraction loses roots.
- CW32L012 Datasheet CN Rev 1.0: pp.10 and 21 confirm accelerator existence.
- CW32L012 Standard Peripheral Library V1.0.5: each consumed C/header/example
  is byte-compared with the official pinned ZIP member and recorded in the
  canonical evidence ledger. The SDK is corroboration, not a replacement manual.

Exact official URLs, versions, SHA-256 values, member chains and page references
are in `l012-math-evidence.json` and `sources/evidence-sources.json`. Generated
source catalog and reference-index views link back to those canonical entries.
No vendor PDFs, text extracts or SDK files are distributed in this patch.

## EAU operations

All MMIO is 32-bit; the manual explicitly says byte/halfword EAU accesses are
ignored. Data is delivered through the direct generated PAC's typed fields.

| API | Hardware mode | Input/result | Trigger | Completion/errors |
| --- | --- | --- | --- | --- |
| divide_unsigned | MODE=0 | u32 dividend/divisor → quotient/remainder | DIVIDEND then DIVISOR | BUSY=0; ZERO checked |
| divide_signed | MODE=1 | i32 dividend/divisor → quotient/remainder | DIVIDEND then DIVISOR | BUSY=0; ZERO/OVR checked |
| square_root | MODE=2 | u32 radicand → integer root/remainder | DIVIDEND only | BUSY=0; ZERO/OVR inapplicable |

Signed remainder has the dividend's sign or is zero; therefore quotient truncates
toward zero. Overflow (`i32::MIN / -1`) and division by zero are reported from
hardware, with no result-register reads. The source definitions give
`dividend = divisor*quotient + remainder` and `radicand = root² + remainder`.
No software divide, sqrt or fallback is used.

## CORDIC operations and exact numeric meanings

`Q1_31` carries signed bits interpreted as `bits / 2^31`, range [-1,1). No float
conversion is implicit. CSR.FORMAT selects Q1.31. CSR.COMP=0 requests hardware
K-factor compensation for modulus and sqrt; it is documented as inapplicable
otherwise. CSR.IE/DMAEOC/DMAIDLE are always disabled. Scalar writes trigger as
listed below; two-input functions write X first and trigger by writing Y.

| API | FUNC/SCALE | Input | Returned Q1.31 value(s) |
| --- | --- | --- | --- |
| sin_cos | COS/0 | z in [-1,1), angle z*pi | X=cos(z*pi), Y=sin(z*pi) |
| phase | ATAN2/0 | x>0, y in [-1,1) | Z=atan(y/x)/pi |
| hypot_half | HYPOT/0 | x,y in [-1,1) | X=sqrt(x²+y²)/2 |
| atan | ATAN/0..7 | y in [-1,1) | Z=atan(2^SCALE*y)/pi |
| sinh_cosh_half | COSH/1 | z in [-0.559,0.559] | X=cosh(2z)/2, Y=sinh(2z)/2 |
| atanh_half | ATANH/1 | y in [-0.403,0.403] | Z=atanh(2y)/2 |
| ln X2Over4 | LN/1 | x in [0.0535,0.5) | Z=ln(2x)/4 |
| ln X4Over2 | LN/2 | x in [0.25,0.75) | Z=ln(4x)/2 |
| ln X8Over2 | LN/3 | x in [0.375,0.875) | Z=ln(8x)/2 |
| ln X16Over4 | LN/4 | x in [0.4375,0.584) | Z=ln(16x)/4 |
| square_root X1 | SQRT/0 | x in [0.027,0.75) | X=sqrt(x) |
| square_root X2Over2 | SQRT/1 | x in [0.375,0.875) | X=sqrt(2x)/2 |
| square_root X4Over2 | SQRT/2 | x in [0.4375,0.585] | X=sqrt(4x)/2 |

Single-input trigger is Z for COS/COSH, Y for ATAN/ATANH, X for LN/SQRT.
Results are read only after BUSY=0 and EOC=1. Hardware clears EOC after result
read and at the next start. Reading both documented result registers of a pair
performs no new trigger. A subsequent new operation may discard unread outputs.

Convergence boundaries are rational source facts in `cw32-data/accelerators.yaml`.
The build generator converts them into exact inclusive bit bounds, rounding the
minimum up and an open maximum down to exclude the boundary; closed maxima are
floored. No floating-point rounding can admit an out-of-domain bit pattern.
Boundary checking happens before MMIO submission. The `phase` restriction x>0
avoids claiming undocumented atan2 branch-cut/zero-vector behavior: the table
calls the function atan2 but describes its output as atan(y/x).

## Source conflicts and explicit limits

Rev1.4 p.160 says ITER=n means n iterations, but the same explicit table maps
0→2, 1→4, …, 15→32. SDK 1.0.5 agrees with this table, while the older EN Rev1.0
formula was 2(n+1). The driver selects only the common explicit maximum encoding
15, exposes no iteration configuration and promises no duration or error bound.
The introductory 6–66 range is not used. Rev1.4 gives duration as iterations+2,
but a polling budget is deliberately not converted into cycles or time.

The SDK sqrt example selects COMP=1 while printing an ordinary sqrt result;
the current manual explicitly describes COMP=1 as uncompensated. This HAL
follows the manual and selects COMP=0, rather than copying that example.

Q1.31 cannot encode +1. The manuals do not specify rounding/saturation at exact
sin/cos unit endpoints, so the API returns hardware bits without correcting or
promising endpoint handling. Convergence qualification does not establish an
accuracy bound. Full-quadrant phase, Q1.15, DMA and automatic range extension are not exposed.
IRQ operation has its separate [async contract](cordic-async.md). Q1.15 requires
operand/result alignment in the 32-bit X/Y/Z registers and K-factor behavior
to be established: §12.3 defines signed 16-bit values, but §12.6.2–4 only lists
32-bit fields, while §12.6.1 limits COMP to Q1.31 hypot/sqrt. Selecting FORMAT=0
alone does not establish these contracts.

## Ownership, gates, reset and bounded behavior

Each driver retains a `Peri` owner, has a sealed instance trait and accesses its
direct typed PAC. CSR fields use own-source enums or bools; payloads are full
32-bit words. No raw HAL register mirror, address table or adapter is introduced.
The only gate/reset access is central RCC_INFO generated from existing verified
peripheral control metadata. No reset is pulsed or released.

Startup checks held reset, enables the gate with central key/neighbor preservation,
checks enable readback, and refuses busy hardware without touching operands or
CSR. Successful idle CORDIC startup disables inherited peripheral DMA/IRQ request
sources without writing an operand or starting an operation. A busy/clock-error startup can leave the gate enabled. Each call checks busy
before reconfiguring; zero polling budget starts nothing. A nonzero budget permits
one preflight busy read plus at most the given number of completion status reads.
Timeout means only that completion was not observed. It does not stop the engine,
prove quiescence, clear flags, discard ownership or read a result. A later call
may run once BUSY clears; timed-out output is not returned by an unrelated call.

Drop reads busy once. If busy, it retains the gate without waiting or resetting.
If idle, it requests gate disable via central policy, which retains any shared
group. No claimed abort/completion guarantee depends on the gate. External raw
PAC or DMA access must not violate the exclusive peripheral ownership contract.

## Verification boundary

The handoff includes exact normal ARM build and firmware-link logs. No HAL tests,
mock MMIO, register model or compile harness was added. Source/provenance and
layout validators are data/static checks. Hardware execution, numerical accuracy,
electrical behavior, completion-time measurements and stuck-busy recovery were
not tested on a device.
