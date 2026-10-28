# ADR-0004: rename Basalt to Scoria

- Status: accepted
- Date: 2026-10-28

## Context

The fork was named Basalt in P0. That name is already used by another open-source Minecraft
launcher written in Rust (`MegalithOfficial/basalt-launcher`, and the shorter `basalt-launcher`
resolves to exactly that project), plus several unrelated well-known projects (`vkBasalt`,
`erikjuhani/basalt`). Keeping it — even as `basalt-launcher` — puts the fork in the same niche
under the same name.

## Decision

1. The product is **Scoria** (a volcanic rock, distinct from basalt and from the Minecraft block
   itself). The launcher is **Scoria Launcher**.
2. User-facing identifiers change: window title, macOS menu, crash screen, desktop entries,
   `Info.plist` display name, app id `dev.scoria.launcher`, keychain service
   `dev.scoria.launcher`, `PLAY.txt`, README/NOTICE, docs, translations, the launcher's Cargo
   description.
3. Internal identifiers stay: the `horizon-launcher` binary and CLI name, `HORIZON_*` environment
   variables, the `~/.local/share/horizon-launcher` data folder, crate names, and the engine's
   `horizon-client`. Renaming them would break installed versions, tests, harnesses and
   compatibility for no user-visible gain.
4. The Entire repository moves from `/et/redmoon/basalt` to `/et/redmoon/scoria`; the old one is
   deleted after the new one has every branch and tag. Local directory becomes
   `/home/shrx/dev/scoria`.
5. Release tags keep their meaning: `basalt-p*` become `scoria-p*` at the same commits.

## Consequences

- Existing installs keep working: data folders, keychain entries from the Basalt era are not
  read (the service name changed), so Microsoft sign-ins have to be redone once; offline names
  live in `accounts.json` and are unaffected.
- `packaging/macos/package.sh` still builds `Horizon.app` (P5 will rename the bundle and its
  assets); only display names and identifiers changed here.
- Attribution stays: Scoria is a fork of Horizon, MIT, with upstream notices in `NOTICE` and
  `THIRD_PARTY_NOTICES.md`.

## Alternatives rejected

- **`basalt-launcher`.** The exact name of another Rust Minecraft launcher; worse than the
  original.
- **Redmoon Launcher.** No conflict, but it merges the product with the studio name; Scoria keeps
  the launcher a distinct project under the Redmoon umbrella.
- **Nadir.** No launcher conflict, but a much more common word and less distinctive.
