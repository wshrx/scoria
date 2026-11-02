# Scoria architecture: the fork's view

Scoria reuses Horizon's engine and replaces the launcher. This page is the map of what the fork
touches and where the boundaries are; upstream's [architecture.md](../architecture.md) and
[AGENTS.md](../../AGENTS.md) describe the engine itself.

## Layers

```
launcher/ui/            gpui-ce window: Home, Mods, Servers, Settings   ← ours
launcher/ui-kit/        tokens, Ember theme, components                ← ours
launcher/               scoria-core: installs, prepare, process, API   ← fork of horizon-launcher
        │  direct Rust calls (no socket, no JSON bridge for the window)
horizon-client          the game                                      ← upstream, re-applied patches
        ├─ horizon-world       worldgen, chunks, NBT, blocks
        ├─ horizon-protocol    protocol 777
        ├─ horizon-command     Brigadier parser
        └─ horizon-core        Java-compatible math/RNG
```

The launcher is a **separate Cargo workspace** (`launcher/Cargo.toml`), so the game build never
waits for gpui. `launcher/xtask` is the only orchestration entry point: `check`, `test`, `pytest`,
`gate`, `ui`.

A release ships two artifacts from one tag:

- the **launcher** (`scoria`, `scoria.exe`) — the window plus the launcher engine, ~15 MB;
- the **engine bundle** (`scoria-game-*`) — `horizon-client`, `horizon-ctl`, `horizon-prepare`,
  `horizon-server`, exporters and notices, ~20–25 MB.

The launcher installs the engine bundle itself from the matching pre-release. No Minecraft assets
are in either archive.

## Accounts and game files

Scoria has no Microsoft account (ADR-0007). `launcher/src/account.rs` is offline profiles plus the
generic keychain used for the Play-with-friends token. There is no sign-in, no session handoff, no
`--session-file`; the game always starts offline with `--name`. Online-mode servers are refused
with a Scoria-specific message.

Preparation downloads the pinned 26.3 client, libraries, assets and server jar from Mojang's
piston-meta/piston-data into Scoria's own folder, or reuses an installation the player already has.
It never redistributes Mojang files. Downloads retry per file, keep partial progress and prefer
IPv4.

## Releases

One launcher tag (`vX.Y.Z-beta`) creates two GitHub Releases:

- `vX.Y.Z-beta` — launcher assets, `SHA256SUMS`, signature; normal, Latest.
- `vX.Y.Z-beta-game` — engine assets, `SHA256SUMS`, signature; pre-release, “do not download”.

Each release is signed with its own tag. The launcher's `newest_installable` selects the newest
bundle with a `scoria-game-*` asset; `-game` is the one pre-release it accepts. See
[`docs/public/RELEASE.md`](../public/RELEASE.md).

## Invariants

- **No Mojang files in the tree.** `tools/check_public_tree.py` runs on commit (`.githooks`);
  `.local/` stays gitignored.
- **Vanilla parity is the specification.** Scoria does not change engine behaviour beyond the 7
  patches in `docs/fork/engine-patches/`; upstream harnesses remain valid.
- **No session forgery.** Online-mode servers still need a real Microsoft session, which Scoria
  cannot provide; it only stops requiring one to start the game.
- **Do not rename `horizon-*` crates** or reformat upstream files: the engine must stay mergeable.

## Known gaps

- Engine sync: upstream `main` is 9 770 commits ahead; the patch series does not apply cleanly.
  Until it lands, `.hnmod` / `native-mod` files install but the engine does not run them.
- Windows first-run download/prepare/play is untested on real hardware.
- Auto-update and the in-app “Update engine” button are not implemented.
- macOS is deferred until signing/notarization.
- Modpacks and shaders have engine support but no launcher screen; CurseForge is unsupported.

## Workflows

`.github/workflows/` on the public mirror keeps `ci.yml` (public tests), `release.yml` (launcher +
engine releases) and the public-tree checks. Upstream's parity/nightly/registry workflows are not
part of Scoria. Entire keeps the private state and docs; GitHub gets an allowlisted snapshot.
