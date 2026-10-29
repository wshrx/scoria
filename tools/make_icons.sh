#!/bin/sh
# Renders the Scoria mark into every icon the launcher and the game embed.
# Requires rsvg-convert and Python 3 with Pillow. Run after changing docs/public/logo.svg.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
svg="$root/docs/public/logo.svg"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

rsvg-convert -w 1024 -h 1024 "$svg" -o "$tmp/1024.png"

python3 - "$tmp" "$root" <<'PY'
import sys
from pathlib import Path
from PIL import Image

tmp = Path(sys.argv[1])
root = Path(sys.argv[2])
base = Image.open(tmp / "1024.png").convert("RGBA")

def png(size):
    return base.resize((size, size), Image.LANCZOS)

# Linux hicolor + the game's window icon.
for size in (16, 24, 32, 48, 64, 128, 256, 512):
    path = root / f"packaging/icons/hicolor/{size}x{size}/apps/horizon.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    png(size).save(path)
(root / "crates/horizon-theme/icons").mkdir(parents=True, exist_ok=True)
png(64).save(root / "crates/horizon-theme/icons/horizon-64.png")

# Windows .ico: every size Explorer and the task bar ask for.
png(256).save(
    root / "packaging/icons/horizon.ico",
    format="ICO",
    sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)],
)

# macOS .icns: Apple's 824 px tile centred on a 1024 transparent canvas.
tile = base.resize((824, 824), Image.LANCZOS)
mac = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))
mac.alpha_composite(tile, (100, 100))
chunks = []
for kind, size in [("ic07", 128), ("ic08", 256), ("ic09", 512), ("ic10", 1024),
                   ("ic11", 32), ("ic12", 64), ("ic13", 256), ("ic14", 512)]:
    data = mac.resize((size, size), Image.LANCZOS).save(tmp / f"{kind}.png", format="PNG")
    chunks.append((kind, (tmp / f"{kind}.png").read_bytes()))

payload = b"".join(kind.encode() + (len(data) + 8).to_bytes(4, "big") + data for kind, data in chunks)
icns = b"icns" + (len(payload) + 8).to_bytes(4, "big") + payload
(root / "packaging/icons/horizon.icns").write_bytes(icns)
print("icons written")
PY
