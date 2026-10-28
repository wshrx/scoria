import { useState } from "react"
import { Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Snapshot } from "../api"

/** Offline identity, appearance and the few launcher behaviours that matter. */
export function SettingsPage({ data, dark, onToggleTheme }: { data: Snapshot; dark: boolean; onToggleTheme: () => void }) {
  const theme = useTheme()
  const [keepOpen, setKeepOpen] = useState(true)
  const [autoUpdate, setAutoUpdate] = useState(false)

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>Offline profile</CardTitle>
          <CardDescription>
            The name the game uses. No Microsoft account, no keychain, no network.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 10 }}>
            <div style={{ display: "flex", flexDirection: "column", gap: 6, width: 260 }}>
              <Label>Player name</Label>
              <Input value={data.offlineName} placeholder="Builder_1" />
            </div>
            <Badge variant="success" dot>Valid</Badge>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Launcher</CardTitle>
          <CardDescription>A few behaviours, kept out of the way.</CardDescription>
        </CardHeader>
        <CardContent>
          <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12 }}>
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
                <Text>Keep the launcher open while playing</Text>
                <Text size="xs" tone="muted">Close it automatically when the game starts instead.</Text>
              </div>
              <Switch checked={keepOpen} onCheckedChange={setKeepOpen} />
            </div>
            <div style={{ height: 1, backgroundColor: theme.colors.border }} />
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12 }}>
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
                <Text>Follow the nightly branch</Text>
                <Text size="xs" tone="muted">Tested dev builds of the launcher; main stays stable.</Text>
              </div>
              <Switch checked={autoUpdate} onCheckedChange={setAutoUpdate} />
            </div>
            <div style={{ height: 1, backgroundColor: theme.colors.border }} />
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12 }}>
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
                <Text>Dark appearance</Text>
                <Text size="xs" tone="muted">flint tokens; the ember accent is Scoria's.</Text>
              </div>
              <Switch checked={dark} onCheckedChange={onToggleTheme} />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card variant="inset">
        <CardHeader>
          <CardTitle>Data</CardTitle>
          <CardDescription>
            No telemetry, no consent screen, no analytics. The launcher talks to GitHub only when you
            ask for a build.
          </CardDescription>
        </CardHeader>
      </Card>
    </>
  )
}
