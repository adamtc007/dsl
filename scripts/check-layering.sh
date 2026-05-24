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

echo "== dsl layering guard =="

if [ -d "$DSL_LANG_SRC" ]; then
  hits="$(grep -rnE 'sem_os_core|sem_os_ontology|sem_os_policy|sem_os_types' \
    "$DSL_LANG_SRC" 2>/dev/null \
    | grep -vE ':[0-9]+:[[:space:]]*//' || true)"
  [ -n "$hits" ] && note "dsl-core references SemOS — layering violation:
$hits"
fi

if [ "$fail" -eq 0 ]; then
  echo "  OK — dsl-core is free of SemOS dependencies."
else
  echo ""
  echo "== Layering guard FAILED =="
fi
exit "$fail"
