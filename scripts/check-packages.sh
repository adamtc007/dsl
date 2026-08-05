#!/usr/bin/env bash
# Package the release graph as one unit and dry-run the first publishable tier.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

leaf_packages=(dsl_types sem_os_types semantic-decision-contracts)
dirty=()
if [ -n "$(git status --porcelain)" ]; then
  dirty=(--allow-dirty)
fi

publish_dry_run_leaf() {
  cargo publish --locked -p "$1" --dry-run "${dirty[@]}" >/dev/null
}

echo "== package guard =="
cargo package --workspace --exclude dsl-integration-tests --locked --no-verify "${dirty[@]}" >/dev/null
echo "  packaged publishable workspace graph"

for package in "${leaf_packages[@]}"; do
  publish_dry_run_leaf "$package"
  echo "  publish dry-run $package"
done

echo "  OK — package contents and leaf publish dry runs are valid."
