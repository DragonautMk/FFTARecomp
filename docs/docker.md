# Linux portable builds with Docker

From the repository root:

```bash
bash scripts/build_linux_docker.sh
```

The host only needs a working Docker Engine/Desktop with Buildx. Rust, GCC,
Python, pkg-config, ALSA and udev development packages are installed in the
build container. Network access is needed on the first build for the base image,
Debian packages and Cargo dependencies. The script explicitly builds
`linux/amd64`; other host architectures need Docker's amd64 emulation.

Output:

- `dist/docker-linux/FFTARecomp-linux-portable.tar.gz`
- `dist/docker-linux/BUILD-INFO.txt` (compiler, OS/libc, linked libraries and hash)

To choose a different export directory, pass it as the sole argument:

```bash
bash scripts/build_linux_docker.sh /tmp/ffta-release
```

Buildx output paths must not contain commas or newlines. The script can be
called from outside the checkout; it resolves the build context from its own
location. The build copies source into the container; it never writes into a
bind-mounted checkout or creates root-owned source/target files on the host.
BuildKit manages Cargo registry/git and target caches. Only the final archive
and build information are exported, using Docker's
[local artifact exporter](https://docs.docker.com/build/building/export/).

## Environment and compatibility

`Dockerfile.linux` selects Rust 1.94.0 on Debian Bookworm and installs its native
build dependencies. `Cargo.lock` is enforced for builds and tests. This avoids
linking releases against the developer host's Ubuntu 26.04 libc. The base image is
pinned by digest, including the Rust version and distribution. Debian package repositories can still
receive updates, so this is not a claim of bit-for-bit reproducible builds.
Check the exported build information and the actual ELF requirements when
changing the base image. The earlier direct Ubuntu build remains a separate
artifact at `dist/FFTARecomp-linux-portable.tar.gz`.

The default build compiles the portable runtime and bundled TinyCC, runs the
input/config and runtime test suites (optional local-ROM tests remain ignored),
and packages only after a successful build; failed tests prevent export.
`.dockerignore` allows only desktop source, assets, docs, licenses and packaging
inputs. ROMs, saves, savestates, recordings, personal `Data`, build outputs and
agent settings are excluded. Do not add a ROM to the build context or image.
TinyCC source and notices accompany the exported native library.

## Run the exported game

```bash
mkdir -p dist/docker-linux/game
tar -xzf dist/docker-linux/FFTARecomp-linux-portable.tar.gz -C dist/docker-linux/game
./dist/docker-linux/game/FFTARecomp-portable/FFTARecomp
```

This container is for **compiling and packaging**. The exported game runs on
the host desktop: its graphics stack, libc, ALSA and udev runtime libraries are
still required. Docker does not embed these libraries in the portable archive.
There is no GUI container, display/device passthrough, privileged mode or Docker
socket mount inside the build. Save data stays under the extracted folder's
`Data`, as described in [portable storage](portable.md).

## Cache maintenance

Subsequent builds reuse downloaded crates and compiled dependencies through
[BuildKit cache mounts](https://docs.docker.com/build/cache/optimize/).
The container's `target` cache is separate from the checkout's `target/`.
Manage Docker cache using your normal Docker workflow; the build script does
not prune images, containers, caches or unrelated projects.

## Validation

The container build/export completed with Rust 1.94.0 on Debian Bookworm
(glibc 2.36). 26 input/config and 41 runtime tests passed inside the container;
two optional ROM-fixture runtime cases remained ignored. ELF inspection found
requirements up to glibc 2.35 for the runtime, 2.34 for the wrapper and 2.34 for
TinyCC. This removes the direct Ubuntu build's 2.43 requirement; compatibility
with older Ubuntu/Steam Deck desktops has not been established by a GUI test.

The exported archive ran natively on the Ubuntu 26.04 host. An isolated
Xvfb/Mesa llvmpipe keyboard smoke saved slot 1 with its preview, closed normally,
moved the entire folder, relaunched and loaded the same slot. The translation
cache bytes/mtime were reused, startup backup copies numbered one then two,
and per-user config/cache/data files were unchanged. The ROM was preplaced in
the test folder. Physical controllers and native Wayland interaction were not
tested. No ROM or test data was copied into the container or exported archive.
