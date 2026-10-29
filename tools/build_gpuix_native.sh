#!/bin/sh
# Builds the GPUIX native addon from source with a static C runtime, so the
# Windows build does not need vcruntime140.dll beside it (the published npm
# binary links the DLL CRT). Pinned to the version launcher-ui depends on.
# Usage: build_gpuix_native.sh <output .node>
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
dest=${1:?usage: build_gpuix_native.sh <output .node>}
ref=${GPUIX_REF:-@gpuix/native@0.11.0}
work=${GPUIX_SRC:-$root/target/gpuix}

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) ;;
  *) echo "build_gpuix_native.sh builds the Windows addon; run it on Windows." >&2; exit 1 ;;
esac

if [ ! -d "$work/.git" ]; then
  echo "Cloning gpuix $ref…"
  rm -rf "$work"
  git clone --quiet --branch "$ref" --depth 1 https://github.com/remorses/gpuix "$work"
fi
echo "Fetching the pinned GPUI submodule…"
git -C "$work" submodule update --init --recursive zed

echo "Building gpuix-native with +crt-static…"
(
  cd "$work/packages/native"
  RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --no-default-features
)

built="$work/packages/native/target/release/gpuix_native.dll"
[ -f "$built" ] || { echo "missing $built after the build" >&2; exit 1; }

tmp="$dest.partial"
cp "$built" "$tmp"
mv "$tmp" "$dest"
echo "Wrote $dest"
