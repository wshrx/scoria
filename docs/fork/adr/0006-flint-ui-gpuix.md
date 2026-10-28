# ADR-0006: adopt flint-ui and gpuix for the launcher UI

- Status: accepted
- Date: 2026-10-28
- Supersedes [ADR-0003](0003-launcher-ui-flint-ui.md) (which deferred the UI until flint-ui existed)

## Context

flint-ui is no longer empty: `flint 0.1.0` ships twelve copy-in components for **GPUIX 0.11.0**
(`@gpuix/react` + `@gpuix/native`, pinned to the same exact version), a CLI that copies the source
into the app, Medeo fonts and a token set. ADR-0003 deferred the launcher UI until exactly this.

The Rust engine (`launcher/`) already exposes everything the UI needs as CLI commands with
`--json` and as MCP tools; ADR-0002 defined the seam. Nothing in the engine requires Microsoft:
offline names are the primary identity since P1.

## Decision

1. **The launcher UI is `launcher-ui/`**, built with **gpuix 0.11.0** and **flint-ui 0.1.0**.
   flint's files are copied into `launcher-ui/src` and owned by this repository, as flint intends.
2. **The engine stays Rust.** `launcher-ui` renders and calls the engine over JSON-RPC/CLI; it
   never touches the file system, keychain or processes directly.
3. **No Microsoft in the UI.** Offline profile, instances, worlds, settings; no sign-in screen, no
   account chips, no telemetry.
4. **Pin gpuix exactly** (`0.11.0` for both packages); flint is pre-1.0, so copy upgrades are
   deliberate.
5. The UI ships as a compiled Bun binary (`bun build --compile`) beside the Rust engine; it is not
   an npm dependency of the engine.

## Status

- `launcher-ui/` uses flint's theme, Text, Button, Card, Badge, Switch, Input, Label and Icons;
  Home, Instances and Settings render real engine state.
- **The bridge exists**: `horizon-launcher api` serves `state`, `prepare`, `play`,
  `select_instance`, `create_instance`, `set_offline_name`, `set_appearance`,
  `set_while_playing` and `backup_world` as JSON lines; `src/api.ts` spawns it and falls back to
  a preview snapshot when the binary is absent. The header says `engine` or `preview`.
- The screens act: Play and world rows start the game, instance rows select, New instance works,
  Servers joins/adds/removes servers.dat entries, Settings persists appearance, while-playing and
  language, versions select/remove/install, Prepare streams its log from Freesm/Prism/official
  installs.
- `bun run typecheck`, `bun run build` (106 MB) and the headless smoke pass.
- Not yet: install/update of versions from the UI, packaging the UI beside the engine in releases,
  and screens beyond Home/Instances/Settings.

## Consequences

- Players need no Node/Bun at runtime: the UI is one compiled executable; developers do.
- The launcher's own window is now flint's (ember accent instead of flint's sky), not upstream's
  GPUI screens; both can coexist until the last screen moves.
- Removing Microsoft from the Rust engine is a separate change; the UI already assumes it is gone.

## Next

1. Engine bridge: JSON-RPC over stdio/socket for `state`, `play`, `prepare`, `instances`, `worlds`,
   `settings`.
2. Replace `src/api.ts` preview data with that bridge.
3. Remove the Rust launcher's Microsoft auth (account.rs rewrite, keychain/oo7 dependencies, CLI
   and remaining GPUI account screens) so the product is offline-only end to end.
