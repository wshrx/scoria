import { useState } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Snapshot } from "../api"
import { createInstance, selectInstance } from "../api"

const KIND: Record<string, string> = {
  release: "release",
  modpack: "modpack",
  port: "ported mod",
}

/** Every profile as a row; clicking one makes it what Play uses. */
export function InstancesPage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState("")
  const [note, setNote] = useState<string | null>(null)

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
    <div style={{ display: "flex", flexDirection: "column", gap: 16, maxWidth: 820 }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Text size="sm" tone="muted">Separate game folders, each with its own worlds, options and packs.</Text>
        <div style={{ flexGrow: 1 }} />
        <Button variant="ghost" size="sm" onClick={() => setCreating((value) => !value)}>
          {creating ? "Cancel" : "New instance"}
        </Button>
      </div>

      {creating ? (
        <div style={{ ...panel, padding: 14, gap: 10 }}>
          <Text mono size="xs" tone="muted">NEW INSTANCE</Text>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
            <div style={{ width: 260 }}>
              <Input
                value={name}
                placeholder="A name, 1 to 40 characters"
                onChange={(event) => setName(event.value ?? "")}
              />
            </div>
            <Button
              variant="secondary"
              onClick={() => {
                void createInstance(name.trim())
                  .then(() => {
                    setCreating(false)
                    setName("")
                    setNote(null)
                    onRefresh()
                  })
                  .catch((error: Error) => setNote(error.message))
              }}
            >
              Create
            </Button>
          </div>
          {note ? <Text size="xs" tone="muted">{note}</Text> : null}
        </div>
      ) : null}

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
              borderLeftWidth: instance.selected ? 2 : 0,
              borderStyle: "solid",
              borderColor: theme.colors.border,
              backgroundColor: instance.selected ? theme.colors.accent : theme.colors.card,
              cursor: "pointer",
              hover: { backgroundColor: theme.colors.muted },
            }}
            onClick={() => {
              void selectInstance(instance.id)
                .then(() => onRefresh())
                .catch((error: Error) => setNote(error.message))
            }}
          >
            <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
                <Text weight="medium">{instance.name}</Text>
                {instance.selected ? <Badge variant="outline">selected</Badge> : null}
              </div>
              <Text mono size="xs" tone="muted">{instance.id}</Text>
            </div>
            <Text mono size="xs" tone="muted" style={{ width: 80 }}>{instance.version}</Text>
            <Text size="xs" tone="muted" style={{ width: 90 }}>{KIND[instance.kind] ?? instance.kind}</Text>
            <div style={{ width: 90 }}>
              <Badge variant={instance.prepared ? "success" : "secondary"} dot>
                {instance.prepared ? "ready" : "prepare"}
              </Badge>
            </div>
            <Icon
              name={instance.selected ? "check" : "chevronRight"}
              size={14}
              color={instance.selected ? theme.colors.primary : theme.colors.mutedForeground}
            />
          </div>
        ))}
      </div>

      {note ? <Text size="xs" tone="muted">{note}</Text> : null}

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
