# Changelog

All notable changes to Scoria. The release workflow takes the section whose heading matches the
tag (`v0.1.1-beta` → `## [0.1.1-beta]`) and uses it as the GitHub Release body, so the notes
live here, in one place, and not in tag names.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html): during the beta every published build
raises the patch and keeps the `-beta` suffix (`0.1.0-beta`, `0.1.1-beta`, ...).

## [Unreleased]

Nothing yet. The next published build becomes `0.1.1-beta`: every beta raises the patch number, and
unpublished work stays untagged in the repository history.

## [0.1.0-beta] — 2026-10-29

First public beta: a Linux and Windows launcher plus the matching 26.3 game bundle, offline by
default.

### Added

- Offline play with no Microsoft account: offline profiles, LAN worlds and offline-mode servers.
  Online-mode servers are refused with an explanation instead of a broken login.
- A launcher that installs the game itself: piston-meta download, local resource preparation,
  version management and world backups.
- Modrinth mods and resource packs with **Works**, **Partly** and **Doesn't run** labels before
  installing.
- New UI: dark and light Ember themes, Russian and English, one state-aware Play action, one
  Settings entry, keyboard navigation and toasts.
- Scoria branding and icons, signed release checksums (`SHA256SUMS` + Ed25519 signature).

### Changed

- Scoria numbers its own releases (`v0.1.N-beta` during the beta); Horizon's `0.2.x` numbers stay
  upstream.
- Windows binaries link the C runtime statically, including the GPUIX addon built from source, so
  no Visual C++ redistributable and no extra DLL are needed.
- Linux archives ship a `.tar.zst` twin beside the `.tar.gz`; the launcher prefers the smaller
  zstd one and still installs gzip-only releases.

### Fixed

- The Windows launcher closed instantly on a clean machine because `VCRUNTIME140.dll` was missing.
- Release CI now checks every Windows binary's DLL imports and fails the build instead of the
  player.
- The online-mode message names Scoria, not Horizon.

### Known limitations

- Online-mode servers need a Microsoft session and are unsupported by design.
- macOS is deferred until signing and notarization; there is no macOS archive.
- Windows builds are not code-signed yet: SmartScreen warns until a certificate is bought.
- CurseForge is not supported; the engine can import and port modpacks, the launcher has no
  modpack browser yet.
