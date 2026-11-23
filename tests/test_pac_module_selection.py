#!/usr/bin/env python3
"""Verify normal shared PAC modules and exact chip-selected compilation inputs.

This validates generator output and runs its ordinary build script for every chip.
It does not add HAL tests or execute firmware.
"""
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]
PAC = ROOT / 'cw32-metapac'
features = tomllib.loads((PAC / 'Cargo.toml').read_text())['features']
chips = sorted(name for name, deps in features.items() if name.startswith('cw32') and not deps)
groups = {}
for path in (ROOT / 'cw32-data/data/package-groups').glob('*.json'):
    groups.update(json.loads(path.read_text()))
expected = {}
for path in sorted((ROOT / 'cw32-data/data/chips').glob('*.json')):
    chip = json.loads(path.read_text())
    for core in chip['cores']:
        name = chip['name'].lower()
        if len(chip['cores']) != 1:
            name += '-' + core['name'].lower()
        aliases = {}
        for peripheral in core['peripherals']:
            if registers := peripheral.get('registers'):
                kind = registers['kind']
                source = kind + '_' + registers['version']
                assert kind not in aliases or aliases[kind] == source, (name, kind)
                aliases[kind] = source
        expected[name] = aliases
assert sorted(expected) == chips
all_versions = {source for aliases in expected.values() for source in aliases.values()}

# Only meaningful groups declare modules. Versioned leaves remain single copies.
for group in ['peripherals', 'registers']:
    source = PAC / 'src' / group
    declarations = re.findall(r'#\[cfg\((\w+)\)\]\s*pub mod (\w+);', (source / 'mod.rs').read_text())
    assert all(predicate == module for predicate, module in declarations), group
    assert len(declarations) == len(all_versions), group
    assert {module for _, module in declarations} == all_versions, group
    assert {path.stem for path in source.glob('*.rs') if path.name != 'mod.rs'} == all_versions, group
    assert 'include!' not in (source / 'mod.rs').read_text(), group
root = (PAC / 'src/lib.rs').read_text()
for feature, group in [('pac', 'peripherals'), ('metadata', 'registers')]:
    assert re.search(r'#\[cfg\(feature = "' + feature + r'"\)\]\s*mod ' + group + ';', root)
for path in PAC.rglob('*.rs'):
    assert not re.search(r'#\s*\[\s*(?:path\s*=|cfg_attr\([^\]]*\bpath\s*=)', path.read_text()), path

# The public chip-level PAC and metadata aliases retain their original names.
for chip, aliases in expected.items():
    directory = PAC / 'src/chips' / chip
    actual = {alias: source for source, alias in re.findall(
        r'pub use crate::peripherals::(\w+) as (\w+);', (directory / 'pac.rs').read_text())}
    assert actual == aliases, (chip, actual, aliases)
    metadata = (directory / 'metadata.rs').read_text()
    shared = re.search(r'include!\("\.\./([^"/]+\.rs)"\);', metadata)
    assert shared, chip
    actual = {alias: source for source, alias in re.findall(
        r'pub use crate::registers::(\w+) as (\w+)_regs;',
        (PAC / 'src/chips' / shared[1]).read_text())}
    assert actual == aliases, (chip, actual, aliases)

# Exercise the actual build script, not a Python recreation of its selection.
# The generated data module is resolved by Rust's ordinary module lookup.
base_env = {key: value for key, value in os.environ.items() if not key.startswith('CARGO_FEATURE_CW32')}
base_env['CARGO_MANIFEST_DIR'] = str(PAC)
with tempfile.TemporaryDirectory(prefix='cw32-module-selection-') as temp:
    for runtime in [False, True]:
        binary = Path(temp) / ('build-rt' if runtime else 'build')
        command = ['rustc', '--edition=2024', str(PAC / 'build.rs'), '-o', str(binary)]
        if runtime:
            command += ['--cfg', 'feature="rt"']
        subprocess.run(command, check=True)
        for chip, aliases in expected.items():
            env = base_env | {'CARGO_FEATURE_' + chip.upper().replace('-', '_'): '1'}
            result = subprocess.run([str(binary)], env=env, text=True, capture_output=True, check=True)
            cfgs = re.findall(r'^cargo:rustc-cfg=(\w+)$', result.stdout, re.MULTILINE)
            checked = re.findall(r'^cargo:rustc-check-cfg=cfg\((\w+)\)$', result.stdout, re.MULTILINE)
            assert len(cfgs) == len(set(aliases.values())) and set(cfgs) == set(aliases.values()), chip
            assert len(checked) == len(all_versions) and set(checked) == all_versions, chip
            assert f'cargo:rustc-env=CW32_METAPAC_PAC_PATH=chips/{chip}/pac.rs\n' in result.stdout
            assert f'cargo:rustc-env=CW32_METAPAC_METADATA_PATH=chips/{chip}/metadata.rs\n' in result.stdout
            assert ('cargo:rustc-link-search=' in result.stdout) == runtime
        for group, members in groups.items():
            for selection in [[group, members[0]], members, [group, *members]]:
                env = base_env | {'CARGO_FEATURE_' + name: '1' for name in selection}
                result = subprocess.run([str(binary)], env=env, text=True, capture_output=True, check=True)
                assert f'CW32_METAPAC_PAC_PATH=chips/{members[0].lower()}/pac.rs' in result.stdout
        for selection, message in [([], 'No cw32xx Cargo feature enabled'), (chips[:2], 'Multiple cw32xx Cargo features enabled')]:
            env = base_env | {'CARGO_FEATURE_' + chip.upper().replace('-', '_'): '1' for chip in selection}
            result = subprocess.run([str(binary)], env=env, text=True, capture_output=True)
            assert result.returncode != 0 and message in result.stderr, (selection, result)
print(f'PASS normal PAC modules: {len(chips)} chips, {len(all_versions)} shared IP versions, PAC/metadata aliases, exact selected cfgs with/without rt, zero/multiple chip rejection')
