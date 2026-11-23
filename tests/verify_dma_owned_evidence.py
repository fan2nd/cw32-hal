#!/usr/bin/env python3
"""Validate own-source DMA/PAC facts. No HAL execution, harness, mock or fixture."""
from pathlib import Path
import hashlib
import json
import os
import re
import yaml
from current_targets import historical_chip_path

ROOT = Path(__file__).resolve().parents[1]
SOURCES = Path(os.environ.get('CW32_SOURCES', '/workspace/shared/cw32-sources'))
def load(path):
    # Exactly two retired chip inputs use byte-pinned historical snapshots.
    # Every other path is current; missing current metadata remains an error.
    path = historical_chip_path(path)
    return yaml.safe_load(path.read_text()) if path.suffix == '.yaml' else json.loads(path.read_text())
def digest(data): return hashlib.sha256(data).hexdigest()
e = load(ROOT / 'docs/dma-owned-copy-evidence.json')
lock = load(ROOT / 'sources/evidence-sources.json')
for key, source in e['sources'].items():
    pin = next(a for a in lock['artifacts'] if a['id'] == source['source_ref'])
    assert all(source[k] == pin[k] for k in ('path', 'url', 'sha256', 'bytes'))
    assert digest((SOURCES / source['path']).read_bytes()) == source['sha256']
    # Consumer backlinks grow independently of the locked text identity.
    assert {k: v for k, v in source['text'].items() if k != 'evidence'} == {
        k: v for k, v in pin['text'].items() if k != 'evidence'
    }
    raw = (SOURCES / source['text']['path']).read_bytes()
    assert digest(raw) == source['text']['sha256']
    if '8.8.3' not in source['sections']:
        continue
    pages = raw.decode().split('\f')
    csr = pages[source['sections']['8.8.3']['pdf_page_index']]
    trig = pages[source['sections']['8.8.4']['pdf_page_index']]
    assert 'DMA_CSRy' in csr and 'STATUS' in csr and 'SIZE' in csr and 'TRANS' in csr
    assert 'SOFTSRC' in trig and 'TYPE' in trig
    if key == 'x030_en':
        for token in ['000: Initial', '101: Transfer completed', '00: 8bit', '01: 16bit', '10: 32bit',
                      '0: Bulk transfer (BULK)', '1: Block transfer (BLOCK)']:
            assert token in csr, (key, token)
        for token in ['0: Software trigger mode', '1: In hardware trigger mode', 'R0: Transfer completed', 'W0: No function']:
            assert token in trig, (key, token)
    else:
        for token in ['000：初始状态', '001：传输错误，传输地址超出寻址范围', '010：传输错误，传输停止请求引起中止',
                      '011：传输错误，访问传输来源地址出错', '100：传输错误，访问传输目的地址出错',
                      '101：传输完成', '00：8 比特', '01：16 比特', '10：32 比特',
                      '0：批量传输（BULK）', '1：块传输（BLOCK）']:
            assert token in csr, (key, token)
        for token in ['0：软件触发模式', '1：硬件触发模式', 'R0：传输已完成', 'W0：无功能']:
            assert token in trig, (key, token)
    if '8.8.2' in source['sections']:
        icr = pages[source['sections']['8.8.2']['pdf_page_index']]
        assert 'DMA_ICR' in icr and 'Reset value: 0xFFFF FFFF' in icr and 'R1W0' in icr
        assert re.search(r'W0[:：].*(?:Clear|clear|清除)', icr)
        assert re.search(r'W1[:：].*(?:No function|无功能)', icr)

reg = 'dmachannel_v1'
authored = yaml.safe_load((ROOT / 'cw32-data/registers' / (reg + '.yaml')).read_text())
generated = load(ROOT / 'cw32-data/data/registers' / (reg + '.json'))
for name, values in e['enums'].items():
    assert {v['name']: v['value'] for v in authored['enum/' + name]['variants']} == values
    assert authored['enum/' + name] == generated['enum/' + name]
for fieldset, field, name in [('CSR','TRANS','TransferMode'), ('CSR','SIZE','Width'), ('CSR','STATUS','Status'), ('TRIG','TYPE','Trigger')]:
    assert next(f for f in generated['fieldset/' + fieldset]['fields'] if f['name'] == field)['enum'] == name
for reg in ('dmachannel_v1', 'dma_v1'):
    ir = load(ROOT / 'cw32-data/data/registers' / (reg + '.json'))
    group = next(g for g in load(ROOT / 'cw32-data/register-reuse.yaml')['groups'] if g['canonical'] == reg + '.yaml')
    assert digest(json.dumps(ir, sort_keys=True, separators=(',', ':')).encode()) == group['canonical_ir_sha256']
command = load(ROOT / 'cw32-data/register-writes.yaml')['registers']['dma_v1'][0]
assert command['reset_value'] == command['write_noop'] == 0xffffffff
assert set(command['zero_to_clear_fields']) == {f'{kind}{index}' for index in range(1,6) for kind in ('TC','TE')}
for name, size in e['sram'].items():
    if name == 'base': continue
    chip = load(ROOT / 'cw32-data/data/chips' / (name + '.json'))
    memory = chip['memory'][0]
    ram = next(region for region in memory if region['name'] == 'RAM')
    assert ram['address'] == e['sram']['base'] and ram['size'] == size
for name in ('CW32F030','CW32A030'):
    assert not load(ROOT / 'cw32-data/data/chips' / (name + '.json')).get('memory')
print('PASS: ten pinned own documents/texts, eight own DMA enum tables, three ICR seeds, generated enums/reuse hashes and x030 exact-part SRAM bounds; no HAL execution')

# Finite native four-channel own-source extension. This reads source facts and
# generated metadata/PAC only; it does not inspect or execute HAL behavior.
import io
import xml.etree.ElementTree as ET
import zipfile

four = load(ROOT / 'docs/four-channel-dma-owned-evidence.json')
assert four['schema_version'] == 1
assert len(four['sources']) == 54
family_names = {'CW32L031', 'CW32L052', 'CW32R031', 'CW32W031'}
assert {f['family'] for f in four['families']} == family_names
assert len(four['families']) == 4
exact_features = {
    'cw32l031c8t6', 'cw32l031c8u6', 'cw32l031f8p6', 'cw32l031f8u6',
    'cw32l031k8u6', 'cw32l031k8v6', 'cw32l052c8t6', 'cw32l052r8s6',
    'cw32l052r8t6', 'cw32r031c8u6', 'cw32w031r8u6',
}
assert {p['feature'] for p in four['exact_parts']} == exact_features
assert len(four['exact_parts']) == 11
assert {p['feature'] for p in four['generic_aliases']} == {f.lower() for f in family_names}
assert len(four['generic_aliases']) == 4


def four_index(pointer):
    value = lock
    for component in pointer.strip('/').split('/'):
        value = value[int(component)] if isinstance(value, list) else value[component]
    return value


def four_compact(value):
    return re.sub(r'\s+', '', value)


def four_tokens(value, tokens, where):
    value = four_compact(value)
    for token in tokens:
        assert four_compact(token) in value, (where, token)


four_bytes = {}
for source in four['sources']:
    pin = four_index(source['source_index_pointer'])
    artifact = lock['artifacts'][int(source['source_index_pointer'].split('/')[2])]
    identity = source['identity']
    # Bind the complete selected node, including provenance/licensing fields.
    # The archive's catalog is not itself a selected member: all used members
    # are bound separately below, without freezing unrelated catalog entries.
    observed_identity = {k: v for k, v in pin.items() if k != 'evidence'
                         and not (k == 'members' and source['source_ref'].startswith('vendor:'))}
    if source['source_ref'].startswith('vendor:'):
        assert source['source_ref'] == artifact['id']
    if 'text' in observed_identity:
        observed_identity['text'] = {k: v for k, v in observed_identity['text'].items() if k != 'evidence'}
    expected_identity = dict(identity)
    if 'text' in expected_identity:
        expected_identity['text'] = {k: v for k, v in expected_identity['text'].items() if k != 'evidence'}
    assert observed_identity == expected_identity, source['source_ref']
    assert source['official_url'] == artifact['url']
    raw = (SOURCES / identity['path']).read_bytes()
    assert len(raw) == identity['bytes'] and digest(raw) == identity['sha256'], identity['path']
    four_bytes[identity['path']] = raw
    if 'archive_source_ref' in source:
        assert source['archive_source_ref'] == artifact['id']
        payload = (SOURCES / artifact['path']).read_bytes()
        for member in identity['members']:
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                payload = archive.read(member)
        assert payload == raw, identity['path']
assert len(four_bytes) == 54

native = load(ROOT / 'cw32-data/data/registers/dma_cw32l031_v1.json')
assert native == load(ROOT / 'cw32-data/registers/dma_cw32l031_v1.yaml')
reuse = next(g for g in load(ROOT / 'cw32-data/register-reuse.yaml')['groups']
             if g['canonical'] == 'dma_cw32l031_v1.yaml')
assert set(reuse['source_versions']) == {f'dma_{f.lower()}_v1.yaml' for f in family_names}
assert digest(json.dumps(native, sort_keys=True, separators=(',', ':')).encode()) == reuse['canonical_ir_sha256']
channel_ir = load(ROOT / 'cw32-data/data/registers/dmachannel_v1.json')
expected_offsets = {'ISR': 0, 'ICR': 4}
for channel in range(1, 5):
    expected_offsets.update({f'{reg}{channel}': 32 * channel + offset
                             for reg, offset in [('CSR', 0), ('CNT', 4), ('SRCADDR', 8), ('DSTADDR', 12), ('TRIG', 16)]})
assert {r['name']: r['byte_offset'] for r in native['block/DMA']['items']} == expected_offsets
expected_channels = [{'channel': i - 1, 'dma': 'DMA', 'name': f'DMA_CH{i}'} for i in range(1, 5)]
expected_vectors = {'DMACH1': 9, 'DMACH23': 10, 'DMACH4': 11, 'UART1': 27, 'UART2': 28, 'UART3': 29}
expected_channel_vectors = [('DMACH1', 9), ('DMACH23', 10), ('DMACH23', 10), ('DMACH4', 11)]
expected_flags = {f'{kind}{i}': (4 * (i - 1) + bit, 1)
                  for i in range(1, 5) for kind, bit in [('TC', 0), ('TE', 1)]}
assert {f['name']: (f['bit_offset'], f['bit_size']) for f in native['fieldset/ICR']['fields']} == expected_flags


def four_svd_field(field):
    encoding = {key for key in ('bitOffset', 'bitWidth', 'lsb', 'msb', 'bitRange')
                if field.find(key) is not None}
    if encoding == {'bitOffset', 'bitWidth'}:
        offset = int(field.findtext('bitOffset'), 0)
        width = int(field.findtext('bitWidth'), 0)
    elif encoding == {'lsb', 'msb'}:
        offset = int(field.findtext('lsb'), 0)
        width = int(field.findtext('msb'), 0) - offset + 1
    else:
        raise AssertionError(('unsupported or conflicting own-SVD field encoding', field.findtext('name'), encoding))
    assert 0 <= offset < 32 and 1 <= width <= 32 - offset
    return {'name': field.findtext('name'), 'bit_offset': offset,
            'bit_size': width, 'access': field.findtext('access')}


def four_svd_register(register):
    return {
        'name': register.findtext('name'),
        'offset': int(register.findtext('addressOffset'), 0),
        'fields': [four_svd_field(f) for f in register.findall('fields/field')],
    }


four_family_pages = {}
for family in four['families']:
    name = family['family']
    pages_by_kind = {}
    for kind in ('manual', 'datasheet'):
        source = next(s for s in four['sources'] if s['source_ref'] == family[kind + '_source_ref'])
        assert source['identity']['path'] == family[kind]
        assert source['identity']['text']['path'] == family[kind + '_text_path']
        pages = four_bytes[family[kind + '_text_path']].decode().split('\f')
        pages_by_kind[kind] = pages
        for page in family[kind + '_pages']:
            assert page['printed_page'] == page['pdf_page_index']
            assert page['pdf_viewer_page'] == page['pdf_page_index'] + 1
            assert digest(pages[page['pdf_page_index']].encode()) == page['extracted_page_utf8_sha256'], (name, kind, page)
    four_family_pages[name] = pages_by_kind
    manual = pages_by_kind['manual']
    claims = family['manual_page_claims']
    for claim, numbers in claims.items():
        assert numbers == [p['pdf_page_index'] for p in family['manual_pages'] if claim in p['claims']], (name, claim)
    claim_text = {claim: '\n'.join(manual[n] for n in numbers) for claim, numbers in claims.items()}
    four_tokens(claim_text['sram'], ['8KB', 'SRAM', '0x2000 0000'], (name, 'SRAM'))
    four_tokens(claim_text['flash'], ['64KB', '0x0000 0000', '0x0000 FFFF'], (name, 'Flash'))
    four_tokens(claim_text['dma_channels'], ['4 条独立 DMA 通道', '1 ~ 65535'], (name, 'channels'))
    four_tokens(claim_text['mmio_width_and_gates'], ['其它外设只支持', '32bit', '读数据的结果不可用'], (name, 'width/gates'))
    four_tokens(claim_text['software_block'], ['软件触发 BLOCK', '传输完成后 DMA_TRIGy.SOFTSRC 自动清零'], (name, 'software'))
    four_tokens(claim_text['width_repeat_count'], ['位宽完全一致', 'REPEAT', '建议写入 1', '1 ~ 65535'], (name, 'width/count'))
    four_tokens(claim_text['terminal'], ['数据传输全部正确完成', 'TEIE', 'TCIE', 'DMA_ICR.TEy 为 0', 'DMA_ICR.TCy 为 0'], (name, 'terminal'))
    four_tokens(claim_text['csr'], ['000：初始状态', '001：传输错误，传输地址超出寻址范围',
                '010：传输错误，传输停止请求引起中止', '011：传输错误，访问传输来源地址出错',
                '100：传输错误，访问传输目的地址出错', '101：传输完成',
                '00：8 比特', '01：16 比特', '10：32 比特', '0：批量传输（BULK）', '1：块传输（BLOCK）'], (name, 'CSR'))
    four_tokens(claim_text['trig'], ['HARDSRC', 'SOFTSRC', 'R0：传输已完成', 'W0：无功能',
                '0：软件触发模式', '1：硬件触发模式'], (name, 'TRIG'))
    if name == 'CW32L052':
        assert claims['trig'] == [138, 139]
        four_tokens(manual[138], ['HARDSRC', '000001：UART1 发送 Buf 空标志'], (name, 'request page'))
        four_tokens(manual[139], ['SOFTSRC', 'TYPE', 'R0：传输已完成'], (name, 'continuation'))
    icr = claim_text['icr']
    four_tokens(icr, ['DMA_ICR', 'Address offset: 0x04', 'Reset value: 0xFFFF FFFF',
                      '保留位，请保持默认值', 'W0：清除', 'W1：无功能'], (name, 'ICR'))
    for field, (bit, width) in expected_flags.items():
        assert re.search(rf'(?m)^\s*{bit}\s+{field}\s+R1W0\b', icr), (name, field)
    four_tokens(claim_text['dma_gate'], ['Address offset: 0x30', 'DMA 模块配置时钟及工作时钟使能控制', '0：关闭', '1：使能'], (name, 'gate'))
    four_tokens(claim_text['dma_reset'], ['Address offset: 0x40', 'DMA 模块复位控制', '0：模块处于复位状态', '1：模块正常工作'], (name, 'reset'))
    four_tokens(claim_text['uart_instances'], ['3 个通用异步收发器'], (name, 'UART scope'))
    four_tokens(claim_text['uart_dma'], ['UARTx_CR2.DMATX 为 1', 'UARTx_TDR', 'DMA_ISR.TCy', 'UARTx_ISR.TXBUSY'], (name, 'UART DMA'))
    four_tokens(claim_text['uart_layout'], ['UART1_BASE = 0x4001 3800', 'UART2_BASE = 0x4000 4400',
                'UART3_BASE = 0x4000 4800', 'UARTx_TDR', 'UARTx_BASE + 0x28'], (name, 'UART bases'))
    four_tokens(claim_text['uart_cr2'], ['7 DMATX RW', 'DMA 发送数据使能控制'], (name, 'DMATX'))
    four_tokens(claim_text['uart_tdr'], ['UARTx_TDR', 'Address offset: 0x28', '8:0 TDR WO'], (name, 'TDR'))
    tx = four_compact(claim_text['uart_byte_tx_example'])
    four_tokens(tx, ['SRAM', 'uint8_t SendBuff[100]', 'DMA_SRCADDR1 = ( uint32_t ) SendBuff',
                'DMA_DSTADDR1 = ( uint32_t ) &UART1_TDR', 'DMA_CSR1.SRCINC 为 1', 'DMA_CSR1.DSTINC 为 0',
                'DMA_CNT1 写入 0x10064', 'DMA_CSR1.SIZE 为 0', 'DMA_CSR1.TRANS 为 1', 'UART1_CR2.SOURCE 为 0'], (name, 'byte TX'))
    ordered = ['DMA_CSR1.EN 为 1', 'UART1_CR2.DMATX 为 1', 'DMA_ISR.TC1 变为 1',
               'UART1_CR2.DMATX 为 0', 'UART1_ISR.TXBUSY 变为 0']
    positions = [tx.index(four_compact(token)) for token in ordered]
    assert positions == sorted(positions), (name, 'EN/DMATX/completion order')
    irq = claim_text['irq']
    for symbol, number, manual_name in [('DMACH1', 9, 'DMA1'), ('DMACH23', 10, 'DMA23'), ('DMACH4', 11, 'DMA4')]:
        assert re.search(rf'(?m)^\s*{number + 16}\s+{number}\s+可配置\s+{manual_name}\b.*0x0000\s+00{4 * (number + 16):02X}', irq), (name, symbol)
    for symbol, number in [('UART1', 27), ('UART2', 28), ('UART3', 29)]:
        assert re.search(rf'(?m)^\s*{number + 16}\s+{number}\s+可配置\s+{symbol}\b', irq), (name, symbol)

    svd = ET.fromstring(four_bytes[family['svd_path']])
    peripherals = {p.findtext('name'): p for p in svd.findall('peripherals/peripheral')}
    for expected in family['svd_geometry']:
        peripheral = peripherals[expected['name']]
        assert int(peripheral.findtext('baseAddress'), 0) == expected['base']
        assert peripheral.get('derivedFrom') == expected['derived_from']
        assert [{'name': i.findtext('name'), 'number': int(i.findtext('value'), 0)}
                for i in peripheral.findall('interrupt')] == expected['interrupts']
        for key in ('registers', 'selected_registers'):
            if key in expected:
                registers = [four_svd_register(r) for r in peripheral.findall('registers/register')]
                if key == 'selected_registers':
                    selected = {r['name'] for r in expected[key]}
                    registers = [r for r in registers if r['name'] in selected]
                assert registers == expected[key], (name, expected['name'], key)
    dma = next(p for p in family['svd_geometry'] if p['name'] == 'DMA')
    assert dma['base'] == 0x40020000
    assert {r['name']: r['offset'] for r in dma['registers']} == expected_offsets
    for register in dma['registers']:
        fields = {f['name']: (f['bit_offset'], f['bit_size']) for f in register['fields']}
        assert fields == {f['name']: (f['bit_offset'], f['bit_size'])
                          for f in native['fieldset/' + register['name']]['fields']}, (name, register['name'])
        if register['name'] not in ('ISR', 'ICR'):
            channel_fieldset = re.sub(r'[1-4]$', '', register['name'])
            assert fields == {f['name']: (f['bit_offset'], f['bit_size'])
                              for f in channel_ir['fieldset/' + channel_fieldset]['fields']}
    for symbol in family['sdk_symbols']:
        lines = four_bytes[symbol['path']].decode().splitlines()
        assert lines[symbol['line'] - 1].strip() == symbol['text'].strip(), (name, symbol)
    sidecar = load(ROOT / family['dma_metadata_path'])
    assert sidecar['core_dma_channels'] == expected_channels
    assert sidecar['channel_details'] == family['physical_channels']
    assert len(family['physical_channels']) == 4
    for index, channel in enumerate(family['physical_channels']):
        assert channel['physical_channel'] == index + 1
        assert channel['name'] == f'DMA_CH{index + 1}'
        assert (channel['interrupt'], channel['interrupt_number']) == expected_channel_vectors[index]
        peripheral = peripherals[channel['register_peripheral']]
        assert int(peripheral.findtext('baseAddress'), 0) == 0x40020020 + 32 * index
        assert (peripheral.findtext('interrupt/name'), int(peripheral.findtext('interrupt/value'), 0)) == expected_channel_vectors[index]
    tx_requests = [r for r in sidecar['requests'] if r['peripheral'] in ('UART1', 'UART2', 'UART3') and r['signal'] == 'TX']
    assert tx_requests == family['qualified_tx_requests']
    assert [r for r in sidecar['requests'] if r not in tx_requests] == family['unqualified_inventory_only']
    assert [(r['peripheral'], r['request']) for r in tx_requests] == [('UART1', 1), ('UART2', 3), ('UART3', 5)]
    dma_header = next(path for path in four_bytes if path.endswith('/' + name.lower() + '_dma.h'))
    dma_lines = four_bytes[dma_header].decode().splitlines()
    for request in tx_requests:
        assert request['channels'] == [c['name'] for c in expected_channels]
        assert request['dma'] == 'DMA'
        assert request['sdk']['raw_value'] == request['request'] << 2
        four_tokens(manual[request['manual']['pdf_page_index']],
                    [request['manual']['binary'] + '：' + request['manual']['description']], (name, request['peripheral']))
        line = dma_lines[request['sdk']['line'] - 1]
        match = re.search(rf"#define\s+{request['sdk']['symbol']}\s+\(uint32_t\)\(0x([0-9a-f]+)UL\s*<<\s*2\)", line, re.I)
        assert match and int(match[1], 16) << 2 == request['sdk']['raw_value'], (name, line)
    clocks = load(ROOT / family['clock_metadata_path'])['peripherals']
    # The frozen ledger records clock facts, excluding only narrative notes and
    # consumer evidence links. Keep status, blockers and all scoped hardware
    # fields (including future fields) in the exact semantic comparison.
    clock_facts = [{k: v for k, v in c.items() if k not in ('notes', 'evidence')}
                   for c in clocks if c['name'] in ('DMA', 'GPIOA', 'UART1', 'UART2', 'UART3')]
    assert clock_facts == family['clocks']

seed = four['native_icr_seed']
command = load(ROOT / 'cw32-data/register-writes.yaml')['registers']['dma_cw32l031_v1']
assert len(command) == 1
command = command[0]
for key in ('block', 'register', 'fieldset', 'reset_value', 'write_noop', 'zero_to_clear_fields'):
    assert command[key] == seed[key]
assert command['block'] == 'DMA' and command['register'] == command['fieldset'] == 'ICR'
assert command['reset_value'] == command['write_noop'] == 0xffffffff
assert command['zero_to_clear_fields'] == [f'{kind}{i}' for i in range(1, 5) for kind in ('TC', 'TE')]
assert load(ROOT / 'cw32-data/data/register-writes/dma_cw32l031_v1.json') == {
    'schema_version': 1, 'registers': {'dma_cw32l031_v1': [command]}}
assert [v['physical_channel'] for v in seed['per_channel_clear_commands']] == [1, 2, 3, 4]
for values in seed['per_channel_clear_commands']:
    i = values['physical_channel']
    for key, bits in [('tc_only', [4 * (i - 1)]), ('te_only', [4 * (i - 1) + 1]), ('both', [4 * (i - 1), 4 * (i - 1) + 1])]:
        assert int(values[key], 16) == 0xffffffff ^ sum(1 << bit for bit in bits)
pac = (ROOT / 'cw32-metapac/src/peripherals/dma_cw32l031_v1.rs').read_text()
assert pac.count('impl regs::Icr {') == 1
icr_helpers = pac.split('impl regs::Icr {', 1)[1]
for helper in ('reset_value', 'write_noop'):
    assert re.search(rf'pub const fn {helper}\(\) -> Self\s*\{{\s*Self\(4294967295\)\s*\}}', icr_helpers)

parts = load(ROOT / 'cw32-data/parts.yaml')['parts']
assert {p['feature'] for p in parts if p['family'] in family_names} == exact_features
selected_count = 0
for part in four['exact_parts'] + four['generic_aliases']:
    chip = load(ROOT / part['metadata_path'])
    assert chip['name'].lower() == part['feature']
    assert chip['memory'] == part['memory']
    core = chip['cores'][0]
    assert core['dma_channels'] == part['dma_channels'] == expected_channels
    assert [p for p in core['peripherals'] if p['name'] == 'DMA' or p['name'].startswith('DMACHANNEL')] == part['dma_instances']
    vectors = [v for v in core['interrupts'] if v['name'] in expected_vectors]
    assert vectors == part['dma_uart_vectors']
    assert {v['name']: v['number'] for v in vectors} == expected_vectors
    if part['feature'] not in exact_features:
        assert part['memory'] == [] and not chip['packages']
        continue
    assert part['flash_interval'] == [0, 0x10000]
    assert part['sram_interval'] == [0x20000000, 0x20002000]
    assert part['memory'] == [[{'address': 0, 'kind': 'flash', 'name': 'FLASH', 'size': 65536},
                              {'address': 0x20000000, 'kind': 'ram', 'name': 'RAM', 'size': 8192}]]
    authored_part = next(p for p in parts if p['feature'] == part['feature'])
    assert authored_part['memory'] == part['memory'][0]
    assert authored_part['package'] == part['package']
    package = chip['packages'][0]
    assert len(chip['packages']) == 1 and package['package'] == part['package']
    bonded = {signal: pin['position'] for pin in package['pins'] for signal in pin['signals']}
    safe = []
    for uart in [p for p in core['peripherals'] if p['name'] in ('UART1', 'UART2', 'UART3')]:
        for pin in uart['pins']:
            if pin['signal'] == 'TX' and pin['pin'] in bonded and pin['pin'] not in ('PA13', 'PA14', 'PF3'):
                safe.append((uart['name'], pin['pin'], pin['af'], bonded[pin['pin']]))
    assert set(safe) == {(p['peripheral'], p['pin'], p['af'], p['package_pad']) for p in part['safe_tx_pin_inventory']}
    assert part['safe_uart1_tx_available'] == any(p[0] == 'UART1' for p in safe)
    f8 = part['feature'] in ('cw32l031f8p6', 'cw32l031f8u6')
    assert part['safe_uart1_tx_available'] != f8
    routes = part['selected_example_tx_routes']
    assert len(routes) == (2 if f8 else 3)
    assert {r['peripheral'] for r in routes} == ({'UART2', 'UART3'} if f8 else {'UART1', 'UART2', 'UART3'})
    family = next(f for f in four['families'] if f['family'] == chip['line'])
    datasheet = four_family_pages[family['family']]['datasheet']
    product_table = datasheet[family['datasheet_pages'][0]['pdf_page_index']]
    product_row = next(line for line in product_table.splitlines() if '外设' in line and part['datasheet_product'] in line)
    products = re.findall(r'CW32[A-Z0-9]+', product_row)
    assert products[part['datasheet_memory_column']] == part['datasheet_product']
    for label, size in [('FLASH', 64), ('SRAM', 8)]:
        row = next(line for line in product_table.splitlines() if label + '（K' in line)
        values = [int(n) for n in re.findall(r'\d+', row)]
        assert values[part['datasheet_memory_column']] == size
    for route in routes:
        selected_count += 1
        assert (route['peripheral'], route['pin'], route['af'], route['package_pad']) in safe
        uart = next(p for p in core['peripherals'] if p['name'] == route['peripheral'])
        assert [r for r in uart['dma_channels'] if r['signal'] == 'TX'] == [route['request']]
        assert route['tdr_address'] == uart['address'] + 0x28
        assert route['uart_irq'] == uart['interrupts']
        expected_channel = {'UART1': 'DMA_CH4', 'UART2': 'DMA_CH2', 'UART3': 'DMA_CH3'}[route['peripheral']]
        assert route['dma_channel'] == expected_channel
        assert route['dma_irq'] == ('DMACH4' if expected_channel == 'DMA_CH4' else 'DMACH23')
        assert route['datasheet_af_pdf_page_index'] == family['gpioa_af_table_pdf_page_index']
        for key in ('datasheet_af_pdf_page_index', 'datasheet_pad_pdf_page_index'):
            assert route[key] in {p['pdf_page_index'] for p in family['datasheet_pages']}
            pin = route['pin'][:2] + route['pin'][2:].zfill(2)
            assert pin in datasheet[route[key]], (part['feature'], route['pin'], key)
        pad_page = datasheet[route['datasheet_pad_pdf_page_index']]
        pad_row = next(line for line in pad_page.splitlines() if re.search(rf'\b{pin}\b', line))
        pads = pad_row.split(pin, 1)[0].split()
        assert pads[part['datasheet_pad_column']] == route['package_pad'], (part['feature'], route)
        af_page = datasheet[route['datasheet_af_pdf_page_index']]
        af_row = next(line for line in af_page.splitlines() if re.match(rf'\s*{pin}\b', line))
        # The selected PA6/AF2, PA8/AF2, PA9/AF1 and PA10/AF6 rows
        # have no empty columns before the selected function.
        assert af_row.split()[route['af']] == route['peripheral'] + '_TXD', (part['feature'], route)
assert selected_count == 31
print('PASS: four own native DMA manuals/SDKs, 54 locked source identities, native ICR seed/PAC, shared CH2/3 and CH4 topology, 11 exact SRAM parts, four empty generics and 31 TX routes; source facts only')

# L012 has distinct native registers, current counts and paired IRQs. Bind its
# originals separately from the reviewed corrections and generated projections.
l012 = load(ROOT / 'docs/l012-dma-owned-evidence.json')
assert l012['schema_version'] == 1
assert len(l012['families']) == 1
family = l012['families'][0]
assert family['family'] == 'CW32L012'
l012_features = {'cw32l012c8t6', 'cw32l012c8u6'}
assert len(l012['exact_parts']) == 2
assert {p['feature'] for p in l012['exact_parts']} == l012_features
assert len(l012['generic_aliases']) == 1
assert l012['generic_aliases'][0]['feature'] == 'cw32l012'
l012_pointers = {'/artifacts/6'} | {
    f'/artifacts/6/members/{i}' for i in (17, 18, 19, 25, 28, 38, 39, 47, 60)
} | {f'/artifacts/{i}{suffix}' for i in (24, 25, 42) for suffix in ('', '/text')}
assert len(l012['sources']) == 16
assert {s['source_index_pointer'] for s in l012['sources']} == l012_pointers
l012_bytes = {}
for source in l012['sources']:
    pointer = source['source_index_pointer']
    pin = four_index(pointer)
    artifact = lock['artifacts'][int(pointer.split('/')[2])]
    identity = source['identity']
    if pointer == '/artifacts/42':
        # These retained superseded identities are not current source inputs.
        # Reverse only their explicit historical key labels before the same
        # full-node equality; no original pin-history field is omitted.
        identity = json.loads(json.dumps(identity))
        assert len(identity['pin_history']) == 2
        for historical in identity['pin_history']:
            assert set(historical) == {
                'historical_path', 'historical_sha256', 'bytes', 'changed_utc', 'reason'
            }
            historical['path'] = historical.pop('historical_path')
            historical['sha256'] = historical.pop('historical_sha256')
    observed = {k: v for k, v in pin.items() if k != 'evidence'
                and not (k == 'members' and pointer == '/artifacts/6')}
    expected = {k: v for k, v in identity.items() if k != 'evidence'}
    for value in (observed, expected):
        if 'text' in value:
            value['text'] = {k: v for k, v in value['text'].items() if k != 'evidence'}
    # Optional consumer backlinks are the only mutable selected-node property.
    # In particular the EN manual's pin_history and legal provenance stay bound.
    assert observed == expected, source['source_ref']
    assert source['official_url'] == artifact['url']
    if '/members/' in pointer:
        assert source['source_ref'] == 'member:' + identity['path']
        assert source['archive_source_ref'] == artifact['id']
    elif pointer.endswith('/text'):
        assert source['source_ref'] == 'text:' + identity['path']
    elif not pointer.endswith('/text'):
        assert source['source_ref'] == artifact['id']
    assert identity['path'] not in l012_bytes
    raw = (SOURCES / identity['path']).read_bytes()
    assert len(raw) == identity['bytes'] and digest(raw) == identity['sha256'], identity['path']
    l012_bytes[identity['path']] = raw
    if '/members/' in pointer:
        payload = (SOURCES / artifact['path']).read_bytes()
        for member in identity['members']:
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                payload = archive.read(member)
        assert payload == raw, identity['path']
assert len(l012_bytes) == 16

l012_pages = {}
for kind, page_delta in [('manual', 25), ('datasheet', 2), ('english_manual', 25)]:
    source = next(s for s in l012['sources'] if s['source_ref'] == family[kind + '_source_ref'])
    assert source['identity']['path'] == family[kind]
    assert source['identity']['text']['path'] == family[kind + '_text_path']
    pages = l012_bytes[family[kind + '_text_path']].decode().split('\f')
    records = family[kind + '_pages']
    assert len({p['pdf_page_index'] for p in records}) == len(records)
    for page in records:
        assert page['pdf_page_index'] == page['printed_page'] + page_delta
        assert page['pdf_viewer_page'] == page['pdf_page_index'] + 1
        assert digest(pages[page['pdf_page_index']].encode()) == page['extracted_page_utf8_sha256'], (kind, page)
    l012_pages[kind] = pages

manual = l012_pages['manual']
for claim, numbers in family['manual_page_claims'].items():
    assert numbers == [p['pdf_page_index'] for p in family['manual_pages'] if claim in p['claims']], claim
# Exact coordinates differ from the older four-family manuals.
assert {p['pdf_page_index'] for p in family['manual_pages']} >= {
    81, 82, 85, 86, 95, 96, 142, 143, 144, 145, 146, 147, 148,
    494, 499, 505, 507, 510, 512, 513,
}
assert {p['pdf_page_index'] for p in family['datasheet_pages']} >= {7, 35, 36, 38}
assert {p['pdf_page_index'] for p in family['english_manual_pages']} >= set(range(149, 160)) | {540, 545}
four_tokens(manual[145], ['13 TE RO', '12 TC RO', '11 RESTART RW', '10:8 STATUS RW',
            '000：初始状态', '001：传输错误，传输地址超出寻址范围', '010：传输错误，传输停止请求引起中止',
            '011：传输错误，访问传输来源地址出错', '100：传输错误，访问传输目的地址出错', '101：传输完成',
            '00：位宽为 8 比特，自增量为 1', '01：位宽为 16 比特，自增量为 2', '10：位宽为 32 比特，自增量为 4',
            '位宽配置仅对存储器（FLASH、RAM）有效', '0：批量传输（BULK）', '1：块传输（BLOCK）'], 'L012 CSR')
four_tokens(manual[144], ['15 DSTLOAD RW', '14 SRCLOAD RW', 'Reset value: 0xFFFF FFFF',
            '保留位，请保持默认值', 'W0：清除', 'W1：无功能'], 'L012 ICR/reload')
four_tokens(manual[146], ['y = 1~4', '7:2 HARDSRC RW', 'R0：传输已完成', 'R1：传输正在进行中',
            'W0：无功能', 'W1：立即触发该通道启动传输', '0：软件触发模式', '1：硬件触发模式'], 'L012 TRIG')
four_tokens(manual[147], ['19:16 REPEAT RW', '15:0 CNT RW', 'REPEAT 只可设置为 1',
            'REPEAT 不可设置为 0', 'CNT×REPEAT'], 'L012 configured count')
four_tokens(manual[148], ['Address offset: 0x14 + 32×y', '19:16 CREPEAT RO', '15:0 CCNT RO',
            '31:0 CSRCADDR RO', '31:0 CDSTADDR RO'], 'L012 current state')
four_tokens(l012_pages['english_manual'][154], ['13 TE RO', '12 TC RO', '10:8 STATUS RW',
            '101: Transfer completed', 'Bit width configuration is only valid for memory'], 'L012 EN CSR')
four_tokens(l012_pages['english_manual'][159], ['19:16 CREPEAT RO', '15:0 CCNT RO',
            '31:0 CSRCADDR RO', '31:0 CDSTADDR RO'], 'L012 EN current state')

l012_svd = ET.fromstring(l012_bytes[family['svd_path']])
assert l012_svd.findtext('name') == 'CW32L012'
l012_peripherals = {p.findtext('name'): p for p in l012_svd.findall('peripherals/peripheral')}
geometry = {p['name']: p for p in family['svd_geometry']}
assert len(geometry) == len(family['svd_geometry']) == 9
assert set(geometry) == {'DMA', 'SYSCTRL', 'UART1', 'UART2', 'UART3'} | {f'DMACHANNEL{i}' for i in range(1, 5)}
l012_registers = {}
for name, expected in geometry.items():
    peripheral = l012_peripherals[name]
    parent = ('DMACHANNEL1' if name in ('DMACHANNEL2', 'DMACHANNEL3', 'DMACHANNEL4')
              else 'UART1' if name in ('UART2', 'UART3') else None)
    assert peripheral.get('derivedFrom') == expected['derived_from'] == parent
    assert int(peripheral.findtext('baseAddress'), 0) == expected['base']
    assert [{'name': i.findtext('name'), 'number': int(i.findtext('value'), 0)}
            for i in peripheral.findall('interrupt')] == expected['interrupts']
    owner = l012_peripherals[parent] if parent else peripheral
    if parent:
        assert peripheral.find('registers') is None and owner.get('derivedFrom') is None
    observed = []
    for register in owner.findall('registers/register'):
        if name == 'SYSCTRL' and register.findtext('name') not in ('AHBEN', 'APBEN1', 'AHBRST', 'APBRST1'):
            continue
        if name.startswith('UART') and register.findtext('name') not in ('CR1', 'CR2', 'TDR', 'IER', 'ISR'):
            continue
        assert register.get('derivedFrom') is None
        fields = register.findall('fields/field')
        for field in fields:
            assert field.get('derivedFrom') is None
            assert {k for k in ('lsb', 'msb', 'bitOffset', 'bitWidth', 'bitRange')
                    if field.find(k) is not None} == {'lsb', 'msb'}
            assert field.findtext('access') in ('read-only', 'read-write', 'write-only')
        record = four_svd_register(register)
        record['access'] = register.findtext('access')
        assert record['access'] in ('read-only', 'read-write', 'write-only')
        observed.append(record)
    key = 'selected_registers' if name == 'SYSCTRL' or name.startswith('UART') else 'registers'
    assert observed == expected[key], ('L012 original SVD', name)
    assert len({r['name'] for r in observed}) == len(observed)
    l012_registers[name] = {r['name']: r for r in observed}

l012_offsets = dict(zip(('CSR', 'CNT', 'SRCADDR', 'DSTADDR', 'TRIG', 'CCNT', 'CSRCADDR', 'CDSTADDR'), range(0, 32, 4)))
l012_flags = {f'{kind}{i}': (4 * (i - 1) + bit, 1)
              for i in range(1, 5) for kind, bit in [('TC', 0), ('TE', 1)]}
l012_vectors = {'DMACH12': 9, 'DMACH34': 10, 'UART1': 27, 'UART2': 28, 'UART3': 29}
l012_channel_vectors = [('DMACH12', 9), ('DMACH12', 9), ('DMACH34', 10), ('DMACH34', 10)]
assert geometry['DMA']['base'] == 0x40020000
assert {r['name']: r['offset'] for r in l012_registers['DMA'].values()} == {
    'ISR': 0, 'ICR': 4, **{f'{r}{i}': 32 * i + offset for i in range(1, 5) for r, offset in l012_offsets.items()}}
for i in range(1, 5):
    name = f'DMACHANNEL{i}'
    assert geometry[name]['base'] == 0x40020000 + 32 * i
    vector, number = l012_channel_vectors[i - 1]
    assert geometry[name]['interrupts'] == [{'name': vector, 'number': number}]
    assert {r['name']: r['offset'] for r in l012_registers[name].values()} == l012_offsets

l012_ir = {}
for version, block, peripheral in [('dmachannel_cw32l012_v1', 'DMACHANNEL', 'DMACHANNEL1'),
                                   ('dma_cw32l012_v1', 'DMA', 'DMA')]:
    authored = load(ROOT / f'cw32-data/registers/{version}.yaml')
    generated = load(ROOT / f'cw32-data/data/registers/{version}.json')
    assert authored == generated == load(ROOT / f'cw32-data/data/registers/{version}.yaml')
    l012_ir[version] = generated
    assert {k for k in generated if k.startswith('block/')} == {'block/' + block}
    reuse = next(g for g in load(ROOT / 'cw32-data/register-reuse.yaml')['groups'] if g['canonical'] == version + '.yaml')
    assert reuse['source_versions'] == [version + '.yaml']
    assert digest(json.dumps(generated, sort_keys=True, separators=(',', ':')).encode()) == reuse['canonical_ir_sha256']
    update = next(u for u in l012['canonical_ir_updates'] if u['canonical'] == version + '.yaml')
    assert update['after_sha256'] == reuse['canonical_ir_sha256']
    items = generated['block/' + block]['items']
    originals = l012_registers[peripheral]
    assert len(items) == len(originals) and {r['name'] for r in items} == set(originals)
    assert {k for k in generated if k.startswith('fieldset/')} == {'fieldset/' + r for r in originals}
    for item in items:
        original = originals[item['name']]
        assert item['byte_offset'] == original['offset']
        assert item['fieldset'] == item['name'] and 'array' not in item
        assert item.get('bit_size', 32) == 32
        fields = generated['fieldset/' + item['fieldset']]['fields']
        assert len(fields) == len(original['fields'])
        assert {f['name']: (f['bit_offset'], f['bit_size']) for f in fields} == {
            f['name']: (f['bit_offset'], f['bit_size']) for f in original['fields']}
        corrected = item['name'].startswith('CCNT')
        expected_access = 'Read' if corrected else {'read-only': 'Read', 'read-write': 'ReadWrite'}[original['access']]
        assert item.get('access', 'ReadWrite') == expected_access
        if corrected:
            assert original['access'] == 'read-write'
            assert {f['name']: f['access'] for f in original['fields']} == {'CCNT': 'read-write', 'CREPEAT': 'read-write'}
        if re.fullmatch(r'CSR[1-4]?', item['name']):
            access = {f['name']: f['access'] for f in original['fields']}
            assert access['TC'] == access['TE'] == 'read-only' and access['STATUS'] == 'read-write'

channel_ir = l012_ir['dmachannel_cw32l012_v1']
native = l012_ir['dma_cw32l012_v1']
for fieldset in ('ISR', 'ICR'):
    assert {f['name']: (f['bit_offset'], f['bit_size']) for f in native['fieldset/' + fieldset]['fields']} == l012_flags
for field, (bit, width) in l012_flags.items():
    assert re.search(rf'(?m)^\s*{bit}\s+{field}\s+R1W0\b', manual[144]), field
assert {f['name']: (f['bit_offset'], f['bit_size']) for f in channel_ir['fieldset/CSR']['fields']} == {
    'EN': (0, 1), 'TCIE': (1, 1), 'TEIE': (2, 1), 'TRANS': (3, 1), 'SRCINC': (4, 1), 'DSTINC': (5, 1),
    'SIZE': (6, 2), 'STATUS': (8, 3), 'RESTART': (11, 1), 'TC': (12, 1), 'TE': (13, 1), 'SRCLOAD': (14, 1), 'DSTLOAD': (15, 1)}
expected_enums = {
    'TransferMode': {'bit_size': 1, 'variants': {'BULK': 0, 'BLOCK': 1}},
    'Width': {'bit_size': 2, 'variants': {'BYTE': 0, 'HALF_WORD': 1, 'WORD': 2}},
    'Status': {'bit_size': 3, 'variants': {'INITIAL': 0, 'ADDRESS_OUT_OF_RANGE': 1, 'STOPPED': 2,
                                       'SOURCE_ERROR': 3, 'DESTINATION_ERROR': 4, 'COMPLETE': 5}},
    'Trigger': {'bit_size': 1, 'variants': {'SOFTWARE': 0, 'HARDWARE': 1}},
}
assert l012['native_enums'] == expected_enums
assert {k for k in channel_ir if k.startswith('enum/')} == {'enum/' + n for n in expected_enums}
assert not any(k.startswith('enum/') for k in native)
for name, expected in expected_enums.items():
    enum = channel_ir['enum/' + name]
    assert enum['bit_size'] == expected['bit_size']
    assert len(enum['variants']) == len(expected['variants'])
    assert {v['name']: v['value'] for v in enum['variants']} == expected['variants']
associations = {('CSR', 'TRANS'): 'TransferMode', ('CSR', 'SIZE'): 'Width',
                ('CSR', 'STATUS'): 'Status', ('TRIG', 'TYPE'): 'Trigger'}
assert {(key.split('/')[1], f['name']): f['enum'] for key, value in channel_ir.items()
        if key.startswith('fieldset/') for f in value['fields'] if 'enum' in f} == associations
assert {tuple(a) for a in l012['field_associations']} == {(f'fieldset/{s}', f, e) for (s, f), e in associations.items()}
overrides = [o for o in load(ROOT / 'cw32-data/inputs/cw32l012.yaml')['register_overrides']
             if o['block'] in ('DMA', 'DMACHANNEL')]
assert overrides == l012['register_access_corrections'] and len(overrides) == 5
assert {(o['block'], o['register'], o['expected_access'], o['access']) for o in overrides} == {
    ('DMACHANNEL', 'CCNT', 'ReadWrite', 'Read'),
    *{('DMA', f'CCNT{i}', 'ReadWrite', 'Read') for i in range(1, 5)},
}
manual_source = next(s for s in l012['sources'] if s['source_ref'] == family['manual_source_ref'])
for override in overrides:
    four_tokens(override['evidence'], ['8.8.8', '123', '149', manual_source['identity']['sha256'],
                'SVD', 'CMSIS', 'RW', 'RO'], ('L012 access correction', override['register']))

assert set(l012['field_access_restrictions']) == set(l012_ir)
for version, expected in l012['field_access_restrictions'].items():
    restrictions = load(ROOT / 'cw32-data/field-access.yaml')['registers'][version]
    assert restrictions == expected
    block, suffixes = ('DMA', ('1', '2', '3', '4')) if version == 'dma_cw32l012_v1' else ('DMACHANNEL', ('',))
    assert len(restrictions) == 2 * len(suffixes)
    assert {(r['block'], r['register'], r['fieldset'], r['field'], r['bit_offset'], r['bit_size'])
            for r in restrictions} == {(block, 'CSR' + s, 'CSR' + s, f, bit, 1)
                                      for s in suffixes for f, bit in [('TC', 12), ('TE', 13)]}
    assert load(ROOT / f'cw32-data/data/field-access/{version}.json') == {
        'schema_version': 1, 'registers': {version: restrictions}}
    pac = (ROOT / f'cw32-metapac/src/peripherals/{version}.rs').read_text()
    for suffix in suffixes:
        csr = re.search(rf'\bimpl Csr{suffix} \{{(.*?)\n    \}}', pac, re.S)
        assert csr, (version, suffix)
        for field in ('tc', 'te'):
            assert len(re.findall(rf'pub const fn {field}\(&self\) -> bool', csr[1])) == 1
            assert not re.search(rf'\bfn set_{field}\(', csr[1])
        assert len(re.findall(r'pub const fn set_status\(', csr[1])) == 1
        register = 'ccnt' + suffix
        assert len(re.findall(rf'pub const fn {register}\(self\) -> crate::common::Reg<regs::Ccnt{suffix}, crate::common::R>', pac)) == 1
    if version == 'dmachannel_cw32l012_v1':
        for (fieldset, field), enum in associations.items():
            field = 'type_' if field == 'TYPE' else field.lower()
            assert len(re.findall(rf'pub const fn {field}\(&self\) -> super::vals::{enum}\b', pac)) == 1
            assert len(re.findall(rf'pub const fn set_{field}\(&mut self, val: super::vals::{enum}\)', pac)) == 1

seed = l012['native_icr_seed']
commands = load(ROOT / 'cw32-data/register-writes.yaml')['registers']['dma_cw32l012_v1']
assert len(commands) == 1
command = commands[0]
assert command == {k: v for k, v in seed.items() if k not in ('register_version', 'per_channel_clear_commands')}
assert seed['register_version'] == 'dma_cw32l012_v1'
assert command['block'] == 'DMA' and command['register'] == command['fieldset'] == 'ICR'
assert command['reset_value'] == command['write_noop'] == 0xffffffff
assert command['zero_to_clear_fields'] == [f'{kind}{i}' for i in range(1, 5) for kind in ('TC', 'TE')]
assert load(ROOT / 'cw32-data/data/register-writes/dma_cw32l012_v1.json') == {
    'schema_version': 1, 'registers': {'dma_cw32l012_v1': commands}}
assert [v['physical_channel'] for v in seed['per_channel_clear_commands']] == [1, 2, 3, 4]
for values in seed['per_channel_clear_commands']:
    i = values['physical_channel']
    for key, bits in [('tc_only', [4 * (i - 1)]), ('te_only', [4 * (i - 1) + 1]), ('both', [4 * (i - 1), 4 * (i - 1) + 1])]:
        assert int(values[key], 16) == 0xffffffff ^ sum(1 << bit for bit in bits)
pac = (ROOT / 'cw32-metapac/src/peripherals/dma_cw32l012_v1.rs').read_text()
assert pac.count('impl regs::Icr {') == 1
icr_helpers = pac.split('impl regs::Icr {', 1)[1]
for helper in ('reset_value', 'write_noop'):
    assert re.search(rf'pub const fn {helper}\(\) -> Self\s*\{{\s*Self\(4294967295\)\s*\}}', icr_helpers)
assert re.search(r'impl Default for Icr \{.*?fn default\(\) -> Icr\s*\{\s*Icr\(0\)', pac, re.S)

dma_header_path = 'cw32l012/Libraries/inc/cw32l012_dma.h'
dma_header = l012_bytes[dma_header_path].decode()
# The own SDK uses literal first values followed by implicit successors. This
# finite extraction never treats the Stage85 shifted request macros as L012.
l012_sdk_enums = {}
for body, name in re.findall(r'typedef\s+enum\s*\{([^{}]*)\}\s*(DMA_\w+TypeDef)\s*;', dma_header, re.S):
    value = -1
    values = {}
    body = re.sub(r'//[^\n]*|/\*.*?\*/', '', body, flags=re.S)
    for entry in body.split(','):
        if not entry.strip():
            continue
        match = re.fullmatch(r'\s*(DMA_\w+)(?:\s*=\s*(0x[0-9a-fA-F]+|[0-9]+))?\s*', entry)
        assert match, (name, entry)
        value = int(match[2], 0) if match[2] else value + 1
        assert match[1] not in values
        values[match[1]] = value
    assert name not in l012_sdk_enums
    l012_sdk_enums[name] = values
for name, values in {
    'DMA_ChannelTypeDef': {f'DMA_CHANNEL_{i}': i - 1 for i in range(1, 5)},
    'DMA_DataSizeTypeDef': {'DMA_DATA_SIZE_8BITS': 0, 'DMA_DATA_SIZE_16BITS': 1, 'DMA_DATA_SIZE_32BITS': 2},
    'DMA_ModeTypeDef': {'DMA_MODE_BULK': 0, 'DMA_MODE_BLOCK': 1},
    'DMA_TriggerTypeDef': {'DMA_TRIGGER_SOFTWARE': 0, 'DMA_TRIGGER_HARDWARE': 1},
    'DMA_StatusTypeDef': {'DMA_STATUS_IDLE': 0, 'DMA_STATUS_ERROR_ADDR': 1, 'DMA_STATUS_ERROR_STOP': 2,
                          'DMA_STATUS_ERROR_SRC': 3, 'DMA_STATUS_ERROR_DST': 4, 'DMA_STATUS_COMPLETE': 5},
}.items():
    assert l012_sdk_enums[name] == values

for symbol in family['sdk_symbols']:
    lines = l012_bytes[symbol['path']].decode().splitlines()
    assert lines[symbol['line'] - 1].strip() == symbol['text'].strip(), symbol
cmsis = l012_bytes['cw32l012/Libraries/inc/cw32l012.h'].decode()
for register in ['CCNT', 'CCNT1', 'CCNT2', 'CCNT3', 'CCNT4']:
    assert len(re.findall(rf'__IOM\s+uint32_t\s+{register}\s*;', cmsis)) == 1, register
assert len(re.findall(r'__IOM\s+uint32_t\s+CREPEAT\s*:\s*4\s*;', cmsis)) == 5
assert len(re.findall(r'__IOM\s+uint32_t\s+CCNT\s*:\s*16\s*;', cmsis)) == 5
for name, bit in [('TCIE', 1), ('TEIE', 2)]:
    assert re.search(rf'#define\s+DMA_CSR1_{name}_Pos\s+\({bit}UL\)', cmsis)
for field, (bit, width) in l012_flags.items():
    assert re.search(rf'#define\s+DMA_ICR_{field}_Pos\s+\({bit}UL\)', cmsis)
four_tokens(dma_header, ['#define DMA_INTERRUPT_TC DMA_CSR1_TCIE_Msk',
            '#define DMA_INTERRUPT_TE DMA_CSR1_TEIE_Msk'], 'L012 SDK interrupt mask domain')
dma_source = l012_bytes['cw32l012/Libraries/src/cw32l012_dma.c'].decode()
four_tokens(dma_source, ['*DMA_ICR &= ~(interrupt_type << channel);',
            '*DMA_CNTy(channel) = (transfer_size & 0xFFFF);'], 'L012 rejected SDK helpers')
assert len(l012['conflicts']) == 4
assert {c['id'] for c in l012['conflicts']} == {
    'current-count-access', 'mixed-csr-access-loss', 'sdk-clear-helper', 'sdk-config-channel-repeat'}

sidecar = load(ROOT / family['dma_metadata_path'])
assert sidecar['family'] == 'CW32L012'
assert sidecar['core_dma_channels'] == expected_channels
assert sidecar['channel_details'] == family['physical_channels']
assert len(family['physical_channels']) == 4
for index, channel in enumerate(family['physical_channels']):
    assert channel['physical_channel'] == index + 1
    assert channel['name'] == f'DMA_CH{index + 1}'
    assert channel['register_peripheral'] == f'DMACHANNEL{index + 1}'
    assert (channel['interrupt'], channel['interrupt_number']) == l012_channel_vectors[index]
requests = sidecar['requests']
tx_requests = [r for r in requests if r['peripheral'] in ('UART1', 'UART2', 'UART3') and r['signal'] == 'TX']
assert tx_requests == family['qualified_tx_requests']
assert [r for r in requests if r not in tx_requests] == family['unqualified_inventory_only']
assert [(r['peripheral'], r['request']) for r in tx_requests] == [('UART1', 1), ('UART2', 3), ('UART3', 5)]
assert len(requests) == 64 and [r['request'] for r in requests] == list(range(64))
assert l012_sdk_enums['DMA_TriggerSourceTypeDef'] == {r['sdk']['symbol']: r['request'] for r in requests}
for request in requests:
    assert request['channels'] == [c['name'] for c in expected_channels] and request['dma'] == 'DMA'
    assert request['sdk']['encoding'] == 'unshifted_enum'
    assert request['sdk']['raw_value'] == request['request']
    assert request['manual']['pdf_page_index'] == 146 and request['manual']['printed_page'] == 121
    assert int(request['manual']['binary'], 2) == request['request']
    four_tokens(manual[146], [request['manual']['binary'] + '：' + request['manual']['description']], request['sdk']['symbol'])
    assert re.match(r'\s*' + re.escape(request['sdk']['symbol']) + r'\b', dma_header.splitlines()[request['sdk']['line'] - 1])
assert sidecar['request_register'] == {'peripheral_kind': 'dmachannel', 'register': 'TRIG', 'field': 'HARDSRC', 'bit_offset': 2, 'bit_size': 6}
assert sidecar['excluded_sdk_requests'] == []
assert sidecar['peripheral_dma_channels'] == {
    name: [{'signal': r['signal'], 'dma': r['dma'], 'request': r['request']} for r in requests if r['peripheral'] == name]
    for name in {r['peripheral'] for r in requests}}

clock_names = {'DMA', 'GPIOA', 'UART1', 'UART2', 'UART3'}
clocks = load(ROOT / family['clock_metadata_path'])['peripherals']
clock_facts = [{k: v for k, v in c.items() if k not in ('notes', 'evidence')}
               for c in clocks if c['name'] in clock_names]
assert clock_facts == family['clocks'] and len(clock_facts) == 5
assert {c['name'] for c in clock_facts} == clock_names
sysctrl_ir = load(ROOT / 'cw32-data/data/registers/sysctrl_cw32l012_v1.json')
for register, offset, page in [('AHBEN', 0x30, 81), ('APBEN1', 0x38, 82), ('AHBRST', 0x40, 85), ('APBRST1', 0x48, 86)]:
    original = l012_registers['SYSCTRL'][register]
    assert original['offset'] == offset and original['access'] == 'read-write'
    item = next(r for r in sysctrl_ir['block/SYSCTRL']['items'] if r['name'] == register)
    assert item['byte_offset'] == offset and item.get('access', 'ReadWrite') == 'ReadWrite'
    assert {f['name']: (f['bit_offset'], f['bit_size']) for f in sysctrl_ir['fieldset/' + register]['fields']} == {
        f['name']: (f['bit_offset'], f['bit_size']) for f in original['fields']}
    fields = {f['name']: f for f in original['fields']}
    if register in ('AHBEN', 'APBEN1'):
        assert (fields['KEY']['bit_offset'], fields['KEY']['bit_size'], fields['KEY']['access']) == (16, 16, 'write-only')
        four_tokens(manual[page], ['31:16 KEY WO', 'KEY 为 0x5A5A', '0：禁止', '1：使能'], register)
    else:
        assert 'KEY' not in fields
        four_tokens(manual[page], ['0：模块处于复位状态', '1：模块正常工作'], register)
    for name, bit in ({'DMA': 0, 'GPIOA': 4} if register.startswith('AHB') else {'UART1': 3, 'UART2': 4, 'UART3': 8}).items():
        assert (fields[name]['bit_offset'], fields[name]['bit_size'], fields[name]['access']) == (bit, 1, 'read-write')
        four_tokens(manual[page], [f'{bit} {name} RW'], (register, name))
for clock in clock_facts:
    name = clock['name']
    uart = name.startswith('UART')
    enable, reset = ('APBEN1', 'APBRST1') if uart else ('AHBEN', 'AHBRST')
    assert clock['enable'] == {'peripheral': 'SYSCTRL', 'register': enable, 'field': name}
    assert clock['reset'] == {'peripheral': 'SYSCTRL', 'register': reset, 'field': name}
    assert clock['enable_active_value'] == 1 and clock['reset_asserted_value'] == 0
    assert clock['enable_write_key'] == {'field': {'peripheral': 'SYSCTRL', 'register': enable, 'field': 'KEY'}, 'value': 0x5a5a}
    assert clock['bus_clock'] == ('PCLK' if uart else 'HCLK')
    if uart:
        assert clock['status'] == 'partial'
        assert clock['kernel_clock'] == {'mux': {'peripheral': name, 'register': 'CR1', 'field': 'SOURCE'}}
        assert clock['kernel_sources'] == ['PCLK', 'LSE', 'LSI']
    if name == 'DMA':
        assert clock['shared_enable_group'] == clock['shared_reset_group'] == 'DMA-DMACHANNEL1-DMACHANNEL2-DMACHANNEL3-DMACHANNEL4'

uart_ir = load(ROOT / 'cw32-data/data/registers/uart_cw32l012_v1.json')
for i, base in [(1, 0x40000c00), (2, 0x40001000), (3, 0x40002000)]:
    name = f'UART{i}'
    assert geometry[name]['base'] == base
    assert geometry[name]['interrupts'] == [{'name': name, 'number': 26 + i}]
    assert set(l012_registers[name]) == {'CR1', 'CR2', 'TDR', 'IER', 'ISR'}
    for register, original in l012_registers[name].items():
        item = next(r for r in uart_ir['block/UART']['items'] if r['name'] == register)
        assert item['byte_offset'] == original['offset']
        assert item.get('access', 'ReadWrite') == {'read-only': 'Read', 'write-only': 'Write', 'read-write': 'ReadWrite'}[original['access']]
        fields = uart_ir['fieldset/' + item['fieldset']]['fields']
        assert len(fields) == len(original['fields'])
        assert {f['name']: (f['bit_offset'], f['bit_size']) for f in fields} == {
            f['name']: (f['bit_offset'], f['bit_size']) for f in original['fields']}
tdr = l012_registers['UART1']['TDR']
assert tdr['offset'] == 0x28 and tdr['access'] == 'write-only'
assert {f['name']: (f['bit_offset'], f['bit_size'], f['access']) for f in tdr['fields']} == {
    'TDR': (0, 9, 'write-only'), 'IDLE': (9, 1, 'write-only'), 'BREAK': (10, 1, 'write-only')}
four_tokens(manual[507], ['Address offset: 0x04', '7 DMATX RW'], 'L012 UART CR2')
four_tokens(manual[510], ['UARTx_TDR', 'Address offset: 0x28', '8:0 TDR WO'], 'L012 UART TDR')
four_tokens(manual[512], ['Address offset: 0x1C', '14 TXBUSY RO'], 'L012 wire completion')
four_tokens(manual[505], ['13:12 SOURCE RW', '00：UCLK 来自 PCLK', '01：UCLK 来自 PCLK',
            '10：UCLK 来自 LSE', '11：UCLK 来自 LSI'], 'L012 local UART clock')
assert {v['name']: v['value'] for v in uart_ir['enum/Source']['variants']} == {'PCLK': 0, 'PCLK_ALT': 1, 'LSE': 2, 'LSI': 3}
four_tokens(manual[494], ['UARTx_CR2.DMATX 为 1', 'UARTx_TDR', 'DMA_ISR.TCy', 'UARTx_ISR.TXBUSY'], 'L012 UART DMA')
tx = four_compact(manual[499])
four_tokens(tx, ['PA09', 'SRAM', 'uint8_t SendBuff[100]', 'DMA_SRCADDR1 = (uint32_t) SendBuff',
            'DMA_DSTADDR1 = (uint32_t) &UART1_TDR', 'DMA_CSR1.SRCINC 为 1', 'DMA_CSR1.DSTINC 为 0',
            'DMA_CNT1 写入 0x10064', 'DMA_CSR1.SIZE 为 0', 'DMA_CSR1.TRANS 为 1',
            'DMA_TRIG1. HARDSRC 为 0x01'], 'L012 SRAM-byte TX example')
ordered = ['DMA_CSR1.EN 为 1', 'UART1_CR2.DMATX 为 1', 'DMA_ISR.TC1 变为 1',
           'UART1_CR2.DMATX 为 0', 'UART1_ISR.TXBUSY 变为 0']
positions = [tx.index(four_compact(token)) for token in ordered]
assert positions == sorted(positions)
for number, name in [(9, 'DMA1/2'), (10, 'DMA3/4')]:
    assert re.search(rf'(?m)^\s*{number + 16}\s+{number}\s+可配置\s+{name}\b', manual[95]), name
for number, name in [(27, 'UART1'), (28, 'UART2'), (29, 'UART3')]:
    assert re.search(rf'(?m)^\s*{number + 16}\s+{number}\s+可配置\s+{name}\b', manual[96]), name

pdsc_path = next(path for path in l012_bytes if path.endswith('.pdsc'))
pdsc = ET.fromstring(l012_bytes[pdsc_path])
devices = pdsc.findall(".//device[@Dname='CW32L012C8']")
assert len(devices) == 1
assert {m.get('name'): (int(m.get('start'), 0), int(m.get('size'), 0), m.get('access'))
        for m in devices[0].findall('memory')} == {
    'IROM1': (0, 0x10000, 'rx'), 'IRAM1': (0x20000000, 0x2000, 'rw')}
datasheet = l012_pages['datasheet']
four_tokens(datasheet[7], ['CW32L012C8T6', 'CW32L012C8U6', 'FLASH 64KB', 'SRAM 8KB', 'LQFP48', 'QFN48'], 'L012 exact parts')
parts = load(ROOT / 'cw32-data/parts.yaml')['parts']
assert {p['feature'] for p in parts if p['family'] == 'CW32L012'} == l012_features
pinout = load(ROOT / 'cw32-data/pinouts/cw32l012.yaml')
af_routes = load(ROOT / 'cw32-data/af/cw32l012-serial.yaml')['routes']
selected_count = 0
for part in l012['exact_parts'] + l012['generic_aliases']:
    chip = load(ROOT / part['metadata_path'])
    assert chip['name'].lower() == part['feature'] and chip['line'] == 'CW32L012'
    assert chip['memory'] == part['memory']
    assert len(chip['cores']) == 1
    core = chip['cores'][0]
    assert core['dma_channels'] == part['dma_channels'] == expected_channels
    instances = [p for p in core['peripherals'] if p['name'] == 'DMA' or p['name'].startswith('DMACHANNEL')]
    assert instances == part['dma_instances'] and len(instances) == 5
    for instance in instances:
        name = instance['name']
        assert name in geometry and instance['address'] == geometry[name]['base']
        assert instance['registers'] == {'block': 'DMA' if name == 'DMA' else 'DMACHANNEL',
                                         'kind': 'dma' if name == 'DMA' else 'dmachannel', 'version': 'cw32l012_v1'}
        assert instance['interrupts'] == [{'interrupt': i['name'], 'signal': 'GLOBAL'} for i in geometry[name]['interrupts']]
        assert instance['rcc'] == instances[0]['rcc'] and instance['rcc_control'] == instances[0]['rcc_control']
    vectors = [v for v in core['interrupts'] if v['name'].startswith('DMA') or v['name'].startswith('UART')]
    assert vectors == part['dma_uart_vectors']
    assert len(vectors) == 5 and {v['name']: v['number'] for v in vectors} == l012_vectors
    assert [v for v in core['interrupts'] if v['number'] == 11] == [{'name': 'CORDIC', 'number': 11}]
    for name in clock_names:
        peripheral = next(p for p in core['peripherals'] if p['name'] == name)
        clock = next(c for c in clock_facts if c['name'] == name)
        control = peripheral['rcc_control']
        assert control['controller'] == 'SYSCTRL' and control['bus_clock'] == clock['bus_clock']
        for key in ('enable', 'reset'):
            assert control[key] == {k: v for k, v in clock[key].items() if k != 'peripheral'}
        assert control['enable_active_value'] is True and control['reset_asserted_value'] is False
        assert control['enable_write_key'] == {'field': {'register': clock['enable']['register'], 'field': 'KEY'}, 'value': 0x5a5a}
        assert 'reset_write_key' not in control
    if part['feature'] == 'cw32l012':
        assert part['memory'] == [] and chip['packages'] == []
        continue
    assert part['flash_interval'] == [0, 0x10000]
    assert part['sram_interval'] == [0x20000000, 0x20002000]
    assert part['memory'] == [[{'address': 0, 'kind': 'flash', 'name': 'FLASH', 'size': 65536},
                              {'address': 0x20000000, 'kind': 'ram', 'name': 'RAM', 'size': 8192}]]
    authored_part = next(p for p in parts if p['feature'] == part['feature'])
    assert authored_part['memory'] == part['memory'][0] and authored_part['package'] == part['package']
    assert part['package'] == ('LQFP48' if part['feature'] == 'cw32l012c8t6' else 'QFN48')
    assert len(chip['packages']) == 1
    package = chip['packages'][0]
    assert package['package'] == part['package']
    bonded = {signal: pin['position'] for pin in package['pins'] for signal in pin['signals']}
    assert part['datasheet_product'] == chip['name']
    assert part['datasheet_product_pdf_page_index'] == 7
    assert part['datasheet_pad_column'] == (0 if part['package'] == 'LQFP48' else 1)
    assert part['safe_uart1_tx_available'] is True
    routes = part['selected_example_tx_routes']
    assert len(routes) == 3
    assert {(r['peripheral'], r['pin'], r['af'], r['package_pad']) for r in routes} == {
        ('UART1', 'PA9', 1, '30'), ('UART2', 'PA7', 1, '17'), ('UART3', 'PA8', 1, '29')}
    for route in routes:
        selected_count += 1
        uart = next(p for p in core['peripherals'] if p['name'] == route['peripheral'])
        assert uart['registers'] == {'block': 'UART', 'kind': 'uart', 'version': 'cw32l012_v1'}
        assert uart['address'] == geometry[uart['name']]['base']
        assert {'signal': 'TX', 'pin': route['pin'], 'af': 1} in uart['pins']
        assert bonded[route['pin']] == route['package_pad']
        assert [r for r in uart['dma_channels'] if r['signal'] == 'TX'] == [route['request']]
        assert route['request'] == {'dma': 'DMA', 'request': {'UART1': 1, 'UART2': 3, 'UART3': 5}[uart['name']], 'signal': 'TX'}
        assert route['tdr_address'] == uart['address'] + 0x28
        assert route['uart_irq'] == uart['interrupts'] == [{'interrupt': uart['name'], 'signal': 'GLOBAL'}]
        assert (route['dma_channel'], route['dma_irq']) == {
            'UART1': ('DMA_CH1', 'DMACH12'), 'UART2': ('DMA_CH2', 'DMACH12'), 'UART3': ('DMA_CH4', 'DMACH34')}[uart['name']]
        pin = route['pin'][:2] + route['pin'][2:].zfill(2)
        assert route['datasheet_af_pdf_page_index'] == family['gpioa_af_table_pdf_page_index'] == 38
        af_row = next(line for line in datasheet[38].splitlines() if re.match(rf'\s*{pin}\b', line))
        assert af_row.split()[1] == uart['name'] + '_TXD'
        page = route['datasheet_pad_pdf_page_index']
        assert page == (35 if pin == 'PA07' else 36)
        pad_row = next(line for line in datasheet[page].splitlines() if re.search(rf'\b{pin}\b', line))
        assert pad_row.split()[:6] == [route['package_pad'], route['package_pad'], pin, 'I/O', 'TTa', '-']
        rows = [r for r in pinout['table_rows'] if route['pin'] in r['signals']]
        assert len(rows) == 1 and rows[0]['positions'][part['package']] == route['package_pad']
        assert rows[0]['source_name'] == pin and rows[0]['pin_type'] == 'I/O' and rows[0]['pdf_page_index'] == page
        authored_routes = [r for r in af_routes if (r['peripheral'], r['signal'], r['pin'], r['af']) == (uart['name'], 'TX', route['pin'], 1)]
        assert len(authored_routes) == 1
        authored_route = authored_routes[0]
        gpio_line = l012_bytes['cw32l012/Libraries/inc/cw32l012_gpio.h'].decode().splitlines()[authored_route['source_line'] - 1]
        assert authored_route['source_macro'] == f'{pin}_AFx_{uart["name"]}TXD'
        assert re.search(r'#define\s+' + re.escape(authored_route['source_macro']) + r'\b', gpio_line)
        four_tokens(gpio_line, [f'CW_GPIOA->{authored_route["gpio_register"]}_f.PIN{int(pin[2:])} = 1'], (part['feature'], pin))
assert selected_count == 6
print('PASS: L012 16 locked own identities and nested bytes, native enums/CCNT corrections/ten RO fields/ICR PAC, paired IRQs, 64 source request facts, two exact 8 KiB parts and six selected TX routes; no HAL execution')

# Five independently qualified classic UART RX endpoints. These are finite
# current source/data/PAC checks, not a runtime simulation or acceptance receipt.
import tomllib

classic_path = 'docs/classic-uart-dma-rx-evidence.json'
classic_checker = 'tests/verify_dma_owned_evidence.py'
classic = load(ROOT / classic_path)
classic_families = {'CW32F020', 'CW32L031', 'CW32L052', 'CW32R031', 'CW32W031'}
classic_features = {
    'cw32f020c6u7', 'cw32f020k6u7', 'cw32f020f6u7',
    'cw32l031c8t6', 'cw32l031c8u6', 'cw32l031f8p6', 'cw32l031f8u6',
    'cw32l031k8u6', 'cw32l031k8v6', 'cw32l052c8t6', 'cw32l052r8s6',
    'cw32l052r8t6', 'cw32r031c8u6', 'cw32w031r8u6',
}
assert classic['schema_version'] == 1
assert set(classic['families']) == set(classic['scope']['families']) == classic_families
assert classic['scope']['exact_part_count'] == len(classic['exact_parts']) == 14
assert {p['feature'] for p in classic['exact_parts']} == classic_features
assert classic['excluded_safe_pins'] == ['PA13', 'PA14', 'PF3']
assert 'L012 DMA RX' in classic['scope']['excluded']
assert classic['shared_facts']['uart_fields'] == {'DMARX': [6, 1], 'DMATX': [7, 1],
        'RC': [2, 1], 'FE': [3, 1], 'PE': [4, 1], 'RDR': [0, 9]}
assert classic['shared_facts']['dma_fields'] == {'STATUS': [8, 3], 'REPEAT': [16, 4],
        'CNT': [0, 16], 'HARDSRC': [2, 6]}
assert classic['shared_facts']['example_cnt'] == (1 << 16) | 100
assert classic['shared_facts']['dma_icr_noop'] == 0xffffffff
assert classic['shared_facts']['error_statuses'] == [1, 2, 3, 4]
assert classic['shared_facts']['cpu_mmio_bits'] == 32
assert classic['shared_facts']['silent_overwrite_possible'] is True
# Each family consumes exactly one SDK archive, two PDFs, their texts and six
# selected SDK members. The existing source index gives child nodes no IDs.
classic_pointers = set()
for sdk, manual, datasheet, members in [
    (2, 18, 40, [2, 17, 31, 9, 7, 0]),
    (7, 27, 26, [2, 17, 35, 9, 7, 0]),
    (8, 29, 28, [2, 19, 39, 9, 7, 0]),
    (10, 33, 32, [2, 17, 32, 9, 7, 0]),
    (11, 35, 34, [2, 17, 32, 9, 7, 0]),
]:
    classic_pointers.add(f'/artifacts/{sdk}')
    classic_pointers.update(f'/artifacts/{sdk}/members/{i}' for i in members)
    classic_pointers.update(f'/artifacts/{i}{suffix}' for i in (manual, datasheet) for suffix in ('', '/text'))
assert len(classic['sources']) == len(classic_pointers) == 55
assert {s['index_pointer'] for s in classic['sources'].values()} == classic_pointers
contract = classic['source_index_append_plan']
assert contract['new_consumer'] == classic_path and contract['checker_extension'] == classic_checker
assert len(contract['bound_source_refs']) == 55
assert set(contract['bound_source_refs']) == set(classic['sources'])
assert {ref for family in classic['families'].values() for ref in family['sources'].values()} == set(classic['sources'])
classic_bytes = {}
for ref, source in classic['sources'].items():
    pointer = source['index_pointer']
    pin = four_index(pointer)
    artifact = lock['artifacts'][int(pointer.split('/')[2])]
    observed = {k: v for k, v in pin.items() if k != 'evidence'
                and not (k == 'members' and ref.startswith('vendor:'))}
    if 'text' in observed:
        observed['text'] = {k: v for k, v in observed['text'].items() if k != 'evidence'}
    assert observed == source['identity'], ref
    assert {k: source[k] for k in ('path', 'sha256', 'bytes')} == {
        k: pin[k] for k in ('path', 'sha256', 'bytes')}
    assert source['official_url'] == artifact['url']
    assert pin['evidence'].count(classic_path) == pin['evidence'].count(classic_checker) == 1
    if '/members/' in pointer:
        assert ref == 'member:' + pin['path'] and 'id' not in pin
        assert source['archive_member_chain'] == pin['members']
    elif pointer.endswith('/text'):
        assert ref == 'text:' + pin['path'] and 'id' not in pin
    else:
        assert ref == artifact['id']
    raw = (SOURCES / pin['path']).read_bytes()
    assert len(raw) == source['bytes'] and digest(raw) == source['sha256'], ref
    assert pin['path'] not in classic_bytes
    classic_bytes[pin['path']] = raw
    if '/members/' in pointer:
        payload = (SOURCES / artifact['path']).read_bytes()
        for member in source['archive_member_chain']:
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                payload = archive.read(member)
        assert payload == raw, ref
classic_reverse = set()
for index, artifact in enumerate(lock['artifacts']):
    nodes = [(f'/artifacts/{index}', artifact)]
    nodes += [(f'/artifacts/{index}/members/{i}', member) for i, member in enumerate(artifact.get('members', []))]
    if 'text' in artifact:
        nodes.append((f'/artifacts/{index}/text', artifact['text']))
    for pointer, node in nodes:
        if classic_path in node.get('evidence', []):
            assert node['evidence'].count(classic_path) == 1
            classic_reverse.add(pointer)
        assert node.get('evidence', []).count(classic_checker) <= 1
assert classic_reverse == classic_pointers

classic_pages = {}
classic_channels = {}
classic_uart_ir = {}
page_count = 0
status_access_views = 0
assert len(classic['source_discrepancies']) == 1
status_discrepancy = classic['source_discrepancies'][0]
assert status_discrepancy['id'] == 'dma-status-access'
assert set(status_discrepancy['families']) == classic_families
assert (status_discrepancy['own_svd_access'], status_discrepancy['own_manual_access'],
        status_discrepancy['current_ir_access'], status_discrepancy['field'],
        status_discrepancy['selected_register_views']) == ('read-only', 'RW', 'ReadWrite', 'STATUS', 23)
for name, family in classic['families'].items():
    refs = family['sources']
    assert len(refs) == 11
    inputs = {kind: classic['sources'][ref] for kind, ref in refs.items()}
    assert inputs['sdk']['identity']['family'] == name
    for kind in ('cmsis_header', 'uart_header', 'uart_driver', 'gpio_header', 'dma_header', 'svd'):
        assert name.lower() in inputs[kind]['path'].lower()
    for kind in ('manual', 'datasheet'):
        assert inputs[kind]['identity']['text']['path'] == inputs[kind + '_text']['path']
        pages = classic_bytes[inputs[kind + '_text']['path']].decode().split('\f')
        classic_pages[name, kind] = pages
        hashes = family[kind + '_page_utf8_sha256']
        for number, sha in hashes.items():
            assert digest(pages[int(number)].encode()) == sha, (name, kind, number)
            page_count += 1
    claims = family['manual_claim_pages']
    assert {int(p) for p in family['manual_page_utf8_sha256']} == {p for pages in claims.values() for p in pages}
    manual = classic_pages[name, 'manual']
    claim_text = {claim: '\n'.join(manual[p] for p in pages) for claim, pages in claims.items()}
    assert claims['byte_rx_example'] == [{'CW32F020': 288, 'CW32L031': 345, 'CW32L052': 382,
                                         'CW32R031': 348, 'CW32W031': 348}[name]]
    rx = four_compact(claim_text['byte_rx_example'])
    four_tokens(rx, ['UART1_RXD', 'PA09', 'SRAM', 'uint8_t RecvBuff[100]',
                'DMA_SRCADDR2 = (uint32_t) &UART1_RDR', 'DMA_DSTADDR2 = (uint32_t) RecvBuff',
                'DMA_CSR2.SRCINC 为 0', 'DMA_CSR2.DSTINC 为 1', 'DMA_CNT2 写入 0x10064',
                'DMA_CSR2.SIZE 为 0', 'DMA_CSR2.TRANS 为 1', 'DMA_TRIG2.Type 为 1',
                'DMA_TRIG2.HardSrc 为 0x00'], (name, 'own byte RX'))
    ordered = ['DMA_CSR2.EN 为 1', 'UART1_CR2.DMARX 为 1', 'DMA_ISR.TC2 变为 1', 'UART1_CR2.DMARX 为 0']
    positions = [rx.index(four_compact(token)) for token in ordered]
    assert positions == sorted(positions), (name, 'RX launch and terminal order')
    four_tokens(claim_text['receive_errors'], ['UARTx_ISR.RC', 'UARTx_ISR.PE', 'UARTx_ISR.FE',
                '新接收的数据会覆盖 UARTx_RDR 寄存器', '造成数据丢失'], (name, 'receive errors and overwrite'))
    four_tokens(claim_text['terminal'], ['数据传输全部正确完成', 'TEIE', 'TCIE'], (name, 'terminal'))
    csr = claim_text['dma_csr' if name == 'CW32F020' else 'csr']
    four_tokens(csr, ['10:8 STATUS RW', '000：初始状态', '001：传输错误，传输地址超出寻址范围',
                '010：传输错误，传输停止请求引起中止', '011：传输错误，访问传输来源地址出错',
                '100：传输错误，访问传输目的地址出错', '101：传输完成',
                '00：8 比特', '0：批量传输（BULK）', '1：块传输（BLOCK）'], (name, 'STATUS'))
    trig = claim_text['request_and_trigger' if name == 'CW32F020' else 'trig']
    four_tokens(trig, ['7:2 HARDSRC RW', 'R0：传输已完成', 'W0：无功能',
                '0：软件触发模式', '1：硬件触发模式'], (name, 'TRIG'))
    count = claim_text['repeat_count' if name == 'CW32F020' else 'count_and_addresses']
    four_tokens(count, ['19:16 REPEAT RW', '15:0 CNT RW'], (name, 'count geometry'))
    four_tokens(claim_text['sram'], ['8KB', 'SRAM', '0x2000 0000'], (name, 'SRAM'))
    mmio = claim_text['cpu_mmio_width' if name == 'CW32F020' else 'mmio_width_and_gates']
    four_tokens(mmio, ['32bit'], (name, 'ordinary CPU register width'))
    four_tokens(claim_text['uart_layout'], ['UART1_BASE = 0x4001 3800', 'UART2_BASE = 0x4000 4400',
                'UART3_BASE = 0x4000 4800'], (name, 'own UART instances'))
    four_tokens(claim_text['uart_cr2'], ['6 DMARX RW', '7 DMATX RW'], (name, 'request gates'))
    four_tokens(claim_text['uart_rdr'], ['Address offset: 0x24', '8:0 RDR RO'], (name, 'RDR'))
    for register, access in [('ier', 'RW'), ('isr', 'RO'), ('icr', 'R1W0')]:
        four_tokens(claim_text['uart_' + register], [f'{bit} {field} {access}' for field, bit in [('RC', 2), ('FE', 3), ('PE', 4)]], (name, register))
    seed = 255 if name == 'CW32F020' else 4095
    assert family['uart_icr_noop'] == seed
    four_tokens(claim_text['uart_icr'], ['Address offset: 0x20', f'Reset value: 0x{seed:08X}',
                '保留位，请保持默认值', 'W0：清除', 'W1：无功能'], (name, 'UART clear seed'))
    driver_lines = classic_bytes[inputs['uart_driver']['path']].decode().splitlines()
    assert len(family['sdk_rc_no_read_clear_lines']) == len(family['sdk_rc_no_read_clear_observed']) == 2
    for line, observed in zip(family['sdk_rc_no_read_clear_lines'], family['sdk_rc_no_read_clear_observed']):
        assert driver_lines[line - 1].strip() == observed
        four_tokens(observed, ['只能软件清除', '不能通过读UARTx_RDR寄存器来清除'], (name, 'RC no-read-clear'))
    cmsis = classic_bytes[inputs['cmsis_header']['path']].decode()
    channels = 2 if name == 'CW32F020' else 4
    expected_channels = [{'channel': i - 1, 'dma': 'DMA', 'name': f'DMA_CH{i}'} for i in range(1, channels + 1)]
    classic_channels[name] = expected_channels
    vectors = [('DMACH1', 9), ('DMACH2', 10)] if channels == 2 else [('DMACH1', 9), ('DMACH23', 10), ('DMACH23', 10), ('DMACH4', 11)]
    irq_map = dict(vectors + [('UART1', 27), ('UART2', 28), ('UART3', 29)])
    assert family['cmsis_irqs'] == {irq + '_IRQn': number for irq, number in irq_map.items()}
    for irq, number in family['cmsis_irqs'].items():
        assert re.search(rf'\b{irq}\s*=\s*{number}\b', cmsis), (name, irq)
    for irq, number in irq_map.items():
        manual_name = irq.replace('DMACH', 'DMA')
        assert re.search(rf'(?m)^\s*{number + 16}\s+{number}\s+可配置\s+{manual_name}\b', claim_text['irq']), (name, irq)
    svd = ET.fromstring(classic_bytes[inputs['svd']['path']])
    peripherals = {p.findtext('name'): p for p in svd.findall('peripherals/peripheral')}
    assert {p for p in peripherals if p.startswith('DMACHANNEL')} == {f'DMACHANNEL{i}' for i in range(1, channels + 1)}
    for peripheral_name, expected in family['svd_geometry'].items():
        peripheral = peripherals[peripheral_name]
        assert int(peripheral.findtext('baseAddress'), 0) == int(expected['base'], 0)
        assert peripheral.get('derivedFrom') == expected.get('derived_from')
        owner = peripherals[expected['derived_from']] if 'derived_from' in expected else peripheral
        if 'derived_from' in expected:
            assert peripheral.find('registers') is None and owner.get('derivedFrom') is None
        registers = {r.findtext('name'): r for r in owner.findall('registers/register')}
        for register, offset in expected.get('register_offsets', {}).items():
            assert int(registers[register].findtext('addressOffset'), 0) == offset
        if peripheral_name.startswith('UART'):
            assert [(i.findtext('name'), int(i.findtext('value'), 0)) for i in peripheral.findall('interrupt')] == [(peripheral_name, 26 + int(peripheral_name[-1]))]
        elif peripheral_name.startswith('DMACHANNEL'):
            assert [(i.findtext('name'), int(i.findtext('value'), 0)) for i in peripheral.findall('interrupt')] == [vectors[int(peripheral_name[-1]) - 1]]
    uart_version = 'uart_' + family['uart_register_version']
    dma_version = 'dma_cw32f020_v1' if channels == 2 else 'dma_cw32l031_v1'
    for version, block, peripheral_name in [(uart_version, 'UART', 'UART1'), (dma_version, 'DMA', 'DMA'), ('dmachannel_v1', 'DMACHANNEL', 'DMACHANNEL1')]:
        ir = load(ROOT / f'cw32-data/data/registers/{version}.json')
        assert ir == load(ROOT / f'cw32-data/registers/{version}.yaml')
        original_registers = {r.findtext('name'): r for r in peripherals[peripheral_name].findall('registers/register')}
        own = {register: four_svd_register(r) for register, r in original_registers.items()}
        items = {r['name']: r for r in ir['block/' + block]['items']}
        selected = {'CR1', 'CR2', 'IER', 'ISR', 'ICR', 'RDR', 'TDR'} if block == 'UART' else set(own)
        if block != 'UART':
            assert set(items) == set(own)
        pac = (ROOT / f'cw32-metapac/src/peripherals/{version}.rs').read_text()
        for register in selected:
            item = items[register]
            assert item['byte_offset'] == own[register]['offset']
            access_names = {'read-only': 'Read', 'write-only': 'Write', 'read-write': 'ReadWrite'}
            register_access = original_registers[register].findtext('access')
            assert item.get('access', 'ReadWrite') == access_names[register_access], (name, register, 'access')
            fields = {f['name']: (f['bit_offset'], f['bit_size']) for f in ir['fieldset/' + item['fieldset']]['fields']}
            assert fields == {f['name']: (f['bit_offset'], f['bit_size']) for f in own[register]['fields']}, (name, version, register)
            for field in ir['fieldset/' + item['fieldset']]['fields']:
                original = next(f for f in own[register]['fields'] if f['name'] == field['name'])
                current_access = field.get('access', item.get('access', 'ReadWrite'))
                if block in ('DMA', 'DMACHANNEL') and re.fullmatch(r'CSR[1-4]?', register) and field['name'] == 'STATUS':
                    # All five own manuals say STATUS RW; their SVDs say RO.
                    # Preserve the existing manual-qualified metadata, while
                    # binding exactly this discrepancy in all 23 selected views.
                    assert original['access'] == status_discrepancy['own_svd_access']
                    assert current_access == status_discrepancy['current_ir_access']
                    four_tokens(csr, ['10:8 STATUS ' + status_discrepancy['own_manual_access']], (name, register, 'STATUS access'))
                    status_access_views += 1
                else:
                    assert current_access == access_names[original['access'] or register_access], (name, register, field['name'], 'access')
            # PAC address accessors are generated facts, not HAL behavior.
            accessor = re.search(rf'pub const fn {register.lower()}\(self\) -> ([^\n]+)\s*\{{\s*unsafe \{{ crate::common::Reg::from_ptr\(self.ptr.wrapping_add\((0x[0-9a-f]+)usize\)', pac)
            assert accessor and int(accessor[2], 16) == item['byte_offset'], (version, register)
            pac_access = {'read-only': 'R', 'write-only': 'W', 'read-write': 'RW'}[register_access]
            handle = re.search(r'crate::common::Reg<regs::(\w+), crate::common::(\w+)>', accessor[1])
            assert handle and handle[2] == pac_access, (version, register, 'PAC access')
            assert re.search(rf'pub struct {handle[1]}\(pub u32\);', pac), (version, register, '32-bit MMIO')
        if block == 'UART':
            classic_uart_ir[name] = ir
            assert [f['name'] for f in own['ISR']['fields']] == family['own_isr_fields']
            assert set(family['own_isr_fields']) == {'TXE', 'TC', 'RC', 'FE', 'PE', 'MATCH', 'CTS', 'CTSLV', 'TXBUSY'} | (set() if channels == 2 else {'TIMOV', 'BAUD', 'RXBRK'})
            for field in family['own_isr_fields']:
                bit = next(f['bit_offset'] for f in own['ISR']['fields'] if f['name'] == field)
                four_tokens(claim_text['uart_isr'], [f'{bit} {field} RO'], (name, 'full own ISR'))
            assert not {'ORE', 'NE'} & set(family['own_isr_fields'])
            assert family['error_fields'] == {'PE': [4, 1], 'FE': [3, 1]}
            assert 'crate::common::R>' in re.search(r'pub const fn rdr\(self\) -> ([^\n]+)', pac)[1]
        if block in ('UART', 'DMA'):
            commands = load(ROOT / 'cw32-data/register-writes.yaml')['registers'][version]
            assert len(commands) == 1
            command = commands[0]
            no_op = seed if block == 'UART' else 0xffffffff
            assert command['reset_value'] == command['write_noop'] == no_op
            flags = {f['name'] for f in own['ICR']['fields']} - {'RFU'}
            assert set(command['zero_to_clear_fields']) == flags
            assert load(ROOT / f'cw32-data/data/register-writes/{version}.json') == {'schema_version': 1, 'registers': {version: commands}}
            helpers = pac.split('impl regs::Icr {', 1)[1]
            for helper in ('reset_value', 'write_noop'):
                assert re.search(rf'pub const fn {helper}\(\) -> Self\s*\{{\s*Self\({no_op}\)\s*\}}', helpers)
            if block == 'DMA':
                expected_flags = {f'{kind}{i}': (4 * (i - 1) + bit, 1) for i in range(1, channels + 1) for kind, bit in [('TC', 0), ('TE', 1)]}
                assert {f['name']: (f['bit_offset'], f['bit_size']) for f in own['ICR']['fields']} == expected_flags
                icr = claim_text['dma_icr' if name == 'CW32F020' else 'icr']
                four_tokens(icr, ['Reset value: 0xFFFF FFFF', 'W0：清除', 'W1：无功能'], (name, 'DMA ICR'))
                for field, (bit, _) in expected_flags.items():
                    four_tokens(icr, [f'{bit} {field} R1W0'], (name, field))
    sidecar = load(ROOT / f'cw32-data/dma/{name.lower()}.yaml')
    assert sidecar['core_dma_channels'] == expected_channels
    assert len(sidecar['channel_details']) == channels
    for i, channel in enumerate(sidecar['channel_details'], 1):
        assert channel['physical_channel'] == i and channel['name'] == f'DMA_CH{i}'
        assert channel['register_peripheral'] == f'DMACHANNEL{i}'
        assert (channel['interrupt'], channel['interrupt_number']) == vectors[i - 1]
    requests = [r for r in sidecar['requests'] if r['peripheral'] in ('UART1', 'UART2', 'UART3') and r['signal'] in ('RX', 'TX')]
    assert [{k: v for k, v in r.items() if k not in ('dma', 'evidence')} for r in requests] == family['uart_requests']
    assert [(r['peripheral'], r['signal'], r['request']) for r in requests] == [(f'UART{i}', signal, 2 * (i - 1) + offset) for i in range(1, 4) for signal, offset in [('RX', 0), ('TX', 1)]]
    dma_lines = classic_bytes[inputs['dma_header']['path']].decode().splitlines()
    for request in requests:
        assert request['dma'] == 'DMA' and request['channels'] == [c['name'] for c in expected_channels]
        assert request['sdk']['encoding'] == 'shifted_left_2_define'
        assert request['sdk']['raw_value'] == request['request'] << 2
        assert int(request['manual']['binary'], 2) == request['request']
        assert request['manual']['printed_page'] == request['manual']['pdf_page_index']
        four_tokens(manual[request['manual']['pdf_page_index']], [request['manual']['binary'] + '：' + request['manual']['description']], (name, request['sdk']['symbol']))
        line = dma_lines[request['sdk']['line'] - 1]
        match = re.search(rf"#define\s+{request['sdk']['symbol']}\s+\(uint32_t\)\(0x([0-9a-f]+)UL\s*<<\s*2\)", line, re.I)
        assert match and int(match[1], 16) == request['request'], (name, line)
    af = load(ROOT / family['af_path'])
    pinout = load(ROOT / family['pinout_path'])
    assert af['source']['sdk_url'] == inputs['sdk']['official_url']
    assert af['source']['sdk_sha256'] == inputs['sdk']['sha256']
    assert af['source']['header_sha256'] == inputs['gpio_header']['sha256']
    for source in (af['datasheet'], pinout['source']):
        assert source['source_ref'] == refs['datasheet']
        assert source['url'] == inputs['datasheet']['official_url'] and source['sha256'] == inputs['datasheet']['sha256']
assert page_count == 140
assert status_access_views == status_discrepancy['selected_register_views'] == 23

parts = load(ROOT / 'cw32-data/parts.yaml')['parts']
assert {p['feature'] for p in parts if p['family'] in classic_families} == classic_features
route_count = pair_count = 0
for part in classic['exact_parts']:
    feature = part['feature']
    authored = next(p for p in parts if p['feature'] == feature)
    name = authored['family']
    family = classic['families'][name]
    chip = load(ROOT / f'cw32-data/data/chips/{feature.upper()}.json')
    assert chip['line'] == name and chip['name'] == feature.upper()
    memory = [{'address': 0, 'kind': 'flash', 'name': 'FLASH', 'size': 32768 if name == 'CW32F020' else 65536},
              {'address': 0x20000000, 'kind': 'ram', 'name': 'RAM', 'size': 8192}]
    assert chip['memory'] == [part['memory']] == [authored['memory']] == [memory]
    assert authored['package'] == part['package']
    assert len(chip['cores']) == len(chip['packages']) == 1
    core = chip['cores'][0]
    package = chip['packages'][0]
    assert package['package'] == part['package']
    assert core['dma_channels'] == classic_channels[name]
    dma_instances = [p for p in core['peripherals'] if p['name'].startswith('DMACHANNEL')]
    assert len(dma_instances) == len(classic_channels[name])
    for instance in dma_instances:
        number = int(instance['name'][10:])
        assert instance['address'] == 0x40020000 + 32 * number
        assert instance['registers'] == {'block': 'DMACHANNEL', 'kind': 'dmachannel', 'version': 'v1'}
        irq = 'DMACH' + str(number) if name == 'CW32F020' or number in (1, 4) else 'DMACH23'
        assert instance['interrupts'] == [{'interrupt': irq, 'signal': 'GLOBAL'}]
    af = load(ROOT / family['af_path'])
    pinout = load(ROOT / family['pinout_path'])
    own_package = next(p for p in pinout['packages'] if feature.upper() in p['documented_orderable_parts'])
    assert own_package['package'] == part['package']
    assert package['pins'] == own_package['pins']
    assert own_package['flash_bytes'] == memory[0]['size'] and own_package['ram_bytes'] == 8192
    bonded = {signal: p['position'] for p in package['pins'] for signal in p['signals']}
    datasheet = classic_pages[name, 'datasheet']
    product_page = datasheet[min(map(int, family['datasheet_page_utf8_sha256']))]
    product_row = next(line for line in product_page.splitlines() if '外设' in line and name in line)
    products = re.findall(r'CW32[A-Z0-9]+', product_row)
    product = next(p for p in products if feature.upper().startswith(p))
    for label, size in [('FLASH', memory[0]['size'] // 1024), ('SRAM', 8)]:
        row = next(line for line in product_page.splitlines() if label + '（K' in line)
        assert [int(n) for n in re.findall(r'\d+', row)][products.index(product)] == size
    assert set(part['uarts']) == {'UART1', 'UART2', 'UART3'}
    for uart_name, routes in part['uarts'].items():
        uart = next(p for p in core['peripherals'] if p['name'] == uart_name)
        assert uart['registers'] == {'block': 'UART', 'kind': 'uart', 'version': family['uart_register_version']}
        assert uart['address'] == {'UART1': 0x40013800, 'UART2': 0x40004400, 'UART3': 0x40004800}[uart_name]
        assert uart['interrupts'] == [{'interrupt': uart_name, 'signal': 'GLOBAL'}]
        assert {'name': uart_name, 'number': 26 + int(uart_name[-1])} in core['interrupts']
        for signal in ('RX', 'TX'):
            expected_request = {'dma': 'DMA', 'request': 2 * (int(uart_name[-1]) - 1) + (signal == 'TX'), 'signal': signal}
            assert [r for r in uart['dma_channels'] if r['signal'] == signal] == [expected_request]
            current = [p for p in uart['pins'] if p['signal'] == signal and p['pin'] not in classic['excluded_safe_pins']]
            projected = sorted(f"{p['pin']}/AF{p['af']}@{bonded[p['pin']]}" for p in current)
            assert routes[signal] == projected and len(set(projected)) == len(projected), (feature, uart_name, signal)
            authored_routes = [p for p in af['routes'] if p['peripheral'] == uart_name and p['signal'] == signal
                               and p['pin'] in bonded and p['pin'] not in classic['excluded_safe_pins']]
            assert projected == sorted(f"{p['pin']}/AF{p['af']}@{bonded[p['pin']]}" for p in authored_routes)
            route_count += len(projected)
            gpio_lines = classic_bytes[classic['sources'][family['sources']['gpio_header']]['path']].decode().splitlines()
            for route in authored_routes:
                pin = route['pin']; af_number = route['af']
                assert 1 <= af_number <= 7
                pin_rows = [row for row in pinout['table_rows'] if pin in row['signals']]
                assert len(pin_rows) == 1
                row = pin_rows[0]
                page = routes['bonded_pad_source_pages'][pin]
                assert page == row['pdf_page_index'] and str(page) in family['datasheet_page_utf8_sha256']
                assert row['positions'][own_package['table_column']] == bonded[pin]
                source_pin = pin[:2] + pin[2:].zfill(2)
                pad_row = next(line for line in datasheet[page].splitlines() if re.search(rf'\b{source_pin}\b', line))
                pads = pad_row.split(source_pin, 1)[0].split()
                assert pads[pinout['table_columns'].index(own_package['table_column'])] == bonded[pin], (feature, pin)
                if route['source_macro'] is not None:
                    line = gpio_lines[route['source_line'] - 1]
                    assert re.search(r'#define\s+' + re.escape(route['source_macro']) + r'\b', line)
                    four_tokens(line, [f"CW_GPIO{pin[1]}->{route['gpio_register']}_f.AFR{int(pin[2:])} = {af_number}"], (name, pin, signal))
                else:
                    assert (name, uart_name, signal, pin, af_number) == ('CW32W031', 'UART3', 'RX', 'PA2', 5)
                    assert route['source_kind'] == 'datasheet' and route['source_line'] is None
                    cell = route['datasheet_cell']
                    assert cell == {'pin': 'PA2', 'af': 5, 'function': 'UART3_RXD', 'table': '5-3', 'pdf_page': 31, 'printed_page': 30}
                    page_text = datasheet[30]
                    header = next(line for line in page_text.splitlines() if '功能 5' in line)
                    af_row = next(line for line in page_text.splitlines() if re.match(r'\s*PA02\b', line))
                    columns = [(int(m[1]), m.start() + len(m[0]) / 2) for m in re.finditer(r'功能\s+(\d)', header)]
                    match = re.search(r'UART3_RXD', af_row)
                    assert min(columns, key=lambda col: abs(col[1] - (match.start() + match.end()) / 2))[0] == 5
        assert set(routes['bonded_pad_source_pages']) == {r.split('/')[0] for signal in ('TX', 'RX') for r in routes[signal]}
        pairs = {(tx, rx) for tx in routes['TX'] for rx in routes['RX'] if tx.split('/')[0] != rx.split('/')[0]}
        assert len(pairs) == routes['distinct_pin_pair_count']
        pair_count += len(pairs)
assert (route_count, pair_count) == (459, 1449)
edge = next(p for p in classic['exact_parts'] if p['feature'] == 'cw32l031f8u6')['uarts']['UART1']
assert edge['TX'] == [] and [r.split('@')[0] for r in edge['RX']] == ['PA9/AF2', 'PC15/AF6']
for feature in ('cw32f020f6u7', 'cw32l031f8p6'):
    uart = next(p for p in classic['exact_parts'] if p['feature'] == feature)['uarts']['UART1']
    assert uart['TX'] == uart['RX'] == [] and uart['distinct_pin_pair_count'] == 0
for name in classic_families:
    generic = load(ROOT / f'cw32-data/data/chips/{name}.json')
    assert generic['memory'] == generic['packages'] == []
    assert generic['cores'][0]['dma_channels'] == classic_channels[name]

# Manifest/capability declarations describe the advertised surface only. Actual
# ARM links and implementation ownership review remain separate acceptance work.
manifest = tomllib.loads((ROOT / 'examples/shared/uart-dma/Cargo.toml').read_text())
assert manifest['package']['name'] == 'cw32-uart-dma-example'
old_features = {'cw32f030c8t7', 'cw32a030c8t7', 'cw32l083rbt6', 'cw32l083rcs6', 'cw32l083rct6', 'cw32l083mct6', 'cw32l083vct6'}
assert {f for f in manifest['features'] if f.startswith('cw32')} == (classic_features - {'cw32w031r8u6'}) | old_features
for feature in (classic_features - {'cw32w031r8u6'}) | old_features:
    assert {f'embassy-cw32/{feature}', f'cw32-metapac/{feature}'} <= set(manifest['features'][feature])
assert (ROOT / 'examples/shared/uart-dma/src/main.rs').is_file()
assert (ROOT / 'examples/shared/uart-dma/src/bin/receive_only.rs').is_file()
assert manifest['package'].get('autobins', True)
assert manifest['bin'] == [{'name': 'shared_vector', 'path': 'src/bin/shared_vector.rs', 'required-features': ['l083']}]
for dependency in ('embassy-executor', 'embassy-futures'):
    assert manifest['dependencies'][dependency]['rev'] == 'f16efeffe37581092ec184718e6fdb1620393214'
capabilities = load(ROOT / 'tools/hal-capabilities.yaml')
for kind in ('uart', 'dma'):
    mode = next(m for m in capabilities['functions'][kind]['modes'] if m['id'] == 'safe_peripheral_dma')
    assert mode['status'] == 'experimental_partial'
    assert set(mode['implemented_families']) == (classic_families - {'CW32W031'}) | {'CW32F030', 'CW32A030', 'CW32L083', 'CW32L012'}
    for name in classic_families - {'CW32W031'}:
        override = next(o for o in mode['overrides'] if name in o['families'])
        assert 'RX' in override['scope'] and 'combined' in override['scope']
        assert 'MissingMemoryMetadata' in ' '.join(override['limitations'])
    l012_override = next(o for o in mode['overrides'] if o['families'] == ['CW32L012'])
    assert 'TX only' in l012_override['scope'] and 'No L012 DMA RX/combined UART' in ' '.join(l012_override['limitations'])
    if kind == 'dma':
        four_tokens(l012_override['scope'], ['SPI1', 'SPI2', 'SPI3', 'C8T6', 'C8U6'], 'native DMA aggregate scope')
print('PASS: classic UART RX: 55 exact own-source nodes/member chains, 140 pages, full ISR/typed ICR PAC and native two/four-channel requests, fourteen exact SRAM parts, 459 safe routes/1449 distinct-pin pairs and empty generics; source/declaration facts only')

# Finite classic SPI endpoint qualification. The own SPI chapters and paired
# examples supply the endpoint authority; the preceding native DMA checks retain
# their exact 23 manual-qualified STATUS access views. No HAL text is consumed.
spi_path = 'docs/classic-spi-dma-evidence.json'
spi = load(ROOT / spi_path)
assert spi['schema_version'] == 1
assert set(spi['families']) == set(spi['scope']['families']) == classic_families
assert spi['scope']['exact_part_count'] == len(spi['exact_parts']) == 14
assert {p['feature'] for p in spi['exact_parts']} == classic_features
assert spi['scope']['family_instance_count'] == 7
assert spi['scope']['selected_part_instance_route_count'] == 20
assert spi['excluded_safe_pins'] == ['PA13', 'PA14', 'PF3']
assert spi['text_extractor'] == lock['text_extractor']
assert spi['text_extractor']['program'] == 'pdftotext'
assert spi['text_extractor']['arguments'] == ['-layout', '-enc', 'UTF-8']
spi_pointers = set()
for sdk, manual, datasheet, members in [
    (2, 18, 40, [0, 7]), (7, 27, 26, [0, 7, 34]),
    (8, 29, 28, [0, 7, 38]), (10, 33, 32, [0, 7, 31]),
    (11, 35, 34, [0, 7, 31]),
]:
    spi_pointers.add(f'/artifacts/{sdk}')
    spi_pointers.update(f'/artifacts/{sdk}/members/{i}' for i in members)
    spi_pointers.update(f'/artifacts/{i}{suffix}' for i in (manual, datasheet) for suffix in ('', '/text'))
assert len(spi['sources']) == len(spi_pointers) == 39
assert {s['index_pointer'] for s in spi['sources'].values()} == spi_pointers
contract = spi['source_index_append_plan']
assert contract['new_consumer'] == spi_path and contract['checker_extension'] == classic_checker
assert len(contract['bound_source_refs']) == 39 and set(contract['bound_source_refs']) == set(spi['sources'])
assert {ref for family in spi['families'].values() for ref in family['sources'].values()} == set(spi['sources'])
spi_bytes = {}
for ref, source in spi['sources'].items():
    pointer = source['index_pointer']
    pin = four_index(pointer)
    artifact = lock['artifacts'][int(pointer.split('/')[2])]
    observed = {k: v for k, v in pin.items() if k != 'evidence'
                and not (k == 'members' and ref.startswith('vendor:'))}
    if 'text' in observed:
        observed['text'] = {k: v for k, v in observed['text'].items() if k != 'evidence'}
    assert observed == source['identity'], ref
    assert {k: source[k] for k in ('path', 'sha256', 'bytes')} == {
        k: pin[k] for k in ('path', 'sha256', 'bytes')}
    assert source['official_url'] == artifact['url']
    assert pin['evidence'].count(spi_path) == pin['evidence'].count(classic_checker) == 1
    if '/members/' in pointer:
        assert ref == 'member:' + pin['path'] and 'id' not in pin
        assert source['archive_member_chain'] == pin['members']
    elif pointer.endswith('/text'):
        assert ref == 'text:' + pin['path'] and 'id' not in pin
    else:
        assert ref == artifact['id']
    raw = (SOURCES / pin['path']).read_bytes()
    assert len(raw) == source['bytes'] and digest(raw) == source['sha256'], ref
    assert pin['path'] not in spi_bytes
    spi_bytes[pin['path']] = raw
    if '/members/' in pointer:
        payload = (SOURCES / artifact['path']).read_bytes()
        for member in source['archive_member_chain']:
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                payload = archive.read(member)
        assert payload == raw, ref
spi_reverse = set()
checker_reverse = set()
for index, artifact in enumerate(lock['artifacts']):
    nodes = [(f'/artifacts/{index}', artifact)]
    nodes += [(f'/artifacts/{index}/members/{i}', member) for i, member in enumerate(artifact.get('members', []))]
    if 'text' in artifact:
        nodes.append((f'/artifacts/{index}/text', artifact['text']))
    for pointer, node in nodes:
        for consumer, reverse in ((spi_path, spi_reverse), (classic_checker, checker_reverse)):
            assert node.get('evidence', []).count(consumer) <= 1
            if consumer in node.get('evidence', []):
                reverse.add(pointer)
assert spi_reverse == spi_pointers
# Existing index consumers plus this finite extension, derived from current
# ledgers rather than from an immutable historical source-index digest.
native_spi_path = 'docs/l012-spi-dma-evidence.json'
native_spi = load(ROOT / native_spi_path)
native_spi_pointers = {'/artifacts/6'} | {
    f'/artifacts/6/members/{i}' for i in (17, 19, 25, 28, 37, 47, 71)
} | {f'/artifacts/{i}{suffix}' for i in (24, 25, 42) for suffix in ('', '/text')}
assert len(native_spi['sources']) == len(native_spi_pointers) == 14
assert {s['source_index_pointer'] for s in native_spi['sources']} == native_spi_pointers
assert checker_reverse == (classic_pointers | l012_pointers | spi_pointers | native_spi_pointers
                           | {s['source_index_pointer'] for s in four['sources']})

facts = spi['shared_facts']
spi_offsets = {'CR1': 0, 'IER': 4, 'CR2': 8, 'SSI': 12, 'ISR': 16, 'ICR': 20, 'DR': 24}
spi_flags = {name: [bit, 1] for bit, name in enumerate(
    ['TXE', 'RXNE', 'SSF', 'SSR', 'UD', 'OV', 'SSERR', 'MODF', 'BUSY', 'SSLVL'])}
assert facts['spi_register_offsets'] == spi_offsets
assert facts['isr_fields'] == spi_flags
assert facts['request_gates'] == {'DMARX': [16, 1], 'DMATX': [17, 1]}
assert facts['error_fields'] == ['UD', 'OV', 'SSERR', 'MODF']
assert facts['spi_icr_noop'] == 255 and facts['destructive_icr_field'] == 'FLUSH'
assert facts['dma_icr_noop'] == 0xffffffff
assert facts['dma_fields'] == {'STATUS': [8, 3], 'REPEAT': [16, 4], 'CNT': [0, 16], 'HARDSRC': [2, 6]}
assert facts['complete_status'] == 5 and facts['error_statuses'] == [1, 2, 3, 4]
assert (facts['example_width'], facts['example_dma_size'], facts['example_count'],
        facts['example_repeat'], facts['cpu_mmio_bits']) == (7, 0, 100, 1, 32)
assert len(spi['source_discrepancies']) == 2
spi_status = spi['source_discrepancies'][0]
for key in ('id', 'families', 'own_svd_access', 'own_manual_access', 'current_ir_access', 'field', 'selected_register_views'):
    assert spi_status[key] == status_discrepancy[key]
assert status_access_views == spi_status['selected_register_views'] == 23
typo = spi['source_discrepancies'][1]
assert typo['id'] == 'paired-example-dstinc-typo' and set(typo['families']) == classic_families
assert (typo['literal'], typo['register']) == ('DMA_CS2.DSTINC', 'DMA_CSR2.DSTINC')
spi_pages = {}
spi_instance_count = spi_page_count = 0
for name, family in spi['families'].items():
    refs = family['sources']
    assert set(refs) == {'sdk', 'svd', 'dma_header', 'manual', 'manual_text', 'datasheet', 'datasheet_text'} | (
        set() if name == 'CW32F020' else {'spi_driver'})
    inputs = {kind: spi['sources'][ref] for kind, ref in refs.items()}
    assert inputs['sdk']['identity']['family'] == name
    assert all(name.lower() in source['path'].lower() for source in inputs.values())
    assert family['page_index_rule'] == 'printed page = zero-based PDF index; viewer page = index+1'
    for kind in ('manual', 'datasheet'):
        assert inputs[kind]['identity']['text']['path'] == inputs[kind + '_text']['path']
        pages = spi_bytes[inputs[kind + '_text']['path']].decode().split('\f')
        spi_pages[name, kind] = pages
        assert {int(n) for n in family[kind + '_page_utf8_sha256']} == {
            n for numbers in family[kind + '_claim_pages'].values() for n in numbers}
        for number, sha in family[kind + '_page_utf8_sha256'].items():
            assert digest(pages[int(number)].encode()) == sha, (name, kind, number)
            spi_page_count += 1
    manual = spi_pages[name, 'manual']
    claims = family['manual_claim_pages']
    text = {claim: '\n'.join(manual[n] for n in numbers) for claim, numbers in claims.items()}
    count = 2 if name in {'CW32F020', 'CW32L052'} else 1
    channels = 2 if name == 'CW32F020' else 4
    assert family['physical_channels'] == list(range(1, channels + 1))
    assert len(family['instances']) == count
    expected_names = [f'SPI{i}' for i in range(1, count + 1)]
    assert [i['name'] for i in family['instances']] == expected_names
    example_name = 'SPI1' if count == 2 else 'SPI'
    chapter_name = 'SPIx' if count == 2 else 'SPI'
    example = four_compact(text['example'])
    four_tokens(example, ['工作于主机模式', '全双工', 'SRAM', 'uint8_t SendBuff[100]', 'uint8_t RecvBuff[100]',
        f'{example_name}_CR1.MODE 为 0x0', f'{example_name}_CR1.MSTR 为 1', f'{example_name}_CR1.WIDTH 为 0x7',
        'DMA_SRCADDR1=(uint32_t)SendBuff', f'DMA_DSTADDR1=(uint32_t)&{example_name}_DR',
        f'DMA_SRCADDR2=(uint32_t)&{example_name}_DR', 'DMA_DSTADDR2=(uint32_t)RecvBuff',
        'DMA_CSR1.SRCINC 为 1', 'DMA_CSR1.DSTINC 为 0', 'DMA_CSR2.SRCINC 为 0',
        'DMA_CS2.DSTINC 为 1', 'DMA_TRIG1.HardSrc 为 0x7', 'DMA_TRIG2.HardSrc 为 0x6'], (name, 'paired byte example'))
    for number in (1, 2):
        four_tokens(example, [f'DMA_CNT{number} 写入 0x10064', f'DMA_CSR{number}.SIZE 为 0',
            f'DMA_CSR{number}.TRANS 为 1', f'DMA_TRIG{number}.Type 为 1'], (name, number, 'byte BLOCK/REPEAT1'))
    sequence = [f'{example_name}_SSI.SSI 为 0', 'DMA_CSR1.EN 为 1', 'DMA_CSR2.EN 为 1',
        f'{example_name}_CR1.DMATX 为 1', f'{example_name}_CR1.DMARX 为 1',
        'DMA_ISR.TC1 变为 1', 'DMA_ISR.TC2 变为 1',
        f'{example_name}_CR1.DMATX 为 0', f'{example_name}_CR1.DMARX 为 0',
        f'{example_name}_ISR.BUSY 变为 0', f'{example_name}_SSI.SSI 为 1']
    positions = [example.index(four_compact(token)) for token in sequence]
    assert positions == sorted(positions), (name, 'both terminals/gates/wire idle/SSI order')
    four_tokens(text['dma'], [f'{chapter_name}_CR1.DMATX 为 1', f'{chapter_name}_CR1.DMARX 为 1',
        '当发送缓冲区为空时，产生 DMA 请求', '当接收缓冲区非空时，产生 DMA 请求',
        f'传输到 {chapter_name}_DR 寄存器', f'从 {chapter_name}_DR 寄存器传输到指定的存储区',
        'DMA_ISR.TCy', f'{chapter_name}_ISR.BUSY'], (name, 'request handshake'))
    four_tokens(text['ready'], [f'读 {chapter_name}_DR 寄存器的同时将清除 {chapter_name}_ISR.RXNE 标志位',
        '该标志为 0 时，才可拉高 CS 信号'], (name, 'RXNE read clear and wire idle'))
    four_tokens(text['error'], [f'{chapter_name}_CR1.EN 被清零', 'SPI 被强制关闭',
        '新接收到的数据将覆盖缓冲器中原有的数据', '从机模式下的从机选择错误', '从机模式下的发送缓冲下溢'], (name, 'errors'))
    four_tokens(text['isr'], [f'{bit} {field} RO' for field, (bit, _) in spi_flags.items()] +
        ['对 DR 寄存器的读操作或对 ICR.RXNE 写 0 均可以清除'], (name, 'complete ISR'))
    four_tokens(text['irq'], ['IER.' + field for field in list(spi_flags)[:8]], (name, 'eight interrupt sources'))
    four_tokens(text['gate'], ['16 DMARX RW', '17 DMATX RW', '13:10 WIDTH RW'], (name, 'CR1 gates/width'))
    four_tokens(text['icr_dr'], ['Address offset: 0x14', 'Reset value: 0x0000 00FF', 'W1：无功能',
        '0 FLUSH R1W0', '清空发送缓冲区和移位寄存器', 'Address offset: 0x18', '15:0 DR RW'] +
        [f'{bit} {field} R1W0' for field, (bit, _) in spi_flags.items() if 1 <= bit <= 7], (name, 'ICR/DR'))
    four_tokens(text['terminal'], ['数据传输全部正确完成', 'TEIE', 'TCIE'], (name, 'DMA terminal'))
    four_tokens(text['status'], ['10:8 STATUS RW', '000：初始状态', '001：传输错误，传输地址超出寻址范围',
        '010：传输错误，传输停止请求引起中止', '011：传输错误，访问传输来源地址出错',
        '100：传输错误，访问传输目的地址出错', '101：传输完成', '00：8 比特', 'DSTINC'], (name, 'DMA STATUS and byte width'))
    four_tokens(text['dma_count'], ['19:16 REPEAT RW', '15:0 CNT RW'], (name, 'DMA finite count/repeat'))
    four_tokens(text['dma_icr'], ['Reset value: 0xFFFF FFFF', 'W0：清除', 'W1：无功能'], (name, 'DMA ICR'))
    four_tokens(text['sram'], ['8KB', 'SRAM', '0x2000 0000'], (name, 'own SRAM'))
    four_tokens(text['cpu_mmio_width'], ['32bit'], (name, 'CPU MMIO width'))
    if 'spi_driver' in refs:
        lines = spi_bytes[inputs['spi_driver']['path']].decode().splitlines()
        assert len(family['sdk_rxne_read_clear']) == 1
        observed = family['sdk_rxne_read_clear'][0]
        assert lines[observed['line'] - 1].strip() == observed['text']
        four_tokens(observed['text'], ['RXNE标志位也可以通过读SPIx_DR寄存器来清除'], (name, 'SDK RXNE'))
    svd = ET.fromstring(spi_bytes[inputs['svd']['path']])
    peripherals = {p.findtext('name'): p for p in svd.findall('peripherals/peripheral')}
    assert {p for p in peripherals if p.startswith('SPI')} == set(expected_names)
    for instance in family['instances']:
        number = int(instance['name'][-1])
        base = 0x40013000 if number == 1 else 0x40003800
        assert instance == {'name': f'SPI{number}', 'manual_name': f'SPI{number}' if count == 2 else 'SPI',
            'base': f'0x{base:08x}', 'dr': f'0x{base + 24:08x}', 'rx_request': 4 + 2 * number,
            'tx_request': 5 + 2 * number, 'irq': 24 + number}
        original = peripherals[instance['name']]
        assert int(original.findtext('baseAddress'), 0) == base
        assert [(i.findtext('name'), int(i.findtext('value'), 0)) for i in original.findall('interrupt')] == [(instance['name'], instance['irq'])]
        assert original.get('derivedFrom') == ('SPI1' if number == 2 else None)
        if number == 2:
            assert original.find('registers') is None
        four_tokens(text['base'], [f'{instance["manual_name"]}_BASE = 0x{base:08x}'], (name, 'own base'))
        spi_instance_count += 1
    version = 'spi_' + family['spi_register_version']
    assert version == ('spi_v1' if name == 'CW32F020' else 'spi_cw32l031_v1')
    ir = load(ROOT / f'cw32-data/data/registers/{version}.json')
    assert ir == load(ROOT / f'cw32-data/registers/{version}.yaml')
    own_registers = {r.findtext('name'): r for r in peripherals['SPI1'].findall('registers/register')}
    own = {register: four_svd_register(r) for register, r in own_registers.items()}
    items = {r['name']: r for r in ir['block/SPI']['items']}
    assert set(items) == set(own) == set(spi_offsets)
    pac = (ROOT / f'cw32-metapac/src/peripherals/{version}.rs').read_text()
    for register, item in items.items():
        assert item['byte_offset'] == own[register]['offset'] == spi_offsets[register]
        access = own_registers[register].findtext('access')
        assert item.get('access', 'ReadWrite') == {'read-only': 'Read', 'read-write': 'ReadWrite'}[access]
        fields = {f['name']: [f['bit_offset'], f['bit_size']] for f in ir['fieldset/' + item['fieldset']]['fields']}
        assert fields == {f['name']: [f['bit_offset'], f['bit_size']] for f in own[register]['fields']}
        if register == 'ISR':
            assert fields == spi_flags
        if register == 'IER':
            assert fields == {f: v for f, v in spi_flags.items() if v[0] <= 7}
        if register == 'CR1':
            assert {f: fields[f] for f in ('DMARX', 'DMATX')} == facts['request_gates']
        accessor = re.search(rf'pub const fn {register.lower()}\(self\) -> ([^\n]+)\s*\{{\s*unsafe \{{ crate::common::Reg::from_ptr\(self.ptr.wrapping_add\((0x[0-9a-f]+)usize\)', pac)
        assert accessor and int(accessor[2], 16) == item['byte_offset']
        handle = re.search(r'crate::common::Reg<regs::(\w+), crate::common::(\w+)>', accessor[1])
        assert handle and handle[2] == ('R' if access == 'read-only' else 'RW')
        assert re.search(rf'pub struct {handle[1]}\(pub u32\);', pac)
        field_code = pac.split('impl ' + handle[1] + ' {', 1)[1].split('\n    impl ', 1)[0]
        for field in ir['fieldset/' + item['fieldset']]['fields']:
            own_field = next(f for f in own[register]['fields'] if f['name'] == field['name'])
            assert field.get('access', item.get('access', 'ReadWrite')) == {
                'read-only': 'Read', 'read-write': 'ReadWrite'}[own_field['access'] or access]
            getter = re.search(rf'pub const fn {field["name"].lower()}\(&self\) -> \w+\s*\{{\s*let val = \(self.0 >> (\d+)usize\) & (0x[0-9a-f]+);', field_code)
            assert getter and (int(getter[1]), int(getter[2], 16)) == (field['bit_offset'], (1 << field['bit_size']) - 1)
    commands = load(ROOT / 'cw32-data/register-writes.yaml')['registers'][version]
    assert len(commands) == 1
    assert commands[0]['reset_value'] == commands[0]['write_noop'] == 255
    assert commands[0]['zero_to_clear_fields'] == ['FLUSH', 'RXNE', 'SSF', 'SSR', 'UD', 'OV', 'SSERR', 'MODF']
    assert load(ROOT / f'cw32-data/data/register-writes/{version}.json') == {'schema_version': 1, 'registers': {version: commands}}
    helpers = pac.split('impl regs::Icr {', 1)[1]
    for helper in ('reset_value', 'write_noop'):
        assert re.search(rf'pub const fn {helper}\(\) -> Self\s*\{{\s*Self\(255\)\s*\}}', helpers)
    # The same selected own SVD was fully compared to native DMA IR/PAC above;
    # rebind its physical topology and SPI request records to this endpoint proof.
    assert inputs['svd']['identity'] == classic['sources'][classic['families'][name]['sources']['svd']]['identity']
    sidecar = load(ROOT / f'cw32-data/dma/{name.lower()}.yaml')
    assert sidecar['core_dma_channels'] == classic_channels[name]
    assert {p for p in peripherals if p.startswith('DMACHANNEL')} == {f'DMACHANNEL{i}' for i in range(1, channels + 1)}
    vectors = [('DMACH1', 9), ('DMACH2', 10)] if channels == 2 else [('DMACH1', 9), ('DMACH23', 10), ('DMACH23', 10), ('DMACH4', 11)]
    for number, channel in enumerate(sidecar['channel_details'], 1):
        assert channel['physical_channel'] == number and channel['name'] == f'DMA_CH{number}'
        assert (channel['interrupt'], channel['interrupt_number']) == vectors[number - 1]
        original = peripherals[f'DMACHANNEL{number}']
        assert int(original.findtext('baseAddress'), 0) == 0x40020000 + 32 * number
        assert [(i.findtext('name'), int(i.findtext('value'), 0)) for i in original.findall('interrupt')] == [vectors[number - 1]]
    requests = [r for r in sidecar['requests'] if r['peripheral'].startswith('SPI')]
    assert requests == family['spi_requests']
    assert [(r['peripheral'], r['signal'], r['request']) for r in requests] == [
        (f'SPI{i}', signal, 4 + 2 * i + offset) for i in range(1, count + 1) for signal, offset in [('RX', 0), ('TX', 1)]]
    header = spi_bytes[inputs['dma_header']['path']].decode().splitlines()
    for request in requests:
        assert set(request) == {'peripheral', 'signal', 'dma', 'request', 'channels', 'sdk', 'manual', 'evidence'}
        assert request['dma'] == 'DMA' and request['channels'] == [f'DMA_CH{i}' for i in range(1, channels + 1)]
        assert request['sdk']['encoding'] == 'shifted_left_2_define'
        assert request['sdk']['raw_value'] == request['request'] << 2
        assert int(request['manual']['binary'], 2) == request['request']
        assert request['manual']['pdf_page_index'] == request['manual']['printed_page'] == claims['selectors'][0]
        four_tokens(text['selectors'], [request['manual']['binary'] + '：' + request['manual']['description']], (name, 'own request selector'))
        line = header[request['sdk']['line'] - 1]
        match = re.search(rf"#define\s+{request['sdk']['symbol']}\s+\(uint32_t\)\(0x([0-9a-f]+)UL\s*<<\s*2\)", line, re.I)
        assert match and int(match[1], 16) == request['request'], (name, line)
    assert family['maximum_sck_hz'] == 12000000
    assert family['minimum_divisor'] == (2 if name == 'CW32F020' else 4)
assert spi_instance_count == 7 and spi_page_count == 105

spi_route_count = 0
clock_profiles = load(ROOT / 'cw32-data/spi.yaml')['profiles']
clock_policy = load(ROOT / 'docs/spi-clock-source-policy.json')['families']
for part in spi['exact_parts']:
    feature, name = part['feature'], part['family']
    family = spi['families'][name]
    authored = next(p for p in parts if p['feature'] == feature)
    assert authored['family'] == name and authored['package'] == part['package']
    chip = load(ROOT / f'cw32-data/data/chips/{feature.upper()}.json')
    memory = [{'address': 0, 'kind': 'flash', 'name': 'FLASH', 'size': 32768 if name == 'CW32F020' else 65536},
              {'address': 0x20000000, 'kind': 'ram', 'name': 'RAM', 'size': 8192}]
    assert chip['memory'] == [part['memory']] == [authored['memory']] == [memory]
    assert chip['line'] == name and chip['name'] == feature.upper()
    assert len(chip['cores']) == len(chip['packages']) == 1
    core, package = chip['cores'][0], chip['packages'][0]
    assert core['dma_channels'] == classic_channels[name]
    af, pinout = load(ROOT / family['af_path']), load(ROOT / family['pinout_path'])
    source = spi['sources'][family['sources']['datasheet']]
    for declaration in (af['datasheet'], pinout['source']):
        assert declaration['source_ref'] == family['sources']['datasheet']
        assert declaration['url'] == source['official_url'] and declaration['sha256'] == source['sha256']
    own_package = next(p for p in pinout['packages'] if feature.upper() in p['documented_orderable_parts'])
    assert package['package'] == own_package['package'] == part['package']
    assert package['pins'] == own_package['pins']
    assert own_package['flash_bytes'] == memory[0]['size'] and own_package['ram_bytes'] == 8192
    bonded = {signal: p['position'] for p in package['pins'] for signal in p['signals']}
    datasheet = spi_pages[name, 'datasheet']
    product_page = datasheet[family['datasheet_claim_pages']['memory'][0]]
    product_row = next(line for line in product_page.splitlines() if '外设' in line and name in line)
    products = re.findall(r'CW32[A-Z0-9]+', product_row)
    product = next(p for p in products if feature.upper().startswith(p))
    for label, size in [('FLASH', memory[0]['size'] // 1024), ('SRAM', 8)]:
        row = next(line for line in product_page.splitlines() if label + '（K' in line)
        assert [int(n) for n in re.findall(r'\d+', row)][products.index(product)] == size
    expected_instances = {i['name'] for i in family['instances']}
    assert len(part['selected_routes']) == len(expected_instances)
    assert {r['instance'] for r in part['selected_routes']} == expected_instances
    assert {p['name'] for p in core['peripherals'] if p.get('registers', {}).get('kind') == 'spi'} == expected_instances
    for route in part['selected_routes']:
        spi_name = route['instance']
        number = int(spi_name[-1])
        peripheral = next(p for p in core['peripherals'] if p['name'] == spi_name)
        assert peripheral['registers'] == {'block': 'SPI', 'kind': 'spi', 'version': family['spi_register_version']}
        assert peripheral['address'] == (0x40013000 if number == 1 else 0x40003800)
        assert peripheral['interrupts'] == [{'interrupt': spi_name, 'signal': 'GLOBAL'}]
        assert {'name': spi_name, 'number': 24 + number} in core['interrupts']
        assert peripheral['dma_channels'] == [{'dma': 'DMA', 'request': 4 + 2 * number + offset, 'signal': signal}
                                               for signal, offset in [('RX', 0), ('TX', 1)]]
        limits = {'maximum_frequency': 12000000, 'minimum_divisor': family['minimum_divisor']}
        assert peripheral['spi'] == clock_profiles[name]['peripherals'][spi_name] == limits
        assert clock_policy[name]['maximum_sck_hz'] == 12000000 and clock_policy[name]['minimum_divisor'] == family['minimum_divisor']
        assert not {p['pin'] for p in peripheral['pins']} & set(spi['excluded_safe_pins'])
        expected_pins = {'sck': 'PA5', 'mosi': 'PA7', 'miso': 'PA6'} if number == 1 else {'sck': 'PA2', 'mosi': 'PA1', 'miso': 'PA0'}
        assert set(route['bonded_pad_source_pages']) == set(expected_pins.values())
        for signal, pin in expected_pins.items():
            assert route[signal] == f'{pin}/AF5@{bonded[pin]}'
            assert {'pin': pin, 'signal': signal.upper(), 'af': 5} in peripheral['pins']
            authored_routes = [r for r in af['routes'] if (r['peripheral'], r['signal'], r['pin'], r['af']) == (spi_name, signal.upper(), pin, 5)]
            assert len(authored_routes) == 1
            row = next(row for row in pinout['table_rows'] if pin in row['signals'])
            page = route['bonded_pad_source_pages'][pin]
            assert row['pdf_page_index'] == page and page in family['datasheet_claim_pages']['bonding']
            assert row['positions'][own_package['table_column']] == bonded[pin]
            source_pin = pin[:2] + pin[2:].zfill(2)
            pad_row = next(line for line in datasheet[page].splitlines() if re.search(rf'\b{source_pin}\b', line))
            pads = pad_row.split(source_pin, 1)[0].split()
            assert pads[pinout['table_columns'].index(own_package['table_column'])] == bonded[pin]
            af_page = datasheet[family['datasheet_claim_pages']['af'][0]]
            header = next(line for line in af_page.splitlines() if '功能 5' in line)
            af_row = next(line for line in af_page.splitlines() if re.match(rf'\s*{source_pin}\b', line))
            columns = [(int(m[1]), m.start() + len(m[0]) / 2) for m in re.finditer(r'功能\s+(\d)', header)]
            source_name = spi_name if len(expected_instances) == 2 else 'SPI'
            match = re.search(source_name + '_' + signal.upper(), af_row)
            assert match and min(columns, key=lambda col: abs(col[1] - (match.start() + match.end()) / 2))[0] == 5
        spi_route_count += 1
assert spi_route_count == 20
for name in classic_families:
    generic = load(ROOT / f'cw32-data/data/chips/{name}.json')
    assert generic['memory'] == generic['packages'] == []
    assert generic['cores'][0]['dma_channels'] == classic_channels[name]
four_tokens(spi_pages['CW32W031', 'datasheet'][13], ['4.4.4', 'RF',
    '当使用射频功能时，芯片外部引脚的 SPI 功能不可用'], ('CW32W031', 'RF exclusion'))

# These are declarations only; links, ownership, cancellation, wakeup and silicon
# behavior require independent acceptance and are never inferred from HAL text.
manifest = tomllib.loads((ROOT / 'examples/shared/spi-dma/Cargo.toml').read_text())
assert manifest['package']['name'] == 'cw32-spi-dma-example' and manifest['package'].get('autobins', True)
representatives = {'cw32f020c6u7', 'cw32l031c8t6', 'cw32l052c8t6', 'cw32r031c8u6'}
assert {f for f in manifest['features'] if f.startswith('cw32')} == old_features | representatives
for feature in old_features | representatives:
    assert {f'embassy-cw32/{feature}', f'cw32-metapac/{feature}'} <= set(manifest['features'][feature])
    assert ('l083' in manifest['features'][feature]) == feature.startswith('cw32l083')
    assert ('spi2-example' in manifest['features'][feature]) == (feature.startswith('cw32l083') or feature in {'cw32f020c6u7', 'cw32l052c8t6'})
assert manifest['features']['l083'] == manifest['features']['spi2-example'] == []
assert manifest['bin'] == [{'name': 'spi2_flash_id', 'path': 'src/bin/spi2_flash_id.rs', 'required-features': ['spi2-example']}]
assert (ROOT / 'examples/shared/spi-dma/src/main.rs').is_file()
assert (ROOT / 'examples/shared/spi-dma/src/bin/spi2_flash_id.rs').is_file()
assert manifest['dependencies']['embassy-executor']['rev'] == 'f16efeffe37581092ec184718e6fdb1620393214'
mode = next(m for m in capabilities['functions']['spi']['modes'] if m['id'] == 'safe_peripheral_dma')
assert mode['status'] == 'experimental_partial'
assert set(mode['implemented_families']) == (classic_families - {'CW32W031'}) | {'CW32F030', 'CW32A030', 'CW32L083', 'CW32L012'}
assert mode['pins_all'] == ['SCK', 'MOSI', 'MISO']
assert 'dma_channels' in mode['facts']
pio_mode = next(m for m in capabilities['functions']['spi']['modes'] if m['id'] == 'async_pio')
assert (classic_families - {'CW32W031'}) | {'CW32L012'} <= set(pio_mode['implemented_families'])
assert 'CW32W031' not in pio_mode['implemented_families']
print('PASS: classic SPI: 39 own-source nodes and exact backlinks, 105 semantic pages, seven own instances/byte endpoints, native DMA topology and preserved 23 STATUS access views, fourteen exact SRAM parts/twenty selected AF5 triples, empty generics and example/capability declarations; no HAL execution')

# Finite native L012 SPI endpoint qualification. Bind its own facts separately
# from classic SPI and the retained native DMA proof. Runtime safety is reviewed
# independently; this section reads no HAL implementation and executes no HAL.
assert native_spi['schema_version'] == 1
assert native_spi['scope'] == {
    'family': 'CW32L012', 'controllers': ['SPI1', 'SPI2', 'SPI3'],
    'exact_parts': ['CW32L012C8T6', 'CW32L012C8U6'], 'generic_compile_only': ['CW32L012'],
    'family_count': 1, 'family_instance_count': 3, 'exact_part_count': 2,
    'selected_part_instance_route_count': 6,
    'mode': 'Finite full-duplex u8 master with paired exclusive static SRAM stages',
    'uart_dma_rx_qualified': False,
}
assert native_spi['text_extractor'] == lock['text_extractor']
contract = native_spi['source_index_append_plan']
assert contract['new_consumer'] == native_spi_path and contract['checker_extension'] == classic_checker
assert set(contract['native_exact_pointers']) == native_spi_pointers
assert len(contract['native_exact_pointers']) == contract['new_ledger_backlinks'] == 14
assert contract['append_member_pointer'] == '/artifacts/6/members/71'
assert contract['new_checker_backlinks'] == ['/artifacts/6/members/37', '/artifacts/6/members/71']
assert contract['old_classic_spi_node_count_unchanged'] == len(spi_reverse) == 39
assert contract['old_l012_native_node_count_unchanged'] == len(l012_pointers) == 16
native_spi_bytes = {}
for source in native_spi['sources']:
    pointer = source['source_index_pointer']
    pin = four_index(pointer)
    artifact = lock['artifacts'][int(pointer.split('/')[2])]
    identity = json.loads(json.dumps(source['identity']))
    if pointer == '/artifacts/42':
        # Retain the entire EN pin history, reversing only its historical labels.
        assert len(identity['pin_history']) == 2
        for historical in identity['pin_history']:
            assert set(historical) == {'historical_path', 'historical_sha256', 'bytes', 'changed_utc', 'reason'}
            historical['path'] = historical.pop('historical_path')
            historical['sha256'] = historical.pop('historical_sha256')
    observed = {k: v for k, v in pin.items() if k != 'evidence'
                and not (k == 'members' and pointer == '/artifacts/6')}
    for value in (observed, identity):
        if 'text' in value:
            value['text'] = {k: v for k, v in value['text'].items() if k != 'evidence'}
    assert observed == identity, source['source_ref']
    assert source['official_url'] == artifact['url']
    assert pin['evidence'].count(native_spi_path) == pin['evidence'].count(classic_checker) == 1
    if '/members/' in pointer:
        assert source['source_ref'] == 'member:' + identity['path']
        assert source['archive_source_ref'] == artifact['id']
    elif pointer.endswith('/text'):
        assert source['source_ref'] == 'text:' + identity['path']
    else:
        assert source['source_ref'] == artifact['id']
    raw = (SOURCES / identity['path']).read_bytes()
    assert identity['path'] not in native_spi_bytes
    assert len(raw) == identity['bytes'] and digest(raw) == identity['sha256']
    native_spi_bytes[identity['path']] = raw
    if '/members/' in pointer:
        payload = (SOURCES / artifact['path']).read_bytes()
        for member in identity['members']:
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                payload = archive.read(member)
        assert payload == raw, source['source_ref']
assert len(native_spi_bytes) == 14
native_spi_reverse = set()
for index, artifact in enumerate(lock['artifacts']):
    nodes = [(f'/artifacts/{index}', artifact)]
    nodes += [(f'/artifacts/{index}/members/{i}', m) for i, m in enumerate(artifact.get('members', []))]
    if 'text' in artifact:
        nodes.append((f'/artifacts/{index}/text', artifact['text']))
    for pointer, node in nodes:
        assert node.get('evidence', []).count(native_spi_path) <= 1
        if native_spi_path in node.get('evidence', []):
            native_spi_reverse.add(pointer)
assert native_spi_reverse == native_spi_pointers

# Exact extraction-page locators, including their separately stated printed
# coordinates. Never replace these with the clock checker's single-page hash.
native_spi_page_numbers = {
    'CW32L012_DataSheet_CN_V1.0.txt': [(7, 5), (29, 27), (34, 32), (35, 33), (36, 34), (38, 36), (40, 38), (70, 68)],
    'CW32L012_UserManual_CN_V1.4.txt': [(38, 13), (81, 56), (82, 57), (85, 60), (86, 61),
        (95, 70), (96, 71), (132, 107), (138, 113), (139, 114), (140, 115), (141, 116),
        (142, 117), (143, 118), (144, 119), (145, 120), (146, 121), (147, 122), (148, 123),
        (531, 506), (532, 507), (533, 508), (541, 516), (542, 517), (543, 518),
        (544, 519), (545, 520), (546, 521), (547, 522)],
    'CW32L012_UserManual_EN_V1.0.txt': [(154, 129)],
}
assert len(native_spi['page_records']) == 38
assert {(p['text_path'], p['pdf_page_index'], p['printed_page']) for p in native_spi['page_records']} == {
    (path, index, printed) for path, values in native_spi_page_numbers.items() for index, printed in values}
native_spi_pages = {path: native_spi_bytes[path].decode().split('\f') for path in native_spi_page_numbers}
for page in native_spi['page_records']:
    assert page['pdf_viewer_page'] == page['pdf_page_index'] + 1
    assert page['claims']
    assert digest(native_spi_pages[page['text_path']][page['pdf_page_index']].encode()) == page['extracted_page_utf8_sha256']
manual = native_spi_pages['CW32L012_UserManual_CN_V1.4.txt']
datasheet = native_spi_pages['CW32L012_DataSheet_CN_V1.0.txt']
for index, tokens in {
    38: ['2.7', '其它外设只支持 32bit 访问方式', '8bit/16bit', 'HardFault', '打开对应外设的配置时钟和工作时钟'],
    81: ['SYSCTRL_AHBEN', '31:16 KEY WO', 'KEY 为 0x5A5A', '0 DMA RW'],
    82: ['SYSCTRL_APBEN1', '31:16 KEY WO', 'KEY 为 0x5A5A', '2 SPI1 RW', '13 SPI2 RW', '14 SPI3 RW'],
    85: ['SYSCTRL_AHBRST', '0 DMA RW', '0：模块处于复位状态', '1：模块正常工作'],
    86: ['SYSCTRL_APBRST1', '2 SPI1 RW', '13 SPI2 RW', '14 SPI3 RW', '0：模块处于复位状态', '1：模块正常工作'],
    95: ['25 9 可配置 DMA1/2', '26 10 可配置 DMA3/4', '27 11 可配置 CORDIC'],
    96: ['41 25 可配置 SPI1', '42 26 可配置 SPI2/3'],
    132: ['8.2', '4 条独立 DMA 通道', '1~65535'],
    138: ['8.4.4', 'DMA会传输1个数据块，然后等待下一次硬件触发请求信号', 'DMA_CNTy.REPEAT=1', 'DMA_CNTy.CNT=3'],
    139: ['8.4.5', '硬件触发 BULK 传输模式没有传输间隙', 'DMA_CNTy.CNT个数据块'],
    140: ['8.5.1', '位宽应与 DMA 通道的源地址和目的地址的位宽完全一致', 'REPEAT',
          '在下次 DMA 传输前需要再写入 1', '1~65535', 'RESTART', 'SRCLOAD', 'DSTLOAD'],
    141: ['传输完成 数据传输全部正确完成', 'DMA_CSRy.TCIE', 'DMA_CSRy.TEIE', 'DMA_ICR.TCy', 'DMA_ICR.TEy'],
    142: ['8.7', 'DMA_CSRy', 'DMA_CNTy', 'DMA_TRIGy', 'DMA_CCNTy', 'y = 1~4'],
    143: ['DMA_ISR', 'TC1', 'TE1', 'TC4', 'TE4', 'RO'],
    144: ['DMA_ICR', 'Reset value: 0xFFFF FFFF', 'R1W0', '15 DSTLOAD RW', '14 SRCLOAD RW'],
    145: ['13 TE RO', '12 TC RO', '11 RESTART RW', '10:8 STATUS RW', '101：传输完成',
          '00：位宽为 8 比特，自增量为 1', '位宽配置仅对存储器（FLASH、RAM）有效', '1：块传输（BLOCK）'],
    146: ['y = 1~4', '7:2 HARDSRC RW', '0 TYPE RW', '1：硬件触发模式', 'W0：无功能', 'R0：传输已完成',
          '000110：SPI1 接收 Buf 非空事件', '000111：SPI1 发送 Buf 空事件', '001000：SPI2 接收 Buf 非空事件', '001001：SPI2 发送 Buf 空事件', '001010：SPI3 接收 Buf 非空事件', '001011：SPI3 发送 Buf 空事件'],
    147: ['19:16 REPEAT RW', '15:0 CNT RW', 'REPEAT 只可设置为 1', 'REPEAT 不可设置为 0', '31:0 SRCADDR RW', '31:0 DSTADDR RW'],
    148: ['Address offset: 0x14 + 32×y', '19:16 CREPEAT RO', '15:0 CCNT RO', '31:0 CSRCADDR RO', '31:0 CDSTADDR RO'],
    531: ['22.3.8', '当发送数据从 SPI_DR 寄存器转移到移位寄存器后',
          '在对 SPI_DR 寄存器写入数据的同时，SPI_ISR.TXE 标志位会被自动清零',
          '读 SPI_DR 寄存器的同时将清除 SPI_ISR.RXNE 标志位',
          '发送缓冲区无数据且移位寄存器无数据', '用户代码需查询到该标志为 0 时，才可拉高 CS 信号'],
    532: ['SPI_CR2.EN 被清零', 'MODF 标志位会保持置位状态', '新接收到的数据将覆盖缓冲器中原有的数据',
          'SPI_ISR.OV 标志位会保持置位状态', 'SPI_ISR.SSERR', 'SPI_ISR.UD'],
    533: ['支持 8 个中断源', 'ISR.TXE IER.TXE 写 SPI_DR', 'ISR.RXNE IER.RXNE 读 SPI_DR', 'IER.MODF', 'IER.SSERR', 'IER.OV', 'IER.UD'],
    541: ['Address offset: 0x00', '30:24 BR RW', '127：SCK = PCLK/256', '17:16 MODE RW', '00：双向全双工'],
    542: ['每帧数据的宽度为 WIDTH+1', '11:8 WIDTH RW', '1 SSM RW', '0 MSTR RW', '1：主机模式',
          '只有当 SPI_CR2.EN 为 0 时，才可以修改本寄存器'],
    543: ['Address offset: 0x04', '2 DMATX RW', '1 DMARX RW', '0 EN RW', '保留位，请保持默认值', 'Address offset: 0x08', '0 HDOE RW'],
    544: ['22.7.4', 'Address offset: 0x0C', '0 SSI RW', 'SPI_CR1.SSM 为 1 时有效'],
    545: ['Address offset: 0x10', '7 MODF RW', '6 SSERR RW', '5 OV RW', '4 UD RW', '3 SSR RW', '2 SSF RW', '1 RXNE RW', '0 TXE RW'],
    546: ['Address offset: 0x14', '8 BUSY RO', '1 RXNE RO', '0 TXE RO', '对 DR 寄存器的读操作或对 ICR.RXNE 写 0 均可以清除该标志'],
    547: ['Address offset: 0x18', 'Reset value: 0x0000 00FF', '0 FLUSH R1W0', '清空发送缓冲区和移位寄存器',
          'W1：无功能', 'Address offset: 0x1C', '15:0 DR RW'],
}.items():
    four_tokens(manual[index], tokens, ('L012 SPI own manual', index))
assert 'BUSY' not in manual[533] and 'BUSY' not in manual[545]
four_tokens(native_spi_pages['CW32L012_UserManual_EN_V1.0.txt'][154],
            ['13 TE RO', '12 TC RO', '10:8 STATUS RW', '101: Transfer completed',
             'Bit width configuration is only valid for memory'], 'L012 SPI independent EN width')
for index, tokens in {
    7: ['表 3-1', 'CW32L012C8T6', 'CW32L012C8U6', 'FLASH 64KB', 'SRAM 8KB', 'SPI 3', 'LQFP48', 'QFN48'],
    29: ['内部集成 3 个串行外设接口', '主机模式下 SCK 频率高达 24MHz', '支持直接内存访问（DMA）'],
    34: ['表 5-2', 'PA00', 'PA01', 'PA02'],
    35: ['PA05', 'PA06', 'PA07', 'PB00', 'PB01', 'PB02'],
    36: ['PA09', 'PA10', 'PA11'],
    38: ['表 5-3', 'SPI1_SCK', 'SPI2_SCK', 'SPI3_SCK'],
    40: ['0x2000 0000 – 0x2000 1FFF', '0x0000 0000 – 0x0000 FFFF', 'SRAM(8KB)', 'FLASH(64KB)', 'SPI1', 'SPI2', 'SPI3'],
    70: ['表 7-39', 'SPI 时钟周期', '主模式', '24', 'ns'],
}.items():
    four_tokens(datasheet[index], tokens, ('L012 SPI own datasheet', index))

# These are distinct widths, not a correction to the manual's general guidance.
facts = native_spi['facts']
assert [facts[k] for k in ('cpu_mmio_bits', 'dma_peripheral_transaction_bits', 'sram_element_bits', 'dr_payload_bits', 'frame_bits')] == [32, 32, 8, 16, 8]
assert facts['cr1'] == {'MODE': 0, 'MSTR': 1, 'SSM': 1, 'WIDTH': 7, 'modify_only_when_cr2_en': 0}
assert facts['native_request_gates'] == {'DMARX': [1, 1], 'DMATX': [2, 1]}
assert facts['dma_descriptor'] == {'TYPE': 1, 'TRANS': 1, 'SIZE': 0, 'REPEAT': 1, 'CNT_min': 1, 'CNT_max': 65535,
    'RESTART': 0, 'SRCLOAD': 0, 'DSTLOAD': 0, 'sram_increment': True, 'dr_increment': False}
assert facts['dma_terminal'] == {'TC': True, 'TE': False, 'STATUS': 5, 'error_statuses': [1, 2, 3, 4],
    'software_trigger_retirement_required_for_hardware_request': False,
    'remaining_register': 'CCNT', 'remaining_fields': ['CCNT', 'CREPEAT']}
assert facts['spi_icr_noop'] == 255 and facts['destructive_icr_field'] == 'FLUSH'
assert facts['error_fields'] == ['UD', 'OV', 'SSERR', 'MODF']
assert facts['dma_icr_noop'] == l012['native_icr_seed']['write_noop'] == 0xffffffff
assert facts['dma_base'] == 0x40020000 and facts['channel_count'] == 4
assert facts['dma_irqs'] == {'DMACH12': 9, 'DMACH34': 10}
assert facts['spi_irqs'] == {'SPI1': 25, 'SPI23': 26} and facts['unrelated_irq'] == {'CORDIC': 11}
assert facts['request_selectors'] == {f'SPI{i}': {'RX': 4 + 2 * i, 'TX': 5 + 2 * i} for i in range(1, 4)}
assert facts['dr_addresses'] == {'SPI1': 0x4000081c, 'SPI2': 0x4000041c, 'SPI3': 0x4000341c}
retained = native_spi['retained_native_dma_proof']
assert retained['ledger'] == 'docs/l012-dma-owned-evidence.json'
assert (retained['source_node_count'], retained['ro_current_count_views'], retained['tc_te_setter_removals']) == (16, 5, 10)

sdk = native_spi['sdk']
with zipfile.ZipFile(io.BytesIO(native_spi_bytes[lock['artifacts'][6]['path']])) as archive:
    spi_examples = sorted({'/'.join(name.split('/')[:3]) for name in archive.namelist()
                           if name.startswith('Examples/SPI/') and len(name.split('/')) > 3})
assert spi_examples == sdk['spi_example_directories'] == ['Examples/SPI/SPI_FLASH', 'Examples/SPI/SPI_FullDuplex_ComPolling']
assert sdk['has_spi_dma_example'] is False and not any('DMA' in name.upper() for name in spi_examples)
assert len(sdk['symbols']) == 17
for symbol in sdk['symbols']:
    assert symbol['source_ref'].startswith('member:')
    lines = native_spi_bytes[symbol['source_ref'].removeprefix('member:')].decode().splitlines()
    assert lines[symbol['line'] - 1].strip() == symbol['text']
spi_header = native_spi_bytes['cw32l012/Libraries/inc/cw32l012_spi.h'].decode()
four_tokens(spi_header, ['#define SPI_DMAReq_Tx (0x00000004)', '#define SPI_DMAReq_Rx (0x00000002)',
    '#define IS_SPI_ALL_PERIPH(PERIPH) (((PERIPH) == CW_SPI1) || ((PERIPH) == CW_SPI2) ||((PERIPH) == CW_SPI3))'], 'L012 SPI SDK gates/instances')
spi_driver = native_spi_bytes['cw32l012/Libraries/src/cw32l012_spi.c'].decode()
four_tokens(spi_driver, ['SPIx->CR2 |= SPI_DMAReq;', 'SPIx->CR2 &= ~SPI_DMAReq;',
    'SPIx->DR = Data;', 'return (uint16_t)(SPIx->DR);'], 'L012 SPI SDK helper semantics')

# All native register geometry is bound to authored/generated IR. Reuse the
# earlier native DMA corrections rather than recreating an independent variant.
for version, geometry in native_spi['native_register_geometry'].items():
    ir = load(ROOT / f'cw32-data/data/registers/{version}.json')
    assert ir == load(ROOT / f'cw32-data/registers/{version}.yaml')
    block = {'spi_cw32l012_v1': 'SPI', 'dma_cw32l012_v1': 'DMA', 'dmachannel_cw32l012_v1': 'DMACHANNEL'}[version]
    assert geometry['registers'] == [{k: v for k, v in r.items() if k != 'description'} for r in ir['block/' + block]['items']]
    assert geometry['fields'] == {k: [{a: b for a, b in f.items() if a != 'description'} for f in v['fields']]
                                  for k, v in ir.items() if k.startswith('fieldset/')}
    if version != 'spi_cw32l012_v1':
        assert ir == l012_ir[version]
assert set(native_spi['native_register_geometry']) == {'spi_cw32l012_v1', 'dma_cw32l012_v1', 'dmachannel_cw32l012_v1'}
native_spi_ir = load(ROOT / 'cw32-data/data/registers/spi_cw32l012_v1.json')
native_spi_offsets = dict(zip(('CR1', 'CR2', 'CR3', 'SSI', 'IER', 'ISR', 'ICR', 'DR'), range(0, 32, 4)))
assert {r['name']: r['byte_offset'] for r in native_spi_ir['block/SPI']['items']} == native_spi_offsets
assert {f['name']: (f['bit_offset'], f['bit_size']) for f in native_spi_ir['fieldset/CR1']['fields']} == {
    'MSTR': (0, 1), 'SSM': (1, 1), 'CPHA': (2, 1), 'CPOL': (3, 1), 'LSBF': (7, 1),
    'WIDTH': (8, 4), 'GAP': (12, 4), 'MODE': (16, 2), 'FLTEN': (18, 1), 'MISOHD': (19, 1), 'SMP': (20, 1), 'BR': (24, 7)}
assert {f['name']: (f['bit_offset'], f['bit_size']) for f in native_spi_ir['fieldset/CR2']['fields']} == {'EN': (0, 1), 'DMARX': (1, 1), 'DMATX': (2, 1)}
for name in ('SPI1', 'SPI2', 'SPI3'):
    peripheral = l012_peripherals[name]
    assert peripheral.get('derivedFrom') == (None if name == 'SPI1' else 'SPI1')
    assert int(peripheral.findtext('baseAddress'), 0) + 28 == facts['dr_addresses'][name]
    vector = 'SPI1' if name == 'SPI1' else 'SPI23'
    assert [(i.findtext('name'), int(i.findtext('value'), 0)) for i in peripheral.findall('interrupt')] == [(vector, facts['spi_irqs'][vector])]
    if name != 'SPI1':
        assert peripheral.find('registers') is None
for original in l012_peripherals['SPI1'].findall('registers/register'):
    name = original.findtext('name')
    ir = next(r for r in native_spi_ir['block/SPI']['items'] if r['name'] == name)
    assert ir['byte_offset'] == int(original.findtext('addressOffset'), 0)
    assert ir.get('bit_size', 32) == 32 and ir['fieldset'] == name
    access = 'Read' if name == 'ISR' else 'ReadWrite'
    assert ir.get('access', 'ReadWrite') == access
    assert original.findtext('access') == ('read-only' if name == 'ISR' else 'read-write')
    fields = original.findall('fields/field')
    assert len(fields) == len(native_spi_ir['fieldset/' + name]['fields'])
    assert {f.findtext('name'): (int(f.findtext('lsb'), 0), int(f.findtext('msb'), 0) - int(f.findtext('lsb'), 0) + 1) for f in fields} == {
        f['name']: (f['bit_offset'], f['bit_size']) for f in native_spi_ir['fieldset/' + name]['fields']}
    assert all(f.findtext('access') == ('read-only' if name == 'ISR' else 'read-write') for f in fields)
assert [{k: v for k, v in f.items() if k != 'description'} for f in native_spi_ir['fieldset/DR']['fields']] == [{'name': 'DR', 'bit_offset': 0, 'bit_size': 16}]
assert [{k: v for k, v in f.items() if k != 'description'} for f in native_spi_ir['fieldset/SSI']['fields']] == [{'name': 'SSI', 'bit_offset': 0, 'bit_size': 1}]
assert {f['name'] for f in native_spi_ir['fieldset/IER']['fields']} == {'TXE', 'RXNE', 'SSF', 'SSR', 'UD', 'OV', 'SSERR', 'MODF'}
command = load(ROOT / 'cw32-data/register-writes.yaml')['registers']['spi_cw32l012_v1']
assert len(command) == 1 and command[0]['register'] == command[0]['fieldset'] == 'ICR'
assert command[0]['write_noop'] == command[0]['reset_value'] == 255
assert command[0]['zero_to_clear_fields'] == ['FLUSH', 'RXNE', 'SSF', 'SSR', 'UD', 'OV', 'SSERR', 'MODF']
assert load(ROOT / 'cw32-data/data/register-writes/spi_cw32l012_v1.json') == {
    'schema_version': 1, 'registers': {'spi_cw32l012_v1': command}}
pac = (ROOT / 'cw32-metapac/src/peripherals/spi_cw32l012_v1.rs').read_text()
for name, offset in native_spi_offsets.items():
    access = 'R' if name == 'ISR' else 'RW'
    match = re.search(rf'pub const fn {name.lower()}\(self\) -> crate::common::Reg<regs::{name.title()}, crate::common::{access}>\s*\{{(.*?)\n    \}}', pac, re.S)
    assert match, ('native SPI typed handle', name)
    pointer = re.search(r'self\.ptr\.wrapping_add\((0x[0-9a-f]+)usize\)', match[1])
    assert pointer and int(pointer[1], 16) == offset, ('native SPI typed offset', name)
assert 'pub struct Dr(pub u32)' in pac
assert 'pub const fn dr(&self) -> u16' in pac and 'pub const fn set_dr(&mut self, val: u16)' in pac
icr_impl = re.search(r'impl regs::Icr \{(.*?)\n\}', pac, re.S)[1]
four_tokens(icr_impl, ['pub const fn reset_value() -> Self { Self(255) }',
    'pub const fn write_noop() -> Self { Self(255) }'], 'native SPI typed ICR seed')

# Three native controllers, six exact-part selections. The generic chip retains
# topology but no memory/package admission. Compare selected GPIO macros, pad
# columns and AF column positions against original sources, including native PIN.
assert facts['selected_routes'] == {
    'SPI1': {'SCK': 'PA5', 'MOSI': 'PA7', 'MISO': 'PA6', 'af': 3, 'cs': 'PB0'},
    'SPI2': {'SCK': 'PA2', 'MOSI': 'PA1', 'MISO': 'PA0', 'af': 8, 'cs': 'PB1'},
    'SPI3': {'SCK': 'PA9', 'MOSI': 'PA11', 'MISO': 'PA10', 'af': 8, 'cs': 'PB2'},
}
assert facts['selected_pads'] == {'PA0': '10', 'PA1': '11', 'PA2': '12', 'PA5': '15', 'PA6': '16', 'PA7': '17',
    'PB0': '18', 'PB1': '19', 'PB2': '20', 'PA9': '30', 'PA10': '31', 'PA11': '32'}
af = load(ROOT / 'cw32-data/af/cw32l012-serial.yaml')
selected_keys = {(name, signal, route[signal], route['af']) for name, route in facts['selected_routes'].items() for signal in ('SCK', 'MOSI', 'MISO')}
assert len(native_spi['selected_af_routes']) == 9
assert native_spi['selected_af_routes'] == [r for r in af['routes'] if (r['peripheral'], r['signal'], r['pin'], r['af']) in selected_keys]
gpio_lines = native_spi_bytes['cw32l012/Libraries/inc/cw32l012_gpio.h'].decode().splitlines()
af_page = datasheet[38]
header = next(line for line in af_page.splitlines() if '功能 3' in line)
columns = [(int(m[1]), m.start() + len(m[0]) / 2) for m in re.finditer(r'功能\s+(\d)', header)]
for route in native_spi['selected_af_routes']:
    pin = route['pin']; af_number = route['af']; source_pin = pin[:2] + pin[2:].zfill(2)
    assert route['datasheet_cell'] == {'pin': pin, 'af': af_number, 'function': route['peripheral'] + '_' + route['signal'],
        'table': '5-3', 'pdf_page': 39, 'printed_page': 36}
    line = gpio_lines[route['source_line'] - 1]
    assert re.search(r'#define\s+' + re.escape(route['source_macro']) + r'\b', line)
    four_tokens(line, [f"CW_GPIO{pin[1]}->{route['gpio_register']}_f.{route['gpio_field']} = {af_number}"], ('native GPIO macro', pin))
    assert route['gpio_field'] == 'PIN' + pin[2:]
    row = next(line for line in af_page.splitlines() if re.match(rf'\s*{source_pin}\b', line))
    match = re.search(route['datasheet_cell']['function'], row)
    assert match and min(columns, key=lambda c: abs(c[1] - (match.start() + match.end()) / 2))[0] == af_number
pinout = load(ROOT / 'cw32-data/pinouts/cw32l012.yaml')
assert len(native_spi['chip_projections']) == 3
assert {p['chip'] for p in native_spi['chip_projections']} == {'CW32L012C8T6', 'CW32L012C8U6', 'CW32L012'}
route_count = 0
for projection in native_spi['chip_projections']:
    chip = load(ROOT / f"cw32-data/data/chips/{projection['chip']}.json")
    core = chip['cores'][0]
    assert projection['feature'] == projection['chip'].lower()
    assert chip['memory'] == projection['memory']
    assert [p['package'] for p in chip['packages']] == projection['package_names']
    assert core['dma_channels'] == projection['dma_channels'] == [
        {'channel': i - 1, 'dma': 'DMA', 'name': f'DMA_CH{i}'} for i in range(1, 5)]
    assert projection['interrupts'] == [i for i in core['interrupts'] if i['name'] in {'DMACH12', 'DMACH34', 'CORDIC', 'SPI1', 'SPI23'}]
    for expected in projection['peripherals']:
        actual = next(p for p in core['peripherals'] if p['name'] == expected['name'])
        assert {k: v for k, v in actual.items() if k not in ('pins', 'rcc')} == expected
    for name in ('SPI1', 'SPI2', 'SPI3'):
        peripheral = next(p for p in core['peripherals'] if p['name'] == name)
        assert peripheral['address'] + 28 == facts['dr_addresses'][name]
        assert peripheral['dma_channels'] == [{'dma': 'DMA', 'request': facts['request_selectors'][name][signal], 'signal': signal} for signal in ('RX', 'TX')]
        assert peripheral['registers'] == {'block': 'SPI', 'kind': 'spi', 'version': 'cw32l012_v1'}
        control = peripheral['rcc_control']
        assert control['enable'] == {'field': name, 'register': 'APBEN1'}
        assert control['reset'] == {'field': name, 'register': 'APBRST1'} and control['reset_asserted_value'] is False
        assert control['enable_write_key'] == {'field': {'field': 'KEY', 'register': 'APBEN1'}, 'value': 0x5a5a}
        assert not any(k.startswith('shared_') for k in control)
        assert peripheral['spi'] == {'maximum_frequency': 24000000, 'minimum_divisor': 2}
        assert [p['name'] for p in core['peripherals'] if p['name'].startswith('SPI') and
                p['interrupts'] == peripheral['interrupts']] == (['SPI1'] if name == 'SPI1' else ['SPI2', 'SPI3'])
    chip_pac = (ROOT / f"cw32-metapac/src/chips/{projection['feature']}/pac.rs").read_text()
    device = (ROOT / f"cw32-metapac/src/chips/{projection['feature']}/device.x").read_text()
    for name, address in facts['dr_addresses'].items():
        constant = re.search(rf'pub const {name}: spi::Spi = unsafe \{{ spi::Spi::from_ptr\((0x[0-9a-f_]+)usize as _\) \}};', chip_pac)
        assert constant and int(constant[1].replace('_', ''), 16) + 28 == address
    for name, number in {**facts['dma_irqs'], **facts['spi_irqs'], **facts['unrelated_irq']}.items():
        assert re.search(rf'\b{name} = {number},', chip_pac)
        assert f'PROVIDE({name} = DefaultHandler);' in device
    if projection['chip'] == 'CW32L012':
        assert chip['memory'] == chip['packages'] == [] and projection['package_signal_positions'] == {}
        continue
    assert chip['memory'] == [[{'address': 0, 'kind': 'flash', 'name': 'FLASH', 'size': 65536},
                              {'address': 0x20000000, 'kind': 'ram', 'name': 'RAM', 'size': 8192}]]
    package = chip['packages'][0]
    own_package = next(p for p in pinout['packages'] if projection['chip'] in p['documented_orderable_parts'])
    assert own_package['package'] == package['package']
    assert package['pins'] == own_package['pins']
    bonded = {s: p['position'] for p in package['pins'] for s in p['signals']}
    assert projection['package_signal_positions'] == {package['package']: facts['selected_pads']}
    for name, route in facts['selected_routes'].items():
        peripheral = next(p for p in core['peripherals'] if p['name'] == name)
        for signal in ('SCK', 'MOSI', 'MISO'):
            assert {'pin': route[signal], 'signal': signal, 'af': route['af']} in peripheral['pins']
        for pin in [route[s] for s in ('SCK', 'MOSI', 'MISO')] + [route['cs']]:
            assert bonded[pin] == facts['selected_pads'][pin]
            row = next(r for r in pinout['table_rows'] if pin in r['signals'])
            assert row['pdf_page_index'] in (34, 35, 36)
            assert row['positions'][own_package['table_column']] == bonded[pin]
            source_pin = pin[:2] + pin[2:].zfill(2)
            source_row = next(line for line in datasheet[row['pdf_page_index']].splitlines() if re.search(rf'\b{source_pin}\b', line))
            assert source_row.split(source_pin, 1)[0].split()[pinout['table_columns'].index(own_package['table_column'])] == bonded[pin]
        route_count += 1
assert route_count == 6
assert facts['clock_policy'] == {k: clock_policy['CW32L012'][k] for k in ('maximum_sck_hz', 'minimum_divisor', 'maximum_divisor', 'divider_kind')}
assert facts['clock_policy'] == {'maximum_sck_hz': 24000000, 'minimum_divisor': 2, 'maximum_divisor': 256, 'divider_kind': 'linear_even'}
assert set(clock_profiles['CW32L012']['peripherals']) == {'SPI1', 'SPI2', 'SPI3'}
assert all(p == {'maximum_frequency': 24000000, 'minimum_divisor': 2} for p in clock_profiles['CW32L012']['peripherals'].values())

# Declared example surfaces only. Ordinary firmware links and source review own
# all method bodies, cancellation, complete handler binding and owner retention.
manifest = tomllib.loads((ROOT / 'examples/l012/spi-dma/Cargo.toml').read_text())
assert manifest['package']['name'] == 'cw32-l012-spi-dma-examples' and manifest['package']['autobins'] is False
assert set(manifest['features']) == {'default', 'cw32l012c8t6', 'cw32l012c8u6'}
assert manifest['features']['default'] == []
for feature in l012_features:
    assert set(manifest['features'][feature]) == {f'embassy-cw32/{feature}', f'cw32-metapac/{feature}'}
assert manifest['bin'] == [{'name': name, 'path': f'src/bin/{name}.rs'} for name in
    ('spi1_dma_pio23', 'spi2_dma_pio3', 'spi3_dma_pio2', 'spi23_dma_pio1')]
assert manifest['dependencies']['embassy-executor']['rev'] == 'f16efeffe37581092ec184718e6fdb1620393214'
assert manifest['build-dependencies']['cw32-metapac']['features'] == ['metadata']
assert (ROOT / 'examples/l012/spi-dma/build.rs').is_file()
for binary in manifest['bin']:
    assert (ROOT / 'examples/l012/spi-dma' / binary['path']).is_file()
pio_manifest = tomllib.loads((ROOT / 'examples/shared/spi-async/Cargo.toml').read_text())
for feature in l012_features:
    assert set(pio_manifest['features'][feature]) == {f'embassy-cw32/{feature}', f'cw32-metapac/{feature}'}
assert pio_manifest['bin'] == [{'name': name, 'path': f'src/bin/{name}.rs'} for name in ('async_master', 'async_words', 'blocking_master')]
assert (ROOT / 'examples/shared/spi-async/src/bin/async_words.rs').is_file()
assert (ROOT / 'examples/shared/spi-async/build.rs').is_file()
mode = next(m for m in capabilities['functions']['spi']['modes'] if m['id'] == 'safe_peripheral_dma')
assert len(set(mode['implemented_families'])) == 8
assert sum(len(clock_profiles[name]['peripherals']) for name in mode['implemented_families']) == 15
print('PASS: native L012 SPI: 14 complete own-source nodes and exact backlinks, 38 semantic pages, native SPI geometry/typed MMIO and retained DMA corrections, three controllers/six exact-package routes, four channels/shared vectors, empty generic admission and declared examples; UART RX remains excluded; source/declaration facts only, no HAL execution or hardware claim')

print('Retired W031 source qualification uses two hash-pinned historical chip snapshots only; neither is a current generated target or runtime/hardware result.')

assert len(list((ROOT / 'cw32-data/data/chips').glob('*.json'))) == 52
for manifest_path in ['embassy-cw32/Cargo.toml', 'cw32-metapac/Cargo.toml']:
    active_features = tomllib.loads((ROOT / manifest_path).read_text())['features']
    assert not {'cw32w031', 'cw32w031r8u6'} & active_features.keys()
print('PASS current DMA target projection: 52 generated selections; both retired W031 Cargo features absent')
