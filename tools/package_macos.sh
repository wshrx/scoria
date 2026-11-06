#!/bin/sh
# Packages Scoria for Apple silicon: Scoria.app + scoria-launcher-macos-arm64.zip.
# Run on macOS with Rust, Bun and the Xcode command line tools installed.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/macos"
app="$out/Scoria.app"
version=$(sed -n 's/^export const LAUNCHER_VERSION = "\(.*\)"/\1/p' "$root/launcher-ui/src/version.ts")

rm -rf "$out"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

echo "Building the Rust engine…"
(cd "$root/launcher" && cargo build --release --locked)

echo "Building the UI…"
(cd "$root/launcher-ui" && bun install --frozen-lockfile && bun run check:ui && bun run typecheck && bun run check:i18n)
(cd "$root/launcher-ui" && bun build --compile app.tsx --outfile "$app/Contents/MacOS/scoria-ui")

cp "$root/launcher/target/release/horizon-launcher" "$app/Contents/MacOS/"
cp "$root/packaging/icons/horizon.icns" "$app/Contents/Resources/scoria.icns"
mkdir -p "$app/Contents/MacOS/fonts" "$app/Contents/MacOS/brand"
cp "$root"/launcher-ui/src/lib/fonts/*.ttf "$app/Contents/MacOS/fonts/"
cp "$root"/launcher-ui/src/assets/*.png "$app/Contents/MacOS/brand/"

addon="$root/launcher-ui/node_modules/@gpuix/native-darwin-arm64/gpuix-native.darwin-arm64.node"
[ -f "$addon" ] || { echo "missing $addon: bun install must run on macOS" >&2; exit 1; }
cp "$addon" "$app/Contents/MacOS/gpuix-native.node"

# The bundle executable is a shell shim: napi-rs needs the addon path before Bun loads the app.
cat > "$app/Contents/MacOS/Scoria" <<'LAUNCH'
#!/bin/sh
DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export NAPI_RS_NATIVE_LIBRARY_PATH="$DIR/gpuix-native.node"
exec "$DIR/scoria-ui" "$@"
LAUNCH
chmod +x "$app/Contents/MacOS/Scoria" "$app/Contents/MacOS/scoria-ui" "$app/Contents/MacOS/horizon-launcher"

cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>Scoria</string>
  <key>CFBundleDisplayName</key><string>Scoria</string>
  <key>CFBundleIdentifier</key><string>io.scoria.launcher</string>
  <key>CFBundleVersion</key><string>${version:-0.1.0}</string>
  <key>CFBundleShortVersionString</key><string>${version:-0.1.0}</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleExecutable</key><string>Scoria</string>
  <key>CFBundleIconFile</key><string>scoria.icns</string>
  <key>LSMinimumSystemVersion</key><string>14.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST

# Ad-hoc signature: enough to run locally; not notarized, so Gatekeeper still asks once.
codesign --force --deep --sign - "$app" >/dev/null 2>&1 || true

(cd "$root/dist" && ditto -c -k --sequesterRsrc --keepParent "$app" scoria-launcher-macos-arm64.zip)
echo "Wrote $root/dist/scoria-launcher-macos-arm64.zip"
