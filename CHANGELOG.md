# Changelog

Unreleased development changes. Generated from `docs/changes/*.json`.
Edit the structured records, then run `python3 scripts/project_memory.py render`.

## 2026-10-20 — Reduce desktop runtime diagnostic overhead

Type: performance. Platforms: Windows (Wine), Linux.

**Changes**

- Hoist tracing configuration out of native dispatch and filter unrelated diagnostic addresses early, preserving guest behavior and cache ABI (commit 0f46623).
- Correct the optional native-observer test loader for TinyCC relocatable output and add a repeatable Windows runtime benchmark.

**Validation**

- 190 Windows tests passed through Wine; 159 Linux core tests passed serially before and after.
- Framebuffers and OAM matched over 6,000 replay frames; three timed runs after 120 warmup frames gave median 16.043990 to 4.092949 ms/frame (74.49% reduction under Wine).

**Limitations**

- Wine environment lookup overhead can amplify the result; native Windows speed and AI-turn performance remain unmeasured.
- The baseline parallel core suite has failures caused by shared display globals.

Documentation: [windows_runtime_performance.md](docs/windows_runtime_performance.md).

[Change record](docs/changes/2026-10-20-windows-runtime-performance.json)

## 2026-10-20 — Save and load persistent machine checkpoints from the desktop menu

Type: added. Platforms: Windows (Wine), Linux core.

**Changes**

- Add Save states > Save state... / Load state... to the pause menu with file selection, error feedback and persistent .ffstate files.
- Capture CPU, hardware scheduling, memory, cartridge protocol/data, graphics and enhanced audio state in a versioned ROM/BIOS-bound format; validate and decode before changing the running machine.
- Write checkpoints atomically and protect cartridge .sav destinations; loading cancels pointer requests, resets audio/presentation history and refuses active input recording.
- Exclude private state files from Git, source packages and release audits; document cartridge-save behavior and future state-format versioning requirements.

**Validation**

- Windows GNU release tests through Wine: 163 core, 27 menu and 29 runtime tests passed; four snapshot tests also passed after extending banked CPU register coverage.
- Optional native replay passed with enhanced audio taps: 120 framebuffer hashes and complete final machine state matched after rewind and after loading into a fresh machine.
- Linux core: 163 tests passed serially. Interactive Windows/Wine smoke created and loaded a checkpoint, loaded again after restart, resumed and exited cleanly using isolated test data.
- Five memory-tool tests, updated skill validation and whitespace checks passed. The Windows archive passed CRC checks, executable hash comparisons and private-data/test-double exclusion checks.

**Limitations**

- Native Windows and physical-controller manual validation remain outstanding; AndroidPort is a separate tree and is unchanged.
- Linux desktop compilation needs the missing ALSA development package on this host.
- States require identical ROM/BIOS content and a compatible state version; they are not cartridge saves or another emulator's format.
- Loading rewinds cartridge memory; normal autosave or clean exit subsequently persists it to the currently selected .sav.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-states.json)

## 2026-10-20 — Undo the last successful save-state save or load from the controller menu

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Undo last operation to the slot picker, identifying the action and its slot independently of the currently selected slot.
- Keep one RAM-only undo entry per play session: restore exact previous slot bytes/preview or an empty slot after saving, and restore the complete pre-load machine after loading.
- Replace undo history only on successful saves/loads, consume it only on successful undo, preserve it across failures/menu reopen, and refuse detected external slot changes during undo save.
- Share audio/input/display restore handling between load and undo load; block undo load during input recording. Slot and machine formats and native cache ABI remain unchanged.

**Validation**

- Windows GNU release under Wine: 30 menu and 36 runtime tests passed, covering undo action routing, five-row UI geometry, empty/occupied/legacy slot restoration, machine restoration after further play, failed-operation retention, external changes, retry and no redo.
- Optional native replay passed: undoing a slot load restored the exact pre-load machine, and the next 120 framebuffer hashes and complete final state matched a reference, with enhanced audio taps enabled.
- Controller-only simulated-XInput smoke undid new-slot creation, replacement and loading; verified slot existence/hash changes, resumed and exited cleanly. Restart preserved the slot and cleared undo history.
- Five memory-tool tests, development skill validation and whitespace checks passed. Windows archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Undo has one level in RAM for the current session, with no redo or recovery history after restart/game switch. A new successful save or load replaces it.
- Undo load also discards gameplay since that load and restores cartridge memory; ordinary .sav autosave/exit behavior still applies.
- Native Windows and physical gamepad validation remain outstanding; testing used Wine and simulated XInput. AndroidPort is unchanged.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-state-undo.json)

## 2026-10-20 — Nine automatic save slots with previews and controller-only menu flow

Type: changed. Platforms: Windows (Wine), Desktop.

**Changes**

- Replace save/load file dialogs with nine numbered slots, wraparound selection, screenshot previews and in-menu success/error feedback.
- Store slot-1.ffstate through slot-9.ffstate under SaveStates/<ROM SHA-256>/ beside the selected cartridge save, isolating different ROMs and preserving slots across restarts.
- Commit the 240x160 game-view/native-UI preview and machine snapshot as one atomic file; require explicit in-menu replacement confirmation and protect newly occupied slots from unconfirmed writes.
- Keep loading compatibility for original raw .ffstate checkpoints copied into a numbered slot; retain the existing core state version and native cache ABI.

**Validation**

- Windows GNU release under Wine: 29 menu tests and 31 runtime tests passed, covering all nine paths, ROM isolation, fresh-machine loading, no-clobber/confirmed replacement, matching previews, damaged files and legacy states.
- Preview drawing and mouse hit geometry passed at 240x160, 640x480, 1280x720 and 3840x2160. Controller navigation tests passed for slot wraparound, empty loads and safe confirmation defaults.
- Interactive simulated-XInput smoke saved slot 9, displayed its preview, cancelled and confirmed replacement, loaded it, exercised empty-slot feedback and loaded slot 1 after restart; both sessions resumed and exited cleanly.
- Five memory-tool tests and development skill validation passed. Release archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Validation used Wine and simulated XInput, not physical controllers or native Windows. AndroidPort remains unchanged.
- Original files outside the numbered slot folder are preserved; importing one requires copying it into an unused slot. A legacy preview appears after resaving.
- Slot location follows the selected cartridge save directory. Loading also restores cartridge memory, subsequently persisted by normal autosave or exit.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-state-slots.json)

## 2026-10-20 — Add project skills and checked development memory

Type: docs. Platforms: Project-wide.

**Changes**

- Add project instructions and local skills for implementation, Windows validation and evidence-based change recording.
- Backfill performance, FPS and menu shortcut history; keep durable project knowledge separate from chronological records.
- Generate CHANGELOG.md from structured JSON records and check that changed implementation files have a new or updated record.
- Add a read-only CI workflow for change coverage and memory tooling tests; preserve existing uncommitted feature work.

**Validation**

- Five Python unittest cases passed in temporary Git repositories, including stale output, duplicate records, unsafe paths, invalid dates, missing references, commit ranges, deleted files and staged changes.
- All three local skills passed the skill-creator validator.

**Limitations**

- Agents must read AGENTS.md or discover the local skills; no global client settings or background services are installed.
- The checker verifies file coverage and metadata, not the truth of descriptions; change summaries and validation evidence require review.
- The CI workflow has been added locally but has not run on GitHub; remote branch protection settings are unchanged.

Documentation: [MEMORY_WORKFLOW.md](docs/development/MEMORY_WORKFLOW.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md), [AGENTS.md](AGENTS.md).

[Change record](docs/changes/2026-10-20-project-memory.json)

## 2026-10-20 — Add configurable keyboard and gamepad menu combinations

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Open menu binding capture under keyboard, mouse/keyboard and gamepad input pages; persist key.menu and pad.menu.
- Default to Escape and Start+Back/Select; preserve Escape recovery and Guide/Mode where available.
- Capture the largest simultaneous combination and consume a completed shortcut through partial release, across desktop controller backends.

**Validation**

- 78 Windows tests passed through Wine: 25 configuration, 26 menu and 27 runtime tests.
- Interactive Wine smoke with an isolated XInput test double verified default/custom gamepad shortcuts, partial release, keyboard capture and persistence.

**Limitations**

- Physical controllers and native Windows manual behavior remain unverified.
- An individual button retains its game action before the full combination is recognized.

Documentation: [menu_shortcuts.md](docs/menu_shortcuts.md).

[Change record](docs/changes/2026-10-20-menu-shortcuts.json)

## 2026-10-20 — Add configurable internal and external FPS overlay

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Video > FPS counter with Off, Internal, External and Both modes, persisted as video.fps in av.cfg (commit 401922d).
- Show a compact high-contrast HUD with guest frame and host presentation rates, independent of game zoom; reset sampling after pause or mode changes.

**Validation**

- 75 Windows tests passed through Wine across runtime, menu, configuration and GPU layer equivalence.
- Interactive Wine smoke verified menu selection, persistence, normal INT/EXT rates and different rates under fast forward.

**Limitations**

- External FPS counts successful host presentations, not physical display scanouts.
- Native Windows interactive behavior remains unverified.

Documentation: [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-20-fps-overlay.json)

## 2026-10-20 — Keep two automatic cartridge-save backups and restore them from the controller menu

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Capture the selected raw .sav before gameplay/autosave on desktop launch, loading exactly those captured bytes and retaining the two most recent complete startup copies.
- Store raw copies under SaveBackups/<save-filename>/ with sequence and SHA-256 filenames; publish a complete copy before pruning managed older files, preserving unrelated files and aborting startup on backup failure.
- Add controller-operated Save backups with latest/previous choices, safe Cancel-default confirmation and pinned-generation restoration. Validate content/size and prepare a clean boot before atomically replacing the current file and machine.
- Preserve both copies and all nine state slots during restore, reset the frontend and state undo, and prevent stale gameplay RAM from overwriting the recovered cartridge data. Missing remembered/default primary saves retain their recovery location when copies exist.
- Fit ten main-menu rows and distinguish main/video layouts with equal row counts; retain ordinary menu navigation and input capture behavior.

**Validation**

- Windows GNU release through Wine: 33 menu and 42 runtime tests passed, covering retention, exact raw data, safe confirmation, generation pinning, clean-boot restore, subsequent save, damaged/missing copies, failed publication, missing primary recovery and failure isolation.
- Controller-only simulated-XInput smoke exercised three starts, two-copy rotation, default cancellation, older-copy restore and clean exit without stale-RAM overwrite. Both remaining raw-copy hashes matched their encoded checksums.
- A separate final-build simulated-XInput smoke restored a missing primary from existing backups, retained the recovered file on exit and left both copies unchanged.
- Five memory-tool tests, development skill validation and whitespace checks passed. Archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Copies are the two most recent startup versions, so unchanged launches can yield identical copies; an absent primary does not create a fake backup.
- Restore replaces session progress and clears save-state undo. It is blocked during input recording. Hash checks validate backup storage integrity, not game-specific save checksums.
- Validation used Wine and simulated XInput, not native Windows or physical controllers. AndroidPort remains unchanged.

Documentation: [save_backups.md](docs/save_backups.md), [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-cartridge-save-backups.json)
