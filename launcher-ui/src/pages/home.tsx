import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import type { DownloadProgress, Snapshot } from "../api"
import { downloadMinecraft, play, prepare } from "../api"
import { useT } from "../i18n"
import { BLUE, BONE, Kicker, Meter, NIGHT, Panel, StrataThumb, Vista, gradient } from "../visual"
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
  const narrow = width < 1080
  const [busy, setBusy] = useState<"play" | "prepare" | null>(null)
  const [note, setNote] = useState<string | null>(null)
  const [log, setLog] = useState<string[]>([])
  const [progress, setProgress] = useState<DownloadProgress | null>(null)

  const files = data.minecraft
  const loot = files.client ? 1 : 0
  const total = 1 + files.libraries.total + files.assets.total
  const done = loot + files.libraries.done + files.assets.done
  const ready = files.installed && data.prepared

  const pushLog = (line: string) => setLog((lines) => [...lines.slice(-5), line])

  const fetchFiles = async () => {
    setProgress(null)
    const next = await downloadMinecraft(pushLog, setProgress)
    setProgress(null)
    onRefresh()
    return next
  }

  const prepareNow = async () => {
    setBusy("prepare")
    setNote(null)
    setLog([])
    try {
      if (!files.installed) await fetchFiles()
      const said = await prepare(undefined, pushLog, false)
      setNote(said)
      onRefresh()
    } catch (error) {
      setNote(`The download stopped: ${(error as Error).message}`)
    } finally {
      setBusy(null)
      setProgress(null)
    }
  }

  const playNow = (world?: string) => {
    setBusy("play")
    setNote(null)
    void play(world ? { world } : {}).then((said) => {
      setNote(said)
      setBusy(null)
    })
  }

  const status = ready
    ? t("Ready to play")
    : !files.installed
      ? t("Minecraft files are not on this machine yet")
      : t("Game files ready. Resources need preparing")

  const lastLog = log.at(-1)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 26 }}>
      <div
        style={{
          position: "relative",
          display: "flex",
          flexDirection: "column",
          height: narrow ? 470 : 384,
          flexShrink: 0,
          overflow: "hidden",
          borderRadius: theme.radius.lg,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#26304A",
          backgroundColor: NIGHT,
          boxShadow: theme.shadow.lg,
        }}
      >
        <Vista height={narrow ? 470 : 384} />

        <div
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            bottom: 0,
            width: narrow ? "100%" : 900,
            background: narrow
              ? gradient("#080A0EDD", "#080A0E99", 0)
              : gradient("#080A0E", "#080A0E00", 90),
          }}
        />

        <div
          style={{
            position: "relative",
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            flexGrow: 1,
            padding: narrow ? 28 : 42,
            paddingRight: narrow ? 28 : 380,
          }}
        >
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 12 }}>
            <Kicker>{t("Minecraft Java Edition")}</Kicker>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                height: 22,
                paddingLeft: 8,
                paddingRight: 8,
                borderRadius: theme.radius.xs,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#6C9EFF55",
                backgroundColor: "#6C9EFF14",
              }}
            >
              <Text mono size="xs" style={{ color: BLUE }}>
                {files.version}
              </Text>
            </div>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
            <Text
              style={{
                fontFamily: theme.font.sans,
                fontSize: narrow ? 34 : 48,
                lineHeight: narrow ? 38 : 52,
                fontWeight: "700",
                color: BONE,
                maxWidth: 620,
              }}
            >
              {t("Your worlds, lit from below.")}
            </Text>
            <Text
              size="sm"
              style={{ color: "#B7BECB", maxWidth: 470, fontSize: 13, lineHeight: 20 }}
            >
              {t(
                "Minecraft 26.3, prepared from your own files and run natively. No account, no session, nothing leaves this machine.",
              )}
            </Text>

            <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10, flexWrap: "wrap" }}>
              <Button
                loading={busy === "play"}
                onClick={() => playNow()}
                style={{
                  height: 42,
                  paddingLeft: 20,
                  paddingRight: 20,
                  borderRadius: theme.radius.sm,
                  backgroundColor: BLUE,
                  color: "#06101F",
                  hover: { backgroundColor: "#8AB4FF" },
                }}
              >
                <Icon name="play" size={14} color="#06101F" />
                <Text style={{ color: "#06101F", fontWeight: "600" }}>{t("Play")}</Text>
              </Button>
              <Button
                variant="outline"
                loading={busy === "prepare"}
                onClick={() => void prepareNow()}
                style={{
                  height: 42,
                  paddingLeft: 18,
                  paddingRight: 18,
                  borderRadius: theme.radius.sm,
                  borderColor: "#FFFFFF26",
                  color: BONE,
                  hover: { backgroundColor: "#FFFFFF12" },
                }}
              >
                <Icon name="download" size={14} color={BONE} />
                <Text style={{ color: BONE }}>
                  {files.installed ? t("Prepare resources") : t("Download game files")}
                </Text>
              </Button>
              <div style={{ display: "flex", alignItems: "center", gap: 6, marginLeft: 4 }}>
                <div
                  style={{
                    width: 6,
                    height: 6,
                    borderRadius: 3,
                    backgroundColor: ready ? theme.colors.success : "#D8B26A",
                  }}
                />
                <Text mono size="xs" style={{ color: ready ? "#A9C9B6" : "#D8B26A" }}>
                  {status}
                </Text>
              </div>
            </div>

            {busy === "prepare" ? (
              <div style={{ display: "flex", flexDirection: "column", gap: 8, maxWidth: 470 }}>
                <Meter
                  value={progress ? progress.done : done}
                  total={progress ? progress.total : total}
                  height={3}
                />
                <Text mono size="xs" style={{ color: "#8E97A8" }} truncate>
                  {progress
                    ? `${progress.done} / ${progress.total} files`
                    : lastLog ?? t("Preparing…")}
                </Text>
              </div>
            ) : lastLog ? (
              <Text mono size="xs" style={{ color: "#8E97A8" }} truncate>
                {lastLog}
              </Text>
            ) : null}
            {note ? (
              <Text size="xs" style={{ color: "#C7CBD3" }}>
                {note}
              </Text>
            ) : null}
          </div>
        </div>
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "stretch",
          borderTopWidth: 1,
          borderBottomWidth: 1,
          borderStyle: "solid",
          borderColor: theme.colors.border,
        }}
      >
        {[
          [t("Minecraft"), files.version],
          [t("Game files"), files.installed ? t("ready") : files.missingBytes > 0 ? megabytes(files.missingBytes) : t("not fetched")],
          [t("Worlds"), String(data.worlds.length)],
          [t("Instance"), data.instances.find((i) => i.selected)?.name ?? data.selectedInstance],
        ].map(([label, value], index) => (
          <div
            key={label}
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 3,
              flexGrow: 1,
              paddingTop: 14,
              paddingBottom: 14,
              paddingLeft: index === 0 ? 0 : 18,
              borderLeftWidth: index === 0 ? 0 : 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
            }}
          >
            <Text mono size="xs" tone="muted">
              {label}
            </Text>
            <Text size="base" weight="medium" truncate>
              {value}
            </Text>
          </div>
        ))}
      </div>

      <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 26, alignItems: "flex-start" }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 14, flexGrow: 1, minWidth: 0 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 10 }}>
            <Text size="lg" weight="semibold">
              {t("Worlds")}
            </Text>
            <Text mono size="xs" tone="muted">
              {data.worlds.length}
            </Text>
            <div style={{ flexGrow: 1 }} />
            <Text size="xs" tone="muted">
              {t("A world opens straight into the game")}
            </Text>
          </div>

          {data.worlds.length === 0 ? (
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 8,
                padding: 22,
                borderRadius: theme.radius.md,
                borderWidth: 1,
                borderStyle: "dashed",
                borderColor: theme.colors.border,
              }}
            >
              <Text size="sm">{t("No worlds yet.")}</Text>
              <Text size="xs" tone="muted">
                {t("Play once, create a world, and it will appear here.")}
              </Text>
            </div>
          ) : (
            <div style={{ display: "flex", flexDirection: "column", borderTopWidth: 1, borderStyle: "solid", borderColor: theme.colors.border }}>
              {data.worlds.map((world, index) => (
                <div
                  key={world.name}
                  onClick={() => playNow(world.name)}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: "center",
                    gap: 16,
                    paddingTop: 12,
                    paddingBottom: 12,
                    borderBottomWidth: 1,
                    borderStyle: "solid",
                    borderColor: theme.colors.border,
                    cursor: "pointer",
                    hover: { backgroundColor: theme.colors.accent },
                  }}
                >
                  <StrataThumb seed={index} width={92} height={56} />
                  <div style={{ display: "flex", flexDirection: "column", gap: 3, flexGrow: 1, flexBasis: 0, minWidth: 0 }}>
                    <Text size="base" weight="semibold" truncate>
                      {world.name}
                    </Text>
                    <Text size="xs" tone="muted" truncate>
                      {`${world.mode} · ${world.size} · ${world.lastPlayed}`}
                    </Text>
                  </div>
                  <div style={{ display: "flex", alignItems: "center", gap: 7 }}>
                    <Text mono size="xs" tone="muted">
                      {t("open")}
                    </Text>
                    <Icon name="arrowRight" size={14} color={theme.colors.mutedForeground} />
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        <Panel style={{ width: narrow ? "100%" : 382, flexShrink: 0, padding: 20, gap: 16 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
            <Text size="lg" weight="semibold" style={{ flexGrow: 1 }}>
              {t("This install")}
            </Text>
            <Text mono size="xs" tone="muted">
              {t("piston-meta")}
            </Text>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
            <Text size="md" weight="semibold" truncate>
              {data.instances.find((i) => i.selected)?.name ?? data.selectedInstance}
            </Text>
            <Text size="xs" tone="muted" truncate>
              {`${t("Minecraft")} ${files.version} · ${t(data.prepared ? "resources prepared" : "resources not prepared")}`}
            </Text>
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: 9 }}>
            {[
              [t("Client jar"), files.client ? t("ready") : t("missing"), files.client],
              [t("Libraries"), `${files.libraries.done} / ${files.libraries.total}`, files.libraries.total > 0 && files.libraries.done === files.libraries.total],
              [t("Assets"), `${files.assets.done} / ${files.assets.total}`, files.assets.total > 0 && files.assets.done === files.assets.total],
              [t("Resources"), data.prepared ? t("prepared") : t("not prepared"), data.prepared],
            ].map(([label, value, good]) => (
              <div key={String(label)} style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                <Icon
                  name={good ? "check" : "dot"}
                  size={12}
                  color={good ? theme.colors.success : theme.colors.mutedForeground}
                />
                <Text size="xs" style={{ flexGrow: 1, color: theme.colors.mutedForeground }}>
                  {label}
                </Text>
                <Text mono size="xs" style={{ color: good ? theme.colors.foreground : theme.colors.mutedForeground }}>
                  {value}
                </Text>
              </div>
            ))}
          </div>

          <div style={{ display: "flex", flexDirection: "column", gap: 7 }}>
            <Meter value={done} total={total} />
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text mono size="xs" tone="muted" truncate>
                {files.installed
                  ? t("all files verified")
                  : files.missingBytes > 0
                    ? `${megabytes(files.missingBytes)} ${t("to fetch")}`
                    : t("not fetched")}
              </Text>
              <Text mono size="xs" tone="muted" truncate>
                {`${Math.round((done / Math.max(total, 1)) * 100)}%`}
              </Text>
            </div>
          </div>

          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 7 }}>
            <Icon name="globe" size={12} color={theme.colors.mutedForeground} />
            <Text mono size="xs" tone="muted" truncate style={{ flexGrow: 1 }}>
              {t("Versions and files from piston-meta.mojang.com")}
            </Text>
          </div>

          <div style={{ display: "flex", flexDirection: "row", gap: 8 }}>
            <Button
              variant="secondary"
              size="sm"
              loading={busy === "prepare"}
              onClick={() => void prepareNow()}
              style={{ flexGrow: 1 }}
            >
              {files.installed ? t("Verify and prepare") : t("Download game files")}
            </Button>
            <Button variant="ghost" size="sm" onClick={() => onPage("files")} aria-label={t("Open game files")}>
              <Icon name="arrowUpRight" size={14} color={theme.colors.mutedForeground} />
            </Button>
          </div>

          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 8,
              paddingTop: 14,
              borderTopWidth: 1,
              borderStyle: "solid",
              borderColor: theme.colors.border,
            }}
          >
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text size="xs" tone="muted">
                {t("Profile")}
              </Text>
              <Text size="xs">{data.offlineName}</Text>
            </div>
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text size="xs" tone="muted">
                {t("Launcher")}
              </Text>
              <Text mono size="xs">
                {`${data.version} · ${data.branch}`}
              </Text>
            </div>
            <div style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
              <Text size="xs" tone="muted">
                {t("Account")}
              </Text>
              <Text size="xs">{t("offline only")}</Text>
            </div>
          </div>
        </Panel>
      </div>
    </div>
  )
}
