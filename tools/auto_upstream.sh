#!/bin/sh
# Unattended upstream check, for a cron/systemd timer.
#
# - Fetches Horizon.
# - Builds and tests a throwaway worktree that merges upstream/main into dev.
# - If the merge is clean and every gate passes, pushes sync/upstream-<date> to origin (Entire).
# - If it conflicts or a gate fails, stops with the worktree left for a human/agent to inspect.
#
# It never touches the working checkout and never merges into dev itself.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
log() { printf '%s %s\n' "$(date -Is)" "$*"; }

git remote get-url upstream >/dev/null 2>&1 || { log "no upstream remote"; exit 1; }
git fetch upstream --prune
behind=$(git rev-list --count dev..upstream/main)
[ "$behind" -eq 0 ] && { log "upstream is current"; exit 0; }
log "upstream is $behind commit(s) ahead"

work="${XDG_CACHE_HOME:-$HOME/.cache}/scoria-upstream-sync"
git worktree remove --force "$work" >/dev/null 2>&1 || true
git worktree add --detach "$work" dev >/dev/null
cd "$work"

if ! git merge --no-edit upstream/main; then
  log "CONFLICTS in $(git diff --name-only --diff-filter=U | wc -l) file(s); left in $work"
  exit 1
fi

export CARGO_TARGET_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/scoria-upstream-target"
log "merge clean; running tests"
if ! (cd launcher && cargo test --quiet); then log "launcher tests FAILED"; exit 1; fi
if ! cargo test -p horizon-client --bin horizon-client cyrillic --quiet; then log "input test FAILED"; exit 1; fi
if ! (cd launcher-ui && bun install --frozen-lockfile --silent && bun run typecheck && bun run check:ui && bun run check:i18n); then
  log "UI gates FAILED"
  exit 1
fi

branch="sync/upstream-$(date +%Y%m%d)"
git push origin "HEAD:refs/heads/$branch"
log "pushed $branch to origin"
log "review and merge $branch into dev; nothing was merged automatically"
