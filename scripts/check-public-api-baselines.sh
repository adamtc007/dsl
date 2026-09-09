#!/usr/bin/env bash
# Public-API baseline gate for crates with a recorded baseline under
# scripts/baselines/<crate>-public-api-v1.txt.
#
# A baseline is refreshed deliberately, in the same change that alters the
# surface, by running this script with --update and reviewing the diff.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

crates=(dsl-diagnostics dsl-parser dsl-atoms dsl-ast sem-os-id sem-os-append-store)
update=0
if [ "${1:-}" = "--update" ]; then
  update=1
fi

fail=0
echo "== public-api baseline guard =="
for crate in "${crates[@]}"; do
  baseline="scripts/baselines/${crate}-public-api-v1.txt"
  actual="$(mktemp)"
  cargo public-api -p "$crate" --all-features --simplified >"$actual" 2>/dev/null
  if [ "$update" -eq 1 ]; then
    cp "$actual" "$baseline"
    echo "  updated $baseline"
  elif ! diff -u "$baseline" "$actual" >/dev/null; then
    echo "  PUBLIC API DRIFT — $crate differs from $baseline:"
    diff -u "$baseline" "$actual" | sed 's/^/    /' || true
    fail=1
  else
    echo "  ok $crate"
  fi
  rm -f "$actual"
done

if [ "$fail" -ne 0 ]; then
  echo "== public-api baseline guard FAILED =="
  exit 1
fi
echo "  OK — recorded public API baselines are unchanged."
