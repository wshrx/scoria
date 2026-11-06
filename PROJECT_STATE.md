# Scoria project state

Read this first after a context loss. Keep it current; this is the state, not a changelog.
Last updated: 2026-11-06.

## What this is

Scoria is an offline-first Minecraft: Java Edition 26.3 client and launcher written in Rust.
It is a fork of Horizon (MIT): the engine comes from upstream, Scoria adds offline play, the
gpui-ce launcher, piston-meta downloads, Modrinth, its own releases and branding. No Microsoft
account, no telemetry, no online-mode servers, no Minecraft assets.

- `origin` = Entire `et/redmoon/scoria` (working remote); `github` = `wshrx/scoria` (public
  showcase); `upstream` = Horizon (fetch-only).
- Entire branches: **`dev`** and **`main`**, both at the same commit. There are no other remote
  branches and no stale trails. Work happens on `dev`; `main` fast-forwards after a green gate.
- GitHub branches: `staging` and `main`; both follow the exported snapshot. Public CI runs on
  `staging`, then `main` fast-forwards and the release tag is cut.
- Do not push without the user's word. Entire is the draft; GitHub receives allowlisted snapshots
  via `tools/export_public.sh` + `tools/publish_public.sh`. Never push `.opencode/`,
  `PROJECT_STATE.md`, `docs/agents/` or internal fixtures there.

## Current release

- **Target `v0.1.3-beta`**: the D · Signal redesign, E · White light tokens, Stop action and the
  prepared launcher fixes below. Not tagged yet; push is blocked by Entire's region error.
- **GitHub `v0.1.2-beta`** is a normal Release (not pre-release) and the Latest release. It carries
  only the launcher: `scoria-launcher-linux-x86_64.tar.zst`,
  `scoria-launcher-windows-x64.zip`, `SHA256SUMS`, `SHA256SUMS.sig`.
- **GitHub `v0.1.2-beta-game`** is a pre-release titled “do not download”. It carries the engine
  bundles `scoria-game-linux-x86_64.tar.zst` / `scoria-game-windows-x64.zip` and their signed
  checksums. The launcher finds this release and installs the bundle itself.
- Tags and releases are immutable per built artifact: `v0.1.2-beta` carries the synced engine.
- The release workflow splits the assets, signs each release with its own tag, and recreates the
  release so an old asset order cannot survive. The README (en/ru) tells players to download only
  `scoria-launcher-*`.

## Launcher

- One Rust binary on gpui-ce 0.3.0. `scoria-core` is the old `horizon-launcher` library;
  `scoria-ui-kit` owns tokens and reusable components (`page_header`, `empty_state`,
  `error_panel`, `progress_bar`, cards, buttons, badges, toasts); `scoria-ui` owns the window,
  split into `app.rs` (state/actions/QA), `app/screens/` (one file per screen) and `assets.rs`.
- The titlebar shows **Scoria + the launcher version** (`0.1.1-beta`). The engine build tag is not
  shown anywhere: Scoria has one public versioning.
- Screens: Home in the D · Signal layout (topbar, no rail, 640 px hero, one action
  Download/Preparing/Play/Stop, selected-world poster, 64 px world rows), Mods (Horizon registry first, Modrinth second, tabs, loading
  skeleton, retry card), Servers (Join and Remove per row), Settings (appearance, language en/ru,
  offline nickname). The rail collapses and the layout adapts below 1080 px.
- The window watches the selected instance (`saves`, `mods`, `shaderpacks`, `config`,
  `screenshots`, `options.txt`) and the launcher's servers/settings files, debounced to 350 ms.
- Mimas scripts are an **opt-in feature** (`cargo build -p scoria-ui --features scripts`), with
  hooks for start/game start/game exit/download done and a small host API; docs/mimas-scripts.md.
- The window and the CLI share `launcher/lang/{ru,fr}.json` through `scoria_core::i18n::tr_in`;
  a test scans both crates for `tr`/`t` calls and fails on a missing or unused key.
- `SCORIA_UI_STATE` paints 19 synthetic states without network or game; `tools/capture_ui_states.py`
  captures them to `~/Pictures/scoria-ui-states/` with a blank-frame check. See
  `docs/qa/launcher-ui-states.md`. A visual change is not done until the states are looked at.
- Home installs the newest engine bundle before preparing; progress and failures go to `ui.log`.
- Auto-update is deliberately off (`self_update_allowed() == false`). Engine updates have a separate
  release but no in-app “Update engine” button yet.
- Linux: `.tar.zst`; Windows: one `scoria.exe` with `+crt-static`. Windows runtime is still untested
  on real hardware.

## Tests and commands

```sh
cargo xtask check     # fmt + clippy for the launcher workspace
cargo xtask test      # nextest: core + ui-kit + ui
cargo xtask pytest    # uv + pytest under tests/pytest
cargo xtask gate      # everything CI runs
cargo xtask ui        # run the window in development
```

Current gate: **228 Rust tests** (2 skipped) and **15 pytest cases** in 3 files green. Every fixed bug
gets a test that failed before the fix.

## Engine sync (done 2026-11-02)

- The sync branch merged `dev` onto `upstream/main` = `b53cfcd14` (2026-11-02) and was deleted
  after the merge. The engine is **0 commits behind** that base.
- The engine delta is now 2 patches in `docs/fork/engine-patches/`: non-Latin input + offline
  (no Microsoft session options), and the Scoria release key/icon. Both apply cleanly to
  `b53cfcd14`; `tools/check_upstream_patches.sh` passes.
- The launcher stays Scoria's: the sync restored `launcher/` from `dev` and fixed only the engine
  API changes (`horizon_prepare::Options.inputs`/`output`, `Kind::NativeMod`).
- Upstream now knows `native-mod`/`.hnmod` itself, so Scoria's old alias was dropped.
- Verified: `check_fork_surfaces.sh`, `cargo check -p horizon-client`, `cargo test -p
  horizon-client --lib` (694 passed), launcher `cargo test` (204 passed), the Cyrillic and
  native-mod tests.
- Stray upstream files `argparse`, `subprocess`, `sys` (PostScript dumps) and tracked launcher
  logs were removed at this sync.

## Design experiments

- `exp/redesign` — Blueprint (cool paper/ink, safety orange).
- `exp/redesign-brawl` — Arcade (yellow/navy/red, chunky).
- `exp/redesign-dungeons` — Dungeons (moss/amber/parchment, one rounded window shell, custom
  titlebar, rounded inner controls; Wayland `app_id=scoria` + Scoria panel icon).
- `exp/gpui-fast` — gpui-fast 0.1.13 comparison; no measurable win on this UI and +16 MB RSS.
- None is merged. Pick one and promote it as `feat/redesign-*`, with tokens, tests and screenshots.
  ADR-0003 records the rule: one rounded window shell, custom titlebar, rounded inner controls,
  Wayland identity (`scoria.desktop` / `Icon=scoria`), original assets.

## Shaders

- `test/shaders` holds the `vibrant` `.hshader` smoke pack and checklist.
- Supported: Horizon `.hshader` schema 1. OptiFine/Java shader packs are not a supported renderer
  path. Vanilla Java 26.3 has no “Vibrant Visuals” (a Bedrock feature); the test pack is original.

## Known gaps

- Windows: release artifacts build in CI, but first-run download/prepare/play has not been tested on
  a Windows machine. Ask for `%LOCALAPPDATA%\horizon-launcher\ui.log` and `crash.log` if it stalls.
- macOS: deferred until signing and notarization.
- `.hnmod` / `native-mod` files now have upstream's `Kind::NativeMod` and install; a runtime smoke
  test of an installed native mod is still pending.
- Engine update button, launcher auto-update, and macOS signing are not implemented.
- Modpacks and shaders: engine support exists; no launcher screen for them. CurseForge unsupported.

## Product rules

- Do not rename `horizon-*` crates; the engine stays mergeable with upstream. Scoria branding lives
  in the launcher, UI, packages and docs.
- One Settings entry, one state-aware Play action, 4 px spacing, 8 px cards, 12 px fields, matte
  buttons, readable contrast in both themes.
- No Minecraft art or assets in the repository or releases. Game files come from Mojang at prepare
  time or from the player's own install.
- Never move or reuse a release tag: a bad build is deleted and the next beta number is used.
