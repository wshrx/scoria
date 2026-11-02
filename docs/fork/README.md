# Scoria

Scoria is a fork of [Horizon](https://github.com/horizon-rust/horizon): Minecraft: Java Edition 26.3
written in Rust, with a launcher that plays without a Microsoft account.

- **Upstream:** `upstream` = `https://github.com/horizon-rust/horizon.git`. Never commit to it.
- **This repository:** `origin` = Entire `/et/redmoon/scoria` (everything), `github` =
  `git@github.com:wshrx/scoria.git` (`main` and `staging` only).
- **Licence:** MIT, as upstream (see `LICENSE` and `THIRD_PARTY_NOTICES.md`). Scoria's own changes
  are MIT too. Minecraft is a trademark of Mojang Synergies AB; Scoria is unofficial and ships no
  Mojang code, assets or data.

## What Scoria changes

| Area | Horizon | Scoria |
|---|---|---|
| Starting the game | requires a signed-in Microsoft account that owns Java Edition (the "owner gate") | any offline name, no account needed |
| Microsoft sign-in | required before Play | **removed**: no sign-in, no tokens, no online-mode |
| Game files | can be downloaded from Mojang for an owner | prepared from an installation you already have, or downloaded from piston-meta when none has 26.3 |
| Launcher UI | Rust + GPUI | **gpui-ce 0.3.0** in `launcher/ui/` + `launcher/ui-kit/`, one `scoria` binary in the same process; custom Wayland titlebar, HeroUI-shaped Ember theme |
| Telemetry | opt-in, shipped | removed |
| Mod registry, nightly channel | shipped | Horizon registry first + Modrinth in the window; no nightly channel |
| Self-update | from Horizon's GitHub releases | disabled; Scoria releases are separate and manual for now |

The scene is unchanged: Scoria is Horizon's engine, parity harnesses and world format. Scoria does
not patch the game to accept unauthenticated sessions on online-mode servers. Offline play is
local: single player, LAN and offline-mode servers, exactly as Horizon already supports without a
session file: there is no account service left in the fork.

## Documents

- [offline-mode.md](offline-mode.md): how to play without an account, and what still needs one.
- [architecture.md](architecture.md): what is where, what P0/P1 touched, what the fork depends on.
- [adr/0001-offline-first.md](adr/0001-offline-first.md): why the owner gate goes and what stays.
- [adr/0002-gpui-x-hybrid.md](adr/0002-gpui-x-hybrid.md): GPUIX analysis (historical).
- [adr/0003-launcher-ui-flint-ui.md](adr/0003-launcher-ui-flint-ui.md): the wait for flint-ui
  (historical, superseded by the gpui-ce rewrite).
- [adr/0006-flint-ui-gpuix.md](adr/0006-flint-ui-gpuix.md): historical flint-ui plan.
- [adr/0007-remove-microsoft.md](adr/0007-remove-microsoft.md): no Microsoft account at all.
- [adr/0004-rename-to-scoria.md](adr/0004-rename-to-scoria.md): Basalt becomes Scoria.
- [adr/0005-two-remotes.md](adr/0005-two-remotes.md): Entire keeps everything, GitHub gets
  `main` and `staging`.
- [branches.md](branches.md): `main`, `dev`, `staging` and the merge/test rules.
- [patches.md](patches.md): the 7 engine patches and the blocked upstream sync.
- [sources.md](sources.md): `sources.json` for Scoria's own builds on your server.
- `../adr/0001-gpui-ce-rewrite.md`: the current launcher stack.
- Upstream docs stay authoritative for the engine: [`docs/README.md`](../README.md),
  [`AGENTS.md`](../../AGENTS.md), [`docs/parity-workflow.md`](../parity-workflow.md).

## Build

```sh
cargo xtask gate                       # launcher fmt, clippy, tests, pytest
cargo xtask ui                         # run the window
cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar   # JDK 25
cargo run --release -p horizon-client
```

The launcher is its own workspace at `launcher/`; the window is `launcher/ui/`.

## Status

- **P0–P2 (done):** fork and rebrand, owner gate gone, offline names, telemetry and the nightly
  channel removed, piston-meta downloads, Modrinth source, Scoria's signed releases.
- **P3 (done):** the launcher is one Rust binary on gpui-ce: `launcher/ui/` (window),
  `launcher/ui-kit/` (tokens/components), `launcher/xtask/` (check/test/pytest/gate/ui). The
  React/Bun/flint-ui line and the native addon are gone.
- **Beta (done):** `v0.1.0-beta` is a normal GitHub Release with only the launcher; the engine
  bundle is a separate pre-release, `v0.1.0-beta-game`.
- **Engine sync (blocked):** upstream `main` is 9 770 commits ahead; the 7 patches do not apply
  cleanly yet. See [patches.md](patches.md).
- **Next:** re-apply the engine patches on upstream (Trail), then a Windows smoke test, then an
  in-app engine update button and `v0.1.1-beta`.
