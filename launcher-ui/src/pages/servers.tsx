import { useEffect, useState } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { addServer, play, removeServer, savedServers, type SavedServer } from "../api"
import { useT } from "../i18n"

/** The instance's servers.dat: join, add, remove. */
export function ServersPage() {
  const theme = useTheme()
  const t = useT()
  const [servers, setServers] = useState<SavedServer[]>([])
  const [adding, setAdding] = useState(false)
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [note, setNote] = useState<string | null>(null)
  const [confirming, setConfirming] = useState<number | null>(null)

  const refresh = () => {
    void savedServers()
      .then(setServers)
      .catch((error: Error) => setNote(error.message))
  }

  useEffect(refresh, [])

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
    <div style={{ display: "flex", flexDirection: "column", gap: 16, maxWidth: 820 }}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 10 }}>
        <Text size="sm" tone="muted">{t("Saved in this instance's servers.dat; offline-mode servers only.")}</Text>
        <div style={{ flexGrow: 1 }} />
        <Button variant="ghost" size="sm" onClick={() => setAdding((value) => !value)}>
          {adding ? t("Cancel") : t("Add server")}
        </Button>
      </div>

      {adding ? (
        <div style={{ ...panel, padding: 14, gap: 10 }}>
          <Text mono size="xs" tone="muted">{t("NEW SERVER")}</Text>
          <div style={{ display: "flex", flexDirection: "row", gap: 8 }}>
            <div style={{ width: 220 }}>
              <Input value={name} placeholder={t("Name")} onChange={(event) => setName(event.value ?? "")} />
            </div>
            <div style={{ width: 260 }}>
              <Input value={ip} placeholder="host:port" onChange={(event) => setIp(event.value ?? "")} />
            </div>
            <Button
              variant="secondary"
              onClick={() => {
                void addServer(name.trim(), ip.trim())
                  .then((list) => {
                    setServers(list)
                    setAdding(false)
                    setName("")
                    setIp("")
                    setNote(null)
                  })
                  .catch((error: Error) => setNote(error.message))
              }}
            >
              {t("Add")}
            </Button>
          </div>
          {note ? <Text size="xs" tone="muted">{note}</Text> : null}
        </div>
      ) : null}

      <div style={panel}>
        {servers.length === 0 ? (
          <div style={{ padding: 20 }}>
            <Text size="sm" tone="muted">{t("No servers yet. Add an offline-mode server, or open a world to LAN.")}</Text>
          </div>
        ) : (
          servers.map((server, index) => (
            <div
              key={`${server.ip}-${index}`}
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                gap: 12,
                paddingLeft: 16,
                paddingRight: 12,
                paddingTop: 13,
                paddingBottom: 13,
                borderTopWidth: index === 0 ? 0 : 1,
                borderStyle: "solid",
                borderColor: theme.colors.border,
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
                <Icon name="server" size={15} color={theme.colors.mutedForeground} />
              </div>
              <div style={{ display: "flex", flexDirection: "column", flexGrow: 1, minWidth: 0 }}>
                <Text weight="medium">{server.name || server.ip}</Text>
                <Text mono size="xs" tone="muted">{server.ip}</Text>
              </div>
              <Badge variant="secondary" dot>offline-mode</Badge>
              <Button
                size="sm"
                variant="secondary"
                onClick={() => {
                  setNote(null)
                  void play({ server: server.ip }).then(setNote)
                }}
              >
                {t("Join")}
              </Button>
              <Button
                size="sm"
                variant={confirming === index ? "destructive" : "ghost"}
                onClick={() => {
                  if (confirming !== index) {
                    setConfirming(index)
                    return
                  }
                  setConfirming(null)
                  void removeServer(index)
                    .then(setServers)
                    .catch((error: Error) => setNote(error.message))
                }}
              >
                {confirming === index ? t("Confirm") : t("Remove")}
              </Button>
            </div>
          ))
        )}
      </div>

      {note ? <Text size="xs" tone="muted">{note}</Text> : null}

      <div style={{ ...panel, padding: 16, gap: 6 }}>
        <Text mono size="xs" tone="muted">{t("ONLINE-MODE")}</Text>
        <Text size="xs" tone="muted">
          Scoria has no Microsoft session, so online-mode servers refuse it. Use an offline-mode
          server, Open to LAN, or Play with friends.
        </Text>
      </div>
    </div>
  )
}
