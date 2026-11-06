<p align="center">
  <img src="logo.svg" width="84" alt="Scoria">
</p>

<h1 align="center">Scoria <span style="color:#FF6A3D">gpui-fast</span></h1>

<p align="center">
  <b>gpui-ce against gpui-fast, measured on the same launcher.</b>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2026-111214?style=flat-square&logo=rust">
  <img alt="GPUI" src="https://img.shields.io/badge/GPUI-CE-FF6A3D?style=flat-square">
  <img alt="Branch" src="https://img.shields.io/badge/branch-experiment-9C948C?style=flat-square">
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-FF6A3D?style=flat-square">
</p>

![gpui-fast comparison](screenshots/home.webp)

_The same Home in the comparison branch._


## What this branch is

gpui-ce against gpui-fast, measured on the same launcher.

## What is here

- The gpui-fast component/backend swap.
- Startup and idle RSS measured with the same release profile.
- The text input lost behaviour we need, so the branch stayed an experiment.
- The result: no measurable startup win and about +16 MB idle RSS.

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
