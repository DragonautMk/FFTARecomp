#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ $(uname -s) != Linux ]]; then
    printf 'This script requires Linux. On Windows, run build-all.ps1.\n' >&2
    exit 1
fi
cargo build --release -p installer -p recomp -p ffta-play
python3 TinyCC/build.py --platform linux
package=Installer/Linux
mkdir -p "$package/payload"
install -m 755 target/release/installer "$package/FFTAInstaller"
install -m 755 target/release/installer "$package/FFTAInstaller-SteamDeck"
install -m 755 target/release/recomp "$package/payload/recomp"
install -m 755 target/release/ffta-play "$package/payload/FFTARecomp"
install -m 644 gamedb.sqlite "$package/payload/gamedb.sqlite"
mkdir -p "$package/payload/tcc"
install -m 755 target/release/tcc/libtcc.so "$package/payload/tcc/libtcc.so"
cp -R TinyCC "$package/payload/"
mkdir -p "$package/payload/Licenses"
cp -R Licenses/. "$package/payload/Licenses/"
printf '%s\n' 'Run ./FFTAInstaller and select your own FFTA USA or Europe ROM.' \
    'Requires libasound.so.2 and libudev.so.1. TinyCC is bundled; no system compiler is needed.' \
    'No ROM, save, compiled ROM library or assets folder is included.' > "$package/README.txt"
tar -C "$package" --exclude=payload/toolchain -czf "$package/FFTAInstaller-linux-x86_64.tar.gz" \
    FFTAInstaller payload README.txt
printf 'Created %s/FFTAInstaller-linux-x86_64.tar.gz\n' "$package"

printf '%s\n' 'FFTARecomp v3.0 - Steam Deck installer' \
    'Run ./FFTAInstaller-SteamDeck. Fullscreen setup, designed for 1280x720.' \
    'Select your own FFTA USA or Europe ROM. Keep payload beside the installer.' \
    'After setup, add the installed FFTARecomp game executable to Steam.' > "$package/README-SteamDeck.txt"
tar -C "$package" --exclude=payload/toolchain -czf "$package/FFTAInstaller-SteamDeck-linux-x86_64.tar.gz" \
    FFTAInstaller-SteamDeck payload README-SteamDeck.txt
printf 'Created %s/FFTAInstaller-SteamDeck-linux-x86_64.tar.gz\n' "$package"
