# Fork patches and upstream sync

Scoria is Horizon's engine plus a Scoria launcher and UI. The smaller the engine diff, the cheaper
every upstream update. This file is the list to keep honest.

Base: `b6c936f82` (2026-10-28). On 2026-10-29 `upstream/main` = `9c15e18c6`:
**781 commits behind**, 55 Scoria commits ahead. A stale ref made an earlier check say "0 behind";
the fetch in `tools/sync_upstream.sh` corrected it. The engine patch is small, the drift is not.

## Engine (`crates/`) — 9 files, +87/−26

One patch: **non-Latin typing**.

- `crates/horizon-client/src/game.rs` — `typed_text()` and `typed_character()`: use
  `KeyEvent.text`, then fall back to `logical_key` when the backend has no text input.
- `game/{chat,books,command_blocks,lan_screen,mod_browser,recipe_book,sign_edit,startup_menu}.rs` —
  pass the result to the text fields.
- Test: `cargo test -p horizon-client --bin horizon-client cyrillic`.
- Upstreamable: yes. Send it as a Horizon PR; if merged, drop the patch at the next sync.

Nothing else in `crates/` differs from upstream. Do not rename crates, reformat upstream files or
move modules: those turn every merge into a conflict.

## Launcher (`launcher/`) — 44 files, +2168/−4210

The legacy GPUI window was removed on 2026-10-29: 33 GUI modules and `gpui` are gone, so this delta
is now the engine/CLI/API only. That removes 33 files from every future merge.

This is the real fork and the source of most conflicts:

- Microsoft accounts and sessions removed; offline names (Unicode) in `account.rs`.
- Telemetry removed (`telemetry.rs`, owner gate tests).
- piston-meta/piston-data download, retries, IPv4-first DNS (`piston.rs`, `mojang.rs`).
- Modrinth source and ported-mod install (`browser/`).
- JSON API: worlds keyed by folder + `lastPlayedAt`, `playing`/`playingPid`, `running.rs` lock.
- New UI screens and registry live in `launcher-ui/` (ours; no upstream).

## One-time large sync (781 upstream commits)

Do it as one deliberate port, not a blind merge:

1. `git switch -c sync/upstream-20261029 upstream/main`
2. Merge `dev` into it. Resolve in this order:
   - `.github/`, `.githooks/`, `AGENTS.md`, docs: keep Scoria's deletions/content.
   - `crates/`: take upstream. Upstream changed 399 engine files; 142 commits touch `game.rs` and
     `chat.rs`, where our patch lives. Re-apply the 9-file `typed_text` patch afterwards (it is one
     commit: `56d0f5d56`), then run the Cyrillic test.
   - `launcher/`: keep Scoria's offline/piston/Modrinth behaviour; port upstream launcher fixes
     selectively (55 files changed upstream).
3. Run `cargo test` in `launcher/`, the client Cyrillic test, and the UI gates.
4. Merge the sync branch into `dev`. Record the new base here.
5. Next sync is weekly; 781 is what happens when it waits.

## Engine patch queue

`docs/fork/engine-patches/*.patch` is a `git format-patch` series of every commit that touched `crates/`
(input, icons, release key, Microsoft removal). To rebuild the fork on a new upstream:

```sh
git switch -c sync/upstream-YYYYMMDD upstream/main
git am -3 docs/fork/engine-patches/*.patch     # resolve conflicts, keep order
sh tools/check_fork_surfaces.sh
```

The check fails if `typed_text`, the Scoria online-mode text, the offline-only server, the session
file removal, the release key or the icon disappear. It also runs at the start of
`tools/sync_upstream.sh`.

## Sync runbook

```sh
sh tools/sync_upstream.sh      # fetch, branch, merge, run the gates; never pushes
```

1. Run it weekly and before a release.
2. Resolve conflicts **in the sync branch**, never on `dev`.
3. If a conflict is in `crates/`, prefer upstream and re-apply our small patch on top.
4. If a conflict is in `launcher/`, keep our fork behaviour and re-run `cargo test`.
5. Merge the sync branch into `dev` only when all gates pass.
6. Record the new upstream commit here and in `PROJECT_STATE.md`.

## Rules

- Engine patches: one reason per patch, small, with a test, with an upstream PR if possible.
- No engine refactors, renames or formatting commits.
- Features that need deep engine changes: prefer a module/feature flag or an upstream PR over a
  long-lived fork.
- Launcher changes are free to diverge; UI changes cannot conflict.
