# Scoria architecture: the fork's view

Scoria reuses Horizon's engine unchanged. This page is the map of what the fork touches and where
the boundaries are; upstream's [architecture.md](../architecture.md) and [AGENTS.md](../../AGENTS.md)
describe the engine itself (1773 Rust files, ~800k lines, `horizon-client` being 406k of them).

## Layers

```
launcher-ui/            flint-ui (planned; stack not chosen yet)   ← new, P3 paused
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
build never waits for GPUI. Scoria keeps that split: the future flint-ui workspace will never enter either Rust build
(ADR-0003). Until it exists, the launcher is the upstream GPUI UI, rebranded.

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

Offline support already exists upstream and is what Scoria promotes to the default:

- `Accounts.offline` / `offline_selected` / `looks` in `launcher/accounts.json` (public data only).
- The game takes `--name <offline name>` and an offline UUID; no session file means fully offline.
- `horizon-friends` (invite codes + relay) never used a Microsoft account; LAN works offline.
- `launcher/src/sources.rs` finds Prism Launcher, MultiMC, Modrinth App, ATLauncher and GDLauncher
  installations, including `instances/<name>/minecraft` — so an install made by Freesm/Prism can
  be prepared without any Mojang download.

## P0/P1/P2 changes

1. `upstream` remote; no `origin` yet; baseline tag.
2. User-facing brand: Scoria (window title, macOS menu, crash screen, desktop entries, app id,
   `launcher/links.json`, `packaging/PLAY.txt`, README notice). Internal names stay: crate names,
   the `horizon-launcher` binary, `HORIZON_*` environment variables, `horizon-client`, data
   folders. Renaming those would touch the game and every harness for no user-visible gain.
3. Upstream infrastructure detached:
   - `backend::self_update_allowed()` returns `false` (no launcher update from Horizon's releases).
   - `nightly::latest()` returns `Ok(None)` (Nightly channel off).
   - `browser::sources()` offers Modrinth only (Horizon's registry removed until Scoria has one).
   - Game builds still come from `backend::REPO` for now: Scoria has no release pipeline yet, and
     the launcher needs *some* `horizon-client` binary to install. P5 replaces this with Scoria
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
- P2 removes the Mojang **game download** (`--download`, `download_for_owner`,
  `mojang::download`): preparation always reads an installation the player has.
- P2 adds Freesm Launcher to `sources::known_folders` (Linux, macOS, Windows) and to
  `launcher_of`, so it is found without choosing a folder.
- P2 makes preparation reuse the launcher's Java: `<root>/java/<runtime>` and an instance's
  `instance.cfg` `JavaPath` are tried before downloading Mojang's runtime
  (`sources::launcher_java_homes`).
- The 26.3 **server jar** is still fetched from Mojang once for data generation unless it is
  already in `<data>/server-jars/26.3.jar`. That is the only remaining Mojang game file (the
  client jar, libraries and assets never are).
- P2 removes the Nightly channel: `launcher/src/nightly.rs`, `Settings::nightly`, the settings
  row and the Nightly instance logic are gone; `backend::Msg` has no `Nightly*` variants.
- P2 removes telemetry from the launcher: `launcher/src/telemetry.rs` and
  `launcher/src/collection.rs` are deleted, so are the consent screen, the `data` CLI command,
  the "I'm playing" ping and benchmark uploads. `backend::client_command` sets
  `HORIZON_TELEMETRY_URL=off` for every launched game, which is the hard switch in
  `crates/horizon-telemetry`; the engine's telemetry code stays but never reaches the network.

## Invariants the fork keeps

- **No Mojang files in the tree.** `tools/check_public_tree.py` still runs on commit
  (`.githooks`); `.local/` stays gitignored.
- **Vanilla parity is the specification.** Scoria does not change the engine's behaviour, so the
  upstream harnesses remain valid; they need a local Minecraft 26.3 and are not run in CI here.
- **No session forgery.** The game still joins online-mode servers only with a real Microsoft
  session; Scoria only stops *requiring* one to start the game.

## Known gaps after P1

- The Mojang game-download code is still compiled (unreachable from the UI); P2 deletes it.
- The UI is still upstream's Rust GPUI (P3 paused until flint-ui has code; ADR-0003).
- Horizon's release, nightly, parity, game-build and registry workflows are deleted. `.github/`
  keeps only `ci.yml` (format, clippy, tests, public-tree check) and `cancel-on-close.yml`;
  `ci.yml` runs on GitHub's runners and needs no secrets from Horizon.
