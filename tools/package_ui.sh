#!/bin/sh
# Packages the flint-ui launcher beside the Rust engine:
#   dist/scoria-linux-x86_64/scoria-launcher-ui   wrapper (sets NAPI_RS_NATIVE_LIBRARY_PATH)
#   dist/scoria-linux-x86_64/scoria-ui            compiled Bun binary
#   dist/scoria-linux-x86_64/horizon-launcher     Rust engine
#   .../gpuix-native.node, fonts/, README.txt
# and a tarball next to it. Run from a checkout with Bun and Cargo.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/scoria-linux-x86_64"

rm -rf "$out"
mkdir -p "$out/fonts"

echo "Building the Rust engine…"
(cd "$root/launcher" && cargo build --release --locked)

echo "Building the flint UI…"
(cd "$root/launcher-ui" && bun install --frozen-lockfile && bun run check:ui && bun run typecheck)
(cd "$root/launcher-ui" && bun build --compile app.tsx --outfile "$out/scoria-ui")

cp "$root/launcher/target/release/horizon-launcher" "$out/horizon-launcher"
cp "$root"/launcher-ui/src/lib/fonts/*.ttf "$out/fonts/"
cp "$root/packaging/linux/scoria-launcher-ui.desktop" "$out/"
cp "$root/packaging/PLAY.txt" "$out/README.txt"

# The napi addon cannot live inside Bun's compiled filesystem; the wrapper points napi-rs at it.
addon="$root/launcher-ui/node_modules/@gpuix/native-linux-x64-gnu/gpuix-native.linux-x64-gnu.node"
cp "$addon" "$out/gpuix-native.node"

cat > "$out/scoria-launcher-ui" <<'WRAPPER'
#!/bin/sh
DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if [ "${SCORIA_UI_WAYLAND:-0}" != "1" ] && [ -n "${DISPLAY:-}" ]; then
  WAYLAND_DISPLAY=
  WINIT_UNIX_BACKEND=x11
  export WAYLAND_DISPLAY WINIT_UNIX_BACKEND
fi
NAPI_RS_NATIVE_LIBRARY_PATH="$DIR/gpuix-native.node" exec "$DIR/scoria-ui" "$@"
WRAPPER
chmod +x "$out/scoria-launcher-ui"

(cd "$root/dist" && tar -czf scoria-launcher-linux-x86_64.tar.gz scoria-linux-x86_64)
echo "Wrote $root/dist/scoria-launcher-linux-x86_64.tar.gz"
