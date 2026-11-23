#!/usr/bin/env python3
"""Verify generated L010 low-bypass metadata, real RCC admission, and ELF links.

Run ./d gen-all first. No board or MMIO execution is performed.
"""
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CHIPS = ['cw32l010f8p6', 'cw32l010f8u6', 'cw32l010y8m6']


def main():
    out = ROOT / 'build/acceptance'
    out.mkdir(parents=True, exist_ok=True)
    results = []
    for chip in CHIPS:
        data = json.loads((ROOT / f'cw32-data/data/chips/{chip.upper()}.json').read_text())
        sysctrl = next(p for p in data['cores'][0]['peripherals'] if p['name'] == 'SYSCTRL')
        hse = sysctrl['clock_limits']['hse']
        assert hse['bypass']['minimum_hz'] == 3_908_159
        assert hse['crystal']['minimum_hz'] == 4_000_000
        # Check the generated PAC metadata used by HAL build.rs, not just JSON.
        metadata = ROOT / f'cw32-metapac/src/chips/{chip}/metadata.rs'
        included = re.search(r'include!\("([^\"]+)"\)', metadata.read_text()).group(1)
        common = (metadata.parent / included).read_text()
        assert re.search(r'minimum_hz:\s*3908159\b', common)
        minimum = hse['bypass']['minimum_hz']
        count = (hse['ccs_numerator_hz'] + minimum - 1) // minimum
        assert count <= hse['ccs_maximum_count']
        assert minimum * count > hse['ccs_cycle_count'] * hse['ccs_lsi_maximum_hz']
        assert (hse['ccs_numerator_hz'] + minimum - 2) // (minimum - 1) > hse['ccs_maximum_count']
        commands = [['cargo', 'test', '--offline', '--locked', '--manifest-path',
                     'firmware/Cargo.toml', '-p', 'embassy-cw32', '--no-default-features',
                     '--features', chip, '--test', 'l010_bypass']]
        for retained in [False, True]:
            selected = chip + ',l010-low-hse' + (',retained-hse' if retained else '')
            commands.append(['cargo', 'build', '--offline', '--locked', '--release',
                '--manifest-path', 'examples/shared/hse-clock/Cargo.toml', '--no-default-features',
                '--features', selected, '--bin', 'bypass-low', '--target', 'thumbv6m-none-eabi'])
        for index, command in enumerate(commands):
            log = out / f'l010-bypass-{chip}-{index}.log'
            with log.open('w') as stream:
                code = subprocess.call(command, cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT)
            results.append(dict(chip=chip, command=command, exit_code=code,
                                log=str(log.relative_to(ROOT))))
            (out / 'l010-bypass.json').write_text(json.dumps(results, indent=2) + '\n')
            print(chip, index, 'PASS' if code == 0 else 'FAIL', flush=True)
    return any(row['exit_code'] for row in results)


if __name__ == '__main__':
    raise SystemExit(main())
