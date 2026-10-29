<p align="center">
  <img src="../../docs/media/logo.png" width="96" alt="Scoria">
</p>

<h1 align="center">Scoria</h1>

<p align="center">
  <b>Minecraft: Java Edition 26.3 на Rust.</b><br>
  Офлайн по умолчанию. Без аккаунта Microsoft. Без телеметрии.
</p>

<p align="center">
  <a href="#скачать">Скачать</a> ·
  <a href="#что-работает">Что работает</a> ·
  <a href="#сборка-из-исходников">Сборка</a> ·
  <a href="README.md">English</a>
</p>

## Скачать

| Платформа | Требования | Файл |
|---|---|---|
| **Linux x86_64** | Wayland, драйвер Vulkan | `scoria-launcher-linux-x86_64.tar.gz` |
| **macOS 14+** | Apple silicon (M1 и новее) | `scoria-launcher-macos-arm64.zip` |
| **Windows 10/11** | x64, драйвер Vulkan | `scoria-launcher-windows-x64.zip` |

Все сборки — **beta**: без подписи, без автообновления. Файлы игры готовятся из вашей копии
Minecraft 26.3 при первом запуске. SHA-256 каждого файла — на странице релиза.

## Что это

Нативный клиент и лаунчер для Minecraft: Java Edition 26.3. Читает ванильные миры, следует
ванильным правилам и запускается без сессии. Миры, настройки и моды остаются на вашей машине.

- **Без аккаунта.** Офлайн-профили, LAN и offline-mode серверы. Для online-mode нужна сессия
  Microsoft, которой у Scoria нет.
- **Ваши файлы.** Scoria использует официальный лаунчер, Prism, Freesm, MultiMC, Modrinth App или
  ATLauncher, если 26.3 уже установлена; иначе скачивает закреплённую версию с piston-meta Mojang.
- **Modrinth с метками.** Моды и ресурспаки показывают **Работает**, **Частично** или
  **Не запустится** до установки. Java-моды портируются; Java не запускается.
- **Нормальное приложение.** 1280×720, темы Night и Lilac, клавиатурная навигация, тосты и
  подтверждения. Linux — нативный Wayland; macOS и Windows — системная рамка.

## Что работает

| | |
|---|---|
| ✅ Одиночная игра, генерация, мобы, деревни, редстоун, команды, HUD | ❌ Online-mode серверы (нет сессии Microsoft) |
| ✅ LAN и offline-mode серверы | ❌ CurseForge |
| ✅ Моды и ресурспаки Modrinth с метками совместимости | ❌ Браузер модпаков в лаунчере |
| ✅ Бэкапы миров, скриншоты, настройки, ресурспаки | ❌ Шейдеры OptiFine/Java |
| ✅ Темы Night / Lilac, русский и английский | ❌ Windows ARM, Intel Mac |
| ✅ Linux (Wayland), macOS (Apple silicon), Windows (x64) | ❌ Подпись и нотаризация (beta) |

## Установка

**Linux**

```sh
tar -xzf scoria-launcher-linux-x86_64.tar.gz
./scoria-linux-x86_64/scoria-launcher-ui
```

**macOS** — распаковать, перенести `Scoria.app` в Applications, открыть. Beta не нотаризована:
первый раз — правый клик → «Открыть».

**Windows** — распаковать и запустить `Scoria.cmd`. SmartScreen предупредит, пока сборка не
подписана: «Подробнее» → «Выполнить в любом случае».

Первый запуск ничего не спрашивает. Нажмите **Скачать и подготовить** один раз — Scoria скачает
закреплённые файлы 26.3 с Mojang и подготовит ресурсы локально.

## Сборка из исходников

Rust 1.90+, Bun 1.4+, GPU с Vulkan.

```sh
cd launcher && cargo build --release
cd ../launcher-ui && bun install && bun run build
sh tools/package_ui.sh        # Linux
sh tools/package_macos.sh     # macOS .app
sh tools/package_windows.sh   # Windows zip
```

## Лицензия

MIT — см. [LICENSE](LICENSE) и [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Scoria — не официальный продукт Minecraft, не одобрена Mojang или Microsoft и не содержит код,
ассеты или данные Minecraft. Нужна собственная копия Minecraft: Java Edition 26.3.
