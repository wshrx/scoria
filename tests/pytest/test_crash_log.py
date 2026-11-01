"""A window that disappears must leave a crash log behind.

The binary is built by `cargo xtask build`; when it is missing (a checkout without a build) the
test skips with the command to run, rather than pretending to pass.
"""
import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
BIN = ROOT / "launcher" / "target" / "debug" / (
    "scoria-ui.exe" if sys.platform == "win32" else "scoria-ui"
)


def test_a_panic_writes_crash_log(tmp_path):
    if not BIN.is_file():
        pytest.skip(f"{BIN} is not built; run `cargo xtask build`")
    game = tmp_path / "game"
    launcher = tmp_path / "launcher"
    game.mkdir()
    launcher.mkdir()
    env = {
        **os.environ,
        "HORIZON_HOME": str(game),
        "HORIZON_LAUNCHER_HOME": str(launcher),
        "XDG_DATA_HOME": str(tmp_path / "data"),
        "XDG_CONFIG_HOME": str(tmp_path / "config"),
        "HORIZON_LAUNCHER_NO_SELF_UPDATE": "1",
    }
    result = subprocess.run(
        [str(BIN), "--crash-test"], capture_output=True, text=True, timeout=60, env=env
    )
    assert result.returncode != 0, "the crash test must not exit successfully"
    log = launcher / "crash.log"
    assert log.is_file(), f"no crash log in {launcher}"
    text = log.read_text()
    assert "crash-test" in text
    assert "Scoria" in text
    assert "panicked" in text
