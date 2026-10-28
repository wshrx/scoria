import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme, type IconName } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import type { DownloadProgress, Snapshot } from "../api"
import { downloadMinecraft, play, prepare } from "../api"
import { useT } from "../i18n"
import { BONE, CHROME_MUTED, Meter } from "../visual"
import type { Page } from "../shell"

function megabytes(bytes: number): string {
  if (bytes >= 1_000_000_000) return `${(bytes / 1e9).toFixed(1)} GB`
  return `${Math.round(bytes / 1e6)} MB`
}

export function HomePage({
  data,
  onRefresh,
  onPage,
}: {
  data: Snapshot
  onRefresh: () => void
  onPage: (page: Page) => void
}) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 1000
  const [busy, setBusy] = useState<"play" | "download" | "prepare" | null>(null)
  const [note, setNote] = useState<string | null>(null)
  const [progress, setProgress] = useState<DownloadProgress | null>(null)

  const files = data.minecraft
  const ready = files.installed && data.prepared
  const recent = data.worlds[0]
  const instance = data.instances.find((entry) => entry.selected)?.name ?? "Scoria"

  const summary = ready
    ? t("Ready to play. Your worlds and settings stay on this machine.")
    : !files.installed
      ? t("Minecraft 26.3 is not here yet. Download it once and it is yours.")
      : t("The files are here. One more step prepares the resources.")

  const fail = (said: string) => setNote(said)

  const playNow = (world?: string) => {
    setBusy("play")
    setNote(null)
    void play(world ? { world } : {}).then((said) => {
      if (!said.startsWith("Started")) fail(said)
      setBusy(null)
    })
  }

  const prepareAll = async () => {
    setBusy("prepare")
    setNote(null)
    setProgress(null)
    try {
      if (!files.installed) await downloadMinecraft(undefined, setProgress)
      const said = await prepare(undefined, undefined, false)
      if (said.startsWith("Preparation failed")) fail(said)
      onRefresh()
    } catch (error) {
      fail(`The download stopped: ${(error as Error).message}`)
    } finally {
      setBusy(null)
      setProgress(null)
    }
  }

  const action = !files.installed || !data.prepared ? prepareAll : () => playNow()
  const actionLabel = !files.installed
    ? t("Download and prepare")
    : !data.prepared
      ? t("Prepare")
      : t("Play")
  const actionIcon: IconName = !files.installed ? "download" : !data.prepared ? "refresh" : "play"
  const percent = progress && progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
        flexGrow: 1,
        minHeight: narrow ? 560 : 600,
        paddingTop: narrow ? 18 : 36,
        paddingBottom: 10,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "column" : "row",
          alignItems: narrow ? "flex-start" : "center",
          gap: 24,
          flexGrow: 1,
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: 22, maxWidth: 720, flexGrow: 1 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
            <Text mono size="xs" style={{ color: CHROME_MUTED }}>
              {t("Minecraft Java Edition")}
            </Text>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                height: 24,
                paddingLeft: 10,
                paddingRight: 10,
                borderRadius: 999,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#FFFFFF1C",
                backgroundColor: "#FFFFFF0D",
              }}
            >
              <Text mono size="xs" style={{ color: BONE }}>
                {files.version}
              </Text>
            </div>
          </div>

          <Text
            style={{
              fontFamily: theme.font.sans,
              fontSize: narrow ? 44 : 68,
              lineHeight: narrow ? 46 : 70,
              fontWeight: "700",
              color: BONE,
              maxWidth: 760,
              lineClamp: 2,
            }}
          >
            {instance}
          </Text>

          <Text size="sm" style={{ maxWidth: 480, color: "#BDB5CC", lineHeight: 22 }}>
            {summary}
          </Text>

          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12, flexWrap: "wrap" }}>
            <Button
              loading={busy !== null}
              onClick={() => void action()}
              style={{
                height: 52,
                paddingLeft: 26,
                paddingRight: 26,
                borderRadius: 999,
                backgroundColor: theme.colors.primary,
                color: theme.colors.primaryForeground,
                hover: { backgroundColor: theme.colors.primaryHover },
              }}
            >
              <Icon name={actionIcon} size={16} color={theme.colors.primaryForeground} />
              <Text style={{ color: theme.colors.primaryForeground, fontWeight: "600", fontSize: 15 }}>
                {actionLabel}
              </Text>
            </Button>

            <Button
              variant="secondary"
              onClick={() => onPage("settings")}
              style={{
                height: 52,
                paddingLeft: 22,
                paddingRight: 22,
                borderRadius: 999,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#FFFFFF1F",
                backgroundColor: "#FFFFFF10",
                color: BONE,
                hover: { backgroundColor: "#FFFFFF1E" },
              }}
            >
              <Icon name="sliders" size={15} color={BONE} />
              <Text style={{ color: BONE }}>{t("Configure")}</Text>
            </Button>

            {ready && recent ? (
              <div
                role="button"
                tabIndex={0}
                onClick={() => playNow(recent.name)}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 7,
                  height: 36,
                  paddingLeft: 12,
                  paddingRight: 14,
                  borderRadius: 999,
                  cursor: "pointer",
                  hover: { backgroundColor: "#FFFFFF14" },
                }}
              >
                <Icon name="chevronRight" size={13} color={CHROME_MUTED} />
                <Text size="xs" style={{ color: CHROME_MUTED }}>
                  {`${t("Continue")}: ${recent.name}`}
                </Text>
              </div>
            ) : null}
          </div>

          {busy === "download" || busy === "prepare" ? (
            <div style={{ display: "flex", flexDirection: "column", gap: 8, width: narrow ? "100%" : 340 }}>
              <Meter value={progress ? progress.done : 1} total={progress ? progress.total : 1} />
              <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
                <Text mono size="xs" style={{ color: CHROME_MUTED }}>
                  {progress ? `${progress.done} / ${progress.total}` : t("Working…")}
                </Text>
                <Text mono size="xs" style={{ color: CHROME_MUTED }}>
                  {progress ? `${percent}%` : ""}
                </Text>
              </div>
            </div>
          ) : !files.installed && files.missingBytes > 0 ? (
            <Text mono size="xs" style={{ color: CHROME_MUTED }}>
              {`${megabytes(files.missingBytes)} ${t("to download")}`}
            </Text>
          ) : null}

          {note ? (
            <div
              style={{
                display: "flex",
                alignSelf: "flex-start",
                maxWidth: 560,
                paddingLeft: 12,
                paddingRight: 12,
                paddingTop: 7,
                paddingBottom: 7,
                borderRadius: 999,
                backgroundColor: "#E5484D1A",
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#E5484D33",
              }}
            >
              <Text size="xs" style={{ color: "#F5B5B8" }}>
                {note}
              </Text>
            </div>
          ) : null}
        </div>
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          gap: 18,
          paddingTop: 16,
          borderTopWidth: 1,
          borderStyle: "solid",
          borderColor: "#FFFFFF0F",
        }}
      >
        <Text mono size="xs" style={{ color: CHROME_MUTED }}>
          {`${t("Worlds")} · ${data.worlds.length}`}
        </Text>
        <div
          role="button"
          tabIndex={0}
          onClick={() => onPage("instances")}
          style={{
            display: "flex",
            alignItems: "center",
            height: 24,
            paddingLeft: 8,
            paddingRight: 8,
            borderRadius: 999,
            cursor: "pointer",
            hover: { backgroundColor: "#FFFFFF14" },
          }}
        >
          <Text mono size="xs" style={{ color: CHROME_MUTED }}>
            {`${t("Instances")} · ${data.instances.length}`}
          </Text>
        </div>
        <div style={{ flexGrow: 1 }} />
      </div>
    </div>
  )
}
