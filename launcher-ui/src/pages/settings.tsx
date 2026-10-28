import { useEffect, useState } from "react"
import { Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import type { Snapshot } from "../api"
import { installLatest, removeVersion, selectVersion, setOfflineName, setWhilePlaying, versions, type VersionInfo } from "../api"

const MODES: { value: "stay" | "minimize" | "hide"; label: string }[] = [
  { value: "stay", label: "Stay" },
  { value: "minimize", label: "Minimize" },
  { value: "hide", label: "Hide" },
]

export function SettingsPage({
  data,
  dark,
  onToggleTheme,
  onRefresh,
}: {
  data: Snapshot
  dark: boolean
  onToggleTheme: () => void
  onRefresh: () => void
}) {
  const theme = useTheme()
  const [name, setName] = useState(data.offlineName)
  const [note, setNote] = useState<string | null>(null)
  const [installed, setInstalled] = useState<VersionInfo[]>([])
  const [confirming, setConfirming] = useState<string | null>(null)

  const refreshVersions = () => {
    void versions()
      .then((state) => setInstalled(state.installed))
      .catch((error: Error) => setNote(error.message))
  }

  useEffect(refreshVersions, [])

  const panel = {
    display: "flex",
    flexDirection: "column" as const,
    backgroundColor: theme.colors.card,
    borderWidth: 1,
    borderStyle: "solid" as const,
    borderColor: theme.colors.border,
    borderRadius: theme.radius.sm,
    padding: 18,
    gap: 12,
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 18, maxWidth: 720 }}>
      <div style={panel}>
        <Text mono size="xs" tone="muted">OFFLINE PROFILE</Text>
        <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 10 }}>
          <div style={{ display: "flex", flexDirection: "column", gap: 6, width: 260 }}>
            <Label>Player name</Label>
            <Input
              value={name}
              placeholder="Builder_1"
              onChange={(event) => setName(event.value ?? "")}
            />
          </div>
          <Button
            variant="secondary"
            onClick={() => {
              void setOfflineName(name.trim())
                .then(() => {
                  setNote("Saved. The next Play uses this name.")
                  onRefresh()
                })
                .catch((error: Error) => setNote(error.message))
            }}
          >
            Save
          </Button>
          <Badge variant="success" dot>offline</Badge>
        </div>
        <Text size="xs" tone="muted">
          3 to 16 letters, digits or _. Kept in accounts.json; no token leaves the machine.
        </Text>
        {note ? <Text size="xs" tone="muted">{note}</Text> : null}
      </div>

      <div style={panel}>
        <Text mono size="xs" tone="muted">WHILE PLAYING</Text>
        <Text size="xs" tone="muted">What this window does when the game starts.</Text>
        <div style={{ display: "flex", flexDirection: "row", gap: 6 }}>
          {MODES.map((mode) => (
            <Button
              key={mode.value}
              size="sm"
              variant={data.whilePlaying === mode.value ? "secondary" : "ghost"}
              onClick={() => {
                void setWhilePlaying(mode.value)
                  .then(() => onRefresh())
                  .catch((error: Error) => setNote(error.message))
              }}
            >
              {mode.label}
            </Button>
          ))}
        </div>

        <div style={{ height: 1, backgroundColor: theme.colors.border, marginTop: 4 }} />

        <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 14 }}>
          <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
            <Text>Dark appearance</Text>
            <Text size="xs" tone="muted">Basalt and ember; the light theme is warm paper.</Text>
          </div>
          <Switch checked={dark} onCheckedChange={() => onToggleTheme()} />
        </div>
      </div>

      <div style={panel}>
        <Text mono size="xs" tone="muted">LAUNCHER</Text>
        <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
          <Text size="xs" tone="muted">version</Text>
          <Text mono size="xs">{data.version}</Text>
        </div>
        <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
          <Text size="xs" tone="muted">branch</Text>
          <Text mono size="xs">{data.branch}</Text>
        </div>
        <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
          <Text size="xs" tone="muted">instance</Text>
          <Text mono size="xs">{data.selectedInstance}</Text>
        </div>
        <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
          <Text size="xs" tone="muted">mode</Text>
          <Text mono size="xs">{data.live ? "engine" : "preview"}</Text>
        </div>
      </div>

      <div style={panel}>
        <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
          <Text mono size="xs" tone="muted">VERSIONS</Text>
          <div style={{ flexGrow: 1 }} />
          <Button
            size="sm"
            variant="ghost"
            onClick={() => {
              void installLatest()
                .then(() => {
                  setNote("Installed the latest build.")
                  refreshVersions()
                  onRefresh()
                })
                .catch((error: Error) => setNote(error.message))
            }}
          >
            Install latest
          </Button>
        </div>
        {installed.length === 0 ? (
          <Text size="xs" tone="muted">No versions installed.</Text>
        ) : (
          installed.map((version) => (
            <div
              key={version.tag}
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                gap: 10,
                paddingTop: 6,
                paddingBottom: 6,
                borderTopWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
              }}
            >
              <Text mono size="xs" style={{ flexGrow: 1 }}>{version.tag}</Text>
              <Badge variant={version.prepared ? "success" : "secondary"} dot>
                {version.prepared ? "prepared" : "prepare"}
              </Badge>
              {version.selected ? (
                <Badge variant="outline">selected</Badge>
              ) : (
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => {
                    void selectVersion(version.tag)
                      .then(() => {
                        refreshVersions()
                        onRefresh()
                      })
                      .catch((error: Error) => setNote(error.message))
                  }}
                >
                  Select
                </Button>
              )}
              <Button
                size="sm"
                variant={confirming === version.tag ? "destructive" : "ghost"}
                onClick={() => {
                  if (confirming !== version.tag) {
                    setConfirming(version.tag)
                    return
                  }
                  setConfirming(null)
                  void removeVersion(version.tag)
                    .then(() => {
                      refreshVersions()
                      onRefresh()
                    })
                    .catch((error: Error) => setNote(error.message))
                }}
              >
                {confirming === version.tag ? "Confirm" : "Remove"}
              </Button>
            </div>
          ))
        )}
      </div>

      <div style={panel}>
        <Text mono size="xs" tone="muted">DATA</Text>
        <Text size="xs" tone="muted">
          No telemetry, no consent screen, no analytics. The launcher talks to GitHub only when you
          ask for a build.
        </Text>
      </div>
    </div>
  )
}
