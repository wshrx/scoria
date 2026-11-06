# Native mods (`.hnmod`) — manual smoke test

Branch: `test/native-mods`. Verification only; no code change is needed unless a step fails.

## Preconditions

- `dev` with the 2026-11-02 engine sync (`Kind::NativeMod` comes from upstream).
- A fresh launcher home, never the player's real one:

```sh
rm -rf /tmp/scoria-native && mkdir -p /tmp/scoria-native
HORIZON_HOME=/tmp/scoria-native/game \
HORIZON_LAUNCHER_HOME=/tmp/scoria-native/launcher \
XDG_DATA_HOME=/tmp/scoria-native/data \
XDG_CONFIG_HOME=/tmp/scoria-native/config \
./scoria
```

- **Download 26.3** and wait for preparation to finish.

## Install

1. Open **Mods**.
2. Search for a registry mod served as `.hnmod` (Horizon tab; a `native-mod` entry).
3. Press **Install**.
4. Check the file and its origin record:

```sh
find /tmp/scoria-native -name '*.hnmod' -o -name '*.origin.json'
```

Expected: one `.hnmod` under the instance's `horizon-native-mods/`, plus an origin record. No
`.hmod`/`.hnmod` is unpacked into the game folder.

## Run

1. Press **Play**.
2. Open the mod's own UI or check its effect (for a HUD mod: the overlay appears; for a key mod:
   the key works).
3. Close the game and check `ui.log` / the game's `logs/latest.log` for a load error.

Expected: the engine loads `Kind::NativeMod` without "unknown kind" errors.

## Remove

1. Back in Mods, remove the mod.
2. Check the `.hnmod` and origin record are gone.
3. Play again: no missing-mod error.

## If it fails

Collect:

```text
/tmp/scoria-native/launcher/ui.log
/tmp/scoria-native/game/logs/latest.log
the .hnmod path and the registry id/version
```

Known state: the Rust unit test `crates/horizon-mods/src/index/tests.rs::reads_native_mods_and_hnmod_files`
covers parsing; this checklist covers install → run → remove.
