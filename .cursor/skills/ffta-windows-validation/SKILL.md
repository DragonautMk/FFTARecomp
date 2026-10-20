---
name: ffta-windows-validation
description: Build, test, benchmark and package FFTARecomp for Windows, distinguishing native Windows results from Wine and simulated controller tests.
---

# Windows validation

Read `docs/development/PROJECT_STATE.md` and the feature's own documentation.
Use release builds for performance. Inspect installed targets, compiler and
libraries before assuming a toolchain exists; `target/` paths are local
artifacts, not repository prerequisites.

- Native build: `cargo build --locked --release -p recomp -p ffta-play`.
- Cross build: select `--target x86_64-pc-windows-gnu` and a compatible linker.
- Compile affected Windows tests, then run their executables on Windows or
  through Wine. Report which environment was actually used. Run core tests
  with `--test-threads=1` because display globals are shared.
- A GUI change needs a relevant menu/HUD check as well as compilation.
  Gamepad tests should identify the backend and whether input was simulated.

Use an isolated save, configuration and Wine prefix for interactive tests.
Wine may initialize APPDATA itself; inspect the effective path rather than
assuming an environment override isolated the config. Keep test controller
DLLs and screenshots under ignored `target/`, outside deliverables.

For performance comparisons, use the same translation, input, warm-up and
settings; separate hash capture from timing. Reuse
`scripts/benchmark_windows_runtime.py`. Wine timing is not native Windows
timing; introductory replay results are not an AI-turn benchmark.

TinyCC caches are relocatable ELF objects even on Windows. Use its loader,
not the OS DLL loader. Cache revision rules live beside `TRANSLATION_REV`.

Package an explicitly selected runtime, launcher, gamedb, TinyCC library,
source and notices. Verify archive integrity and executable hashes; exclude
ROMs, saves, recordings, generated game code and test doubles. Document
commands, environment, evidence and limitations with the project-memory skill.

For dependency-contained builds use `Dockerfile.windows` and
`scripts/build_docker.ps1` (Windows host) or `scripts/build_windows_docker.sh`
(Bash). The builder is Linux/MinGW and its tests run through Wine; do not call
these native Windows validation. Verify PE resources and imported DLLs, build
TinyCC's helper with the host compiler and its library with the target compiler,
and keep Windows target caches separate from Linux target caches.
