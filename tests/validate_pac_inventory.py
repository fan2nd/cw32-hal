#!/usr/bin/env python3
"""Validate complete chip inventory/reference integrity, independent of Rust renderer."""
import json
import tomllib
from pathlib import Path
import yaml

from current_targets import is_retired, retired_targets
ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'cw32-data/data'
expected = {}
groups = {}
for file in sorted((ROOT / 'cw32-data/inputs').glob('*.yaml')):
    source = yaml.safe_load(file.read_text())
    if source.get('quarantine') or is_retired(source['line']):
        continue
    family_groups = source.get('firmware_targets', {})
    if family_groups:
        assert json.loads((DATA / 'package-groups' / (source['line'] + '.json')).read_text()) == family_groups
        groups.update(family_groups)
    chips = source['chips']
    if source.get('parts_catalog'):
        catalog_parts = yaml.safe_load((ROOT / source['parts_catalog']).read_text())['parts']
        chips = chips + [p for p in catalog_parts if p['family'] == source['line']
                         and p['name'] not in {c['name'] for c in chips}]
    for chip in chips:
        assert chip['name'] not in expected, f'duplicate chip {chip["name"]}'
        expected[chip['name']] = source
actual = {p.stem: p for p in (DATA / 'chips').glob('*.json')}
assert set(actual) == set(expected), {'unexpected': sorted(actual.keys()-expected.keys()), 'missing': sorted(expected.keys()-actual.keys())}
features = tomllib.loads((ROOT / 'cw32-metapac/Cargo.toml').read_text())['features']
assert {name for name in features if name.startswith('cw32')} == {name.lower() for name in expected.keys() | groups.keys()}
hal = tomllib.loads((ROOT / 'embassy-cw32/Cargo.toml').read_text())['features']
for group, members in groups.items():
    assert features[group.lower()] == [members[0].lower()]
    assert hal[group.lower()] == [f'cw32-metapac/{group.lower()}']
    assert all(member in actual for member in members)
instances = total_interrupts = register_templates = 0
for name, path in actual.items():
    chip = json.loads(path.read_text())
    assert chip['name'] == name
    assert chip['cores']
    for variant in chip['memory']:
        spans = []
        for memory in variant:
            start, size = memory['address'], memory['size']
            assert 0 <= start < 2**32 and 0 < size <= 2**32 - start
            assert all(start + size <= low or start >= high for low, high in spans)
            spans.append((start, start+size))
    for core in chip['cores']:
        assert 1 <= core['nvic_priority_bits'] <= 8
        interrupts = {i['name']: i['number'] for i in core['interrupts']}
        assert len(interrupts) == len(core['interrupts'])
        assert len(set(interrupts.values())) == len(interrupts)
        limit = 32 if core['name'] == 'cm0p' else 240
        assert all(0 <= number < limit for number in interrupts.values())
        names = [p['name'] for p in core['peripherals']]
        assert len(set(names)) == len(names)
        pins = [p['name'] for p in core['pins']]
        assert len(set(pins)) == len(pins)
        for p in core['peripherals']:
            assert 0 <= p['address'] < 2**32
            for interrupt in p.get('interrupts', []):
                assert interrupt['interrupt'] in interrupts
            reg = p.get('registers')
            if reg:
                file = DATA / 'registers' / f'{reg["kind"]}_{reg["version"]}.json'
                ir = json.loads(file.read_text())
                assert 'block/' + reg['block'] in ir, (name, p['name'])
        instances += len(core['peripherals'])
        total_interrupts += len(interrupts)
    pac = ROOT / 'cw32-metapac/src/chips' / name.lower()
    for file in ['pac.rs', 'metadata.rs', 'device.x']:
        assert (pac / file).is_file(), (name, file)
register_templates = len(list((DATA / 'registers').glob('*.json')))
print(json.dumps({
    'chip_features': len(expected),
    'package_neutral_groups': len(groups),
    'chip_peripheral_instances': instances,
    'chip_interrupt_entries': total_interrupts,
    'register_templates': register_templates,
    'result': 'all inventory and references valid; not a hardware validation',
}, indent=2))
