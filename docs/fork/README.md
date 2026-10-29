# Scoria

Scoria is a fork of [Horizon](https://github.com/horizon-rust/horizon): Minecraft: Java Edition 26.3
written in Rust, with a launcher that plays without a Microsoft account.

- **Upstream:** `upstream` = `https://github.com/horizon-rust/horizon.git`. Never commit to it.
- **This repository:** `origin` = Entire `/et/redmoon/scoria` (everything), `github` =
  `git@github.com:wshrx/scoria.git` (`main` and `nightly` only).
- **Licence:** MIT, as upstream (see `LICENSE` and `THIRD_PARTY_NOTICES.md`). Scoria's own changes
  are MIT too. Minecraft is a trademark of Mojang Synergies AB; Scoria is unofficial and ships no
  Mojang code, assets or data.

## What Scoria changes

| Area | Horizon | Scoria |
|---|---|---|
| Starting the game | requires a signed-in Microsoft account that owns Java Edition (the "owner gate") | any offline name, no account needed |
| Microsoft sign-in | required before Play | **removed**: no sign-in, no tokens, no online-mode |
| Game files | can be downloaded from Mojang for an owner | prepared from an installation you already have, or downloaded from piston-meta when none has 26.3 |
| Launcher UI | Rust + GPUI | **flint-ui 0.1.2 on gpuix 0.11.0** (`launcher-ui/`, ADR-0006); modular screens, Modrinth mods, custom frameless chrome |
| Telemetry | opt-in, shipped | removed |
| Mod registry, nightly channel | shipped | removed; Scoria's own registry later |
| Self-update | from Horizon's GitHub releases | disabled until Scoria has its own releases |

The scene is unchanged: Scoria is Horizon's engine, parity harnesses and world format. Scoria does
not patch the game to accept unauthenticated sessions on online-mode servers. Offline play is
local: single player, LAN and offline-mode servers, exactly as Horizon already supports without a
session file (`docs/microsoft-account.md`, "Handing the session to the game").

## Documents

- [offline-mode.md](offline-mode.md): how to play without an account, and what still needs one.
- [architecture.md](architecture.md): what is where, what P0/P1 touched, what the fork depends on.
- [adr/0001-offline-first.md](adr/0001-offline-first.md): why the owner gate goes and what stays.
- [adr/0002-gpui-x-hybrid.md](adr/0002-gpui-x-hybrid.md): GPUIX analysis (timing superseded).
- [adr/0003-launcher-ui-flint-ui.md](adr/0003-launcher-ui-flint-ui.md): the wait for flint-ui
  (superseded).
- [adr/0006-flint-ui-gpuix.md](adr/0006-flint-ui-gpuix.md): the launcher UI is flint-ui on gpuix.
- [adr/0007-remove-microsoft.md](adr/0007-remove-microsoft.md): no Microsoft account at all.
- [adr/0004-rename-to-scoria.md](adr/0004-rename-to-scoria.md): Basalt becomes Scoria.
- [adr/0005-two-remotes.md](adr/0005-two-remotes.md): Entire keeps everything, GitHub gets
  `main` and `nightly`.
- [branches.md](branches.md): `main`, `dev`, `nightly` and the merge/test rules.
- [sources.md](sources.md): `sources.json` for Scoria's own builds on your server.
- Upstream docs stay authoritative for the engine: [`docs/README.md`](../README.md),
  [`AGENTS.md`](../../AGENTS.md), [`docs/parity-workflow.md`](../parity-workflow.md).
- Russian translations land in `docs/fork/ru/` (P4 of the plan).

## Build (same as upstream)

```sh
cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar   # JDK 25
cargo run --release -p horizon-client
cd launcher && cargo run
```

The launcher is its own workspace. Scoria's copy is at `/home/shrx/dev/scoria/launcher`.

## Status

- **P0 (done):** fork, rebrand, upstream infrastructure detached, ADRs.
- **P1 (done):** the owner gate is gone. `account::launch` never refuses, offline names need no
  account, the owner screen is deleted, and `horizon-launcher play` / MCP `play` start without one.
- **P2 (in progress):** the Mojang game download is gone (`--download`, `download_for_owner`,
  `mojang::download`); Freesm/Prism are auto-detected and their Java 25 is reused. Telemetry is
  removed from the launcher (consent screen, `data` command, ping, benchmark upload) and the game
  is launched with `HORIZON_TELEMETRY_URL=off`. The Nightly channel is gone too: no
  `nightly.rs`, no setting, no Nightly instance. Still to do: the remaining registry code, then
  CI.
- P3 (started): `launcher-ui/` runs on **gpuix 0.11.0 + flint-ui 0.1.2** (ADR-0006); Home,
  Instances and Settings render real state through `horizon-launcher api` (JSON lines);
  Microsoft is removed from the Rust launcher (ADR-0007). Next: packaging the UI with the engine
  and the remaining screens. P4: full RU/EN documentation. P5: own signed releases.
