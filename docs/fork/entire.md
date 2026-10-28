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
- У нас **включён только хостинг**. Захват сессий (`entire enable`) не запускался:
  `entire status` в репозитории отвечает `not set up`.

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
- Пока Entire — только удалённый хостинг; агентские сессии/чекпоинты не пишутся.
- Если включим `entire enable`, получим аудит того, как AI писал код, но обязаны сначала
  решить вопрос публичности транскриптов и проверить совместимость с `.githooks`.
- Для CI: push в GitHub-зеркало по-прежнему триггерит GitHub Actions; прямые push в
  Entire-native репозиторий — нет.

## Источники

- <https://entire.com> · <https://docs.entire.com>
- <https://github.com/entireio/cli> (MIT, Go)
- `entire --help`, `entire repo --help`, `entire repo remote add --help` (0.12.2)
- Локальная проверка: `entire repo list --project redmoon`, `entire status`
