---
name: ffta-development
description: Develop FFTARecomp runtime, UI, input, rendering and recompiler features using the project's existing architecture and compatibility constraints.
---

# FFTARecomp development

Read `AGENTS.md` and `docs/development/PROJECT_STATE.md` from the repository
root. Inspect current code and Git state; historical docs include old build
flags and paths, so manifests and implementations resolve disagreements.

Choose the narrowest relevant area:
- `crates/recomp/src/play.rs`: desktop event loop, pacing and presentation.
- `crates/menu/`: menu navigation, capture state and drawing.
- `crates/input-config/`: persisted settings and backend-independent input.
- `crates/screen/`: presentation, UI layers and GPU/software equivalence.
- `crates/gba-core/`: guest execution, hardware, timing and save bytes.
- `crates/recomp-core/`: analysis, generated C, compiler backends and loaders.

For settings, connect storage/defaults, menu controls and runtime behavior.
For gamepad changes, inspect XInput, DirectInput and Gilrs/Linux paths;
capture must not also confirm a menu item or execute a game action.
Preserve individual game bindings when introducing a combo; distinguish
simultaneous chords from sequential presses and account for release state.

Keep guest timing and host presentation separate. FPS INT counts completed
emulated frames; FPS EXT counts successful submissions, not monitor scanouts.
Read the relevant feature doc before changing an existing hook or UI layer.

Save states use `crates/gba-core/src/save_state.rs`, separate from cartridge
saves and native translation caches. Keep decoding transactional; include new
mutable machine/hardware state in serialization and bump the state format
version for incompatible layouts or execution semantics. Never serialize host
pointers or ROM/BIOS bytes. Test deterministic replay and failed-load isolation.
Keep `.ffstate` files out of commits and packages. Slot persistence and preview
headers live in `crates/recomp/src/save_state.rs`; the controller-only picker
lives in `crates/menu/src/save_states.rs`. Preserve nine per-ROM generic paths,
legacy loading, atomic preview/state writes and safe overwrite confirmation.
Keep state-operation errors and success feedback inside the menu.

Use targeted behavioral checks. Shared display globals require serial core
tests. Use the Windows validation skill when building or packaging Windows.
Finish with the project-memory skill, recording only observed results and
preserving unresolved limitations.
