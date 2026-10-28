# Playing offline

Basalt starts the game without a Microsoft account. This page is the player-facing side of
[ADR-0001](adr/0001-offline-first.md).

## First run

1. Install Basalt and open the launcher. Pick a name (3–16 letters, digits or `_`); that is your
   offline profile. It is kept in `accounts.json` next to the launcher's data, with the skins you
   choose for it.
2. Press **Play**. No sign-in, no keychain, no network.
3. If the game says it is not prepared, point the launcher at an installation of Minecraft: Java
   Edition 26.3 you already have (see below) and press **Prepare** once.

## Where the game files come from

Basalt never downloads the game itself (client jar, libraries, assets). Preparation reads a copy
you already installed, and
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

The 26.3 **server jar** is the one Mojang file preparation fetches (into
`<launcher data>/server-jars/26.3.jar`) when it is not there yet; put it there yourself to avoid
the download. There is no `--download` flag and no "download the game" button: the client jar,
libraries and assets always come from your install.

## What needs a Microsoft account

Nothing, to play. Signing in (Settings → Account) adds:

- joining **online-mode servers**, which check your session with Mojang;
- your cloud skin and cape, and your real UUID on servers that use it.

Basalt does not fake sessions and does not patch the online-mode handshake. An offline name cannot
join an online-mode server, exactly as in vanilla and Prism.

LAN worlds, `Open to LAN`, offline-mode servers and **Play with friends** (invite codes and the
relay) work without an account; `horizon-friends` never used one.

## Where things are kept

| What | Where |
|---|---|
| Offline names, skins, selected profile | launcher data folder, `accounts.json` (public data only) |
| Microsoft tokens, if you sign in | the OS keychain, service `dev.basalt.launcher` |
| Game session handed to the client | a short-lived `session-*.json` in the game home, deleted by the game on start |

Signing out removes the tokens; offline names stay.

## Troubleshooting

- **"Not prepared"** — the launcher did not find a 26.3 installation. Choose the folder, then
  Prepare.
- **A server says "online mode"** — that server requires a real account; sign in or pick an
  offline-mode server.
- **Skins are default on servers** — offline profiles use the skin file you picked locally; servers
  only see it when signed in.
