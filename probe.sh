#!/usr/bin/env bash
# probe.sh
# Run from ~/dev/dsl — tests whether the §1D "0 external refs" demotions are sound.
# Demotes representative symbols ONE AT A TIME, rebuilds all targets in BOTH repos,
# records pass/fail, restores after each. Nothing is committed.
#
# --all-targets is the load-bearing flag: it compiles tests/, benches, examples
# (separate compilation units that see the crate externally), which is exactly
# what a sem-os-only `rg` search misses. Also builds sem-os to catch consumption
# through the sem_os_ontology glob re-export shim.

set -u
DSL=~/dev/dsl
SEMOS=~/dev/sem-os

cd "$DSL" || { echo "cannot cd to $DSL"; exit 1; }
git rev-parse --git-dir >/dev/null 2>&1 || { echo "$DSL is not a git repo"; exit 1; }

echo "baseline branch: $(git rev-parse --abbrev-ref HEAD)"
git stash -u -m "e7-probe-stash" >/dev/null 2>&1

# file | symbol  — representative sample across the risk classes
PROBES=(
  "crates/dsl_types/src/constellation_map_def.rs|VerbAvailability"   # cross-crate substrate + glob shim
  "crates/dsl-core/src/config/loader.rs|load_dag_registry"           # likely intra-crate cross-module use
  "crates/dsl-core/src/config/dag_registry.rs|DagRegistry"           # type used across config modules?
  "crates/dsl-core/src/config/manifest.rs|build_manifest"            # candidate truly dead
  "crates/dsl-core/src/resolver/manifest.rs|SlotManifestRow"         # §1B says 'used in dsl-core tests'
)

probe() {
  local file="$1" sym="$2" line dslres semres
  line=$(grep -nE "pub (struct|enum|fn|type|trait|const|static) ${sym}\b" "$file" | head -1 | cut -d: -f1)
  if [ -z "$line" ]; then
    echo "  SKIP $sym ($file) — pub decl not found by pattern"
    return
  fi
  # demote only that line: pub X -> pub(crate) X
  sed -i.bak "${line}s/\bpub /pub(crate) /" "$file"
  ( cd "$DSL"   && cargo build --workspace --all-targets ) >/tmp/dslbuild.log 2>&1; dslres=$?
  ( cd "$SEMOS" && cargo build --workspace --all-targets ) >/tmp/sembuild.log 2>&1; semres=$?
  if [ "$dslres" -eq 0 ] && [ "$semres" -eq 0 ]; then
    echo "  PASS  $sym — pub(crate) builds clean in both repos (demotion SOUND)"
  else
    echo "  FAIL  $sym — demotion breaks build:"
    [ "$dslres" -ne 0 ] && grep -E "error\[E0603\]|is private|private" /tmp/dslbuild.log | head -3 | sed 's/^/        dsl:    /'
    [ "$semres" -ne 0 ] && grep -E "error\[E0603\]|is private|private" /tmp/sembuild.log | head -3 | sed 's/^/        sem-os: /'
  fi
  mv "${file}.bak" "$file"
}

echo "=== §1D demotion soundness probe (pub -> pub(crate), one at a time) ==="
for p in "${PROBES[@]}"; do
  probe "${p%%|*}" "${p##*|}"
done

git stash pop >/dev/null 2>&1
echo "=== done — working tree restored ==="
