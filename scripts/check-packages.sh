#!/usr/bin/env bash
# Package the release graph as one unit and dry-run the first publishable tier.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

# A crates.io dry run resolves dependencies against the live index, so only
# packages whose workspace dependencies are already published can be verified
# before this release exists. Higher tiers are still packaged together above
# and are dry-run publishable after these leaves are released.
leaf_packages=(dsl_types sem_os_types semantic-decision-contracts)
dirty_flag=""
if [ -n "$(git status --porcelain)" ]; then
  dirty_flag="--allow-dirty"
fi

publish_dry_run_leaf() {
  cargo publish --locked -p "$1" --dry-run ${dirty_flag:+"$dirty_flag"} >/dev/null
}

echo "== package guard =="
cargo package --workspace --exclude dsl-integration-tests --locked --no-verify ${dirty_flag:+"$dirty_flag"} >/dev/null
echo "  packaged publishable workspace graph"

for package in "${leaf_packages[@]}"; do
  publish_dry_run_leaf "$package"
  echo "  publish dry-run $package"
done

echo "  OK — package contents and leaf publish dry runs are valid."
