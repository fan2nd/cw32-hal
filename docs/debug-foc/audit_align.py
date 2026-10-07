"""Audit raw same-phase endpoints against switched RL; bypass transport and PLL."""
import argparse
import json
import math
import re
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("log", type=Path)
parser.add_argument("--first-blank", type=int, default=480)
parser.add_argument("--second-blank", type=int, default=240)
parser.add_argument("--resistance", type=float, default=3.35)
parser.add_argument("--inductance-uh", type=float, default=875)
args = parser.parse_args()
text = args.log.read_text(encoding="utf-8")
offset = int(re.search(r"REPLAY_MOTOR .*?offset=(\d+)", text)[1])
pattern = r"REPLAY_ROW n=(\d+) age=(\d+) raw=\[(\d+), (\d+)\] duty=\[(\d+), (\d+), (\d+)\] vdda=(\d+) bus=(\d+)"
rows = [tuple(map(int, match)) for match in re.findall(pattern, text)]
if len(rows) != 80 or [row[0] for row in rows] != list(range(80)):
    raise ValueError("Expected exactly 80 ordered records")
if [row[1] for row in rows] != list(range(rows[0][1], rows[0][1] + 80)):
    raise ValueError("Missing or reordered age")
if "REPLAY_END records=80 complete=true" not in text:
    raise ValueError("Missing complete trailer")

# SI units, ideal pole switching. These residuals include model and analog errors
# and possible rotor motion; they are not independent measurements of actual EMF.
resistance, inductance, clock, period = args.resistance, args.inductance_uh * 1e-6, 96e6, 24000
rate = resistance / inductance
endpoints = [[] for _ in range(3)]
for n, age, raw0, raw1, da, db, dc, vdda, bus in rows:
    duty = [da, db, dc]
    order = sorted(range(3), key=duty.__getitem__)
    for channel, phase, blank, raw, sign in (
        (0, order[0], args.first_blank, raw0, -1),
        (1, order[2], args.second_blank, raw1, 1),
    ):
        hold = duty[order[channel]] + blank + 140
        current = sign * (raw - offset) * vdda / 4095 / 100
        endpoints[phase].append((n * period + hold, current, channel))

result = []
for phase, samples in enumerate(endpoints):
    for (start, i0, ch0), (end, i1, ch1) in zip(samples, samples[1:]):
        response = 0.0
        for frame in range(start // period, end // period + 1):
            row = rows[frame]
            duty, bus = row[4:7], row[8] / 1000
            origin = frame * period
            cuts = sorted({start, end, max(start, origin), min(end, origin + period)} | {
                origin + value for value in duty if start < origin + value < end
            })
            for a, b in zip(cuts, cuts[1:]):
                if a < origin or b > origin + period or b <= a:
                    continue
                high = [int((a + b) / 2 - origin < value) for value in duty]
                voltage = bus * (high[phase] - sum(high) / 3)
                response += voltage * (math.exp(-rate * (end - b) / clock) - math.exp(-rate * (end - a) / clock))
        decay = math.exp(-rate * (end - start) / clock)
        emf = (response + resistance * (decay * i0 - i1)) / (1 - decay)
        result.append({"phase": phase, "channels": f"{ch0}->{ch1}", "start_ticks": start, "end_ticks": end, "emf_mv": emf * 1000})

summary = {"records": len(rows), "intervals": len(result), "first_blank": args.first_blank, "second_blank": args.second_blank, "resistance": resistance, "inductance_uh": args.inductance_uh}
for phase in range(3):
    values = [r["emf_mv"] for r in result if r["phase"] == phase]
    summary[f"phase_{phase}"] = {"count": len(values), "mean_mv": sum(values) / len(values), "min_mv": min(values), "max_mv": max(values), "rms_mv": math.sqrt(sum(v * v for v in values) / len(values))}
differences = []
for phase in range(3):
    values = [r for r in result if r["phase"] == phase]
    for a, b in zip(values, values[1:]):
        # Compare adjacent one-frame intervals only; gaps include real rotor motion.
        if a["end_ticks"] - a["start_ticks"] < period * 1.5 and b["end_ticks"] - b["start_ticks"] < period * 1.5:
            differences.append(b["emf_mv"] - a["emf_mv"])
summary["adjacent_jump_rms_mv"] = math.sqrt(sum(v * v for v in differences) / len(differences))
summary["adjacent_jump_max_mv"] = max(map(abs, differences))
print(json.dumps(summary, indent=2))
args.log.with_suffix(".audit.json").write_text(json.dumps({"summary": summary, "intervals": result}, indent=2), encoding="utf-8")
