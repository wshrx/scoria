---
name: Scoria QA
description: Test Scoria's launcher UI for missing translations, validation that is not inline, stale versions, broken download/remove states and visual artifacts. Use after UI changes, before a build, or when the user reports "not translated", "message in the wrong place", "visual bug".
---

# Scoria QA

Run from `launcher-ui/`. Do not edit code in this workflow unless the user asked for a fix.

## 1. Static gates

```sh
bun run typecheck
bun run check:ui      # no shadow, one Settings entry, 1280x720, no transparent window
bun run check:i18n    # every t("...") has a Russian line
```

`check:i18n` fails with the missing keys. A key used through `t(variable)` is not detected, so
also read the screen's label arrays (tabs, themes, languages, modes).

## 2. Screenshots

Build once, launch, capture each page:

```sh
sh tools/package_ui.sh
SCORIA_UI_PAGE=home ./dist/scoria-linux-x86_64/scoria-launcher-ui   # native Wayland
python3 .opencode/skills/scoria-qa/scripts/shot.py screen /tmp/opencode/qa.png
```

`shot.py` captures the screen through KWin's `ScreenShot2` and prints its size; the launcher window
sits in it with its compositor frame. Read the PNG with the read tool and check the list below.
(`active` also works, but only while the launcher is the focused window.) Start each page in a fresh process: `SCORIA_UI_PAGE`
is read once at mount (`home`, `mods`, `servers`, `settings`). `SCORIA_UI_SETTINGS_TAB` (`general`,
`game`, `builds`, `about`) opens a tab without a click; GPUIX ignores synthetic clicks, so this is
the only way to screenshot a sub-page. The launcher is Wayland-only: X11 `import`/`xwd` see nothing.

State fixtures for screenshots that normally need a click (`SCORIA_UI_QA`):

```sh
SCORIA_UI_QA=toast    ...   # success, info and error toasts (60 s)
SCORIA_UI_QA=dialog   ...   # destructive confirm dialog
SCORIA_UI_QA=skeleton ...   # Mods results and installed skeletons
```

They are inert unless the variable is set and live in `launcher-ui/src/qa.ts` / `QaFixtures`.

## 3. What to look for

- **Text**: no English on a Russian screen. Watch engine data: world modes, relative times, errors.
  Those are translated in the UI (`t(world.mode)`, `lib/time.ts`), not by the engine.
- **Versions**: `Launcher 0.1.0 · beta` (from `src/version.ts`) and `Engine v0.2.x` (from the
  snapshot) are separate rows. The launcher version must never show the engine version.
- **Validation**: the message sits under the field it belongs to, the field is red (`Input invalid`),
  the message is translated, and it clears as soon as the user types.
- **Progress**: the bar shows the file label and a percent; it must not push the results panel or
  jump when it appears or finishes.
- **Remove**: the button keeps its width between Remove and Confirm; the list updates without a
  reload and a short "Removed." note confirms it.
- **Chrome**: 1280x720, the compositor draws the frame and rounded corners, no black rectangles or
  shadows, one Settings entry, no X11/XWayland code.

## 4. Interaction states (manual)

Synthetic X11 clicks are ignored by GPUIX; use a real mouse or ask the user. Reproduce:
empty name + Save; install a mod and watch the bar; Remove twice (Confirm); switch theme on every
screen; resize to 1040x660.

Report findings as: screen, symptom, expected, file. Then fix only if asked.
