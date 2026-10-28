# Scoria

Scoria is a fork of [Horizon](https://github.com/horizon-rust/horizon): Minecraft: Java Edition 26.3
written in Rust, with a launcher that plays without a Microsoft account.

- **Upstream:** `upstream` = `https://github.com/horizon-rust/horizon.git`. Never commit to it.
- **This repository:** `origin` = `git@github.com:wshrx/scoria.git` (GitHub).
- **Licence:** MIT, as upstream (see `LICENSE` and `THIRD_PARTY_NOTICES.md`). Scoria's own changes
  are MIT too. Minecraft is a trademark of Mojang Synergies AB; Scoria is unofficial and ships no
  Mojang code, assets or data.

## What Scoria changes

| Area | Horizon | Scoria |
|---|---|---|
| Starting the game | requires a signed-in Microsoft account that owns Java Edition (the "owner gate") | any offline name, no account needed (P1) |
| Microsoft sign-in | required before Play | optional: online-mode servers, cloud skins |
| Game files | can be downloaded from Mojang for an owner | prepared from an installation you already have; only the 26.3 server jar is fetched once, for data generation |
| Launcher UI | Rust + GPUI | stays on GPUI for now; moves to Redmoon's **flint-ui** when it exists (ADR-0003) |
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
- [adr/0003-launcher-ui-flint-ui.md](adr/0003-launcher-ui-flint-ui.md): the launcher UI waits for
  flint-ui.
- [adr/0004-rename-to-scoria.md](adr/0004-rename-to-scoria.md): Basalt becomes Scoria.
- [branches.md](branches.md): `main`, `dev`, `nightly` and the merge/test rules.
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
- P3 (paused): the launcher UI moves to Redmoon's **flint-ui** once it has code; the GPUIX plan
  is deferred (ADR-0003). P4: full RU/EN documentation. P5: Scoria's own signed releases.
