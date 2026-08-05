#!/usr/bin/env bash
# Metadata-backed workspace dependency and host-boundary validation.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata="$(mktemp)"
trap 'rm -f "$metadata"' EXIT

cd "$root"
cargo metadata --locked --format-version 1 >"$metadata"

fail=0
note() {
  printf '  DEPENDENCY VIOLATION  %s\n' "$1"
  fail=1
}

assert_allowed_workspace_dependencies() {
  local package="$1"
  shift
  local allowed=" $* "
  local dependency

  while IFS= read -r dependency; do
    [ -z "$dependency" ] && continue
    if [[ "$allowed" != *" $dependency "* ]]; then
      note "$package has forbidden production workspace dependency: $dependency"
    fi
  done < <(
    jq -r --arg package "$package" '
      .packages[]
      | select(.name == $package)
      | .dependencies[]
      | select(.path != null and .kind != "dev")
      | .name
    ' "$metadata" | LC_ALL=C sort -u
  )
}

assert_no_host_sources() {
  local hit
  hit="$(
    jq -r '
      .workspace_members as $members
      | .packages[]
      | select(.id as $id | $members | index($id))
      | .dependencies[]
      | select(
          ((.source // "") | test("adamtc007/(ob-poc|bpmn-lite)"; "i"))
          or ((.path // "") | test("/(ob-poc|bpmn-lite)(/|$)"; "i"))
        )
      | "\(.name): \(.source // .path)"
    ' "$metadata" 2>/dev/null || true
  )"
  if [ -n "$hit" ]; then
    note "shared packages reference a host source:\n$hit"
  fi
}

echo "== metadata dependency guard =="
assert_allowed_workspace_dependencies dsl_types
assert_allowed_workspace_dependencies sem_os_types
assert_allowed_workspace_dependencies semantic-decision-contracts
assert_allowed_workspace_dependencies semantic-pack dsl_types semantic-decision-contracts
assert_allowed_workspace_dependencies semantic-embedder
assert_allowed_workspace_dependencies dsl-core dsl_types
assert_allowed_workspace_dependencies sem_os_core dsl-core dsl_types sem_os_types
assert_allowed_workspace_dependencies sem_os_ontology dsl_types sem_os_types semantic-decision-contracts
assert_allowed_workspace_dependencies sem_os_policy dsl_types sem_os_core sem_os_ontology sem_os_types semantic-decision-contracts semantic-pack
assert_no_host_sources

if [ "$fail" -ne 0 ]; then
  echo "== metadata dependency guard FAILED =="
  exit 1
fi

echo "  OK — workspace dependency direction and host boundary are valid."
