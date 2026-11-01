# ADR 0001: rewrite the launcher UI on gpui-ce, one process

- Status: accepted
- Date: 2026-11-02

## Context

The launcher is two processes: a Bun-compiled React UI (`launcher-ui/`, on the GPUIX native
addon) that talks JSON lines to the Rust engine (`launcher/`). It works on Linux, but on a clean
Windows machine it closes the moment it starts, and a prebuilt native addon plus a second runtime
is the wrong base for a desktop app that must run on Linux, Windows and macOS.

Horizon's launcher is a single Rust binary on GPUI, which is why it needs no addon, no Bun and no
Visual C++ redistributable.

## Decision

Rewrite the UI in Rust on **gpui-ce 0.3** (Apache-2.0) as one process:

- `launcher/` keeps the engine as `scoria-core`;
- `launcher/ui-kit/` is `scoria-ui-kit`: semantic tokens, the Ember theme and components. It does
  not depend on the engine, the file system or the network;
- `launcher/ui/` is `scoria-ui`: the window, screens and state; it calls the engine directly;
- `launcher/xtask/` is the single entry point for checks, tests, packaging and releases.

The JSON-lines API stays for the CLI and MCP, not for the window.

## Why gpui-ce and not Zed's gpui

Both are Apache-2.0. Zed's `gpui` 0.2.2 is what Horizon ships and is proven on the three targets;
`gpui-ce` is a community fork with the web-like styling (transitions, hover styles, blur) the
Scoria design needs. The API is close enough to move between them, and the version is pinned.

## Why not the alternatives

- **Slint**: GPLv3 or a paid licence. Scoria is MIT; copyleft is not acceptable.
- **Tauri/wry**: a webview is still a browser engine, and the point is a native window.
- **Dioxus desktop / Yew**: webview or wasm/DOM.
- **Freya**: native (Dioxus + Skia), but 0.5 RC and a small component ecosystem.
- **Electron**: not a candidate for a 150 MB launcher.

## Consequences

- The React UI is removed at cutover; until then it remains the shipped UI.
- No `vcruntime140.dll`, no `.node`, no Bun: the Windows build is one binary plus assets.
- The UI tests move to `gpui::test` and headless screenshots.
- The design is rebuilt on HeroUI's semantic structure with Scoria's palette.
- The migration is a rewrite, not a port: screens land one at a time behind a feature flag.
