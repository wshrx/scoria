# Workflows disabled in Scoria

These are Horizon's release, nightly, parity, game-build, launcher, registry and telemetry-server
workflows, moved out of `.github/workflows/` so GitHub never runs them on this fork.

- `release.yml`, `game-build.yml`, `launcher.yml` build and publish Horizon's signed releases with
  Horizon's keys and repository variables. Scoria must build its own; running these would publish
  to the wrong place, or fail on missing secrets.
- `parity.yml`, `parity-main.yml`, `run-parity.yml` expect self-hosted machines holding a copy of
  Minecraft 26.3 and a dispatcher app. Scoria has neither.
- `nightly.yml`, `nightly-build.yml`, `nightly-command.yml` publish Horizon's Nightly channel.
  Scoria's `nightly::BASALT_NIGHTLY` is off.
- `mod-registry.yml` and `telemetry-server.yml` deploy Horizon's registry and telemetry ingest.
  Scoria removes both.
- `triage-command.yml` is Horizon's issue-triage bot.

`ci.yml` stays: it runs the public-tree check, formatting, clippy and tests, and needs nothing
private. `cancel-on-close.yml` stays: it only cancels superseded runs.

Restore a workflow by moving it back into `.github/workflows/` once Scoria has its own
infrastructure for it.
