#!/usr/bin/env python3
"""Measure the visual bounding box of every Lucide icon used by build.py.

Renders each path to a transparent PNG via rsvg-convert and reads the alpha
bbox, in 24-unit viewBox coordinates. build.py then centres icons by their
actual ink, not by the nominal 24x24 box.
"""
import json
import os
import subprocess
import sys
import tempfile

from PIL import Image

sys.path.insert(0, os.path.dirname(__file__))
from build import ICONS  # noqa: E402


def measure(path):
    svg = (
        '<svg xmlns="http://www.w3.org/2000/svg" width="480" height="480" '
        'viewBox="0 0 24 24">'
        f'<path d="{path}" fill="none" stroke="#000" stroke-width="1.5" '
        'stroke-linecap="round" stroke-linejoin="round"/></svg>'
    )
    with tempfile.TemporaryDirectory() as d:
        sp = os.path.join(d, "i.svg")
        pp = os.path.join(d, "i.png")
        with open(sp, "w") as f:
            f.write(svg)
        subprocess.run(["rsvg-convert", "-w", "480", "-h", "480", sp, "-o", pp],
                       check=True, capture_output=True)
        im = Image.open(pp).convert("RGBA")
        box = im.getchannel("A").getbbox()
        if box is None:
            raise RuntimeError(f"no ink for {path}")
    x0, y0, x1, y1 = [v / 20.0 for v in box]
    return [round(x0, 3), round(y0, 3), round(x1 - x0, 3), round(y1 - y0, 3)]


if __name__ == "__main__":
    out = {kind: measure("".join(parts)) for kind, parts in ICONS.items()}
    print(json.dumps(out, indent=2, ensure_ascii=False))
    with open(os.path.join(os.path.dirname(__file__), "icon_bbox.json"), "w") as f:
        json.dump(out, f, indent=2)
