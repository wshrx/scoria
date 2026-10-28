import { useState, type ReactNode } from "react"
import { useWindowSize } from "@gpuix/react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import type { Snapshot } from "../api"
import { setOfflineName, setWhilePlaying } from "../api"
import { useT } from "../i18n"
import { Panel } from "../visual"

const MODES: { value: "stay" | "minimize" | "hide"; label: string }[] = [
  { value: "stay", label: "Stay" },
  { value: "minimize", label: "Minimize" },
  { value: "hide", label: "Hide" },
]

function Section({
  title,
  detail,
  children,
}: {
  title: string
  detail: string
  children?: ReactNode
}) {
  const theme = useTheme()
  const { width } = useWindowSize()
  const narrow = width < 900
  return (
    <div
      style={{
        display: "flex",
        flexDirection: narrow ? "column" : "row",
        gap: narrow ? 12 : 28,
        paddingTop: 22,
        paddingBottom: 22,
        borderTopWidth: 1,
        borderStyle: "solid",
        borderColor: theme.colors.border,
      }}
    >
      <div style={{ width: narrow ? "100%" : 220, flexShrink: 0, display: "flex", flexDirection: "column", gap: 5 }}>
        <Text size="base" weight="semibold">
          {title}
        </Text>
        <Text size="xs" tone="muted">
          {detail}
        </Text>
      </div>
      <div style={{ display: "flex", flexDirection: "column", gap: 12, flexGrow: 1, minWidth: 0 }}>
        {children}
      </div>
    </div>
  )
}

export function SettingsPage({
  data,
  dark,
  onToggleTheme,
  onRefresh,
  onLanguage,
}: {
  data: Snapshot
  dark: boolean
  onToggleTheme: () => void
  onRefresh: () => void
  onLanguage: (value: "en" | "ru" | "system") => void
}) {
  const theme = useTheme()
  const t = useT()
  const { width } = useWindowSize()
  const narrow = width < 900
  const [name, setName] = useState(data.offlineName)
  const [note, setNote] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 22 }}>
      <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
        <Text style={{ fontSize: 38, lineHeight: 42, fontWeight: "700" }}>{t("Settings")}</Text>
        <Text size="sm" tone="muted" style={{ maxWidth: 620 }}>
          {t("The launcher keeps everything on this machine. These are the only switches there are.")}
        </Text>
      </div>

      <div style={{ display: "flex", flexDirection: "column" }}>
        <Section title={t("Profile")} detail={t("The name the game shows in single player and on LAN.")}>
          <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 8, alignItems: narrow ? "stretch" : "flex-end" }}>
            <div style={{ width: narrow ? "100%" : 280 }}>
              <Input value={name} placeholder="Builder_1" onChange={(event) => setName(event.value ?? "")} />
            </div>
            <Button
              variant="secondary"
              loading={busy}
              onClick={() => {
                setBusy(true)
                void setOfflineName(name.trim())
                  .then(() => {
                    setNote(t("Saved. The next game starts with this name."))
                    onRefresh()
                  })
                  .catch((error: Error) => setNote(error.message))
                  .finally(() => setBusy(false))
              }}
            >
              {t("Save")}
            </Button>
            <div style={{ display: "flex", alignItems: "center", gap: 6, paddingBottom: 7 }}>
              <div
                style={{
                  width: 6,
                  height: 6,
                  borderRadius: 3,
                  backgroundColor: theme.colors.success,
                }}
              />
              <Text mono size="xs" tone="muted">
                {t("offline profile")}
              </Text>
            </div>
          </div>
          <Text size="xs" tone="muted">
            {t("3 to 16 letters, digits or _. Kept in accounts.json; no token leaves the machine.")}
          </Text>
          {note ? (
            <Text size="xs" tone="muted">
              {note}
            </Text>
          ) : null}
        </Section>

        <Section title={t("Appearance")} detail={t("Two surfaces, the same blue fire.")}>
          <div style={{ display: "flex", flexDirection: narrow ? "column" : "row", gap: 10 }}>
            {[
              { key: "dark", label: t("Basalt"), detail: t("Cold rock, bone text"), colors: ["#1A2130", "#6C9EFF", "#EEE9DD"] },
              { key: "light", label: t("Paper"), detail: t("Warm paper, ink"), colors: ["#F0ECE3", "#1F4FD8", "#191C22"] },
            ].map((option) => {
              const active = (option.key === "dark") === dark
              return (
                <div
                  key={option.key}
                  onClick={() => {
                    if (!active) onToggleTheme()
                  }}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: "center",
                    gap: 12,
                    width: narrow ? "100%" : 240,
                    padding: 12,
                    borderRadius: theme.radius.sm,
                    borderWidth: 1,
                    borderStyle: "solid",
                    borderColor: active ? theme.colors.primary : theme.colors.border,
                    backgroundColor: active ? theme.colors.accent : "transparent",
                    cursor: active ? "default" : "pointer",
                    hover: active ? {} : { borderColor: theme.colors.input },
                  }}
                >
                  <div
                    style={{
                      display: "flex",
                      flexDirection: "row",
                      width: 44,
                      height: 32,
                      borderRadius: theme.radius.xs,
                      overflow: "hidden",
                      borderWidth: 1,
                      borderStyle: "solid",
                      borderColor: "#00000033",
                    }}
                  >
                    {option.colors.map((color) => (
                      <div key={color} style={{ flexGrow: 1, backgroundColor: color }} />
                    ))}
                  </div>
                  <div style={{ display: "flex", flexDirection: "column", gap: 2, flexGrow: 1 }}>
                    <Text size="xs" weight="medium">
                      {option.label}
                    </Text>
                    <Text mono size="xs" tone="muted">
                      {option.detail}
                    </Text>
                  </div>
                  <Icon
                    name={active ? "check" : "circle"}
                    size={13}
                    color={active ? theme.colors.primary : theme.colors.mutedForeground}
                  />
                </div>
              )
            })}
          </div>
        </Section>

        <Section title={t("While playing")} detail={t("What this window does when the game starts.")}>
          <div style={{ display: "flex", flexDirection: "row", gap: 6 }}>
            {MODES.map((mode) => (
              <Button
                key={mode.value}
                size="sm"
                variant={data.whilePlaying === mode.value ? "secondary" : "ghost"}
                onClick={() => {
                  void setWhilePlaying(mode.value)
                    .then(() => onRefresh())
                    .catch((error: Error) => setNote(error.message))
                }}
              >
                {t(mode.label)}
              </Button>
            ))}
          </div>
        </Section>

        <Section title={t("Language")} detail={t("The launcher's own words. Game text comes from your packs.")}>
          <div style={{ display: "flex", flexDirection: "row", gap: 6 }}>
            {[
              ["en", "English"],
              ["ru", "Русский"],
              ["system", t("System")],
            ].map(([value, label]) => (
              <Button
                key={value}
                size="sm"
                variant={data.language === value ? "secondary" : "ghost"}
                onClick={() => onLanguage(value as "en" | "ru" | "system")}
              >
                {label}
              </Button>
            ))}
          </div>
        </Section>

        <Section title={t("This machine")} detail={t("Read-only facts about the build that is running.")}>
          <Panel style={{ padding: 0 }}>
            {[
              [t("Launcher"), `${data.version} · ${data.branch}`],
              [t("Minecraft"), data.minecraft.version],
              [t("Instance"), data.instances.find((i) => i.selected)?.name ?? data.selectedInstance],
              [t("Engine"), data.live ? t("connected") : t("preview snapshot")],
              [t("Appearance"), dark ? t("basalt") : t("paper")],
            ].map(([label, value], index) => (
              <div
                key={label}
                style={{
                  display: "flex",
                  flexDirection: "row",
                  alignItems: "center",
                  paddingTop: 10,
                  paddingBottom: 10,
                  paddingLeft: 14,
                  paddingRight: 14,
                  borderTopWidth: index === 0 ? 0 : 1,
                  borderStyle: "solid",
                  borderColor: theme.colors.border,
                }}
              >
                <Text size="xs" tone="muted" style={{ flexGrow: 1 }}>
                  {label}
                </Text>
                <Text mono size="xs">
                  {value}
                </Text>
              </div>
            ))}
          </Panel>
        </Section>
      </div>

      <Text size="xs" tone="muted">
        {t(
          "No telemetry, no consent screen, no analytics. The launcher talks to Mojang and GitHub only when you ask for files or a build.",
        )}
      </Text>
    </div>
  )
}
