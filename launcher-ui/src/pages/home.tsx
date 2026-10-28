import { useState } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { useT } from "../i18n"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import type { Snapshot } from "../api"
import { play, prepare } from "../api"
import { Terrain } from "../terrain"

/** Overview: one loud {t("version")}, one Play, then the worlds as a list. */
export function HomePage({ data, onRefresh }: { data: Snapshot; onRefresh: () => void }) {
  const theme = useTheme()
  const t = useT()
  const [note, setNote] = useState<string | null>(null)
  const [log, setLog] = useState<string | null>(null)
  const [busy, setBusy] = useState<"play" | "prepare" | null>(null)

  const panel = {
    display: "flex",
    flexDirection: "column" as const,
    backgroundColor: theme.colors.card,
    borderWidth: 1,
    borderStyle: "solid" as const,
    borderColor: theme.colors.border,
    borderRadius: theme.radius.sm,
  }

  return (
    <div style={{ display: "flex", flexDirection: "row", gap: 22, alignItems: "flex-start" }}>
      <div style={{ display: "flex", flexDirection: "column", gap: 22, flexGrow: 1, minWidth: 0 }}>
        <div style={{ ...panel, overflow: "hidden" }}>
          <div style={{ display: "flex", flexDirection: "row", gap: 24, padding: 28 }}>
            <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
              <Text mono size="xs" tone="muted">
                MINECRAFT 26.3 · {data.branch.toUpperCase()}
              </Text>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "flex-end", gap: 12, marginTop: 6 }}>
                <Text style={{ fontSize: 50, lineHeight: 52, fontWeight: "700" }}>{data.version}</Text>
                <Badge variant={data.prepared ? "success" : "secondary"} dot>
                  {t(data.prepared ? "ready" : "needs preparing")}
                </Badge>
              </div>
              <Text size="sm" tone="muted" style={{ marginTop: 10, maxWidth: 520 }}>
                {t("Your own Minecraft files, prepared locally. No account, no session, no telemetry.")}
              </Text>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8, marginTop: 22 }}>
                <Button
                  size="lg"
                  loading={busy === "play"}
                  onClick={() => {
                    setBusy("play")
                    void play().then((said) => {
                      setNote(said)
                      setBusy(null)
                    })
                  }}
                >
                  <Icon name="play" size={14} color={theme.colors.primaryForeground} />
                  <Text style={{ color: theme.colors.primaryForeground }}>{t("Play")}</Text>
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
                <div style={{ flexGrow: 1 }} />
                <Text mono size="xs" tone="muted">
                  {data.instances.length} INSTANCES
                </Text>
              </div>
              {busy === "prepare" && log ? (
                <Text mono size="xs" tone="muted" style={{ marginTop: 12 }}>{log}</Text>
              ) : null}
              {note ? (
                <Text size="xs" tone="muted" style={{ marginTop: 12 }}>{note}</Text>
              ) : null}
            </div>

            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "flex-end",
                gap: 8,
                width: 200,
                flexShrink: 0,
              }}
            >
              <Text mono size="xs" tone="muted">{t("Playing as").toUpperCase()}</Text>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 6 }}>
                <Badge variant="success" dot>offline</Badge>
                <Text size="md" weight="semibold">{data.offlineName}</Text>
              </div>
              <Text mono size="xs" tone="muted" style={{ marginTop: 18 }}>{t("PROFILE")}</Text>
              <Text size="xs" tone="muted">Local name and skin, kept in accounts.json.</Text>
              <Text mono size="xs" tone="muted" style={{ marginTop: 18 }}>{t("FILES")}</Text>
              <Text size="xs" tone="muted">Freesm / Prism / official install, detected automatically.</Text>
            </div>
          </div>
          <Terrain height={88} />
        </div>

        <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 10 }}>
          <Text size="lg" weight="semibold">{t("Worlds")}</Text>
          <Text mono size="xs" tone="muted">{data.worlds.length} SAVES</Text>
        </div>

        <div style={panel}>
          {data.worlds.length === 0 ? (
            <div style={{ padding: 20 }}>
              <Text size="sm" tone="muted">{t("No worlds yet. Play and create one.")}</Text>
            </div>
          ) : (
            data.worlds.map((world, index) => (
              <div
                key={world.name}
                style={{
                  display: "flex",
                  flexDirection: "row",
                  alignItems: "center",
                  gap: 14,
                  paddingLeft: 16,
                  paddingRight: 14,
                  paddingTop: 13,
                  paddingBottom: 13,
                  borderTopWidth: index === 0 ? 0 : 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                  cursor: "pointer",
                  hover: { backgroundColor: theme.colors.muted },
                }}
                onClick={() => {
                  setNote(null)
                  void play({ world: world.name }).then(setNote)
                }}
              >
                <div
                  style={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    width: 34,
                    height: 34,
                    flexShrink: 0,
                    backgroundColor: theme.colors.accent,
                  }}
                >
                  <Icon name="layers" size={15} color={theme.colors.mutedForeground} />
                </div>
                <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
                  <Text weight="medium">{world.name}</Text>
                  <Text mono size="xs" tone="muted">{world.mode} · {world.size}</Text>
                </div>
                <Text mono size="xs" tone="muted">{world.lastPlayed}</Text>
                <Icon name="play" size={13} color={theme.colors.primary} />
              </div>
            ))
          )}
        </div>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 16, width: 280, flexShrink: 0 }}>
        <div style={{ ...panel, padding: 16, gap: 10 }}>
          <Text mono size="xs" tone="muted">{t("PROFILE")}</Text>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                width: 44,
                height: 44,
                backgroundColor: theme.colors.primary,
              }}
            >
              <Text style={{ color: theme.colors.primaryForeground, fontSize: 18, fontWeight: "700" }}>
                {data.offlineName.slice(0, 1).toUpperCase()}
              </Text>
            </div>
            <div style={{ display: "flex", flexDirection: "column" }}>
              <Text weight="semibold">{data.offlineName}</Text>
              <Text mono size="xs" tone="muted">OFFLINE</Text>
            </div>
          </div>
          <Text size="xs" tone="muted">{t("No Microsoft account. Online-mode servers are out of scope.")}</Text>
        </div>

        <div style={{ ...panel, padding: 16, gap: 8 }}>
          <Text mono size="xs" tone="muted">{t("ENGINE")}</Text>
          {[
            [t("version"), data.version],
            [t("branch"), data.branch],
            [t("resources"), data.prepared ? t("prepared") : t("missing")],
            [t("mode"), data.live ? t("live") : t("preview")],
          ].map(([key, value]) => (
            <div key={key} style={{ display: "flex", flexDirection: "row", justifyContent: "space-between", gap: 8 }}>
              <Text size="xs" tone="muted">{key}</Text>
              <Text mono size="xs">{value}</Text>
            </div>
          ))}
        </div>

        <div style={{ ...panel, padding: 16, gap: 6 }}>
          <Text mono size="xs" tone="muted">{t("NO ACCOUNT")}</Text>
          <Text size="xs" tone="muted">
            {t("LAN, offline-mode servers and Play with friends work. The game never receives a session.")}
          </Text>
        </div>
      </div>
    </div>
  )
}
