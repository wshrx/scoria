export interface World {
  /** The folder under `saves/`, unique even when two worlds share a name. */
  folder: string
  name: string
  mode: string
  lastPlayed: string
  /** When the game last saved it, Unix milliseconds; 0 when unknown. */
  lastPlayedAt: number
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
  /** A game holds the folder's `.scoria-running` mark right now. */
  playing: boolean
  playingPid: number | null
  offlineName: string
  selectedInstance: string
  appearance: "dark" | "light" | "system"
  language: "en" | "fr" | "ru" | "system"
  whilePlaying: "stay" | "minimize" | "hide"
  worlds: World[]
  instances: Instance[]
  minecraft: GameFiles
}

export interface DownloadProgress {
  done: number
  total: number
}

export interface VersionInfo {
  tag: string
  prepared: boolean
  selected: boolean
}

export interface SavedServer {
  index: number
  name: string
  ip: string
}

export interface ModProject {
  id: string
  slug: string
  title: string
  summary: string
  author: string
  page: string
  kind: string
  support: string
  reason: string | null
  installable: boolean
}

export interface ModEntry {
  source: string
  project: string
  slug: string
  title: string
  kind: string
  version: string
  number: string
  file: string
  checksum: string
  page: string
  dependency: boolean
  verdict?: string
  skipped?: string[]
}
