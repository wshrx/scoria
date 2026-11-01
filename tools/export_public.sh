#!/bin/sh
# Builds dist/public: the exact tree that may be published to GitHub. It is an allowlist, not a
# copy of the working checkout: no agent skills, project memory, QA fixtures, nightly branches,
# build output or Minecraft assets/screenshots. Review it once, then push it as public history.
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

# Product sources and build files. docs/media is game art and old launcher screenshots: it stays
# out. launcher/docs is the same. The public README carries only UI screenshots.
copy crates launcher launcher-ui docs packaging mods third_party xtask
# mods/horizon holds the clean-room Java oracles the preparer compiles; they ship in the game
# bundle and are allowed by check_public_tree.py (original code, no Mojang sources).
cp "$root/Cargo.toml" "$root/Cargo.lock" "$root/research-lock.json" "$root/CHANGELOG.md" "$out/"
rm -rf "$out/docs/media" "$out/docs/public" "$out/docs/agents" "$out/docs/fork" \
       "$out/launcher/docs" "$out/packaging/macos/background.png" "$out/packaging/macos/background@2x.png"
mkdir -p "$out/tools"
cp "$root"/tools/package_ui.sh "$root"/tools/package_macos.sh "$root"/tools/package_windows.sh \
   "$root"/tools/package_game.sh "$root"/tools/package_game_windows.sh \
   "$root"/tools/windows_runtime.sh "$root"/tools/build_gpuix_native.sh \
   "$root"/tools/check_imports.py "$out/tools/"
mkdir -p "$out/tests"
cp "$root"/tests/test_release_artifacts.py "$out/tests/"
mkdir -p "$out/tools/ci"
cp "$root/tools/ci/sign_release.sh" "$out/tools/ci/"
cp "$root/LICENSE" "$root/THIRD_PARTY_NOTICES.md" "$out/" 2>/dev/null || true

# Landing page and brand mark. The readme only points at files that exist in the public tree.
cp "$root/docs/public/README.md" "$out/README.md"
cp "$root/docs/public/README.ru.md" "$out/README.ru.md"
cp "$root/docs/public/logo.svg" "$out/logo.svg"
cp "$root/docs/public/BUILDING.md" "$out/BUILDING.md"
mkdir -p "$out/screenshots"
for shot in home mods settings; do
  if [ -f "$root/docs/public/screenshots/$shot.webp" ]; then
    cp "$root/docs/public/screenshots/$shot.webp" "$out/screenshots/"
  fi
done

mkdir -p "$out/.github/workflows"
cp "$root/.github/workflows/release.yml" "$out/.github/workflows/"
cp "$root/.github/workflows/public-ci.yml" "$out/.github/workflows/ci.yml"

cat > "$out/.gitignore" <<'IGNORE'
node_modules/
target/
dist/
.local/
logs/
*.log
IGNORE

# A last gate: no game assets, no decompiled code, no internal notes.
sh "$root/tools/check_public_assets.sh" "$out"

echo "Public tree: $out"
[ -f "$out/screenshots/home.webp" ] || echo "Missing screenshots: capture home/mods/settings into docs/public/screenshots/ first."
echo "Next: review dist/public, then push it as the public history."
