import { call } from "./client"
import type {
  DownloadProgress,
  GameFiles,
  SavedServer,
  Snapshot,
  VersionInfo,
} from "./types"

const preview: Snapshot = {
  live: false,
  version: "v0.2.94",
  branch: "dev",
  prepared: true,
  playing: false,
  playingPid: null,
  offlineName: "Builder_1",
  selectedInstance: "release",
  appearance: "dark",
  language: "en",
  whilePlaying: "stay",
  worlds: [
    { folder: "Survival", name: "Survival", mode: "Survival", lastPlayed: "2 hours ago", lastPlayedAt: Date.now() - 7_200_000, size: "184 MB" },
    { folder: "Creative flat", name: "Creative flat", mode: "Creative", lastPlayed: "yesterday", lastPlayedAt: Date.now() - 86_400_000, size: "12 MB" },
  ],
  instances: [
    { id: "release", name: "Scoria", version: "v0.2.94", minecraft: "26.3", kind: "release", prepared: true, selected: true },
    { id: "atm10", name: "All the Mods 10", version: "v0.2.94", minecraft: "26.3", kind: "modpack", prepared: false, selected: false },
    { id: "ports", name: "YUNG's dungeons", version: "v0.2.94", minecraft: "26.3", kind: "port", prepared: true, selected: false },
  ],
  minecraft: {
    version: "26.3",
    path: "~/.local/share/horizon-launcher/minecraft",
    source: "https://piston-meta.mojang.com",
    manifestSha1: "11f1aefe473eefba0af62e3286c33b692807b2eb",
    installed: true,
    manifest: true,
    client: true,
    libraries: { done: 114, total: 114 },
    assets: { done: 2409, total: 2409 },
    missingBytes: 0,
  },
}

export async function snapshot(): Promise<Snapshot> {
  try {
    const state = await call<Omit<Snapshot, "live">>("state", {}, 5000)
    return { ...state, live: true }
  } catch (error) {
    console.warn(`[scoria] preview: ${(error as Error).message}`)
    return preview
  }
}

/** Starts the game. Returns the line the UI shows under the Play button. */
export async function play(options: { world?: string; server?: string } = {}): Promise<string> {
  try {
    const started = await call<{ pid: number; version: string; instance: string }>(
      "play",
      options,
      30_000,
    )
    return `Started ${started.version} as ${started.instance} (pid ${started.pid}).`
  } catch (error) {
    return `The game did not start: ${(error as Error).message}`
  }
}

/** Prepares the selected version from the player's own Minecraft installation, downloading the
 * pinned game files from Mojang first when they are not on this machine. */
export async function prepare(
  instance?: string,
  onLog?: (line: string) => void,
  download = true,
): Promise<string> {
  try {
    const result = await call<{ prepared: boolean; log: string[] }>(
      "prepare",
      { ...(instance ? { instance } : {}), download },
      30 * 60_000,
      (event) => {
        if (event.event === "log" && typeof event.line === "string") onLog?.(event.line)
      },
    )
    const last = result.log.at(-1)
    return result.prepared
      ? `Prepared.${last ? ` ${last}` : ""}`
      : "The preparation did not finish."
  } catch (error) {
    return `Preparation failed: ${(error as Error).message}`
  }
}

/** Downloads whatever Minecraft 26.3 is missing from Mojang's piston-meta. */
export async function downloadMinecraft(
  onLog?: (line: string) => void,
  onProgress?: (progress: DownloadProgress) => void,
): Promise<GameFiles> {
  return call<GameFiles>("download_minecraft", {}, 60 * 60_000, (event) => {
    if (event.event === "log" && typeof event.line === "string") onLog?.(event.line)
    if (
      event.event === "progress" &&
      typeof event.done === "number" &&
      typeof event.total === "number"
    ) {
      onProgress?.({ done: event.done, total: event.total })
    }
  })
}

/** Renames the offline profile through the engine (the same accounts.json the CLI uses). */
export async function setOfflineName(name: string): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("set_offline_name", { name }, 5000)
  return { ...state, live: true }
}

export async function selectInstance(id: string): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("select_instance", { id }, 5000)
  return { ...state, live: true }
}

export async function createInstance(name: string): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("create_instance", { name }, 10_000)
  return { ...state, live: true }
}

export async function setAppearance(value: "dark" | "light" | "system"): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("set_appearance", { value }, 5000)
  return { ...state, live: true }
}

export async function setWhilePlaying(value: "stay" | "minimize" | "hide"): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("set_while_playing", { value }, 5000)
  return { ...state, live: true }
}

export async function versions(): Promise<{ installed: VersionInfo[]; selected: string | null }> {
  return call("versions", {}, 5000)
}

export async function selectVersion(tag: string): Promise<void> {
  await call("select_version", { tag }, 5000)
}

export async function removeVersion(tag: string): Promise<void> {
  await call("remove_version", { tag }, 15_000)
}

export async function installLatest(): Promise<void> {
  await call("install_latest", {}, 30 * 60_000)
}

export async function savedServers(): Promise<SavedServer[]> {
  const result = await call<{ instance: string; servers: SavedServer[] }>("servers", {}, 5000)
  return result.servers
}

export async function addServer(name: string, ip: string): Promise<SavedServer[]> {
  const result = await call<{ servers: SavedServer[] }>("add_server", { name, ip }, 5000)
  return result.servers
}

export async function removeServer(index: number): Promise<SavedServer[]> {
  const result = await call<{ servers: SavedServer[] }>("remove_server", { index }, 5000)
  return result.servers
}

export async function setLanguage(value: "en" | "ru" | "system"): Promise<Snapshot> {
  const state = await call<Omit<Snapshot, "live">>("set_language", { value }, 5000)
  return { ...state, live: true }
}
