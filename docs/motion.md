# Motion in the launcher

Branch: `exp/motion`. This is where launcher animation work is tried; nothing here ships until it
is merged into `dev` and included in a release.

## Principles

- **Short**: 120–200 ms for state changes, 220–280 ms for entering panels. Nothing loops.
- **Ease-out** on entry, ease-in on exit; no bounce, no spring.
- **Never move content under the pointer** while the player is clicking.
- **One motion per action**: a page fades in, it does not also slide and scale.
- **Reduced motion**: `SCORIA_REDUCED_MOTION=1` (and later the system setting) turns every animation
  into an immediate state change.
- **No layout jank**: animate opacity/transform/colour, not width/height, unless the layout is
  already isolated.
- The launcher sleeps when nothing changes; animations must not keep a timer alive afterwards.

## Candidates

| Where | Motion | Notes |
|---|---|---|
| Screen switch | 120 ms fade of the content column | keep the rail and titlebar still |
| Rail collapse | 160 ms cross-fade of labels + width step | width animation needs a layout-safe wrapper |
| Theme switch | existing 9×20 ms overlay fade | keep, tune to 180 ms |
| Toast | 140 ms fade/slide up, 120 ms out | do not delay the action it confirms |
| Prepare progress | 180 ms width interpolation | already reports percent; smooth the bar |
| Mods list | 90 ms fade-in per card, capped at 6 cards | no stagger on scroll re-renders |
| Settings dropdown | 120 ms fade/4 px rise | the menu already expands in place |
| Buttons | existing 140 ms background/border transition | keep one duration |

## Constraints in gpui-ce

- `transitions(...)` animates colours/borders; use it for hover/press/theme.
- Opacity needs the existing timer pattern (`Shell::fade`), or a small helper that drives one
  `cx.notify()` per frame and stops at zero.
- Width/height animation needs a state field and a per-frame rebuild; only do it where the layout
  cannot reflow the page.
- Do not add an animation framework; one small helper in `scoria-ui-kit` is enough.

## QA

- `cargo xtask gate`.
- Screenshot Home, Mods, Servers and Settings in dark and light.
- Toggle `SCORIA_REDUCED_MOTION=1`: no timers, no visual delay, same final state.
- Watch idle CPU after the animation ends (must return to ~0%).
- Window resize below 1080 px: the rail collapse and page fade must not clip.
