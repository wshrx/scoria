"""The release page must not look like a menu of builds to pick from.

A player downloads the launcher; the launcher installs the game itself. These
checks are about that order and about the names, because both are easy to break
in a workflow edit and neither shows up in a unit test.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"


def _publish_block() -> str:
    text = WORKFLOW.read_text()
    start = text.index("- name: Publish release")
    return text[start:]


def test_the_launcher_is_uploaded_before_the_game_bundle():
    block = _publish_block()
    launcher = block.index("dist/scoria-launcher-")
    game = block.index("dist/scoria-game-")
    assert launcher < game, "GitHub lists assets in upload order: launcher first"


def test_the_release_is_recreated_so_old_asset_order_cannot_survive():
    block = _publish_block()
    assert "gh release delete" in block
    assert "gh release create" in block


def test_no_horizon_named_release_assets():
    text = WORKFLOW.read_text()
    assert "dist/horizon-" not in text
    assert "scoria-game-" in text


def test_the_package_scripts_use_scoria_game_names():
    for script in ("package_game.sh", "package_game_windows.sh"):
        text = (ROOT / "tools" / script).read_text()
        assert "scoria-game-" in text, script


def test_the_readme_tells_players_which_file_to_download():
    english = (ROOT / "docs" / "public" / "README.md").read_text()
    russian = (ROOT / "docs" / "public" / "README.ru.md").read_text()
    for text in (english, russian):
        lowered = text.lower()
        assert "scoria-launcher-" in lowered
        assert "scoria-game-" in lowered
        # The game bundle is described as the launcher's job, not a player download.
        assert "сам" in text or "itself" in text
