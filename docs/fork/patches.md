# Fork patches and upstream sync

Scoria is Horizon's engine plus a Scoria launcher and UI. The smaller the engine diff, the cheaper
every upstream update. This file is the list to keep honest.

Base: `b53cfcd14` (upstream `main`, 2026-11-02). The engine sync landed on `sync/upstream-20261102`
and merged into `dev`; the engine is **current** with that base and 0 commits behind. The engine
delta is now two patches in `docs/fork/engine-patches/`.

## Engine (`crates/`) — 2 patches

1. **Non-Latin input and offline engine** — `typed_text`/`typed_character` use `KeyEvent.text` and
   fall back to the logical key for Cyrillic and other layouts; chat, books, command blocks, the
   LAN screen, the mod browser, the recipe book, sign edit, the startup menu and Options pass the
   result through. The same patch removes the Microsoft session options (`--session-file`,
   `--session-stdin`, `HORIZON_SESSION_FILE`), the Launch API entries, and changes the online-mode
   text to Scoria. Test: `cargo test -p horizon-client --lib a_cyrillic_logical_key_is_text`.
2. **Brand** — the Scoria release key in `crates/horizon-release-sign/release-keys.txt` and the
   Scoria icon.

Upstream now knows `native-mod` / `.hnmod` itself (`Kind::NativeMod`), so Scoria's old alias was
dropped at this sync; the test in `crates/horizon-mods/src/index/tests.rs` proves the mapping.

Nothing else in `crates/` differs from upstream. Do not rename crates, reformat upstream files or
move modules: those turn every merge into a conflict.

## Launcher (`launcher/`, `launcher/ui/`, `launcher/ui-kit/`, `launcher/xtask/`)

This is the real fork and the source of most conflicts:

- Microsoft accounts and sessions removed; offline names (Unicode) in `account.rs`.
- Telemetry removed.
- piston-meta/piston-data download, retries, IPv4-first DNS (`piston.rs`, `mojang.rs`).
- Modrinth source and ported-mod install (`browser/`).
- JSON API: worlds keyed by folder + `lastPlayedAt`, `playing`/`playingPid`, `running.rs` lock.
- The window is `launcher/ui/` on gpui-ce with `launcher/ui-kit/`; `launcher/xtask/` orchestrates.
  Upstream's GPUI/Bun window must never replace them.

## Sync runbook

```sh
sh tools/check_upstream_patches.sh   # before: the two patches apply to upstream/main
sh tools/sync_upstream.sh            # fetch, branch, merge, run the gates; never pushes
```

1. Run it weekly and before a release.
2. Branch from `upstream/main`, merge `dev`, resolve only in the sync branch.
3. `crates/` takes upstream; re-apply the two patches on top.
4. `launcher/` is ours: restore it from `dev` wholesale, then fix only API changes the new engine
   requires (the 2026-11-02 sync needed `horizon_prepare::Options.inputs`/`output` and
   `Kind::NativeMod`).
5. Run `check_fork_surfaces.sh`, the launcher tests, `cargo test -p horizon-client --lib`, and the
   Cyrillic test.
6. Regenerate `docs/fork/engine-patches/` from the new base and record it here and in
   `PROJECT_STATE.md`.

## What a sync may and may not touch

- **`crates/`** — upstream owns the code; our changes are the two patches. Take upstream, re-apply.
- **`launcher/` and the UI crates** — ours. Never `git checkout upstream/main -- launcher`; that
  replaces the Rust UI with Horizon's and loses the offline launcher.
- **Docs, packaging, release workflow** — ours.
- Upstream occasionally carries stray generated files (`argparse`, `subprocess`, `sys` were
  PostScript dumps at this base); delete them instead of committing them to Scoria.

## Rules

- Engine patches: one reason per patch, small, with a test, upstreamable where possible.
- No engine refactors, renames or formatting commits.
- Features that need deep engine changes: prefer a module/feature flag or an upstream PR over a
  long-lived fork.
