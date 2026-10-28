/**
 * The bridge to the Rust engine: `horizon-launcher api` speaks JSON lines on stdin/stdout.
 *
 * The UI never touches the file system or starts the game itself. If the engine is not found
 * (a checkout without a built launcher, or the compiled UI before packaging), the screens fall
 * back to the preview snapshot and say so.
 */
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process"
import { existsSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { fileURLToPath } from "node:url"

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
  minecraft: string
  kind: "release" | "modpack" | "port"
  prepared: boolean
  selected: boolean
}

export interface FileCount {
  done: number
  total: number
}

/** Minecraft's files in Scoria's own folder, and where they come from. */
export interface GameFiles {
  version: string
  path: string
  source: string
  manifestSha1: string | null
  installed: boolean
  manifest: boolean
  client: boolean
  libraries: FileCount
  assets: FileCount
  missingBytes: number
}

export interface Snapshot {
  live: boolean
  version: string
  branch: string
  prepared: boolean
  offlineName: string
  selectedInstance: string
  appearance: "dark" | "light" | "system"
  language: "en" | "fr" | "ru" | "system"
  whilePlaying: "stay" | "minimize" | "hide"
  worlds: World[]
  instances: Instance[]
  minecraft: GameFiles
}

const preview: Snapshot = {
  live: false,
  version: "v0.2.94",
  branch: "dev",
  prepared: true,
  offlineName: "Builder_1",
  selectedInstance: "release",
  appearance: "dark",
  language: "en",
  whilePlaying: "stay",
  worlds: [
    { name: "Survival", mode: "Survival", lastPlayed: "2 hours ago", size: "184 MB" },
    { name: "Creative flat", mode: "Creative", lastPlayed: "yesterday", size: "12 MB" },
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

const here = dirname(fileURLToPath(import.meta.url))

function engineBinary(): string {
  if (process.env.SCORIA_LAUNCHER_BIN) return process.env.SCORIA_LAUNCHER_BIN
  // A packaged build: the Rust engine sits beside the compiled UI.
  const beside = resolve(dirname(process.execPath), "horizon-launcher")
  if (existsSync(beside)) return beside
  const dev = resolve(here, "../../launcher/target/debug/horizon-launcher")
  return existsSync(dev) ? dev : beside
}

type Pending = {
  resolve: (value: unknown) => void
  reject: (error: Error) => void
  timer: ReturnType<typeof setTimeout>
  onEvent?: (event: Record<string, unknown>) => void
}

let engine: ChildProcessWithoutNullStreams | null = null
let buffer = ""
let nextId = 1
let unavailable: string | null = null
const pending = new Map<number, Pending>()

function failAll(error: Error) {
  for (const [, entry] of pending) {
    clearTimeout(entry.timer)
    entry.reject(error)
  }
  pending.clear()
}

function onLine(line: string) {
  let message: { id?: number; ok?: boolean; result?: unknown; error?: string; event?: string }
  try {
    message = JSON.parse(line)
  } catch {
    return
  }
  if (typeof message.id !== "number") return
  const entry = pending.get(message.id)
  if (!entry) return
  if (message.event) {
    entry.onEvent?.(message as Record<string, unknown>)
    return
  }
  pending.delete(message.id)
  clearTimeout(entry.timer)
  if (message.ok) entry.resolve(message.result)
  else entry.reject(new Error(message.error ?? "the engine refused"))
}

function start(): ChildProcessWithoutNullStreams | null {
  if (engine) return engine
  if (unavailable) return null
  const bin = engineBinary()
  if (!existsSync(bin)) {
    unavailable = `engine not found at ${bin}`
    return null
  }
  try {
    const child = spawn(bin, ["api"], { stdio: ["pipe", "pipe", "pipe"] })
    child.stdout.on("data", (chunk: Buffer) => {
      buffer += chunk.toString("utf8")
      let newline = buffer.indexOf("\n")
      while (newline >= 0) {
        const line = buffer.slice(0, newline)
        buffer = buffer.slice(newline + 1)
        onLine(line)
        newline = buffer.indexOf("\n")
      }
    })
    child.stderr.on("data", (chunk: Buffer) => {
      const text = chunk.toString("utf8").trim()
      if (text) console.error(`[scoria] ${text}`)
    })
    child.on("error", (error) => {
      unavailable = error.message
      failAll(error)
      engine = null
    })
    child.on("exit", (code) => {
      if (pending.size > 0) failAll(new Error(`the engine exited (${code ?? "signal"})`))
      engine = null
    })
    engine = child
    return child
  } catch (error) {
    unavailable = String(error)
    return null
  }
}

function call<T>(
  method: string,
  params: unknown = {},
  timeoutMs = 10_000,
  onEvent?: (event: Record<string, unknown>) => void,
): Promise<T> {
  const child = start()
  if (!child) return Promise.reject(new Error(unavailable ?? "the engine is not available"))
  const id = nextId++
  return new Promise<T>((resolvePromise, rejectPromise) => {
    const timer = setTimeout(() => {
      pending.delete(id)
      rejectPromise(new Error(`the engine did not answer ${method} in time`))
    }, timeoutMs)
    pending.set(id, {
      resolve: resolvePromise as (value: unknown) => void,
      reject: rejectPromise,
      timer,
      onEvent,
    })
    child.stdin.write(`${JSON.stringify({ id, method, params })}\n`)
  })
}

process.on("exit", () => {
  engine?.kill()
})

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

export interface DownloadProgress {
  done: number
  total: number
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

export interface VersionInfo {
  tag: string
  prepared: boolean
  selected: boolean
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


export interface SavedServer {
  index: number
  name: string
  ip: string
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
