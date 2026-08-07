#!/usr/bin/env python3
"""Permanent API, dependency, facade and fuzz-regression gate for gameboard contracts."""

import hashlib
import json
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASELINE = ROOT / "scripts/baselines/semantic-gameboard-contracts-public-api-v1.json"
FIXTURES = ROOT / "scripts/fixtures/semantic_gameboard_contracts"
CRATE = ROOT / "crates/semantic-decision-contracts"
FUZZ = CRATE / "fuzz"


def run(command, *, cwd=ROOT, check=True):
    return subprocess.run(
        command,
        cwd=cwd,
        check=check,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def api_surface(package, extra):
    output = run(
        ["cargo", "public-api", "-p", package, "-sss", *extra]
    ).stdout.encode()
    return {"items": len(output.splitlines()), "sha256": hashlib.sha256(output).hexdigest()}


def check_public_api():
    baseline = json.loads(BASELINE.read_text())
    observed = {
        "default": api_surface("semantic-decision-contracts", []),
        "all_features": api_surface("semantic-decision-contracts", ["--all-features"]),
    }
    if observed != baseline["surfaces"]:
        raise SystemExit(
            "shared public API drift; name the real consumer, facade, stability contract "
            f"and reason before updating the snapshot:\nexpected={baseline['surfaces']}\n"
            f"observed={observed}"
        )
    if observed["default"] != observed["all_features"]:
        raise SystemExit("test/tooling features changed the production public surface")
    pack_observed = {
        "default": api_surface("semantic-pack", []),
        "all_features": api_surface("semantic-pack", ["--all-features"]),
    }
    if pack_observed != baseline["semantic_pack_surfaces"]:
        raise SystemExit(
            "semantic-pack public API drift; name the real consumer, facade, stability "
            f"contract and reason before updating the snapshot:\nexpected={baseline['semantic_pack_surfaces']}\n"
            f"observed={pack_observed}"
        )
    if pack_observed["default"] != pack_observed["all_features"]:
        raise SystemExit("semantic-pack tooling features changed its production public surface")
    return {"semantic_decision_contracts": observed, "semantic_pack": pack_observed}


def check_facade_shape():
    root = (CRATE / "src/lib.rs").read_text()
    if re.search(r"^pub\s+mod\s+", root, re.MULTILINE):
        raise SystemExit("public implementation module refused")
    for source in (CRATE / "src").rglob("*.rs"):
        text = source.read_text()
        if re.search(r"pub\s+use\s+[^;]*::\s*\*\s*;", text):
            raise SystemExit(f"glob public re-export refused: {source.relative_to(ROOT)}")
    gameboard = (CRATE / "src/gameboard.rs").read_text().lower()
    for forbidden in ("bpmn", "ob-poc", "ob_poc"):
        if forbidden in gameboard:
            raise SystemExit(f"application vocabulary '{forbidden}' entered generic gameboard code")


def check_dependencies():
    metadata = json.loads(
        run(["cargo", "metadata", "--locked", "--format-version", "1", "--no-deps"]).stdout
    )
    package = next(
        package
        for package in metadata["packages"]
        if package["name"] == "semantic-decision-contracts"
    )
    normal = {
        dependency["name"]
        for dependency in package["dependencies"]
        if dependency.get("kind") in (None, "normal")
    }
    allowed = {"hex", "serde", "serde_json", "sha2", "thiserror"}
    if normal != allowed:
        raise SystemExit(f"shared-contract dependency direction drift: {sorted(normal)}")


def compile_fixture(source_name, should_pass):
    with tempfile.TemporaryDirectory(prefix="semantic-gameboard-contract-") as directory:
        root = Path(directory)
        (root / "src").mkdir()
        shutil.copyfile(FIXTURES / source_name, root / "src/main.rs")
        (root / "Cargo.toml").write_text(
            "[package]\nname='contract-consumer'\nversion='0.0.0'\nedition='2021'\n\n"
            "[workspace]\n\n[dependencies]\n"
            f"semantic-decision-contracts={{path={json.dumps(str(CRATE))}}}\n"
        )
        result = run(["cargo", "check", "--quiet"], cwd=root, check=False)
        if should_pass and result.returncode != 0:
            raise SystemExit(f"facade consumer failed:\n{result.stderr}")
        if not should_pass and result.returncode == 0:
            raise SystemExit(f"compile-fail fixture passed: {source_name}")
        if not should_pass and "private" not in result.stderr:
            raise SystemExit(f"fixture failed for the wrong reason:\n{result.stderr}")


def check_fuzz_inventory_and_regressions():
    expected = sorted(json.loads((FUZZ / "regressions/manifest.json").read_text())["targets"])
    observed = sorted(run(["cargo", "+nightly", "fuzz", "list"], cwd=FUZZ).stdout.splitlines())
    if observed != expected:
        raise SystemExit(f"fuzz target/receipt drift: expected={expected}, observed={observed}")
    manifest = json.loads((FUZZ / "regressions/manifest.json").read_text())
    for target, regressions in manifest["targets"].items():
        for regression in regressions:
            packet = FUZZ / "regressions" / regression["file"]
            if not packet.is_file() or packet.stat().st_size == 0:
                raise SystemExit(f"missing or empty regression packet: {packet}")
            run(
                ["cargo", "+nightly", "fuzz", "run", target, str(packet), "--", "-runs=1"],
                cwd=FUZZ,
            )


def main():
    observed = check_public_api()
    check_facade_shape()
    check_dependencies()
    compile_fixture("facade_consumer.rs", True)
    compile_fixture("internal_module_import.rs", False)
    compile_fixture("unchecked_constructor.rs", False)
    check_fuzz_inventory_and_regressions()
    print(json.dumps({"status": "pass", "surfaces": observed}, sort_keys=True))


if __name__ == "__main__":
    main()
