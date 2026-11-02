# Changelog

All notable changes to Scoria. The release workflow takes the section whose heading matches the
tag (`v0.1.1-beta` → `## [0.1.1-beta]`) and uses it as the GitHub Release body, so the notes
live here, in one place, and not in tag names.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html): during the beta every published build
raises the patch and keeps the `-beta` suffix (`0.1.0-beta`, `0.1.1-beta`, ...).

## [Unreleased]

### Added

- The launcher UI is being rewritten in Rust on gpui-ce: one process, no Bun, no native addon.
  `scoria-ui-kit` holds the HeroUI-shaped tokens and components, `scoria-ui` the window.
- `cargo xtask` is the single entry point for checks, tests, pytest and the UI.
- `uv` + pytest for the black-box suite; `pyproject.toml` pins the environment.
- Settings edits the offline name through the engine's accounts file, with the same
  3–16 letter validation as the CLI.

## [0.1.1-beta] — 2026-11-01

### Fixed

- `Scoria.cmd` runs the UI in the console, writes `scoria-ui.log` and pauses with the exit code, so
  a launcher that closes on Windows says why.
- Windows ships the published GPUIX addon with `vcruntime140.dll` again: the from-source static-CRT
  addon is not proven on a real Windows machine and is opt-in until it is.

### Added

- `tests/test_release_artifacts.py`: checks archive layout, `Scoria.cmd`, every Windows binary's DLL
  imports and the checksums before a release is published.

## [0.1.0-beta] — 2026-11-02

The launcher is one Rust binary on gpui-ce. Bun, the GPUIX native addon and every
`.exe`/`.node`/`vcruntime140.dll` combination are gone; a launcher archive is a single
`scoria` (`scoria.exe`) with the fonts and the mark inside it.

### Added

- **One process.** The window calls the engine directly; the JSON-lines API stays for the CLI
  and MCP.
- **Settings** edits the offline name through the engine's account file, and switches the Ember
  theme and the language live.
- **Home** shows the selected version, whether resources are prepared, the world count and
  starts the game, with the engine's error under the button.
- **Mods** searches Horizon's registry first and Modrinth second, labels each result with its
  source and support level, and installs with a background task.
- **Servers** lists, adds and removes offline-mode servers.
- A crash writes `crash.log`; the release build is a GUI binary with the Scoria icon and version
  resource on Windows.

### Changed

- **HeroUI-shaped design**: 4px spacing, 8px cards, 12px fields, one primary action, matte
  buttons, 120/180 ms transitions, dark by default and light at parity.
- The Linux archive is a `.tar.zst` of a 47 MB binary (14 MB compressed); Windows ships one
  `scoria.exe`; macOS follows after signing.
- `cargo xtask` is the single entry point: check, test, build, pytest, gate, ui.

### Removed

- `launcher-ui/` (React on GPUIX), the native addon, the Bun runtime, the wrapper script and
  `Scoria.cmd`.

### Known limitations

- Online-mode servers need a Microsoft session and are unsupported by design.
- macOS is deferred until signing and notarization.
- Windows builds are not code-signed yet: SmartScreen warns.
- CurseForge is not supported; the engine can import and port modpacks.

