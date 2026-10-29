# Scoria

Minecraft: Java Edition 26.3, rewritten in Rust and played without a Microsoft account.

Scoria is a native client and launcher: it reads vanilla worlds, follows vanilla rules, and starts
offline. There is no sign-in, no session file and no telemetry. You can play on your own files, on
LAN, and on offline-mode servers. Online-mode servers need a Microsoft session, which Scoria does
not have.

> Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft.
> Scoria contains no Minecraft code, assets or data. You need your own copy of Minecraft: Java
> Edition 26.3. Scoria is an independent, MIT-licensed fork of [Horizon](https://github.com/horizon-rust/horizon).

## What you can do

- **Play** single player with vanilla world generation, mobs, villages, redstone, commands and the
  HUD, on saves in vanilla's format.
- **Use your own game files.** Scoria reads the official launcher, Prism, Freesm, MultiMC, the
  Modrinth App or ATLauncher. If none has 26.3, it downloads the pinned version from Mojang's
  piston-meta into its own folder.
- **One game folder by default.** The instance manager is a test-only feature
  (`SCORIA_UI_INSTANCES=1`, branch `test/instances`), not part of the release UI.
- **Install from Modrinth** — mods, ported to Scoria's runtime, and resource packs. Every project is
  labelled **Works**, **Partly** or **Doesn't run** before you install it. Java mods are ported;
  Java itself is not run. CurseForge is not supported yet.
- **Play offline** — offline-mode servers and LAN. There is no Microsoft sign-in and no session
  handoff.
- **Use a 1280×720 window.** On Linux the launcher is Wayland-native: the compositor draws the
  titlebar, drag, resize, minimize, maximize and close.
- **Switch Night / Lilac** appearance. The theme covers the rail, the titlebar, the stage and every
  page, not just one panel.

Modpacks can be imported and ported by the engine; the new launcher has no modpack browser yet.

## Run it

```sh
# engine
cd launcher && cargo build --release

# launcher UI
cd ../launcher-ui
bun install
bun run dev            # hot reload, 1280x720
bun run build          # dist/scoria-launcher-ui
```

Or build the shippable bundle:

```sh
sh tools/package_ui.sh
# dist/scoria-launcher-linux-x86_64.tar.gz
```

`bun run dev` and the packaged wrapper start the UI on the session's native backend (Wayland), so
keyboard layouts and text input work without an XWayland hop.

## How it is put together

- `crates/` — the Rust engine: client, world, protocol, assets, launcher preparation, servers.
- `launcher/` — the Rust launcher engine and the JSON-lines API (`horizon-launcher api`).
- `launcher-ui/` — the desktop UI: React on GPUIX with flint-ui 0.1.2.
  - `src/screens/` — one component per screen, listed in `src/screens/registry.tsx`.
  - `src/components/layout/` — rail, topbar, window controls, resize edges.
  - `src/api/` — the typed engine client, split by concern (`client`, `launcher`, `mods`).
- `docs/` — engine docs; Scoria's fork decisions are in [`docs/fork/`](docs/fork/README.md).

## Status

The game engine is Horizon's, with Scoria's offline and launcher changes on top. The launcher
UI is being rebuilt as Scoria's own product. Known gaps:

- no online-mode servers and no Microsoft account, by design;
- no CurseForge browser;
- no modpack browser in the new UI (the engine can import and port them);
- Windows and macOS launcher windows keep the native titlebar.

## Licence

MIT. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Minecraft is a
trademark of Mojang Synergies AB.
