# Building Scoria from source

Scoria is two pieces: the Rust engine/launcher and the desktop UI. Building the UI is quick;
building the game client is a full Rust build (the same engine as Horizon).

## Requirements

- Rust 1.90+ (`rustup toolchain install stable`)
- Bun 1.4+
- Linux: a Wayland session and a Vulkan driver; `libxkbcommon`, `libwayland`, `libfontconfig`,
  `libasound`, `libudev` development packages
- macOS 14+ on Apple silicon, or Windows 10/11 x64 with Vulkan

## Launcher

```sh
cd launcher
cargo build --release

cd ../launcher-ui
bun install
bun run build

cd ..
sh tools/package_ui.sh        # Linux bundle
sh tools/package_macos.sh     # macOS .app
sh tools/package_windows.sh   # Windows zip
```

The archives contain the launcher only. They do **not** contain Minecraft files.

## Game client

```sh
cargo build --release -p horizon-client
```

That binary is the game. The launcher expects a game version folder and a prepared resource set;
public client bundles are the remaining release-engineering task (see `docs/public/RELEASE.md` in
the private tree). A launcher build without a version bundle cannot play.

For engine development, the upstream Horizon docs (`docs/native-client.md`, `docs/parity-workflow.md`)
describe running the client directly with resources prepared from your own Minecraft: Java Edition
26.3 installation. Scoria does not ship those files.

## Tests

```sh
cd launcher && cargo test
cargo test -p horizon-client --bin horizon-client cyrillic
cd launcher-ui && bun run typecheck && bun run check:ui && bun run check:i18n
```

## No assets

Scoria ships no Minecraft code, textures, sounds, language files or other game data. You need your
own copy of Minecraft: Java Edition 26.3. See `THIRD_PARTY_NOTICES.md` and the README.
