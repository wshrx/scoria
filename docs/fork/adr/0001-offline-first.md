# ADR-0001: Offline-first launching

- Status: accepted (P0), implemented (P1)
- Date: 2026-10-28

## Context

Horizon decides on 2026-10-27 to gate Play behind an account that owns Minecraft: Java Edition
(`docs/microsoft-account.md`, "The owner gate"). The gate lives in `launcher/src/account.rs`
(`launch` → `NoOwner`, `has_owner`, `play_as` → `OFFLINE_NEEDS_OWNER`, `session_for_game`), with UI
in `owner_page.rs` and refusals in `main.rs:656`, `main.rs:2357`, `cli.rs:1356`, `mcp.rs:203`.

Basalt targets players who already have the game files — often installed by Freesm Launcher or
Prism — and who cannot or do not want to sign in with Microsoft. The engine already supports this:
without a session file the game runs offline with `--name` and an offline UUID, LAN and
offline-mode servers work, and `horizon-friends` never needed an account.

## Decision

1. **Offline names are first-class.** A fresh install offers a name field; Play works with no
   account at all. `Player::Offline` and `Player::GameName` no longer require an owner.
2. **Microsoft sign-in stays, optional.** It is needed for online-mode servers, cloud skins and
   anything that talks to Mojang. It is never required to start the game.
3. **No Mojang downloads.** Basalt removes the "download from Mojang for an owner" path
   (`backend.rs:1201`, `mojang.rs`, `prepare --download`). Preparation reads an existing
   installation, including Prism/Freesm instances via `sources.rs`.
4. **No authentication bypass.** Basalt does not mint sessions, does not patch online-mode
   handshakes and does not impersonate a licensed client. Enabling offline play is a launcher
   policy change, not a protocol change.
5. **Docs say "offline profile", not "pirate mode".** The product states plainly that it needs the
   player's own copy of the game and that online servers require a real account.

## Consequences

- `NoOwner` becomes unreachable for Play; keep the type only while the per-account "owns" answer
  is still used by the (soon removed) download path, then delete it.
- `accounts.json` becomes the primary identity source; its schema already holds offline names and
  per-name skins, so no migration is needed.
- Tests that assert refusal (`launcher/tests/owner_gate.rs`, `account/tests.rs:418`) are rewritten:
  Play with zero accounts must succeed, and the removed download path must not be reachable.
- The fork inherits the legal risk upstream already carries (code ported from decompiled sources,
  `docs/how-horizon-is-built.md`). Basalt does not add to it: no game files are distributed or
  fetched.

## Alternatives rejected

- **Remove Microsoft support entirely.** Loses online servers, cloud skins and the ability to
  verify ownership for anything; more code churn for a worse product.
- **Keep the gate but add a hidden bypass flag.** Same user outcome, worse honesty: the UI would
  keep lying to the player. Basalt changes the default, not a secret switch.
- **Ship a bundled game.** Illegal, and contrary to both upstream's design and this fork's premise.
