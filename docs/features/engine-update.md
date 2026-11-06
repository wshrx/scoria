# Engine update, without replacing the launcher

Branch: `feat/engine-update`. Implements the first automatic update path after the 2026-11-02 sync.

## Goal

A player on `v0.1.2-beta` with `v0.1.2-beta-game` installed opens the launcher after a newer engine
release exists and sees **Update engine** on Home. The launcher downloads the new `scoria-game-*`,
verifies its signed checksums, installs it as the selected version, keeps the already prepared 26.3
resources, and removes older engine folders except the current one and the one before it.

The launcher binary is not replaced.

## Design

- `backend::newest_installable()` already picks the newest `scoria-game-*`, including the `-game`
  pre-release. Add `backend::newer_engine(installed: &str) -> Result<Option<Release>, String>`.
- `api::engine_update()` returns the current tag and the offered tag; network errors become a
  quiet `None` on Home, not a toast on startup.
- `api::update_engine()` installs the offered tag through the existing signed path and calls
  `prune_versions(keep = 2)`.
- `prune_versions`: never delete the selected tag; keep the newest two installed tags; remove the
  rest with `backend::remove`.
- Home: a small accent badge/button appears only when an update is known. Pressing it runs the
  install with the same progress/log line as Download.
- Nothing happens automatically on a beta. A later Settings toggle can make it automatic.

## Tests

- `prune_versions` keeps the selected tag, the newest, and one previous; removes the rest.
- `newer_engine` compares tags (installed newest, older, equal) without network.
- `api::prepare_plan` still installs on a fresh machine.
- UI: Home shows the update action only when `engine_update` has a newer tag.
- Manual: install an older engine, publish/fake a newer one, press Update, check the old folder is
  pruned and 26.3 resources are reused.

## Out of scope

Launcher self-update, installers, delta downloads and Windows signing are separate features.
