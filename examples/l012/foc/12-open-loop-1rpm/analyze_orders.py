"""Fit 66/rev and 264/rev speed ripple to a complete RTT run, using mechanical angle."""
import math
import re
import sys
from collections import Counter
from pathlib import Path

COUNTS = 16384
ORDERS = (66, 264)


def fit(samples, direction):
    # Equal total weight per occupied mechanical angle bin so stalls do not dominate.
    observations = []
    for (t0, a), (t1, b) in zip(samples, samples[1:]):
        if t1 - t0 != 10 or max(a, b) >= COUNTS:
            continue
        delta = (b - a + COUNTS // 2) % COUNTS - COUNTS // 2
        turns = ((a + delta / 2) % COUNTS) / COUNTS
        theta = math.tau * turns
        speed = direction * delta * 6000 / COUNTS
        basis = [1.0]
        for order in ORDERS:
            basis.extend((math.cos(order * theta), math.sin(order * theta)))
        observations.append((int(turns * 2048), speed, basis))

    occupancy = Counter(row[0] for row in observations)
    size = 1 + 2 * len(ORDERS)
    matrix = [[0.0] * (size + 1) for _ in range(size)]
    for bin_id, speed, basis in observations:
        for i, x in enumerate(basis):
            for j, y in enumerate([*basis, speed]):
                matrix[i][j] += x * y / occupancy[bin_id]
    # Solve the five normal equations with partial pivoting.
    for i in range(size):
        pivot = max(range(i, size), key=lambda k: abs(matrix[k][i]))
        matrix[i], matrix[pivot] = matrix[pivot], matrix[i]
        scale = matrix[i][i]
        if abs(scale) < 1e-10:
            raise ValueError("insufficient angle coverage")
        matrix[i] = [x / scale for x in matrix[i]]
        for j in range(size):
            if j != i:
                scale = matrix[j][i]
                matrix[j] = [a - scale * b for a, b in zip(matrix[j], matrix[i])]
    coeff = [row[-1] for row in matrix]
    mean_rpm = sum(row[1] for row in observations) / len(observations)
    print(f"intervals={len(observations)} bins={len(occupancy)}/2048 mean_abs_direction_rpm={mean_rpm:.4f}")
    peaks = {}
    for i, order in enumerate(ORDERS):
        a, b = coeff[1 + 2*i:3 + 2*i]
        peaks[order] = math.hypot(a, b)
        print(f"  {order}/rev peak_rpm={peaks[order]:.4f} phase_deg={math.degrees(math.atan2(b,a)):.2f}")
    return peaks


def main(path):
    log = Path(path).read_text()
    samples = []
    for end, values in re.findall(r'end_ms=(\d+) mech=\[([\d, ]+)\]', log):
        block = [int(x) for x in values.split(',')]
        if len(block) != 20:
            raise ValueError("incomplete encoder block")
        samples.extend((int(end) - 190 + 10*i, a) for i, a in enumerate(block))
    deadtime_ab = "deadtime A/B:" in log
    laps = 4 if deadtime_ab else 2
    if [t for t, _ in samples] != list(range(10, laps * 60_000 + 1, 10)) or 'test complete: bridge off' not in log:
        raise ValueError("incomplete test or missing RTT block")
    print("invalid encoder samples:", sum(a >= COUNTS for _, a in samples))
    peaks = []
    for lap in range(laps):
        direction = 1 if lap % 2 == 0 else -1
        start = lap * 60_000
        leg = [(t, a) for t, a in samples if start <= t <= start + 60000]
        label = "forward" if direction == 1 else "reverse"
        if deadtime_ab:
            label += " compensation=" + ("on" if lap >= 2 else "off")
        print(label)
        peaks.append(fit(leg, direction))
    if deadtime_ab:
        for i, direction in enumerate(("forward", "reverse")):
            before, after = peaks[i][66], peaks[i + 2][66]
            if before > 0:
                print(f"{direction} 66/rev reduction={(1 - after / before) * 100:.2f}%")


if __name__ == "__main__":
    main(sys.argv[1])
