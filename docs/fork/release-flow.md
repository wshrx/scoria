# Draft and showcase: Entire → GitHub

Two repositories, two jobs. They are not two development lines.

| | Entire (`origin`) | GitHub (`github`) |
|---|---|---|
| Role | draft, full history, agents, heavy harnesses | showcase, clean history, releases |
| Branches | `dev`, `test/*`, `main` | `staging`, `main` |
| Content | everything, including memory/tools | allowlisted product tree only |
| Tests | full suite + agents on a runner with `.local` data | public CI: UI gates, launcher tests, engine check, bundle |
| Releases | tags for internal builds | tags + artifacts + release notes |

## Flow

1. Work lands on Entire `dev`. Agents run the heavy tests there (or on a runner with the private
   `.local` data).
2. `tools/export_public.sh` builds `dist/public` by allowlist: no `.opencode`, memory, fork notes,
   game media or build output.
3. `tools/publish_public.sh` commits that tree as one snapshot on GitHub `staging` and pushes it.
4. GitHub Actions (`ci.yml`, from `public-ci.yml`) runs on `staging`: UI typecheck/design/i18n
   gates, launcher tests, engine check, UI bundle.
5. Only a green `staging` is promoted to GitHub `main` (fast-forward or squash). A tag on `main`
   runs `release.yml` and attaches the three launcher archives plus `SHA256SUMS`.
6. Entire `main` is moved to the same tested commit at release time; it is a pointer, not a second
   line of work.

GitHub never receives `dev`, `nightly`, `test/instances`, agent files or game assets. There is no
automatic merge from GitHub back to Entire: the snapshot is one-way.

## Launcher

The launcher is part of the same snapshot and the same flow:

- `launcher-ui` is Scoria's UI and goes public in full.
- `launcher/` is the engine/CLI/API (no window); public CI runs its tests.
- `crates/` is Horizon plus the small fork patch; `docs/fork/patches.md` tracks it.
- `release.yml` (public) builds `scoria-ui` + `horizon-launcher` for Linux/macOS/Windows, embeds
  the Scoria icon from `tools/make_icons.sh`, and attaches the archives to the tag.

## Commands

```sh
sh tools/export_public.sh      # filtered tree in dist/public
sh tools/publish_public.sh     # snapshot -> github staging
# after CI is green:
#   merge staging into main, tag v0.1.0-beta.N, release.yml builds the archives
```
