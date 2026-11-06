# Developer scripts (Mimas)

A developer-only harness for driving the launcher UI without a game, network or a full download.
It is **not a player feature**: release builds do not enable it, and the binary never looks for
scripts in a player's data folder.

```sh
SCORIA_DEV_SCRIPTS=$PWD/launcher/ui/dev/scripts \
  cargo run -p scoria-ui --features dev-scripts
```

The feature compiles Mimas, a small typed scripting language. `SCORIA_DEV_SCRIPTS` is the only
source folder; without it the window runs normally and loads nothing.

## Hooks

The window compiles every `*.mim` file in the folder and calls the function named after the event:

| Hook | When |
| --- | --- |
| `on_start` | the window opens |
| `on_game_start` | Play started a game |
| `on_game_exit` | the game process is gone |
| `on_download_done` | preparation finished |

A script without a hook is fine. A type or runtime error becomes a toast and goes to the log.

## Host functions

| Function | Effect |
| --- | --- |
| `toast(message)` | an in-app toast |
| `log(message)` | one line in the launcher log |
| `screen("home"\|"mods"\|"servers"\|"settings")` | switch screens |
| `state("home-preparing")` | apply one synthetic `SCORIA_UI_STATE` fixture |
| `progress(42.0, "Downloading…")` | draw a synthetic download at 0..100; no network |
| `download()` | start the real prepare |
| `cancel()` | cancel a prepare |
| `play()` | the same one game action as the button |
| `stop()` | stop the running game |

Scripts only queue actions for the window; they cannot read files, start processes or touch the
engine. A script's `play()` cannot re-enter a hook: the window has a one-level guard. Mimas floats
need a decimal point (`42.0`, not `42`).

## What it is for

- **Download UI**: `download-demo.mim` sets 42%, the Mojang line and a toast, so the preparing
  button, progress line, status row and cancel path can be inspected without downloading 26.3.
- **States**: `states.mim` jumps to `home-playing`; change the name to any QA state.
- **Screens**: `screens.mim` walks to Mods for a quick visual smoke test.

The repository examples are in [`launcher/ui/dev/scripts/`](../launcher/ui/dev/scripts/). They are
developer fixtures, not a supported player API.

## Why developer-only

Mimas is 0.4 and young; it adds a compiler, a VM and a GC. The launcher stays a single small binary
with the feature off. Keeping scripts behind `SCORIA_DEV_SCRIPTS` means a player cannot drop code
into the launcher's data folder and have it execute.
