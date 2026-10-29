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
  `theme.colors.border` (1 px) for separation. **No `boxShadow`**: it paints the
  black rectangles that were reported around avatars, active rail items and window corners.
- Background: painted mesh, survey grid and dust. No Minecraft art, no screenshots, no chart-like
  skyline.

## Layout

- Sidebar: brand, Play / Mods / Servers, Settings at the bottom, static profile. The sidebar is
  the only page navigation and the only Settings entry. Instances are a test-only screen
  (`SCORIA_UI_INSTANCES=1`), never in a release build.
- Topbar: engine status and window controls. Profile is not repeated there.
- Content: page header (28 px title, one-line subtitle, one action), then cards. Max width 1360.
- Home: Minecraft 26.3 hero with Play or Download+Prepare, Continue; three stats; Worlds list;
  Mods and Servers shortcuts. No file counts, hashes or logs.
- Instances: test-only, hidden from the release registry until create/delete/switch are complete.
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

## Feedback

- **Toasts** (`components/feedback/toast.tsx`): one bottom-right stack, three at most, success/info
  4 s, errors 7 s. Every screen uses `useToast()` for completed or failed actions; close buttons
  are icon buttons with a focus ring.
- **Confirm dialog** (`confirm.tsx`): destructive actions ask through `useConfirm()`; it traps Tab,
  closes on Escape or the backdrop, and never uses a second click on the row.
- **Skeleton** (`skeleton.tsx`): lists that have not loaded show row-shaped placeholders, not
  `Loading…`.
- **Empty state** (`empty-state.tsx`): an icon tile, one short title, one line, optional action.
- **Error boundary** (`error-boundary.tsx`): a render error shows a panel with the message and
  Retry instead of a blank window.
- **Keyboard**: rail items and world rows are tab stops with Enter/Space, and `focusVisible` draws
  the ring; buttons/inputs already carry one.

## Window rules

- Linux runs on Wayland only. The compositor owns the frame: titlebar, drag, resize and rounded
  corners. The app draws no titlebar and no transparent padding.
- Default window 1280×720. The content must fit there; pages scroll, cards never squeeze.
- Do not reintroduce `transparent: true` or a shell shadow. Both caused the edge artefacts.

## Known GPUIX traps

- Multi-node text (`{a} · {b}`) can wrap as one word per line; compose one template string.
- Empty drag/resize elements need `pointerEvents: "auto"`, or they never receive the press.
- `boxShadow` on rounded surfaces paints black rectangles at the corners.
- Never force `WINIT_UNIX_BACKEND=x11`; XWayland breaks layout switching and text input.

## Check

`bun run check:ui` enforces the mechanical rules: no `boxShadow` in UI surfaces, no `src/pages`,
no `onPage("settings")` outside the registry, 1280×720 default, no transparent window.
