# Scoria — D · Signal + белая тема

Фокус: **D Signal** и его состояния, плюс **E White** — светлая тема без цветных
токенов. F Raw Grid, System, Mods и Cover удалены по правкам.

```
scoria-directions.figcraft  5 фреймов
exports/                    PNG 2× + contact-sheet.png
art/                        in-game captures + CREDITS.md
build.py                    генератор (FigCraft CLI)
mcp_audit.py                аудит через FigCraft MCP
tokens.json                 dark / light + button states
```

## Фреймы

| Фрейм | Что это |
|---|---|
| `D Signal` | Home, ready |
| `D Preparing` | Home, подготовка 42% |
| `D Playing` | Home, игра запущена |
| `D States` | 9 состояний кнопки + статусы, прогресс, тост |
| `E White` | светлая тема, без rail и без цветных токенов |

Раскладка в файле: D Signal `(0,0)`, D Preparing `(1280,0)`, D Playing
`(2560,0)`, D States `(0,800)`, E White `(1280,800)`.

## D · Signal

- 1280×720, поля 64 px, две колонки: 640 px контент + 40 px зазор + 472 px постер.
- Topbar 56 px: марка, версия, nav с Lucide-иконками, поиск, статус, системные
  кнопки.
- Play `192×52`, иконка 12, gap 12, focus ring 2 px / offset 4 px.
- Строки миров 64 px; миниатюра 64×48; выбранная строка с фоном и ember-полосой.
- Постер 472×560: in-game кадр, чип «выбранный мир», нижняя панель с названием,
  режимом, координатами и «Открыть мир».
- Состояния: Ready, Hover, Pressed, Focus, Preparing 42%, Playing, Download,
  Error / Retry, Disabled. Полные экраны — Preparing и Playing.

## E · White

- Светлая тема без цветных акцентов: белый shell, серый canvas, чёрный accent.
- **Rail удалён** — контент начинается с поля 48 px.
- Hero 640×320: белая карточка + in-game кадр правой половиной 320×320.
- Справа две карточки 520×152: состояние и 4 цифры.
- Полка: три карточки 376×168, зазор 28 px.
- Палитра: `#F1F1F1` canvas, `#FFFFFF` shell/surface, `#111214` ink/accent,
  `#6E7175` muted, `#E3E3E3` border.

## Сетка и иконки

| Параметр | Значение |
|---|---|
| Базовая сетка | 4 px |
| Поля D / E | 64 px / 48 px |
| Колонки D | 640 + 40 + 472 |
| Колонки E | 640 + 24 + 520 |
| Nav-иконки | 14 px, stroke 1.5, Lucide |
| Статусные иконки | 12 px |
| Primary Play | 12 px, fill |
| Системные кнопки | 14 px, stroke 1.4 |
| Icon + label | 8 px gap (12 px в кнопке), центр по ink-bbox |
| Радиусы D / E | 8/12/20 · 12/14/18/20/24 |

Иконки центрируются по **ink-bbox**, а не по номинальному 24×24: `icon_bbox.py`
рендерит каждый путь через rsvg и пишет `icon_bbox.json`, а `build.py` ставит
иконку так, чтобы центр её реального контура совпал с центром слота. Поэтому
треугольник Play и галочка не съезжают относительно текста.

Иконки: play, package, globe, settings, search, download, refresh, clock,
map-pin, hard-drive, activity, terminal, keyboard, help, check, alert, x, plus,
arrow-up-right, chevron, minus, square.

## Типографика

- **Display** — Medeo Sans Bold, 72 / 92% / −1%.
- **Title** — 40 / 100% / −0.5%; в E — Onest ExtraBold.
- **Body** — 14–15 / 150%; **caption** — 12 / 140%.
- **Mono** — Medeo Mono 10 / 140% / +4–6%.
- **Numbers** — табличные `tnum` для версий, дат, координат.
- Uppercase-моно только в eyebrow. Минимум 10 px в UI.

## Изображения

Все миры — **in-game captures** из тестовых миров Scoria/Horizon
(`~/Pictures/horizon-visuals-*.png`); `mc_art.py` вырезает область между
tutorial-плашкой и HUD, кропает и делает цветокоррекцию. Это не генерация и не
стоковые фото. Для публичного релиза заменяются на скриншоты игрока.
Кредиты — `art/CREDITS.md`.

## Токены

`tokens.json`:
- `themes.dark` — D Signal.
- `themes.light` — E White: только нейтрали, без цветных акцентов.
- `components.button_dark` / `button_light` — размеры, иконки, 8 состояний и
  focus ring.

Контраст D: fg 17.3:1, muted 6.6:1, accent-fg на accent 6.8:1.
Контраст E: ink на белом 19:1, muted 5.2:1, white на accent 19:1.

## MCP и скиллы

- **FigCraft MCP** зарегистрирован в `~/.config/opencode/opencode.json`
  (`figcraft-cli mcp`, headless) и использован для `open_file` /
  `inspect_document` — координаты 5 фреймов проверены.
- `python3 mcp_audit.py` — аудит файла через MCP.
- Установлены скиллы: `figcraft`, `figma-use`, `figma-generate-design`,
  `figma-design-to-code`, `figma-spec`, `figma-create-new-file`,
  `figma-code-connect`, `design-figma`.
- Figma-скиллы требуют Figma MCP server + URL/токен файла.

## Реализация в gpui-ce

1. **D** — topbar 56, две колонки через flex, строки миров `h(px(64))`, постер
   `flex_1` с `overflow_hidden`.
2. **E White** — один rounded shell (ADR-0003), без rail; hero 640 + 24 + 520,
   полка 3×376 + 2×28.
3. Кнопка — state machine: ready/hover/pressed/focus/preparing/playing/download/
   error/disabled; прогресс — 3 px линия внутри кнопки.
4. Проверить 1280×720 и 900×640; снять 18 состояний через
   `tools/capture_ui_states.py`.

## Что упрощено

- Servers/Settings не перерисовывались; D States покрывает кнопку, статусы,
  прогресс и тост.
- Миры — in-game captures из тестовых миров, не финальные скриншоты игрока.
- Числа (2 ч назад, 1 024 МБ, 0.6.9) демонстрационные.
