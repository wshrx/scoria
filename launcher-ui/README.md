# Scoria launcher UI

The launcher's interface: React on **GPUIX 0.11.0** with **flint-ui 0.1.2** components copied in
(`src/lib/flint`, `src/components/ui`). It is a desktop window, not a web app. Everything is a pill
or a soft rounded panel; the backdrop is a procedurally painted, blurred world, not game art.

## Run

```sh
bun install
bun run dev            # hot reload, through scripts/run.ts
bun run typecheck
bun run build          # dist/scoria-launcher-ui, one Linux binary (~106 MB); opens at 1280x720
```

On Linux the launcher is Wayland-native: the compositor draws the titlebar, drag, resize and
rounded corners, and text input goes through the native Wayland path. There is no X11/XWayland
code and no in-app window chrome.

The UI talks to the Rust engine over JSON lines through `src/api/` (`client`, `launcher` and
`mods`). The engine downloads Minecraft 26.3 from Mojang's piston-meta when it is not already
prepared:

```
horizon-launcher api   # stdin: {"id":1,"method":"state"}  stdout: {"id":1,"ok":true,...}
```

`src/api/` finds the engine at `../launcher/target/debug/horizon-launcher` (from this folder) or
`SCORIA_LAUNCHER_BIN`. If it is absent, the screens render the `preview` snapshot and the header
says **preview** instead of **engine**.

Headless smoke, for CI or an agent (no window, exits after 2 s):

```sh
SCORIA_UI_HEADLESS=1 bun app.tsx
# [gpuix] created native window
# [gpuix] mount complete
# [scoria] preview: engine not found at …
```

`SCORIA_UI_PAGE=instances|mods|servers|settings` opens one page; `SCORIA_UI_WIDTH` and
`SCORIA_UI_HEIGHT` override the window size. The engine path is `SCORIA_LAUNCHER_BIN`.

Design and regression rules live in [DESIGN.md](DESIGN.md): tokens, layout, the one-entry
navigation rule, the black-corner/shape checklist and known GPUIX traps. Run
`bun run check:ui` before shipping.

## Methods

| Method | Params | Result |
|---|---|---|
| `state` | — | version, branch, prepared, offlineName, selectedInstance, appearance, whilePlaying, worlds, instances |
| `prepare` | `{ "instance": "horizon", "download": true }` (both optional) | prepared, log (last 200 lines); `download` fetches Minecraft from Mojang first when it is missing |
| `play` | `{ "instance": "horizon", "world": "Survival" }` (both optional) | pid, version, instance |
| `select_instance` | `{ "id": "horizon" }` | the new state |
| `create_instance` | `{ "name": "Survival testing" }` | the new state |
| `set_offline_name` | `{ "name": "Builder_1" }` | the new state |
| `set_appearance` | `{ "value": "dark"\|"light"\|"system" }` | the new state |
| `set_while_playing` | `{ "value": "stay"\|"minimize"\|"hide" }` | the new state |
| `backup_world` | `{ "world": "Survival" }` | path of the backup |
| `versions` | — | installed tags, prepared, selected |
| `select_version` | `{ "tag": "v0.2.93" }` | the version list |
| `remove_version` | `{ "tag": "v0.2.93" }` | the version list |
| `install_latest` | — | installs the newest release from `sources.json` |
| `mods` | — | installed Modrinth entries for the selected instance |
| `search_mods` | `{ "query": "sodium", "kind": "mod"\|"resource-pack" }` | Modrinth results with a support label |
| `install_mod` | `{ "id": "AANobbMI" }` | installs/ports the project; `progress` events with `label` and `value` |
| `remove_mod` | `{ "name": "sodium" }` | the remaining installed entries |
| `minecraft` | — | the pinned Minecraft version, its folder, source and per-file counts |
| `download_minecraft` | — | downloads what 26.3 lacks from piston-meta and piston-data; `log` and `progress` events on the same id |
| `servers` | — | the selected instance's servers.dat |
| `add_server` | `{ "name": "SMP", "ip": "host:port" }` | the server list |
| `remove_server` | `{ "index": 0 }` | the server list |
| `set_language` | `{ "value": "en"\|"ru"\|"system" }` | the new state |

The engine is stateless between calls; it reads the same `accounts.json`, `settings.json` and
instances the CLI does.

## Screens

- **Play**: one instance, one Play button, a Continue link for the last world, and only the
  state a player needs (ready, needs files, needs preparing). Screens are listed in
  `src/screens/registry.tsx`; the rail is generated from that list.
- **Instances** are not in the release UI. They live behind `SCORIA_UI_INSTANCES=1` on the
  `test/instances` branch until creating, switching and removing them is complete.
- **Mods**: Modrinth mods and resource packs; Java mods are ported and labelled Works / Partly /
  Doesn't run before install.
- **Servers**: the instance's servers.dat; join, add, remove. Offline-mode only.
- **Settings**: everything else — profile, appearance, language, launcher behaviour, Minecraft
  26.3 file status and download, installed Scoria builds, and build facts. Technical details
  (folder, manifest sha1, per-file counts) sit behind **Show details**.

There is no Microsoft account anywhere: offline profiles are the only identity.
