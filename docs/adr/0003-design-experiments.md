# ADR-0003: launcher design experiments

- Status: accepted for the workflow; the visual direction is still open.
- Date: 2026-11-03.

## Context

Three visual directions were built as separate branches:

- `exp/redesign` — Blueprint: cool paper, ink, safety orange, drafting-table rules.
- `exp/redesign-brawl` — Arcade: saturated yellow/navy/red, chunky radii and outlines.
- `exp/redesign-dungeons` — Dungeons: mossy stone, parchment, amber, deep teal, rounded inner
  controls.

They share the gpui-ce architecture and the custom titlebar. None is merged. The experiments also
showed that an outer rounded shell does not match the main launcher: the window/titlebar/rail are
square, only inner controls are rounded.

## Decision

1. `exp/*` is a sketch, never released as-is. A chosen direction moves to `feat/redesign-*`, gets
   tokens/tests/screenshots, then merges into `dev`.
2. The **custom titlebar stays**: drag region, minimise/maximise/close, Scoria name and launcher
   version.
3. The **window shell is square**: titlebar full width, rail flush. Buttons, fields, cards, badges
   and world markers may be rounded.
4. No Minecraft, Supercell or other third-party assets, logos or characters. Palettes, shapes and
   copy are original; a direction may be *inspired by* a mood, not copied.
5. Do not mix GUI stacks. `exp/gpui-fast` remains a separate experiment; a switch would need a
   `feat/gpui-fast` branch with a real text input and measurements.

## Consequences

- The release (`dev`, `v0.1.2-beta`) keeps the original gpui-ce look until a direction is promoted.
- A redesign is a product decision, not a merge of an untested experiment.
- Screenshots are captured on Wayland with `WAYLAND_DISPLAY`, without scripting the live compositor.
