#!/bin/sh
# Packages Scoria for Windows x64: scoria-launcher-windows-x64.zip.
# Run in Git Bash on Windows with Rust and Bun on PATH.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/windows"

rm -rf "$out"
mkdir -p "$out"

echo "Building the Rust engine…"
(cd "$root/launcher" && cargo build --release --locked)

echo "Building the UI…"
(cd "$root/launcher-ui" && bun install --frozen-lockfile && bun run check:ui && bun run typecheck && bun run check:i18n)
(cd "$root/launcher-ui" && bun build --compile app.tsx --outfile "$out/scoria-ui.exe")

cp "$root/launcher/target/release/horizon-launcher.exe" "$out/"
mkdir -p "$out/fonts" "$out/brand"
cp "$root"/launcher-ui/src/lib/fonts/*.ttf "$out/fonts/"
cp "$root"/launcher-ui/src/assets/*.png "$out/brand/"

addon="$root/launcher-ui/node_modules/@gpuix/native-win32-x64-msvc/gpuix-native.win32-x64-msvc.node"
[ -f "$addon" ] || { echo "missing $addon: bun install must run on Windows" >&2; exit 1; }
cp "$addon" "$out/gpuix-native.node"
sh "$root/tools/windows_runtime.sh" "$out"

# Batch files need CRLF; a shell here-string would write LF and cmd.exe can misread it.
printf '@echo off\r\nsetlocal\r\nset NAPI_RS_NATIVE_LIBRARY_PATH=%%~dp0gpuix-native.node\r\nstart "" "%%~dp0scoria-ui.exe" %%*\r\n' > "$out/Scoria.cmd"

cat > "$out/README.txt" <<'TXT'
Scoria — Minecraft: Java Edition 26.3, offline by default.

Run Scoria.cmd. Windows SmartScreen will warn until the build is signed:
More info -> Run anyway.

Saves and options live in %APPDATA%\horizon; the launcher's files live in
%LOCALAPPDATA%\horizon-launcher. Delete neither while the game is running.
TXT

if command -v 7z >/dev/null 2>&1; then
  (cd "$out" && 7z a -tzip -mx=9 "$root/dist/scoria-launcher-windows-x64.zip" ./* >/dev/null)
else
  (cd "$root/dist" && powershell.exe -NoProfile -Command "Compress-Archive -Path 'windows/*' -DestinationPath 'scoria-launcher-windows-x64.zip' -Force")
fi
echo "Wrote $root/dist/scoria-launcher-windows-x64.zip"
