#!/usr/bin/env bash
# Prevent new host/domain vocabulary from entering shared production files.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
allowlist="$root/.ci/domain-token-allowlist.txt"
actual="$(mktemp)"
allowed="$(mktemp)"
new_hits="$(mktemp)"
stale="$(mktemp)"
trap 'rm -f "$actual" "$allowed" "$new_hits" "$stale"' EXIT

cd "$root"

pattern='(ob[-_ ]?poc|\bcbu\b|\bkyc\b|\bmandate\b|\btollgate\b|client[-_ ]?group|\bmic\b|\bbic\b|\bpricing\b|\bsteward(ship)?\b|\bcompliance\b|regulatory officer|semantic-os:ob-poc)'

rg -il --glob '**/src/**/*.rs' "$pattern" crates | LC_ALL=C sort -u >"$actual" || true
sed -e '/^[[:space:]]*#/d' -e '/^[[:space:]]*$/d' "$allowlist" | LC_ALL=C sort -u >"$allowed"

comm -23 "$actual" "$allowed" >"$new_hits"
comm -13 "$actual" "$allowed" >"$stale"

echo "== shared domain-vocabulary guard =="
if [ -s "$new_hits" ]; then
  echo "  DOMAIN VIOLATION — new production files contain host/domain tokens:"
  sed 's/^/    /' "$new_hits"
  exit 1
fi

if [ -s "$stale" ]; then
  echo "  ALLOWLIST DEBT REDUCED — remove these stale entries and review the change:"
  sed 's/^/    /' "$stale"
  exit 1
fi

echo "  OK — no new host/domain vocabulary files; reviewed debt is unchanged."
