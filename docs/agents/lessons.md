# Lessons: mistakes this fork has already made

Every entry is a real failure from this repository, with the rule that prevents it. Read this
before changing the release flow, CI, packaging or the UI theme. When a new mistake happens, add an
entry here and a test that catches it — a lesson without a check comes back.

## Releases

**L1 — Never reuse a release number.** `v0.1.0-beta.1` was rebuilt four times by moving the tag
while fixing Windows, publishing, then the digest check. Users cannot tell the builds apart and a
signature is bound to the tag.
Rule: one tag = one published build. A failed release is deleted and the next number is used.

**L2 — The tag belongs to the commit CI tested.** Tag after the staging run is green, on that
commit; never move `main` first.
Rule: `cargo xtask release` sets the tag itself and refuses a dirty tree or a missing CHANGELOG
section.

**L3 — A release ships both halves.** A launcher archive without the matching game bundle does not
play, and a game bundle without a launcher has no entry point.
Rule: the release workflow builds launcher and game on every platform and the artifact tests check
both archives before publishing.

**L4 — A published asset can be signed only for its own tag.** Renaming a release tag invalidates
`SHA256SUMS.sig`, because the signed message contains the tag. Re-signing needs the release key.
Rule: never rename a release tag; bump the version.

## Windows

**W1 — `VCRUNTIME140.dll` is not part of Windows.** The launcher closed instantly on a clean
machine because the Rust exes and the prebuilt GPUIX addon imported it.
Rule: `+crt-static` for Rust, build third-party addons from source with the same flag, and
`tools/check_imports.py` runs on every binary before publishing.

**W2 — A silent close is undiagnosable.** `start "" scoria-ui.exe` detached the console, so the
error vanished.
Rule: the UI runs in the console, writes `scoria-ui.log`, and `Scoria.cmd` pauses with the exit
code. Panics write `crash.log`.

## CI and tests

**C1 — Invalid workflow YAML fails before any job runs.** A duplicated `generate_release_notes`
key made every tag run fail with the workflow path as its name.
Rule: `cargo xtask check` validates the workflow files; a tag run that has no jobs is a YAML bug.

**C2 — Upload what the publish job expects.** The publish step failed on
`dist/*.tar.zst` because the package job uploaded only one file per matrix row.
Rule: artifact globs and publish globs are the same string, checked by the release test.

**C3 — pytest must not collect the unittest harnesses.** `tests/` holds Python harnesses that need
Mojang data; collecting them from `pyproject.toml` produced 79 collection errors.
Rule: pytest only collects `tests/pytest/`; harnesses stay out of the black-box suite.

**C4 — A green suite that proves nothing is a bug.** Tests that only assert "it did not throw"
hide the failure they were written for.
Rule: every fixed bug lands with a test that failed before the fix; run `test-suite-audit` before a
release.

## UI and design

**D1 — Contrast is a test, not a taste.** The light theme's ember `#E06A2C` with white text was
3.35:1, below WCAG AA.
Rule: the palette is checked in `scoria-ui-kit` tests at ≥ 4.5:1 for text in both themes.

**D2 — Do not mix design systems.** Pills, HeroUI radii and custom shadows in one window look
unfinished.
Rule: 4px spacing, 8px cards, 12px fields, one primary action per screen, no custom shadows in dark
mode.

## Process

**P1 — Entire is the source of truth; GitHub is a snapshot.** Development branches never reach
GitHub, and the public tree is an allowlist.
Rule: `tools/export_public.sh` plus the public-tree test; no `exp/*`, `spike/*` or internal docs in
the export.

**P2 — Research before rewriting.** The Windows failure looked like a missing DLL in one build and
like a broken addon in another; the log said which.
Rule: reproduce with the smallest command and read the log before changing architecture.
