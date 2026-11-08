# scoria-hooks

The launcher's developer hook engine, as a standalone library. It has no GPUI dependency.

```text
Event  ──▶  every .mim script  ──▶  Command
              (typed host API)
```

The launcher sends an event, each script gets its matching hook, and the returned commands are
applied by the launcher. The crate owns:

- the OS watcher (`notify`, no folder polling);
- one compiled VM per script file;
- per-file error isolation — a broken script does not stop the others;
- the capability filter (`capabilities` file);
- public state getters (`prepare_done`, `last_ok`, `screen_name`, …).

## Usage

```rust
use scoria_hooks::{Event, Hooks};

let mut hooks = Hooks::new("launcher/ui/dev/scripts");
hooks.set_snapshot(true, false);
let result = hooks.dispatch(Event::PrepareDone {
    ok: true,
    error: String::new(),
});
for command in result.commands {
    // apply Toast / Play / Install / AddServer / ...
}
for error in result.errors {
    log(error);
}
```

Hot reload:

```rust
if let Some(changes) = hooks.watch() {
    // debounce changes, then dispatch Event::Reload
}
```

## Scripts

See [`docs/mimas-scripts.md`](../../docs/mimas-scripts.md) and
[`launcher/ui/dev/scripts`](../ui/dev/scripts). The library is only pulled in by
`scoria-ui`'s `dev-scripts` feature; a player build contains no Mimas runtime.

## Why a library

The engine is independent of the window, so it can be used by:

- the launcher UI;
- the CLI (`horizon-launcher` recipes);
- headless tests and screenshot harnesses;
- a future separate project, without changing its API.

It is deliberately not a full UI framework: rendering, input, focus and accessibility stay in
Rust/GPUI.
