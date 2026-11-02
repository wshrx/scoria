"""The release page must not look like a menu of builds to pick from.

A player downloads the launcher; the launcher installs the game itself. These
checks are about that order and about the names, because both are easy to break
in a workflow edit and neither shows up in a unit test.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"


def _files_block() -> str:
    text = WORKFLOW.read_text()
    start = text.index("files: |")
    return text[start:].split("\n\n", 1)[0]


def test_the_launcher_is_listed_before_the_game_bundle():
    block = _files_block()
    launcher = block.index("scoria-launcher-")
    game = block.index("scoria-game-")
    assert launcher < game, "the launcher archive must be the first asset"


def test_no_horizon_named_release_assets():
    text = WORKFLOW.read_text()
    assert "dist/horizon-" not in text
    assert "scoria-game-" in text


def test_the_package_scripts_use_scoria_game_names():
    for script in ("package_game.sh", "package_game_windows.sh"):
        text = (ROOT / "tools" / script).read_text()
        assert "scoria-game-" in text, script
