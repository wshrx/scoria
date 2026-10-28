import { call } from "./client"
import type { ModEntry, ModProject } from "./types"

export async function installedMods(): Promise<{ instance: string; entries: ModEntry[] }> {
  return call("mods", {}, 10_000)
}

export async function searchMods(
  query: string,
  kind: "mod" | "resource-pack",
): Promise<ModProject[]> {
  const result = await call<{ results: ModProject[] }>("search_mods", { query, kind }, 30_000)
  return result.results
}

export async function installMod(
  id: string,
  onProgress?: (label: string, value: number) => void,
): Promise<ModEntry[]> {
  const result = await call<{ entries: ModEntry[] }>(
    "install_mod",
    { id },
    30 * 60_000,
    (event) => {
      if (
        event.event === "progress" &&
        typeof event.label === "string" &&
        typeof event.value === "number"
      ) {
        onProgress?.(event.label, event.value)
      }
    },
  )
  return result.entries
}

export async function removeMod(name: string): Promise<ModEntry[]> {
  const result = await call<{ entries: ModEntry[] }>("remove_mod", { name }, 20_000)
  return result.entries
}
