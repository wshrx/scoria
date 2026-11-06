#!/bin/sh
# Packages the single-binary Scoria launcher for macOS:
#   dist/Scoria.app
#   dist/scoria-launcher-macos-arm64.zip  (Apple Silicon)
#   dist/scoria-launcher-macos-x64.zip    (Intel, cross-built on the same Mac)
#
# Run on macOS (ditto, codesign, sips, iconutil). Ad-hoc signed unless MACOS_SIGN_IDENTITY
# names a Developer ID; notarization is a separate step.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
arch=${1:-arm64}
case "$arch" in
  arm64) target=aarch64-apple-darwin ;;
  x64) target=x86_64-apple-darwin ;;
  *) echo "usage: $0 [arm64|x64]" >&2; exit 1 ;;
esac
version=${SCORIA_VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/launcher/ui/Cargo.toml" | head -1)}
out="$root/dist/macos-$arch"
app="$out/Scoria.app"
zip="$root/dist/scoria-launcher-macos-$arch.zip"

rm -rf "$out" "$zip"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

echo "Building scoria ($arch)…"
(cd "$root/launcher" && cargo build --release --locked --target "$target" -p scoria-ui)
cp "$root/launcher/target/$target/release/scoria" "$app/Contents/MacOS/scoria"
chmod +x "$app/Contents/MacOS/scoria"

# Render the 256 px mark into the iconset macOS wants; upscaling is fine for the dock.
iconset="$out/Scoria.iconset"
mkdir -p "$iconset"
for size in 16 32 64 128 256 512; do
  sips -z "$size" "$size" "$root/launcher/ui/assets/scoria-mark.png" \
    --out "$iconset/icon_${size}x${size}.png" >/dev/null
  double=$((size * 2))
  sips -z "$double" "$double" "$root/launcher/ui/assets/scoria-mark.png" \
    --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app/Contents/Resources/scoria.icns"
rm -rf "$iconset"

sed "s/@VERSION@/$version/g" "$root/packaging/macos/Info.plist" > "$app/Contents/Info.plist"
printf 'APPL????' > "$app/Contents/PkgInfo"
cp "$root/LICENSE" "$app/Contents/Resources/LICENSE"
plutil -lint "$app/Contents/Info.plist" >/dev/null

identity=${MACOS_SIGN_IDENTITY:--}
if [ "$identity" = "-" ]; then
  codesign --force --sign - "$app" >/dev/null
else
  codesign --force --options runtime --timestamp --sign "$identity" "$app"
fi
codesign --verify --strict --verbose=2 "$app"

(cd "$out" && ditto -c -k --norsrc --noextattr --noqtn --keepParent Scoria.app "$zip")
echo "Wrote $zip"
