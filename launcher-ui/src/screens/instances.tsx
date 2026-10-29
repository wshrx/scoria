import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Snapshot } from "../api"
import { createInstance, selectInstance } from "../api"
import { useT } from "../i18n"
import { PageHeader } from "../components/layout/page-header"
import { useToast } from "../components/feedback/toast"
import { friendlyError } from "../errors"
import { SceneThumb } from "../visual"

const KIND: Record<string, string> = {
  release: "release",
  modpack: "modpack",
  port: "ported mod",
}

export function InstancesPage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const t = useT()
  const toast = useToast()
  const { width } = useWindowSize()
  const narrow = width < 1000
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState("")
  const [busy, setBusy] = useState<string | null>(null)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 18 }}>
      <PageHeader
        title={t("Instances")}
        subtitle={t("Separate game folders, each with its own worlds, options and packs.")}
        actions={
          <Button
            variant={creating ? "ghost" : "secondary"}
            onClick={() => setCreating((value) => !value)}
            style={{ borderRadius: 999, height: 36, paddingLeft: 16, paddingRight: 16 }}
          >
            <Icon name={creating ? "x" : "plus"} size={13} color={theme.colors.foreground} />
            {creating ? t("Cancel") : t("New instance")}
          </Button>
        }
      />
      {creating ? (
        <div
          style={{
            display: "flex",
            flexDirection: narrow ? "column" : "row",
            alignItems: narrow ? "stretch" : "center",
            gap: 8,
            padding: 14,
            borderRadius: 18,
            borderWidth: 1,
            borderStyle: "dashed",
            borderColor: theme.colors.border,
            backgroundColor: theme.colors.muted,
          }}
        >
          <div style={{ width: narrow ? "100%" : 320 }}>
            <Input
              value={name}
              placeholder={t("A name, 1 to 40 characters")}
              onChange={(event) => setName(event.value ?? "")}
            />
          </div>
          <Button
            variant="primary"
            loading={busy === "create"}
            onClick={() => {
              setBusy("create")
              void createInstance(name.trim())
                .then(() => {
                  setCreating(false)
                  setName("")
                  toast({ tone: "success", title: t("Instance created.") })
                  onRefresh()
                })
                .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                .finally(() => setBusy(null))
            }}
            style={{ borderRadius: 999, height: 36, paddingLeft: 18, paddingRight: 18 }}
          >
            {t("Create")}
          </Button>
        </div>
      ) : null}

      <div style={{ display: "flex", flexDirection: "row", flexWrap: "wrap", gap: 16 }}>
        {data.instances.map((instance, index) => {
          const selected = instance.selected
          return (
            <div
              key={instance.id}
              onClick={() => {
                if (selected) return
                setBusy(instance.id)
                void selectInstance(instance.id)
                  .then(() => onRefresh())
                  .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
                  .finally(() => setBusy(null))
              }}
              style={{
                display: "flex",
                flexDirection: "column",
                width: narrow ? "100%" : 340,
                overflow: "hidden",
                borderRadius: 20,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: selected ? theme.colors.primary : theme.colors.border,
                backgroundColor: selected ? "#FF8A4C14" : theme.colors.muted,
                cursor: selected ? "default" : "pointer",
                hover: selected ? {} : { borderColor: theme.colors.input },
              }}
            >
              <div style={{ padding: 8 }}>
                <SceneThumb seed={index + 1} width="100%" height={116} />
              </div>
              <div style={{ display: "flex", flexDirection: "column", gap: 8, paddingLeft: 16, paddingRight: 16, paddingBottom: 16 }}>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                  <Text size="md" weight="semibold" truncate style={{ flexGrow: 1 }}>
                    {instance.name}
                  </Text>
                  {selected ? (
                    <Text mono size="xs" style={{ color: theme.colors.primary }}>
                      {t("in use")}
                    </Text>
                  ) : null}
                </div>
                <Text size="xs" tone="muted">
                  {`Minecraft ${instance.minecraft} · ${t(KIND[instance.kind] ?? instance.kind)}`}
                </Text>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                  <div
                    style={{
                      display: "flex",
                      alignItems: "center",
                      gap: 5,
                      height: 22,
                      paddingLeft: 8,
                      paddingRight: 8,
                      borderRadius: 999,
                      backgroundColor: instance.prepared ? "#7FD8A41A" : theme.colors.muted,
                    }}
                  >
                    <div
                      style={{
                        width: 5,
                        height: 5,
                        borderRadius: 3,
                        backgroundColor: instance.prepared ? theme.colors.success : theme.colors.mutedForeground,
                      }}
                    />
                    <Text mono size="xs" tone={instance.prepared ? "default" : "muted"}>
                      {instance.prepared ? t("ready") : t("needs preparing")}
                    </Text>
                  </div>
                  <div style={{ flexGrow: 1 }} />
                  {!selected ? (
                    <Button
                      size="sm"
                      variant="secondary"
                      loading={busy === instance.id}
                      style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
                    >
                      {t("Use")}
                    </Button>
                  ) : null}
                </div>
              </div>
            </div>
          )
        })}
      </div>
    </div>
  )
}
