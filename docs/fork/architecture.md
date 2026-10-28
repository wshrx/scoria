# Basalt architecture: the fork's view

Basalt reuses Horizon's engine unchanged. This page is the map of what the fork touches and where
the boundaries are; upstream's [architecture.md](../architecture.md) and [AGENTS.md](../../AGENTS.md)
describe the engine itself (1773 Rust files, ~800k lines, `horizon-client` being 406k of them).

## Layers

```
launcher-ui/            React + TypeScript on GPUIX (planned)      ← new, P3
        │  JSON-RPC over a local socket
launcher/               Rust: installs, preparation, processes, MCP  ← upstream, rebranded
        │
horizon-client          the game (800k-line workspace)               ← upstream, untouched
        ├─ horizon-world       worldgen, chunks, NBT, blocks
        ├─ horizon-protocol    protocol 777
        ├─ horizon-command     Brigadier parser
        └─ horizon-core        Java-compatible math/RNG
```

The launcher is a **separate Cargo workspace** on purpose (`launcher/Cargo.toml`), so the game's
build never waits for GPUI. Basalt keeps that split: the GPUIX UI is a third workspace
(`launcher-ui/`, Bun + TypeScript) that never enters either Rust build.

## Where the Microsoft account decides things (upstream)

| Concern | File |
|---|---|
| device-code sign-in, Xbox/XSTS/Minecraft exchange, entitlements | `launcher/src/account.rs` |
| the owner gate (`NoOwner`, `has_owner`, `play_as`, `session_for_game`) | `launcher/src/account.rs:854-1168` |
| refusal paths | `launcher/src/main.rs:656`, `launcher/src/main.rs:2357`, `launcher/src/cli.rs:1356`, `launcher/src/mcp.rs:203` |
| the gate's screen | `launcher/src/owner_page.rs` |
| downloads from Mojang for an owner | `launcher/src/backend.rs:1201`, `launcher/src/mojang.rs` |
| keychain storage of tokens | `launcher/src/account.rs:552-740` |
| handing a session to the game | `launcher/src/account.rs:1057`; game side in `docs/microsoft-account.md` |

Offline support already exists upstream and is what Basalt promotes to the default:

- `Accounts.offline` / `offline_selected` / `looks` in `launcher/accounts.json` (public data only).
- The game takes `--name <offline name>` and an offline UUID; no session file means fully offline.
- `horizon-friends` (invite codes + relay) never used a Microsoft account; LAN works offline.
- `launcher/src/sources.rs` finds Prism Launcher, MultiMC, Modrinth App, ATLauncher and GDLauncher
  installations, including `instances/<name>/minecraft` — so an install made by Freesm/Prism can
  be prepared without any Mojang download.

## P0/P1 changes

1. `upstream` remote; no `origin` yet; baseline tag.
2. User-facing brand: Basalt (window title, macOS menu, crash screen, desktop entries, app id,
   `launcher/links.json`, `packaging/PLAY.txt`, README notice). Internal names stay: crate names,
   the `horizon-launcher` binary, `HORIZON_*` environment variables, `horizon-client`, data
   folders. Renaming those would touch the game and every harness for no user-visible gain.
3. Upstream infrastructure detached:
   - `backend::self_update_allowed()` returns `false` (no launcher update from Horizon's releases).
   - `nightly::latest()` returns `Ok(None)` (Nightly channel off).
   - `browser::sources()` offers Modrinth only (Horizon's registry removed until Basalt has one).
   - Game builds still come from `backend::REPO` for now: Basalt has no release pipeline yet, and
     the launcher needs *some* `horizon-client` binary to install. P5 replaces this with Basalt
     builds; `docs/fork/README.md` says so.
4. No signature/verification code was deleted. Self-update is off, so the release-signing path is
   dormant, not bypassed.

P1 removes the gate itself:

- `account::launch` returns `Launch` (no `NoOwner`); `session_for_game` returns the session pair
  and never refuses; `play_as(Player::Offline)` no longer asks for an owner.
- `NoOwner`, `NEEDS_OWNER` and `OFFLINE_NEEDS_OWNER` are gone; `owner_page.rs` is deleted; Home's
  "Sign in to play" row and the "Download and prepare" button are gone.
- `horizon-launcher play` and MCP `play` reach preparation instead of stopping at the gate
  (`launcher/tests/offline_play.rs` asserts it).
- Microsoft sign-in, its keychain storage and the session-file handoff stay: they are what
  online-mode servers need.
- The Mojang **download** path (`download_for_owner`) still exists but is only reachable from the
  removed UI/CLI entry points; P2 deletes it.

## Invariants the fork keeps

- **No Mojang files in the tree.** `tools/check_public_tree.py` still runs on commit
  (`.githooks`); `.local/` stays gitignored.
- **Vanilla parity is the specification.** Basalt does not change the engine's behaviour, so the
  upstream harnesses remain valid; they need a local Minecraft 26.3 and are not run in CI here.
- **No session forgery.** The game still joins online-mode servers only with a real Microsoft
  session; Basalt only stops *requiring* one to start the game.

## Known gaps after P1

- The Mojang game-download code is still compiled (unreachable from the UI); P2 deletes it.
- The GPUIX UI does not exist yet; the Rust GPUI UI is unchanged (P3).
- No CI in the fork yet; upstream workflows are left in place but they target Horizon's runners,
  secrets and self-hosted machines and must be trimmed before the fork is pushed (P2).
