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
- **Draft → showcase**: Entire is the draft; GitHub is a filtered snapshot. `tools/publish_public.sh`
  pushes `dist/public` to GitHub `staging`; public CI (`public-ci.yml`) tests it; only green
  `staging` is promoted to `main` and tagged. There is no merge from GitHub back to Entire.
- **Entire is the working remote** (all branches, project memory, QA fixtures, agent skills).
  **GitHub is the public showcase**: curated history, README/screenshots, release tags and
  artifacts. Never push `.opencode/`, `PROJECT_STATE.md`, `docs/agents/` or internal QA fixtures
  there; keep them on Entire or in a private tools repo. Push GitHub at milestones/tags, not WIP.
- **Builds are release artifacts, not commits**: CI builds on a tag, attaches the tarball plus a
  SHA-256 file to the GitHub Release, and never stores the 100 MB binary in git.
- **First public beta**: GitHub release `v0.2.98` (game tag) carries Linux/Windows launchers and
  game bundles, `SHA256SUMS` and a valid `SHA256SUMS.sig` (Scoria key). Fresh-home
  `install_latest` accepted the signature and installed `v0.2.98`; Linux prepare + game start
  were verified. Tags `v0.2.94`–`v0.2.97` were retries and were deleted; only `v0.2.98` remains.
- **Microsoft cut**: launcher accounts, the server `--check-accounts` switch, the client
  `--session-file`/`HORIZON_SESSION_FILE` path and the Microsoft account document are gone.
  There is no way to hand the game a Microsoft session; online-mode servers are refused with a
  Scoria-specific message. The engine's dormant online-mode/auth module stays for server parity
  but is unreachable without a session.
- **Entire tidy**: `main` fast-forwarded to `dev`; `lab`, `feat/launcher-ui` and `nightly` are
  deleted (local and remote). The old nightly UI line is kept as the tag
  `archive/nightly-ui` (`6433f621b`). Arch/remote now has only `main`, `dev`, `test/instances`.
- **Online-mode message**: Scoria cannot join online-mode servers (no Microsoft session). The
  disconnect text now says Scoria and points to offline-mode/LAN; the `v0.2.98` binary still has
  the old Horizon wording until the next game build.
- **Linux game bundle proven**: `tools/package_game.sh` builds `horizon-linux-x86_64.tar.gz`
  (26 MB, no Mojang assets). On a fresh `HORIZON_LAUNCHER_HOME` it was recognised as `v0.2.94`,
  downloaded 26.3 from piston, prepared 1.4 GB of resources locally, and launched the client.
  Windows has the same job (`package_game_windows.sh` + `release.yml`); it has not run yet.
  Signing the checksums is the remaining install blocker.
- **Cross-platform releases**: `.github/workflows/release.yml` builds Linux x86_64, macOS arm64
  and Windows x64 launchers on a tag. `tools/package_{macos,windows}.sh` make `Scoria.app` and
  the Windows zip. See `docs/public/RELEASE.md`: signed client bundles are the remaining step
  before the mac/win downloads can play on a clean machine.
- **Public showcase**: `docs/public/README.md` + `README.ru.md` are the GitHub landing (downloads,
  what works, screenshots); `tools/export_public.sh` builds `dist/public/` by allowlist, without
  `.opencode/`, memory, fork notes, game screenshots or build output. Screenshots are still to
  capture.
- **Legal line (Freesm-style)**: say plainly that Scoria is an independent Horizon fork, not
  affiliated with Mojang/Microsoft, and ships no Minecraft assets. Releases carry binaries, UI,
  OFL fonts and branding only; game files are downloaded from Mojang on request or taken from the
  user's install. `tools/check_public_assets.sh` gates the public tree.
- **Download resilience**: IPv4-only resolution when the host has A records, five attempts per
  file and a 64-error cap; a flaky route no longer aborts a 5 200-file asset download.
- **Engine patch queue**: `docs/fork/engine-patches/*.patch` (7 commits touching `crates/`);
  `tools/check_fork_surfaces.sh` fails if the input fallback, no-session rule, offline-only
  server, release key or icon disappears after a sync.
- **Rewrite (2026-11-02)**: the launcher UI moves to Rust on gpui-ce in the same process;
  `launcher/ui-kit` (tokens, Ember theme, components), `launcher/ui` (window, screens), and
  `launcher/xtask` (the only entry point: check, test, pytest, gate, ui). The React UI stays the
  shipped one until parity, then `launcher-ui/`, Bun and GPUIX are removed. See
  `docs/adr/0001-gpui-ce-rewrite.md`.
- **Design**: HeroUI semantic structure (4px rhythm, 8px radius, 12px fields, one primary action)
  with Scoria's basalt surfaces and ember accent; dark by default, light at parity.
- **Release state**: `v0.1.0-beta.1` is the only tag and the only release (pre-release, 8 assets,
  Linux `.tar.gz` + `.tar.zst`, Windows `.zip`, signed). The Horizon-numbered `v0.2.98`/`v0.2.99`
  releases and tags were deleted; `latest` now resolves to the beta. GitHub `main` == `staging`
  at `d58211061`; `protect-main` blocks deletion and force-pushes only, so a green staging can be
  fast-forwarded to main without a PR.
- **Versioning**: Scoria is its own product, so releases are `v0.1.0-beta.N` (pre-release) and the
  launcher reports the tag (`HORIZON_LAUNCHER_VERSION`); Horizon's 0.2.x numbers apply to upstream
  only. `v0.2.98`/`v0.2.99` were Horizon-numbered previews and are removed once the beta is out.
- **Windows runtime**: the Rust exes and the GPUIX addon are built with `+crt-static`
  (`tools/build_gpuix_native.sh`); `tools/windows_runtime.sh` ships `vcruntime140.dll` only if
  `tools/check_imports.py` would see it, and release CI runs that check on the Windows folder.
- **Archives**: Linux ships `.tar.gz` and a `.tar.zst` twin (22-25% smaller, faster to unpack); the
  launcher prefers the zstd one and still installs gzip-only releases.
- **Upstream sync**: `docs/fork/patches.md` lists every fork patch; `tools/sync_upstream.sh`
  fetches Horizon, merges on a throwaway branch and runs the gates. Engine delta is 9 files
  (+87/−26, only the typed-text fallback) on top of `b6c936f82`, but upstream is **781 commits
  ahead** (`9c15e18c6`, 2026-10-29): a large sync is due, engine patch re-applied after taking
  upstream. The launcher is the real fork.
- **Branches**: `main` = public stable (release = merge `dev`, then tag on `main`); `dev` =
  integration/beta; `test/instances` = unpublished. There is no `nightly` branch: nightly builds, if
  added, are a rolling pre-release tag built from `dev`.

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
- `launcher/` — Rust launcher engine and `horizon-launcher api` JSON-lines API. The legacy GPUI
  window is gone: the crate is CLI/API only (the 33 GUI modules and `gpui` dependency were removed).
- `launcher-ui/` — React on GPUIX 0.11.0 + flint-ui 0.1.2.
  - `src/screens/registry.tsx` — the only screen list; rail and app render from it.
  - `src/screens/` — home, mods, servers, settings, test-only instances.
  - `src/components/layout/` — shell, rail, page header.
  - `src/components/feedback/` — toast, confirm, skeleton, empty state, error boundary.
  - `src/qa.ts` — screenshot fixtures behind `SCORIA_UI_QA`; inert in a normal launch.
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
- Mojang files: first prepare downloads the pinned 26.3 from piston into Scoria's own folder;
  other launchers are no longer scanned (a folder chosen in Settings is the only exception).
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
- Brand: the chosen Scoria mark (`s2`, the motion cube) is the source for the `.ico`, `.icns`,
  hicolor PNGs, the game icon and the in-app `<Mark>` (`launcher-ui/src/assets/scoria-mark.png`).
  `tools/make_icons.sh` regenerates from `docs/public/logo.svg`.
- Medeo Sans/Mono are registered **before** the first frame in `launcher-ui/app.tsx`; a React
  effect left the first paint in the fallback. Credits are in About, THIRD_PARTY_NOTICES and the
  public README; the wordmark is outlined Medeo.
- SaaS P0 feedback layer: global toasts, confirm dialogs, list skeletons, empty states, an error
  boundary, tooltips in the compact rail and keyboard focus on rail/world rows.
- Legacy launcher window removed: `scoria-launcher-ui` is the only UI; the Rust binary is the
  engine/CLI (release ~16 MB, 23k lines fewer, no GPUI/sculk partner art). Translations pruned to
  the strings the engine still uses.
- One game per folder: `.scoria-running` holds the PID; a second Play is refused, Home shows
  "Playing" and disables Play, the 4 s poll clears a stale mark after a crash.
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
- **An empty 30 px band sat under the Mods search**: the progress row was always
  reserved to avoid a jump → render it only while an install is running; the panel
  grows for the duration and collapses afterwards.
- **Input grew and its focus ring looked like a thick border**: after Medeo was
  registered before the first frame, the native editor's font metrics pushed the
  wrapper past 32 px and the 2 px outline read as a second border → the wrapper is
  34 px with `overflow: hidden`, the editor 30 px, and the ring is 1 px at offset 2.
- **World time and modes were English**: `lastPlayedAt` + `Intl.RelativeTimeFormat`; `t(world.mode)`.
- **A second game copy started on a second Play**: `play` spawned and forgot the child → a
  per-folder PID mark refuses a second launch, a waiter clears it on exit, and the snapshot
  exposes `playing` so the button is disabled.

## Known gaps / next

Order of work: (1) release pipeline on GitHub (tag → CI artifact → checksums), (2) P1 UI
(Ctrl+K, dashboard numbers, mod sorting, update centre, onboarding), (3) modpacks and shaders,
(4) Windows/macOS if they are promised.

- Modpacks in the new UI: engine can import/port; no screen yet. CurseForge unsupported.
- Shaders: Scoria's `.hshader` packs came from Horizon's registry, removed; Modrinth shader packs
  are OptiFine/Java and do not run. No shader UI until a Scoria shader source exists.
- LOD at max: `lodDistance=1024` + `resolutionScale=200` shows approximate distant peaks; lower
  LOD to 256/512, resolution to 100, Full Detail 12–16. Consider a warning/cap in Graphics.
- Instances: finish create/delete/switch on `test/instances`; then decide release.
- Cyrillic in the game: verify by typing in chat and a sign. The local `v0.2.93` client was
  rebuilt with the fix (`horizon-client` in its version folder); the old binary is backed up at
  `/tmp/opencode/horizon-client-v0.2.93.old` until a release build supersedes it.
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
