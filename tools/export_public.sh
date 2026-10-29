#!/bin/sh
# Builds dist/public: the exact tree that may be published to GitHub. It is an allowlist, not a
# copy of the working checkout: no agent skills, project memory, QA fixtures, nightly branches or
# build output. Review the result once, then push it as the public history.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/public"

rm -rf "$out"
mkdir -p "$out"

copy() {
  (cd "$root" && tar -cf - \
    --exclude='node_modules' --exclude='target' --exclude='dist' --exclude='.local' \
    --exclude='logs' --exclude='*.log' --exclude='.git' "$@") | (cd "$out" && tar -xf -)
}

copy crates launcher launcher-ui docs packaging
mkdir -p "$out/tools"
cp "$root"/tools/package_ui.sh "$root"/tools/package_macos.sh "$root"/tools/package_windows.sh "$out/tools/"
[ -f "$root/tools/ci/sign_release.sh" ] && mkdir -p "$out/tools/ci" && cp "$root/tools/ci/sign_release.sh" "$out/tools/ci/"
cp "$root/LICENSE" "$root/THIRD_PARTY_NOTICES.md" "$out/" 2>/dev/null || true

# Internal notes stay out: agent memory, fork decision log, QA and release engineering notes.
rm -rf "$out/docs/agents" "$out/docs/fork"
rm -f "$out/docs/public/RELEASE.md"

# The landing page becomes the repository README, with paths relative to the repo root.
cp "$root/docs/public/README.md" "$out/README.md"
cp "$root/docs/public/README.ru.md" "$out/README.ru.md"
sed -i 's#\.\./\.\./docs/media/#docs/media/#g' "$out/README.md" "$out/README.ru.md"

mkdir -p "$out/.github/workflows"
cp "$root/.github/workflows/release.yml" "$out/.github/workflows/"

cat > "$out/.gitignore" <<'IGNORE'
node_modules/
target/
dist/
.local/
logs/
*.log
IGNORE

echo "Public tree: $out"
echo "Next: capture docs/public/screenshots/{home,mods,settings}.webp, review, then push as a new history."
