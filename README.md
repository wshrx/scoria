<p align="center">
  <img src="logo.svg" width="84" alt="Scoria">
</p>

<h1 align="center">Scoria <span style="color:#FF6A3D">shaders</span></h1>

<p align="center">
  <b>A native Horizon .hshader pack: sky, material, bloom, sun shafts and a filmic composite.</b>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2026-111214?style=flat-square&logo=rust">
  <img alt="GPUI" src="https://img.shields.io/badge/GPUI-CE-FF6A3D?style=flat-square">
  <img alt="Branch" src="https://img.shields.io/badge/branch-experiment-9C948C?style=flat-square">
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-FF6A3D?style=flat-square">
</p>

![Horizon Visuals](screenshots/home.webp)

_Six stages, compared off/on in the same world._


## What this branch is

A native Horizon .hshader pack: sky, material, bloom, sun shafts and a filmic composite.

## What is here

- sky.wgsl turns the procedural sky into a warmer horizon.
- material.wgsl adds vividness, emission and water reflection.
- Bloom extract and blur make the highlights survive the composite.
- Screen-space sun shafts and the engine shadow map add depth.
- Filmic composite, contrast, saturation and vignette finish the frame.
- No Iris/OptiFine compatibility layer; this is Horizon's WGSL pack format.

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
