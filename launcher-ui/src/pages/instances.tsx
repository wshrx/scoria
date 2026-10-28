import { useState } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Snapshot } from "../api"
import { createInstance, selectInstance } from "../api"
import { useT } from "../i18n"
import { gradient, Thumb } from "../visual"

const KIND: Record<string, string> = {
  release: "release",
  modpack: "modpack",
  port: "ported mod",
}

/** Instance cards: a thumbnail, the version, and one action. */
export function InstancesPage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const t = useT()
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState("")
  const [note, setNote] = useState<string | null>(null)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 18, maxWidth: 1080, width: "100%", alignSelf: "center" }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Text size="sm" tone="muted">
          {t("Separate game folders, each with its own worlds, options and packs.")}
        </Text>
        <div style={{ flexGrow: 1 }} />
        <Button variant="ghost" size="sm" onClick={() => setCreating((value) => !value)}>
          {creating ? t("Cancel") : t("New instance")}
        </Button>
      </div>

      {creating ? (
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 10,
            padding: 16,
            borderRadius: theme.radius.lg,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "#FFFFFF14",
            background: gradient("#151110F5", "#0E0B0AF7", 160),
          }}
        >
          <Text mono size="xs" tone="muted">NEW INSTANCE</Text>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
            <div style={{ width: 280 }}>
              <Input
                value={name}
                placeholder={t("A name, 1 to 40 characters")}
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
              {t("Create")}
            </Button>
          </div>
          {note ? <Text size="xs" tone="muted">{note}</Text> : null}
        </div>
      ) : null}

      <div style={{ display: "flex", flexDirection: "row", flexWrap: "wrap", gap: 16 }}>
        {data.instances.map((instance, index) => (
          <div
            key={instance.id}
            style={{
              display: "flex",
              flexDirection: "column",
              width: 300,
              overflow: "hidden",
              borderRadius: theme.radius.lg,
              borderWidth: 1,
              borderStyle: "solid",
              borderColor: instance.selected ? theme.colors.primary : "#FFFFFF12",
              background: gradient("#151110FA", "#0D0A09FC", 160),
              boxShadow: theme.shadow.md,
              cursor: "pointer",
              hover: { borderColor: theme.colors.primary },
            }}
            onClick={() => {
              void selectInstance(instance.id)
                .then(() => onRefresh())
                .catch((error: Error) => setNote(error.message))
            }}
          >
            <Thumb seed={index} width={298} height={94} />
            <div style={{ display: "flex", flexDirection: "column", gap: 8, padding: 16 }}>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
                <Text weight="semibold" style={{ flexGrow: 1 }}>{instance.name}</Text>
                {instance.selected ? <Badge variant="outline">{t("selected")}</Badge> : null}
              </div>
              <Text mono size="xs" tone="muted">{instance.id}</Text>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                <Text mono size="xs" tone="muted">MC {instance.version}</Text>
                <Badge variant="secondary">{KIND[instance.kind] ?? instance.kind}</Badge>
                <div style={{ flexGrow: 1 }} />
                <Badge variant={instance.prepared ? "success" : "secondary"} dot>
                  {t(instance.prepared ? "ready" : "prepare")}
                </Badge>
              </div>
            </div>
          </div>
        ))}
      </div>

      {note ? <Text size="xs" tone="muted">{note}</Text> : null}

      <div
        style={{
          padding: 16,
          borderRadius: theme.radius.lg,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#FFFFFF10",
          background: "#0E0B0A",
        }}
      >
        <Text mono size="xs" tone="muted">{t("DETECTED INSTALLS")}</Text>
        <Text size="xs" tone="muted" style={{ marginTop: 6 }}>
          The official launcher, Prism, Freesm, MultiMC and the Modrinth App are found automatically.
        </Text>
      </div>
    </div>
  )
}
