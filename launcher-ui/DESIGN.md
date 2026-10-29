# Scoria launcher UI: design and regression rules

## Thesis

A quiet control room for a living world. The product is a desktop launcher, so the UI is
operational first: compact type, clear columns, one accent, and surfaces that stay readable over
the painted backdrop. Rounding is a house rule; shadows are not.

## Tokens

- Type: Medeo Sans; Medeo Mono for metadata and numbers. Page titles 28/32, section titles 14–17,
  body 13, metadata 10–11.
- Space: 4 px grid; page gap 18; card padding 14–24; sidebar 220 (64 when narrow).
- Radius: controls 8–10, cards 14, window 24, pills 999. Never 0 on a component that has a
  background, never a new arbitrary radius.
- Colour: graphite page, one violet action, mint for success, amber for pending, red for danger.
  Night and Lilac are complete themes: rail, topbar, stage, backdrop and every card.
- Surface: `theme.colors.card` over the mesh, `theme.colors.muted` for inset rows,
  `theme.colors.border` (1 px) for separation. **No `boxShadow`**: on the X11 window it paints the
  black rectangles that were reported around avatars, active rail items and window corners.
- Background: painted mesh, survey grid and dust. No Minecraft art, no screenshots, no chart-like
  skyline.

## Layout

- Sidebar: brand, current instance, Play / Instances / Mods / Servers, Settings at the bottom,
  static profile. The sidebar is the only page navigation and the only Settings entry.
- Topbar: engine status and window controls. Profile is not repeated there.
- Content: page header (28 px title, one-line subtitle, one action), then cards. Max width 1360.
- Home: current-instance hero with Play or Download+Prepare, Continue; three stats; Worlds list;
  Mods and Servers shortcuts. No file counts, hashes or logs.
- Instances: cards, current instance outlined, Use in place.
- Mods: Modrinth search, support pill, install progress, installed list.
- Servers: list, add form, offline-mode note.
- Settings: Profile, Appearance, Language, While playing; Game files with one state-aware action;
  Builds; About. File-by-file logs and sha1 live behind **Show details**.

## Interaction rules

- One action per state: download+prepare / prepare / play / verify. Never two buttons that do the
  same thing.
- One way into Settings. The profile is identity, not a button. Do not add a second Configure
  entry.
- Theme changes `theme.ts` only; no fixed chrome colours in screens.
- Download failures are shown in plain language; technical detail stays in the console/log.
- Offline names accept Unicode; do not fall back to ASCII validation.
- Every custom control has `aria-label`, a visible focus state, and a hit area of at least 32 px.
- Motion: page fade, meter easing, spinner dots, particles. No layout animation. Keep rounded
  controls; no new shadows.

## Window rules

- Linux is frameless through X11: `_MOTIF_WM_HINTS` plus a rounded SHAPE, refreshed on resize and
  every 250 ms. If SHAPE is unavailable, restore native decorations instead of leaving a black
  inset.
- Drag from the topbar; eight resize edges; minimize, maximize and close in the topbar.
- Default window 1280×720. The content must fit there; pages scroll, cards never squeeze.
- Do not reintroduce `transparent: true` or a shell shadow. Both caused the edge artefacts.

## Known GPUIX traps

- Multi-node text (`{a} · {b}`) can wrap as one word per line; compose one template string.
- Empty drag/resize elements need `pointerEvents: "auto"`, or they never receive the press.
- `boxShadow` on rounded/transparent surfaces paints black rectangles on X11.
- A window resize can outrun React's size subscription; the SHAPE timer is the safety net.

## Check

`bun run check:ui` enforces the mechanical rules: no `boxShadow` in UI surfaces, no `src/pages`,
no `onPage("settings")` outside the registry, 1280×720 default, no transparent window.
