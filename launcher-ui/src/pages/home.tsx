import { useState } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "../api"
import { play, prepare } from "../api"
import { useT } from "../i18n"
import { gradient, HeroScene, Panel, Thumb } from "../visual"
import type { Page } from "../shell"

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
  const narrow = width < 1060
  const [note, setNote] = useState<string | null>(null)
  const [log, setLog] = useState<string | null>(null)
  const [busy, setBusy] = useState<"play" | "prepare" | null>(null)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 24 }}>
      <div
        style={{
          position: "relative",
          display: "flex",
          flexDirection: "column",
          height: narrow ? 470 : 340,
          flexShrink: 0,
          overflow: "hidden",
          borderRadius: theme.radius.xl,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: "#FFFFFF14",
          background: gradient("#341C14", "#0B0908", 150),
          boxShadow: theme.shadow.lg,
        }}
      >
        <HeroScene height={300} />

        <div
          style={{
            position: "relative",
            display: "flex",
            flexDirection: "column",
            flexGrow: 1,
            padding: 32,
          }}
        >
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
            <Icon name="sparkle" size={15} color={theme.colors.primary} />
            <Text mono size="xs" tone="muted">SCORIA</Text>
            <Badge variant="outline">{data.branch}</Badge>
            <div style={{ flexGrow: 1 }} />
            <Badge variant={data.live ? "success" : "secondary"} dot>
              {t(data.live ? "engine" : "preview")}
            </Badge>
          </div>

          <div style={{ flexGrow: 1 }} />

          <div
            style={{
              display: "flex",
              flexDirection: narrow ? "column" : "row",
              alignItems: narrow ? "stretch" : "flex-end",
              gap: narrow ? 18 : 32,
            }}
          >
            <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
              <Text mono size="xs" tone="muted">MINECRAFT 26.3</Text>
              <Text
                style={{
                  fontSize: 68,
                  lineHeight: 68,
                  fontWeight: "700",
                  color: theme.colors.foreground,
                }}
              >
                {data.version}
              </Text>
              <Text size="sm" tone="muted" style={{ marginTop: 8, maxWidth: 520 }}>
                {t("Your own Minecraft files, prepared locally. No account, no session, no telemetry.")}
              </Text>

              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10, marginTop: 22 }}>
                <Button
                  size="lg"
                  loading={busy === "play"}
                  style={{
                    height: 46,
                    paddingLeft: 22,
                    paddingRight: 22,
                    borderRadius: 13,
                    background: gradient("#FF7A45", "#C2410C", 150),
                    boxShadow: { offsetX: 0, offsetY: 12, blurRadius: 28, spreadRadius: -8, color: "#FF6B3577" },
                  }}
                  onClick={() => {
                    setBusy("play")
                    void play().then((said) => {
                      setNote(said)
                      setBusy(null)
                    })
                  }}
                >
                  <Icon name="play" size={15} color={theme.colors.primaryForeground} />
                  <Text style={{ color: theme.colors.primaryForeground, fontWeight: "600" }}>{t("Play")}</Text>
                </Button>
                <Button
                  size="lg"
                  variant="secondary"
                  loading={busy === "prepare"}
                  onClick={() => {
                    setBusy("prepare")
                    setLog(null)
                    void prepare(undefined, (line) => setLog(line)).then((said) => {
                      setNote(said)
                      setBusy(null)
                      onRefresh()
                    })
                  }}
                >
                  {t("Prepare")}
                </Button>
                <Badge variant={data.prepared ? "success" : "secondary"} dot>
                  {t(data.prepared ? "ready" : "needs preparing")}
                </Badge>
              </div>

              {busy === "prepare" && log ? (
                <Text mono size="xs" tone="muted" style={{ marginTop: 10 }}>{log}</Text>
              ) : null}
              {note ? <Text size="xs" tone="muted" style={{ marginTop: 10 }}>{note}</Text> : null}
            </div>

            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 10,
                width: narrow ? "100%" : 250,
                flexShrink: 0,
                padding: 18,
                borderRadius: theme.radius.lg,
                borderWidth: 1,
                borderStyle: "solid",
                borderColor: "#FFFFFF1A",
                background: gradient("#0D0A09D9", "#0D0A09F5", 160),
                boxShadow: theme.shadow.md,
              }}
            >
              <Text mono size="xs" tone="muted">{t("Playing as").toUpperCase()}</Text>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
                <div
                  style={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    width: 38,
                    height: 38,
                    borderRadius: 11,
                    background: gradient("#FF7A45", "#B33A12", 150),
                  }}
                >
                  <Text style={{ color: theme.colors.primaryForeground, fontWeight: "700" }}>
                    {data.offlineName.slice(0, 1).toUpperCase()}
                  </Text>
                </div>
                <div style={{ display: "flex", flexDirection: "column" }}>
                  <Text weight="semibold">{data.offlineName}</Text>
                  <Text mono size="xs" tone="muted">{t("offline").toUpperCase()}</Text>
                </div>
              </div>
              <div style={{ height: 1, backgroundColor: "#FFFFFF12" }} />
              {[
                [t("resources"), data.prepared ? t("prepared") : t("missing")],
                ["WORLDS", String(data.worlds.length)],
                ["INSTANCES", String(data.instances.length)],
              ].map(([key, value]) => (
                <div key={key} style={{ display: "flex", flexDirection: "row", justifyContent: "space-between" }}>
                  <Text mono size="xs" tone="muted">{key}</Text>
                  <Text mono size="xs">{value}</Text>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: narrow ? "column" : "row",
          gap: 20,
          alignItems: "flex-start",
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: 14, flexGrow: 1, minWidth: 0 }}>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 10 }}>
            <Text size="lg" weight="semibold">{t("Worlds")}</Text>
            <Text mono size="xs" tone="muted">{data.worlds.length} SAVES</Text>
          </div>
          <div style={{ display: "flex", flexDirection: "row", flexWrap: "wrap", gap: 14 }}>
            {data.worlds.length === 0 ? (
              <Text size="sm" tone="muted">{t("No worlds yet. Play and create one.")}</Text>
            ) : (
              data.worlds.map((world, index) => (
                <div
                  key={world.name}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: "center",
                    gap: 12,
                    width: narrow ? "100%" : 330,
                    padding: 10,
                    borderRadius: theme.radius.lg,
                    borderWidth: 1,
                    borderStyle: "solid",
                    borderColor: "#FFFFFF10",
                    background: gradient("#151110F5", "#0E0B0AF7", 160),
                    boxShadow: theme.shadow.sm,
                    cursor: "pointer",
                    hover: { borderColor: theme.colors.primary },
                  }}
                  onClick={() => {
                    setNote(null)
                    void play({ world: world.name }).then(setNote)
                  }}
                >
                  <Thumb seed={index} width={104} height={70} />
                  <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
                    <Text weight="semibold">{world.name}</Text>
                    <Text mono size="xs" tone="muted">{world.mode} · {world.size}</Text>
                    <Text mono size="xs" tone="muted">{world.lastPlayed}</Text>
                  </div>
                  <Icon name="play" size={14} color={theme.colors.primary} />
                </div>
              ))
            )}
          </div>
        </div>

        <Panel style={{ width: narrow ? "100%" : 280, padding: 18, gap: 10 }}>
          <Text mono size="xs" tone="muted">QUICK</Text>
          {[
            { page: "instances" as Page, label: t("Instances"), detail: `${data.instances.length}` },
            { page: "servers" as Page, label: t("Servers"), detail: "servers.dat" },
            { page: "settings" as Page, label: t("Settings"), detail: data.version },
          ].map((item) => (
            <div
              key={item.page}
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                gap: 10,
                paddingTop: 10,
                paddingBottom: 10,
                borderTopWidth: 1,
                borderStyle: "solid",
                borderColor: "#FFFFFF0D",
                cursor: "pointer",
                hover: { backgroundColor: theme.colors.secondaryHover },
              }}
              onClick={() => onPage(item.page)}
            >
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}>
                <Text size="sm">{item.label}</Text>
                <Text mono size="xs" tone="muted">{item.detail}</Text>
              </div>
              <Icon name="chevronRight" size={14} color={theme.colors.mutedForeground} />
            </div>
          ))}
        </Panel>
      </div>

      <Panel style={{ padding: 20, flexDirection: "row", alignItems: "center", gap: 26 }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 6, width: 260 }}>
          <Text mono size="xs" tone="muted">LOCAL BY DESIGN</Text>
          <Text size="sm" tone="muted">
            Your files, your worlds, your rules. Scoria never asks for an account and never sends
            telemetry.
          </Text>
        </div>
        <div style={{ display: "flex", flexDirection: "row", flexGrow: 1, gap: 20 }}>
          {[
            ["home", "Own installation", "Freesm, Prism or the official launcher"],
            ["server", "LAN and friends", "Open to LAN, invites and offline servers"],
            ["sliders", "Nothing hidden", "No account, no session, no collection"],
          ].map(([icon, title, detail]) => (
            <div key={title} style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10, flexGrow: 1 }}>
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  width: 36,
                  height: 36,
                  flexShrink: 0,
                  borderRadius: 11,
                  backgroundColor: theme.colors.accent,
                }}
              >
                <Icon name={icon as "home"} size={16} color={theme.colors.primary} />
              </div>
              <div style={{ display: "flex", flexDirection: "column" }}>
                <Text size="sm">{title}</Text>
                <Text mono size="xs" tone="muted">{detail}</Text>
              </div>
            </div>
          ))}
        </div>
      </Panel>
    </div>
  )
}
