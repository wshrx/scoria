# ADR 0002: use Horizon's mod registry as the first catalogue

- Status: accepted
- Date: 2026-11-02

## Context

The engine's native mods, modpacks and shader packs live in Horizon's registry
(`playhorizon.dev/mods`, index at `horizon-rust.github.io/horizon-mods/index.json`). Scoria had
disabled that source while it was undecided whether to depend on another project's service; only
Modrinth was searched.

The owner's decision: use Horizon's registry.

## Decision

`browser::sources()` returns Horizon's registry first and Modrinth second. The launcher's default
source (search, install) is Horizon; Modrinth remains for Java mods the engine can port, shown with
the same support labels.

## Consequences

- Native `.hmod` content is installable again; its index, signatures and takedowns are checked by
  `horizon-mods::index` before a file reaches the game folder.
- Scoria now depends on Horizon's public infrastructure. If the index moves or disappears, the
  source fails with Horizon's error; Modrinth still works, and `HORIZON_MOD_INDEX` can point at a
  mirror or a Scoria index in the same format.
- Ratings and report links go to `playhorizon.dev`; the launcher never signs in.
- The long-term plan stays: host a Scoria index in the same format when the fork has its own
  moderation. Until then, this is a deliberate dependency, not an accident.
