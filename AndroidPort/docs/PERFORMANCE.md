# Estado de rendimiento Android

Base de trabajo: AndroidPort. El proyecto de PC y el instalador quedan intactos.
Sin commits. No se usa FFTARecomp - Copy.

## Prueba reproducible

RG Rotate, Android 12, ARM64, SoC T618, pantalla 720x720.
Replay completo desde frame 0 hasta 13100; zoom 6, salida 512x512.
Se conserva el arranque de batalla-1 y se anaden barridos rectos por los
cuatro extremos. Solo se mide como movimiento cuando cambia HOFS/VOFS:
610 frames de camara en movimiento a partir del frame 9700.
benchmark.sav es una copia privada; la partida original no se modifica.

Ejemplo PowerShell desde esta carpeta, ajustando las rutas locales:

    python scripts/ffta_android_camera_bench.py --adb C:/platform-tools/adb.exe --serial ROTATE01523130 --input E:/AI/ARC/gba-recomp/ffta-batalla-1.input --save E:/AI/ARC/ffta-respaldo.sav --out target/camera-test
    python scripts/ffta_android_bench_summary.py target/camera-test

Para comparar, pasa dos carpetas al script de resumen. Comprueba cuatro
hashes framebuffer/OAM y la trayectoria completa. Los tiempos CPU no son
FPS de presentacion. No comparar ejecuciones con simpleperf activo contra
mediciones sin perfilador. Separar cache nueva y preparada.

## Cambios de esta iteracion

- Buffers del PPU reutilizados por cada hilo de zoom; limpiar solo el ancho
  activo, preservando el borrado por fila y la transparencia.
- La rutina pump_audio retorna antes de extraer los mezcladores si todavia
  no corresponde generar ninguna muestra. No cambia el reloj de audio.
- El benchmark registra faltas de audio por frame, tambien durante la camara.
- Opcion Gradle -PfftaProfileSymbols para conservar simbolos nativos cuando
  la biblioteca Rust se compila con profile.release.strip=false.

Se probo una precarga de audio de tres frames. No demostro una mejora y se
retiro; no forma parte de la APK entregada.

## Validacion

MEDIDO: 90 pruebas gba-core pasadas, 0 fallidas, 3 ignoradas.
MEDIDO: prueba con ROM real, TinyCC del host y 600 frames sin extra:
framebuffer y OAM identicos al interprete original en cada frame.
MEDIDO: pruebas unitarias Kotlin y ensamblado Android correctos.
NO MEDIDO en esta iteracion: recuento independiente de los 8 personajes
por frame. No se modifican los productores, rangos ni la lista de personajes.

## Limites conocidos

El rendimiento no esta cerrado como 60 FPS fijos. La captura de pantalla
medida en la iteracion anterior rondaba 55-60 FPS, con caidas puntuales.
Persisten faltas de audio; no confundirlas con un problema resuelto.
No se ha medido la nueva APK en Snapdragon 855 durante esta iteracion.
No se altera la logica del juego para acelerar el renderizado.

Los registros de prueba quedan fuera de esta carpeta portable:
../target/android-validation/. No se incluyen ROM, saves ni replays en fuentes.
