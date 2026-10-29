---
name: Project map
description: Find the right Scoria file before reading a large tree. Use when asked where something lives, how the launcher UI is structured, or after moving files.
---

# Project map

Read `docs/agents/project-map.md` first. Follow a path only for the task at hand.

- UI screens are registered once in `launcher-ui/src/screens/registry.tsx`.
- Layout/window chrome is in `launcher-ui/src/components/layout/` and `launcher-ui/src/chrome.ts`.
- Engine calls are in `launcher-ui/src/api/`; the Rust side is `launcher/src/api.rs`.
- UI invariants and known GPUIX traps are in `launcher-ui/DESIGN.md`.

When a path moves, update the map in the same change and keep the entry one line.
