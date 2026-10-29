#!/bin/sh
# Copies the allowlisted tree to the public GitHub repository's staging branch.
#
#   tools/export_public.sh      # build dist/public
#   tools/publish_public.sh     # commit that tree on `staging` and push to the `github` remote
#
# The private Entire repo stays the source of truth. GitHub only ever receives filtered snapshots:
# no .opencode, no project memory, no fork notes, no game media, no build output. Public CI runs on
# staging; a human (or the release workflow) promotes a green staging to main and tags it.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"

git remote get-url github >/dev/null 2>&1 || {
  echo "No github remote. Add: git remote add github git@github.com:wshrx/scoria.git" >&2
  exit 1
}
[ -f dist/public/README.md ] || { echo "Run tools/export_public.sh first." >&2; exit 1; }

work="${XDG_CACHE_HOME:-$HOME/.cache}/scoria-public"
remote=$(git remote get-url github)

if [ ! -d "$work/.git" ]; then
  if ! git clone --branch staging "$remote" "$work" 2>/dev/null; then
    git clone "$remote" "$work"
    (cd "$work" && git checkout --orphan staging && git rm -rf . >/dev/null 2>&1 || true)
  fi
fi

# A filtered snapshot: replace the tree, keep the public history one commit per publish.
( cd "$work" && git rm -rq --ignore-unmatch . && git clean -fdq )
( cd "$root/dist/public" && tar -cf - . ) | ( cd "$work" && tar -xf - )

cd "$work"
git add -A
if git diff --cached --quiet; then
  echo "Public tree is unchanged; nothing to publish."
  exit 0
fi
source=$(git -C "$root" rev-parse --short HEAD)
git -c user.name="${GIT_AUTHOR_NAME:-Scoria}" -c user.email="${GIT_AUTHOR_EMAIL:-scoria@users.noreply.github.com}" \
  commit -q -m "snapshot: Scoria $source"
git push origin staging
echo "Pushed staging from $source. CI runs there; merge to main only after it is green."
