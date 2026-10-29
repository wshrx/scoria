import { Component, Fragment } from "react"
import type { ErrorInfo, ReactNode } from "react"
import { Icon, Text, useTheme } from "@/lib/flint"
import { Button } from "@/components/ui/button"
import { useT } from "../../i18n"

/** Last line of defence: a render error shows a panel instead of a blank window. */
export class ErrorBoundary extends Component<{ children?: ReactNode }, { error: Error | null; attempt: number }> {
  state = { error: null as Error | null, attempt: 0 }

  static getDerivedStateFromError(error: Error) {
    return { error }
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error("[scoria] render failed", error, info.componentStack)
  }

  retry = () => {
    this.setState((state) => ({ error: null, attempt: state.attempt + 1 }))
  }

  render() {
    if (this.state.error) {
      return <Failure error={this.state.error} onRetry={this.retry} />
    }
    return <Fragment key={this.state.attempt}>{this.props.children}</Fragment>
  }
}

function Failure({ error, onRetry }: { error: Error; onRetry: () => void }) {
  const theme = useTheme()
  const t = useT()
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: 12,
        width: "100%",
        height: "100%",
        padding: 32,
        backgroundColor: theme.colors.background,
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          width: 46,
          height: 46,
          borderRadius: 16,
          backgroundColor: theme.colors.muted,
        }}
      >
        <Icon name="alert" size={20} color={theme.colors.destructive} />
      </div>
      <Text size="lg" weight="semibold">
        {t("The launcher hit an unexpected error.")}
      </Text>
      <Text size="sm" tone="muted" style={{ maxWidth: 520, textAlign: "center", lineClamp: 3 }}>
        {error.message}
      </Text>
      <Button variant="secondary" onClick={onRetry} style={{ borderRadius: 999, height: 36, paddingLeft: 18, paddingRight: 18 }}>
        {t("Retry")}
      </Button>
      <Text mono size="xs" tone="muted">
        {t("The console has the full error.")}
      </Text>
    </div>
  )
}
