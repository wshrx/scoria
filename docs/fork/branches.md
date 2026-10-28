# Branches

Scoria uses three long-lived branches. `main` is what players get; `dev` is where work is tried;
`nightly` is disposable.

| Branch | Purpose | Rules |
|---|---|---|
| `main` | Stable Scoria. Releases are cut from here. | Only fast-forward/merge from `dev` after the full check below. No direct work. |
| `dev` | Integration and testing. Everything lands here first. | Build, `cargo test`, clippy, fmt and the Python tests must pass before merging into `main`. |
| `nightly` | Experimental builds and risky spikes. | May be rebased or reset at any time; never a base for `main`. |

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
cargo test --manifest-path launcher/Cargo.toml
python3 -m unittest discover -s tests
git switch main && git merge --ff-only dev && git push origin main
```

A change that cannot fast-forward (because `main` moved) goes back to `dev` with a rebase
(`git switch dev && git rebase main`) and is tested again.

## Hosting

`origin` is `git@github.com:wshrx/flint-ui.git`; `upstream` stays Horizon for engine updates.
Pushes to `main`, `dev` and `nightly` go to GitHub. No agent-checkpoint tooling is part of the
repository any more.

## Notes

- `nightly` is a branch name only: Scoria has no Nightly build channel (P2 removed it). Do not
  expect auto-published builds from it.
- A tag named `nightly` exists from upstream, so push the branch with
  `git push origin refs/heads/nightly` (or delete the local tag).
