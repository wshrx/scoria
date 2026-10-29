---
name: scoria-ui
description: Scoria launcher UI rules. Use for any change to launcher-ui design, screens, layout, theme, window chrome or player-facing copy. Loads the house design system and the black-corner/shape regressions to avoid.
---

# Scoria UI

Read `launcher-ui/DESIGN.md` before changing the launcher.

Non-negotiables:

- graphite SaaS surfaces, one violet action, purple/mint/amber status; rounded controls;
- Night and Lilac apply to sidebar, topbar, stage, backdrop and every screen;
- no `boxShadow` on rounded or transparent surfaces; use 1 px borders and layered surfaces;
- Linux window: Wayland-native, compositor decorations, 1280x720 default; no X11 code;
- one navigation entry per screen; Settings only in the rail; profile is not a button;
- one state-aware game action, no duplicate download/prepare buttons;
- no technical file counts, hashes or logs outside Settings > Show details;
- Unicode offline names;
- compose interpolated text into one template string; empty drag/resize regions need pointerEvents.

Run `bun run check:ui`, `bun run typecheck`, then a headless mount. Do not push without explicit
permission.
