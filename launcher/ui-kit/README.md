# scoria-ui-kit

The launcher's small component library. It knows tokens, gpui and nothing about the engine, the
network or the file system, so a component can be rendered in a test or reused by another Scoria
window.

## What is here

| Module | Gives |
| --- | --- |
| `tokens` | `Theme` with semantic colours, radii, spacing, type scale and motion; dark and light. Contrast is tested in both. |
| `badge` | Status pills and `tone_for_support` for catalogue labels. |
| `button` | `Button` with Primary/Secondary/Tertiary/Outline/Ghost/Danger and three sizes. |
| `card` | `card` for a page surface, `soft_card` for a nested region. |
| `page` | `page_header`, `empty_state`, `error_panel`: the same hierarchy on every screen. |
| `progress` | `progress_bar` and `clamp_fraction` for the download card. |
| `toast` | The transient bottom-right message. |

## Rules

- Take every colour, radius, gap and type size from `Theme`; no literals in a component.
- One primary action per context. Card surfaces carry hierarchy; borders separate, not decorate.
- A component returns `Div`/an element and the caller appends actions and data. The kit never
  calls the engine.
- Add a test for a pure helper, a contrast pair or a size rule; do not pin pixels a theme may move.
- When a pattern appears on two screens, move it here instead of copying it.

The window is `launcher/ui`: `app.rs` holds state and actions, `app/screens/` one file per screen,
and `app/assets.rs` the bundled fonts, icons and backdrop. The shared language tables are
`launcher/lang/{ru,fr}.json`; both the CLI and the window read them through
`scoria_core::i18n::tr_in`, and a missing key falls back to English. A test scans both crates for
`tr(`/`trf(` and `self.t(`/`self.tf(` calls.
