#!/bin/sh
# Fetch Horizon, merge upstream/main on a throwaway branch and run the gates.
# It never pushes and never merges into dev: a human resolves conflicts and reviews the result.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"

git remote get-url upstream >/dev/null 2>&1 || {
  echo "No upstream remote. Add it: git remote add upstream https://github.com/horizon-rust/horizon.git" >&2
  exit 1
}

git fetch upstream --prune
behind=$(git rev-list --count HEAD..upstream/main)
echo "upstream/main is $behind commit(s) ahead of $(git rev-parse --short HEAD)"
[ "$behind" -eq 0 ] && { echo "Already current."; exit 0; }
if [ "$behind" -gt 200 ] && [ "${SCORIA_SYNC_APPLY:-0}" != "1" ]; then
  echo "Large sync ($behind commits). Read docs/fork/patches.md, then re-run with SCORIA_SYNC_APPLY=1."
  exit 0
fi

echo "Checking that the engine patches still apply to upstream/main…"
if ! sh tools/check_upstream_patches.sh; then
  echo "The fork patches do not apply cleanly: rebase them first (docs/fork/patches.md)." >&2
  exit 1
fi

branch="sync/upstream-$(date +%Y%m%d-%H%M)"
git switch -c "$branch"

if ! git merge --no-edit upstream/main; then
  cat <<'MSG' >&2

Conflicts are expected in launcher/ and occasionally in crates/. Resolve them in this branch:
  - crates/: prefer upstream, then re-apply the small typed_text patch (docs/fork/patches.md).
  - launcher/: keep Scoria's offline/piston/Modrinth behaviour.
Then run the gates and merge into dev.
MSG
  exit 1
fi

echo "Merged; checking the Scoria-only surfaces…"
sh tools/check_fork_surfaces.sh

echo "Running the gates…"
(cd launcher && cargo test)
cargo check -p horizon-client
cargo test -p horizon-client --bin horizon-client cyrillic 2>/dev/null || true
(cd launcher-ui && bun run typecheck && bun run check:ui && bun run check:i18n)

cat <<MSG
Sync branch $branch is ready.
Review the diff, update docs/fork/patches.md and PROJECT_STATE.md, then merge into dev.
Nothing was pushed.
MSG
