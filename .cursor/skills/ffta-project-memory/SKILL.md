---
name: ffta-project-memory
description: Maintain FFTARecomp's changelog and durable project memory after meaningful implementation changes or when recording development progress is requested.
---

# Record project memory

Read `AGENTS.md`, `docs/development/PROJECT_STATE.md`, relevant existing
records and the current diff. Use files, Git and test output as evidence;
conversation recollection only helps locate it. Preserve unrelated work.

After a meaningful change:
1. Update the affected feature doc with behavior, configuration and validation.
2. Record what changed, why, platforms, actual validation and known limits in
   `docs/changes/`. Keep existing history intact; use one coherent record per
   change, not one per tool call. Do not create records for read-only answers.
3. Use `python3 scripts/project_memory.py record --help` for a new record.
   Supply the session's actual date and the implementation files it covers.
   Updating an existing record is appropriate while finishing that same
   uncommitted change; then run `python3 scripts/project_memory.py render`.
4. Update `PROJECT_STATE.md` only for durable decisions, entry points,
   constraints or verified outstanding work. Do not paste transient logs or
   keep a second chronological changelog there.
5. Run `python3 scripts/project_memory.py check`. For a branch or PR, also
   pass `--base <commit>`. Resolve missing coverage with an accurate record;
   do not change the checker just to accept undocumented implementation.

`CHANGELOG.md` is generated from records; edit the JSON source rather than
the generated Markdown. See `docs/development/MEMORY_WORKFLOW.md` for schema,
commands, CI behavior and recovery from failed checks.

Separate completed behavior from planned work. Never infer passing tests,
native Windows performance, physical-controller coverage, releases or commits
from compatible-looking code. Reference detailed tracked docs for durable
evidence; ignored local logs may be mentioned but are not portable fixtures.
Do not store copyrighted game data, save contents, secrets or machine-specific
absolute paths in memory records. Recording a change does not authorize a
commit, push, release, deployment or global agent configuration change.
