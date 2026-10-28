/**
 * The launcher's data, as the UI sees it.
 *
 * These shapes are the contract with the Rust engine; today they are filled with
 * a preview snapshot so the screens can be built and reviewed before the IPC
 * bridge lands. `play` and `prepare` will call `horizon-launcher` over JSON-RPC
 * and stream its progress lines.
 */

export interface World {
  name: string
  mode: string
  lastPlayed: string
  size: string
}

export interface Instance {
  id: string
  name: string
  version: string
  kind: "release" | "modpack" | "port"
  prepared: boolean
}

export interface Snapshot {
  version: string
  branch: "nightly" | "main"
  prepared: boolean
  offlineName: string
  worlds: World[]
  instances: Instance[]
}

const preview: Snapshot = {
  version: "v0.2.94",
  branch: "nightly",
  prepared: true,
  offlineName: "Builder_1",
  worlds: [
    { name: "Survival", mode: "Survival", lastPlayed: "2 hours ago", size: "184 MB" },
    { name: "Creative flat", mode: "Creative", lastPlayed: "yesterday", size: "12 MB" },
  ],
  instances: [
    { id: "release", name: "Scoria", version: "26.3", kind: "release", prepared: true },
    { id: "atm10", name: "All the Mods 10", version: "26.3", kind: "modpack", prepared: false },
    { id: "ports", name: "YUNG's dungeons", version: "26.3", kind: "port", prepared: true },
  ],
}

export async function snapshot(): Promise<Snapshot> {
  return preview
}

/** Starts the game. Returns the log line the engine reports, once IPC exists. */
export async function play(): Promise<string> {
  return "The engine bridge is not wired yet: this is the flint-ui preview."
}

/** Prepares the selected version from the player's own Minecraft installation. */
export async function prepare(): Promise<string> {
  return "Preparation will run through the engine bridge; nothing is downloaded."
}
