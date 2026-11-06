# Shaders — smoke test

Branch: `feat/shaders`. Verification and the first original look.

## Packs

- `tests/shader-packs/horizon-visuals` — the current look: sky glow/haze, block vividness,
  emissive boost, water reflection, a sun/moon shadow map, half-res bloom, five-tap blur,
  screen-space sun shafts, a film curve and vignette. `common.wgsl` is generated from
  `shaders/wesl/common.wesl`. It is checked by `horizon-shader-api`:

```sh
cargo test -p horizon-shader-api --test horizon_visuals
```

## What Scoria can run

- **`.hshader` / schema 1** (Horizon shader API): `horizon-shaders.toml` + WGSL stages.
- **OptiFine/Java shader packs** are not a supported renderer path. Vanilla Java 26.3 has no
  “Vibrant Visuals” (Bedrock); `Horizon Visuals` is an original pack.
- WESL is source only: it links to WGSL before the game reads the pack.

## Install and check

1. Start with a fresh profile and prepared 26.3.
2. Copy a pack into the game folder:

```sh
cp -a tests/shader-packs/horizon-visuals "<game folder>/shaderpacks/horizon-visuals"
```

3. Game → **Options → Shader Packs** → select **Horizon Visuals (test)**.
4. Change Exposure, Film curve, Bloom threshold/strength, Saturation, Contrast, Warmth,
   Vignette, Sky glow, Horizon haze, Block vividness, Emissive boost, Water reflection, Shadow
   strength and Sun shafts; the frame updates. The pack is six stages: `sky.wgsl`,
   `material.wgsl`, bloom extract, bloom blur, sun shafts and the filmic composite.
5. Save a WGSL file while selected; hot reload reports an error and keeps the last good frame.
6. Turn the pack off: vanilla; remove it: no missing-pack error.

## Logs

```text
<game folder>/logs/latest.log
<launcher home>/ui.log
```

Record the pack id/version and whether the error overlay names the WGSL file and line.

Spectator: the sun shafts are a screen-space approximation, not volumetric lighting. PBR is limited
to the material stage's vividness/emission/reflection; a full PBR lighting model needs engine
support.
