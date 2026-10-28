# ADR-0002: Launcher on GPUIX, hybrid with the Rust core

- Status: superseded in timing by [ADR-0003](0003-launcher-ui-flint-ui.md): the launcher waits for flint-ui; the hybrid analysis stands
- Date: 2026-10-28

## Context

The launcher is 12.7k lines of Rust UI (`launcher/src/main.rs` 4.2k, `backend.rs` 4.0k, `cli.rs`
2.7k, plus ~40 screens) on **GPUI 0.2.2** (`launcher/Cargo.toml:11`) — the same renderer Zed uses.

The request is to "rewrite the launcher on GPUIX". GPUIX (`https://gpuix.dev`, packages
`@gpuix/react`, `@gpuix/solid`, `@gpuix/native`) is **not** a renderer or a GPUI fork: it is a
React/Solid reconciler over GPUI, bridged through napi-rs (desktop) and wasm-bindgen (browser).
The app is TypeScript, runs on Bun or Node, and ships a native `.node` addon. It is pre-1.0
(0.2.x) and the docs say to pin the adapter and the native package to the same exact version.

What GPUIX gives: React components, hot reload, a native window, file picker, window controls,
font registration, screenshots/automation, `cargo-packager` packaging and a signed updater.
What it does not give: keychain access, process supervision, MCP, GitHub release installation,
Minecraft preparation, or any of the launcher's Rust backend.

## Decision

**Hybrid.** Keep `launcher/` as the engine and build `launcher-ui/` in GPUIX:

```
launcher-ui (TS/React, Bun)  ──JSON-RPC──▶  launcher --ipc (Rust)
     screens, state                            installs, prepare, launch, servers, accounts
```

- The Rust side gains an IPC mode (newline-delimited JSON over stdio or a unix socket) reusing the
  existing `cli.rs`/`mcp.rs` command surface. It stays the single place that touches the file
  system, keychain, processes and the network.
- The GPUIX side owns layout, navigation, theming, localization and accessibility. No business
  logic in TypeScript beyond view state and RPC calls.
- Packaging: `bun build --compile` for the UI, `cargo-packager` for the bundle, the Rust engine as
  a sidecar binary. The updater points at Basalt's own releases (P5), never Horizon's.

## Why not the alternatives

- **Full TypeScript rewrite.** Reimplements keychain, installs, preparation, process supervision,
  MCP and the CLI in JS: weeks of work, and it throws away code that is already tested (265
  launcher tests). It also couples the whole launcher to a pre-1.0 framework.
- **Keep Rust GPUI, restyle only.** Cheapest, but it is not what was asked and it keeps UI code in
  the 4.2k-line `main.rs` monolith.
- **egui / iced / Slint / Tauri.** Each is a rewrite of the same magnitude as GPUIX without the
  GPUI renderer the project already uses; Tauri adds a web view, which GPUIX explicitly avoids.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| GPUIX is pre-1.0, breaking changes between 0.2.x | pin `@gpuix/*` to one exact version, no ranges; upgrade deliberately |
| Linux support is younger than macOS | run the P3 spike on this machine (Wayland + GPU) before committing; fall back to the Rust UI if packaging fails |
| No keychain/process APIs in JS | they stay in the Rust engine over IPC; the UI never touches secrets |
| Bigger download (~82 MB app per GPUIX docs) | acceptable; the game dwarfs it; revisit if the launcher ships standalone |
| Two runtimes to ship and debug | the engine is already a separate binary; the UI is one compiled executable |
| Accessibility/IME gaps in a young framework | verify keyboard focus and text input in the spike; keep the CLI/MCP path fully usable regardless |

## Consequences

- `launcher/src/main.rs` and the ~40 `*_page.rs` files are retired screen by screen, not deleted in
  one commit; the Rust CLI and MCP remain supported.
- Until P3 lands, the fork keeps working on the upstream GPUI UI; P1 (offline-first) does not
  depend on the rewrite.
