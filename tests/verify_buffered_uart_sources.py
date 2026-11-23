#!/usr/bin/env python3
"""Check present own-source/data/capability facts for buffered UART reception.

This finite checker does not read HAL implementation text, model IRQ timing,
mutate fixtures, invoke other checkers, or compare historical HAL implementation
hashes. Immutable evidence and the current declaration baseline are hash-pinned;
those fingerprints are not runtime or implementation equivalence proof.
Runtime lifecycle and hardware behavior require separate review/evidence.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import tomllib
from urllib.parse import urlparse

import yaml
from current_targets import historical_chip_path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = 'docs/uart-buffered-rx-evidence.json'
CHECKER = 'tests/verify_buffered_uart_sources.py'
FAMILIES = {'CW32A030', 'CW32F002', 'CW32F003', 'CW32F020', 'CW32F030',
            'CW32L010', 'CW32L011', 'CW32L012', 'CW32L031', 'CW32L052',
            'CW32L083', 'CW32R031', 'CW32W031'}
NATIVE = {'CW32L010', 'CW32L011', 'CW32L012'}
VERSIONS = {'v1', 'cw32f002_v1', 'cw32l010_v1', 'cw32l012_v1',
            'cw32l031_v1', 'cw32l052_v1', 'cw32l083_v1'}
EXAMPLES = {('cw32f030c8t7', 'receive'), ('cw32f002f3p7', 'receive'),
            ('cw32l031c8t6', 'split'), ('cw32l052c8t6', 'rx_only'),
            ('cw32l083rct6', 'receive'), ('cw32l010f8p6', 'flow_control'),
            ('cw32l011k8t6', 'rx_only'), ('cw32l012c8t6', 'split'),
            ('cw32l083rct6', 'shared_vector')}
EMBASSY_REV = 'f16efeffe37581092ec184718e6fdb1620393214'


def read_json(path):
    return json.loads(historical_chip_path(ROOT / path).read_text())


def read_yaml(path):
    return yaml.safe_load((ROOT / path).read_text())


def read_toml(path):
    return tomllib.loads((ROOT / path).read_text())


def named(entries, key='name'):
    result = {entry[key]: entry for entry in entries}
    assert len(result) == len(entries), ('duplicate metadata identity', key)
    return result


def fields(registers, name):
    entries = registers['fieldset/' + name]['fields']
    assert len(named(entries)) == len(entries)
    return {f['name']: [f['bit_offset'], f['bit_size']] for f in entries}


def compact(text):
    return re.sub(r'\s+', '', text)


def macro(header, name):
    match = re.search(r'^#define\s+' + re.escape(name)
                      + r'\s+\(?((?:0x)?[0-9A-Fa-f]+)UL\)?', header, re.M)
    assert match, name
    return int(match[1], 16 if match[1].startswith('0x') else 10)


def page(pages, pair, family):
    printed, viewer = pair
    text = pages[viewer - 1]
    assert family in text and re.search(r'\b' + str(printed) + r'\b', text), (family, pair)
    return text


def register_section(pages, pair, family, register):
    # Native ISR and L012 IER continue past their first viewer page.
    first = page(pages, pair, family)
    text = first + '\f'.join(pages[pair[1]:pair[1] + 3])
    start = re.search(r'^\s*(\d+\.\d+\.\d+)\s+UARTx_' + register + r'\b', text, re.M)
    assert start, (family, register, pair)
    tail = text[start.end():]
    end = re.search(r'^\s*\d+\.\d+(?:\.\d+)*\s+\S', tail, re.M)
    return start[1], tail[:end.start()] if end else tail


def source_material(evidence, cache):
    assert evidence['identity_authority'] == 'sources/evidence-sources.json'
    assert evidence['checker'] == CHECKER
    index = read_json(evidence['identity_authority'])
    nodes, material = {}, {}
    for binding in evidence['sources']:
        ref = binding['source_ref']
        assert ref not in nodes, ref
        keys = binding['index_pointer'].strip('/').split('/')
        assert keys[:1] == ['artifacts'] and keys[1].isdigit(), ref
        node = index
        for key in keys:
            node = node[int(key)] if isinstance(node, list) else node[key]
        kind, path = ref.split(':', 1)
        assert kind in {'vendor', 'member', 'text'} and node['path'] == binding['path'] == path
        assert keys[2:] == ([] if kind == 'vendor' else ['text'] if kind == 'text'
                           else ['members', keys[-1]]), ref
        artifact = index['artifacts'][int(keys[1])]
        url = urlparse(artifact['url'])
        assert url.scheme == 'https' and url.hostname == 'www.whxy.com', ref
        assert artifact['url'] == binding['url'], ref
        assert node['sha256'] == binding['sha256'] and node['bytes'] == binding['bytes'], ref
        assert node['evidence'].count(EVIDENCE) == node['evidence'].count(CHECKER) == 1, ref
        if kind == 'member':
            assert node['members'] == binding['archive_member_chain'], ref
        data = (cache / path).read_bytes()
        assert len(data) == node['bytes'] and hashlib.sha256(data).hexdigest() == node['sha256'], ref
        nodes[ref] = node
        if kind != 'vendor':
            material[ref] = data.decode('utf-8', errors='replace')
    assert len(nodes) == 111
    # Both consumers must navigate back to exactly the forward-bound identities.
    reverse = {EVIDENCE: set(), CHECKER: set()}
    for artifact in index['artifacts']:
        entries = [('vendor', artifact), *(('member', m) for m in artifact.get('members', []))]
        if 'text' in artifact:
            entries.append(('text', artifact['text']))
        for kind, node in entries:
            for consumer in reverse:
                if consumer in node.get('evidence', []):
                    reverse[consumer].add(kind + ':' + node['path'])
    assert all(refs == set(nodes) for refs in reverse.values())
    return nodes, material


def register_contracts(evidence):
    contracts = evidence['register_contracts']
    assert set(contracts) == VERSIONS
    commands = read_yaml('cw32-data/register-writes.yaml')['registers']
    for version, contract in contracts.items():
        assert contract['project_yaml'] == f'cw32-data/registers/uart_{version}.yaml'
        assert contract['project_json'] == f'cw32-data/data/registers/uart_{version}.json'
        assert contract['generated_pac'] == f'cw32-metapac/src/peripherals/uart_{version}.rs'
        for path, loader in [(contract['project_yaml'], read_yaml), (contract['project_json'], read_json)]:
            registers = loader(path)
            items = named(registers['block/UART']['items'])
            for name, expected in contract['registers'].items():
                assert items[name]['byte_offset'] == expected['offset'], (path, name)
                assert items[name].get('access', 'ReadWrite') == expected['access'], (path, name)
                assert items[name].get('bit_size', 32) == 32, (path, name)
            for name, expected in contract['fields'].items():
                assert fields(registers, name) == expected, (path, name)
        assert contract['fields']['RDR'] == {'RDR': [0, 9]}
        assert {k: contract['registers'][k] for k in ['IER', 'ISR', 'ICR', 'RDR']} == {
            'IER': {'offset': 8, 'access': 'ReadWrite'}, 'ISR': {'offset': 28, 'access': 'Read'},
            'ICR': {'offset': 32, 'access': 'ReadWrite'}, 'RDR': {'offset': 36, 'access': 'Read'}}
        icr = contract['fields']['ICR']
        clearable = set(icr) - {'RFU'}
        mask = sum(((1 << width) - 1) << bit for name, (bit, width) in icr.items() if name in clearable)
        expected = ((0x1fff, 0x1ffe) if version in {'cw32l010_v1', 'cw32l012_v1'} else
                    (0x0fff, 0x0e5e) if version in {'cw32l031_v1', 'cw32l052_v1'} else
                    (0x00ff, 0x005e))
        assert (contract['icr_write_noop'], contract['icr_clearable_mask']) == expected
        assert mask == expected[1]
        command, = commands['uart_' + version]
        assert (command['block'], command['register'], command['fieldset']) == ('UART', 'ICR', 'ICR')
        assert command['write_noop'] == command['reset_value'] == expected[0]
        assert set(command['zero_to_clear_fields']) == set(contract['zero_to_clear_fields']) == clearable
        assert ('RFU' in icr) == (version == 'cw32l052_v1')
        if 'RFU' in icr:
            assert icr['RFU'] == [0, 1] and expected[0] & 1 and not mask & 1
        # Inspect the present typed command value, never a generated-file hash.
        generated = (ROOT / contract['generated_pac']).read_text()
        impl = re.search(r'impl regs::Icr\s*\{(.*?)\n\}', generated, re.S)
        assert impl, version
        for method in ['reset_value', 'write_noop']:
            value = re.search(r'fn ' + method + r'\(\) -> Self\s*\{\s*Self\((\d+)\)', impl[1])
            assert value and int(value[1]) == command[method], (version, method)


def family_sources(family, facts, evidence, nodes, material):
    sources = facts['sources']
    roles = {'sdk', 'cmsis_header', 'uart_header', 'uart_driver', 'gpio_header',
             'manual', 'manual_text', 'datasheet', 'datasheet_text'}
    assert set(sources) == roles | ({'rcc_header'} if family == 'CW32L083' else set())
    assert len(set(sources.values())) == len(sources) and set(sources.values()) <= nodes.keys()
    for role, ref in sources.items():
        authority = ('cw32f030' if family == 'CW32A030' and role not in
                     {'manual', 'manual_text', 'datasheet', 'datasheet_text'} else
                     'cw32x030' if family in {'CW32A030', 'CW32F030'} and role in
                     {'manual', 'manual_text'} else family.lower())
        assert authority in ref.lower(), (family, role, ref)
    for role in ['manual', 'datasheet']:
        assert nodes[sources[role]]['text']['path'] == nodes[sources[role + '_text']]['path']
    members = {member['path'] for member in nodes[sources['sdk']]['members']}
    assert all(nodes[ref]['path'] in members for role, ref in sources.items() if role.endswith('_header') or role == 'uart_driver')
    text = {role: material[ref] for role, ref in sources.items() if ref in material}
    pages = text['manual_text'].split('\f')
    receive = compact(''.join(page(pages, pair, family)
                              for pair in facts['manual_receive_pages_printed_then_viewer']))
    assert all(term in receive for term in ['UARTx_RDR', 'UARTx_ISR.RC', '硬件置位', '读取', '并清除', '覆盖', '数据丢失']), family
    native = family in NATIVE
    errors = {'ORE': [11, 1], 'PE': [9, 1], 'FE': [8, 1], 'NE': [10, 1]} if native else {'PE': [4, 1], 'FE': [3, 1]}
    assert facts['error_fields'] == errors and facts['pre_isr_overwrite_detectable'] is native
    if native:
        assert all(term in receive for term in ['UARTx_ISR.ORE', 'UARTx_ISR.NE', '还没有清零', '16倍采样或8倍采样时有效']), family
    contract = evidence['register_contracts'][facts['register_version']]
    for register, access in [('IER', 'RW'), ('ISR', 'RO'), ('ICR', 'R1W0'), ('RDR', 'RO')]:
        heading, section = register_section(pages, facts['manual_control_pages_printed_then_viewer'][register], family, register)
        actual = {}
        for high, low, name, permission in re.findall(r'^\s*(\d+)(?::(\d+))?\s+([A-Z][A-Z0-9]+)\s+(RW|RO|R1W0)\b', section, re.M):
            assert permission == access and name not in actual, (family, register, name)
            actual[name] = [int(low or high), int(high) - int(low or high) + 1]
        assert actual == {k: v for k, v in contract['fields'][register].items() if k != 'RFU'}, (family, register, actual)
        assert re.search(r'Address\s+offset:\s*0x0*' + format(contract['registers'][register]['offset'], 'x') + r'\b', section, re.I), (family, register)
        if register == 'ICR':
            assert heading == facts['manual_icr_section']
            assert f"Resetvalue:0x{contract['icr_write_noop']:08X}" in compact(section)
            assert 'W0：清除' in compact(section) and 'W1：无功能' in compact(section)
            assert '保留位，请保持默认值' in compact(section)
        if not native:
            assert not {'ORE', 'NE'} & actual.keys(), (family, register)
    driver = text['uart_driver'].splitlines()
    for line in facts['sdk_rdr_no_clear_lines']:
        assert all(term in compact(driver[line - 1]) for term in ['RC', '不能', 'RDR', '清除']), (family, line)
    for line in facts['sdk_rdr_read_lines']:
        assert 'return' in driver[line - 1] and '->RDR' in driver[line - 1], (family, line)
    for line in facts['sdk_icr_clear_lines']:
        assert 'REGBITS_CLR(' in driver[line - 1] and '->ICR' in driver[line - 1], (family, line)
    for flag, (bit, _) in {'RC': [2, 1], **errors}.items():
        for kind in ['IT', 'FLAG']:
            match = re.search(r'^#define\s+(?:USART|UART)_' + kind + '_' + flag + r'\s+.*?(0x[0-9a-fA-F]+)', text['uart_header'], re.M)
            assert match and int(match[1], 16) == 1 << bit, (family, kind, flag)
    prefix = 'UART' if family == 'CW32L012' else 'UARTx'
    for register in ['IER', 'ISR', 'ICR', 'RDR']:
        expected = contract['fields'][register]
        own_fields = set(re.findall(r'^#define\s+' + prefix + '_' + register + r'_(\w+)_Pos\b', text['cmsis_header'], re.M))
        assert own_fields == set(expected), (family, register, own_fields)
        for name, (bit, width) in expected.items():
            stem = f'{prefix}_{register}_{name}'
            assert macro(text['cmsis_header'], stem + '_Pos') == bit, (family, stem)
            assert macro(text['cmsis_header'], stem + '_Msk') == ((1 << width) - 1) << bit, (family, stem)
    for name in ['RXEN', 'OVER']:
        bit, width = contract['fields']['CR1'][name]
        assert macro(text['cmsis_header'], f'{prefix}_CR1_{name}_Pos') == bit
        assert macro(text['cmsis_header'], f'{prefix}_CR1_{name}_Msk') == ((1 << width) - 1) << bit
    if family == 'CW32L011':
        manual = nodes[sources['manual']]
        assert manual['url'] == 'https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf'
        provenance = manual['provenance']
        assert provenance['status'] == 'selected' and provenance['cover_date'] == '2026-06'
        assert re.search(r'2026\s*年\s*0?6\s*月', pages[0]), family
        previous = provenance['previous_snapshot']
        assert previous['cover_date'] == '2025-09' and previous['sha256'] != manual['sha256']
        assert previous['status'] == 'superseded, not current audit input'
    if family == 'CW32A030':
        alias, = read_yaml('cw32-data/register-source-aliases.yaml')['aliases']
        assert (alias['source_line'], alias['target_line']) == ('CW32F030', 'CW32A030')
        assert alias['allow_register_map_reuse'] is True and alias['independent_svd_available'] is False
        assert {r.get('source_ref') for r in alias['evidence']} >= {sources['manual'], sources['datasheet']}
    return text


def family_metadata(family, facts, evidence, nodes, text, capabilities, features, parts):
    instances = named(facts['instances'])
    expected_count = 6 if family == 'CW32L083' else 2 if family in {'CW32F002', 'CW32F003', 'CW32L010'} else 3
    assert set(instances) == {f'UART{i}' for i in range(1, expected_count + 1)}
    cmsis = text['cmsis_header']
    irqs = {name: int(number) for name, number in re.findall(r'\b(UART\d(?:_UART\d)?)_IRQn\s*=\s*(\d+)\b', cmsis)}
    assert irqs == {i['vector']: i['irq_number'] for i in instances.values()}, family
    inputs = read_yaml(f'cw32-data/inputs/{family.lower()}.yaml')
    version = facts['register_version']
    assert inputs['register_versions']['uart'] == version
    assert inputs['alias_pin_policy'] == facts['alias_pin_policy'] == 'common-package-intersection'
    assert inputs['af_metadata'] == facts['af_path'] and inputs['pinout_metadata'] == facts['pinout_path']
    af, pinout = read_yaml(facts['af_path']), read_yaml(facts['pinout_path'])
    sources = facts['sources']
    for binding in [af['datasheet'], pinout['source']]:
        own = nodes[sources['datasheet']]
        assert binding['source_ref'] == sources['datasheet'] and binding['sha256'] == own['sha256'] and binding['url'] == own['url']
    assert af['source']['header_sha256'] == nodes[sources['gpio_header']]['sha256']
    assert af['source']['sdk_sha256'] == nodes[sources['sdk']]['sha256']
    assert af['source']['sdk_url'] == nodes[sources['sdk']]['url']
    routes = [r for r in af['routes'] if r['peripheral'] in instances]
    assert len(routes) == facts['uart_route_cells']
    gpio_lines = text['gpio_header'].splitlines()
    for route in routes:
        if route['source_macro'] is None:
            assert family == 'CW32W031' and (route['pin'], route['af'], route['peripheral'], route['signal']) == ('PA2', 5, 'UART3', 'RX')
            assert route['source_kind'] == 'datasheet' and route['datasheet_cell']['function'] == 'UART3_RXD'
        else:
            line = gpio_lines[route['source_line'] - 1]
            assert route['source_macro'] + '()' in line, (family, route)
            assert re.search(r'=\s*' + str(route['af']) + r'\s*\)', line), (family, route)
            assert route['gpio_register'] + '_f.' + route['gpio_field'] in line, (family, route)
    packages = named(pinout['packages'])
    aliases = named(inputs['chips'])
    selections = set(facts['selections'])
    assert selections == {s.lower() for s in set(packages) | set(aliases)}
    # Source qualification keeps all 54 historical selections; current support
    # excludes exactly the two retired W031 names, never arbitrary missing files.
    active_selections = selections
    if family == 'CW32W031':
        assert selections == {'cw32w031', 'cw32w031r8u6'}
        active_selections = set()
        assert family not in capabilities['pin_policy']['excluded_by_family']
        assert facts['hal_excluded_pads'] == ['PA13', 'PA14', 'PF3']
    else:
        assert facts['hal_excluded_pads'] == capabilities['pin_policy']['excluded_by_family'][family]
    assert {p.stem.lower() for p in (ROOT / 'cw32-data/data/chips').glob(family + '*.json')} == active_selections
    assert {f for f in features if f.startswith(family.lower())} == active_selections
    exclusions = set(facts['hal_excluded_pads'])
    clock = named(read_yaml(inputs['clock_metadata'])['peripherals'])
    sys_version = inputs['register_versions']['sysctrl']
    sysctrl = read_json(f'cw32-data/data/registers/sysctrl_{sys_version}.json')
    projections = named(evidence['selection_projection'], 'selection')
    route_count = 0
    for selection in sorted(selections):
        chip = read_json(f'cw32-data/data/chips/{selection.upper()}.json')
        assert chip['line'] == family and chip['name'] == selection.upper()
        projection = projections[selection]
        assert projection['family'] == family and projection['current_chip_metadata'] == f'cw32-data/data/chips/{selection.upper()}.json'
        assert chip['memory'] == projection['memory']
        assert [{k: p[k] for k in ['name', 'package']} for p in chip['packages']] == projection['packages']
        if selection.upper() in packages:
            package = packages[selection.upper()]
            assert chip['packages'] == [{k: package[k] for k in ['name', 'package', 'pins']}]
            pads = set(package['gpio_pins'])
            assert chip['memory'] == [parts[selection.upper()]['memory']] == [[
                {'name': 'FLASH', 'kind': 'flash', 'address': 0, 'size': package['flash_bytes']},
                {'name': 'RAM', 'kind': 'ram', 'address': 0x20000000, 'size': package['ram_bytes']}]]
        else:
            assert chip['packages'] == [] and chip['memory'] == aliases[selection.upper()]['memory']
            matching = [set(p['gpio_pins']) for p in packages.values()
                        if selection.upper() == family or selection.upper() == p['family_part']]
            assert matching, selection
            pads = set.intersection(*matching)
            # Compatibility aliases have no invented physical package assignment.
            assert selection == family.lower() or (family == 'CW32F030' and selection in
                   {'cw32f030c8', 'cw32f030k8', 'cw32f030f8', 'cw32f030f6'})
        core, = chip['cores']
        peripherals, vectors = named(core['peripherals']), named(core['interrupts'])
        assert {name for name, p in peripherals.items() if p.get('registers', {}).get('kind') == 'uart'} == set(instances)
        assert peripherals['SYSCTRL']['registers']['version'] == sys_version
        for name, instance in instances.items():
            uart = peripherals[name]
            assert instance['source_cmsis'] == sources['cmsis_header']
            assert macro(cmsis, name + '_BASE') == uart['address'] == int(instance['base'], 16)
            assert instance['register_version'] == version
            assert uart['registers'] == {'block': 'UART', 'kind': 'uart', 'version': version}
            vector = instance['vector']
            assert uart['interrupts'] == [{'signal': 'GLOBAL', 'interrupt': vector}]
            assert vectors[vector]['number'] == instance['irq_number']
            users = {(p['name'], link['signal']) for p in core['peripherals']
                     for link in p.get('interrupts', []) if link['interrupt'] == vector}
            assert users == {(n, 'GLOBAL') for n, i in instances.items() if i['vector'] == vector}
            control = uart['rcc_control']
            assert control == instance['rcc_control']
            assert control['controller'] == 'SYSCTRL' and control['bus_clock'] == 'PCLK'
            assert control['enable_active_value'] == clock[name]['enable_active_value'] == 1
            assert control['reset_asserted_value'] == clock[name]['reset_asserted_value'] == 0
            for kind in ['enable', 'reset']:
                item = control[kind]
                assert clock[name][kind] == {'peripheral': 'SYSCTRL', **item}, (family, name, kind)
                assert fields(sysctrl, item['register'])[item['field']] == [instance[kind + '_bit'], 1]
                assert macro(cmsis, f"SYSCTRL_{item['register']}_{item['field']}_Pos") == instance[kind + '_bit']
            if family in NATIVE:
                assert control['enable_write_key'] == {'field': {'register': 'APBEN1', 'field': 'KEY'}, 'value': 0x5a5a}
                assert fields(sysctrl, 'APBEN1')['KEY'] == [16, 16]
            else:
                assert 'enable_write_key' not in control
            expected = {(r['pin'], r['signal'], r['af']) for r in routes if r['peripheral'] == name and r['pin'] in pads}
            actual = {(r['pin'], r['signal'], r['af']) for r in uart['pins']}
            assert len(actual) == len(uart['pins']) and actual == expected, (selection, name)
            assert len(actual) == projection['uart_route_counts'][name]
            safe_rx = {pin for pin, signal, _ in actual if signal == 'RX'} - exclusions
            assert len(safe_rx) == projection['safe_rx_route_counts'][name]
            route_count += len(actual)
    if family == 'CW32L083':
        shared = facts['shared_vector_authority']
        assert shared['clock_metadata'] == inputs['clock_metadata']
        assert shared['all_pairs'] == [{'vector': f'UART{i}_UART{i + 3}', 'number': i + 26,
                                       'members': [f'UART{i}', f'UART{i + 3}']} for i in range(1, 4)]
        rcc = text['rcc_header'].splitlines()
        lines = [rcc[n - 1] for n in shared['rcc_enable_lines']]
        driver = text['uart_driver'].splitlines()
        for instance, pair in zip(facts['instances'], shared['uart_reset_lines'], strict=True):
            name, control = instance['name'], instance['rcc_control']
            assert any(f'__RCC_{name}_CLK_ENABLE()' in line and
                       f"REGBITS_SET(CW_SYSCTRL->{control['enable']['register']}," in line for line in lines)
            reset = f"CW_SYSCTRL->{control['reset']['register']}_f.{name}"
            assert compact(driver[pair[0] - 1]) == reset + '=0;'
            assert compact(driver[pair[1] - 1]) == reset + '=1;'
    return af, pinout, route_count


def example_routes(evidence, family_data):
    rows = evidence['examples']
    assert {(r['selection'], r['bin']) for r in rows} == EXAMPLES and len(rows) == 9
    unique_routes = set()
    for row in rows:
        selection, family = row['selection'], row['selection'][:8].upper()
        af, pinout, text = family_data[family]
        chip = read_json(f'cw32-data/data/chips/{selection.upper()}.json')
        assert row['memory'] == chip['memory']
        package, = chip['packages']
        assert row['package'] == package['package']
        qualified = named(pinout['packages'])[selection.upper()]
        assert row['ring_capacity'] == 64 and row['caller_chunk'] == 16
        peripherals = named(chip['cores'][0]['peripherals'])
        pages = text['datasheet_text'].split('\f')
        for route in row['routes']:
            matches = [r for r in af['routes'] if (r['peripheral'], r['signal'], r['pin'], r['af']) ==
                       (route['uart'], route['signal'], route['pin'], route['af'])]
            source, = matches
            for key in ['source_macro', 'source_line']:
                assert source[key] == route[key], (selection, route)
            assert source.get('datasheet_cell') == route['datasheet_cell']
            assert {'pin': route['pin'], 'signal': route['signal'], 'af': route['af']} in peripherals[route['uart']]['pins']
            assert route['pin'] not in evidence['families'][family]['hal_excluded_pads']
            pin, = [p for p in package['pins'] if p['position'] == route['package_position']]
            assert route['pin'] in pin['signals']
            table_row, = [p for p in pinout['table_rows'] if route['pin'] in p['signals']]
            assert table_row['positions'][qualified['table_column']] == route['package_position']
            pin_page = pages[table_row['pdf_page_index']]
            assert table_row['source_name'] in pin_page and route['package_position'] in pin_page
            sheet = pages[route['datasheet_viewer_page_1_based'] - 1]
            spelling = route['pin'][:2] + route['pin'][2:].zfill(2)
            start = re.search(r'(?m)^[ \t]*' + spelling + r'(?:/|\s)', sheet)
            assert start, (selection, route)
            row_text = re.split(r'(?m)^[ \t]*P[A-F][0-9]+(?:/|\s)', sheet[start.end():], maxsplit=1)[0]
            signal = {'RX': 'RXD', 'TX': 'TXD'}.get(route['signal'], route['signal'])
            assert route['uart'] + '_' + signal in row_text, (selection, route)
            if route['datasheet_cell'] is None:
                assert family == 'CW32F030' and af['status'] == 'verified-sdk-and-datasheet'
                assert 'Tables5-3through5-6' in compact(af['datasheet']['af_evidence'])
            else:
                cell = route['datasheet_cell']
                assert (cell['pin'], cell['af'], cell['pdf_page']) == (route['pin'], route['af'], route['datasheet_viewer_page_1_based'])
                assert re.search(r'表\s+' + re.escape(cell['table']) + r'\b', sheet), (selection, route)
            unique_routes.add((selection, route['uart'], route['signal'], route['pin'], route['af']))
    assert len(unique_routes) == 19


def example_manifest():
    directory = 'examples/shared/uart-buffered/'
    manifest = read_toml(directory + 'Cargo.toml')
    assert manifest['package']['autobins'] is False and manifest['package']['publish'] is False
    assert manifest['package']['name'] == 'cw32-uart-buffered-example'
    assert manifest['workspace'] == {}
    features = manifest['features']
    selections = {selection for selection, _ in EXAMPLES}
    assert set(features) == {'default', 'idle-notify'} | selections and features['default'] == []
    assert features['idle-notify'] == []
    for selection in selections:
        assert features[selection] == [f'embassy-cw32/{selection}', f'cw32-metapac/{selection}']
    bins = named(manifest['bin'])
    assert set(bins) == {'receive', 'split', 'rx_only', 'flow_control', 'shared_vector', 'borrowed', 'idle_queue', 'idle_notify', 'replace_buffer'}
    for name, entry in bins.items():
        assert entry['path'] == f'src/bin/{name}.rs' and (ROOT / directory / entry['path']).is_file()
        required = {'flow_control': ['cw32l010f8p6'], 'shared_vector': ['cw32l083rct6'],
                    'borrowed': ['cw32f002f3p7'], 'idle_queue': ['idle-notify'],
                    'idle_notify': ['idle-notify']}.get(name, [])
        assert entry.get('required-features', []) == required
    dependencies = manifest['dependencies']
    assert set(dependencies) == {'embassy-cw32', 'embassy-executor', 'embassy-futures', 'cortex-m', 'cortex-m-rt', 'embedded-io-async'}
    assert dependencies['embassy-cw32'] == {'path': '../../../embassy-cw32', 'default-features': False, 'features': ['rt']}
    for name in ['embassy-executor', 'embassy-futures']:
        assert dependencies[name]['git'] == 'https://github.com/embassy-rs/embassy' and dependencies[name]['rev'] == EMBASSY_REV
    assert dependencies['embassy-executor']['features'] == ['platform-cortex-m', 'executor-thread']
    assert dependencies['cortex-m'] == {'version': '0.7.7', 'features': ['critical-section-single-core']}
    assert dependencies['cortex-m-rt'] == '0.7.5' and dependencies['embedded-io-async'] == '0.7.0'
    assert manifest['build-dependencies'] == {'cw32-metapac': {'path': '../../../cw32-metapac', 'default-features': False, 'features': ['metadata']}}
    assert manifest['profile']['dev'] == {'opt-level': 's', 'debug': 0, 'incremental': False}
    assert manifest['profile']['release'] == {'opt-level': 's', 'lto': True, 'codegen-units': 1, 'debug': 0, 'incremental': False}
    identity = lambda p: (p['name'], p['version'], p.get('source'), p.get('checksum'))
    existing = {identity(p) for path in ['firmware/Cargo.lock', 'examples/shared/uart-idle/Cargo.lock']
                for p in read_toml(path)['package'] if 'source' in p}
    lock = read_toml(directory + 'Cargo.lock')['package']
    assert all(identity(p) in existing for p in lock if 'source' in p), 'new or changed locked external dependency'
    local = [p for p in lock if 'source' not in p]
    assert {(p['name'], p['version']) for p in local} == {
        ('cw32-uart-buffered-example', '0.1.0'), ('embassy-cw32', '0.1.0'), ('cw32-metapac', '0.1.0')}
    for name, version in [('cortex-m', '0.7.9'), ('cortex-m-rt', '0.7.7'), ('embedded-io-async', '0.7.0'),
                          ('embassy-executor', '0.10.0'), ('embassy-futures', '0.1.2')]:
        packages = [p for p in lock if p['name'] == name]
        assert len(packages) == 1 and packages[0]['version'] == version, name
        if name.startswith('embassy-'):
            assert packages[0]['source'] == f'git+https://github.com/embassy-rs/embassy?rev={EMBASSY_REV}#{EMBASSY_REV}'


def retire_w031_capability(mode_id, expected):
    # Exact leaf replacements against the immutable baseline. No subset compare,
    # absent-file exemption, or rewriting of historical receipts is permitted.
    changes = {'buffered_ring_receive': [(('implemented_families',),
                                ['CW32A030',
                                 'CW32F002',
                                 'CW32F003',
                                 'CW32F020',
                                 'CW32F030',
                                 'CW32L010',
                                 'CW32L011',
                                 'CW32L012',
                                 'CW32L031',
                                 'CW32L052',
                                 'CW32L083',
                                 'CW32R031',
                                 'CW32W031'],
                                ['CW32A030',
                                 'CW32F002',
                                 'CW32F003',
                                 'CW32F020',
                                 'CW32F030',
                                 'CW32L010',
                                 'CW32L011',
                                 'CW32L012',
                                 'CW32L031',
                                 'CW32L052',
                                 'CW32L083',
                                 'CW32R031'])],
     'finite_idle_receive': [],
     'safe_peripheral_dma': [(('scope',),
                              'Safe staged TX, finite RX and combined/split UART DMA on F030/A030/L083 and '
                              'fourteen exact F020/L031/L052/R031/W031 parts; exact L012 C8T6/C8U6 remain UART '
                              'TX-only.',
                              'Safe staged TX, finite RX and combined/split UART DMA on F030/A030/L083 and '
                              'thirteen exact F020/L031/L052/R031 parts; exact L012 C8T6/C8U6 remain UART '
                              'TX-only.'),
                             (('implemented_families',),
                              ['CW32A030',
                               'CW32F020',
                               'CW32F030',
                               'CW32L083',
                               'CW32L031',
                               'CW32L052',
                               'CW32R031',
                               'CW32W031',
                               'CW32L012'],
                              ['CW32A030',
                               'CW32F020',
                               'CW32F030',
                               'CW32L083',
                               'CW32L031',
                               'CW32L052',
                               'CW32R031',
                               'CW32L012']),
                             (('overrides', 2, 'families'),
                              ['CW32L031', 'CW32L052', 'CW32R031', 'CW32W031'],
                              ['CW32L031', 'CW32L052', 'CW32R031']),
                             (('overrides', 2, 'scope'),
                              'Safe staged byte UART TX, finite RX and combined/split DMA on eleven exact native '
                              'four-channel selections.',
                              'Safe staged byte UART TX, finite RX and combined/split DMA on ten exact native '
                              'four-channel selections.'),
                             (('overrides', 2, 'limitations', 0),
                              'Exactly CW32L031C8T6/C8U6/F8P6/F8U6/K8U6/K8V6, CW32L052C8T6/R8S6/R8T6, '
                              'CW32R031C8U6 and CW32W031R8U6; each has 64 KiB Flash and 8 KiB SRAM at '
                              '0x20000000..0x20002000.',
                              'Exactly CW32L031C8T6/C8U6/F8P6/F8U6/K8U6/K8V6, CW32L052C8T6/R8S6/R8T6, '
                              'CW32R031C8U6; each has 64 KiB Flash and 8 KiB SRAM at 0x20000000..0x20002000.'),
                             (('overrides', 2, 'limitations', 1),
                              'Generic cw32l031/cw32l052/cw32r031/cw32w031 compile the controller/UART DMA '
                              'surface but reject safe admission and staging with MissingMemoryMetadata; no '
                              'memory fallback.',
                              'Generic cw32l031/cw32l052/cw32r031 compile the controller/UART DMA surface but '
                              'reject safe admission and staging with MissingMemoryMetadata; no memory '
                              'fallback.'),
                             (('overrides', 2, 'limitations', 10),
                              'R031 whole-GPIOA effects and W031 internal RF boundaries preserve existing '
                              'functional handover obligations; no RF-safe handover/coexistence claim.',
                              'R031 whole-GPIOA effects preserve existing functional handover obligations; no '
                              'RF-safe handover/coexistence claim.')]}
    for path, old, new in changes[mode_id]:
        owner = expected
        for key in path[:-1]:
            owner = owner[key]
        assert owner[path[-1]] == old, ('retirement baseline drift', mode_id, path)
        owner[path[-1]] = new


def current_capability_boundary(evidence, modes):
    # Historical source receipts stay immutable; current policy is a separate,
    # hash-pinned baseline with only explicitly listed source relocations.
    path = ROOT / 'docs/uart-current-capability-boundary.json'
    raw = path.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == '74e63841f17eb0ab28469b1e58d1c6d189826d17949ead2305f61cae9ff8df34'
    boundary = json.loads(raw)
    assert boundary['schema_version'] == 1
    assert boundary['baseline_commit'] == 'e75f62e7f792dede2b2d3c89bbaba5f9aee25531'
    assert boundary['baseline_capabilities_sha256'] == 'a51e20fd804f7c158ca5e9d590f77bc2730c117a1a5ede13fff3e62cff20ee79'
    example = boundary['baseline_example_manifest']
    assert example['path'] == 'examples/uart-buffered/Cargo.toml'
    # Reverse only the reviewed additive examples; keep the historical receipt immutable.
    manifest = (ROOT / 'examples/shared/uart-buffered/Cargo.toml').read_text()
    manifest = manifest.replace('path = "../../../', 'path = "../../')
    additions = ['idle-notify = []\n']
    for name in ('idle_queue', 'idle_notify', 'replace_buffer'):
        entry = f'[[bin]]\nname = "{name}"\npath = "src/bin/{name}.rs"\n'
        if name != 'replace_buffer':
            entry += 'required-features = ["idle-notify"]\n'
        additions.append(entry + '\n')
    for addition in additions:
        assert manifest.count(addition) == 1
        manifest = manifest.replace(addition, '')
    assert hashlib.sha256(manifest.encode()).hexdigest() == example['sha256']
    assert boundary['historical_evidence_path'] == EVIDENCE
    assert hashlib.sha256((ROOT / EVIDENCE).read_bytes()).hexdigest() == boundary['historical_evidence_sha256']
    historical_dma = evidence['capability_boundaries']['dma']
    canonical = json.dumps(historical_dma, sort_keys=True, separators=(',', ':')).encode()
    assert hashlib.sha256(canonical).hexdigest() == boundary['historical_dma_canonical_sha256']
    assert historical_dma['status'] == 'experimental_partial'
    assert set(historical_dma['implemented_families']) == {'CW32A030', 'CW32F030', 'CW32L083'}
    assert historical_dma['pins_any'] == ['TX', 'RX'] and historical_dma['facts'] == ['dma_channels']
    assert set(boundary['baseline_modes']) == {'buffered_ring_receive', 'finite_idle_receive', 'safe_peripheral_dma'}
    relocations = {(r['from'], r['anchor']): r['to'] for r in boundary['relocations']}
    assert len(relocations) == len(boundary['relocations']) == 3
    used = set()
    for mode_id, baseline in boundary['baseline_modes'].items():
        expected = json.loads(json.dumps(baseline))
        for item in expected['evidence']:
            identity = (item['path'], item.get('anchor'))
            if identity in relocations:
                item['path'] = relocations[identity]
                used.add(identity)
        retire_w031_capability(mode_id, expected)
        if mode_id == 'buffered_ring_receive':
            # Reviewed owner-lifetime storage return/replacement and native idle additions.
            expected['limitations'][3] = 'Safe constructors consume the nonempty static ring even on constructor error; successful RX owners can return storage with the owner lifetime via into_buffer after normal shutdown (software check scope in docs/validation-scope.md); unsafe borrowed constructors release the borrow after normal RX Drop and the shared TX/RX lifetime ends, and require storage to remain valid and exclusive if the owner is leaked.'
            expected['limitations'][8:] = ['L010/L011/L012 wait_for_idle observes a new coalesced native idle event while IRQ buffering continues. Cancellation masks only idle; data/errors stay queued. BufferedIdle records unread byte count and first retained error after the ISR observation; later appends do not extend it. No wire packet boundary, lossless history or bounded wait; software check scope is recorded in docs/validation-scope.md.', 'Native idle capture retains four adjusted unread-prefix markers; overflow is reported after retained markers and blocks new markers until consumed. Queued error fields are historical; no lossless history, DMA ring or numeric timeout. into_buffer discards unread data/errors and returns neither pins nor the peripheral; only a static RX owner returns a static slice.', 'try_replace_buffer accepts a nonempty static replacement only when no queued bytes or error remain; failures return the new buffer unchanged. Active idle capture rejects replacement until explicitly stopped. Success returns old storage with the owner lifetime without stopping RX or changing TX/shared IRQ ownership. Pending hardware bytes may enter the new ring; no packet boundary or guaranteed progress. Software check scope is recorded in docs/validation-scope.md.']
        assert modes[mode_id] == expected, ('current capability boundary drift', mode_id)
    assert used == set(relocations)


def capability_contract(evidence, capabilities):
    modes = named(capabilities['functions']['uart']['modes'], 'id')
    current_capability_boundary(evidence, modes)
    buffered = modes['buffered_ring_receive']
    assert buffered['status'] == 'partial' and set(buffered['implemented_families']) == FAMILIES - {'CW32W031'}
    assert len(buffered['implemented_families']) == 12 and buffered['pins_all'] == ['RX']
    assert {item['path'] for item in buffered['evidence']} == {'embassy-cw32/src/usart/mod.rs', 'docs/uart-buffered-rx.md'}
    assert all((ROOT / item['path']).is_file() for item in buffered['evidence'])
    declaration = (buffered['scope'] + ' ' + ' '.join(buffered['limitations'])).lower()
    for term in ['inherent and async trait reads return an available clean prefix; read_exact fills the slice.',
                 'empty reads preserve queued bytes and pending errors',
                 'fill_buf lends a contiguous occupied prefix without copying',
                 'consume releases only exposed bytes', 'irq writes remain disjoint',
                 'unsafe borrowed constructors', 'valid and exclusive if the owner is leaked']:
        assert term in declaration, ('buffered current declaration missing', term)
    for term in ['continuous', 'irq', 'static', 'bounded', 'exact', 'trait',
                 'silent', 'overwrite', 'ore', 'ne', '16', '8', 'overflow', 'discard',
                 'pending', 'error', 'progress', 'rts', 'occupancy', 'idle', 'dma', 'hardware']:
        assert term in declaration, ('buffered limitation missing', term)
    assert 'buffered_ring_idle' not in modes
    idle = modes['finite_idle_receive']
    assert set(evidence['capability_boundaries']['native_idle_families']) == NATIVE
    assert idle['status'] == 'partial' and set(idle['implemented_families']) == NATIVE
    assert len(idle['implemented_families']) == 3 and idle['pins_all'] == ['RX']
    assert {e['path'] for e in idle['evidence']} == {'embassy-cw32/src/usart/rx.rs', 'docs/uart-idle.md'}
    # Named current capability invariant, maintained only with separate DMA qualification.
    dma = modes['safe_peripheral_dma']
    # Complete current equality is enforced above; historical DMA is pinned separately.
    assert dma['status'] == 'experimental_partial' and set(dma['implemented_families']) == {
        'CW32A030', 'CW32F020', 'CW32F030', 'CW32L012', 'CW32L031',
        'CW32L052', 'CW32L083', 'CW32R031'}
    assert dma['pins_any'] == ['TX', 'RX'] and dma['facts'] == ['dma_channels']
    assert [item['families'] for item in dma['overrides']] == [
        ['CW32L012'], ['CW32F020'], ['CW32L031', 'CW32L052', 'CW32R031']]
    ordinary = ' '.join(modes['async']['limitations'])
    assert 'Uart' in ordinary and 'UartRx' in ordinary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sources', type=Path, required=True,
                        help='Absolute directory containing already locked official evidence')
    args = parser.parse_args()
    assert args.sources.is_absolute() and args.sources.is_dir(), args.sources
    evidence = read_json(EVIDENCE)
    assert evidence['cohort'] == {'families': 13, 'selections': 54, 'family_instances': 39,
                                  'register_versions': 7, 'external_identities': 111}
    assert set(evidence['families']) == FAMILIES
    assert len(evidence['selection_projection']) == 54
    nodes, material = source_material(evidence, args.sources)
    register_contracts(evidence)
    capabilities = read_yaml('tools/hal-capabilities.yaml')
    features = read_toml('embassy-cw32/Cargo.toml')['features']
    parts = named(read_yaml('cw32-data/parts.yaml')['parts'])
    selections, consumed, instance_count, route_count, family_data = set(), set(), 0, 0, {}
    for family, facts in evidence['families'].items():
        text = family_sources(family, facts, evidence, nodes, material)
        af, pinout, count = family_metadata(family, facts, evidence, nodes, text, capabilities, features, parts)
        family_data[family] = af, pinout, text
        assert not selections & set(facts['selections'])
        selections.update(facts['selections'])
        consumed.update(facts['sources'].values())
        instance_count += len(facts['instances'])
        route_count += count
    assert len(selections) == 54 and instance_count == 39 and consumed == set(nodes)
    assert set(named(evidence['selection_projection'], 'selection')) == selections
    example_routes(evidence, family_data)
    example_manifest()
    capability_contract(evidence, capabilities)
    print('PASS buffered UART historical source cohort: 111 own locked identities; 13 families, 54 selections, 39 instances, 7 register profiles')
    print(f'PASS own RC/RDR/overwrite, native ORE/NE, R1W0/no-op, independent shared gates and {route_count} package-filtered routes')
    print('PASS 9 qualified example tuples, 19 selected signal routes, 8 exact features, 9 declared bins and current locked dependencies')
    assert len(list((ROOT / 'cw32-data/data/chips').glob('*.json'))) == 52
    print('PASS current targets: 52 generated selections; W031 family and exact part absent; two byte-pinned historical snapshots only')
    print('PASS partial buffered capability; separate 3-family native idle and current staged DMA boundaries')
    print('Source/data/manifest verification only; runtime lifecycle and hardware timing require separate review/evidence.')


if __name__ == '__main__':
    main()
