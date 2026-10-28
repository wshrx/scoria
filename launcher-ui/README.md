# Scoria launcher UI

The launcher's interface: React on **GPUIX 0.11.0** with **flint-ui 0.1.0** components copied in
(`src/lib/flint`, `src/components/ui`). It is a desktop window, not a web app.

## Run

```sh
bun install
bun run dev            # hot reload
bun run typecheck
bun run build          # dist/scoria-launcher-ui, one Linux binary (~106 MB)
```

The UI talks to the Rust engine over JSON lines:

```
horizon-launcher api   # stdin: {"id":1,"method":"state"}  stdout: {"id":1,"ok":true,...}
```

`src/api.ts` finds the engine at `../launcher/target/debug/horizon-launcher` (from this folder) or
`SCORIA_LAUNCHER_BIN`. If it is absent, the screens render the `preview` snapshot and the header
says **preview** instead of **engine**.

Headless smoke, for CI or an agent (no window, exits after 2 s):

```sh
SCORIA_UI_HEADLESS=1 bun app.tsx
# [gpuix] created native window
# [gpuix] mount complete
```

## Methods

| Method | Params | Result |
|---|---|---|
| `state` | — | version, branch, prepared, offlineName, selectedInstance, appearance, whilePlaying, worlds, instances |
| `prepare` | `{ "instance": "horizon" }` (optional) | prepared, log (last 200 lines) |
| `play` | `{ "instance": "horizon", "world": "Survival" }` (both optional) | pid, version, instance |
| `select_instance` | `{ "id": "horizon" }` | the new state |
| `create_instance` | `{ "name": "Survival testing" }` | the new state |
| `set_offline_name` | `{ "name": "Builder_1" }` | the new state |
| `set_appearance` | `{ "value": "dark"\|"light"\|"system" }` | the new state |
| `set_while_playing` | `{ "value": "stay"\|"minimize"\|"hide" }` | the new state |
| `backup_world` | `{ "world": "Survival" }` | path of the backup |

The engine is stateless between calls; it reads the same `accounts.json`, `settings.json` and
instances the CLI does.

## Screens

- **Home**: Play, Prepare, worlds.
- **Instances**: release, ported packs, modpacks; which are prepared.
- **Settings**: offline name, launcher behaviour, dark appearance.

There is no Microsoft account anywhere: offline profiles are the only identity.
