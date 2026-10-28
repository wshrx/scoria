# ADR-0005: two remotes, Entire for everything, GitHub for releases

- Status: accepted
- Date: 2026-10-28

## Context

The launcher lives in two places: Entire (`/et/redmoon/scoria`, private) and GitHub
(`wshrx/scoria`, public). Entire is where the whole history, experiments and the `dev` branch
belong; GitHub is what players and contributors see, and it should not carry every work-in-progress
branch or the upstream tag noise.

Pushing the same set to both by hand is error-prone: `git push` with two remotes either asks every
time or silently picks one.

## Decision

1. **`origin` = Entire** (`entire://aws-eu-central-1.entire.io/et/redmoon/scoria`). It is the
   primary: `git push` sends the current branch there, and every branch and tag lives there.
2. **`github` = GitHub** (`git@github.com:wshrx/scoria.git`). It carries only:
   - `main` — stable;
   - `nightly` — preview builds.
   `dev`, feature branches and release tags stay on Entire (tags may be pushed to GitHub for a
   release later, deliberately).
3. **The push refspecs encode that.** `remote.github.push` is set to `main` and `nightly` only, so
   `git push github` from any branch sends exactly those two. `remote.github.tagOpt` is
   `--no-tags`, so a fetch never drags upstream tags in.
4. **`remote.pushDefault = origin`**, so a bare `git push` goes to Entire even from a branch without
   an upstream yet.
5. A branch is only moved to GitHub after it has passed the checks in `branches.md`; `dev` never
   appears there.

## Consequences

- `git push` (Entire) is safe for work in progress; `git push github` is the deliberate "publish"
  step and cannot accidentally publish `dev`.
- GitHub's default branch is `main`; `nightly` is a normal branch one can clone.
- Contributors without an Entire account can still read/clone GitHub; Entire stays private.
- The old `wshrx/flint-ui` repository is **not** the launcher: it is the UI project, on Entire as
  `/et/redmoon/flint-ui` too.

## Alternatives rejected

- **GitHub only.** Loses the private full-history host and forces every experiment into a public
  repo.
- **Entire only.** Contributors and players cannot clone without an Entire account.
- **Two identical mirrors.** Noise: `dev`, feature branches and hundreds of upstream tags on
  GitHub, plus two force-pushes to keep aligned.
