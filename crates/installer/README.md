# FFTARecomp - instalador

Instalador de escritorio (Rust + egui) que toma **tu propia ROM** de Final Fantasy Tactics Advance
y la instala junto al reproductor y tus ajustes. Este repositorio **no contiene ninguna ROM, asset ni
archivo derivado del juego**,.



## Compilar (Windows)
Requisitos: Rust (rustup) y el toolchain MSVC.
Coloca tus propios `assets/logo.png` y `assets/icon.ico`, luego:

    cargo run --release

El instalador busca una carpeta `payload/` junto al `.exe` con `FFTARecomp.exe` y los ajustes
(widescreen, etc.); se copia entera a `%LOCALAPPDATA%\FFTARecomp`, junto con la ROM como `ffta.gba`.

## Aviso
Proyecto de hobby, sin afiliacion con Square Enix ni Nintendo. Final Fantasy Tactics Advance es
marca y propiedad de sus titulares. Usa solo una ROM de un cartucho que poseas.
