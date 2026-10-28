# ADR-0003: launcher UI waits for flint-ui

- Status: accepted
- Date: 2026-10-28
- Supersedes the timing of [ADR-0002](0002-gpui-x-hybrid.md), not its analysis

## Context

ADR-0002 proposed rewriting the launcher UI on GPUIX (React/Solid over GPUI) as a hybrid: the
Rust engine over JSON-RPC, the screens in TypeScript. The P3 spike had not started.

Redmoon now has its own UI project, **flint-ui**, at `/et/redmoon/flint-ui` on Entire. At the
time of this decision the repository is **empty** (no commits, no stack chosen yet), so there is
nothing to integrate against and no API to design for.

The launcher currently works: it is the upstream GPUI launcher, rebranded, with Basalt's offline
and import changes (P0–P2). It does not block any other work.

## Decision

1. **Do not start the GPUIX rewrite now.** P3 is paused.
2. **The launcher's UI target is flint-ui**, Redmoon's own UI, once it exists and its stack,
   component model and packaging are known.
3. **Keep the engine/UI split as the seam.** `launcher/` (Rust: installs, preparation,
   processes, CLI, MCP) stays the source of truth, and the IPC/JSON command surface ADR-0002
   described remains the integration point, whatever flint-ui turns out to be built with.
4. **Do not add a UI framework dependency in the meantime.** No GPUIX packages, no second
   frontend. The upstream GPUI screens stay until flint-ui replaces them screen by screen.

## Consequences

- The repo stays lighter: no `launcher-ui/` workspace, no Bun/Node requirement for players.
- The P3 spike on GPUIX (Linux/Wayland packaging, 82 MB app, pre-1.0 pinning) is not needed
  unless flint-ui adopts GPUIX itself.
- ADR-0002 remains useful as the analysis of the hybrid seam and of GPUIX's tradeoffs; its
  decision to build on GPUIX is superseded.
- When flint-ui has commits, write ADR-0004 with: its stack, the IPC contract, which screens move
  first (Home, Instances, Settings, Account), and how the bundle ships beside the Rust engine.

## Alternatives considered

- **GPUIX now, flint-ui later.** Two rewrites, and the first one would be thrown away; rejected.
- **Keep GPUI forever.** Possible, but it leaves the launcher UI in upstream's 4.2k-line
  `main.rs`; the user wants flint-ui, so the target is recorded here.
- **Rewrite on egui/iced/Slint/Tauri now.** Same cost as GPUIX with no relation to flint-ui;
  rejected until flint-ui's stack is known.
