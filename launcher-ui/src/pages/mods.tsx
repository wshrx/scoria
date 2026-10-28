import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { ModEntry, ModProject, Snapshot } from "../api"
import { installMod, installedMods, removeMod, searchMods } from "../api"
import { useT } from "../i18n"
import { Meter, Sheet } from "../visual"

type ModKind = "mod" | "resource-pack"

function supportTone(support: string, theme: ReturnType<typeof useTheme>) {
  if (support === "Works") return { dot: theme.colors.success, bg: "#7FD8A41A" }
  if (support === "Partly") return { dot: "#D8B26A", bg: "#D8B26A1A" }
  return { dot: theme.colors.destructive, bg: "#E5484D14" }
}

function kindLabel(kind: string, t: (text: string) => string) {
  return kind === "resource-pack" ? t("Resource pack") : t("Mod")
}

export function ModsPage({ data }: { data: Snapshot }) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 1000

  const [kind, setKind] = useState<ModKind>("mod")
  const [query, setQuery] = useState("")
  const [results, setResults] = useState<ModProject[]>([])
  const [entries, setEntries] = useState<ModEntry[]>([])
  const [searching, setSearching] = useState(false)
  const [busy, setBusy] = useState<string | null>(null)
  const [progress, setProgress] = useState<{ label: string; value: number } | null>(null)
  const [note, setNote] = useState<string | null>(null)
  const [confirming, setConfirming] = useState<string | null>(null)

  const loadInstalled = () => {
    void installedMods()
      .then((state) => setEntries(state.entries))
      .catch((error: Error) => {
        if (data.live) setNote(error.message)
      })
  }

  useEffect(loadInstalled, [])

  useEffect(() => {
    setSearching(true)
    setNote(null)
    void searchMods("", kind)
      .then(setResults)
      .catch((error: Error) => {
        if (data.live) setNote(error.message)
      })
      .finally(() => setSearching(false))
  }, [kind])

  const search = () => {
    setSearching(true)
    setNote(null)
    void searchMods(query, kind)
      .then(setResults)
      .catch((error: Error) => setNote(error.message))
      .finally(() => setSearching(false))
  }

  const install = (project: ModProject) => {
    setBusy(project.id)
    setNote(null)
    setProgress({ label: "", value: 0 })
    void installMod(project.id, (label, value) => setProgress({ label, value }))
      .then(() => {
        loadInstalled()
        setNote(t("Installed."))
      })
      .catch((error: Error) => setNote(error.message))
      .finally(() => {
        setBusy(null)
        setProgress(null)
      })
  }

  const percent = progress ? Math.round(progress.value * 100) : 0

  return (
    <Sheet style={{ padding: narrow ? 20 : 30, gap: 20, minHeight: 0 }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 16 }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 4, flexGrow: 1 }}>
          <Text style={{ fontSize: narrow ? 30 : 38, lineHeight: 42, fontWeight: "700" }}>
            {t("Mods")}
          </Text>
          <Text size="sm" tone="muted">
            {t("Modrinth mods and resource packs. Java mods are ported; packs install as they are.")}
          </Text>
        </div>
        <div style={{ display: "flex", flexDirection: "row", gap: 6 }}>
          {[
            ["mod", t("Mods")],
            ["resource-pack", t("Resource packs")],
          ].map(([value, label]) => (
            <Button
              key={value}
              size="sm"
              variant={kind === value ? "secondary" : "ghost"}
              onClick={() => setKind(value as ModKind)}
              style={{ borderRadius: 999, height: 32, paddingLeft: 14, paddingRight: 14 }}
            >
              {label}
            </Button>
          ))}
        </div>
      </div>

      <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 8 }}>
        <div style={{ flexGrow: 1 }}>
          <Input
            value={query}
            placeholder={t("Search Modrinth")}
            onChange={(event) => setQuery(event.value ?? "")}
            onSubmit={search}
          />
        </div>
        <Button
          variant="primary"
          loading={searching}
          onClick={search}
          style={{ borderRadius: 999, height: 38, paddingLeft: 18, paddingRight: 18 }}
        >
          {t("Search")}
        </Button>
      </div>

      {busy && progress ? (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          <Meter value={progress.value} total={1} height={4} />
          <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
            <Text mono size="xs" tone="muted">
              {t("Installing…")}
            </Text>
            <Text mono size="xs" tone="muted">
              {`${percent}%`}
            </Text>
          </div>
        </div>
      ) : null}

      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {searching && results.length === 0 ? (
          <Text size="sm" tone="muted">
            {t("Searching…")}
          </Text>
        ) : results.length === 0 ? (
          <Text size="sm" tone="muted">
            {t("Nothing found.")}
          </Text>
        ) : (
          results.map((project) => {
            const tone = supportTone(project.support, theme)
            return (
              <div
                key={project.id}
                style={{
                  display: "flex",
                  flexDirection: narrow ? "column" : "row",
                  alignItems: narrow ? "stretch" : "center",
                  gap: 12,
                  padding: 12,
                  borderRadius: 16,
                  backgroundColor: theme.colors.muted,
                }}
              >
                <div style={{ display: "flex", flexDirection: "column", gap: 3, flexGrow: 1, minWidth: 0 }}>
                  <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                    <Text size="base" weight="semibold" truncate>
                      {project.title}
                    </Text>
                    <div
                      style={{
                        display: "flex",
                        alignItems: "center",
                        gap: 5,
                        height: 20,
                        paddingLeft: 8,
                        paddingRight: 8,
                        borderRadius: 999,
                        backgroundColor: tone.bg,
                      }}
                    >
                      <div style={{ width: 5, height: 5, borderRadius: 3, backgroundColor: tone.dot }} />
                      <Text mono size="xs" tone="muted">
                        {t(project.support)}
                      </Text>
                    </div>
                  </div>
                  <Text size="xs" tone="muted" truncate>
                    {project.author}
                  </Text>
                  <Text size="xs" tone="muted" style={{ lineClamp: 2 }}>
                    {project.summary}
                  </Text>
                  {!project.installable && project.reason ? (
                    <Text size="xs" tone="muted">
                      {project.reason}
                    </Text>
                  ) : null}
                </div>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                  {project.installable ? (
                    <Button
                      size="sm"
                      variant="secondary"
                      loading={busy === project.id}
                      onClick={() => install(project)}
                      style={{ borderRadius: 999, height: 30, paddingLeft: 14, paddingRight: 14 }}
                    >
                      <Icon name="download" size={12} color={theme.colors.foreground} />
                      {t("Install")}
                    </Button>
                  ) : (
                    <Text mono size="xs" tone="muted">
                      {t("Doesn't run")}
                    </Text>
                  )}
                </div>
              </div>
            )
          })
        )}
      </div>

      <div style={{ height: 1, backgroundColor: theme.colors.border }} />

      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 8 }}>
          <Text size="md" weight="semibold">
            {t("Installed")}
          </Text>
          <Text mono size="xs" tone="muted">
            {entries.length}
          </Text>
        </div>
        {entries.length === 0 ? (
          <Text size="xs" tone="muted">
            {t("Nothing installed from Modrinth yet.")}
          </Text>
        ) : (
          entries.map((entry) => (
            <div
              key={`${entry.source}-${entry.project}`}
              style={{
                display: "flex",
                flexDirection: narrow ? "column" : "row",
                alignItems: narrow ? "stretch" : "center",
                gap: 10,
                paddingTop: 8,
                paddingBottom: 8,
                borderTopWidth: 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
              }}
            >
              <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1, minWidth: 0 }}>
                <Text size="sm" weight="medium" truncate>
                  {entry.title}
                </Text>
                <Text mono size="xs" tone="muted" truncate>
                  {`${kindLabel(entry.kind, t)} · ${entry.number}`}
                </Text>
              </div>
              <Button
                size="sm"
                variant={confirming === entry.project ? "destructive" : "ghost"}
                onClick={() => {
                  if (confirming !== entry.project) {
                    setConfirming(entry.project)
                    return
                  }
                  setConfirming(null)
                  void removeMod(entry.project)
                    .then(setEntries)
                    .catch((error: Error) => setNote(error.message))
                }}
                style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
              >
                {confirming === entry.project ? t("Confirm") : t("Remove")}
              </Button>
            </div>
          ))
        )}
      </div>

      {note ? (
        <Text size="xs" tone="muted">
          {note}
        </Text>
      ) : null}
    </Sheet>
  )
}
