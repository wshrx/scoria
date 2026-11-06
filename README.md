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

## What this branch is

`exp/mimas` is a research branch, not a release. It asks one question:

> Can interface geometry be described by a small typed script while GPUI still does the
> rendering?

The answer so far: **yes for absolute positioning and dev tooling; not yet for a full UI
layout engine.**

The example queue maps one-to-one:

```mimas
fn draw() {
    clear();
    rect(64.0, 208.0, 592.0, 440.0, "#141416", 20.0);
    text(96.0, 262.0, 28.0, "#F4F0EA", "Every node has x, y, w, h.");
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

- **Positioning can live in a script**: x/y/w/h, radius, colours and text are computed by Mimas.
- **Rust keeps the renderer**: nodes become normal GPUI `div`s; the VM never touches the window.
- **No user code**: the example and the dev harness are opt-in; a player build does not include
  the VM or load scripts from the data folder.

## Limits

- No flow layout, flex/grid, constraints, text measurement or auto-sizing in the VM yet.
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
