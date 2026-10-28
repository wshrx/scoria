import { useEffect, useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import type { DownloadProgress, Snapshot } from "../api"
import {
  downloadMinecraft,
  installLatest,
  prepare,
  removeVersion,
  selectVersion,
  versions,
  type VersionInfo,
} from "../api"
import { useT } from "../i18n"
import { Meter, Panel } from "../visual"

function megabytes(bytes: number): string {
  if (bytes >= 1_000_000_000) return `${(bytes / 1e9).toFixed(1)} GB`
  if (bytes <= 0) return "0 MB"
  return `${Math.round(bytes / 1e6)} MB`
}

export function FilesPage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 980
  const files = data.minecraft

  const [busy, setBusy] = useState<"download" | "prepare" | null>(null)
  const [note, setNote] = useState<string | null>(null)
  const [log, setLog] = useState<string[]>([])
  const [progress, setProgress] = useState<DownloadProgress | null>(null)

  const [installed, setInstalled] = useState<VersionInfo[]>([])
  const [selected, setSelected] = useState<string | null>(null)
  const [confirming, setConfirming] = useState<string | null>(null)
  const [versionBusy, setVersionBusy] = useState<string | null>(null)

  const refreshVersions = () => {
    void versions()
      .then((state) => {
        setInstalled(state.installed)
        setSelected(state.selected)
      })
      .catch((error: Error) => setNote(error.message))
  }

  useEffect(refreshVersions, [])

  const pushLog = (line: string) => setLog((lines) => [...lines.slice(-5), line])

  const download = async () => {
    setBusy("download")
    setNote(null)
    setLog([])
    setProgress(null)
    try {
      await downloadMinecraft(pushLog, setProgress)
      setNote(t("Every file is verified against Mojang's sha1."))
      onRefresh()
    } catch (error) {
      setNote(`The download stopped: ${(error as Error).message}`)
    } finally {
      setBusy(null)
      setProgress(null)
    }
  }

  const prepareNow = async () => {
    setBusy("prepare")
    setNote(null)
    setLog([])
    try {
      const said = await prepare(undefined, pushLog, true)
      setNote(said)
      onRefresh()
    } finally {
      setBusy(null)
    }
  }

  const done = (files.client ? 1 : 0) + files.libraries.done + files.assets.done
  const total = 1 + files.libraries.total + files.assets.total
  const percent = total > 0 ? Math.round((done / total) * 100) : 0

  const rows: { label: string; detail: string; good: boolean }[] = [
    {
      label: t("Version file"),
      detail: files.manifest ? t("from piston-meta") : t("missing"),
      good: files.manifest,
    },
    {
      label: t("Client jar"),
      detail: files.client ? t("verified") : t("missing"),
      good: files.client,
    },
    {
      label: t("Libraries"),
      detail: files.libraries.total > 0 ? `${files.libraries.done} / ${files.libraries.total}` : t("not fetched"),
      good: files.libraries.total > 0 && files.libraries.done === files.libraries.total,
    },
    {
      label: t("Assets"),
      detail: files.assets.total > 0 ? `${files.assets.done} / ${files.assets.total}` : t("not fetched"),
      good: files.assets.total > 0 && files.assets.done === files.assets.total,
    },
  ]

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 22 }}>
      <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
        <Text style={{ fontSize: 38, lineHeight: 42, fontWeight: "700" }}>{t("Game files")}</Text>
        <Text size="sm" tone="muted" style={{ maxWidth: 680 }}>
          {t(
            "Scoria runs Minecraft 26.3. The launcher reads an install you already have, or fetches the pinned version straight from Mojang's metadata.",
          )}
        </Text>
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "column" : "row",
          gap: 24,
          alignItems: "stretch",
        }}
      >
        <Panel style={{ flexGrow: 1, padding: 22, gap: 16 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-start", gap: 12 }}>
            <div style={{ display: "flex", flexDirection: "column", gap: 3, flexGrow: 1 }}>
              <Text size="xl" weight="semibold">
                {`Minecraft ${files.version}`}
              </Text>
              <Text mono size="xs" tone="muted">
                {files.source.replace("https://", "")}
              </Text>
            </div>
            <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
              <div
                style={{
                  width: 6,
                  height: 6,
                  borderRadius: 3,
                  backgroundColor: files.installed ? theme.colors.success : theme.colors.mutedForeground,
                }}
              />
              <Text mono size="xs" tone="muted">
                {files.installed ? t("installed") : t("incomplete")}
              </Text>
            </div>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: 7 }}>
            <Meter value={progress ? progress.done : done} total={progress ? progress.total : total} />
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text mono size="xs" tone="muted">
                {progress
                  ? `${progress.done} / ${progress.total} ${t("files")}`
                  : `${done} / ${total} ${t("files")}`}
              </Text>
              <Text mono size="xs" tone="muted">
                {`${progress ? (progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0) : percent}%`}
              </Text>
            </div>
          </div>

          <div style={{ display: "flex", flexDirection: "column" }}>
            {rows.map((row, index) => (
              <div
                key={row.label}
                style={{
                  display: "flex",
                  flexDirection: "row",
                  alignItems: "center",
                  gap: 10,
                  paddingTop: 10,
                  paddingBottom: 10,
                  borderTopWidth: index === 0 ? 1 : 1,
                  borderBottomWidth: index === rows.length - 1 ? 1 : 0,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                }}
              >
                <Icon
                  name={row.good ? "check" : "dot"}
                  size={13}
                  color={row.good ? theme.colors.success : theme.colors.mutedForeground}
                />
                <Text size="sm" style={{ flexGrow: 1 }}>
                  {row.label}
                </Text>
                <Text mono size="xs" tone="muted">
                  {row.detail}
                </Text>
              </div>
            ))}
          </div>

          <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 8 }}>
            <Button
              variant="primary"
              loading={busy === "download"}
              onClick={() => void download()}
              style={{ flexGrow: 1 }}
            >
              <Icon name="download" size={14} color={theme.colors.primaryForeground} />
              {files.installed
                ? t("Verify and repair")
                : files.missingBytes > 0
                  ? `${t("Download")} ${megabytes(files.missingBytes)}`
                  : t("Download game files")}
            </Button>
            <Button
              variant="secondary"
              loading={busy === "prepare"}
              onClick={() => void prepareNow()}
              style={{ flexGrow: 1 }}
            >
              {t("Prepare resources")}
            </Button>
          </div>

          {log.length > 0 ? (
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 3,
                padding: 10,
                borderRadius: theme.radius.sm,
                backgroundColor: theme.colors.muted,
              }}
            >
              {log.map((line, index) => (
                <Text key={index} mono size="xs" tone="muted" truncate>
                  {line}
                </Text>
              ))}
            </div>
          ) : null}

          {note ? (
            <Text size="xs" tone="muted">
              {note}
            </Text>
          ) : null}
        </Panel>

        <Panel style={{ width: narrow ? "100%" : 380, flexShrink: 0, padding: 22, gap: 14 }}>
          <Text size="md" weight="semibold">
            {t("Where they live")}
          </Text>
          <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
            <Text mono size="xs" tone="muted">
              {t("Folder")}
            </Text>
            <Text mono size="xs" style={{ color: theme.colors.foreground }}>
              {files.path}
            </Text>
          </div>
          <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
            <Text mono size="xs" tone="muted">
              {t("Version manifest")}
            </Text>
            <Text mono size="xs" style={{ color: theme.colors.foreground }}>
              {files.manifestSha1 ?? t("unknown")}
            </Text>
          </div>
          <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
            <Text mono size="xs" tone="muted">
              {t("Already on disk")}
            </Text>
            <Text size="xs">
              {files.installed
                ? t("Every file the version needs is present.")
                : files.manifest
                  ? `${megabytes(files.missingBytes)} ${t("still to fetch.")}`
                  : t("No files fetched yet.")}
            </Text>
          </div>
          <div
            style={{
              display: "flex",
              flexDirection: "row",
              gap: 8,
              paddingTop: 12,
              borderTopWidth: 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
            }}
          >
            <Icon name="info" size={13} color={theme.colors.mutedForeground} />
            <Text size="xs" tone="muted" style={{ flexGrow: 1, maxWidth: 300 }}>
              {t(
                "Mojang's sha1 is checked for every file. A download stopped half way keeps what it finished; the next one fetches only the rest.",
              )}
            </Text>
          </div>
        </Panel>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
        <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 10 }}>
          <Text size="lg" weight="semibold">
            {t("Launcher builds")}
          </Text>
          <Text mono size="xs" tone="muted">
            {`${installed.length} ${t("installed")}`}
          </Text>
          <div style={{ flexGrow: 1 }} />
          <Button
            variant="ghost"
            size="sm"
            onClick={() => {
              setVersionBusy("latest")
              void installLatest()
                .then(() => {
                  setNote(t("Installed the latest build."))
                  refreshVersions()
                  onRefresh()
                })
                .catch((error: Error) => setNote(error.message))
                .finally(() => setVersionBusy(null))
            }}
          >
            <Icon name="download" size={13} color={theme.colors.mutedForeground} />
            {t("Install latest")}
          </Button>
        </div>

        <div style={{ display: "flex", flexDirection: "column", borderTopWidth: 1, borderStyle: "solid", borderColor: theme.colors.border }}>
          {installed.length === 0 ? (
            <Text size="sm" tone="muted" style={{ paddingTop: 14 }}>
              {t("No versions installed.")}
            </Text>
          ) : (
            installed.map((version) => (
              <div
                key={version.tag}
                style={{
                  display: "flex",
                  flexDirection: narrow ? "column" : "row",
                  alignItems: narrow ? "stretch" : "center",
                  gap: 10,
                  paddingTop: 11,
                  paddingBottom: 11,
                  borderBottomWidth: 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                }}
              >
                <Text mono size="sm" style={{ flexGrow: 1 }}>
                  {version.tag}
                </Text>
                <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 5 }}>
                  <Icon
                    name={version.prepared ? "check" : "dot"}
                    size={12}
                    color={version.prepared ? theme.colors.success : theme.colors.mutedForeground}
                  />
                  <Text size="xs" tone="muted">
                    {version.prepared ? t("resources prepared") : t("needs preparing")}
                  </Text>
                </div>
                {selected === version.tag ? (
                  <Text mono size="xs" style={{ color: theme.colors.primary, width: 62 }}>
                    {t("selected")}
                  </Text>
                ) : (
                  <Button
                    size="sm"
                    variant="ghost"
                    loading={versionBusy === version.tag}
                    onClick={() => {
                      setVersionBusy(version.tag)
                      void selectVersion(version.tag)
                        .then(() => {
                          refreshVersions()
                          onRefresh()
                        })
                        .catch((error: Error) => setNote(error.message))
                        .finally(() => setVersionBusy(null))
                    }}
                  >
                    {t("Use")}
                  </Button>
                )}
                <Button
                  size="sm"
                  variant={confirming === version.tag ? "destructive" : "ghost"}
                  onClick={() => {
                    if (confirming !== version.tag) {
                      setConfirming(version.tag)
                      return
                    }
                    setConfirming(null)
                    void removeVersion(version.tag)
                      .then(() => {
                        refreshVersions()
                        onRefresh()
                      })
                      .catch((error: Error) => setNote(error.message))
                  }}
                >
                  {confirming === version.tag ? t("Confirm") : t("Remove")}
                </Button>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  )
}
