#!/bin/sh
# Packages the Scoria launcher for Linux x86_64: one binary, no Bun, no native addon.
#   dist/scoria-linux-x86_64/scoria
#   dist/scoria-launcher-linux-x86_64.tar.zst
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/scoria-linux-x86_64"

rm -rf "$out"
mkdir -p "$out"

echo "Building scoria…"
(cd "$root/launcher" && cargo build --release --locked -p scoria-ui)

cp "$root/launcher/target/release/scoria" "$out/scoria"
cp "$root/LICENSE" "$out/LICENSE"
cp "$root/THIRD_PARTY_NOTICES.md" "$out/THIRD_PARTY_NOTICES.md" 2>/dev/null || true
cat > "$out/README.txt" <<'TXT'
Scoria — Minecraft: Java Edition 26.3, offline by default.

Run ./scoria. Saves and options live in ~/.local/share/horizon; the launcher's
files live in ~/.local/share/horizon-launcher.
TXT

(cd "$root/dist" && tar -cf - scoria-linux-x86_64 \
  | zstd -19 -T0 -q -o scoria-launcher-linux-x86_64.tar.zst)
echo "Wrote $root/dist/scoria-launcher-linux-x86_64.tar.zst"
