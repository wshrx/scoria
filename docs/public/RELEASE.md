# Release engineering

The tag workflow (`.github/workflows/release.yml`) builds the **launcher** and the **game bundle**
for Linux and Windows (macOS is deferred until signing and notarization). They are published as
two releases:

| Release | Tag | Contents |
|---|---|---|
| Launcher (Latest) | `v0.1.0-beta` | `scoria-launcher-*`, `SHA256SUMS`, signature |
| Engine bundle (pre-release) | `v0.1.0-beta-game` | `scoria-game-*`, `SHA256SUMS`, signature |

A player downloads only the launcher. The launcher discovers the engine release and installs the
matching `scoria-game-*` itself, so the engine can be updated without replacing the launcher.

| Target | Runner | Archive |
|---|---|---|
| Linux x86_64 (Wayland) | `ubuntu-latest` | `scoria-launcher-linux-x86_64.tar.zst` |
| Windows 10/11 x64 | `windows-latest` | `scoria-launcher-windows-x64.zip` |

Tag format: `v0.1.0-beta` (a `beta`/`alpha` tag becomes a GitHub pre-release).

## Assets

Nothing in a release may contain Minecraft code, textures, sounds, language files or other game
data. Archives carry binaries, the UI, the bundled SIL OFL fonts and Scoria/Horizon branding only.
The game files are downloaded from Mojang on request or read from an installation the user owns.
`tools/package_native_client.py` includes prepared resources and is therefore **private only**;
never attach its output to a release. `tools/check_public_assets.sh` rejects game media and
decompiled Java in the public tree.

## What a launcher archive contains

One binary, `scoria` (`scoria.exe` on Windows). The fonts, the mark and the engine are inside it:
there is no Bun, no native addon and no Visual C++ redistributable. Windows builds use
`+crt-static`; macOS ships `Scoria.app` with the binary in `Contents/MacOS`.

## The game client is not in these archives

A fresh launcher installs a **game version** from the engine release asset
(`scoria-game-linux-x86_64.tar.zst`, `scoria-game-macos-arm64.zip`,
`scoria-game-windows-x64.zip`) and prepares Mojang resources locally. The engine release is marked
pre-release so it stays out of the Latest slot, but its API entry is public: the launcher has no
GitHub token.

1. Build `horizon-client` and its helper binaries for each target in CI (matrix job).
2. Package binaries only — **never prepared Minecraft assets**; first play prepares them from the
   user's files or piston-meta.
3. Sign the engine release's own `SHA256SUMS` with the tag `v0.1.0-beta-game`. The launcher looks
   up checksums for the release that carries the asset, so both releases are signed separately.
4. Keep the engine tag derived from the launcher tag (`vX.Y.Z-game`); a newer engine alone can use
   a later engine tag, and the launcher installs the newest bundle with a `scoria-game-*` asset.

## Branches and channels

- `main` — the public stable line. A release is `dev` merged into `main`, then a tag on `main`.
- `dev` — integration and the beta line; every push is a candidate.
- `test/instances` — unfinished instance manager, never published.
- `nightly` — **upstream Horizon's branch**, inherited with the fork. Scoria does not release from
  it and never pushes to it: its history is Horizon's, not ours.

Tags are the only release channel: `v0.1.0-beta.N` (pre-release) and `v0.1.0` (stable). A nightly
channel, if wanted, is a rolling `nightly` pre-release tag built from `dev` on a schedule — still
not a branch.

## Signing

- macOS: ad-hoc `codesign` in `package_macos.sh`; Gatekeeper still asks once. Notarization needs an
  Apple Developer ID and `notarytool`, which is not configured.
- Windows: unsigned; SmartScreen warns. Authenticode needs a certificate.
- Linux: no signature; SHA-256 only.

## Checklist for a release

- [ ] `cargo xtask gate` (fmt, clippy, Rust tests, pytest)
- [ ] `cargo test` in `launcher/`, and `cargo test -p horizon-client --bin horizon-client`
- [ ] Fresh-machine install test on each OS
- [ ] Screenshots in `docs/public/screenshots/` match the build
- [ ] `SHA256SUMS` attached
- [ ] No `.opencode/`, `PROJECT_STATE.md`, `docs/agents/` or `dist/` in the public tree
