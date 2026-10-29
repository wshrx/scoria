import type { ComponentType } from "react"
import type { IconName } from "@/lib/flint"
import type { Snapshot } from "../api"
import { HomePage } from "./home"
import { InstancesPage } from "./instances"
import { ModsPage } from "./mods"
import { ServersPage } from "./servers"
import { SettingsPage } from "./settings"

/** What every screen is handed. A screen may use only the parts it needs. */
export interface ScreenProps {
  data: Snapshot
  onRefresh: () => void
  onPage: (page: ScreenId) => void
  dark: boolean
  onToggleTheme: () => void
  onLanguage: (value: "en" | "ru" | "system") => void
}

export type ScreenId = "home" | "instances" | "mods" | "servers" | "settings"

export interface ScreenDefinition {
  id: ScreenId
  label: string
  icon: IconName
  /** The rail's main column, or its quiet footer. */
  rail: "main" | "footer"
  component: ComponentType<ScreenProps>
}

/**
 * One entry per screen. Adding a page means adding a component and one line
 * here; the rail and the app's renderer both read this list.
 */
const INSTANCES_ENABLED = process.env.SCORIA_UI_INSTANCES === "1"

export const SCREENS: ScreenDefinition[] = [
  { id: "home", label: "Play", icon: "play", rail: "main", component: HomePage },
  ...(INSTANCES_ENABLED
    ? [{ id: "instances" as const, label: "Instances", icon: "layers" as const, rail: "main" as const, component: InstancesPage }]
    : []),
  { id: "mods", label: "Mods", icon: "box", rail: "main", component: ModsPage },
  { id: "servers", label: "Servers", icon: "server", rail: "main", component: ServersPage },
  { id: "settings", label: "Settings", icon: "sliders", rail: "footer", component: SettingsPage },
]

export function screenFor(id: ScreenId): ScreenDefinition {
  return SCREENS.find((screen) => screen.id === id) ?? SCREENS[0]
}
