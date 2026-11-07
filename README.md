<p align="center">
  <img src="launcher/ui/assets/scoria-mark.png" width="72" alt="Scoria">
</p>

<h1 align="center">Scoria · exp/mimas</h1>

<p align="center">
  <b>An experimental Mimas → GPUI layout playground.</b><br>
  Positions, sizes and text come from a <code>.mim</code> script; Rust paints the nodes.
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2026-000?style=flat-square&logo=rust">
  <img alt="GPUI" src="https://img.shields.io/badge/GPUI-CE-FF6A3D?style=flat-square">
  <img alt="Mimas" src="https://img.shields.io/badge/Mimas-0.4-9C948C?style=flat-square">
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-111214?style=flat-square">
</p>

![Mimas layout experiment](screenshots/home.webp)

| Home | Mods |
| --- | --- |
| ![](screenshots/home.webp) | ![](screenshots/mods.webp) |

| Servers | Settings |
| --- | --- |
| ![](screenshots/servers.webp) | ![](screenshots/settings.webp) |

All four screens are Mimas trees. Navigation and the Home action change preview state;
the backend calls (download, launch, Modrinth, servers, accounts) are still Rust-side work.

---

## What this branch is

`exp/mimas` is a research branch, not a release. It asks one question:

> Can interface geometry be described by a small typed script while GPUI still does the
> rendering?

The answer so far: **yes for absolute positioning and dev tooling; not yet for a full UI
layout engine.**

The script builds a small flex tree and reloads when it changes:

```mimas
fn draw() {
    clear();
    column(24.0, 64.0, 1280.0, 720.0, "#0B0B0D", 0.0);
        text("Scoria", 56.0, "#F4F0EA");
        row(12.0, 0.0, 0.0, 0.0, "#00000000", 0.0);
            column(10.0, 20.0, 192.0, 52.0, "#FF6A3D", 12.0);
                text("Draw with Mimas", 16.0, "#1A0A04");
            end_box();
        end_box();
    end_box();
}
```

```text
launcher/ui/examples/mimas_ui.rs   Mimas host functions → absolute GPUI nodes
launcher/ui/dev/mimas-ui/scene.mim  the scene in the screenshot
```

## Run it

```sh
cd launcher
cargo run -p scoria-ui --features dev-scripts --example mimas-ui
```

Use another scene:

```sh
SCORIA_MIMAS_UI=/path/to/scene.mim   cargo run -p scoria-ui --features dev-scripts --example mimas-ui
```

## What it proves

- **Layout can live in a script**: rows, columns, gaps, padding, align, justify, grow, wrap, fixed sizes, radius, colours and text are computed by Mimas; Rust maps them to GPUI flex elements.
- **Rust keeps the renderer**: nodes become normal GPUI `div`s; the VM never touches the window.
- **Live edit**: the example watches the `.mim` file and repaints within ~300 ms; this is the
  first step towards React/Vue-like scripting, not a full framework.
- **No user code**: the example and the dev harness are opt-in; a player build does not include
  the VM or load scripts from the data folder.

## Limits

- The flow is a row/column subset with align, justify, grow and wrap; no grid, constraints, text measurement or auto-sizing yet.
- No focus, accessibility or event routing from Mimas; those stay in Rust.
- A full UI DSL would need a real layout pass and a security model before it could ship.

## The rest of Scoria

The main launcher (D · Signal / E · White), docs, build instructions and release links live on
`dev`, `main` and `staging`:

- [`README.md`](https://github.com/wshrx/scoria/blob/dev/README.md)
- [`docs/`](https://github.com/wshrx/scoria/tree/dev/docs)
- [Latest release](https://github.com/wshrx/scoria/releases)

> Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft.
> Scoria contains no Minecraft code, assets or data; you need your own copy of Minecraft: Java
> Edition 26.3.

## State and buttons

Buttons have an action string. Rust updates a tiny preview mode and calls `draw()` again; the
script reads it with `preparing()` / `playing()`:

```mimas
if preparing() {
    button("Подготовка 42%", "ready", "#2A1710");
} else if playing() {
    button("Остановить", "ready", "#1A1A1D");
} else {
    button("Играть", "play", "#FF6A3D");
}
```

This is a full-tree rerender, not a diff. It is enough to prove the state → event → draw loop;
keys, focus and incremental patching are still missing.

## Measurements

Measured on the development machine (release, 1280×720, X11/XWayland, 3 runs, median):

| Build | Binary | RSS after 3 s | Window appears |
|---|---:|---:|---:|
| launcher, no Mimas feature | 50 MB | 142.0 MB | 248–281 ms |
| same launcher, `dev-scripts` feature, no script | 53 MB | 143.4 MB | 254–271 ms |
| same launcher + a script that runs `draw()` | 53 MB | 145.5 MB | 254–271 ms |
| Mimas preview example (no launcher core/assets) | 35 MB | 125.8 MB | 252–254 ms |

So in the same app the VM plus one compiled scene costs **~3.9 MB RSS (+2.8%)** and **~3 MB of
binary**; startup is within noise (a few ms). The preview example is *lighter* than the launcher
because it does not load the launcher's backdrop, fonts, instance data or engine code — do not use
it as a "UI replaced by Mimas" comparison.

Reproduce:

```sh
cd launcher
cargo build --release --features dev-scripts -p scoria-ui
SCORIA_DEV_SCRIPTS=$PWD/ui/dev/scripts target/release/scoria
```

The `dev-scripts` feature is still developer-only; a player build does not include the VM.
