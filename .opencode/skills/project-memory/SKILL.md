---
name: Project memory
description: Keep PROJECT_STATE.md accurate and compact after a change so the next agent does not rediscover the project. Use after implementing, fixing, or deciding something, or when asked to record progress.
---

# Project memory

1. Read `PROJECT_STATE.md`, then `git status`, `git log --oneline -5`, and the diff.
2. Update only facts that changed: decisions, completed work, bugs with cause, gaps, branch/push state.
3. Use one short line per fact. No narrative, no duplicated code, no secrets, no token-heavy logs.
4. Keep the file under ~140 lines. Replace stale facts instead of appending a diary.
5. Do not change code or push. Report what you changed in one paragraph.
6. If a bug was fixed, record **symptom → cause → fix/check** so it is not reintroduced.
