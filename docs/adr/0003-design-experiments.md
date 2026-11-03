# ADR-0003: launcher visual language

- Status: accepted; the palette is still being chosen between experiments.
- Date: 2026-11-03.
- Supersedes the earlier “square shell” note in this file.

## Context

Three visual directions were built as separate branches:

- `exp/redesign` — Blueprint: cool paper, ink, safety orange, drafting-table rules.
- `exp/redesign-brawl` — Arcade: saturated yellow/navy/red, chunky radii and outlines.
- `exp/redesign-dungeons` — Dungeons: mossy stone, parchment, amber, deep teal.

The first pass made the titlebar and rail look like floating square panels. That is not the
language we want: the **window itself is one rounded surface**, the custom titlebar belongs to it,
and the controls inside are rounded and modern.

## Decision

1. **One rounded window.** The top-level window is transparent and the root content is clipped to
   a 14–18 px radius. The titlebar, rail and content are parts of that single surface, not cards
   floating inside it. The compositor may add its own shadow; we do not fake one.
2. **Custom titlebar stays.** Drag region, minimise/maximise/close, Scoria name and launcher
   version. Controls are rounded and inset; the titlebar is full width inside the rounded shell.
3. **Modern inner controls.** Cards, fields, buttons, badges and world markers use a consistent
   rounded scale. One primary action per screen; hierarchy comes from surfaces and spacing, not
   borders everywhere.
4. **Wayland identity is part of the design.** The window sets `app_id = "scoria"`; the launcher
   writes `~/.local/share/applications/scoria.desktop` (`Icon=scoria`,
   `StartupWMClass=scoria`) and the Scoria hicolor icons on start. The old
   `horizon-launcher.desktop` is removed on install. This is what makes the panel/taskbar show the
   right icon and group the window correctly.
5. **Experiments are not releases.** A chosen direction moves from `exp/*` to `feat/redesign-*`,
   gets tokens, tests and Wayland screenshots, then merges into `dev`.
6. **Original assets only.** No Minecraft, Supercell or other third-party assets, logos or
   characters. A direction may be inspired by a mood; the palette, shapes and copy are Scoria's.

## Consequences

- `dev` and the release keep the original gpui-ce look until a direction is promoted.
- A rounded window needs a transparent window background and a clipped root; the resize grips and
  titlebar drag stay inside that root.
- The panel icon is no longer an afterthought: `app_id`, desktop entry and icon are tested together.
- Do not merge an `exp/*` branch as-is; port the chosen parts into `feat/*`.
