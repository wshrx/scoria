#!/usr/bin/env python3
"""Capture a window on Wayland through KWin's ScreenShot2 (no grim/spectacle needed).

    shot.py active /tmp/opencode/qa.png
    shot.py screen /tmp/opencode/qa.png
    shot.py area /tmp/opencode/qa.png 0 0 1280 720

`active` captures the focused window, so click the launcher first. The launcher is Wayland-only;
X11 `import` and `xwd` see nothing.
"""
import os
import sys

import numpy as np
from gi.repository import Gio, GLib
from PIL import Image


def capture(kind: str, area=None):
    bus = Gio.bus_get_sync(Gio.BusType.SESSION)
    read_fd, write_fd = os.pipe()
    options = {"include-cursor": GLib.Variant("b", False)}
    if kind == "area":
        x, y, width, height = area
        args = GLib.Variant("(iiuua{sv}h)", (x, y, width, height, options, 0))
        method = "CaptureArea"
    elif kind == "active":
        args = GLib.Variant("(a{sv}h)", (options, 0))
        method = "CaptureActiveWindow"
    else:
        args = GLib.Variant("(a{sv}h)", (options, 0))
        method = "CaptureActiveScreen"
    fds = Gio.UnixFDList.new()
    fds.append(write_fd)
    reply, _ = bus.call_with_unix_fd_list_sync(
        "org.kde.KWin",
        "/org/kde/KWin/ScreenShot2",
        "org.kde.KWin.ScreenShot2",
        method,
        args,
        GLib.VariantType("(a{sv})"),
        Gio.DBusCallFlags.NONE,
        8000,
        fds,
        None,
    )
    os.close(write_fd)
    meta = reply.unpack()[0]
    width, height, stride, fmt = meta["width"], meta["height"], meta["stride"], meta["format"]
    want = stride * height
    data = b""
    while len(data) < want:
        chunk = os.read(read_fd, min(1 << 20, want - len(data)))
        if not chunk:
            break
        data += chunk
    os.close(read_fd)
    print(meta, len(data))
    # Qt formats: 16/17 are RGBA; 18/19 arrive in the same pixel order on this compositor.
    # `screen` (format 16) is the reliable mode for the launcher; `active` depends on focus.
    pixels = np.frombuffer(data, np.uint8).reshape(height, stride)[:, : width * 4]
    rgba = pixels.reshape(height, width, 4)
    channels = [0, 1, 2] if fmt in (16, 17, 18, 19) else [2, 1, 0]
    rgb = rgba[:, :, channels]
    return Image.fromarray(rgb, "RGB")


if __name__ == "__main__":
    kind = sys.argv[1]
    out = sys.argv[2]
    area = tuple(int(value) for value in sys.argv[3:7]) if len(sys.argv) > 3 else None
    capture(kind, area).save(out)
    print(out)
