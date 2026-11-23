#!/usr/bin/env python3
"""Local HAL acceptance; source audits and hardware checks are separate.

Run after ./d gen-all. Requires Python 3.11+, cached Cargo dependencies and the
thumbv6m-none-eabi target. Reports commands and exit codes under build/acceptance.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = '''shared/uart-lin shared/uart-buffered shared/uart-idle shared/uart-ninebit shared/uart-half-duplex
shared/uart-flow-control shared/classic-i2c l012/i2c shared/classic-adc-scan shared/adc-scan l012/adc-scan
l012/analog shared/l012-wwdt l012/halltim shared/comparator shared/comparator-reference shared/classic-timer-input
shared/classic-atim-input shared/timer-input shared/low-power-timers'''.split()
HOSTS = '''cw32a030c8t7 cw32f002f3p7 cw32f003e4p7 cw32f020c6u7 cw32f030c8t7
cw32l010f8p6 cw32l011k8t6 cw32l012c8t6 cw32l031c8t6 cw32l052c8t6
cw32l083mct6 cw32r031c8u6'''.split()


def features(manifest):
    return tomllib.loads((ROOT / manifest).read_text())['features']


def commands(stage):
    if stage == 'host':
        for chip in HOSTS:
            yield chip, ['cargo', 'test', '--offline', '--locked', '--manifest-path',
                         'firmware/Cargo.toml', '-p', 'embassy-cw32',
                         '--no-default-features', '--features', chip, '--lib', '--test', 'lin']
    elif stage == 'hal':
        for chip in features('embassy-cw32/Cargo.toml'):
            if not chip.startswith('cw32'):
                continue
            timer = 'gtim' if chip.startswith(('cw32f002', 'cw32f003')) else 'gtim1'
            yield chip, ['cargo', 'build', '--offline', '--locked', '--manifest-path',
                         'firmware/Cargo.toml', '-p', 'embassy-cw32', '--no-default-features',
                         '--features', f'{chip},rt,defmt,time-driver-{timer}',
                         '--target', 'thumbv6m-none-eabi']
    else:
        for directory in EXAMPLES:
            example = Path(directory).name
            manifest = f'examples/{directory}/Cargo.toml'
            for chip in features(manifest):
                if not chip.startswith('cw32'):
                    continue
                selected = [chip]
                if example == 'uart-lin':
                    selected.append('timed')
                if example == 'uart-buffered' and chip.startswith(('cw32l010', 'cw32l011', 'cw32l012')):
                    selected.append('idle-notify')
                if example == 'classic-adc-scan':
                    selected.append('async-gtim' if chip.startswith(('cw32f002', 'cw32f003')) else 'async-gtim1')
                if example == 'adc-scan':
                    selected.append('async')
                label = directory.removeprefix('shared/').replace('/', '-')
                yield f'{label}-{chip}', ['cargo', 'build', '--offline', '--locked',
                     '--manifest-path', manifest, '--no-default-features', '--features',
                     ','.join(selected), '--target', 'thumbv6m-none-eabi', '--bins', '--release']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('stage', choices=['host', 'hal', 'examples'])
    args = parser.parse_args()
    out = ROOT / 'build/acceptance'
    out.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    if args.stage == 'examples':
        env.update(CARGO_TARGET_DIR=str(ROOT / 'target/acceptance-examples'), CARGO_INCREMENTAL='0')
    results = []
    for name, command in commands(args.stage):
        log = out / f'{args.stage}-{name}.log'
        with log.open('w') as stream:
            code = subprocess.call(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT)
        results.append(dict(name=name, command=command, exit_code=code, log=str(log.relative_to(ROOT))))
        (out / f'{args.stage}.json').write_text(json.dumps(results, indent=2) + '\n')
        print(f'{name}: {"PASS" if code == 0 else "FAIL"}', flush=True)
    failures = sum(r['exit_code'] != 0 for r in results)
    print(f'{len(results)} commands, {failures} failures')
    return bool(failures)


if __name__ == '__main__':
    raise SystemExit(main())
