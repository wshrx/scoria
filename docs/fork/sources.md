# Build sources

Scoria can take its **own builds** (the launcher and the Rust engine) from a server instead of
GitHub. Minecraft files are never part of this: the game is imported from the player's install.

`~/.local/share/horizon-launcher/sources.json` (or the path in `SCORIA_SOURCES`):

```json
{
  "builds": {
    "repo": "wshrx/scoria",
    "releases": "https://our.server/scoria/releases.json",
    "assets": "https://our.server/scoria/{tag}/{name}"
  }
}
```

| Key | Meaning | Default |
|---|---|---|
| `repo` | `owner/repo` for the GitHub API and asset fallback | `wshrx/scoria` |
| `releases` | URL answering with a GitHub-compatible releases array (`tag_name`, `assets[]` with `browser_download_url`, `size`, `name`) | GitHub API for `repo` |
| `assets` | Base URL for a release file; `{tag}` and `{name}` are replaced | `https://github.com/<repo>/releases/download/{tag}/{name}` |

`SHA256SUMS` and `SHA256SUMS.sig` are fetched through the same `assets` template (as names), so a
mirror must publish them beside the archives.

## Signing

The launcher verifies every archive against `SHA256SUMS` signed by a key built into
`crates/horizon-release-sign/release-keys.txt` (Horizon's release key today). A server holding
Scoria builds must either:

- sign with a key already trusted there, or
- add its public key to that file before building the launcher (a deliberate, reviewed change).

Unsigned builds are refused. A future `builds.keys` entry will make this configurable without a
rebuild; until then it is a source change, not a config one.

## What is not configurable

- Minecraft's client jar, libraries, assets and Java are **not** downloaded. Preparation reads the
  player's installation (Freesm, Prism, the official launcher).
- The `26.3` server jar is fetched from Mojang once for data generation unless it is already in
  `<data>/server-jars/26.3.jar`.
