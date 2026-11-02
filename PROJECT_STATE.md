# Scoria project state

Read this first after a context loss. Keep it current; this is the state, not a changelog.
Last updated: 2026-11-02.

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

- **GitHub `v0.1.0-beta`** is a normal Release (not pre-release) and the Latest release. It carries
  only the launcher: `scoria-launcher-linux-x86_64.tar.zst`,
  `scoria-launcher-windows-x64.zip`, `SHA256SUMS`, `SHA256SUMS.sig`.
- **GitHub `v0.1.0-beta-game`** is a pre-release titled “do not download”. It carries the engine
  bundles `scoria-game-linux-x86_64.tar.zst` / `scoria-game-windows-x64.zip` and their signed
  checksums. The launcher finds this release and installs the bundle itself.
- Tags and releases are immutable per built artifact: the next public build is `v0.1.1-beta`.
- The release workflow splits the assets, signs each release with its own tag, and recreates the
  release so an old asset order cannot survive. The README (en/ru) tells players to download only
  `scoria-launcher-*`.

## Launcher

- One Rust binary on gpui-ce 0.3.0. `scoria-core` is the old `horizon-launcher` library;
  `scoria-ui-kit` owns tokens and components; `scoria-ui` owns the window.
- The titlebar shows **Scoria + the launcher version** (`0.1.0-beta`). The engine build tag is not
  shown anywhere: Scoria has one public versioning.
- Screens: Home (prepare progress + log line, Play), Mods (Horizon registry first, Modrinth second,
  tabs), Servers, Settings (appearance, language en/ru, offline nickname). The rail collapses and
  the layout adapts below 1080 px.
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

Current gate: **225 Rust tests** (2 skipped) and **15 pytest cases** in 3 files green. Every fixed bug
gets a test that failed before the fix.

## Engine sync (blocked)

- `upstream/main` = `b53cfcd14` (2026-11-02); Scoria is **9 770 commits behind** and 163 ahead on
  `dev`.
- The fork's engine delta is the 7 patches in `docs/fork/engine-patches/`: typed non-Latin input,
  Scoria icons/motion mark, the Scoria release key, Scoria wording, and removal of the Microsoft
  account/session surfaces.
- `tools/check_upstream_patches.sh` does not pass: patch `0001` conflicts on `game.rs`,
  `command_blocks.rs`, `lan_screen.rs`, `native_panel.rs` (upstream renamed it `mod_browser.rs`)
  and `recipe_book.rs`.
- A merge was attempted in a throwaway worktree (`sync/upstream-20261102`) and removed without
  pushing. The conflict list is known: 7 engine files, plus launcher/workflow/docs conflicts (keep
  Scoria's launcher and deletions, take upstream for `crates/`, then re-apply the 7 patches).
- Do the sync as a Trail, not on `dev`: branch from `upstream/main`, merge `dev`, resolve, re-apply
  the patches, run `check_fork_surfaces.sh`, the Launcher gate, and the client Cyrillic test.

## Known gaps

- Windows: release artifacts build in CI, but first-run download/prepare/play has not been tested on
  a Windows machine. Ask for `%LOCALAPPDATA%\horizon-launcher\ui.log` and `crash.log` if it stalls.
- macOS: deferred until signing and notarization.
- `.hnmod` / `native-mod` files install in the launcher, but the engine does not run them until the
  engine sync lands.
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
