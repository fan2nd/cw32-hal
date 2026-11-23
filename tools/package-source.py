#!/usr/bin/env python3
"""Create a deterministic source-only checkpoint that can bootstrap its own PAC."""
from __future__ import annotations

import argparse
from fnmatch import fnmatchcase
import hashlib
import json
import os
from pathlib import Path
import sys
import subprocess
import zipfile


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "cw32-data/tools"))
from source_provenance import APPROVED_SDK_DOCS, APPROVED_SDK_ROOT, approved_sdk_members, validate_distribution

SOURCE_LOCK = "sources/evidence-sources.json"
# Authored scope survives packaging; generated coverage under build/ does not.
SOURCE_DECLARATIONS = {"tools/hal-capabilities.yaml"}
SOURCE_MANIFESTS = {SOURCE_LOCK, "sources/catalog.json", "sources/layout-history.json", "sources/README.md",
                    "sources/SOURCES.md", "sources/REFERENCE-PACKAGE.md",
                    "sources/x030-f020-hsi-pll-source-receipt.json"} | APPROVED_SDK_DOCS
EXCLUDED_ROOTS = {".cargo", ".rustup", "build", ".git", "target", "cw32-metapac"}
EXCLUDED_DIRECTORIES = {"target", "__pycache__", ".git", ".venv", "verification-logs"}
EXCLUDED_FILES = {
    "cw32-data/coverage.json", "cw32-data/source-audit.json",
    "cw32-data/source-lock.json", "cw32-data/vendor-sources.json",
    "cw32-data/reference-index.json", "cw32-data/SOURCE-CATALOG.md",
    "cw32-data/evidence-sources.json", "docs/generated-data-parity.json",
    "docs/schema-reimplementation.json",
}
REPORT_PATTERNS = (
    "*coverage*.json", "*verification*.json", "*report*.json", "*manifest*.json",
    "*migration*.json", "*refactor*.json", "*equivalence*.json", "*layout.json",
    "*review.json", "*rebase.json",
)
BINARY_SUFFIXES = {".pyc", ".zip", ".elf", ".bin", ".hex", ".map", ".o", ".a", ".log"}


def required_evidence(root: Path) -> set[str]:
    """Assert the source lock's evidence references survive source-only packaging."""
    lock = json.loads((root / SOURCE_LOCK).read_text())
    required = SOURCE_MANIFESTS | SOURCE_DECLARATIONS | set(approved_sdk_members(lock))
    required.add("docs/f020-x030-hsi-pll-independent-review.json")
    required.add("docs/l083-lse-sysclk-metadata-review.json")
    required.add("docs/l083-lse-sysclk-runtime-review.json")
    required.add("docs/l010-lse-sysclk-metadata-review.json")
    required.add("docs/l010-lse-sysclk-runtime-review.json")
    required.add("docs/l011-lse-sysclk-metadata-review.json")
    required.add("docs/l011-lse-sysclk-runtime-review.json")
    required.add("docs/l012-lse-sysclk-metadata-review.json")
    required.add("docs/l012-lse-sysclk-runtime-review.json")
    required.add("docs/f002-lsi-option-consumer-supplement.json")
    required.add("docs/f002-factory-lsi-runtime-review.json")
    required.add("docs/f003-factory-lsi-runtime-review.json")
    required.add("docs/l031-factory-lsi-runtime-review.json")
    required.add("docs/l031-factory-lsi-metadata-review.json")
    required.add("docs/r031-factory-lsi-runtime-review.json")
    required.add("docs/r031-factory-lsi-metadata-review.json")
    required.add("docs/w031-factory-lsi-runtime-review.json")
    required.add("docs/w031-factory-lsi-metadata-review.json")
    required.add("docs/l052-factory-lsi-runtime-review.json")
    required.add("docs/l052-factory-lsi-metadata-review.json")
    required.add("docs/l083-factory-lsi-runtime-review.json")
    required.add("docs/l083-factory-lsi-metadata-review.json")
    required.add("docs/lptim-fractional-delay-review.json")
    required.add("docs/native-lse-flash-wait-review.json")
    required.add("docs/lse-l010-independent-runtime-review.json")
    required.add("docs/lse-native-low-power-independent-runtime-review.json")
    required.add("docs/lse-native-low-power-final-correspondence.json")
    required.add("docs/lse-native-low-power-independent-runtime-review.md")
    required.add("docs/hse-pll-runtime-acceptance.json")
    required.add("docs/hse-pll-runtime-acceptance.md")
    required.add("docs/hse-pll-final-correspondence.json")
    required.add("docs/hse-pll-final-correspondence.md")
    required.add("docs/uart-buffered-rx-runtime-review.json")
    required.add("docs/four-channel-dma-runtime-review.json")
    required.add("docs/l012-dma-runtime-review.json")
    required.add("docs/classic-i2c-async-runtime-review.json")
    required.add("docs/l012-i2c-async-runtime-review.json")
    required.add("docs/spi-async-pio-runtime-review.json")
    required.add("docs/spi-async-pio-metadata-review.json")
    required.add("docs/timer-event-input-runtime-review.json")
    required.add("docs/timer-event-input-metadata-review.json")
    for audit in lock.get("project_audit_inputs", []):
        path = root / audit["path"]
        if path.stat().st_size != audit["bytes"] or hashlib.sha256(path.read_bytes()).hexdigest() != audit["sha256"]:
            raise ValueError("Immutable project audit input changed: " + audit["path"])
        required.add(audit["path"])
    for artifact in lock["artifacts"]:
        acquisition = artifact.get("provenance", {}).get("acquisition_evidence")
        if acquisition:
            required.add(acquisition.split("#")[0])
        specs = [artifact, *artifact.get("members", [])]
        if "text" in artifact:
            specs.append(artifact["text"])
        for spec in specs:
            required.update(ref.split("#")[0] for ref in spec.get("evidence", []))
    # These acceptance records are authored provenance, even when historical
    # names include "review" or "verification". Preserve the full review chain.
    def review_references(value):
        if isinstance(value, dict):
            for key, child in value.items():
                if key in {"review_reference", "verification_reference"} and isinstance(child, str):
                    required.add(child.split("#")[0])
                review_references(child)
        elif isinstance(value, list):
            for child in value:
                review_references(child)
    review = json.loads((root / "docs/upstream-file-provenance.json").read_text())
    for row in review["files"]:
        review_references(row.get("replacement_review", {}))
    required.add("docs/adc-l012-adc2-async-runtime-review.json")
    required.add("docs/adc-l012-adc2-async-source-review.json")
    required.add("docs/classic-uart-dma-rx-runtime-review.json")
    required.add("docs/classic-uart-dma-rx-source-review.json")
    required.add("docs/classic-spi-dma-runtime-review.json")
    required.add("docs/classic-spi-dma-source-review.json")
    required.add("docs/l012-spi-dma-runtime-review.json")
    required.add("docs/l012-spi-dma-source-review.json")
    return required


def include_file(relative: Path, required: set[str]) -> bool:
    name = relative.as_posix()
    if relative.parts[0] in EXCLUDED_ROOTS:
        return False
    if any(part in EXCLUDED_DIRECTORIES for part in relative.parts):
        return False
    if name.startswith("cw32-data/data/") or name in EXCLUDED_FILES:
        return False
    if relative.parts[0] == "sources" and name not in SOURCE_MANIFESTS and not (
            name.startswith(APPROVED_SDK_ROOT + "/") and name in required):
        return False
    if relative.suffix in BINARY_SUFFIXES or name.endswith(".zip.sha256"):
        return False
    if name in required:
        return True
    if relative.parent == Path("docs") and any(fnmatchcase(relative.name, pattern) for pattern in REPORT_PATTERNS):
        return False
    return True


def collect_files(root: Path) -> list[tuple[Path, Path]]:
    required = required_evidence(root)
    files = []
    for directory, directories, names in os.walk(root):
        base = Path(directory)
        relative_directory = base.relative_to(root)
        directories[:] = sorted(name for name in directories
                                if not (base / name).is_symlink()
                                and name not in EXCLUDED_DIRECTORIES
                                and not (relative_directory == Path(".") and name in EXCLUDED_ROOTS)
                                and not (relative_directory == Path("cw32-data") and name == "data")
                                and not (relative_directory == Path("sources") and name != "approved-sdk-members"))
        for name in sorted(names):
            path = base / name
            relative = path.relative_to(root)
            if path.is_file() and not path.is_symlink() and include_file(relative, required):
                files.append((relative, path))
    included = {relative.as_posix() for relative, _ in files}
    missing = required - included
    if missing:
        raise ValueError("Source bundle would omit required evidence: " + ", ".join(sorted(missing)))
    return files


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    subprocess.run([sys.executable, str(ROOT / "tests/test_module_layout.py")], cwd=ROOT, stdout=sys.stderr, check=True)
    files = collect_files(ROOT)
    validate_distribution(ROOT, files)
    assert any(str(relative) == "examples/f030/basic/.cargo/config.toml" for relative, _ in files)
    assert any(str(relative) == "firmware/Cargo.toml" for relative, _ in files)
    assert any(str(relative) == "firmware/Cargo.lock" for relative, _ in files)
    assert any(str(relative) == "cw32-metapac-gen/res/Cargo.toml" for relative, _ in files)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(args.output, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for relative, path in sorted(files):
            info = zipfile.ZipInfo("embassy-cw32/" + relative.as_posix(), date_time=(2026, 10, 8, 0, 0, 0))
            info.external_attr = ((0o755 if path.stat().st_mode & 0o111 else 0o644) | 0o100000) << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, path.read_bytes())
    digest = hashlib.sha256(args.output.read_bytes()).hexdigest()
    args.output.with_suffix(args.output.suffix + ".sha256").write_text(f"{digest}  {args.output.name}\n")
    print(json.dumps({"archive": str(args.output), "files": len(files),
                      "bytes": args.output.stat().st_size, "sha256": digest}))


if __name__ == "__main__":
    main()
