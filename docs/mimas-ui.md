# Mimas UI scripting

The `exp/mimas` launcher window is drawn by one Mimas script:

```text
launcher/ui/dev/mimas-ui/scene.mim     the whole UI: topbar, Home, Mods, Servers, Settings
launcher/ui/examples/mimas_ui.rs       the host: data, actions and GPUI rendering
```

The script never touches the window or the file system. It calls host functions; Rust turns the
resulting node list into GPUI elements and performs every real action through `scoria-core`.

Run it:

```sh
cd launcher
cargo run -p scoria-ui --features dev-scripts --example mimas-ui
```

Use another script (hot reload still works):

```sh
SCORIA_MIMAS_UI=/tmp/experiment.mim \
  cargo run -p scoria-ui --features dev-scripts --example mimas-ui
```

`--features dev-scripts` is required. A player build does not include the VM and does not read
scripts from the data folder.

## The drawing model

`draw()` is called after every action. It starts with `clear()` and builds one tree; there is no
diff and no retained widget state. The host keeps the state (screen, data, inputs, progress).

| Function | Meaning |
| --- | --- |
| `screen(fill)` | the window root: a column that fills the window |
| `row(gap, padding, w, h, fill, radius)` | open a horizontal flex box |
| `column(gap, padding, w, h, fill, radius)` | open a vertical flex box |
| `scroll(gap, padding, w, h, fill, radius)` | vertical column with `overflow-y: scroll` |
| `end_box()` | close the last open box |
| `pad(x, y)` | override horizontal and vertical padding inside the current box |
| `fill_w()` / `fill_h()` | make the current box fill its parent |
| `w_pct(0.56)` | width as a fraction of the parent (responsive columns) |
| `border_bottom(width, fill)` | bottom border on the current box |
| `drag_window()` | let the current box drag the window |
| `align("start\|center\|end\|stretch")` | cross-axis alignment |
| `justify("start\|center\|end\|between\|around")` | main-axis distribution |
| `grow(1.0)` | flex-grow for the current box |
| `wrap()` | allow row wrapping |
| `spacer()` | flexible empty space (`flex-grow: 1`) |
| `theme_color(role)` | semantic colour from the active theme |
| `text(s, size, fill)` | one line of text |
| `text_fill(s, size, fill)` | wrapping text that fills the parent's width |
| `text_w(s, size, fill, max_width)` | text with a maximum width, so lines wrap |
| `button(label, action, fill)` | 192×52 button |
| `small_button(label, action, fill)` | 112×36 button |
| `button_w(label, action, fill, width)` | button with an explicit width |
| `small_button_w(label, action, fill, width)` | small button with an explicit width |
| `link(label, action, fill, size)` | text that behaves like a button |
| `icon(name, size, fill)` | bundled SVG icon, tinted by `fill` |
| `image(name, w, h, radius)` | bundled image (`mark`, `backdrop`, `backdrop-light`) |
| `image_fill(name, h, radius)` | an image that fills its parent's width |
| `input(id, placeholder, width)` | editable single-line field |
| `window_control("min\|max\|close", size, fill)` | real window buttons |

Sizes are page units (1 unit = 1 px at 100% zoom). `w = 0` or `h = 0` means auto. Colours are
`#RRGGBB` or `#RRGGBBAA`; `#00000000` is transparent.

`theme_color` roles: `bg`, `surface`, `surface2`, `surface3`, `overlay`, `text`, `muted`,
`border`, `separator`, `accent`, `accent_fg`, `accent_soft`, `success`, `warning`, `danger`,
`transparent`. The scene never hardcodes a palette, so switching Dark/Light repaints at once.

## A complete screen

This is the shape of `home()` in `scene.mim`, shortened:

```mimas
fn home() {
    scroll(0.0, 0.0, 0.0, 0.0, theme_color("transparent"), 0.0);
        fill_w();
        row(32.0, 0.0, 0.0, 0.0, theme_color("transparent"), 0.0);
            fill_w();
            align("stretch");
            pad(0.0, 32.0);
            column(12.0, 0.0, 0.0, 0.0, theme_color("transparent"), 0.0);
                w_pct(0.56);
                text("ПОСЛЕДНИЙ ЗАПУСК", 10.0, theme_color("muted"));
                text(version(), 56.0, theme_color("text"));
                text_fill("Ваши миры, моды и серверы.", 15.0, theme_color("muted"));
                action_button();
                status_line();
                text("Ваши миры", 20.0, theme_color("text"));
                let i = 0.0;
                while i < world_count() {
                    row(14.0, 14.0, 0.0, 56.0, theme_color("surface2"), 12.0);
                        fill_w();
                        align("center");
                        icon("map-pin", 16.0, theme_color("accent"));
                        column(2.0, 0.0, 0.0, 0.0, theme_color("transparent"), 0.0);
                            text(world_name(i), 16.0, theme_color("text"));
                            text(f"{world_mode(i)} · {world_last(i)}", 12.0, theme_color("muted"));
                        end_box();
                        spacer();
                        small_button("Играть", f"play-world:{world_folder(i)}", theme_color("transparent"));
                    end_box();
                    i = i + 1.0;
                }
            end_box();
        end_box();
    end_box();
}
```

Mimas is statically typed; literals are inferred, `f"…"` interpolates, `while` and `if` are
statements, and host functions are type-checked like native ones.

## State functions

These are read-only host functions. An empty player profile returns zero/empty values; the
launcher never invents worlds, servers or mods.

| Group | Functions |
| --- | --- |
| Build | `version()`, `branch()`, `prepared()`, `playing()`, `preparing()`, `progress()`, `status_text()`, `last_error()`, `appearance()`, `offline_name()` |
| Worlds | `world_count()`, `world_name(i)`, `world_folder(i)`, `world_mode(i)`, `world_last(i)`, `world_size(i)` |
| Servers | `server_count()`, `server_name(i)`, `server_ip(i)` |
| Mods | `mod_count()`, `mod_title(i)`, `mod_version(i)`, `mod_source(i)`, `mod_verdict(i)` |
| Accounts | `account_count()`, `account_name(i)`, `account_selected(i)` |
| Catalogue | `result_count()`, `result_title(i)`, `result_summary(i)`, `result_id(i)`, `result_source(i)`, `result_support(i)`, `result_installable(i)` |
| Instances | `instance_count()`, `instance_name(i)`, `instance_minecraft(i)`, `instance_id(i)`, `instance_selected(i)` |
| UI | `current_screen()`, `index_text(i)` |

## Actions

Buttons, links and inputs pass an action string. Rust handles it:

| Action | What it does |
| --- | --- |
| `screen:home` / `screen:mods` / `screen:servers` / `screen:settings` | switch screen |
| `prepare` | download Mojang files and prepare the selected build, with progress |
| `cancel` | cancel an in-flight prepare |
| `play` | start the selected instance |
| `play-world:<folder>` | start the game in a world |
| `stop` | stop the running game |
| `refresh` | reload the snapshot, servers and installed mods |
| `install-latest` | install the newest published game bundle |
| `search` | search the catalogue using `input("mods-search", …)` |
| `install:<project-id>` | install a catalogue project |
| `server-add` | add a server from `server-name` and `server-ip` inputs |
| `server-join:<ip>` | start the game and join a server |
| `server-remove:<index>` | remove a saved server |
| `account-add` | add and select an offline account from `account-name` |
| `account-select:<name>` | play as an offline name |
| `account-remove:<name>` | forget an offline name |
| `appearance:dark\|light\|system` | save the launcher appearance |
| `instance-select:<id>` | select an instance |
| `dismiss-error` | close the error toast |
| `quit` | close the preview |

Inputs are real `EditableTextState` fields: caret, selection, clipboard, undo and IME work. Their
text survives a redraw. `Enter` submits the field: search for `mods-search`, next field for
`server-name`, add for `server-ip` and `account-name`.

`Tab` and `Shift+Tab` walk inputs, links and buttons in tree order and draw a focus ring.
`Enter` activates the focused control.

## Keyboard

| Keys | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | next / previous input, link or button |
| `Enter` | submit the focused field or activate the focused button |
| `Ctrl+1` … `Ctrl+4` | Home, Mods, Servers, Settings |
| `Ctrl+Enter` | play |
| `Esc` | leave a focused input; otherwise close the preview |

The topbar brand area drags the window, and the minimize, maximize and close buttons are real.

## Development helpers

When the profile is empty, demo rows make screenshots reproducible:

```sh
SCORIA_MIMAS_DEMO=1 \
SCORIA_MIMAS_SCREEN=mods \
  cargo run -p scoria-ui --features dev-scripts --example mimas-ui
```

| Variable | Purpose |
| --- | --- |
| `SCORIA_MIMAS_UI` | path to another `.mim` scene |
| `SCORIA_MIMAS_SCREEN` | initial screen: `home`, `mods`, `servers`, `settings` |
| `SCORIA_MIMAS_MODE` | force `ready`, `preparing` or `playing` for screenshots |
| `SCORIA_MIMAS_DEMO` | show demo rows when the profile is empty |
| `SCORIA_MIMAS_INPUTS` | seed inputs, e.g. `server-name=SMP;server-ip=play.example.net` |
| `SCORIA_MIMAS_ACTION` | dispatch one action after the first draw |
| `SCORIA_MIMAS_FOCUS` | focus an input by id at startup |
| `SCORIA_MIMAS_DEBUG_KEYS` | print incoming keystrokes to stderr |

These helpers are for automation; they are not part of a player build.

## What is real and what is not

Real, through `scoria-core`:

- snapshot/version/prepared/playing state;
- prepare with live progress and cancel;
- start and stop the game;
- catalogue search, install and support verdicts;
- saved servers: add, remove, join;
- offline accounts: add, select, remove;
- appearance setting and instance selection;
- the same on-disk files the production launcher uses.

Still limited:

- every action rebuilds the whole tree; there is no incremental diff;
- no accessibility tree or screen-reader labels yet (the editable fields do have roles);
- `System` follows the production behaviour and currently resolves to the dark palette;
- no native system menu; only the three window buttons and dragging;
- no grid, constraints or measured text; the layout is flex-only. Widths adapt with
  `fill_w`, `w_pct` and `grow`, and pages scroll vertically;
- a few launcher features are still missing: remove-mod, language switch, while-playing,
  create/backup world, LAN list, modpack import.

The production UI remains Rust/GPUI + `scoria-ui-kit`. This branch is a proof that the whole
launcher shell can be scripted, not a decision to ship it.
