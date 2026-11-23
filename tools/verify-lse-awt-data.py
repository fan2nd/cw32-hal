#!/usr/bin/env python3
"""Check current exact-eight LSE AWT source/data facts; never execute HAL code."""
import argparse
import sys
import hashlib
import json
from pathlib import Path
import re
import tomllib

import yaml

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tests"))
from current_targets import active_names, retired_targets
EVIDENCE = "docs/lse-awt-evidence.json"
PARTS = {
    "CW32F020C6U7": ("CW32F020", "QFN48", "3", "4", (1650, 5500), (-40, 105)),
    "CW32F030C8T7": ("CW32F030", "LQFP48", "3", "4", (1650, 5500), (-40, 105)),
    "CW32A030C8T7": ("CW32A030", "LQFP48", "3", "4", (1650, 5500), (-40, 105)),
    "CW32L031C8T6": ("CW32L031", "LQFP48", "3", "4", (1650, 5500), (-40, 85)),
    "CW32L031C8U6": ("CW32L031", "QFN48", "3", "4", (1650, 5500), (-40, 85)),
    "CW32L031F8U6": ("CW32L031", "QFN20", "1", "2", (1650, 5500), (-40, 85)),
    "CW32R031C8U6": ("CW32R031", "QFN48", "2", "3", (2200, 3600), (-40, 85)),
    "CW32W031R8U6": ("CW32W031", "QFN64", "61", "62", (2000, 3600), (-40, 85)),
}
FAMILIES = {
    "CW32F020": ("CW32F020_UserManual_CN_V1.4.pdf", "current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf", "cw32f030_v1", 36),
    "CW32F030": ("CW32x030_UserManual_CN_V2.5.pdf", "CW32F030_DataSheet_CN_V1.9.pdf", "cw32f030_v1", 38),
    "CW32A030": ("CW32x030_UserManual_CN_V2.5.pdf", "CW32A030_DataSheet_CN_V1.1.pdf", "cw32f030_v1", 35),
    "CW32L031": ("CW32L031_UserManual_CN_V1.6.pdf", "CW32L031_DataSheet_CN_V1.9.pdf", "cw32l031_v1", 38),
    "CW32R031": ("CW32R031_UserManual_CN_V1.3.pdf", "CW32R031_DataSheet_CN_V1.2.pdf", "cw32l031_v1", 5),
    "CW32W031": ("CW32W031_UserManual_CN_V1.4.pdf", "CW32W031_DataSheet_CN_V1.3.pdf", "cw32l031_v1", 4),
}
REGISTERS = {
    "CR": (0, "ReadWrite"), "ARR": (4, "ReadWrite"), "CNT": (8, "Read"),
    "IER": (16, "ReadWrite"), "ISR": (20, "Read"), "ICR": (28, "ReadWrite"),
}
FIELDS = {
    "CR": {"EN": [0, 1], "MD": [1, 2], "PRS": [4, 4], "SRC": [8, 3]},
    "ARR": {"ARR": [0, 16]}, "CNT": {"CNT": [0, 16]},
    "IER": {"UD": [3, 1]}, "ISR": {"UD": [3, 1]}, "ICR": {"UD": [3, 1]},
}
CONTROL = {
    "controller": "SYSCTRL", "bus_clock": "PCLK",
    "enable": {"register": "APBEN2", "field": "AWT"}, "enable_active_value": True,
    "reset": {"register": "APBRST2", "field": "AWT"}, "reset_asserted_value": False,
}


def unique(pairs):
    result = {}
    for key, value in pairs:
        assert key not in result, ("duplicate key", key)
        result[key] = value
    return result


class StrictLoader(yaml.SafeLoader):
    pass


def mapping(loader, node):
    loader.flatten_mapping(node)
    return unique((loader.construct_object(k, deep=True), loader.construct_object(v, deep=True))
                  for k, v in node.value)


StrictLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, mapping)


def load(path):
    if path.suffix == ".json":
        return json.loads(path.read_text(), object_pairs_hook=unique)
    return yaml.load(path.read_text(), Loader=StrictLoader)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def within(root, relative):
    path = Path(relative)
    assert not path.is_absolute() and ".." not in path.parts, ("relative source path", relative)
    return root / path


def one(rows, label):
    rows = list(rows)
    assert len(rows) == 1, (label, len(rows))
    return rows[0]


def compact(text):
    return re.sub(r"\s+", "", text)


def check_ir(ir, lse):
    regs = {row["name"]: row for row in ir["block/AWT"]["items"]}
    assert set(regs) == set(REGISTERS)
    for name, (offset, access) in REGISTERS.items():
        row = regs[name]
        assert row["byte_offset"] == offset and row.get("access", "ReadWrite") == access
        assert row.get("bit_size", 32) == 32 and row.get("array") is None
        fields = ir["fieldset/" + row["fieldset"]]["fields"]
        assert {f["name"]: [f["bit_offset"], f["bit_size"]] for f in fields} == FIELDS[name]
        assert all(f.get("array") is None for f in fields)
    assert ir["enum/Source"]["bit_size"] == 3
    sources = {v["name"]: v["value"] for v in ir["enum/Source"]["variants"]}
    assert sources["HSIOSC"] == 0
    if lse:
        assert sources["LSE"] == 3 and sources["HSE"] == 2
        assert [name for name, value in sources.items() if value == 3] == ["LSE"]
        assert "HEX_PB0" not in sources and "HEX_PB1" not in sources
    else:
        assert sources["HEX_PB0"] == 2 and sources["HEX_PB1"] == 3 and "LSE" not in sources
    assert ir["enum/Mode"]["bit_size"] == 2
    assert [(v["name"], v["value"]) for v in ir["enum/Mode"]["variants"]] == [("TIMER", 3)]
    assert ir["enum/Prescaler"]["bit_size"] == 4
    assert [(v["name"], v["value"]) for v in ir["enum/Prescaler"]["variants"]] == [
        ("DIV" + str(1 << value), value) for value in range(1, 16)]
    cr = {f["name"]: f for f in ir["fieldset/CR"]["fields"]}
    assert {key: cr[key]["enum"] for key in ["SRC", "MD", "PRS"]} == {
        "SRC": "Source", "MD": "Mode", "PRS": "Prescaler"}


def check_sources(proof, source_root):
    lock = load(ROOT / proof["source_authority"])
    authority = {a["id"]: a for a in lock["artifacts"]}
    sources = {s["source_ref"]: s for s in proof["sources"]}
    assert len(sources) == len(proof["sources"]) == 11
    assert set(sources) == {"vendor:" + path for row in FAMILIES.values() for path in row[:2]}
    pages = {}
    for source_ref, source in sources.items():
        record = authority[source_ref]
        assert record["provenance"]["status"] == "selected"
        assert EVIDENCE in record["evidence"]
        assert source["path"] == record["path"] and source["url"] == record["url"]
        assert source["sha256"] == record["sha256"]
        assert digest(within(source_root, source["path"]).read_bytes()) == record["sha256"]
        assert (source["text_path"], source["text_sha256"]) == (record["text"]["path"], record["text"]["sha256"])
        text = within(source_root, source["text_path"]).read_bytes()
        assert digest(text) == source["text_sha256"]
        text_pages = text.decode().split("\f")
        indices = [p["pdf_page_1_based"] for p in source["pages"]]
        assert len(indices) == len(set(indices)) and set(indices) == set(source["locators"].values())
        for page in source["pages"]:
            number = page["pdf_page_1_based"]
            assert 1 <= number <= len(text_pages)
            assert page["printed_page"] == number - 1
            assert digest(text_pages[number - 1].encode()) == page["text_sha256"]
            pages[(source_ref, number)] = text_pages[number - 1]
    for family, (manual, datasheet, version, temperature_page) in FAMILIES.items():
        row = proof["families"][family]
        assert row["manual_source_ref"] == "vendor:" + manual
        assert row["datasheet_source_ref"] == "vendor:" + datasheet
        assert row["register_version"] == version
        for source_ref in [row["manual_source_ref"], row["datasheet_source_ref"]]:
            assert family in authority[source_ref]["provenance"]["chip_scope"]
        rm = sources[row["manual_source_ref"]]
        ds = sources[row["datasheet_source_ref"]]
        def manual_page(name):
            return compact(pages[(rm["source_ref"], rm["locators"][name])])
        for term in ["011：LSE", "MD", "请写入11", "AWT_CR[10:1]"]:
            assert term in manual_page("control"), (family, "AWT CR", term)
        for term in ["R1W0", "W0：清除", "W1：无功能"]:
            assert term in manual_page("event"), (family, "UD command", term)
        assert "AWT" in manual_page("configuration_clock") and "工作时钟通过各模块的时钟源选择寄存器进行配置" in manual_page("configuration_clock")
        assert "AWT模块配置时钟使能控制" in manual_page("gate")
        assert "AWT模块复位控制" in manual_page("reset")
        assert "0：模块处于复位状态" in manual_page("reset")
        assert "1：模块正常工作" in manual_page("reset")
        irq_page = pages[(rm["source_ref"], rm["locators"]["irq"]) ]
        assert re.search(r"\b46\s+30\s+可配置\s+AWT\b", irq_page), (family, "AWT IRQ30")
        for term in ["AWT_CR.EN", "AWT_ARR", "AWT_ICR.UD", "SYSCTRL_APBEN2.AWT"]:
            assert term in manual_page("sequence"), (family, "timer sequence", term)
        assert "ARR+1" in manual_page("timing")
        assert "256" in manual_page("fault") and "128" in manual_page("fault") and "LSEFAULT" in manual_page("fault")
        native = family in {"CW32L031", "CW32R031", "CW32W031"}
        assert row["gpio_filter_awt_selector"] == (7 if native else None)
        assert row["gpio_filter_banks"] == (["GPIOA", "GPIOB", "GPIOC", "GPIOF"] if native else [])
        assert ("111：AWT溢出" in manual_page("gpio_filter")) == native
        assert ds["locators"]["temperature"] == temperature_page
        temperature = compact(pages[(ds["source_ref"], temperature_page)])
        assert "-40" in temperature and ("85" if native else "105") in temperature
        assert "环境温度" in temperature or "工作温度" in temperature
        pad_page = compact(pages[(ds["source_ref"], ds["locators"]["pads"])])
        assert all(term in pad_page for term in ["PC14", "PC15", "OSC32_IN", "OSC32_OUT"])
    return len(sources), len(pages)


def check_data(proof, data):
    catalog = load(ROOT / "cw32-data/lse-qualified.yaml")
    intersection = set()
    for name, row in catalog["parts"].items():
        family_input = load(ROOT / f"cw32-data/inputs/{row['family'].lower()}.yaml")
        has_awt = "awt" in family_input["register_versions"]
        assert (row["configuration"]["awt_source"] is not None) == has_awt
        if has_awt:
            intersection.add(name)
    assert intersection == set(PARTS) == set(proof["parts"])
    assert set(proof["families"]) == set(FAMILIES)
    part_catalog = {p["name"]: p for p in load(ROOT / "cw32-data/parts.yaml")["parts"]}
    for name, (family, package, input_pos, output_pos, supply, temperature) in PARTS.items():
        row = proof["parts"][name]
        current = catalog["parts"][name]
        expected = {"family": family, "package": package, "input_pin": "PC14", "output_pin": "PC15",
                    "input_position": input_pos, "output_position": output_pos,
                    "nominal_hz": 32768, "maximum_hz": 1000000,
                    "supply_mv": list(supply), "temperature_c": list(temperature), "awt_source": 3}
        assert row == expected
        assert all(current[key] == row[key] for key in ["family", "package", "input_pin", "output_pin"])
        assert all(current["configuration"][key] == row[key] for key in ["nominal_hz", "maximum_hz", "supply_mv", "temperature_c", "awt_source"])
        assert part_catalog[name]["feature"] == name.lower() and part_catalog[name]["package"] == package
        f = proof["families"][family]
        for key, directory in [("input_path", "inputs"), ("pinout_path", "pinouts"), ("clock_path", "clock")]:
            assert f[key] == f"cw32-data/{directory}/{family.lower()}.yaml"
        assert load(ROOT / f["input_path"])["register_versions"]["awt"] == f["register_version"]
        package_row = one((p for p in load(ROOT / f["pinout_path"])["packages"] if p["name"] == name), name)
        assert package_row["package"] == package
        for pin, position, signal in [("PC14", input_pos, "OSC32_IN"), ("PC15", output_pos, "OSC32_OUT")]:
            assert any(p["position"] == position and pin in p["signals"] and signal in p["signals"] for p in package_row["pins"])
        clock = one((p for p in load(ROOT / f["clock_path"])["peripherals"] if p["name"] == "AWT"), family)
        assert clock["bus_clock"] == "PCLK"
        assert clock["kernel_clock"] == {"mux": {"peripheral": "AWT", "register": "CR", "field": "SRC"}}
        for key in ["enable", "reset"]:
            assert clock[key] == {"peripheral": "SYSCTRL", **CONTROL[key]}
        assert clock["enable_active_value"] == 1 and clock["reset_asserted_value"] == 0
    for family in ["cw32f002", "cw32f003"]:
        assert load(ROOT / f"cw32-data/inputs/{family}.yaml")["register_versions"]["awt"] == "v1"
        assert not any(p.startswith(family.upper()) for p in catalog["parts"])
    assert "CW32F030" not in catalog["parts"]
    writes = load(ROOT / "cw32-data/register-writes.yaml")["registers"]
    for version in ["cw32f030_v1", "cw32l031_v1", "v1"]:
        name = "awt_" + version
        current = load(ROOT / f"cw32-data/registers/{name}.yaml")
        check_ir(current, version != "v1")
        command = one(writes[name], name)
        assert {key: command[key] for key in ["block", "register", "fieldset", "reset_value", "write_noop", "zero_to_clear_fields"]} == {
            "block": "AWT", "register": "ICR", "fieldset": "ICR", "reset_value": 63, "write_noop": 63, "zero_to_clear_fields": ["UD"]}
        if data:
            assert load(data / f"registers/{name}.json") == current
    generated_count = None
    if data:
        actual = set()
        chips = sorted((data / "chips").glob("*.json"))
        assert chips, "current generated chip data missing"
        for path in chips:
            chip = load(path)
            core = chip["cores"][0]
            sysctrl = one((p for p in core["peripherals"] if p["name"] == "SYSCTRL"), chip["name"])
            lse = sysctrl["clock_limits"].get("lse_configuration")
            awt = [p for p in core["peripherals"] if p.get("registers", {}).get("kind") == "awt"]
            if lse and awt:
                actual.add(chip["name"])
                assert chip["name"] in PARTS and len(awt) == 1
                assert lse == catalog["parts"][chip["name"]]["configuration"]
                a = awt[0]
                assert a["address"] == 0x40014C00 and a["name"] == "AWT"
                assert a["registers"] == {"kind": "awt", "version": FAMILIES[chip["line"]][2], "block": "AWT"}
                assert a["rcc_control"] == CONTROL
                assert a["interrupts"] == [{"signal": "GLOBAL", "interrupt": "AWT"}]
                assert one((i for i in core["interrupts"] if i["name"] == "AWT"), chip["name"])["number"] == 30
        assert actual == active_names(PARTS)
        generated_count = len(chips)
    return generated_count


def check_declarations(proof):
    example = proof["example"]
    assert example["manifest"] == "examples/shared/low-power-timers/Cargo.toml"
    manifest = tomllib.loads((ROOT / example["manifest"]).read_text())
    assert manifest["package"]["name"] == example["package"] == "cw32-low-power-timer-examples"
    assert example["marker_feature"] == "lse-awt" and manifest["features"]["lse-awt"] == []
    assert "lse-awt" not in manifest["features"]["default"]
    assert example["bin"] == "lse_events"
    binary = one((b for b in manifest["bin"] if b["name"] == "lse_events"), "LSE example bin")
    assert binary["path"] == "src/bin/lse_events.rs" and binary["required-features"] == ["lse-awt"]
    assert set(example["features"]) == {"cw32f020c6u7", "cw32f030c8t7", "cw32a030c8t7", "cw32l031c8t6", "cw32r031c8u6", "cw32w031r8u6"}
    for feature in active_names(example["features"]):
        assert set(manifest["features"][feature]) == {"embassy-cw32/" + feature, "cw32-metapac/" + feature}
    assert not {name.lower() for name in retired_targets()} & set(manifest["features"])
    capability = load(ROOT / "tools/hal-capabilities.yaml")["functions"]["awt"]
    mode = one((m for m in capability["modes"] if m["id"] == "bounded_api"), "AWT bounded mode")
    assert mode["status"] == "partial" and "LSE" in mode["scope"] and "HSIOSC" in mode["scope"]
    assert set(re.findall(r"\bCW32[A-Z0-9]+\b", mode["scope"])) == active_names(PARTS)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sources", type=Path, required=True)
    parser.add_argument("--data", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    proof = load(ROOT / EVIDENCE)
    assert proof["schema_version"] == 1 and proof["source_authority"] == "sources/evidence-sources.json"
    assert proof["hardware_validated"] is False
    contract = proof["register_contract"]
    assert contract == {
        "address": 0x40014C00, "source": 3, "mode": 3, "prescaler_encodings": list(range(1, 16)),
        "registers": {name: {"offset": offset, "access": access} for name, (offset, access) in REGISTERS.items()},
        "fields": FIELDS, "icr_write_noop": 63, "icr_zero_to_clear_fields": ["UD"],
        "irq_name": "AWT", "irq_number": 30, "rcc_control": CONTROL,
    }
    original_count, page_count = check_sources(proof, args.sources)
    generated_count = check_data(proof, args.data)
    check_declarations(proof)
    result = {"status": "passed", "evidence": EVIDENCE, "evidence_sha256": digest((ROOT / EVIDENCE).read_bytes()),
              "source_originals": original_count, "page_hashes": page_count, "qualified_parts": sorted(PARTS),
              "register_versions": ["awt_cw32f030_v1", "awt_cw32l031_v1"],
              "excluded_controls": ["CW32F002F3P7", "CW32F003E4P7", "CW32F030"],
              "generated_selections_checked": generated_count, "hal_execution": False, "hardware_execution": False}
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
