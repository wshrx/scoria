# Shaders — smoke test

Branch: `test/shaders`. Verification only.

The pack is also checked by `crates/horizon-shader-api/tests/vibrant.rs`: `Pack::read` plus
`check` compile every stage. Run it with `cargo test -p horizon-shader-api --test vibrant`.

## What Scoria can run

- **`.hshader` / schema 1** (Horizon shader API): `horizon-shaders.toml` + WGSL stages. This is the
  supported path; the sample is `tests/shader-packs/vibrant/`.
- **OptiFine/Java shader packs** are read by the pack library but are not a supported renderer path.
  Do not promise them as working.
- Vanilla **Java 26.3 has no "Vibrant Visuals"**; that is a Bedrock feature. The `Vibrant` test pack
  is an original `.hshader` post grade, not a copy of it.

## Install

1. Start with a fresh profile and prepared 26.3.
2. Copy the pack into the game folder:

```sh
cp -a tests/shader-packs/vibrant "<game folder>/shaderpacks/vibrant"
# or zip the folder as vibrant.hshader and copy that
```

3. Open the game's shader pack screen (Options → Shader Packs) and select **Vibrant (test)**.

## Check

- The picture becomes more saturated/contrasty; the HUD and menus are unchanged.
- Change **Saturation** and **Contrast**; the frame updates, no restart.
- Save `post/vibrant.wgsl` while the pack is selected; hot reload reports an error, and the last good
  frame keeps drawing if the edit is invalid.
- Turn the pack off: the picture returns to vanilla; no shader code runs.
- Remove the pack folder; the game starts without a missing-pack error.

## Logs

```text
<game folder>/logs/latest.log
<launcher home>/ui.log
```

Record the pack id/version and whether the error overlay names the WGSL file and line.

## Out of scope

The launcher has no shader browser yet; installation is a manual copy. OptiFine, Iris and CurseForge
shader packs are not part of this test.
