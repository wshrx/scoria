# ADR-0007: remove Microsoft accounts entirely

- Status: accepted
- Date: 2026-10-28
- Supersedes the "optional sign-in" part of [ADR-0001](0001-offline-first.md)

## Context

P1 removed the owner gate but kept Microsoft device-code sign-in for online-mode servers and
cloud skins. The product is offline-first, the new `launcher-ui` has no Microsoft surface, and the
fork's reason to exist is playing without an account. Keeping ~1200 lines of OAuth/Xbox/XSTS,
token storage and session handoff for a feature Scoria will not use is dead weight — and every
line of it is a place where a future change could quietly reintroduce the gate.

## Decision

1. **Delete Microsoft auth from the launcher.** `account.rs` is now offline profiles plus the
   launcher's generic keychain (still used for the Play-with-friends Tailscale token). No
   `Endpoints`, `DeviceCode`, `MsTokens`, `SignIn`, `session_for_game` or `write_session_file`.
2. **No session file reaches the game.** `backend::client` starts the client with `--name` (when
   an offline name plays) and nothing else; `Msg::Session` is a bare "resources are ready".
3. **CLI and MCP follow.** `account list|select|offline|forget-offline|skin`; `account sign-in`,
   `account sign-out` and the `account_sign_in` tool are gone.
4. **UI follows.** The GPUI settings lose the Microsoft rows and sign-in panel; `switch_page`
   lists offline names only. The flint UI never had them.
5. **Online-mode servers are out of scope.** No fake sessions, no authlib patches, no custom auth
   servers. Import game files from an installation the player already has.

## Consequences

- `accounts.json` from older builds still loads: unknown `accounts`/`selected` fields are ignored.
- Existing keychain items from the Microsoft era are unused; they can be deleted by hand.
- `oo7`/`keyring` dependencies stay for the Tailscale token, not for a game account.
- The launcher can only join offline-mode servers; that is stated in `offline-mode.md`.
- The game engine still contains its own online-mode client code; nothing in the launcher feeds
  it a session, so it cannot be used through Scoria.

## Alternatives rejected

- **Keep optional sign-in.** Dead weight and a temptation to restore the gate; rejected.
- **Custom auth (Ely.by, authlib injector).** That is an auth bypass; rejected.
