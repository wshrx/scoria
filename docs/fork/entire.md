# Entire: что это и как мы это используем

Короткая справка по платформе, на которой теперь лежит `origin` Basalt. Проверено на
установленном CLI (`entire 0.12.2`, Linux) и на нашем репозитории.

## Коротко

- **Entire** — Git-форж и слой наблюдаемости для работы «человек + AI-агенты»:
  <https://entire.com>, CLI с открытым кодом (MIT, Go): <https://github.com/entireio/cli>.
- Репозитории адресуются как `/et/<project>/<repo>` (Entire-native) или
  `/gh/<owner>/<repo>` (зеркало GitHub). Наш путь — **`/et/redmoon/basalt`**.
- `git remote` указывает не на HTTPS, а на схему Entire:
  `entire://aws-eu-central-1.entire.io/et/redmoon/basalt`.
- У нас включены и хостинг, и **checkpoints**: `entire enable --agent opencode` (2026-10-28).
  `entire status` → `Enabled · branch main`, агент OpenCode, чекпоинты синхронизируются в
  `origin` (Entire), backend — git refs (`refs/entire/checkpoints/…`).

## Что умеет

1. **Хостинг и зеркала.** Свои репозитории на кластерах Entire (регионы, репликация) или
   зеркала GitHub-репозиториев. Push в зеркало всё равно запускает GitHub Actions.
2. **Trails.** Вместо «посмотри diff» — обзор изменения с eval'ами, которые агенты
   прогоняют на каждый push.
3. **Agent Sessions / checkpoints.** Каждый коммит, сделанный агентом, получает
   чекпоинт: промпты, транскрипт, вызванные инструменты, тронутые файлы, расход токенов.
   Чекпоинт привязан к коммиту трейлером `Entire-Checkpoint` и хранится отдельными
   ref'ами (`refs/entire/checkpoints/…`). **Новых коммитов в ветку Entire не добавляет.**
4. **Поиск и память.** `entire search`, `entire checkpoint explain <id>`, `entire recap`,
   плюс `entire-brain`/`entire-graph` (карта кода + накопленный контекст).
5. **Агенты.** Claude Code, Codex, Cursor, Copilot CLI, **OpenCode**, Antigravity, Droid, Pi
   и другие; интеграции ставятся хуками.

## Как включить захват сессий (пока не включено)

```sh
cd /home/shrx/dev/basalt
entire enable          # выбрать агентов; ставит их хуки и git-хуки Entire
entire status          # сессии и куда уходят чекпоинты
entire checkpoint list # чекпоинты текущей ветки
entire checkpoint explain <id>
entire session resume <branch>
entire disable         # выключить; --uninstall убирает Entire из репозитория
```

`entire enable` **сохраняет уже существующие хуки** (у нас `core.hooksPath = .githooks`,
`check_public_tree.py`), но это надо перепроверить после включения: публичный репозиторий
начнёт получать транскрипты сессий как ref'ы, а в них могут попасть секреты, пути и
внутренние обсуждения. Для публичного Basalt включать осознанно.

## Команды, которые мы уже использовали

```sh
entire repo list --project redmoon
#   /et/redmoon/basalt   aws-eu-central-1.entire.io   Private   ready

entire repo remote add origin --override /et/redmoon/basalt
#   ✓ Repointed remote "origin" at the mirror
#     entire://aws-eu-central-1.entire.io/et/redmoon/basalt

git push -u origin main
git push origin basalt-p0 basalt-p1 basalt-p2a upstream-baseline
```

`repo remote add` меняет **только локальный git config**; на сервере ничего не создаёт.
Кластер должен уже обслуживать репозиторий (`entire repo create` или `entire repo mirror add`).
`--override` перепривязывает занятое имя и печатает старый URL (единственная запись о нём —
сохраните, если он ещё нужен).

Другие полезные команды:

```sh
entire auth contexts          # сохранённые логины (у нас eu.auth.entire.io)
entire repo create basalt --project redmoon
entire repo clone /et/redmoon/basalt
entire repo view /et/redmoon/basalt
entire repo mirror add        # зеркало GitHub
entire trail ...              # Trails и eval'ы
```

## Совместимость: OpenCode 2.2.0 пока не работает

Проверено на `entire 0.12.2` + `opencode 2.2.0` (Linux, 2026-10-28):

- `entire session attach <ses_…>` падает: Entire вызывает `opencode export <id>`, а в OpenCode 2
  этой команды нет (теперь `opencode session export <id>`).
- Обход через shim (`export` → `session export`) создаёт чекпоинт и пушит ref, но OpenCode 2
  отдаёт другой JSON: `entire checkpoint explain` пишет `failed to parse transcript`,
  `no prompt recorded`, `checkpoint list` показывает 0.
- Живой захват через плагин `.opencode/plugins/entire.ts` тоже не подтверждён: после рестарта
  `entire session list` пуст. Вероятно, плагин рассчитан на API OpenCode 1.x.
- Тестовый чекпоинт `01M60WWCV09TC103XSDNVXXAJD` удалён (локально и на origin).

Итог: хостинг, remote и ветки работают; **чекпоинты с OpenCode 2.2 — нет**. Варианты:
дождаться поддержки в Entire, вести чекпоинтуемые сессии в Claude Code/Codex (их форматы
Entire понимает), или завести issue с версиями (`entire 0.12.2`, `opencode 2.2.0`).

## Безопасность и приватность

- Локальный захват работает и без аккаунта; аккаунт нужен для хостинга, зеркал и
  синхронизации чекпоинтов.
- Чекпоинты содержат **промпты, ответы, вызовы инструментов и содержимое файлов**. Для
  публичного репозитория это фактически публикация истории разработки; секреты и чужие
  данные туда попадать не должны.
- Remote-URL со схемой `entire://` обслуживает сам CLI (remote helper); обычный `git push`
  работает, пока CLI установлен и контекст жив.
- `entire disable --uninstall` убирает хуки и интеграцию; история чекпоинтов остаётся в ref'ах,
  пока её не удалить.

## Что это значит для Basalt

- `origin` = Entire (`/et/redmoon/basalt`), `upstream` = Horizon. Push идёт в Entire,
  GitHub-зеркало можно добавить позже через `entire repo mirror add`.
- Entire включён (`--agent opencode`), но чекпоинты с OpenCode 2.2 пока не работают (см. выше);
  хостинг и push в `origin` работают. Репозиторий на Entire **Private**.
- `.githooks/pre-commit` (наш `check_public_tree.py`) сохранён и по-прежнему запускается;
  Entire добавил `prepare-commit-msg`, `commit-msg`, `post-commit`, `post-rewrite` и обернул
  `pre-push` (`pre-push.pre-entire`). После `git config core.hooksPath .githooks` всё это живёт
  в `.githooks/`.
- Отключить захват: `entire disable` (или `--uninstall`); остановить только выгрузку сессий:
  `entire configure --local --skip-push-sessions`.
- Для CI: push в GitHub-зеркало по-прежнему триггерит GitHub Actions; прямые push в
  Entire-native репозиторий — нет.

## Источники

- <https://entire.com> · <https://docs.entire.com>
- <https://github.com/entireio/cli> (MIT, Go)
- `entire --help`, `entire repo --help`, `entire repo remote add --help` (0.12.2)
- Локальная проверка: `entire repo list --project redmoon`, `entire status`
