# Project map

Short index for agents. Read only what the task needs.

| Area | Path | Notes |
|---|---|---|
| Scoria state | `PROJECT_STATE.md` | Current decisions, done, bugs, gaps. Read first. |
| UI rules | `launcher-ui/DESIGN.md` | Tokens, layout, regression checklist. |
| Screens | `launcher-ui/src/screens/registry.tsx` | Add a screen = component + one entry. |
| Layout | `launcher-ui/src/components/layout/` | shell, rail, topbar, controls, resize, page header. |
| Engine API client | `launcher-ui/src/api/` | `client.ts` JSON lines; `launcher.ts`; `mods.ts`. |
| Window chrome | (none) | Wayland compositor decorations; no client-side chrome. |
| Visuals | `launcher-ui/src/visual.tsx` | Mesh, particles, SceneThumb, Panel, Meter. |
| Theme | `launcher-ui/src/theme.ts` | Night/Lilac tokens; no fixed chrome colours. |
| UI checks | `launcher-ui/scripts/check-ui.ts` | no shadow, one Settings entry, 1280x720, no transparent window. |
| Translations | `launcher-ui/src/i18n.tsx`, `scripts/check-i18n.ts` | Every `t("...")` key needs a Russian line. |
| QA workflow | `.opencode/skills/scoria-qa/SKILL.md` | Gates, page screenshots, state checklist. |
| Launcher version | `launcher-ui/src/version.ts` | `0.1.0` + `beta`; the snapshot `version` is the engine. |
| Launcher engine | `launcher/src/api.rs` | `state`, `prepare`, `play`, `mods`, `servers`, versions. |
| Downloads | `launcher/src/piston.rs` | piston-meta 26.3, sha1, retries in `mojang.rs`. |
| Packs/mods | `launcher/src/browser/` | Modrinth source, install/remove, support labels. |
| Offline names | `launcher/src/account.rs` | Unicode validation and case folding. |
| Game | `crates/horizon-client/` | Engine; don't rename. LOD: `gfx/lod.rs`, `graphics.md`. |
| Docs | `docs/` | Engine docs; fork decisions in `docs/fork/`. |
| Test branch | `test/instances` | Instance manager behind the flag; not release. |

## Verification

`cargo test` (launcher), `bun run typecheck`, `bun run check:ui`, headless mount, `tools/package_ui.sh`.
The UI runs on native Wayland (`scripts/run.ts` / wrapper); X11 and XWayland are unsupported.
