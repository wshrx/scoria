import { useState } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Snapshot } from "../api"
import { play, prepare } from "../api"

/** What a player sees first: the version, one Play, and their worlds. */
export function HomePage({ data }: { data: Snapshot }) {
  const theme = useTheme()
  const [note, setNote] = useState<string | null>(null)

  return (
    <>
      <Card>
        <CardHeader>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
            <CardTitle>Play {data.version}</CardTitle>
            <Badge variant={data.prepared ? "success" : "secondary"} dot>
              {data.prepared ? "Ready" : "Needs preparing"}
            </Badge>
          </div>
          <CardDescription>
            Minecraft: Java Edition 26.3, natively. Your own installation, no account.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
            <Button
              onClick={() => {
                void play().then(setNote)
              }}
            >
              <Icon name="chevronRight" size={14} color={theme.colors.primaryForeground} />
              Play
            </Button>
            <Button
              variant="secondary"
              onClick={() => {
                void prepare().then(setNote)
              }}
            >
              Prepare
            </Button>
            <div style={{ flexGrow: 1 }} />
            <Text size="sm" tone="muted">Playing as {data.offlineName}</Text>
          </div>
          {note ? (
            <Text size="xs" tone="muted" style={{ paddingTop: 10 }}>{note}</Text>
          ) : null}
        </CardContent>
      </Card>

      <div style={{ display: "flex", flexDirection: "row", alignItems: "baseline", gap: 8 }}>
        <Text size="lg" weight="semibold">Worlds</Text>
        <Text size="sm" tone="muted">{data.worlds.length} in this instance</Text>
      </div>

      <div style={{ display: "flex", flexDirection: "row", flexWrap: "wrap", gap: 12 }}>
        {data.worlds.map((world) => (
          <Card key={world.name} style={{ width: 300 }}>
            <CardHeader>
              <CardTitle>{world.name}</CardTitle>
              <CardDescription>{world.mode} · {world.size}</CardDescription>
            </CardHeader>
            <CardContent>
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center", gap: 8 }}>
                <Text size="xs" tone="muted">Played {world.lastPlayed}</Text>
                <div style={{ flexGrow: 1 }} />
                <Button variant="ghost" size="sm">Play</Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card variant="inset">
        <CardHeader>
          <CardTitle>No Microsoft, by design</CardTitle>
          <CardDescription>
            Offline profiles, LAN and Play with friends work out of the box. Online-mode servers need a
            real account and are not part of Scoria.
          </CardDescription>
        </CardHeader>
      </Card>
    </>
  )
}
