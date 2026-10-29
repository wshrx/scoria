#!/bin/sh
# Rejects Minecraft assets, decompiled Java and internal notes in a public tree.
# Usage: tools/check_public_assets.sh dist/public
set -eu
tree=${1:?usage: check_public_assets.sh <tree>}
bad=0

fail() { echo "public tree: $1" >&2; bad=1; }

# Game assets and old Horizon screenshots are never published.
for dir in docs/media launcher/docs .opencode docs/agents docs/fork; do
  [ -e "$tree/$dir" ] && fail "$dir must not be public"
done
[ -e "$tree/PROJECT_STATE.md" ] && fail "PROJECT_STATE.md must not be public"

# Decompiled or original Java/Minecraft artifacts.
find "$tree" -type f \( -name '*.jar' -o -name '*.class' -o -name '*.java' -o -name '*.jfr' -o -name '*.hprof' \) \
  ! -path "$tree/packaging/exporters/*" \
  | while read -r file; do echo "public tree: game artifact ${file#"$tree"/}" >&2; exit 1; done || bad=1

# Game media outside the allowed UI screenshots. launcher/assets is Horizon's original partner
# art embedded in the old GUI, not a Minecraft asset.
if [ -d "$tree/docs" ]; then
  find "$tree/docs" -type f \( -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' -o -iname '*.webp' -o -iname '*.ogg' \) \
    | while read -r file; do echo "public tree: media ${file#"$tree"/}" >&2; exit 1; done || bad=1
fi

if [ "$bad" -ne 0 ]; then
  echo "public tree check failed" >&2
  exit 1
fi
echo "public tree check passed"
