# Basalt

Basalt is a fork of [Horizon](https://github.com/horizon-rust/horizon): Minecraft: Java Edition 26.3
written in Rust, with a launcher that plays without a Microsoft account.

- **Upstream:** `upstream` = `https://github.com/horizon-rust/horizon.git`. Never commit to it.
- **This repository:** `origin` is added when the fork gets a home; until then it is local only.
- **Licence:** MIT, as upstream (see `LICENSE` and `THIRD_PARTY_NOTICES.md`). Basalt's own changes
  are MIT too. Minecraft is a trademark of Mojang Synergies AB; Basalt is unofficial and ships no
  Mojang code, assets or data.

## What Basalt changes

| Area | Horizon | Basalt |
|---|---|---|
| Starting the game | requires a signed-in Microsoft account that owns Java Edition (the "owner gate") | any offline name, no account needed |
| Microsoft sign-in | required before Play | optional: online-mode servers, cloud skins |
| Game files | can be downloaded from Mojang for an owner | never downloaded; prepared from an installation you already have |
| Launcher UI | Rust + GPUI | React/TypeScript on GPUIX (planned, see ADR-0002) |
| Telemetry | opt-in, shipped | removed |
| Mod registry, nightly channel | shipped | removed; Basalt's own registry later |
| Self-update | from Horizon's GitHub releases | disabled until Basalt has its own releases |

The scene is unchanged: Basalt is Horizon's engine, parity harnesses and world format. Basalt does
not patch the game to accept unauthenticated sessions on online-mode servers. Offline play is
local: single player, LAN and offline-mode servers, exactly as Horizon already supports without a
session file (`docs/microsoft-account.md`, "Handing the session to the game").

## Documents

- [architecture.md](architecture.md): what is where, what P0 touched, what the fork depends on.
- [adr/0001-offline-first.md](adr/0001-offline-first.md): why the owner gate goes and what stays.
- [adr/0002-gpui-x-hybrid.md](adr/0002-gpui-x-hybrid.md): the launcher rewrite on GPUIX.
- Upstream docs stay authoritative for the engine: [`docs/README.md`](../README.md),
  [`AGENTS.md`](../../AGENTS.md), [`docs/parity-workflow.md`](../parity-workflow.md).
- Russian translations land in `docs/fork/ru/` (P4 of the plan).

## Build (same as upstream)

```sh
cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar   # JDK 25
cargo run --release -p horizon-client
cd launcher && cargo run
```

The launcher is its own workspace. Basalt's copy is at `/home/shrx/dev/basalt/launcher`.

## Status

P0 (fork, rebrand, detach upstream infrastructure, docs) is done. P1 removes the owner gate in
code; until then the launcher still refuses Play without an owning account — that is upstream
behaviour, not Basalt's.
