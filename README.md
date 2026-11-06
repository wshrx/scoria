<p align="center">
  <img src="logo.svg" width="84" alt="Scoria">
</p>

<h1 align="center">Scoria <span style="color:#FF6A3D">native-mods</span></h1>

<p align="center">
  <b>A repeatable install, run and remove checklist for Scoria's native Rust mods.</b>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2026-111214?style=flat-square&logo=rust">
  <img alt="GPUI" src="https://img.shields.io/badge/GPUI-CE-FF6A3D?style=flat-square">
  <img alt="Branch" src="https://img.shields.io/badge/branch-experiment-9C948C?style=flat-square">
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-FF6A3D?style=flat-square">
</p>



## What this branch is

A repeatable install, run and remove checklist for Scoria's native Rust mods.

## What is here

- Build a native mod with the matching API level.
- Install it into the selected instance and start the game.
- Check the mod's log line, behaviour and unload path.
- Remove it and verify the game still starts cleanly.
- The checklist is in docs/qa/native-mods.md; no runtime test harness is on this branch.

## Status

This is a working branch, not a release. It exists to answer one question, keep the answer and the
evidence together, and make the change reviewable before anything reaches `dev`/`main`.

## Verify

```sh
cd launcher
cargo xtask gate
```

## Back to the launcher

- [dev](https://github.com/wshrx/scoria/tree/dev)
- [main](https://github.com/wshrx/scoria/tree/main)
- [latest release](https://github.com/wshrx/scoria/releases)

## License

MIT — see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Scoria is not an official Minecraft product, is not approved by or associated with Mojang or
Microsoft, and ships no Minecraft code, assets or data. You need your own copy of Minecraft: Java
Edition 26.3.
