#!/bin/sh
# Packages the Scoria launcher for Windows x64: one GUI binary, no addon, no CRT DLL.
#   dist/windows/scoria.exe
#   dist/scoria-launcher-windows-x64.zip
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/windows"

rm -rf "$out"
mkdir -p "$out"

echo "Building scoria.exe…"
(cd "$root/launcher" && cargo build --release --locked -p scoria-ui)

cp "$root/launcher/target/release/scoria.exe" "$out/scoria.exe"
cat > "$out/README.txt" <<'TXT'
Scoria — Minecraft: Java Edition 26.3, offline by default.

Run scoria.exe. Windows SmartScreen will warn until the build is signed:
More info -> Run anyway. A crash writes crash.log beside the launcher settings
in %LOCALAPPDATA%\horizon-launcher.
TXT

if command -v 7z >/dev/null 2>&1; then
  (cd "$out" && 7z a -tzip -mx=9 "$root/dist/scoria-launcher-windows-x64.zip" ./* >/dev/null)
else
  (cd "$root/dist" && powershell.exe -NoProfile -Command "Compress-Archive -Path 'windows/*' -DestinationPath 'scoria-launcher-windows-x64.zip' -Force")
fi
echo "Wrote $root/dist/scoria-launcher-windows-x64.zip"
