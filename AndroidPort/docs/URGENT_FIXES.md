# Personajes y fast-forward Normal

## Personajes

Causa leida: la comprobacion de visibilidad 030062EC, ya usada por el
fix de PC, ampliaba X pero seguia rechazando Y fuera de 0..159.
Android necesita tambien el margen vertical extra_y del zoom.
Se amplia solo esa comprobacion hasta -extra_y .. 159+extra_y, manteniendo
las verificaciones SHA256 de ROM, hash de funcion y coincidencia ROM/RAM.
No cambia el resultado con extra_x=0 y extra_y=0.
FFTA_PATCH_ZOOM_SPR=0 permite comparar en el runner del host.
Los productores, atributos, prioridad y limites de OAM originales se conservan.

MEDIDO en el motor Android con TinyCC del host, replay desde frame 0:
frames 11900..13100, 1201 en total, salida 512x512, zoom 6.
Antes: 646 frames con menos de 8 personajes; minimo 2 publicados y dibujados.
Despues: 8 publicados y 8 dibujados en los 1201 frames; cero duplicados.
Los propietarios ausentes antes incluian posiciones Y=-48 e Y=240.

La metrica usa el roster medido de PC en 020223A4, registros de 0x90 bytes:
identificador en +64 hex, cuerpo en +5C y auxiliar en +60.
Agrupa todas las piezas del cuerpo por owner y cuenta ids 0..7 una sola vez.
Excluye auxiliares, cursor, efectos y el id 8. Para dibujado exige pixels
compuestos del cuerpo. Rechaza el resultado si la firma del roster no coincide.
La captura de pixels se activa solo durante el final del benchmark; no en juego
normal. Durante esa metrica se renderizan filas extra en serie para conservar
los contadores; sus tiempos no sirven para medir el rendimiento normal.

CSV de evidencia local, fuera de las fuentes portables:
../target/android-validation/characters-before.csv y characters-after.csv.
La lista completa de fallos anteriores esta en characters-before-failures.csv.

## Audio Normal con fast-forward

Causa leida: Android consumia un solo bloque de audio de 16.74 ms despues
de cada lote de simulacion. PC lo consume desde un hilo de salida independiente.
Cuando un lote tardaba mas que ese bloque, Android vaciaba su AudioTrack.

Se conserva el PCM a x1 y su cola en disco, pero un hilo de audio independiente
la consume a 48000 Hz, sin tomar el mutex del emulador. Los lotes del juego
no determinan cuantos ms de audio se entregan. Se usa solo en fast-forward
con Normal; audio acelerado, Silent y juego a velocidad normal conservan su ruta.

MEDIDO: prueba de drenaje de lotes completos, orden de muestras y pares estereo.
NO MEDIDO: escucha subjetiva del audio; requiere feedback del usuario.
La prueba debug --audio-check ejecuta 600 frames de arranque y despues 120
lotes x4. benchmark-normal-audio.csv recoge pares reproducidos y underruns.

## Comprobaciones

MEDIDO: 600 frames con ROM real y extra=0, framebuffer/OAM identicos.
MEDIDO: 90 pruebas gba-core, 9 pruebas Android nativas y 12 pruebas Kotlin pasadas.
APK: ../APK/FFTARecomp-android-arm64-preview.apk, version 0.16.
Sin commits, sin cambios en PC/instaladores, sin acceso a saves originales.

## Confirmacion en RG Rotate

MEDIDO ARM64: benchmark completo desde frame 0, 1201 frames finales
11900..13100, 8 ids distintos publicados y dibujados en todos; cero fallos
y cero duplicados. CSV: ../target/android-validation/urgent-device/benchmark-characters.csv.

MEDIDO audio Normal: 120 lotes con multiplicador configurado x4,
4.159 s de prueba, 197760 pares reproducidos y cero underruns del hilo
independiente. Salida configurada a 48000 Hz; el tiempo total incluye su
arranque. No se afirma que el T618 emule a x4 efectivo ni que la escucha
subjetiva este validada. CSV: ../target/android-validation/urgent-device-audio/benchmark-normal-audio.csv.

La prueba de personajes activa contadores; no usar sus tiempos para evaluar
rendimiento. El juego normal conserva el renderizado paralelo del zoom.

## Fragmento fijo en el cielo

MEDIDO: en el tutorial, 113 entradas OAM sobrantes (slots 15..127) usan
atributos 00A8,00F8,0000 y no corresponden a piezas publicadas por el host.
La pantalla GBA original no los muestra. El zoom vertical hacia arriba
expone su tile 0 en X=248,Y=-88; todas las entradas se superponen ahi.
Se observo el fragmento en la pantalla del dispositivo antes del arreglo.

El PPU omite solo ese patron exacto si no tiene coordenadas del host,
con el parche FFTA habilitado y extra_y>0. El gate FFTA ya exige la ROM
verificada. No se modifican OAM, VRAM, ROM ni coordenadas de personajes.
Una pieza legitima del host con esos mismos atributos sigue dibujandose.
MEDIDO: prueba especifica pasada; 91 pruebas gba-core pasadas.
La lista de entrada OAM queda en ../target/android-validation/sky-before.oam.csv.

MEDIDO visual en RG Rotate: el fragmento ya no aparece en la captura nueva.
En su rectangulo fijo (x=540..551,y=124..135 de pantalla 720x720),
107 pixels diferian del cielo de su misma fila antes, y 0 despues.
Las capturas tienen camaras distintas; esta comprobacion compara solo el
fragmento fijo con el cielo contiguo, no el framebuffer completo.
Capturas locales: ../target/android-validation/sky-game.png y sky-after-device.png.
MEDIDO: replay del host conserva 8 personajes publicados/dibujados durante
1201 frames; 600 frames sin extra conservan framebuffer y OAM identicos.
