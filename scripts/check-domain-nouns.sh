#!/usr/bin/env bash
# Domain-noun gate for the shared core crates (EOP-PLAN-CA-REUSE-001 DSL-T2).
#
# Rule: no domain noun (kyc, cbu, onboarding, ca, ubo) in any generic crate's
# non-test code — types, enums, strings, defaults or docs. Test modules
# (`#[cfg(test)] mod …`), `tests/`, `integration_tests/` and `fuzz/` trees are
# out of scope; fixtures may name a domain.
#
# The allowlist at .ci/domain-noun-allowlist.txt is read for completeness and
# is expected to be empty. A non-empty allowlist is itself a failure: there is
# no reviewed-debt tier for this rule.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

allowlist=".ci/domain-noun-allowlist.txt"
echo "== shared domain-noun guard =="

if [ -f "$allowlist" ] && grep -vE '^[[:space:]]*(#|$)' "$allowlist" >/dev/null; then
  echo "  DOMAIN NOUN ALLOWLIST IS NOT EMPTY — $allowlist must stay empty:"
  grep -vE '^[[:space:]]*(#|$)' "$allowlist" | sed 's/^/    /'
  exit 1
fi

python3 - <<'PY'
import os, re, sys

# `OnBoard` (a move on the board, semantic-decision-contracts) is not
# "onboard"; the case-mixed spellings below deliberately exclude it.
pattern = re.compile(
    r"(?i:kyc)|(?i:cbu)|\bonboard(ing)?\b|\bOnboard(ing)?\b|\bONBOARD(ING)?\b"
    r"|(?i:\bca\b)|\bubo\b|\bUbo\b|\bUBO\b"
)
test_module = re.compile(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\b")
skip_dirs = ("/target", "/tests", "integration_tests", "/fuzz")

hits = []
for crate in sorted(os.listdir("crates")):
    src_root = os.path.join("crates", crate, "src")
    if not os.path.isdir(src_root):
        continue
    for root, _, files in os.walk(src_root):
        if any(part in root for part in skip_dirs):
            continue
        for name in sorted(files):
            if not name.endswith(".rs"):
                continue
            path = os.path.join(root, name)
            with open(path, encoding="utf-8") as handle:
                source = handle.read()
            cut = test_module.search(source)
            body = source if cut is None else source[: cut.start()]
            for number, line in enumerate(body.splitlines(), 1):
                if pattern.search(line):
                    hits.append(f"{path}:{number}: {line.strip()}")

if hits:
    print("  DOMAIN NOUN VIOLATION — non-test core code names a domain:")
    for hit in hits:
        print(f"    {hit}")
    sys.exit(1)
print("  OK — no domain nouns in non-test core code; allowlist is empty.")
PY
