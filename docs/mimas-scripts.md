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
| `on_reload` | a `.mim` file changed on disk |
| `on_prepare_start` | a prepare starts |
| `on_prepare_progress` | progress crossed another 10% |
| `on_prepare_done` | preparation succeeded or failed |
| `on_download_done` | the same success hook, kept for old scripts |
| `on_install_start` / `on_install_done` | a catalogue install starts/finishes |
| `on_server_add` / `on_server_remove` | a saved server changed |
| `on_account_switch` | the playing profile changed |
| `on_screen_change` | the screen changed |
| `on_settings_change` | appearance or language was saved |
| `on_game_start` | Play started a game |
| `on_game_exit` | the game process is gone |

A script without a hook is fine. A type or runtime error becomes a toast and goes to the log.
The folder is watched by the OS (`notify`); saving any `.mim` calls `on_reload` without
recompiling Rust. A 350 ms debounce coalesces editor save bursts; the window never scans the
folder on a timer. The compiled VM is cached until the source changes.

## Host functions

| Function | Effect |
| --- | --- |
| `toast(message)` | an in-app toast |
| `log(message)` | one line in the launcher log |
| `screen("home"\|"mods"\|"servers"\|"settings")` | switch screens |
| `prepared()` / `playing()` | read the snapshot, so a script can choose the next action |
| `prepare_done()` / `prepare_total()` / `prepare_line()` | the current prepare |
| `last_ok()` / `last_error()` / `last_project()` | the last prepare or install |
| `screen_name()` | the current screen |
| `account_kind()` / `account_name()` | the selected profile |
| `server_name()` / `server_ip()` / `server_index()` | the server that just changed |
| `setting_key()` / `setting_value()` | the setting that just changed |
| `state("home-preparing")` | apply one synthetic `SCORIA_UI_STATE` fixture |
| `progress(42.0, "Downloading…")` | draw a synthetic download at 0..100; no network |
| `download()` | start the real prepare |
| `cancel()` | cancel a prepare |
| `play()` | the same one game action as the button |
| `stop()` | stop the running game |
| `install(project)` | install a catalogue project |
| `add_server(name, ip)` | add a saved server |
| `remove_server(index)` | remove a saved server by index |
| `select_offline(name)` | play as an offline name |
| `set_appearance("dark\|light\|system")` | save the appearance |
| `set_language("system\|en\|ru\|fr")` | save the language |
| `refresh()` | reload the snapshot and servers |

A folder may limit what scripts can do with a `capabilities` file:

```text
allow = toast, log, screen, play, download, cancel, install
```

Without it every dev action is allowed. A listed action not in `allow` is dropped before the
window sees it.

Scripts only queue actions for the window; they cannot read files, start processes or touch the
engine. A script's `play()` cannot re-enter a hook: the window has a one-level guard. Mimas floats
need a decimal point (`42.0`, not `42`).

## What it is for

- **Download UI**: `download-demo.mim` sets 42%, the Mojang line and a toast, so the preparing
  button, progress line, status row and cancel path can be inspected without downloading 26.3.
- **States**: `states.mim` jumps to `home-playing`; change the name to any QA state.
- **Logic**: `logic.mim` uses `playing()`/`prepared()` and `if`/`else` to decide between
  switching Home, starting a prepared game and downloading.
- **Screens**: `screens.mim` walks to Mods for a quick visual smoke test.
- **All hooks**: `hooks-all.mim` shows every event and its getters, including prepare, install,
  servers, accounts, settings and game exit.

The engine is a separate library, [`launcher/hooks`](../launcher/hooks/README.md): one VM per
`.mim`, per-file error isolation, `notify` watcher and a `capabilities` filter. The window only
maps its `Command`s to GPUI actions.

The repository examples are in [`launcher/ui/dev/scripts/`](../launcher/ui/dev/scripts/). They are
developer fixtures, not a supported player API.

## Why developer-only

Mimas is 0.4 and young; it adds a compiler, a VM and a GC. The launcher stays a single small binary
with the feature off. Keeping scripts behind `SCORIA_DEV_SCRIPTS` means a player cannot drop code
into the launcher's data folder and have it execute.
