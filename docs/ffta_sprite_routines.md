# FFTA USA: productores de OBJ

MEDIDO por lectura/desensamblado de la ROM local requerida; no se incluye ROM.
SHA256 y comprobaciones atómicas: `crates/gba-core/src/ffta_rom.rs`.
Ambas funciones son Thumb. Los hashes de los rangos completos incluyen sus
literales; las comprobaciones de instrucciones individuales son de dos bytes.

## ABI y entrada

`0x080014C8`: r0 lista, r1 origen X, r2 origen Y, r3 máscara de atributos.
En la pila de entrada: +0 desplazamiento de tile, +4 prioridad, +8 paleta,
+12 buffer OAM, +16 límite, +20 índice inicial. Retorna r0 = índice final;
si ya se alcanzó el límite al entrar, la salida inicial conserva r0 original.
`0x080016D4`: r0 lista, r1 X, r2 Y, r3 desplazamiento de tile; pila:
+0 prioridad (solo byte bajo), +4 paleta, +8 buffer, +12 límite, +16 índice.
Retorna r0 = índice final, incluso al salir inmediatamente por límite.

Lista: uint16 little endian de cantidad; después registros de seis bytes:
attr0 uint16 (Y firmado de 8 bits), attr1 uint16 (X firmado de 9 bits),
attr2 uint16. No hay relleno entre registros. La tabla de dimensiones se lee
en `0x08149510 + 4*(4*shape+size)`, dos uint16 de ancho/alto.

Primera función: máscara 0x30000000 selecciona reflexión del origen X/Y;
en ese caso X=origenX-offsetX-ancho, Y=origenY-offsetY-alto. De lo contrario
suma offsets. La función original reduce ambos resultados a int16 antes del
recorte. La segunda siempre suma y también reduce a int16.

## Comportamiento observado

The literal game disassembly was removed from the public source distribution.
Only independently described behavior, hook addresses and verification hashes are retained.

Recorte: descarta si el origen y el origen+dimensión quedan fuera del mismo
borde (X<0 o ambos X>239; Y<0 o ambos Y>159). No descarta solo por X>239
sin leer la dimensión. Se procesan hasta agotar descriptores o capacidad.
Solo escribe los primeros seis bytes de cada OBJ; conserva el cuarto halfword
que contiene las matrices affine. Atributos: tile suma módulo 1024, prioridad
suma módulo cuatro, paleta suma módulo 16; los demás bits se conservan.

Llamadores directos Thumb BL leídos en la región explorada:
14C8: 08001D80, 08001EA4, 08001FC0, 080020DC.
16D4: 08001DEE, 08001F0A, 08002026, 08002142.
NO MEDIDO: exhaustividad de llamadores indirectos o código en RAM.

## Enganche

Paso a: intérprete de instrucciones originales, una instrucción por despacho,
con el mismo cálculo de ciclos, bus e IRQ. No hay una copia binaria del juego.
La ruta nativa incluye un guard etiquetado en los bloques de estas funciones,
incluidas llamadas encadenadas; el despacho también protege bibliotecas previas.
La caché del producto usa el sufijo `-ffta-spr1` para reconstruir esos guardas.
`FFTA_PATCH_SPR=0` conserva la ejecución nativa anterior. Extra=0 no usa lista host.
El código propio permite efectuar la comprobación A/B sin modificar la ROM.
