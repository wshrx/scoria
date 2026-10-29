#!/bin/sh
# Packages the Windows x64 game bundle. Run in Git Bash on Windows (or CI's windows-latest).
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/game-windows-x64"
stage="$out/horizon-windows-x64"

rm -rf "$out"
mkdir -p "$stage"

echo "Building the game binaries…"
(cd "$root" && cargo build --release -p horizon-client -p horizon-prepare -p horizon-server --bins)

for binary in horizon-client horizon-ctl horizon-prepare horizon-server; do
  cp "$root/target/release/$binary.exe" "$stage/"
done
cp "$root/research-lock.json" "$root/LICENSE" "$root/THIRD_PARTY_NOTICES.md" "$root/packaging/PLAY.txt" "$stage/"
cp -a "$root/packaging/exporters" "$stage/exporters"
sh "$root/tools/windows_runtime.sh" "$stage"
# The exporters' key hashes the clean-room Java oracles; keep them so a new Java runtime can
# rebuild the classes and `available()` can verify the shipped bundle.
mkdir -p "$stage/mods"
cp -a "$root/mods/horizon" "$stage/mods/horizon"

if command -v 7z >/dev/null 2>&1; then
  (cd "$out" && 7z a -tzip -mx=9 "$root/dist/horizon-windows-x64.zip" ./horizon-windows-x64 >/dev/null)
else
  (cd "$out" && powershell.exe -NoProfile -Command "Compress-Archive -Path 'horizon-windows-x64' -DestinationPath 'horizon-windows-x64.zip' -Force" && mv horizon-windows-x64.zip "$root/dist/")
fi
echo "Wrote $root/dist/horizon-windows-x64.zip"
