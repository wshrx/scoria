import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import type { DownloadProgress } from "../api"
import { downloadMinecraft, play, prepare } from "../api"
import { useT, useLang } from "../i18n"
import { ago } from "../lib/time"
import { downloadStopped, friendlyError } from "../errors"
import { EmptyState } from "../components/feedback/empty-state"
import { useToast } from "../components/feedback/toast"
import { PageHeader } from "../components/layout/page-header"
import type { ScreenProps } from "./registry"
import { Meter, Panel, SceneThumb } from "../visual"

function megabytes(bytes: number): string {
  if (bytes >= 1_000_000_000) return `${(bytes / 1e9).toFixed(1)} GB`
  return `${Math.round(bytes / 1e6)} MB`
}

export function HomePage({ data, onRefresh, onPage }: ScreenProps) {
  const theme = useTheme()
  const t = useT()
  const lang = useLang()
  const toast = useToast()
  const { width } = useWindowSize()
  const narrow = width < 1060
  const [busy, setBusy] = useState(false)
  const [progress, setProgress] = useState<DownloadProgress | null>(null)

  const files = data.minecraft
  const ready = files.installed && data.prepared
  const playing = data.playing
  const recent = data.worlds[0]

  const summary = ready
    ? t("Ready to play. Your worlds and settings stay on this machine.")
    : !files.installed
      ? t("Minecraft 26.3 is not here yet. Download it once and it is yours.")
      : t("The files are here. One more step prepares the resources.")

  const percent = progress && progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0

  const playNow = (world?: string) => {
    setBusy(true)
    void play(world ? { world } : {})
      .then((said) => {
        if (!said.startsWith("Started")) {
          toast({ tone: "error", title: friendlyError(said, t) })
        }
      })
      .catch((error: Error) => toast({ tone: "error", title: friendlyError(error, t) }))
      .finally(() => setBusy(false))
  }

  const prepareAll = async () => {
    setBusy(true)
    setProgress(null)
    try {
      if (!files.installed) await downloadMinecraft(undefined, setProgress)
      const said = await prepare(undefined, undefined, false)
      if (said.startsWith("Preparation failed")) {
        toast({ tone: "error", title: friendlyError(said, t) })
      } else {
        toast({ tone: "success", title: t("Resources prepared.") })
      }
      onRefresh()
    } catch (error) {
      toast({ tone: "error", title: downloadStopped(error, t) })
    } finally {
      setBusy(false)
      setProgress(null)
    }
  }

  const stats = [
    { label: t("Worlds"), value: String(data.worlds.length), icon: "layers" as const },
    { label: t("Minecraft"), value: files.version, icon: "globe" as const },
    {
      label: t("Status"),
      value: playing ? t("Playing") : ready ? t("ready") : t("needs attention"),
      icon: "check" as const,
    },
  ]

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 18 }}>
      <PageHeader
        title={t("Play")}
        subtitle={t("Your worlds, mods and servers in one place.")}
      />

      <Panel style={{ overflow: "hidden", flexDirection: narrow ? "column" : "row", padding: 0 }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 14, padding: narrow ? 18 : 24, flexGrow: 1, maxWidth: 720 }}>
          <Text mono size="xs" tone="muted">
            {t("Minecraft Java Edition").toUpperCase()}
          </Text>
          <Text size="2xl" weight="bold" truncate>
            {`Minecraft ${files.version}`}
          </Text>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, flexWrap: "wrap" }}>
            <Text size="xs" tone="muted">
              {`Minecraft ${files.version}`}
            </Text>
            <Text size="xs" tone="muted">·</Text>
            <Text size="xs" tone="muted">
              {ready ? t("resources prepared") : t("resources not prepared")}
            </Text>
          </div>
          <Text size="sm" tone="muted" style={{ maxWidth: 470, lineHeight: 20 }}>
            {summary}
          </Text>
          {playing ? (
            <Text size="xs" tone="muted">
              {t("The game is already running. Close its window to play again.")}
            </Text>
          ) : null}

          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, flexWrap: "wrap" }}>
            <Button
              size="lg"
              disabled={playing}
              loading={busy}
              onClick={() => void (ready ? playNow() : prepareAll())}
              style={{ height: 44, paddingLeft: 20, paddingRight: 20, borderRadius: 999 }}
            >
              <Icon
                name={playing ? "loader" : ready ? "play" : files.installed ? "refresh" : "download"}
                size={15}
                color={theme.colors.primaryForeground}
              />
              {playing ? t("Playing") : ready ? t("Play") : files.installed ? t("Prepare") : t("Download and prepare")}
            </Button>
            {ready && recent ? (
              <Button
                variant="secondary"
                onClick={() => playNow(recent.folder)}
                style={{ height: 44, paddingLeft: 16, paddingRight: 16, borderRadius: 999 }}
              >
                <Icon name="chevronRight" size={14} color={theme.colors.foreground} />
                {`${t("Continue")}: ${recent.name}`}
              </Button>
            ) : null}
          </div>

          {busy && progress ? (
            <div style={{ display: "flex", flexDirection: "column", gap: 6, maxWidth: 360 }}>
              <Meter value={progress.done} total={progress.total} height={4} />
              <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
                <Text mono size="xs" tone="muted">
                  {t("Downloading…")}
                </Text>
                <Text mono size="xs" tone="muted">
                  {`${percent}%`}
                </Text>
              </div>
            </div>
          ) : !files.installed && files.missingBytes > 0 ? (
            <Text mono size="xs" tone="muted">
              {`${megabytes(files.missingBytes)} ${t("to download")}`}
            </Text>
          ) : null}

        </div>

        <div style={{ width: narrow ? "100%" : 360, flexShrink: 0, padding: narrow ? 16 : 10 }}>
          <SceneThumb seed={0} width="100%" height={narrow ? 170 : 220} />
        </div>
      </Panel>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          borderRadius: 14,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
          backgroundColor: theme.colors.card,
        }}
      >
        {stats.map((stat, index) => (
          <div
            key={stat.label}
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 5,
              flexGrow: 1,
              minWidth: 0,
              paddingTop: 14,
              paddingBottom: 14,
              paddingLeft: 16,
              paddingRight: 10,
              borderLeftWidth: index === 0 ? 0 : 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
            }}
          >
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
              <Icon name={stat.icon} size={12} color={theme.colors.mutedForeground} />
              <Text mono size="xs" tone="muted">
                {stat.label}
              </Text>
            </div>
            <Text size="md" weight="semibold" truncate>
              {stat.value}
            </Text>
          </div>
        ))}
      </div>

      <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 16, alignItems: "flex-start" }}>
        <Panel style={{ flexGrow: 1, minWidth: 0, width: "100%" }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, padding: 16, paddingBottom: 8 }}>
            <Text size="md" weight="semibold" style={{ flexGrow: 1 }}>
              {t("Worlds")}
            </Text>
            <Text mono size="xs" tone="muted">
              {data.worlds.length}
            </Text>
          </div>
          {data.worlds.length === 0 ? (
            <div style={{ padding: 12, paddingTop: 4 }}>
              <EmptyState
                compact
                icon="layers"
                title={t("No worlds yet.")}
                description={t("Play once, create a world, and it will appear here.")}
              />
            </div>
          ) : (
            <div style={{ display: "flex", flexDirection: "column", paddingLeft: 8, paddingRight: 8, paddingBottom: 8 }}>
              {data.worlds.slice(0, 4).map((world, index) => (
                <div
                  key={world.folder}
                  role="button"
                  tabIndex={0}
                  aria-label={world.name}
                  onClick={() => playNow(world.folder)}
                  onKeyDown={(event) => {
                    if (event.key === "enter" || event.key === " " || event.key === "space") {
                      event.preventDefault()
                      playNow(world.folder)
                    }
                  }}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: "center",
                    gap: 12,
                    padding: 8,
                    borderRadius: 10,
                    cursor: "pointer",
                    hover: { backgroundColor: theme.colors.accent },
                    focusVisible: { outlineWidth: 2, outlineColor: theme.colors.ring, outlineOffset: -2 },
                  }}
                >
                  <SceneThumb seed={index + 2} width={64} height={40} />
                  <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1, minWidth: 0 }}>
                    <Text size="sm" weight="medium" truncate>
                      {world.name}
                    </Text>
                    <Text mono size="xs" tone="muted" truncate>
                      {`${t(world.mode)} · ${world.size} · ${ago(world.lastPlayedAt, lang) || world.lastPlayed}`}
                    </Text>
                  </div>
                  <Icon name="play" size={13} color={theme.colors.mutedForeground} />
                </div>
              ))}
            </div>
          )}
        </Panel>

        <div style={{ display: "flex", flexDirection: "column", gap: 16, width: narrow ? "100%" : 320, flexShrink: 0 }}>
          <Panel style={{ padding: 16, gap: 10 }}>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
              <Icon name="box" size={15} color={theme.colors.primary} />
              <Text size="sm" weight="semibold" style={{ flexGrow: 1 }}>
                {t("Mods")}
              </Text>
            </div>
            <Text size="xs" tone="muted">
              {t("Modrinth mods and resource packs.")}
            </Text>
            <Button variant="secondary" size="sm" onClick={() => onPage("mods")} style={{ borderRadius: 999, height: 30 }}>
              {t("Open Mods")}
            </Button>
          </Panel>

          <Panel style={{ padding: 16, gap: 10 }}>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
              <Icon name="server" size={15} color={theme.colors.primary} />
              <Text size="sm" weight="semibold" style={{ flexGrow: 1 }}>
                {t("Servers")}
              </Text>
            </div>
            <Text size="xs" tone="muted">
              {t("Offline-mode servers and LAN.")}
            </Text>
            <Button variant="secondary" size="sm" onClick={() => onPage("servers")} style={{ borderRadius: 999, height: 30 }}>
              {t("Open Servers")}
            </Button>
          </Panel>
        </div>
      </div>
    </div>
  )
}
