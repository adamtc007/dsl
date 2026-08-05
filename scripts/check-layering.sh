#!/usr/bin/env bash
# check-layering.sh — layering guard for the dsl library.
# Run from the repo root (~/dev/dsl/).
#
# Rule: dsl-core (parser, compiler, IR) must NOT reference any SemOS layer.
# The design test: dsl-core compiles with sem-os entirely absent.
set -uo pipefail

fail=0
note() { printf '  \033[31mFORBIDDEN EDGE\033[0m  %s\n' "$1"; fail=1; }

DSL_LANG_SRC="crates/dsl-core/src"
SHARED_SRC=(
  "crates/dsl-core/src"
  "crates/dsl_types/src"
  "crates/sem_os_core/src"
  "crates/sem_os_ontology/src"
  "crates/sem_os_policy/src"
  "crates/sem_os_types/src"
  "crates/semantic-decision-contracts/src"
)

echo "== dsl layering guard =="

if [ -d "$DSL_LANG_SRC" ]; then
  hits="$(grep -rnE 'sem_os_core|sem_os_ontology|sem_os_policy|sem_os_types' \
    "$DSL_LANG_SRC" 2>/dev/null \
    | awk '{ code=$0; sub(/^[^:]*:[0-9]+:/, "", code); sub(/\/\/.*/, "", code); if (code ~ /sem_os_(core|ontology|policy|types)/) print $0 }' || true)"
  [ -n "$hits" ] && note "dsl-core references SemOS — layering violation:
$hits"
fi

host_imports="$(rg -n '(^|[[:space:]])(use|extern[[:space:]]+crate)[[:space:]]+(bpmn_lite|designer_graph|utterance_engine|ob_poc)' \
  "${SHARED_SRC[@]}" 2>/dev/null || true)"
[ -n "$host_imports" ] && note "shared DSL/SemOS crates import a host/application crate:
$host_imports"

if [ "$fail" -eq 0 ]; then
  echo "  OK — dsl-core is free of SemOS dependencies and shared crates are host-agnostic."
else
  echo ""
  echo "== Layering guard FAILED =="
fi
exit "$fail"
