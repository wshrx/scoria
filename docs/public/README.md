<p align="center">
  <img src="logo.svg" width="88" alt="Scoria">
</p>

<h1 align="center">Scoria</h1>

<p align="center">
  <b>A launcher and client for Minecraft: Java Edition 26.3, written in Rust.</b><br>
  Offline by default. No Microsoft account. No telemetry. No game assets.
</p>

<p align="center">
  <a href="#download">Download</a> ·
  <a href="#what-it-is">What it is</a> ·
  <a href="#assets-and-ownership">Assets</a> ·
  <a href="#what-works">What works</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-FF8A4C">
  <img alt="Platforms" src="https://img.shields.io/badge/platform-Linux%20·%20macOS%20·%20Windows-1f2937">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-D8B26A">
  <img alt="No assets" src="https://img.shields.io/badge/ships%20no%20Minecraft%20assets-2f855a">
</p>

## Download

**Download only the launcher.** `scoria-launcher-*` is the file a player needs.
`scoria-game-*` is the engine bundle: the launcher downloads and installs it itself, so you do
**not** download it by hand.

| Platform | File to download | Start |
|---|---|---|
| Linux x86_64 | `scoria-launcher-linux-x86_64.tar.zst` | unpack, then `./scoria` |
| Windows 10/11 x64 | `scoria-launcher-windows-x64.zip` | unzip, then `scoria.exe` |

1. Download the launcher for your system and start it.
2. Press **Download 26.3** once. Scoria installs the matching game bundle and fetches Mojang's
   26.3 files into its own folder: about 600 MB on the first run, with a percentage on screen.
3. Press **Play**.

The release page also lists `scoria-game-*` for transparency, plus `SHA256SUMS` and its signature.
Those are not player downloads: leave them to the launcher. SmartScreen warns about `scoria.exe`
until the build is code-signed.

**The first beta is out:** [v0.1.0-beta](https://github.com/wshrx/scoria/releases/tag/v0.1.0-beta).

| Platform | Status |
|---|---|
| Linux x86_64 (Wayland, Vulkan) | `v0.1.0-beta` |
| Windows 10/11 x64 (Vulkan) | `v0.1.0-beta` |
| macOS 14+, Apple silicon | deferred until signing and notarization |

## What it is

Scoria is a native launcher and client. It reads vanilla worlds, follows vanilla rules and starts
without a session. Saves, options and mods stay on your machine.

**Scoria is an independent fork of [Horizon](https://github.com/horizon-rust/horizon) (MIT).** The
engine and parts of the launcher come from Horizon; Scoria adds the offline launcher, piston-meta
downloads, Modrinth support, the new UI and its branding. It is not an official Minecraft product
and is not approved by or associated with Mojang or Microsoft.

- **No account.** Offline profiles, LAN and offline-mode servers. Online-mode servers need a
  Microsoft session, which Scoria does not have.
- **Downloads from Mojang.** Scoria fetches the pinned 26.3 files from Mojang's piston servers
  into its own folder on first prepare. It does not scan or modify other launchers. Scoria ships
  no game data and never redistributes it.
- **Modrinth, labelled.** Mods and resource packs show **Works**, **Partly** or **Doesn't run**
  before you install. Java mods are ported to Scoria's runtime; Java itself is not run.
- **A real desktop app.** 1280×720, dark and light Ember themes, keyboard navigation, toasts and
  confirmations. Linux is Wayland-native; macOS and Windows use the native frame.

## Assets and ownership

This is the legal boundary, in plain words:

- Scoria contains **no Minecraft code, textures, sounds, language files, data or other assets**.
- Release archives contain only Scoria/Horizon binaries, the UI, the bundled fonts (SIL OFL) and
  the Scoria mark.
- Game files are **downloaded from Mojang's servers on request** or taken from an installation you
  already have. They are stored locally and never uploaded.
- You need your own copy of **Minecraft: Java Edition 26.3**.
- Screenshots in this README show the **launcher UI only**, not game content.
- Horizon's MIT notice and every third-party licence are in
  [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

If you redistribute Scoria, keep the MIT notice and do not add game assets to the archive.

## What works

| | |
|---|---|
| ✅ Single player, vanilla world generation, mobs, villages, redstone, commands, HUD | ❌ Online-mode servers (no Microsoft session, by design) |
| ✅ LAN and offline-mode servers | ❌ CurseForge |
| ✅ Modrinth mods and resource packs, with support labels | ❌ Modpack browser in the launcher (the engine can import and port) |
| ✅ World backups, screenshots, options, resource packs | ❌ OptiFine/Java shader packs; Scoria's `.hshader` source is not published yet |
| ✅ Dark / light Ember themes, Russian and English | ❌ Windows on ARM, Intel Macs |
| ✅ Linux (Wayland), macOS (Apple silicon), Windows (x64) | ❌ Code signing / notarization (beta) |

## Screenshots

UI captures will be added with the first beta. This page will not show game content: Scoria ships
no Minecraft assets.

## Install

Linux: unpack `scoria-launcher-linux-x86_64.tar.zst` and run `./scoria`. Windows: unzip
`scoria-launcher-windows-x64.zip` and run `scoria.exe` (SmartScreen warns until signed). macOS is
deferred: there is no archive yet. The first launch asks for nothing; press **Download 26.3** once
and Scoria installs the game bundle and prepares 26.3 locally from your own files or Mojang's
servers.

## Release channels

There is no `nightly` branch. A branch is not a distribution channel: releases are **tags** on
`dev`, built by CI and attached to GitHub Releases. Tags are plain pointers to commits; the notes
live in [CHANGELOG.md](CHANGELOG.md) and are copied into the release body.

| Tag | Meaning |
|---|---|
| `v0.1.N-beta` | public beta; `N` grows by one for every published build (`0.1.0-beta`, `0.1.1-beta`, ...) |
| `v0.2.0` | first stable release, once the betas are clean |
| `v0.2.1`, ... | fixes after that |

Work that is not published is not tagged: it lives in the history of `dev`. A CI run without a tag
is a build, not a release.

## Build from source

Rust 1.90+ and a Vulkan-capable GPU.

```sh
cd launcher && cargo build --release -p scoria-ui
sh tools/package_ui.sh        # Linux bundle
sh tools/package_macos.sh     # macOS .app
sh tools/package_windows.sh   # Windows zip
```

Run the window in development with `cargo xtask ui` (Linux: native Wayland, no XWayland).

## How it is put together

```
crates/        Rust engine: client, world, protocol, assets, preparation, servers
launcher/      Rust launcher engine + JSON-lines API (horizon-launcher api)
launcher/ui/    the gpui-ce window (screens, layout, mods, feedback)
launcher/ui-kit/ the design tokens, theme and components
docs/          engine and user documentation
```

The window and the launcher are one process: `scoria` calls the engine directly, no JSON bridge.
The engine prepares Mojang files, ports Java mods to the Rust runtime, and starts the client.

## Status

Beta. The engine is Horizon's; Scoria adds the offline launcher, piston-meta downloads, Modrinth
support, the new UI and its branding. Roadmap and known gaps are in
[Issues](https://github.com/wshrx/scoria/issues).

## Credits

- Engine: [Horizon](https://github.com/horizon-rust/horizon), MIT.
- Typefaces: **Medeo Sans** and **Medeo Mono** by the Medeo Project Authors (DNFDEVS), SIL OFL.
  The Scoria wordmark is outlined from Medeo Sans Bold.
- UI: gpui-ce and Scoria's own `scoria-ui-kit`.

## License

MIT — see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Scoria is not an official Minecraft product, is not approved by or associated with Mojang or
Microsoft, and ships no Minecraft code, assets or data. You need your own copy of Minecraft: Java
Edition 26.3.
