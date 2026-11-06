<p align="center">
  <img src="logo.svg" width="88" alt="Scoria">
</p>

<h1 align="center">Scoria <span style="color:#D9A441">Dungeons</span></h1>

<p align="center">
  <b>Mossy stone, parchment and amber in a rounded, game-like shell.</b><br>
  Offline by default. No Microsoft account. No telemetry. No game assets.
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2026-111214?style=flat-square&logo=rust">
  <img alt="GPUI" src="https://img.shields.io/badge/GPUI-CE-D9A441?style=flat-square">
  <img alt="Experiment" src="https://img.shields.io/badge/status-experiment-9C948C?style=flat-square">
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-D9A441?style=flat-square">
</p>

![Scoria Dungeons](docs/public/screenshots/home.webp)

## The direction

**Dungeons** is one of the Scoria launcher explorations. It keeps the product rules — one game
action, offline profile, world list, Modrinth, servers, settings — and changes the visual language:

- One rounded window with a visible edge.
- Moss, parchment and amber surfaces.
- Rounded controls that still keep the 4 px grid.
- A game-like but readable launcher.

## Screens

| Home | Settings |
| --- | --- |
| ![Home](docs/public/screenshots/home.webp) | ![Settings](docs/public/screenshots/settings.webp) |

## What stays the same

- **Offline first**: offline names, no Microsoft session, no telemetry.
- **Vanilla worlds**: saves stay in vanilla's format.
- **One action**: install, prepare, play and stop are one state-aware control.
- **Modrinth and Horizon**: search, install and label every project before it reaches the game.
- **The same Rust**: gpui-ce window, `scoria-ui-kit` tokens, `launcher/lang` translations.

## Status

This branch is an **experiment**, not a release. The production launcher lives on `dev`, `main`
and `staging`; nothing here is tagged or shipped.

## Build

```sh
cd launcher
cargo xtask gate
cargo xtask ui
```

The full guide, release notes and the other directions are in the
[main repository](https://github.com/wshrx/scoria).

## License

MIT — see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Scoria is not an official Minecraft product, is not approved by or associated with Mojang or
Microsoft, and ships no Minecraft code, assets or data. You need your own copy of Minecraft: Java
Edition 26.3.
