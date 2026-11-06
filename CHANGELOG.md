# Changelog

All notable changes to Scoria. The release workflow takes the section whose heading matches the
tag (`v0.1.0-beta` → `## [0.1.0-beta]`).

## [Unreleased]

### Fixed

- Home has one state-aware action: Download 26.3, Preparing, or Play, with a progress bar and the
  engine's last log line instead of a text-only percentage.
- Leaving the game clears "Playing": the launcher polls while a game runs, and a zombie or the
  launcher's own recycled PID no longer counts as a live game.
- Horizon Visuals: calmer sky haze and sun shafts, clamped before the composite.

### Changed

- The download is a card: a title, the engine's last line, a large mono percentage, a full-width
  progress track and a quiet Cancel, instead of a 360 px bar squeezed beside the button.
- The window is split into `app.rs`, `app/screens/` and `assets.rs`; `scoria-ui-kit` owns page
  headers, empty/error states and the progress bar.
- The window uses the launcher's existing `launcher/lang/{ru,fr}.json` tables through
  `scoria_core::i18n`, so the CLI and the UI share one translation source; French is selectable
  again. Engine lines and errors pass through the same lookup and fall back unchanged.
- Settings gains a Sidebar control (expanded/compact), and screen changes fade/rise for 160 ms.
- Keyboard: `Ctrl+1..4` switches screens, `Ctrl+Enter` runs the one game action and `Esc` closes
  the appearance/language menus; server rows gained a hover state.
- The window watches `saves/`, `mods/`, `shaderpacks/`, `config/`, `screenshots/`, `options.txt`
  and the launcher's `servers.json`/`settings.json`, so a world or pack written by the game (or a
  dev build) appears in about 350 ms without a restart. Toasts now stack up to three instead of
  replacing each other.

### Added

- Opt-in Mimas scripts (`--features scripts`) with `on_start`, `on_game_start`, `on_game_exit`
  and `on_download_done` hooks and `toast`/`log`/`play`/`download`/`screen` host functions; see
  `docs/mimas-scripts.md`.
- Every launcher screen has a capturable state: `SCORIA_UI_STATE` paints synthetic data and
  `tools/capture_ui_states.py` saves all 18 states as PNGs (docs/qa/launcher-ui-states.md).
- Home shows the game state honestly (Not installed, Not prepared, Prepared, Playing), errors in a
  card with Try again, and world tiles with the mode and a key-art placeholder.
- Mods show a skeleton while loading and a retry card on failure; servers can be joined directly,
  and the server error path has a retry card.
- Mods and servers no longer repeat an author that is also the source.

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

