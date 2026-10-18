# Parche 1: bordes del terreno

Solo cambia la lectura del PPU; no altera ROM, VRAM ni instrucciones del juego.
`FFTA_PATCH_BG` está activo salvo valor `0`. SHA256 requerido:
`43FC8204C6DCEEE58828AEBC7AF0C72EB807E99F35AD641C8BB0A4FA8B6EDC19`.
Una ROM distinta mantiene el comportamiento anterior. No hay enganches de función.
Con extra=0 siempre se ejecuta la política original.

Identificación: modo de vídeo 0, pareja BG0/BG1 o BG2/BG3 activa,
BGxCNT exactos 4C03/4E02 y HOFS/VOFS coincidentes entre ambos fondos.
Esto exige mapas 64x32, caracteres en 06000000 y mapas en 06006000/06007000.
SUPOSICION: esa firma identifica semánticamente el terreno; no se ha validado
visualmente en todos los menús o escenas. Otros registros no coincidentes no cambian.

`FFTA_BG_EDGE=transparent` (predeterminado) deja transparente el exterior del
plano de tiles; `clamp` repite el píxel del borde del plano; `wrap` mantiene
el comportamiento anterior. El plano no equivale necesariamente a los límites
del mapa lógico: este parche no genera columnas que el juego no haya escrito.
El recorte de sprites queda pendiente del Parche 2.

## Comprobación visual pendiente

La salida PPM headless actual es de 240x160; no se usó para afirmar nada sobre
los márgenes. Ejecutar desde la raíz, en PowerShell, una vez por cada modo:

```powershell
$env:FFTA_OAM_LOG='0'
$env:FFTA_PATCH_BG='1'
$env:FFTA_WIDESCREEN='32:9'
$env:FFTA_BG_EDGE='wrap' # repetir con 'transparent' y 'clamp'
.\target\release\recomp.exe play 'Final Fantasy Tactics Advance (USA).gba' --status --gamedb gamedb.sqlite
```

Entrar en la misma batalla y mover la cámara a sus cuatro extremos. Capturar
manualmente la ventana con la herramienta de capturas de Windows, manteniendo
el mismo zoom en las tres ejecuciones. Lista de doce capturas:

| Modo | Izquierda | Derecha | Arriba | Abajo |
| --- | --- | --- | --- | --- |
| wrap | bg-wrap-izquierda.png | bg-wrap-derecha.png | bg-wrap-arriba.png | bg-wrap-abajo.png |
| transparent | bg-transparent-izquierda.png | bg-transparent-derecha.png | bg-transparent-arriba.png | bg-transparent-abajo.png |
| clamp | bg-clamp-izquierda.png | bg-clamp-derecha.png | bg-clamp-arriba.png | bg-clamp-abajo.png |

NO MEDIDO: desaparición visual del espejo en batalla. Transparent y clamp
evitan la lectura circular fuera del plano en la prueba sintética.

## Validación realizada

`cargo test -p gba-core --lib`: 76 passed; 0 failed.
Pruebas propias: extra=0 conserva los píxeles con las tres políticas; límites
izquierdo/derecho a 1536 px; exclusión de fondos no coincidentes y ROM no válida.
`cargo build -p recomp --release`: compilación correcta.
Dos ejecuciones nativas con `FFTA_PATCH_BG=0/1`, `FFTA_OAM_LOG=0`,
`FFTA_WIDESCREEN=0`, `FFTA_BG_EDGE=transparent`:

```powershell
.\target\release\recomp.exe runc 'Final Fantasy Tactics Advance (USA).gba' --frames 600
```

Ambas: `frames: 600 hash: 727e9157e9bcb0a6 native_blocks: 2116712 fallback_steps: 2`.
Es una prueba de arranque sin entrada de batalla; no demuestra corrección visual
en batalla. `runc` no configura widescreen desde la variable de entorno y usa
extra=0 por defecto en estos procesos.
