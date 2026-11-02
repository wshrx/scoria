#!/bin/sh
# Fails when an upstream merge or a careless checkout removed a Scoria-only engine surface.
# Run after every sync; it is cheap and text-based on purpose.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
bad=0
need() { grep -q -e "$1" "$2" || { echo "missing: $1 in $2" >&2; bad=1; }; }
forbid() { ! grep -rq -e "$1" "$2" || { echo "came back: $1 in $2" >&2; bad=1; }; }

# Non-Latin input fallback.
need "fn typed_character" crates/horizon-client/src/game.rs
need "typed_text" crates/horizon-client/src/game/chat.rs
# No Microsoft session can be handed to the game.
need "Scoria has no Microsoft sign-in" crates/horizon-client/src/net.rs
forbid "--session-file" crates/horizon-client/src/game.rs
forbid "HORIZON_SESSION_FILE" crates/horizon-client/src/game.rs
# The server CLI is offline-only.
forbid "check_accounts" launcher/src
need '"--offline"' launcher/src/servers.rs
# The launcher rewrite is ours: upstream's launcher must never replace it on a sync.
need '"ui-kit"' launcher/Cargo.toml
need '"xtask"' launcher/Cargo.toml
need 'name = "scoria-ui"' launcher/ui/Cargo.toml
need 'name = "scoria-ui-kit"' launcher/ui-kit/Cargo.toml
for f in launcher/ui/src/main.rs launcher/ui-kit/src/tokens.rs launcher/xtask/src/main.rs; do
  [ -s "$f" ] || { echo "missing: $f" >&2; bad=1; }
done
# The in-process UI talks to the engine through these, not JSON.
need "pub fn snapshot" launcher/src/api.rs
need "pub fn start_game" launcher/src/api.rs
need "Horizon's registry comes first" launcher/src/browser.rs
# Scoria's release key and icon stay in the tree.
need "i4LXDB7dVIooNErNdjfRUWWstL9j00RWGKHtQjwDIOs=" crates/horizon-release-sign/release-keys.txt
[ -s packaging/icons/horizon.ico ] || { echo "missing: packaging/icons/horizon.ico" >&2; bad=1; }

if [ "$bad" -ne 0 ]; then
  echo "Fork surfaces are missing. Re-apply docs/fork/engine-patches/*.patch and fix launcher/ by hand." >&2
  exit 1
fi
echo "fork surfaces check passed"
