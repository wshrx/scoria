#!/bin/sh
# Dry-runs the engine patch series on the newest upstream/main. A conflict here is the signal that
# the next sync needs a manual rebase — run it weekly, before upstream drifts further.
# Nothing is merged and no branch is touched: the patches are applied in a temporary worktree.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
ref=${UPSTREAM_REF:-upstream/main}

git rev-parse --verify --quiet "$ref" >/dev/null ||
  { echo "no $ref; run: git fetch upstream" >&2; exit 2; }

patches=$(ls "$root"/docs/fork/engine-patches/*.patch 2>/dev/null || true)
[ -n "$patches" ] || { echo "no engine patches in docs/fork/engine-patches" >&2; exit 2; }

work=$(mktemp -d)
trap 'git worktree remove --force "$work" >/dev/null 2>&1 || true; rm -rf "$work"' EXIT
git worktree add --detach "$work" "$ref" >/dev/null

failed=0
for patch in $patches; do
  name=$(basename "$patch")
  if git -C "$work" am -3 --quiet "$patch"; then
    echo "ok: $name"
  else
    echo "conflict: $name" >&2
    git -C "$work" diff --name-only --diff-filter=U | sed 's/^/  /' >&2 || true
    git -C "$work" am --abort >/dev/null 2>&1 || true
    failed=1
    break
  fi
done

if [ "$failed" -ne 0 ]; then
  echo "The patch series no longer applies to $ref. Rebase it before the next sync (docs/fork/patches.md)." >&2
  exit 1
fi
echo "engine patches apply cleanly to $ref"
