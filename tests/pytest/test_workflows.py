"""GitHub Actions files must stay parseable and keep their triggers and jobs."""
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def test_every_workflow_parses_and_has_a_trigger_and_jobs():
    files = sorted((ROOT / ".github" / "workflows").glob("*.yml"))
    assert files, "no workflow files found"
    for path in files:
        document = yaml.safe_load(path.read_text(encoding="utf-8"))
        assert isinstance(document, dict), path
        # PyYAML reads the YAML 1.1 key `on` as the boolean True.
        assert "on" in document or True in document, f"{path}: no trigger"
        assert isinstance(document.get("jobs"), dict) and document["jobs"], f"{path}: no jobs"


def test_release_runs_the_gate_before_packaging():
    document = yaml.safe_load((ROOT / ".github" / "workflows" / "release.yml").read_text(encoding="utf-8"))
    jobs = document["jobs"]
    assert jobs["package"]["needs"] == "verify"
    assert jobs["game"]["needs"] == "verify"
    assert jobs["publish"]["needs"] == ["package", "game"]
