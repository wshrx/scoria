"""The release page must not look like a menu of builds to pick from.

A player downloads the launcher; the launcher installs the game itself. These
checks keep the launcher release free of game bundles and the engine release out
of the way, because both are easy to break in a workflow edit.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"


def _step(name: str) -> str:
    text = WORKFLOW.read_text()
    start = text.index(f"- name: {name}")
    return text[start:].split("\n      - name:", 1)[0]


def test_the_launcher_release_is_published_before_the_engine_release():
    text = WORKFLOW.read_text()
    launcher = text.index("- name: Publish launcher release")
    engine = text.index("- name: Publish engine release")
    assert launcher < engine


def test_the_launcher_release_carries_only_the_launcher():
    block = _step("Publish launcher release")
    assert "dist/launcher/scoria-launcher-" in block
    assert "scoria-game-" not in block


def test_the_engine_release_is_a_prerelease_with_only_the_game():
    block = _step("Publish engine release")
    assert "dist/game/scoria-game-" in block
    assert "scoria-launcher-" not in block
    assert "--prerelease" in block


def test_no_horizon_named_release_assets():
    text = WORKFLOW.read_text()
    assert "dist/horizon-" not in text
    assert "scoria-game-" in text


def test_the_package_scripts_use_scoria_game_names():
    for script in ("package_game.sh", "package_game_windows.sh"):
        text = (ROOT / "tools" / script).read_text()
        assert "scoria-game-" in text, script


def test_the_readme_tells_players_which_file_to_download():
    # In the public repository the Scoria READMEs are at the root; in the private
    # tree they live under docs/public and are exported from there.
    base = ROOT if (ROOT / "README.ru.md").exists() else ROOT / "docs" / "public"
    english = (base / "README.md").read_text(encoding="utf-8")
    russian = (base / "README.ru.md").read_text(encoding="utf-8")
    for text in (english, russian):
        lowered = text.lower()
        assert "scoria-launcher-" in lowered
        assert "scoria-game-" in lowered
        # The game bundle is described as the launcher's job, not a player download.
        assert "сам" in text or "itself" in text
