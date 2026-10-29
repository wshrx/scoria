<p align="center">
  <img src="logo.svg" width="88" alt="Scoria">
</p>

<h1 align="center">Scoria</h1>

<p align="center">
  <b>Лаунчер и клиент для Minecraft: Java Edition 26.3 на Rust.</b><br>
  Офлайн по умолчанию. Без аккаунта Microsoft. Без телеметрии. Без игровых ассетов.
</p>

<p align="center">
  <a href="#скачать">Скачать</a> ·
  <a href="#что-это">Что это</a> ·
  <a href="#ассеты-и-права">Ассеты</a> ·
  <a href="#что-работает">Что работает</a> ·
  <a href="#сборка-из-исходников">Сборка</a> ·
  <a href="README.md">English</a>
</p>

## Сборки

**Публичных beta-сборок пока нет.** Первые архивы для Linux, macOS и Windows появятся в
[Releases](https://github.com/wshrx/scoria/releases) вместе с бандлом игрового клиента и
`SHA256SUMS`. Пока репозиторий — только исходники: см. [BUILDING.md](BUILDING.md).

Один архив лаунчера играть не позволит: нужен бандл игровой версии. Это оставшаяся задача
релиз-инженерии (подписанные клиентские бандлы, без ассетов Minecraft внутри).

| Платформа | Статус |
|---|---|
| Linux x86_64 (Wayland, Vulkan) | первая beta, не опубликована |
| macOS 14+, Apple silicon | первая beta, не опубликована |
| Windows 10/11 x64 (Vulkan) | первая beta, не опубликована |

## Что это

Scoria — нативный лаунчер и клиент. Читает ванильные миры, следует ванильным правилам и
запускается без сессии. Миры, настройки и моды остаются на вашей машине.

**Scoria — независимый форк [Horizon](https://github.com/horizon-rust/horizon) (MIT).** Движок и
часть лаунчера взяты из Horizon; Scoria добавляет офлайн-лаунчер, загрузку с piston-meta,
поддержку Modrinth, новый интерфейс и брендинг. Это не официальный продукт Minecraft, он не
одобрен Mojang или Microsoft и не связан с ними.

- **Без аккаунта.** Офлайн-профили, LAN и offline-mode серверы. Для online-mode нужна сессия
  Microsoft, которой у Scoria нет.
- **Скачивает с Mojang.** При первой подготовке Scoria берёт закреплённые файлы 26.3 с
  piston-серверов Mojang в свою папку. Другие лаунчеры не сканируются и не меняются. Scoria не
  поставляет и не распространяет игровые данные.
- **Modrinth с метками.** Моды и ресурспаки показывают **Работает**, **Частично** или
  **Не запустится** до установки. Java-моды портируются; Java не запускается.
- **Нормальное приложение.** 1280×720, темы Night и Lilac, клавиатурная навигация, тосты и
  подтверждения. Linux — нативный Wayland; macOS и Windows — системная рамка.

## Ассеты и права

Граница простыми словами:

- В Scoria **нет кода, текстур, звуков, языковых файлов, данных и других ассетов Minecraft**.
- В релизных архивах только бинарники Scoria/Horizon, интерфейс, шрифты (SIL OFL) и знак Scoria.
- Файлы игры **скачиваются с серверов Mojang по запросу** или берутся из вашей установки.
  Хранятся локально и никуда не отправляются.
- Нужна собственная копия **Minecraft: Java Edition 26.3**.
- Скриншоты в README показывают **только интерфейс лаунчера**, не игровой контент.
- MIT-уведомление Horizon и все сторонние лицензии — в
  [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Если вы распространяете Scoria, сохраните MIT-уведомление и не добавляйте игровые ассеты в архив.

## Что работает

| | |
|---|---|
| ✅ Одиночная игра, генерация, мобы, деревни, редстоун, команды, HUD | ❌ Online-mode серверы (нет сессии Microsoft) |
| ✅ LAN и offline-mode серверы | ❌ CurseForge |
| ✅ Моды и ресурспаки Modrinth с метками совместимости | ❌ Браузер модпаков в лаунчере |
| ✅ Бэкапы миров, скриншоты, настройки, ресурспаки | ❌ Шейдеры OptiFine/Java |
| ✅ Темы Night / Lilac, русский и английский | ❌ Windows ARM, Intel Mac |
| ✅ Linux (Wayland), macOS (Apple silicon), Windows (x64) | ❌ Подпись и нотаризация (beta) |

## Скриншоты

Снимки интерфейса появятся с первой beta. Игрового контента здесь не будет: Scoria не поставляет
ассеты Minecraft.

## Установка (когда выйдет первая beta)

Linux: распаковать архив и запустить `scoria-launcher-ui`. macOS: распаковать, перенести
`Scoria.app` в Applications, один раз открыть через правый клик (не нотаризована). Windows:
распаковать и запустить `Scoria.cmd` (SmartScreen предупредит, пока не подписана). Первый запуск
ничего не спрашивает; нажмите **Скачать и подготовить** один раз — Scoria подготовит 26.3 из
ваших файлов или с серверов Mojang.

## Каналы релизов

Ветки `nightly` нет. Ветка — не канал распространения: релизы — это **теги** на `dev`, CI
собирает их и прикладывает к GitHub Releases. `v0.1.0-beta.N` — бета-канал. Если появится
nightly-канал, это будет плавающий pre-release тег, собранный из `dev` по расписанию, а не
отдельная ветка с другой историей.

## Сборка из исходников

Rust 1.90+, Bun 1.4+, GPU с Vulkan.

```sh
cd launcher && cargo build --release
cd ../launcher-ui && bun install && bun run build
sh tools/package_ui.sh        # Linux
sh tools/package_macos.sh     # macOS .app
sh tools/package_windows.sh   # Windows zip
```

## Благодарности

- Движок: [Horizon](https://github.com/horizon-rust/horizon), MIT.
- Шрифты: **Medeo Sans** и **Medeo Mono** от Medeo Project Authors (DNFDEVS), SIL OFL.
  Логотип Scoria построен из Medeo Sans Bold.
- Интерфейс: GPUIX и flint-ui.

## Лицензия

MIT — см. [LICENSE](LICENSE) и [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Scoria — не официальный продукт Minecraft, не одобрена Mojang или Microsoft и не содержит код,
ассеты или данные Minecraft. Нужна собственная копия Minecraft: Java Edition 26.3.
