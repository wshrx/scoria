# Release engineering

The tag workflow (`.github/workflows/release.yml`) builds the **launcher** for three targets and
attaches the archives with a SHA-256 file:

| Target | Runner | Archive |
|---|---|---|
| Linux x86_64 (Wayland) | `ubuntu-latest` | `scoria-launcher-linux-x86_64.tar.gz` |
| macOS 14+, Apple silicon | `macos-14` | `scoria-launcher-macos-arm64.zip` |
| Windows 10/11 x64 | `windows-latest` | `scoria-launcher-windows-x64.zip` |

Tag format: `v0.1.0-beta.1` (a `beta`/`alpha` tag becomes a GitHub pre-release).

## What a launcher archive contains

`scoria-ui` (the Bun-compiled UI), `horizon-launcher` (the Rust engine), `gpuix-native.node`, and
`fonts/`. Linux adds a `scoria-launcher-ui` wrapper; macOS ships `Scoria.app` with a shell shim that
sets `NAPI_RS_NATIVE_LIBRARY_PATH`; Windows ships `Scoria.cmd` for the same reason.

## The game client is not in these archives

A fresh launcher installs a **game version** from a release asset (`horizon-linux-x86_64.tar.gz`,
`horizon-macos-arm64.zip`, `horizon-windows-x64.zip`) and prepares Mojang resources locally. Scoria
must therefore publish its own client bundles before the downloads above can play on a clean
machine:

1. Build `horizon-client` and its helper binaries for each target in CI (matrix job).
2. Package binaries only — **never prepared Minecraft assets**; first play prepares them from the
   user's files or piston-meta.
3. Sign `SHA256SUMS` with Scoria's release key. The launcher currently trusts Horizon's built-in
   keys (`crates/horizon-release-sign`), so a Scoria key must be added to `trusted_keys` before an
   update can be verified.
4. Tag the client bundle with the game tag the launcher looks for (not the launcher's `v0.1.0`
   tag), and attach it to the release that has the matching asset name.

Until step 3 is done, mac/win downloads are **launcher previews**: they start, but installing 26.3
uses the existing asset path and may fall back to an already installed version.

## Signing

- macOS: ad-hoc `codesign` in `package_macos.sh`; Gatekeeper still asks once. Notarization needs an
  Apple Developer ID and `notarytool`, which is not configured.
- Windows: unsigned; SmartScreen warns. Authenticode needs a certificate.
- Linux: no signature; SHA-256 only.

## Checklist for a release

- [ ] `bun run typecheck && bun run check:ui && bun run check:i18n`
- [ ] `cargo test` in `launcher/`, and `cargo test -p horizon-client --bin horizon-client`
- [ ] Fresh-machine install test on each OS
- [ ] Screenshots in `docs/public/screenshots/` match the build
- [ ] `SHA256SUMS` attached
- [ ] No `.opencode/`, `PROJECT_STATE.md`, `docs/agents/` or `dist/` in the public tree
