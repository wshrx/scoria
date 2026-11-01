"""Release artifact checks: catch a build that would close on a player's machine.

Run against a release folder (the archives, or the unpacked `dist/windows`):

    SCORIA_ARTIFACTS=dist python3 -m pytest tests/test_release_artifacts.py -q

The release workflow runs it after packaging, before anything is uploaded.
"""
import hashlib
import importlib.util
import re
import subprocess
import sys
import tarfile
import zipfile
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = Path(__import__("os").environ.get("SCORIA_ARTIFACTS", ROOT / "dist"))


def _check_imports():
    spec = importlib.util.spec_from_file_location(
        "check_imports", ROOT / "tools" / "check_imports.py"
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _launcher_zip() -> Path:
    direct = ARTIFACTS / "scoria-launcher-windows-x64.zip"
    if direct.is_file():
        return direct
    found = sorted(ARTIFACTS.glob("**/scoria-launcher-windows-x64.zip"))
    if not found:
        pytest.skip(f"no Windows launcher archive in {ARTIFACTS}")
    return found[0]


def _linux_archives():
    return sorted(ARTIFACTS.glob("scoria-launcher-linux-x86_64.tar.*")) + sorted(
        ARTIFACTS.glob("**/scoria-launcher-linux-x86_64.tar.*")
    )


def _windows_folder(tmp_path: Path) -> Path:
    with zipfile.ZipFile(_launcher_zip()) as archive:
        archive.extractall(tmp_path)
    return tmp_path


def test_windows_archive_has_everything_the_launcher_needs(tmp_path):
    folder = _windows_folder(tmp_path)
    names = {path.name for path in folder.rglob("*")}
    for required in (
        "scoria-ui.exe",
        "horizon-launcher.exe",
        "gpuix-native.node",
        "Scoria.cmd",
    ):
        assert required in names, f"{required} is missing from the Windows archive"
    assert any(path.suffix == ".ttf" for path in folder.rglob("*")), "no bundled fonts"


def test_windows_cmd_runs_in_the_console_and_keeps_a_log(tmp_path):
    folder = _windows_folder(tmp_path)
    cmd = next(folder.rglob("Scoria.cmd"))
    raw = cmd.read_bytes()
    assert b"\r\n" in raw, "Scoria.cmd must use CRLF line endings for cmd.exe"
    text = raw.decode("utf-8", "replace")
    assert "NAPI_RS_NATIVE_LIBRARY_PATH" in text, "the GPUIX addon path is not set"
    assert "scoria-ui.log" in text, (
        "the launcher must keep a log: a silent exit on Windows is undiagnosable otherwise"
    )
    assert "pause" in text, "a failed start must leave the console open"


def test_windows_binaries_import_only_dlls_that_are_there(tmp_path):
    folder = _windows_folder(tmp_path)
    check = _check_imports()
    shipped = {path.name.lower() for path in folder.rglob("*") if path.is_file()}
    binaries = [
        path
        for path in folder.rglob("*")
        if path.is_file() and path.suffix.lower() in {".exe", ".node", ".dll"}
    ]
    assert binaries, "the Windows archive has no binaries"
    for binary in binaries:
        names = check.pe_imports(binary.read_bytes())
        missing = [n for n in names if not check.windows_allowed(n, shipped)]
        assert not missing, f"{binary.name} imports {missing}, which Windows does not ship"


def test_windows_binaries_do_not_need_the_c_runtime_unless_it_is_shipped(tmp_path):
    folder = _windows_folder(tmp_path)
    check = _check_imports()
    imports_crt = [
        path.name
        for path in folder.rglob("*")
        if path.is_file()
        and path.suffix.lower() in {".exe", ".node", ".dll"}
        and any(n.lower() == "vcruntime140.dll" for n in check.pe_imports(path.read_bytes()))
    ]
    if imports_crt:
        assert any(
            path.name.lower() == "vcruntime140.dll" for path in folder.rglob("*")
        ), f"{imports_crt} import VCRUNTIME140.dll but the archive does not ship it"


def test_linux_archive_is_complete():
    archives = _linux_archives()
    if not archives:
        pytest.skip(f"no Linux launcher archive in {ARTIFACTS}")
    archive = archives[0]
    if archive.suffix == ".zst":
        names = _zstd_members(archive)
    else:
        with tarfile.open(archive) as tar:
            names = tar.getnames()
    assert any(name.endswith("/horizon-launcher") for name in names), "no launcher engine"
    assert any(name.endswith("/scoria-ui") for name in names), "no UI"
    assert any(name.endswith("/gpuix-native.node") for name in names), "no GPUIX addon"
    wrapper = next((name for name in names if name.endswith("/scoria-launcher-ui")), None)
    assert wrapper, "no scoria-launcher-ui wrapper (the addon path would not be set)"


def _zstd_members(archive: Path):
    if not _has("zstd"):
        pytest.skip("zstd is not installed")
    with subprocess.Popen(
        ["zstd", "-dc", str(archive)], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL
    ) as decoder:
        with tarfile.open(fileobj=decoder.stdout, mode="r|") as tar:
            return [member.name for member in tar]


def _has(command: str) -> bool:
    return subprocess.run(
        ["sh", "-c", f"command -v {command}"], capture_output=True
    ).returncode == 0


def test_checksums_cover_every_archive():
    sums = ARTIFACTS / "SHA256SUMS"
    if not sums.is_file():
        found = sorted(ARTIFACTS.glob("**/SHA256SUMS"))
        if not found:
            pytest.skip(f"no SHA256SUMS in {ARTIFACTS}")
        sums = found[0]
    entries = {}
    for line in sums.read_text().splitlines():
        match = re.fullmatch(r"([0-9a-f]{64})  (.+)", line.strip())
        assert match, f"malformed SHA256SUMS line: {line!r}"
        entries[match.group(2)] = match.group(1)
    for name in entries:
        path = sums.parent / name
        assert path.is_file(), f"SHA256SUMS lists {name}, which is not beside it"
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        assert digest == entries[name], f"{name} does not match SHA256SUMS"


def _game_zip() -> Path:
    direct = ARTIFACTS / "horizon-windows-x64.zip"
    if direct.is_file():
        return direct
    found = sorted(ARTIFACTS.glob("**/horizon-windows-x64.zip"))
    if not found:
        pytest.skip(f"no Windows game archive in {ARTIFACTS}")
    return found[0]


def test_windows_game_archive_has_the_client_and_clean_imports(tmp_path):
    with zipfile.ZipFile(_game_zip()) as archive:
        archive.extractall(tmp_path)
    check = _check_imports()
    shipped = {path.name.lower() for path in tmp_path.rglob("*") if path.is_file()}
    client = next(tmp_path.rglob("horizon-client.exe"), None)
    assert client, "horizon-client.exe is missing from the Windows game archive"
    binaries = [
        path
        for path in tmp_path.rglob("*")
        if path.is_file() and path.suffix.lower() in {".exe", ".dll"}
    ]
    for binary in binaries:
        missing = [
            n
            for n in check.pe_imports(binary.read_bytes())
            if not check.windows_allowed(n, shipped)
        ]
        assert not missing, f"{binary.name} imports {missing}, which Windows does not ship"


def test_linux_game_archive_has_the_client():
    archives = sorted(ARTIFACTS.glob("horizon-linux-x86_64.tar.*")) + sorted(
        ARTIFACTS.glob("**/horizon-linux-x86_64.tar.*")
    )
    if not archives:
        pytest.skip(f"no Linux game archive in {ARTIFACTS}")
    archive = archives[0]
    if archive.suffix == ".zst":
        names = _zstd_members(archive)
    else:
        with tarfile.open(archive) as tar:
            names = tar.getnames()
    assert any(name.endswith("/horizon-client") for name in names), "no horizon-client"
