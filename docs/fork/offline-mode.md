# Playing offline

Scoria starts the game without a Microsoft account. This page is the player-facing side of
[ADR-0001](adr/0001-offline-first.md).

## First run

1. Install Scoria and open the launcher. Pick a name (3–16 letters, digits or `_`); that is your
   offline profile. It is kept in `accounts.json` next to the launcher's data, with the skins you
   choose for it.
2. Press **Play**. No sign-in, no keychain, no account.
3. If the game says it is not prepared, press **Prepare** once. Scoria reads an installation of
   Minecraft: Java Edition 26.3 you already have, or downloads the pinned version from Mojang
   (see below).

## Where the game files come from

Preparation first looks for a copy you already installed, and
`launcher/src/sources.rs` looks in:

- the official launcher (`~/.minecraft`, Flatpak included);
- Prism Launcher and **Freesm Launcher** (a Prism fork, detected automatically) and MultiMC,
  including their `instances/<name>/minecraft` folders;
- Modrinth App, ATLauncher and GDLauncher.

If yours lives somewhere else, choose the folder in Settings (**Minecraft folder**) or pass
`--minecraft-dir`, or set `HORIZON_MINECRAFT_DIR`. Preparation needs Java 25 once and reuses the
launcher's own runtime (`<root>/java/<runtime>`, or the `JavaPath` in an instance's
`instance.cfg`); if there is none it can download Mojang's JDK (a JDK, not game content).
Sounds are optional and come from the same installation.

When none of those folders has 26.3, the launcher downloads it from Mojang's own metadata,
piston-meta and piston-data (`launcher/src/piston.rs`), into `minecraft/` in its data folder in
the official launcher's layout: the version file, the client jar, this system's libraries, the
asset index and every asset object. Each file is checked against the size and sha1 Mojang lists,
and a stopped download keeps what it finished. The same download is on the **Game files** page.

The 26.3 **server jar** is the other Mojang file preparation fetches (into
`<launcher data>/server-jars/26.3.jar`) when it is not there yet.

## No Microsoft account at all

Scoria has no sign-in: no device code, no tokens, no keychain entry for an account, no session
file. `accounts.json` holds offline names and their skins only.

Online-mode servers therefore cannot be joined: Scoria does not fake a session and does not patch
the handshake. LAN worlds, `Open to LAN`, offline-mode servers and **Play with friends** (invite
codes and the relay) work; `horizon-friends` never used an account.

## Where things are kept

| What | Where |
|---|---|
| Offline names, skins, selected profile | launcher data folder, `accounts.json` (public data only) |
| Microsoft tokens, if you sign in | the OS keychain, service `dev.scoria.launcher` |
| Game session handed to the client | a short-lived `session-*.json` in the game home, deleted by the game on start |

Signing out removes the tokens; offline names stay.

## Troubleshooting

- **"Not prepared"** — the launcher did not find a 26.3 installation. Choose the folder, then
  Prepare.
- **A server says "online mode"** — that server requires a real account; sign in or pick an
  offline-mode server.
- **Skins are default on servers** — offline profiles use the skin file you picked locally; servers
  only see it when signed in.
