# Android additions

The Android port uses Jetpack Compose / AndroidX under Apache-2.0 and the
repository's existing Rust components under their respective licenses.

TinyCC / libtcc is a separately loaded LGPL-2.1-or-later library. The build
pins upstream revision `43c7708b85681a2fd4451c8a541af4494a8919b2`:
https://github.com/TinyCC/tinycc/tree/43c7708b85681a2fd4451c8a541af4494a8919b2

Source ZIP SHA256:
`f660447b16d6e0efe27ba8a9122aa45ee950f84b574eeb8dd1e86bc8ad34c0ea`.

No upstream source changes are applied. The Android build generates `config.h`
and the embedded predefined-header representation, then uses the Android NDK.
Complete source, configuration, build instructions/script and the LGPL license
accompany the compiled library inside the APK's `assets/licenses/tinycc/`.
`libtcc.so` remains a separate native library and can be rebuilt/replaced by
rebuilding the APK; users may modify/debug these components.

The copyright notices and actual LGPL text are retained in the upstream source
and packaged `COPYING`. This component does not change the licenses of unrelated
repository code. No Nintendo/Square Enix game data is packaged.

Rayon / rayon-core (MIT OR Apache-2.0): reusable CPU workers for the extended zoom rows. Their original notices and those of their dependencies are preserved in THIRD_PARTY_NOTICES.md and packaged Android assets.
