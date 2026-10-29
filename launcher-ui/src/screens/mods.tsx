import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Kbd } from "@/components/ui/kbd"
import type { ModEntry, ModProject } from "../api"
import { installMod, installedMods, removeMod, searchMods } from "../api"
import { useT } from "../i18n"
import { PageHeader } from "../components/layout/page-header"
import { Meter, Panel } from "../visual"

type ModKind = "mod" | "resource-pack"

function supportTone(support: string, theme: ReturnType<typeof useTheme>) {
  if (support === "Works") return { dot: theme.colors.success, bg: "#7FD8A41A" }
  if (support === "Partly") return { dot: "#D8B26A", bg: "#D8B26A1A" }
  return { dot: theme.colors.destructive, bg: "#E5484D14" }
}

function kindLabel(kind: string, t: (text: string) => string) {
  return kind === "resource-pack" ? t("Resource pack") : t("Mod")
}

export function ModsPage() {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 1060

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
      .catch((error: Error) => setNote(error.message))
  }

  useEffect(loadInstalled, [])

  const runSearch = (text = "") => {
    setSearching(true)
    setNote(null)
    void searchMods(text, kind)
      .then(setResults)
      .catch((error: Error) => setNote(error.message))
      .finally(() => setSearching(false))
  }

  useEffect(() => runSearch(""), [kind])

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
  const isInstalled = (project: ModProject) =>
    entries.some((entry) => entry.project === project.id || entry.slug === project.slug)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
      <PageHeader
        title={t("Mods")}
        subtitle={t("Modrinth mods and resource packs. Java mods are ported; packs install as they are.")}
        actions={
          <div style={{ display: "flex", flexDirection: "row", gap: 4 }}>
            {[
              ["mod", t("Mods")],
              ["resource-pack", t("Resource packs")],
            ].map(([value, label]) => (
              <Button
                key={value}
                size="sm"
                variant={kind === value ? "secondary" : "ghost"}
                onClick={() => setKind(value as ModKind)}
                style={{ borderRadius: 999, height: 30, paddingLeft: 13, paddingRight: 13 }}
              >
                {label}
              </Button>
            ))}
          </div>
        }
      />

      <Panel style={{ padding: 10 }}>
        <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
          <div style={{ flexGrow: 1 }}>
            <Input
              value={query}
              placeholder={t("Search Modrinth")}
              onChange={(event) => setQuery(event.value ?? "")}
              onSubmit={() => runSearch(query)}
            />
          </div>
          {narrow ? null : <Kbd>Enter</Kbd>}
          <Button
            variant="primary"
            loading={searching}
            onClick={() => runSearch(query)}
            style={{ borderRadius: 999, height: 34, paddingLeft: 16, paddingRight: 16 }}
          >
            {t("Search")}
          </Button>
        </div>
        {progress ? (
          <div style={{ display: "flex", flexDirection: "column", gap: 5, paddingTop: 10 }}>
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
      </Panel>

      <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 16, alignItems: "flex-start" }}>
        <Panel style={{ flexGrow: 1, width: "100%", minWidth: 0 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, padding: 14, paddingBottom: 8 }}>
            <Text size="sm" weight="semibold" style={{ flexGrow: 1 }}>
              {kind === "mod" ? t("Mods") : t("Resource packs")}
            </Text>
            <Text mono size="xs" tone="muted">
              {searching ? t("Searching…") : `${results.length} ${t("results")}`}
            </Text>
          </div>
          {results.length === 0 && !searching ? (
            <Text size="sm" tone="muted" style={{ padding: 14, paddingTop: 4 }}>
              {t("Nothing found.")}
            </Text>
          ) : (
            <div style={{ display: "flex", flexDirection: "column" }}>
              {results.map((project) => {
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
                      paddingLeft: 14,
                      paddingRight: 14,
                      borderTopWidth: 1,
                      borderStyle: "solid",
                      borderColor: theme.colors.border,
                      hover: { backgroundColor: theme.colors.accent },
                    }}
                  >
                    <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1, minWidth: 0 }}>
                      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 7 }}>
                        <Text size="sm" weight="semibold" truncate>
                          {project.title}
                        </Text>
                        <div
                          style={{
                            display: "flex",
                            alignItems: "center",
                            gap: 5,
                            height: 20,
                            paddingLeft: 7,
                            paddingRight: 7,
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
                      <Text mono size="xs" tone="muted" truncate>
                        {project.author}
                      </Text>
                      <Text size="xs" tone="muted" style={{ lineClamp: 2 }}>
                        {project.summary}
                      </Text>
                    </div>
                    {isInstalled(project) ? (
                      <div
                        style={{
                          display: "flex",
                          flexDirection: "row",
                          alignItems: "center",
                          gap: 5,
                          height: 28,
                          paddingLeft: 10,
                          paddingRight: 10,
                          borderRadius: 999,
                          borderWidth: 1,
                          borderStyle: "solid",
                          borderColor: theme.colors.successBorder,
                          backgroundColor: "#7FD8A41A",
                        }}
                      >
                        <Icon name="check" size={12} color={theme.colors.success} />
                        <Text mono size="xs" tone="muted">
                          {t("Installed")}
                        </Text>
                      </div>
                    ) : project.installable ? (
                      <Button
                        size="sm"
                        variant="secondary"
                        loading={busy === project.id}
                        onClick={() => install(project)}
                        style={{ borderRadius: 999, height: 28, paddingLeft: 12, paddingRight: 12 }}
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
                )
              })}
            </div>
          )}
        </Panel>

        <Panel style={{ width: narrow ? "100%" : 320, flexShrink: 0 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 8, padding: 14, paddingBottom: 8 }}>
            <Text size="sm" weight="semibold" style={{ flexGrow: 1 }}>
              {t("Installed")}
            </Text>
            <Text mono size="xs" tone="muted">
              {entries.length}
            </Text>
          </div>
          {entries.length === 0 ? (
            <Text size="xs" tone="muted" style={{ padding: 14, paddingTop: 4 }}>
              {t("Nothing installed from Modrinth yet.")}
            </Text>
          ) : (
            entries.map((entry) => (
              <div
                key={`${entry.source}-${entry.project}`}
                style={{
                  display: "flex",
                  flexDirection: "column",
                  gap: 6,
                  padding: 12,
                  paddingLeft: 14,
                  paddingRight: 14,
                  borderTopWidth: 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                }}
              >
                <Text size="xs" weight="medium" truncate>
                  {entry.title}
                </Text>
                <Text mono size="xs" tone="muted" truncate>
                  {`${kindLabel(entry.kind, t)} · ${entry.number}`}
                </Text>
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
                  style={{ borderRadius: 999, height: 26, paddingLeft: 10, paddingRight: 10, alignSelf: "flex-start" }}
                >
                  {confirming === entry.project ? t("Confirm") : t("Remove")}
                </Button>
              </div>
            ))
          )}
        </Panel>
      </div>

      {note ? (
        <Text size="xs" tone="muted">
          {note}
        </Text>
      ) : null}
    </div>
  )
}
