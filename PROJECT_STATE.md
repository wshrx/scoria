# Scoria project state

Read this first after a context loss. Keep it current; do not turn it into a changelog.
Last updated: 2026-10-29.

## What this is

Scoria is an offline-first Minecraft: Java Edition 26.3 client and launcher written in Rust.
It is a fork of Horizon (MIT): the engine is upstream, Scoria adds offline play, a new launcher
UI, piston-meta downloads, Modrinth mods and its own branding. No Microsoft account, no telemetry,
no online-mode servers, no Minecraft assets.

- `origin` = Entire `et/redmoon/scoria`; `github` = `wshrx/scoria`; `upstream` = Horizon.
- `dev` = release line. `test/instances` = unfinished instance manager. Never touch `nightly`
  without an explicit request.
- Do not push without the user's word. Pushing to Entire is allowed when asked; GitHub only when
  explicitly asked.

## Product decisions

- **Do not rename the 29 `horizon-*` crates.** The engine stays mergeable with upstream. Scoria
  branding lives in the launcher, README, package and UI.
- **Instances are not in release.** They are behind `SCORIA_UI_INSTANCES=1` and on
  `test/instances`; create/delete/switch are not finished. The chosen game folder is preserved so
  installed Modrinth mods and resource packs do not disappear.
- **One Settings entry** (sidebar). Profile is static identity; theme lives in Settings.
- **One state-aware game action**: download+prepare / prepare / play / verify. No second button.
- **Visual thesis**: quiet SaaS control room; labelled sidebar, mesh + survey grid, cards, one
  violet accent, rounded controls, no `boxShadow`. Night and Lilac are complete themes.
- **Wayland only on Linux.** No X11/XWayland and no client-side titlebar; the compositor owns the
  frame. This is what makes layout switching and text input work.
- **No Minecraft art** in the UI. Scene art is painted from shapes.

## Architecture map

- `crates/` — Rust engine (client, world, protocol, assets, prepare, servers).
- `launcher/` — Rust launcher engine and `horizon-launcher api` JSON-lines API.
- `launcher-ui/` — React on GPUIX 0.11.0 + flint-ui 0.1.2.
  - `src/screens/registry.tsx` — the only screen list; rail and app render from it.
  - `src/screens/` — home, mods, servers, settings, test-only instances.
  - `src/components/layout/` — shell, rail, page header.
  - `src/api/` — `client` (process + JSON lines), `launcher`, `mods`, `types`.
  - `src/visual.tsx` — mesh, particles, mark, avatar, SceneThumb, Panel, Meter.
  - `DESIGN.md` — tokens, layout and regression rules; `bun run check:ui` enforces the mechanical ones.
- `.opencode/skills/scoria-ui`, `scoria-qa`, `project-memory`, `project-map`;
  `.opencode/agents/memory-keeper.md`.

## Done

- Offline identity, no Microsoft; Unicode names such as `Игрок` are valid and case-folded in any
  script. Engine tests cover it.
- piston-meta/piston-data download of 26.3: version JSON, client, platform libraries, asset index
  and objects; sha1 checked, resumable. Per-file retries (3), partial progress kept, stops after 8
  failures with a plain-language connection note; DNS prefers IPv4.
- Mojang files also prepare from official/Prism/Freesm/MultiMC/Modrinth/ATLauncher installs.
- Modrinth Mods page: mods/resource packs, Works/Partly/Doesn't run, progress, installed list,
  remove. Installed results now read **Installed** instead of offering Install again.
- Release UI: labelled sidebar (Play/Mods/Servers/Settings/profile), no instance switcher; Home is
  a Minecraft 26.3 dashboard; Instances/online note removed from release.
- Full Night/Lilac theme over rail, stage, cards, illustrations and particles.
- Wayland-native window: the compositor draws the frame; input, layout switching and text input use
  the native Wayland path. No X11, XWayland or client-side chrome. Default 1280x720.
- Keyboard layouts: the game uses `KeyEvent.text`, then falls back to the logical key, so Russian
  (and any non-Latin) typing works in chat, signs, books, anvils, search and dialogs.
- flint-ui 0.1.2 copied in (Field/Kbd/Combobox added; button spinner/icons kept).
- README rewritten for Scoria; DESIGN.md, skills, check:ui, package script.
- New worlds appear without restart: `App` polls `state` every 4 s while the engine is live.
- Worlds are keyed and launched by `folder` (from `worlds::list`), not by display name; the
  snapshot also carries `lastPlayedAt`, formatted with `Intl.RelativeTimeFormat`.
- Launcher version is `0.1.0` + `beta` (`launcher-ui/src/version.ts`), separate from the engine
  version (`data.version`, v0.2.93): rail badge and About show both.
- Every `t("...")` key has a Russian line; `bun run check:i18n` fails on a missing one.
- `dev` and `test/instances` are pushed to Entire; GitHub untouched.

## Bugs fixed (and why they happened)

- **Black corners/rectangles**: transparent window + `boxShadow` on rounded surfaces. Removed both;
  the window is opaque and the compositor draws its frame.
- **Text wrapped one word per line**: GPUIX multi-node interpolation; compose one template string.
- **Theme only on sheets**: fixed chrome colours; now everything reads `theme`.
- **Three Settings entries**: profile/Configure/rail/theme toggle; now only rail.
- **Technical noise**: file counts/logs/hashes moved under Settings → Show details.
- **ASCII-only names**: `valid_offline_name` now Unicode alphanumeric + `_`; lowercase compares use
  `to_lowercase`.
- **Download aborted on one asset**: retries, keeps partial, 8-error cap, friendly note.
- **New world missing until restart**: the snapshot was fetched once on mount, and the game writes the
  world after `play` returns → poll `state` every 4 s while live.
- **Cyrillic could not be typed**: `KeyEvent.text` is empty without XIM, and the UI ran on
  XWayland → Wayland-only launcher plus a logical-key fallback in every game text field.
- **Two worlds named "Новый мир" collided**: the API sent only the display name, so React keys
  duplicated and Play could open the wrong folder → send `folder` and key/launch by it.
- **Profile error appeared at the bottom and the field stayed normal**: the message used the shared
  page note and the input was never marked → `Field error` under the input plus `Input invalid`;
  local regex validates the Unicode name and clears the error on typing.
- **About said "Лаунчер v0.2.93"**: the snapshot version is the engine → add `Engine` row and show
  launcher `0.1.0 · beta` from `version.ts`.
- **Mods progress/remove jumped the layout and notes sat at the page bottom**: reserve a fixed
  progress row showing the file label, keep the note in the search panel, fixed-width Remove button.
- **World time and modes were English**: `lastPlayedAt` + `Intl.RelativeTimeFormat`; `t(world.mode)`.

## Known gaps / next

- Modpacks in the new UI: engine can import/port; no screen yet. CurseForge unsupported.
- Shaders: Scoria's `.hshader` packs came from Horizon's registry, removed; Modrinth shader packs
  are OptiFine/Java and do not run. No shader UI until a Scoria shader source exists.
- LOD at max: `lodDistance=1024` + `resolutionScale=200` shows approximate distant peaks; lower
  LOD to 256/512, resolution to 100, Full Detail 12–16. Consider a warning/cap in Graphics.
- Instances: finish create/delete/switch on `test/instances`; then decide release.
- Cyrillic in the game: the source fix is tested, but a version folder still carries the old
  `horizon-client` until it is rebuilt and copied into `~/.local/share/horizon-launcher/versions/`.
- Windows/macOS are not tested; the launcher UI and the game are Linux-first.

## Run and check

```sh
cd launcher && cargo test                 # 196 lib/tests
cargo test -p horizon-client --bin horizon-client cyrillic   # layout fallback
cd launcher-ui && bun run typecheck && bun run check:ui && bun run check:i18n
cd launcher-ui && bun run dev             # 1280x720, native Wayland
sh tools/package_ui.sh                    # dist/scoria-launcher-linux-x86_64.tar.gz
```

No push without permission. Update this file only with facts, not narrative.
