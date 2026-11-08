# Scoria · feat/launcher-v2

Branch for the next launcher: **online + offline profiles**, Microsoft sign-in, a proper
service layer, a faster development build, motion, and a design system that is not hardcoded.

> Status: branch opened from `dev` (`e6295ae30`). This README is the branch map; the product
> README stays on `dev`/`main` until this lands there.
>
> Implemented so far: dev/fast profiles, game `--session-file`, Microsoft device-code auth core,
> online profiles in `accounts.json`, CLI `account sign-in` / `sign-out`, Settings sign-in panel,
> and the upstream motion scale. Still next: full profile UI, service split, gpui-fast spike.

## Why this branch

The launcher grew organically:

- `scoria-ui/src/app.rs` is a 1,100-line state/action/render file;
- screens call `scoria-core` functions directly, mixing filesystem, network and presentation;
- `scoria-ui-kit/src/tokens.rs` duplicates design tokens instead of sharing upstream Horizon's
  `crates/horizon-theme`;
- the UI is on `gpui-ce 0.3.0`, while upstream Horizon uses `gpui-fast 0.1.13`;
- Microsoft sign-in and online-mode play were removed from the fork;
- animations exist (screen fade/rise, button transitions) but are ad-hoc and not tokenised.

This branch fixes the architecture before adding more screens.

## Goals

### 1. Online and offline profiles

One account model with two kinds:

| Kind | Sign-in | Plays | Stores |
| --- | --- | --- | --- |
| Offline | none | single-player, LAN, offline-mode servers | name, UUID, skin/cape |
| Online (Microsoft) | device code at `microsoft.com/link` | everything offline does, plus online-mode servers with the account's name/skin | public profile in `accounts.json`; tokens only in the OS keychain |

Offline-first stays: offline names keep working without a Microsoft account. The launcher does
not force the upstream owner gate; an online account is needed only for online-mode servers.

The account screen will show:

- profile head, name, kind, and selected state;
- online status: ready / needs refresh / sign in again;
- Microsoft device-code panel with copy-link and cancel;
- add offline name, select, remove, sign out;
- skin/cape for offline names, profile skin for Microsoft accounts (when the game can use it).

### 2. Microsoft authentication

Port the upstream Horizon flow, adapted to Scoria:

1. OAuth device code (`consumers` tenant, `XboxLive.signin offline_access`);
2. Xbox Live user auth;
3. XSTS;
4. Minecraft services login;
5. entitlements (ownership) and profile;
6. refresh before expiry; sign-out deletes secrets.

Tokens live in the OS keychain (Secret Service / Keychain / Credential Manager), or the encrypted
fallback file on Linux without a keychain. They never appear in logs, errors, CLI output or argv.

**Blocker:** sign-in needs Scoria's own Azure app registration and Mojang app-ID approval. A
build without `SCORIA_MSA_CLIENT_ID` must say so and keep offline play working. Never reuse
Horizon's or another launcher's client ID.

The game side is already present in this fork: `horizon-client-render/src/auth.rs` and
`net.rs::join_server` still support a Microsoft session. `game.rs` only needs the
`--session-file` / `--session-stdin` plumbing back, then the launcher can hand over a session.

### 3. Service architecture

Target layers:

```text
scoria-core        domain: accounts, profiles, worlds, servers, instances, catalog, settings
scoria-services    application services: AccountService, LaunchService, CatalogService, ...
scoria-ui          presentation only: GPUI views, state, actions
scoria-ui-kit      design system: tokens and components
scoria-mimas       optional scripting host (feature)
```

Rules:

- UI never opens files, starts processes or calls HTTP directly;
- services return typed DTOs and errors, not `serde_json::Value` blobs;
- account secrets stay behind `SecretStore`; the UI sees public status only;
- each service has a fake for tests; the existing integration tests stay green;
- keep modules and traits where they pay for themselves; no distributed-systems cosplay.

### 4. Build profiles and Cargo features

Development must be quick, not maximally optimised:

- `dev`: `opt-level = 0`, `debug = "line-tables-only"`, incremental on, many codegen units;
- dependencies at `opt-level = 1`, so the UI is usable without waiting for a full release build;
- `fast`: `opt-level = 2`, no LTO, incremental on — the daily driver;
- `release`: LTO thin + one codegen unit, stripped.

Features stay in `Cargo.toml`, not environment variables:

- `dev-scripts` — Mimas harness and demo helpers (off by default);
- later: `microsoft` / `inspector` only if they actually remove code from player builds;
- no feature that changes the meaning of a player build silently.

### 5. Animations

Adopt the upstream motion scale:

| Token | Duration | Use |
| --- | --- | --- |
| `QUICK` | 150 ms | hover, press, colour |
| `MOVE` | 240 ms | selection, segmented control, page switches |
| `ENTER` | 600 ms | page content fade + 8 px rise |
| `STAGGER` | 40 ms | list rows arriving |

First set: page enter/exit, hover plate, segmented slide, progress fill, toast, button press.
Respect reduced motion from the OS where GPUI exposes it.

### 6. Developer hooks

The dev-only Mimas layer is a typed event/command bus, not a second UI:

- events: `on_prepare_start`, `on_prepare_progress`, `on_prepare_done`, `on_install_start`,
  `on_install_done`, `on_server_add`, `on_server_remove`, `on_account_switch`,
  `on_screen_change`, `on_settings_change`, `on_game_start`, `on_game_exit`;
- commands: `download`, `cancel`, `play`, `stop`, `install`, `add_server`, `remove_server`,
  `select_offline`, `set_appearance`, `set_language`, `refresh`;
- getters: `prepare_done/total/line`, `last_ok/error/project`, `screen_name`, `account_*`,
  `server_*`, `setting_*`;
- every script is `dev-scripts` only; a player build has no VM and no script folder;
- the OS watcher (`notify`) reloads on save with a 350 ms debounce, and a compiled VM is cached
  until the source hash changes.

The engine is extracted into [`launcher/hooks`](launcher/hooks/README.md): one VM per `.mim`,
per-file errors, an OS watcher and a capability filter. The window only turns `Command`s into
actions. The example is `launcher/ui/dev/scripts/hooks-all.mim`.

### 7. Design system

One source of truth, mirroring upstream Horizon:

- tokens for colour, type, spacing, radii, control heights and motion;
- `launcher/ui-kit/src/tokens.rs` is the source, and
  `launcher/ui-kit/_variables.css` is generated from it for the website and tools;
- a test fails when the CSS and Rust tokens diverge:
  `WRITE_VARIABLES=1 cargo test -p scoria-ui-kit variables_css_is_in_sync`;
- buttons, selects, rows and page widths read `Theme.sizes`, not local numbers;
- no `#RRGGBB` or magic `px` in screens;
- contrast tests for every text/background pair, both appearances;
- the existing `scoria-ui-kit` becomes a consumer of tokens, not a second copy.

### 8. UI stack

Evaluate `gpui-fast` + `gpui-base` in a spike before migrating the whole window. Upstream
Horizon already ships on `gpui-fast`; it is the natural target. `gpui-component` is a widget
source only if we can theme it without losing Scoria's identity.

## Milestones

1. **Foundation** — branch, README, dev/fast builds, token rules. ✅
2. **Game session** — `--session-file` / `--session-stdin` restored. ✅
3. **Auth core** — device code, Xbox/XSTS, Minecraft, ownership, profile, keychain, refresh. ✅
4. **Profiles UI** — Settings sign-in panel and online rows ✅; full switcher still next.
   Microsoft's four-square mark is used, the button stays on Scoria surfaces instead of the
   orange primary action. Screen tabs now live in the window body under the titlebar, and
   appearance/language use a floating opaque popover that does not resize the page.
5. **Service split** — move filesystem/network out of `app.rs`, add fakes. Next.
6. **Motion + tokens** — motion tokens, page enter, `_variables.css` and unified control sizes
   done; hover plates, stagger and a full hardcoded-value audit still next.
7. **gpui-fast spike** — one screen, measurements, then decision.
8. **Merge** — `dev`, then `main` and a release.

## Commands

```sh
cargo xtask ui          # quick development build
cargo xtask ui --fast   # optimised, incremental daily build
cargo xtask gate        # fmt, clippy, 229 tests, pytest
```

Sign in from a terminal (the UI has the same flow):

```sh
SCORIA_MSA_CLIENT_ID=<azure-app-id> \
  cargo run --manifest-path launcher/Cargo.toml --bin horizon-launcher -- account sign-in
```

The client ID is Scoria's own Azure public-client app; a build without it still plays offline.

Run with a scratch profile during development:

```sh
HORIZON_HOME=$(mktemp -d) HORIZON_LAUNCHER_HOME=$(mktemp -d) cargo xtask ui
```

## Removed branches

The three superseded redesign experiments were archived as tags and removed locally and on
GitHub:

```text
archive/exp/redesign
archive/exp/redesign-brawl
archive/exp/redesign-dungeons
```

`exp/mimas`, `exp/motion` and `exp/gpui-fast` stay: they feed this branch.

## Notes

- This is still a fork of Horizon (MIT). Keep the upstream attribution and notices.
- Do not commit Minecraft files, tokens, client secrets or personal account data.
- A build without a Microsoft client ID is valid: it signs in offline only.
