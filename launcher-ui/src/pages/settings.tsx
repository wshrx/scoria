import { useState, type ReactNode } from "react"
import { Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import type { Snapshot } from "../api"
import { setOfflineName } from "../api"

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
  const [keepOpen, setKeepOpen] = useState(true)
  const [followNightly, setFollowNightly] = useState(data.branch === "nightly")

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

  const row = (title: string, detail: string, control: ReactNode) => (
    <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 14 }}>
      <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
        <Text>{title}</Text>
        <Text size="xs" tone="muted">{detail}</Text>
      </div>
      {control}
    </div>
  )

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
              void setOfflineName(name)
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
        <Text mono size="xs" tone="muted">LAUNCHER</Text>
        {row(
          "Keep the launcher open while playing",
          "Close it automatically when the game starts instead.",
          <Switch checked={keepOpen} onCheckedChange={setKeepOpen} />,
        )}
        <div style={{ height: 1, backgroundColor: theme.colors.border }} />
        {row(
          "Follow the nightly branch",
          "Tested dev builds of the launcher; main stays stable.",
          <Switch checked={followNightly} onCheckedChange={setFollowNightly} />,
        )}
        <div style={{ height: 1, backgroundColor: theme.colors.border }} />
        {row(
          "Dark appearance",
          "Basalt and ember; the light theme is warm paper.",
          <Switch checked={dark} onCheckedChange={() => onToggleTheme()} />,
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
