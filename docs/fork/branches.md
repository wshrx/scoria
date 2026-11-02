# Branches

Scoria keeps two long-lived branches on both remotes. Nothing else is needed.

| Branch | Where | Purpose | Rules |
|---|---|---|---|
| `main` | Entire + GitHub | Stable Scoria / the public snapshot | Fast-forward only, after the full gate; no direct work |
| `dev` | Entire | Integration and testing; everything lands here first | Build, `cargo xtask gate` green before merging into `main` |
| `staging` | GitHub | Exported public snapshot used by public CI | Pushed by `tools/publish_public.sh`; promoted to `main` only when green |

There is no `nightly`, `lab`, `test/instances` or `feat/*` branch right now. Feature work may use a
short-lived `feat/*` branch on Entire; merge it into `dev` and delete it. Do not publish WIP
branches to GitHub.

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
