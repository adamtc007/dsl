#!/usr/bin/env python3
"""Write and validate one independently budgeted Gameboard fuzz receipt."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


ATTEMPT_OUTCOMES = {
    "applied",
    "incomplete",
    "ambiguous",
    "inapplicable",
    "disclosure_safe_refusal",
    "stale",
    "compiler_refused",
    "rejected_by_user",
    "corrected",
    "system_failure",
}
DISCLOSURE_CLASSES = {
    "public",
    "authenticated",
    "restricted",
    "policy_hidden",
    "technical",
}
COUNTER_PATTERN = re.compile(r"^semantic-counter ([a-z_]+)=([a-z_]+)$", re.MULTILINE)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target", required=True)
    parser.add_argument("--log", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--exit-code", required=True, type=int)
    parser.add_argument("--runs-budget", required=True, type=int)
    parser.add_argument("--seconds-budget", required=True, type=int)
    parser.add_argument("--max-len", required=True, type=int)
    parser.add_argument("--started-at", required=True)
    parser.add_argument("--finished-at", required=True)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    observed: dict[str, set[str]] = {}
    for category, label in COUNTER_PATTERN.findall(args.log.read_text(errors="replace")):
        observed.setdefault(category, set()).add(label)

    required: dict[str, set[str]] = {}
    if args.target == "attempt_receipt_contract":
        required["attempt_outcome"] = ATTEMPT_OUTCOMES
    if args.target == "rule_explanation_contract":
        required["disclosure_class"] = DISCLOSURE_CLASSES

    missing = {
        category: sorted(labels - observed.get(category, set()))
        for category, labels in required.items()
        if labels - observed.get(category, set())
    }
    receipt = {
        "schema_version": 1,
        "target": args.target,
        "started_at": args.started_at,
        "finished_at": args.finished_at,
        "runs_budget": args.runs_budget,
        "seconds_budget": args.seconds_budget,
        "max_len": args.max_len,
        "exit_code": args.exit_code,
        "semantic_counters": {
            category: sorted(labels) for category, labels in sorted(observed.items())
        },
        "required_semantic_counters_complete": not missing,
        "missing_semantic_counters": missing,
    }
    args.output.write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    if missing:
        raise SystemExit(f"missing semantic counters for {args.target}: {missing}")


if __name__ == "__main__":
    main()
