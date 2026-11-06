# Launcher scripts (Mimas)

The launcher can run small local scripts written in [Mimas](https://mim.as), a statically typed
embeddable language for Rust. Scripts are **opt-in**: the release build does not include the VM.

```sh
cargo build -p scoria-ui --features scripts
```

Put `*.mim` files in the launcher's `scripts` folder:

```text
$HORIZON_LAUNCHER_HOME/scripts/   (default: ~/.local/share/horizon-launcher/scripts/)
```

The window compiles all files at a hook and calls the function named after the event:

| Hook | When |
| --- | --- |
| `on_start` | the window opens |
| `on_game_start` | Play started a game |
| `on_game_exit` | the game process is gone |
| `on_download_done` | preparation finished |

A script without a hook is fine. A type or runtime error becomes a toast and is written to
`ui.log`.

## Host functions

| Function | Effect |
| --- | --- |
| `toast(message)` | an in-app toast |
| `log(message)` | one line in the launcher log |
| `play()` | the same one game action as the button |
| `download()` | prepare the game files |
| `screen("home"\|"mods"\|"servers"\|"settings")` | switch screens |

Scripts cannot read or write files, start processes or touch the engine directly. They queue
actions for the window, which applies them after the hook returns. A script's `play()` cannot
re-enter a hook: the window has a one-level guard.

## Example

The repository has [`launcher/ui/scripts/example.mim`](../launcher/ui/scripts/example.mim). Copy it
to the scripts folder and open the window:

```mimas
fn on_start() {
    toast("Scoria scripts are enabled");
}

fn on_game_exit() {
    toast("Game closed");
}
```

## Why opt-in

Mimas is 0.4 and young; it adds a compiler, a VM and a GC to the window. The launcher stays a
single small binary with the feature off. Enable it for local tooling, custom toasts and
development experiments, not for the default release until the language and the API have settled.
