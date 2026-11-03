#!/usr/bin/env python3
"""Capture every synthetic launcher state without a game or network.

The window reads `SCORIA_UI_STATE` and paints synthetic data (see `apply_qa_state`
in launcher/ui/src/main.rs). This script launches one process per state, finds
its X11 window, and saves a PNG. It is a QA tool, not part of the release.

  python3 tools/capture_ui_states.py
  python3 tools/capture_ui_states.py --state home-preparing --state mods-loading
"""
from __future__ import annotations

import argparse
import os
import shutil
import signal
import subprocess
import tempfile
import time
from pathlib import Path

from Xlib import X, display

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BINARY = ROOT / "launcher" / "target" / "debug" / "scoria"

STATES = {
    "home-empty": ("HTTP 404", "1280x720"),
    "home-not-prepared": ("A version is installed, resources are missing", "1280x720"),
    "home-preparing": ("A download is in flight, 42%", "1280x720"),
    "home-prepared": ("Ready to play", "1280x720"),
    "home-playing": ("The game is running", "1280x720"),
    "home-error": ("Play failed with an error", "1280x720"),
    "mods-empty": ("No search yet", "1280x720"),
    "mods-loading": ("Catalogue request in flight", "1280x720"),
    "mods-results": ("Three catalogue results", "1280x720"),
    "mods-error": ("Catalogue request failed", "1280x720"),
    "servers-empty": ("No saved servers", "1280x720"),
    "servers-list": ("Two saved servers", "1280x720"),
    "servers-error": ("servers.json cannot be read", "1280x720"),
    "settings": ("Settings, dark", "1280x720"),
    "settings-light": ("Settings, light", "1280x720"),
    "compact": ("Rail collapsed, 900x640", "900x640"),
}


def find_window(pid: int, timeout: float = 15.0):
    d = display.Display()
    root = d.screen().root
    net_pid = d.intern_atom("_NET_WM_PID")

    def walk(window):
        try:
            children = window.query_tree().children
        except Exception:
            return None
        for child in children:
            try:
                value = child.get_full_property(net_pid, X.AnyPropertyType)
                if value and value.value and int(value.value[0]) == pid:
                    return child
            except Exception:
                pass
            found = walk(child)
            if found is not None:
                return found
        return None

    deadline = time.time() + timeout
    while time.time() < deadline:
        window = walk(root)
        if window is not None:
            try:
                geometry = window.get_geometry()
                mapped = window.get_attributes().map_state == X.IsViewable
                if mapped and geometry.width >= 500 and geometry.height >= 300:
                    return d, window
            except Exception:
                pass
        time.sleep(0.05)
    return d, None


def capture(binary: Path, state: str, out_dir: Path) -> Path:
    work = Path(tempfile.mkdtemp(prefix="scoria-ui-state-"))
    env = os.environ.copy()
    env.pop("WAYLAND_DISPLAY", None)
    env.update(
        SCORIA_UI_STATE=state,
        HORIZON_HOME=str(work / "game"),
        HORIZON_LAUNCHER_HOME=str(work / "launcher"),
        XDG_DATA_HOME=str(work / "data"),
        XDG_CONFIG_HOME=str(work / "config"),
    )
    _, size = STATES[state]
    proc = subprocess.Popen(
        [str(binary), "--size", size],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
    )
    try:
        d, window = find_window(proc.pid)
        if window is None:
            raise RuntimeError(f"{state}: no X11 window (is DISPLAY set?)")
        window.configure(stack_mode=X.Above)
        d.sync()
        # The first frame may still be black while the GPU surface is created.
        time.sleep(0.8)
        out = out_dir / f"{state}.png"
        subprocess.run(
            ["import", "-window", hex(window.id), str(out)],
            check=True,
            env=env,
        )
        # A window that mapped before its first GPU frame captures as a near-empty PNG.
        if out.stat().st_size < 20_000:
            raise RuntimeError(f"{state}: capture looks blank ({out.stat().st_size} bytes)")
        return out
    finally:
        try:
            os.killpg(proc.pid, signal.SIGTERM)
            proc.wait(timeout=5)
        except Exception:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except Exception:
                pass
        shutil.rmtree(work, ignore_errors=True)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--out", type=Path, default=Path.home() / "Pictures" / "scoria-ui-states")
    parser.add_argument("--state", action="append", choices=sorted(STATES))
    args = parser.parse_args()
    if not args.binary.is_file():
        raise SystemExit(f"build the window first: cargo build -p scoria-ui ({args.binary})")
    args.out.mkdir(parents=True, exist_ok=True)
    states = args.state or list(STATES)
    for state in states:
        path = capture(args.binary.resolve(), state, args.out)
        print(f"{state}: {path}")
    # A subset run must not drop the rest of the index.
    index = ["# Launcher state captures", ""]
    for state, (note, _) in STATES.items():
        if (args.out / f"{state}.png").exists():
            index += [f"- `{state}` — {note}", "", f"  ![{state}]({state}.png)", ""]
    (args.out / "index.md").write_text("\n".join(index))
    print(f"index: {args.out / 'index.md'}")


if __name__ == "__main__":
    main()
