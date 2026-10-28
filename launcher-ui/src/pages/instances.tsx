import { Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Snapshot } from "../api"

const KIND: Record<string, string> = {
  release: "Release",
  modpack: "Modpack",
  port: "Ported mod",
}

/** Every profile the launcher can switch to: the release, packs and ports. */
export function InstancesPage({ data }: { data: Snapshot }) {
  const theme = useTheme()

  return (
    <>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Text size="sm" tone="muted">
          Each instance has its own worlds, options and packs.
        </Text>
        <div style={{ flexGrow: 1 }} />
        <Button variant="secondary" size="sm">New instance</Button>
      </div>

      <div style={{ display: "flex", flexDirection: "row", flexWrap: "wrap", gap: 12 }}>
        {data.instances.map((instance) => (
          <Card key={instance.id} style={{ width: 300 }}>
            <CardHeader>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                <CardTitle>{instance.name}</CardTitle>
                <Badge variant="outline">{KIND[instance.kind] ?? instance.kind}</Badge>
              </div>
              <CardDescription>Minecraft {instance.version}</CardDescription>
            </CardHeader>
            <CardContent>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                <Badge variant={instance.prepared ? "success" : "secondary"} dot>
                  {instance.prepared ? "Ready" : "Not prepared"}
                </Badge>
                <div style={{ flexGrow: 1 }} />
                <Button variant="ghost" size="sm">Open</Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card variant="outline">
        <CardHeader>
          <CardTitle>Imported from Freesm or Prism</CardTitle>
          <CardDescription>
            Scoria finds instances in the official launcher, Prism, Freesm, MultiMC and the Modrinth
            App. Nothing is downloaded: preparation reads the files you already have.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Text size="xs" tone="muted" style={{ color: theme.colors.primary }}>Detection is automatic</Text>
        </CardContent>
      </Card>
    </>
  )
}
