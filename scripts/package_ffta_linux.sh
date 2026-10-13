#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ $(uname -s) != Linux ]]; then
    printf 'This script requires Linux. On Windows, run build-all.ps1.\n' >&2
    exit 1
fi
cargo build --release -p installer -p recomp -p ffta-play
package=Installer/Linux
mkdir -p "$package/payload"
install -m 755 target/release/installer "$package/FFTAInstaller"
install -m 755 target/release/recomp "$package/payload/recomp"
install -m 755 target/release/ffta-play "$package/payload/FFTARecomp"
install -m 644 gamedb.sqlite "$package/payload/gamedb.sqlite"
printf '%s\n' 'Run ./FFTAInstaller and select your own FFTA USA ROM.' \
    'Requires libasound.so.2, libudev.so.1, and clang or gcc on PATH.' \
    'No ROM, save, compiled ROM library or assets folder is included.' > "$package/README.txt"
tar -C "$package" --exclude=payload/Licenses -czf "$package/FFTAInstaller-linux-x86_64.tar.gz" \
    FFTAInstaller payload README.txt
printf 'Created %s/FFTAInstaller-linux-x86_64.tar.gz\n' "$package"
