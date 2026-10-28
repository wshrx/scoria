import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Snapshot } from "../api"
import { createInstance, selectInstance } from "../api"
import { useT } from "../i18n"
import { Panel, StrataThumb } from "../visual"

const KIND: Record<string, string> = {
  release: "release",
  modpack: "modpack",
  port: "ported mod",
}

export function InstancesPage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 900
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState("")
  const [note, setNote] = useState<string | null>(null)
  const [busy, setBusy] = useState<string | null>(null)

  const create = () => {
    setBusy("create")
    void createInstance(name.trim())
      .then(() => {
        setCreating(false)
        setName("")
        setNote(null)
        onRefresh()
      })
      .catch((error: Error) => setNote(error.message))
      .finally(() => setBusy(null))
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 22 }}>
      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "column" : "row",
          alignItems: narrow ? "flex-start" : "flex-end",
          gap: 16,
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: 6, flexGrow: 1 }}>
          <Text style={{ fontSize: 38, lineHeight: 42, fontWeight: "700" }}>{t("Instances")}</Text>
          <Text size="sm" tone="muted" style={{ maxWidth: 640 }}>
            {t("Separate game folders, each with its own worlds, options and packs.")}
          </Text>
        </div>
        <Button
          variant={creating ? "ghost" : "secondary"}
          size="sm"
          onClick={() => setCreating((value) => !value)}
        >
          <Icon name={creating ? "x" : "plus"} size={13} color={theme.colors.mutedForeground} />
          {creating ? t("Cancel") : t("New instance")}
        </Button>
      </div>

      {creating ? (
        <Panel style={{ padding: 16, gap: 10, borderStyle: "dashed" }}>
          <Text size="xs" tone="muted">
            {t("A new folder starts empty. Use it for a pack, a port, or a clean survival world.")}
          </Text>
          <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 8, alignItems: narrow ? "stretch" : "center" }}>
            <div style={{ width: narrow ? "100%" : 320 }}>
              <Input
                value={name}
                placeholder={t("A name, 1 to 40 characters")}
                onChange={(event) => setName(event.value ?? "")}
              />
            </div>
            <Button variant="primary" loading={busy === "create"} onClick={create}>
              {t("Create")}
            </Button>
          </div>
          {note ? (
            <Text size="xs" tone="destructive">
              {note}
            </Text>
          ) : null}
        </Panel>
      ) : null}

      <div style={{ display: "flex", flexDirection: "column", borderTopWidth: 1, borderStyle: "solid", borderColor: theme.colors.border }}>
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
                  .catch((error: Error) => setNote(error.message))
                  .finally(() => setBusy(null))
              }}
              style={{
                display: "flex",
                flexDirection: narrow ? "column" : "row",
                alignItems: narrow ? "stretch" : "center",
                gap: 16,
                paddingTop: 14,
                paddingBottom: 14,
                borderBottomWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
                position: "relative",
                paddingLeft: 18,
                backgroundColor: selected ? theme.colors.accent : "transparent",
                cursor: selected ? "default" : "pointer",
                hover: { backgroundColor: theme.colors.accent },
              }}
            >
              <div
                style={{
                  position: "absolute",
                  left: 0,
                  top: 0,
                  bottom: 0,
                  width: 2,
                  backgroundColor: selected ? theme.colors.primary : "transparent",
                }}
              />
              <StrataThumb seed={index + 2} width={narrow ? 120 : 150} height={narrow ? 64 : 84} />
              <div style={{ display: "flex", flexDirection: "column", gap: 5, flexGrow: 1, minWidth: 0 }}>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                  <Text size="md" weight="semibold" truncate>
                    {instance.name}
                  </Text>
                  {selected ? (
                    <Text mono size="xs" style={{ color: theme.colors.primary }}>
                      {t("in use")}
                    </Text>
                  ) : null}
                </div>
                <Text mono size="xs" tone="muted" truncate>
                  {instance.id}
                </Text>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10, flexWrap: "wrap" }}>
                  <Text size="xs" tone="muted" truncate>
                    {`Minecraft ${instance.minecraft}`}
                  </Text>
                  <Text size="xs" tone="muted" truncate>
                    {t(KIND[instance.kind] ?? instance.kind)}
                  </Text>
                  <div style={{ display: "flex", alignItems: "center", gap: 5 }}>
                    <Icon
                      name={instance.prepared ? "check" : "dot"}
                      size={11}
                      color={instance.prepared ? theme.colors.success : theme.colors.mutedForeground}
                    />
                    <Text size="xs" tone="muted" truncate>
                      {instance.prepared ? t("ready") : t("needs preparing")}
                    </Text>
                  </div>
                </div>
              </div>

              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12 }}>
                {selected ? (
                  <Text mono size="xs" tone="muted">
                    {t("selected")}
                  </Text>
                ) : (
                  <Button size="sm" variant="outline" loading={busy === instance.id}>
                    {t("Use")}
                    <Icon name="arrowRight" size={13} color={theme.colors.foreground} />
                  </Button>
                )}
              </div>
            </div>
          )
        })}
      </div>

      {note ? (
        <Text size="xs" tone="destructive">
          {note}
        </Text>
      ) : null}

      <Panel style={{ padding: 18, gap: 6 }}>
        <Text size="sm" weight="medium">
          {t("Scoria also reads the launchers already on this machine")}
        </Text>
        <Text size="xs" tone="muted">
          {t(
            "The official launcher, Prism, Freesm, MultiMC, the Modrinth App and ATLauncher are found automatically. If none has Minecraft 26.3, Scoria downloads it from Mojang into its own folder.",
          )}
        </Text>
      </Panel>
    </div>
  )
}
