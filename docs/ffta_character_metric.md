# Metrica del tutorial

MEDIDO: replay nativo completo desde frame 0, entrada ffta-batalla-3.input,
save privado target/ffta-replay3/start.sav, sin ventana ni vsync.
Tramo analizado: 11340..12540, 1201 frames consecutivos.

MEDIDO: tabla de unidades 020223A4, paso 0x90. Cada registro contiene
ID en +0x64, instancia corporal en +0x5C e instancia auxiliar en +0x60.
Se agrupan las piezas por instancia corporal y luego por ID logico 0..7.
No se cuentan entradas OAM individuales. El cursor 0201C988 no pertenece
a esos registros corporales; las instancias auxiliares y los productores
de interfaz tampoco se cuentan. Las piezas publicadas conservan el propietario
y el grupo de llamada, para comprobar publicaciones duplicadas.

SUPOSICION pendiente de confirmar con el usuario: los ocho personajes de
referencia corresponden a IDs 0..7. Hay otro registro, ID 8, cuerpo 0201C698,
nombre Leslaie, categoria 3 frente a 0/2 de los otros registros. Se informa
por separado; no se oculta ni se considera un duplicado de otra unidad.
La semantica de esas categorias no se ha demostrado desensamblando su consumidor.

MEDIDO: present cuenta IDs publicados hacia el PPU; visible cuenta IDs con
pixeles corporales ganadores en la composicion final. Ambos se registran
por frame. Un cuerpo fuera de Y=0..159 puede estar publicado sin tener pixeles.
Con extra=0 se usa OAM etiquetada por el observador, sin activar la lista host.

MEDIDO: actual, FFTA_PATCH_SPR=1 y FFTA_PATCH_VIS=0, 32:9:
995 frames con present<8; 998 con visible<8. Rango present 2..8.
MEDIDO: propuesto, FFTA_PATCH_SPR=1 y FFTA_PATCH_VIS=1, 32:9:
493 frames con present<8; 512 con visible<8. Rango present 2..8.
El criterio de exito NO se cumple. Cero grupos duplicados observados.
Incluyendo el cuerpo adicional hay 9 cuerpos publicados en 204 frames
del actual y 702 del propuesto; no se afirma que la escena tenga solo 8 cuerpos.

MEDIDO: listas completas, una fila por frame con cantidad e IDs ausentes:
target/ffta-eight/current32-failures.csv y proposed32-failures.csv.
Las variantes visible-failures.csv contienen la prueba de pixeles.
Los archivos frames.csv incluyen todos los frames, no solo los fallos.
Se regeneran con python scripts/ffta_character_metric.py target/ffta-eight.
El analizador toma proposed32-corrected.log; el log provisional anterior
contenia un error de calculo de Y y queda excluido de los resultados.

MEDIDO: en frame 12539, predicado 030062EC, LR 0808F837:
x=288 y=-16 -> 0; x=256 y=-16 -> 0; x=176 y=-8 -> 0;
x=240 y=-40 -> 0; x=144 y=-8 -> 0; x=208 y=-40 -> 0.
Las seis instancias siguen en RAM. En 12540 solo se publican IDs 1 y 2.
El nuevo predicado amplia X, pero conserva el rechazo vertical original.
ROM Thumb 08A38F90 cmp r0,#0; 08A38F92 blt 08A38FA6;
08A38F94 cmp r1,#0x9f; 08A38FA0 bgt 08A38FA6.
El consumidor no llama al productor para los cuerpos rechazados.
NO MEDIDO: atribucion exhaustiva de cada uno de los 493 fallos a una unica
condicion; el desfase de publicacion requiere correlacion por instancia.
MEDIDO: el productor alternativo 030068A4 no se llamo en el tramo final
del recorrido anterior. No se generaliza a otras escenas.
MEDIDO: las seis instancias finales tienen 320 bytes de tiles examinados,
175..191 bytes no nulos y cero direcciones fuera de OBJ VRAM.
NO MEDIDO: integridad visual de todos sus tiles, o de unidades nunca producidas.

MEDIDO: extra=0 original contra propuesto: 1201 metricas identicas;
41 muestras de framebuffer, OAM y OBJ VRAM identicas, cero diferencias.
Prueba contra el predicado original de la ROM del usuario: 70 casos identicos.
No se distribuyen bytes largos de ROM; las comprobaciones usan hashes y
cabeceras cortas. No se modifica la ROM ni el save original.

Propuesta siguiente, NO IMPLEMENTADA: eliminar tambien el rechazo Y del
predicado en widescreen para conservar todos los cuerpos en el flujo host,
con las mismas comprobaciones de ROM y RAM. Requiere aprobacion porque cambia
la decision del juego. Esto no hace visibles cuerpos fuera de los 160 pixels
verticales; la prueba de ocho publicados y la de ocho visibles son distintas.
