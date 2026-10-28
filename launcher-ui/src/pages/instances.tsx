import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "../api"

const KIND: Record<string, string> = {
  release: "release",
  modpack: "modpack",
  port: "ported mod",
}

/** Every profile as a row: no cards, no repeated chrome. */
export function InstancesPage({ data }: { data: Snapshot }) {
  const theme = useTheme()
  const panel = {
    display: "flex",
    flexDirection: "column" as const,
    backgroundColor: theme.colors.card,
    borderWidth: 1,
    borderStyle: "solid" as const,
    borderColor: theme.colors.border,
    borderRadius: theme.radius.sm,
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 16, maxWidth: 760 }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Text size="sm" tone="muted">Separate game folders, each with its own worlds, options and packs.</Text>
        <div style={{ flexGrow: 1 }} />
        <Button variant="ghost" size="sm">New instance</Button>
      </div>

      <div style={panel}>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            gap: 12,
            paddingLeft: 16,
            paddingRight: 16,
            paddingTop: 10,
            paddingBottom: 10,
            borderBottomWidth: 1,
            borderStyle: "solid",
            borderColor: theme.colors.border,
          }}
        >
          <Text mono size="xs" tone="muted" style={{ flexGrow: 1 }}>INSTANCE</Text>
          <Text mono size="xs" tone="muted" style={{ width: 80 }}>MINECRAFT</Text>
          <Text mono size="xs" tone="muted" style={{ width: 90 }}>KIND</Text>
          <Text mono size="xs" tone="muted" style={{ width: 90 }}>STATUS</Text>
        </div>
        {data.instances.map((instance, index) => (
          <div
            key={instance.id}
            style={{
              display: "flex",
              flexDirection: "row",
              alignItems: "center",
              gap: 12,
              paddingLeft: 16,
              paddingRight: 12,
              paddingTop: 13,
              paddingBottom: 13,
              borderTopWidth: index === 0 ? 0 : 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
              cursor: "pointer",
              hover: { backgroundColor: theme.colors.muted },
            }}
          >
            <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
              <Text weight="medium">{instance.name}</Text>
              <Text mono size="xs" tone="muted">{instance.id}</Text>
            </div>
            <Text mono size="xs" tone="muted" style={{ width: 80 }}>{instance.version}</Text>
            <Text size="xs" tone="muted" style={{ width: 90 }}>{KIND[instance.kind] ?? instance.kind}</Text>
            <div style={{ width: 90 }}>
              <Badge variant={instance.prepared ? "success" : "secondary"} dot>
                {instance.prepared ? "ready" : "prepare"}
              </Badge>
            </div>
            <Icon name="chevronRight" size={14} color={theme.colors.mutedForeground} />
          </div>
        ))}
      </div>

      <div style={{ ...panel, padding: 16, gap: 6 }}>
        <Text mono size="xs" tone="muted">DETECTED INSTALLS</Text>
        <Text size="xs" tone="muted">
          The official launcher, Prism, Freesm, MultiMC and the Modrinth App are found automatically.
          Nothing is downloaded; preparation reads the files you already have.
        </Text>
      </div>
    </div>
  )
}
