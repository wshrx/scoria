# Launcher UI states

The launcher draws every screen from one snapshot. A state that is hard to reach by hand (a failed
download, an empty catalogue, a game that exited) is still a state the window must survive. The
window therefore has a QA hook that replaces the snapshot with synthetic data:

```sh
SCORIA_UI_STATE=home-preparing launcher/target/debug/scoria
```

`SCORIA_UI_STATE` is read once, before the first frame. While it is set the window does not call the
network, prepare the game or poll the running process, so a state can be captured on a machine that
is offline. Nothing is persisted.

## States

| State | What it paints |
| --- | --- |
| `home-empty` | Home with no installed game version |
| `home-not-prepared` | A version is installed, the game files are missing |
| `home-preparing` | Download in flight at 42% with a last log line |
| `home-prepared` | Ready to play |
| `home-playing` | A game process is running |
| `home-error` | Play failed; the error card and Try again |
| `mods-empty` | Mods screen before a search |
| `mods-loading` | Catalogue skeleton cards |
| `mods-results` | Horizon and Modrinth results |
| `mods-error` | Catalogue request failed; retry card |
| `servers-empty` | No saved servers |
| `servers-list` | Two saved servers with Join and Remove |
| `servers-error` | `servers.json` cannot be read; retry card |
| `settings` | Settings, dark |
| `settings-light` | Settings, light |
| `compact` | Collapsed rail in a 900x640 window |
| `small` | 1024x600, just under the compact breakpoint |
| `wide` | 1920x1080, two-column Home |

The fixtures live in `apply_qa_state` and `qa_snapshot` in `launcher/ui/src/app.rs`. Add a state
there and to `STATES` in the capture tool at the same time.

## Capture

```sh
cd launcher
cargo build -p scoria-ui
cd ..
DISPLAY=:0 python3 tools/capture_ui_states.py
```

The tool launches one process per state with a temporary `HORIZON_HOME`, finds the X11 window by
`_NET_WM_PID`, waits 0.8 s for the first GPU frame, and saves a PNG to
`~/Pictures/scoria-ui-states/`. It writes an `index.md` next to the images. A capture below 20 KB is
treated as a blank frame and fails the run, so a black first frame cannot pass as a state.

Pass `--state` more than once to capture a subset, and `--out` to choose another folder:

```sh
DISPLAY=:0 python3 tools/capture_ui_states.py \
  --state home-preparing --state mods-loading --out /tmp/scoria-states
```

The capture uses X11 because the tool needs a window id; on a Wayland session run it with
`DISPLAY=:0` (XWayland) and do not script the compositor. The window is brought to the top of the
X stack only, not of the live desktop.

## Why these states

- **Home** covers the one game action: Download 26.3, Preparing, Play and Playing. The progress
  view shows the bar, the percent and the engine's last log line; Cancel writes the same
  `CANCEL` flag the CLI uses.
- **Mods** covers the search lifecycle. Loading is a skeleton, not a spinner over an empty page, so
  a slow registry does not look like a crash. The error card keeps the request text and offers a
  retry.
- **Servers** covers the list and the empty/error paths. A row has Join (starts the game with
  `--server`) and Remove.
- **Settings** covers both appearances; **compact** covers the collapsed rail.

A visual change is not done until these captures are regenerated and looked at. The gate checks
the code, not the pixels.

## Where the window lives

| File | Owns |
| --- | --- |
| `launcher/ui/src/main.rs` | boot, window options |
| `launcher/ui/src/app.rs` | `Shell` state, actions, QA fixtures, crash log |
| `launcher/ui/src/app/screens/` | one file per screen plus the titlebar/rail/render shell |
| `launcher/ui/src/assets.rs` | bundled fonts, icons, backdrop |
| `launcher/ui-kit/` | tokens and reusable components |
| `launcher/lang/{ru,fr}.json` | one table per language, shared with the CLI |

Strings from the engine (log lines, errors) are not rewritten in the window: they go through the
same table with `tr_in`, so a known line can be translated and an unknown one stays as it is.
