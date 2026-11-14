# Bounded buffered-ATIM complementary PWM

This source-qualified subset covers L010, L011 and L012. The separate
[classic F030/A030 scope](classic-atim-complementary-pwm.md) uses its own CR/DTR
register path and makes different API and behavior guarantees. It uses the actual
pinned Embassy `timer::complementary_pwm::{ComplementaryPwm,
ComplementaryPwmPin}` organization and controller methods: `enable`, `disable`,
`set_duty`, `get_duty`, `get_max_duty`, `get_frequency`,
`set_master_output_enable` and `get_master_output_enable`. The pinned revision is
`f16efeffe37581092ec184718e6fdb1620393214`. CW32 extensions add checked construction,
owned `BreakInput`, constructor-only dead time in nanoseconds, source-qualified
bounds, and explicit BK1 polling/acknowledgement. It does not promise API parity
with STM32's larger set of modes.

Each owner retains the whole ATIM token, every supplied output token and the
optional BK1 token. RCC gate/reset remains in central RCC_INFO through the
existing `low_level::Timer`. No GTIM time-driver resource is reused. Generated
`atim_complementary` metadata creates the sealed capability and its 1008-tick
limit; generated pin traits accept only qualified, bonded AF cells.

## Configuration and updates

`ConfigError::UnqualifiedCycleTiming` rejects rate-only PCLK before frequency
selection, strict dead-time selection or timer initialization, including a
zero-nanosecond request. This includes exact-L010 live LSI. Rejection does not
enable/reset ATIM, change BDTR/AF1, connect outputs or start the counter. Earlier
`PwmPin`, `ComplementaryPwmPin` and `BreakInput` construction may already have
used `Flex::new` to disconnect GPIO pads; the error consumes those wrappers and
does not restore their prior state. `new` returns `Result<Self, ConfigError>`
when construction succeeds or is rejected.

Construction validates the requested frequency/dead time before touching ATIM.
The existing timer initialization resets/stops the timer and clears CCER/MOE,
selects forced-low main references, enables compare/ARR preloads, commits one
update and leaves LOCK=0. LOCK is write-once after reset: that initial zero stays
zero, and this API does not claim to install a nonzero protection lock afterward.

While every output gate and MOE remain disabled, construction writes DTG,
OSSI=OSSR=1, BKP=1, AOE=0 and BKF=0. BKE is set only when a `BreakInput` is
supplied; with no routed BK1 pad the break source stays disabled, matching the
reference C initialization and preventing an unrelated package/system source
from clearing MOE. The constructor then performs an APB read to complete the
documented one-cycle BKE/BKP write delay. AF1 enables only the supplied BK1 pin,
with BKINP selecting its active level. Comparator routes and BK2 are disabled by
timer initialization. Already configured system-level break requests are not
owned or changed by this driver and can still assert BK1 when BKE is enabled. The
input AF is connected before output AFs. The counter then starts; all pair gates
and MOE remain disabled. Users explicitly enable pairs and then explicitly enable
MOE.

DTG uses the own manuals' four ranges: 0–127 PCLK cycles, 128–254 in steps of two,
256–504 in steps of eight, and 512–1008 in steps of sixteen. CKD stays /1, so the
counter prescaler never scales dead time. The constructor picks the first code
whose conservatively rounded minimum duration meets the request, using exact
RCC ClockBounds arithmetic. Unrepresentable requests fail rather than saturate.
The reported interval uses outward rounding at both clock-envelope endpoints.

Interior duty changes use CCR preload and commit on update. Endpoint transitions
clear global MOE, stop and restart the whole period, commit every pending CCR,
and leave MOE disabled until the application explicitly enables it. Thus these
operations never restore a stale cached MOE after a hardware break. Endpoints
use forced reference modes and accept a full period of 65536 without truncation.
Runtime changes have no glitch-free, phase-continuity or minimum-pulse guarantee.

## Break and output limits

Unfiltered BK1 is asynchronous according to each own manual. An asserted source
clears MOE and prevents both MOE setting and BIF acknowledgement. AOE=0 prevents
automatic restart. `get_master_output_enable` reads hardware, while
`is_break_pending` reads a coalescing event flag, not the live pad. `clear_break`
writes zero only to BIF and one to every other defined ICR flag, keeping reserved
bits zero, with no ISR/ICR read-modify-write. It does not re-enable outputs or
implement an atomic fault-clear/rearm protocol. Software BG generates BK1.

CCyP/CCyNP=0, OISy/OISyN=0 and OSSI=OSSR=1 are fixed documented register choices.
No board-level shutdown voltage, fault latching, motor safety or power-stage
qualification is claimed. Pair disable drops both channel gates. Drop clears
MOE, disables all gates, stops counting, disconnects all owned pins, then lets
the timer owner disable requests and its RCC gate. No pad level after disable,
break or disconnect is promised by this API.

## Qualified routes and deferred modes

Own PDFs, package grids and exact SDK macros qualify five L010 complementary
routes, twelve L011 complementary/BK routes, and nineteen L012 routes. L010's
external BK and CH4N pads are withheld because they need oscillator/debug
ownership; CH4N therefore has no constructible pin on L010. L011 and L012 expose
CH1N–4N and qualified BK1 pins. Exact packages retain only their bonded routes.
SDK `ATIMBKIN` is explicitly normalized to the manuals' `ATIM_BK` signal.

Classic ATIM A/B complementary behavior, CH5/6, BK2, comparator/system-source
configuration, break filters, output inversion, nonzero LOCK, automatic restart,
asymmetric/preloaded or runtime dead time, runtime frequency, center alignment,
DMA and interrupts remain unimplemented. RF is deferred. Ordinary production
ARM builds and external-pin firmware links do not establish silicon behavior.

`atim-complementary-evidence.json` records source URLs, revisions, SHA256 hashes,
PDF page indices, register sections and bounded facts. The separate route evidence
contains authored cell coordinates and qualifications, not vendor source or
rendered tables. Reproduce with `python ci/verify-atim-complementary-data.py
--sources /path/to/official/sources`, then `./d gen-all`. Source PDFs and SDKs
remain outside the release tree.
