# Changelog

All notable changes to Scoria. The release workflow takes the section whose heading matches the
tag (`v0.1.0-beta` → `## [0.1.0-beta]`).

## [0.1.2-beta] — 2026-11-02

The first engine sync since the beta. The game now runs Horizon's `main` at `b53cfcd14`
(9 770 commits newer) with Scoria's engine changes re-applied, and the launcher stays Scoria's.

### Changed

- Non-Latin typing still works on every text field: the game reads `KeyEvent.text` and falls back to
  the logical key for Cyrillic and other layouts.
- No Microsoft session anywhere: `--session-file`, `--session-stdin`, `HORIZON_SESSION_FILE` and
  their Launch API entries are gone. Online-mode servers are refused with the Scoria message.
- `native-mod` / `.hnmod` is upstream's `Kind::NativeMod` now; Scoria's old alias is gone.
- The launcher keeps its gpui-ce window, offline profiles, piston-meta downloads and Modrinth
  browser. Two engine API changes were adapted (`horizon_prepare::Options`, `Kind::NativeMod`).

### Known limitations

- Windows first-run download/prepare/play is still untested on real hardware.
- A runtime smoke test of an installed `.hnmod` is still pending.
- macOS is deferred until signing/notarization.

## [0.1.0-beta] — 2026-11-02

The launcher is one Rust binary on gpui-ce. Bun, the GPUIX native addon and every
`.exe`/`.node`/`vcruntime140.dll` combination are gone; a launcher archive is a single
`scoria` (`scoria.exe`) with the fonts and the mark inside it.

### Added

- **One process.** The window calls the engine directly; the JSON-lines API stays for the CLI
  and MCP.
- **Settings** edits the offline name through the engine's account file, and switches the Ember
  theme and the language live.
- **Home** shows whether resources are prepared and the world count, and starts the game, with
  the engine's error under the button.
- **Mods** searches Horizon's registry first and Modrinth second, labels each result with its
  source and support level, and installs with a background task.
- **Servers** lists, adds and removes offline-mode servers.
- A crash writes `crash.log`; the release build is a GUI binary with the Scoria icon and version
  resource on Windows.

### Changed

- **HeroUI-shaped design**: 4px spacing, 8px cards, 12px fields, one primary action, matte
  buttons, 120/180 ms transitions, dark by default and light at parity.
- The launcher's own version sits beside the Scoria name in the titlebar. The engine build tag is
  no longer shown: players see one version, the launcher's.
- Settings dropdowns expand in place instead of covering the next field.
- The engine bundle has its own pre-release (`v0.1.0-beta-game`). The player-facing release lists
  only the launcher archives, so there is nothing to choose by mistake; the launcher installs the
  engine itself.
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

