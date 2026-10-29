#!/bin/sh
# Packages the Linux game bundle for a release: the engine and helpers, the pinned
# research-lock, the compiled exporters and the notices. No Minecraft assets, no prepared
# resources: those come from Mojang on first prepare.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out="$root/dist/game-linux-x86_64"
stage="$out/horizon-linux-x86_64"

rm -rf "$out"
mkdir -p "$stage"

echo "Building the game binaries…"
(cd "$root" && cargo build --release -p horizon-client -p horizon-prepare -p horizon-server --bins)

for binary in horizon-client horizon-ctl horizon-prepare horizon-server; do
  cp "$root/target/release/$binary" "$stage/"
done
cp "$root/research-lock.json" "$root/LICENSE" "$root/THIRD_PARTY_NOTICES.md" "$root/packaging/PLAY.txt" "$stage/"
cp -a "$root/packaging/exporters" "$stage/exporters"
# The exporters' key hashes the clean-room Java oracles; keep them so a new Java runtime can
# rebuild the classes and `available()` can verify the shipped bundle.
mkdir -p "$stage/mods"
cp -a "$root/mods/horizon" "$stage/mods/horizon"

(cd "$out" && tar -czf "$root/dist/horizon-linux-x86_64.tar.gz" horizon-linux-x86_64)
echo "Wrote $root/dist/horizon-linux-x86_64.tar.gz"
