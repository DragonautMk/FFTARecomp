# Project skills and change memory

The repository contains three local skills, routed by [AGENTS.md](../../AGENTS.md):

- [ffta-development](../../.cursor/skills/ffta-development/SKILL.md) identifies
  code entry points and implementation checks.
- [ffta-windows-validation](../../.cursor/skills/ffta-windows-validation/SKILL.md)
  guides builds, realistic validation and packaging.
- [ffta-project-memory](../../.cursor/skills/ffta-project-memory/SKILL.md)
  records implemented behavior, evidence and lasting knowledge.

Clients supporting `.cursor/skills` can discover them locally. Other agents
should read the paths in AGENTS.md directly. No global skill installation or
background service is required. These files guide agents that read project
instructions; they cannot guarantee that every editor or agent follows them.

## After a meaningful change

Update the relevant feature documentation, add a JSON record under
`docs/changes/`, then regenerate [CHANGELOG.md](../../CHANGELOG.md). Revise
[PROJECT_STATE.md](PROJECT_STATE.md) only for durable facts or decisions.
Use the real session date and only validation actually performed.

```sh
python3 scripts/project_memory.py record \
  --date 2026-10-20 --slug example-fix --title "Describe the resulting behavior" \
  --kind fixed --platform Windows \
  --change "Describe the implemented change and its purpose." \
  --validation "Name the checks actually performed and their result." \
  --limitation "Name relevant behavior or platforms still unverified." \
  --file crates/recomp/src/play.rs --reference docs/menu_shortcuts.md
python3 scripts/project_memory.py check
```

The example is a template, not a change to record verbatim. All repeatable flags
can be supplied multiple times. `--kind` accepts added, changed, fixed,
performance, or docs. The script uses Python 3.9+ and the standard library.
On Windows use `py -3` in place of `python3` where appropriate.

The record command never overwrites an existing entry. While finishing the same
uncommitted change, edit its JSON explicitly and run:

```sh
python3 scripts/project_memory.py render
python3 scripts/project_memory.py check
```

Preserve historical entries. For a subsequent change, create a new record.
Never edit the generated changelog directly. Records use schema 1 with fields
`date`, `slug`, `title`, `kind`, `platforms`, `changes`, `validation`,
`limitations`, `files`, and `references`, plus `schema`. Platforms and the five fields following it
are arrays of single-line strings; changes and files must be
nonempty. References are existing repository files. Files identify implementation
paths covered by the change, including deleted files. Record filenames are
`YYYY-MM-DD-slug.json`; output order is descending filename, not commit order.

## Automated checks

`check` validates the schema, references, generated changelog and documentation
coverage of changed implementation files. It compares the working tree,
including staged changes and untracked files, with HEAD by default. Only a new
or changed record can cover a changed implementation file; an old record alone
cannot satisfy a new change.

```sh
python3 scripts/project_memory.py check --base <base-commit>
python3 scripts/project_memory.py check --metadata-only
python3 -m unittest discover -s scripts/tests -p test_project_memory.py
```

Use `--base` for a branch/PR range. A clean working tree checked against HEAD
has no pending implementation coverage to assess. `--metadata-only` is for
validating a snapshot without Git history; it does not check change coverage.

The [CI workflow](../../.github/workflows/project-memory.yml) runs tests and
checks PR changes against their base commit, or push changes against the
previous commit. An initial push without an available base validates metadata
only. CI does not write files or publish changes. Enable the workflow as a
required status check in repository settings if merges must be blocked by it;
this work does not change remote branch protections.

Covered areas are `crates/`, `AndroidPort/`, `scripts/`, `TinyCC/`, `android/`,
`assets/`, `Installer/`, `tools/`, `.github/workflows/`, `.cursor/skills/`, plus root AGENTS.md and Cargo
manifests/build entry points listed in the script. Ordinary documentation-only
edits do not require their own record. Extend the path policy when adding a new
implementation area. The checker detects missing file coverage and stale output;
humans and agents must still assess whether descriptions and evidence are true.

If coverage fails, add the missing paths to an accurate record of that change
(or create one), render and check again. Do not list unrelated paths to silence
it. If a reference moved, update the record's reference or restore the document.
Never store game data, save contents, secrets or machine-specific absolute paths.
