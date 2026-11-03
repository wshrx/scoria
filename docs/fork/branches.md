# Branches

Scoria keeps two long-lived branches on both remotes. Nothing else is needed.

| Branch | Where | Purpose | Rules |
|---|---|---|---|
| `main` | Entire + GitHub | Stable Scoria / the public snapshot | Fast-forward only, after the full gate; no direct work |
| `dev` | Entire | Integration and testing; everything lands here first | Build, `cargo xtask gate` green before merging into `main` |
| `staging` | GitHub | Exported public snapshot used by public CI | Pushed by `tools/publish_public.sh`; promoted to `main` only when green |

There is no `nightly`, `lab` or `test/instances` branch. Work branches are short-lived and live on
Entire until merged or dropped:

| Prefix | Use | Examples |
|---|---|---|
| `exp/<topic>` | throwaway experiment; never released as-is | `exp/gpui-fast`, `exp/motion` |
| `feat/<topic>` | a shippable feature | `feat/engine-update`, `feat/installer` |
| `fix/<topic>` | a bug fix | `fix/prepare-progress` |
| `test/<topic>` | verification only; no release | `test/native-mods`, `test/windows-smoke` |
| `sync/<date>` | temporary upstream sync; delete after merge | `sync/upstream-20261102` |

Merge `feat/*`, `fix/*` and `test/*` into `dev` and delete the branch; keep `exp/*` only while it is
being evaluated. Do not publish WIP branches to GitHub.

## From an experiment to dev

An experiment is a sketch, not a release candidate:

```text
exp/<idea>          try it; screenshots, throwaway code, no test requirement
      │  liked?
      ▼
feat/<idea>         start from dev, port only the good parts, add tests
      │  cargo xtask gate + review
      ▼
dev                 merge and delete the feature branch
      │
      ▼
main / tag          only from a green dev snapshot
```

- **Never merge `exp/*` into `dev` as-is.** Port the parts that survive into a fresh `feat/*`
  branch, or cherry-pick the clean commits.
- **Automated tests belong in the `feat/*`/`fix/*` branch**, not in `exp/*`. A feature is not ready
  for `dev` until its test fails without it. `test/*` is only for manual QA plans and black-box
  checks that need a real machine.
- If an experiment is rejected, delete the branch (or keep it explicitly as a reference, like
  `exp/gpui-fast`).
- A redesign follows the same rule: `exp/redesign-*` is for visual directions; the chosen one moves
  to `feat/redesign-*` with tokens, screenshots in dark/light, responsive checks and tests. The
  design-system boundary stays `launcher/ui-kit/`.

```sh
git switch -c feat/motion dev
git cherry-pick <clean exp commit>       # or copy the files
cargo xtask gate
git switch dev && git merge --no-ff feat/motion
git branch -d feat/motion && git push origin --delete feat/motion
```

## Working

```sh
git switch dev
# ... change, test ...
git commit
git push origin dev
```

Merging into `main`:

```sh
git switch dev && git pull --ff-only
cargo xtask gate
git switch main && git merge --ff-only dev && git push origin main
```

A change that cannot fast-forward (because `main` moved) goes back to `dev` with a rebase
(`git switch dev && git rebase main`) and is tested again.

## Hosting

| Remote | Where | Carries |
|---|---|---|
| `origin` | Entire, `/et/redmoon/scoria` (private) | everything: `main`, `dev`, private docs, checkpoints |
| `github` | `git@github.com:wshrx/scoria.git` (public) | only `main` and `staging` |
| `upstream` | Horizon | the engine, fetch-only |

- `git push` goes to **Entire** (`remote.pushDefault = origin`).
- `git push github` sends the configured refspecs; it never publishes `dev`.
- Tags stay on Entire; push one to GitHub only when a release needs it.
- `flint-ui` is a different repository (the UI project), not the launcher.

## Releases

The public release is cut from a tag on the GitHub `main` snapshot. The release workflow publishes
two GitHub Releases from one launcher tag:

- `vX.Y.Z-beta` — the launcher, `SHA256SUMS`, signature; Latest.
- `vX.Y.Z-beta-game` — the engine bundle, `SHA256SUMS`, signature; pre-release, “do not download”.

The launcher finds the engine release and installs it. See `docs/public/RELEASE.md`.
