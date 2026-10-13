# FFTA: estado de investigacion del fallo real

Prioridad: reproducir ffta-batalla-3.input antes de afirmar una solucion.
La prueba previa de 44 objetos no reproduce el fallo comunicado por el usuario.
No se aplica otro cambio de logica del juego sin aprobacion.
Sin commits y sin cambios al instalador en esta investigacion.

MEDIDO: AGENTS.md leido y git status --short ejecutado; el repo ya contiene
cambios preparados, cambios sin preparar y archivos sin seguimiento.
MEDIDO: compilacion release inicial correcta, 0.42 s; compilacion de los
diagnosticos correcta, 26.86 s; tests de recomp: 6 aprobados.

MEDIDO: ffta-batalla-3.input termina con un evento en frame 12440.
Contiene eventos de soltar y pulsar en el mismo frame, por ejemplo 11730.
InputScript ordenaba tuplas (frame, mascara), alterando su orden de registro.
Se corrigio el lector para ordenar de forma estable solo por frame.
Esto afecta al replay del host, no a la logica ni a los bytes del juego.

MEDIDO: runc ejecuta la ruta nativa con el mismo Machine, MemMap, run_dma
y PPU que play, sin ventana, audio de escritorio, vsync ni espera por frame.
Se anadio --sav para precargar una copia en memoria; runc no escribe el .sav.
La entrada se reproduce desde frame 0, no desde el inicio del tramo analizado.

MEDIDO: creado E:/AI/ARC/ffta-respaldo.sav sin modificar el original.
La copia, el original y ffta-batalla-3.input.sav tienen SHA256:
71189F7FB6AED638640078FBA3A35FDA6C39C8962E74DCC75935AAC948DA9063.
El replay usa otra copia privada: target/ffta-replay3/start.sav.
SUPOSICION: arranque HLE como en las rutas BIOS buscadas localmente;
se pregunto al usuario si la grabacion utilizo BIOS real.

Comando exacto para el primer replay completo:

```powershell
$env:FFTA_WIDESCREEN='32:9'
$env:FFTA_PATCH_BG='1'
$env:FFTA_PATCH_SPR='1'
$env:FFTA_BG_EDGE='transparent'
$env:FFTA_WIDE_DEBUG=''
$env:FFTA_OAM_LOG='1'
$env:FFTA_OAM_LOG_FROM='10500'
$env:FFTA_OAM_CAPTURE_PREFIX='target/ffta-replay3/run1'
$env:FFTA_BENCH_LIBRARY='target/ffta-movement/native-normal.dll'
.\target\release\recomp.exe runc 'Final Fantasy Tactics Advance (USA).gba' --frames 12540 --input ffta-batalla-3.input --sav target/ffta-replay3/start.sav --out target/ffta-replay3/run1-final.ppm *> target/ffta-replay3/run1.log
```

El DLL indicado es una copia local ya compilada de la ROM del usuario;
no es un archivo que se deba distribuir. Desde frame 10500 el log registra
por frame camara BG, llamadas/argumentos a los tres productores conocidos,
descriptores recibidos, lista host y entradas OAM. Cada 30 frames guarda
hashes de ambos framebuffers, OAM y OBJ VRAM; cada 120 puede guardar una imagen.
MEDIDO: el replay llego a la batalla; se generaron imagenes del tramo final.
Dos ejecuciones completas desde frame 0 dieron 69 muestras identicas de
framebuffer central, framebuffer ancho, OAM y OBJ VRAM, frames 10500..12540.
Salida literal del script: determinism_samples 69 mismatches [].
Hash FNV final de ambas: abc92001e317eae0.
Hash SHA256 central final de ambas:
3ffb91edfe2e1bc047624f0b43c0c30b39770af3ba031d364a260f6b48f66bc2.
NO MEDIDO: confirmacion del usuario de que el arranque grabado era HLE;
la grabacion no guarda metadatos de BIOS y no se encontro BIOS instalada.

MEDIDO: hay nueve instancias graficas identificadas por el puntero
de prioridades pasado al productor menos 0x1e, no por palette ni r5.
En el tramo final desaparecen llamadas de algunas instancias antes de
030065A8; no hay mismatches en la lista host ni el OAM transferido.
Ejemplo: 0201af18 deja de llegar despues del frame 12042, junto al borde
izquierdo original. Falta terminar la medicion de los saltos y de sus tiles.

MEDIDO por lectura: 030062EC, Thumb, origen ROM 08A38EA0..08A38FC8,
calcula visibilidad de una lista con limites 0..239 en X y 0..159 en Y.
08394028 contiene su puntero Thumb 030062ED. El llamador 0808F804 la invoca
en 0808F832 por el thunk 08142254 y solo marca visible si retorna 1.
08021DA8 devuelve NULL si flags & 0x40 == 0; el llamador 0808F1F4 sale
en 0808F216 si obtiene NULL, antes de emitir objetos.
MEDIDO: igualdad de los 296 bytes de RAM con el origen ROM, hash:
1e988998a21067596046e2b303da1ad9b6924c18146f396ce0bf5f6e038433c7.
MEDIDO: en frame de productor 12041, X=-7, la funcion retorna 1 y se
invoca 030065A8 para 0201af18. En frame 12042, X=-10, sus seis componentes
son rechazados a la izquierda; retorna 0 y ya no se invoca el productor.
La traza lee instrucciones reales del interprete en la ruta nativa:

The literal game disassembly was removed from the public source distribution.
Only independently described behavior, hook addresses and verification hashes are retained.

En 0300638A, los valores r0 de los seis extremos derechos son
-2, -10, -2, -10, -2, -10. Todos estan por debajo de 0 pero dentro del
margen de 32:9, cuyo limite izquierdo es -164.
El llamador 0808F804 no marca el resultado de visibilidad cuando retorna 0.
MEDIDO: se pierde antes del productor y no por fallo de publicacion del host.

MEDIDO: los 320 bytes de OBJ VRAM referidos por esa instancia en su ultima
lista recibida permanecen identicos en frames de imagen 12041..12044:
8dac051f7fa7a16c7abd58bbaef877779ae968e1b88dbb2defc903151d6d84eb.
170 bytes no nulos, cero referencias fuera de OBJ VRAM. Por tanto, una
liberacion de estos tiles no explica esta desaparicion concreta.
NO MEDIDO: todos los assets de unidades nunca visibles ni todos los mapas.
El filtrado 03006170 es un ajuste de prioridad por intersecciones, no una
comparacion directa con 240; no se propone modificarlo por similitud.

MEDIDO: gamedb.sqlite no devolvio funciones de esta ROM en 0808E000..08090000;
estas direcciones se obtuvieron de llamadas reales y lectura de instrucciones.
Los desensamblados y logs completos quedan en target/ffta-replay3, ignorado
por git. No se incorporan bytes de ROM ni DLL generados a la distribucion.

MEDIDO por lectura: existe otro productor Thumb en 030068A4, procedente
de 08A39458, con comparaciones contra 239/159; no esta en el enganche actual.
MEDIDO: la observacion de 030068A4 no registro entradas en frames
10500..12539. La tabla del DLL tiene other=false para 030068A5: su entrada
habria pasado por el interprete observado. No participa en este tramo;
NO MEDIDO: su uso en otras escenas o animaciones.
MEDIDO, frames 11334..12539: 080014C8 desde LR 08001D85, 1377 llamadas;
080016D4 desde LR 08001DF3, 1206; ambas con limite 48 y dentro del enganche.
030065A8 desde LR 0808F313, 5645 llamadas, y LR 0808F6CD, 1204.
Los demas llamadores documentados de 14C8/16D4 no se observaron en ese tramo;
las dos funciones se enganchan por PC, independientemente del llamador.

MEDIDO: capturas exactas proof-12042.png y proof-12044.png muestran el
personaje desapareciendo en el margen izquierdo mientras el fondo sigue.
El periodo del productor se etiqueta con bus.frames antes de ejecutarse;
la captura se etiqueta al completarse el frame. La publicacion y el PPU
introducen el desfase visible entre el descarte de 12042 y la imagen 12044.
NO MEDIDO: correspondencia exclusiva de cada componente con una unidad
logica del juego; los identificadores son punteros de instancias graficas.

MEDIDO: tests finales normales: 80 + 7 + 3 aprobados, 0 fallos;
3 pruebas ignoradas por defecto. Prueba nueva del lector de entrada aprobada.
Los diagnosticos de instrucciones y tiles conservaron el mismo hash final
en los replays adicionales. No se modifico la ROM, el .sav original ni
la condicion del juego. La aplicacion de un parche de visibilidad esta pendiente.

Estado anterior, sustituido por la medicion de ocho unidades descrita abajo:
Propuesta minima de enganche por PC del predicado 030062EC,
activo bajo FFTA_PATCH_SPR y solo en widescreen. Conservar la ejecucion
original y sus ciclos; al retornar, usar el resultado calculado para el
rectangulo X=-extra..239+extra, manteniendo Y=0..159. Guardar argumentos
de entrada en el host para ese calculo y comprobar SHA256 de ROM y el
contenido completo del predicado en ROM/RAM antes de habilitarlo.
Extra=0, hash incorrecto o bytes diferentes: resultado original intacto.
Esto cambia el resultado que ve el juego y requiere aprobacion antes de
implementarlo. El productor y la lista i32 existentes seguirian evitando
el descarte posterior y el limite de X de 9 bits para los objetos ampliados.
Pruebas propuestas: los dos replays completos A/B, presencia de la instancia
12041..12046, tiles, OAM original, equivalencia 600 frames con extra=0 y
32:9/1536; revisar tambien el productor 030068A4 si hay llamadas reales.

NO MEDIDO: no se encontro un checkpoint completo utilizable para restaurar
CPU, RAM y perifericos. Las instantaneas IWRAM del recompilador son imagenes
de codigo y no sustituyen el estado acumulado. No se implemento un checkpoint.

Actualizacion: metrica por unidad del tutorial
MEDIDO: enganche X implementado bajo FFTA_PATCH_VIS y FFTA_PATCH_SPR,
comprobando SHA256 y funcion ROM/RAM. Resultado original con extra=0.
MEDIDO: 1201 frames finales: actual falla en 995; propuesto falla en 493.
NO RESUELTO: no hay ocho unidades publicadas en todos los frames.
MEDIDO: seis cuerpos finales siguen en RAM pero reciben rechazo Y antes
del productor. Hay tambien un cuerpo adicional Leslaie, informado separado.
SUPOSICION: referencia ocho corresponde a IDs 0..7, excluyendo ese cuerpo.
MEDIDO: extra=0 conserva las 1201 metricas y 41 muestras de hashes.
Detalles, limites y listas completas en docs/ffta_character_metric.md.
No se ha aplicado la propuesta adicional de quitar el rechazo Y.

Siguiente investigacion: casillas durante zoom-out
MEDIDO POR EL USUARIO: personajes validados jugando en 32:9, ya no
desaparecen. El parche de personajes se conserva sin modificaciones.
Los fallos anteriores de ocho publicados no anulan esa validacion visual;
la definicion del octavo personaje y la diferencia con visibilidad Y
siguen documentadas en ffta_character_metric.md.
MEDIDO: ffta-batalla-4.input y su sidecar no estan en la raiz del repo.
Busqueda rg --files bajo E:/AI/ARC, filtrada por batalla-4, sin resultados.
NO MEDIDO: replay 4, determinismo, lista de rango en RAM, frames del fallo,
condicion del escritor BG0/BG1 y capturas antes/despues. Falta la grabacion.
MEDIDO por lectura: InputScript v1 contiene frame y mascara KEYINPUT;
el grabador escribe solo cambios de keys. No graba zoom_level ni extra_y.
El zoom se procesa aparte en play.rs por Minus/Equal. El replay headless
configure_headless_widescreen solo fija extra X; no reproduce zoom-out.
MEDIDO por lectura: MAX_ZOOM=2, MAX_EXTRA_Y=320; play limita los pasos por
MAX_ZOOM y por ancho/alto maximos. No equivale a poder alcanzar extra_y=320
desde 32:9 con la configuracion actual. El maximo real de la captura falta.
Logging anadido bajo FFTA_OAM_LOG: FFTA_VIEW al cambiar dimensiones,
con frame, nivel de zoom, extra X/Y y dimensiones de vista/render.
No cambia el estado emulado ni el zoom, no toca sprites ni instalador.
No se aplica un parche BG nuevo sin medir antes la causa.

Actualizacion zoom y casillas, prueba manual pendiente
MEDIDO: runner runc admite FFTA_ZOOM_LEVEL=n o calendario frame:nivel.
32:9 niveles 0..6: extra X 164,199,238,282,331,387,448;
extra Y 0,10,21,34,47,63,80. Variable ausente conserva el runner anterior.
MEDIDO: replays completos de batalla-3 coinciden en 1201 hashes finales.
Se construyo movement-freeze.input conservando todos los eventos y anadiendo
12540 3ff. No se requiere otra grabacion del usuario.
MEDIDO: matriz calculada en 02005CB0, 64x64; BG0 copia una ventana 32x32.
En la muestra final, 108 tiles RAM frente a 102 VRAM; 1024/1024 palabras
VRAM coinciden con el recorte RAM de origen (33,35).
MEDIDO: escritor Thumb 0801AF08, llamador 0801B80C. Productor de casillas
0801D130: 20 llamadas style=1, caller 0801D803, coordenadas logicas X/Y
y proyeccion en tiles, cuatro cuadrantes por casilla. La tabla de flags
02008190 contiene marcas parciales de vecinos; contar bytes no cuenta casillas.
MEDIDO: PPU solo escribia 160 filas. En zoom 6 las 160 filas adicionales
quedaban negras; capturas before-11347.png y frozen-13101.ppm.
IMPLEMENTADO PARA PRUEBA: FFTA_PATCH_MOVEMENT=1, requiere PATCH_BG y ROM
exacta, SHA256 de 0801AF08..0801B5D0 y coincidencia de los 1024 valores
de cada plano VRAM con su recorte RAM. Si no coincide usa la ruta anterior.
El host lee los planos RAM completos y termina las filas extra en batalla.
No cambia escrituras originales del juego, sus ciclos, el enganche de
personajes ni la politica de borde del terreno. Por defecto esta desactivado.
MEDIDO: extra=0, 600 frames, MOVEMENT=0/1: framebuffer y OAM identicos.
NO MEDIDO TODAVIA: exito visual del parche nuevo, metrica por casilla antes
y despues, y regresion de los 1201 frames con el parche nuevo activo.
Prueba manual preparada en target/ffta-zoom/manual/FFTA-movement-test.gba;
su save privado procede de ffta-respaldo.sav. No se toco ningun save original.
Actualizacion: el usuario confirma que las casillas ya no desaparecen.
MOVEMENT esta activo por defecto, con las mismas verificaciones de ROM y RAM.
MEDIDO: el escenario conserva planos RAM 64x64 en 0200B1A0 y 020091A0;
VRAM contiene recortes 64x32. El parche STAGE lee esos planos completos solo
tras verificar el recorte entero (2048 palabras por capa). No modifica el juego.
MEDIDO: comparacion STAGE=0/1: 1861 frames de personajes sin cambios,
incluidos los 1201 finales de la grabacion; 63 muestras de framebuffer central
y OAM sin cambios. Frame final: 8 personajes, sin grupos duplicados.
MEDIDO: extra=0, comparacion por frame durante 600 frames: framebuffer y OAM
identicos. Capturas stage-off-13101.png y stage-on-13101.png muestran mas tejado
y arquitectura lateral. La confirmacion en juego real sigue NO MEDIDO.
MEDIDO: las filas adicionales de zoom ahora se dibujan, no quedan vacias.
Input contiene Keyboard y Gamepad, remapeo y layout Xbox por defecto.
NO MEDIDO: mando real; no se prueba por instruccion del usuario.
Instalador Linux: codigo, script y workflow preparados; ejecucion NO MEDIDO.
La carpeta assets tiene referencias de compilacion y no se elimina.

Correccion del cielo durante zoom:
MEDIDO: replay frame 13100, palette0 cambia por linea: 7D21,7DA4,7E67,
7F2A,7FCC en lineas 0,40,80,120,159. El gradiente es raster de paleta,
no una capa affine o bitmap como se afirmo antes sin medir.
Las filas host adicionales leian la paleta de VBlank (azul inicial).
Ahora conservan los colores de los extremos antes de VBlank y prolongan
esos colores fuera de las 160 lineas. No se copian filas del escenario.
Actualizacion: prolongar un color fijo seguia dejando una banda lisa.
El host ahora extrapola por canal RGB555 el gradiente observado entre
las lineas 0 y 159; los pixeles originales de 240x160 no cambian.
MEDIDO: en frame 13101, filas 0/79/80/239/240/319 cambian RGB
de 0,0,255 a 8,74,255 a 99,247,255 a 140,255,255 sin salto en
los bordes de la pantalla original. Captura sky-extended-13101.png.
MEDIDO: 600 frames extra=0 framebuffer y OAM iguales; 1201 frames de
personajes del tutorial sin cambios respecto al parche validado.
Retiradas heuristicas OBJ VRAM/paleta/OAM no vacias: no resolvian las
siluetas segun el usuario y anadian exploraciones de memoria por linea.
Se intento una comprobacion de la paleta propia de cada pieza OBJ en
widescreen: evita dibujar una pieza cuando su banco de 16 colores esta
completamente vacio. No se ha confirmado si elimina las siluetas reales.
NO MEDIDO: causa de las siluetas transitorias; no aparecen en las muestras
10020,10200,10400,10600 del replay existente. No se dan por arregladas.

Zoom de escenario durante menus y cinematicas (2026-10-12):
MEDIDO EN CODIGO: la ruta de planos completos solo aceptaba BG2/BG3 y
exigia ambos fondos activos con scroll identico. El detector anterior tambien
admitia terreno en BG0/BG1, pero esas capas no leian el plano RAM completo.
CORRECCION: identificar cada plano por BGCNT 4C03/4E02 y su estado RAM
ya verificado, sin depender de la activacion ni del scroll de su pareja.
Se conserva la verificacion de ROM, FFTA_PATCH_STAGE y FFTA_PATCH_BG.
No se modifican datos del juego ni el parche de personajes.
MEDIDO: tests gba-core: 85 passed, 3 ignored. Prueba sintetica de pareja
oculta, cambio BG0/BG2, extension vertical y 600 frames con extra=0 iguales.
NO MEDIDO: replay del fallo real, hashes de framebuffer/OAM de 600 frames
reales y metrica de 8 personajes. No hay grabaciones .input en el repo
ni en la raiz E:/AI/ARC. Requiere validacion jugando; no se afirma resuelto.

Panel de stats y zoom (2026-10-12):
MEDIDO EN CODIGO: las filas extra excluian capas UI, pero dibujaban OBJ.
Cambio: incluir todas las capas texto activas tambien en las filas extra;
las UI no repiten verticalmente fuera de su mapa de tiles. Ruta extra=0 intacta.
MEDIDO: 86 tests passed, 3 ignored; prueba de panel en y=180 y no repeticion.
NO MEDIDO: reproduccion visual del panel del enemigo de la captura del usuario.

Panel al pasar el cursor sobre unidades (2026-10-12):
MEDIDO: replay derivado zoom-no-range.input recuperado del repo anterior.
La grabacion ffta-batalla-3.input de ese repo contiene solo su cabecera.
Dos ejecuciones desde frame 0 hasta 13000: hash f158877f466426fb igual.
Se usa exclusivamente target/ffta-stats/repro.sav, copia del respaldo.
MEDIDO: el panel de Ritz no es un BG: productor 080014C8, lista 083916FA,
cuatro piezas semitransparentes, tiles 1F0/200/208, paleta OBJ 11.
Todas las piezas y tiles existen. BLDCNT=3F43, BLDALPHA=090E: el fondo
se mezcla con terreno y parece parcialmente ausente; no es descarte de piezas.
Cambio de presentacion: FFTA_PATCH_STATS=1 por defecto vuelve opaco solo
ese fondo identificado en la lista host de la ROM exacta y funciones verificadas.
No cambia atributos OAM ni logica del juego; extra=0 conserva la ruta original.
MEDIDO: 87 tests normales y 3 con ROM local pasan. Frame 13000 cambia
2558 pixels, todos dentro de x600..687/y184..231; exterior identico y OAM igual.
MEDIDO: framebuffer y OAM iguales en 600 frames con extra=0, STATS=0/1.
Capturas hover-13000.png y after-13000.png en target/ffta-stats.
NO MEDIDO: panel rojo de enemigos; el replay construido no llego a mostrarlo.
La metrica de este recorrido derivado tiene frames con menos de 8: no se
considera aprobada la prueba de 8 constantes ni se toca el parche de personajes.
MEDIDO: comparacion STATS=0/1 en 1201 frames (11800..13000): metrica
identica en todos. 297 frames dan menos de 8 en ambas versiones de este
replay derivado con la biblioteca nativa recuperada; no es una regresion nueva.
La lista completa queda en target/ffta-stats/character-failures.json.
Windows Installer/payload y TAR Linux compilados y actualizados, sin commits.

Correccion del intento anterior de stats (2026-10-12):
El usuario confirma que quitar semitransparencia NO arreglo el recorte.
Se RESTAURA siempre el modo semi original; se descarta el intento opaco.
MEDIDO en VRAM: las cuatro piezas del fondo contienen una curva con
pixels transparentes: cobertura de 67 pixels en fila 104 y 88 en fila 127.
Esto es distinto de las capas BG del menu de acciones.
Cambio host: completar solo los texels transparentes de esas piezas
identificadas, usando el indice de color vivo del tile rectangular 208.
Se preservan paleta, letras, atributos OAM, prioridad y alpha original.
FFTA_PATCH_STATS=0 conserva forma original; 1 completa el fondo en widescreen.
No es una recuperacion de sprites o tiles ausentes: es completar la geometria
del fondo del panel, limitado a la plantilla verificada y sin tocar la ROM.
MEDIDO: 88 tests passed, 3 ignored. Test de huecos, color vivo, semitransparencia
y extra=0. Nueva comparacion real de 600 frames: framebuffer/OAM iguales.
NO MEDIDO: panel rojo; la validacion visual del usuario sigue pendiente.
MEDIDO: replay completo hasta 13000 con el cambio nuevo. Fondo sin huecos
y alpha preservado: cap-after-13000.png. Se completan 217 pixels, todos
dentro del panel; fuera del panel identico y atributos OAM identicos.
MEDIDO: metrica de personajes identica en los 1201 frames de STATS=0/1.
Los 297 frames con menos de 8 del recorrido derivado permanecen iguales.
El comando PowerShell devolvio 1 pese a llegar al frame 13000 y generar
los volcados; el log registra NativeCommandError para mensajes de stderr.
Los verificadores posteriores terminan con codigo 0; no se oculta ese retorno.
Windows y Linux compilados y actualizados, sin tocar saves originales.

Objetivo correcto del panel de stats (aclaracion del usuario):
El semicirculo NO estaba mal. El fallo es que el panel no llega al borde.
Se REVIERTE completar sus texels transparentes. Forma y alpha originales.
Nuevo STATS=1: solo las piezas rectangulares del tile 208 se prolongan
hasta el borde del framebuffer ancho. H-flip elige izquierda o derecha.
Se repite su columna exterior viva, con su textura, paleta y semitransparencia.
Capas, posiciones de texto, curva, OAM y logica de personajes intactas.
MEDIDO: 88 tests passed, 3 ignored. Test de ambas orientaciones, curva
sin rellenar, alpha y no extension con STATS=0 o extra=0.
MEDIDO: comparacion real de 600 frames extra=0: framebuffer/OAM iguales.
MEDIDO: reconstruccion del volcado privado en edge-preview.png muestra
la banda transparente hasta el borde derecho, con la curva conservada.
NO MEDIDO: replay completo nuevo de 1201 frames, panel rojo y validacion
jugando del usuario. El preview reconstruye el volcado, no es una captura nueva.
Windows y Linux compilados; Installer/payload y TAR actualizados, sin commits.

Menu de pausa nativo adaptado del HTML del usuario (2026-10-13):
Estilo pergamino, gradientes crema, bordes laton, esquinas ornamentales,
seleccion cuero/dorado y dibujos propios para iconos. Sin motor HTML,
red, dependencias nuevas ni fuentes o iconos web empaquetados.
Se conserva la fuente pixel nativa: Cinzel/Inter y los efectos Web Audio
no se trasladan literalmente. Valores reales de av.cfg/input.cfg conservados.
Input abre pestanas Keyboard/Gamepad y conserva el dispositivo elegido.
Listas desplazables por teclado/mando/rueda; boton Back siempre visible.
Selectores video desplegables con aceptar/cancelar; FF segmentado clickable.
Exit pide confirmacion con No seleccionado por defecto.
MEDIDO: 42 tests passed: input-config 12, menu 19, recomp 11.
Pruebas de dropdown, targets de remapeo con scroll, tabs, FF y salida segura.
MEDIDO: previews nativos generados/revisados para main, gamepad, FF,
salida y selector. Windows/Linux release compilados y payload/TAR actualizados.
NO MEDIDO: interaccion jugando con ventana/mando fisico y efectos del GPU
sobre el estilo; no se inicio una partida ni se tocaron saves originales.
Referencia HTML en target/pause-menu/reference.html; previews en esa carpeta.

## 2026-10-13 - Menu moderno e iconos de mando completos

Menu renderizado en un canvas independiente de la GBA, sin alterar el framebuffer
del juego. Cinzel/Inter con antialiasing y avisos OFL en Licenses/fonts.
Iconos vectoriales para A/B/X/Y, LB/RB, LT/RT, View/Menu/Guide, L3/R3,
las cuatro direcciones del D-pad y las direcciones de ambos sticks.
Ratones usan coordenadas del canvas; GPU y fallback CPU presentan el mismo menu.
Asignaciones y parches del juego conservados. Preview: target/pause-menu/gamepad-modern.png.
Prueba fisica del mando: NO MEDIDO.
