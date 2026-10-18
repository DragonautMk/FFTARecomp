# Pendientes de mouse para la proxima sesion

Reporte del usuario del 2026-10-17. Pruebas manuales sobre la preview de Windows.
Se archiva para continuar manana; no dar estas funciones por terminadas.

## MEDIDO POR EL USUARIO

- Ultima prueba: Pub sigue sin quedar arreglado (microcambios de anclaje y
  salto directo). PENDIENTE por indicacion del usuario; parar trabajo sobre Pub.
  Las pruebas del replay anteriores no acreditan su resultado en la ventana.
  Siguiente trabajo autorizado: soporte de mouse para la tienda.

## Primera entrega tienda (Pub sigue pendiente)

### Segunda entrega: categorias, paginas y seguimiento

Nuevo adaptador propio crates/gba-core/src/ffta_shop_mouse.rs. Perfil exacto de
ROM y firmas cortas: selector USA0806EFA8/EUR08071490; store original +30;
widget de categorias gestor[0200F428]+4470. Propietario original de tienda
USA08069659/EUR0806BB0D; no aceptar widgets viejos de otra escena.
Lee indice, numero y tabla de categorias disponibles, y geometria +78/+79.
MEDIDO: compra ids[0,1,2,3,4,5], venta ids[1,2,3,5] en esta partida.
El hover cambia de categoria mediante el store original y su redibujado, sin
pasar por categorias intermedias. Un clic rapido sobre objetos espera a que se
publique la categoria solicitada. Cambiar de contexto cancela la peticion.
La lista lee comienzo de pagina list+1A y cursor relativo: tras scroll, primero1
y fila3; el mouse salta a fila0 y fila3 manteniendo primero1, probado en replay.
Categorias, listas y cantidad comparten un lienzo de UI fijo y su transformacion
inversa; antes las listas podian dibujarse en UI y detectar mouse en el escenario.
Rueda: recorre lista, o ajusta cantidad; clicar decenas/unidades elige el digito.
Cantidad: original Up cambia01->02, medido; A/izquierdo confirma, B/derecho cancela.
No cambiar cantidad con rueda en la confirmacion final. Los huecos de categorias
no disponibles al vender y las filas vacias consumen el clic sin comprar un objeto.
Pruebas nuevas: ambas regiones, disponibilidad, cambio de contexto, peticion
diferida, bytes incorrectos, pagina desplazada y coordenadas UI en varias ventanas.
Replays EUR:shop-mouse-buy y shop-mouse-sell cubren todas las categorias disponibles;
script privado target/shop_verify_replays.py. Captura de UI final verifica lado derecho.
Windows compilado. Extra=0:600/600 hashes de framebuffer/OAM coinciden con baseline.
NO MEDIDO: aceptacion manual de esta entrega, todas las tiendas/idiomas y pruebas
fisicas de mouse en USA. Pub conserva estado pendiente; no se modifica instalador.

MENU Buy/Sell/Leave: perfil de3 filas, origen[8,92], ancho del juego por idioma.
Se extrae junto con el retrato/UI a un lienzo fijo derecho y el mouse invierte
el mismo layout. No se cambia el perfil ni el enganche del Pub en esta entrega.
Seleccion directa propia de tienda: setter original USA08017FB0/EUR080190F4;
su llamador usa r4/r6 (no el convenio del Pub). Firmas cortas de setter y
llamador verificadas antes de modificar registros; las escrituras son originales.
MEDIDO replay shop-after:0->2 frame4902,2->0 frame5002, sin filas intermedias.
Lista Buy: callback de dibujo USA0806EB48/EUR08071030; firmas verificadas.
Lee contador y geometria RAM;0->2 en5402, abre cantidad y despues A/B de compra.
Prueba de cantidad/confirmacion: A a5800 abre That will be800gils, OK/CANCEL.
El clic izquierdo/derecho del runner ya envia A/B fuera de listas; no confundir
esa confirmacion con un selector de dos filas. Cantidad conserva controles GBA.
LIMITACION EXPLICITA: solo listas sin scroll/paginacion (total<=filas visibles,
inicio0). Categorias por mouse, listas paginadas y venta completa NO MEDIDOS;
son trabajo posterior, no se presenta esta entrega como tienda completa.
Prueba manual de la ventana nativa pendiente; capturas/logs privados en target/.

- El mapa todavia no funciona con el mouse. La prueba de coordenadas del runner
  no demuestra que funcione en el juego real.
- El mapa se estira demasiado hacia el lado izquierdo.
- El menu del mapa de Cyril responde al mouse. Otros pueblos NO MEDIDOS.
  Sin embargo, recorre las opciones una por una hasta alcanzar el objetivo.
- Shop no funciona con el mouse. En Buy / Sell / Leave tampoco funciona y la
  parte azul del menu esta desfasada.
- En Pub, el menu completo aparece demasiado desplazado a la izquierda.
  Debe estar colocado a la derecha.
- Los submenus de Pub responden al mouse, pero no la confirmacion de aceptar
  misiones (Accept / Yes / No).
- Los menus nuevos en general recorren filas intermedias en vez de saltar
  directamente a la opcion bajo el mouse.

## Comportamiento requerido

Si la opcion seleccionada es 1 y el mouse pasa directamente a 3, seleccionar 3
de inmediato, sin seleccionar ni recorrer 2. El adaptador debe usar el estado
logico original y su actualizacion completa; no simular una sucesion de teclas.
Esto aplica tambien al menu de Cyril, no solo a Shop y Pub.

Revisar conjuntamente la posicion visual de los paneles y la transformacion
de coordenadas del mouse. No asumir que un acierto en coordenadas nativas o
una captura del interprete prueba el comportamiento de la ventana nativa.

## Estado al parar

- Se entrego el comando de preview con target/release/recomp.exe, ROM EUR,
  FFTA_WIDESCREEN=32:9 y FFTA_MOUSE=1.
- La preview entregada no contiene necesariamente todos los ultimos cambios
  del codigo fuente: el ultimo ajuste de ancla de la flecha quedo sin confirmar
  como recompilado. Comprobar fuente, artefacto y logs al retomar.
- El replay de Pub seguia mostrando selected=2 al pedir target=3. No marcar
  el salto directo ni el soporte universal de listas como resueltos.
- Pendientes anteriores: orientacion del personaje al acabar turno, efectos
  repetidos como trueno en 32:9 y posterior port a Android, desactivado por defecto.
- Sin commits. No tocar instalador ni ROM/save originales. Compilar Windows
  tras cada fix entregado. Las pruebas del agente usan copias de los saves.

Consultar tambien docs/ffta_mouse_world.md y docs/ffta_estado.md, teniendo este
reporte manual como referencia mas reciente sobre lo que realmente funciona.

## Primera correccion para revision del usuario

Menu de batalla: el enrutado del mouse daba prioridad a action_layout (HUD
principal, capa0) incluso cuando la lista de acciones estaba en extra/Actions
(capa1). Ahora la lista activa tiene prioridad. Do it/Cancel conserva la ruta
principal cuando no hay una lista de acciones extra activa.
El cambio esta en crates/recomp/src/play.rs y sirve para ambos perfiles ROM.
La prueba reproduce un HUD simultaneo y una opcion de la lista tras pack_panel,
con distintas ventanas, aspectos, Size y Length. Falta aceptacion en juego real.
Se entrega esta correccion por separado; los otros problemas siguen pendientes.
MEDIDO:17 tests del runner y14 de input de batalla pasan. Framebuffer y OAM
identicos en600/600 frames con extra=0 y mouse apagado frente al ejecutable
anterior. Windows release compilado; logs target/battle-mouse-routing-build.log,
target/battle-mouse-routing-tests.log y target/battle-mouse-zero.log.
NO MEDIDO: aceptacion manual de este primer arreglo. Detenerse para esa prueba.

## Reporte adicional del usuario, anterior a esta correccion

## Correccion Pub 2026-10-17

### Salto directo del MENU y estados transitorios del cursor

MEDIDO replay pub-jump-ready.input, desde0, hover a opcion2 en frame4900:
antes escribia0->1 en4902 y1->2 en4910 (02025F19, PC080190F8).
Ahora escribe0->2 en4902, sin indice intermedio. El setter original guarda
el indice y el desplazamiento de fila; el llamador publica el mismo indice
desde el byte alto de r6. El enganche cambia r1/r6 y deja las escrituras al juego.
Firmas cortas verificadas: setter USA08017FB0/EUR080190F4;
retorno de llamador USA0805FA32/EUR08061C96, firmas verificadas en ambas ROMs.
Solo MENU raiz y solicitudes de mouse activas; no se altera entrada fisica.
El anclaje raiz ignora el bit08 de estado del cursor: estados07/0F conservan
la misma capa, con anchura variable por idioma. LIST mantiene su ruta anterior.
Pruebas cubren ambos perfiles, anchos96..224, estados07/0F y firma alterada.
Replay despues:161 frames4800..4960 con anclaje derecho sin cambios de capa.
NO MEDIDO: reproduccion exacta del microparpadeo reportado por el usuario;
falta su feedback del ejecutable nuevo y prueba manual de todos los idiomas.

### Revision final: raiz izquierda, submenus restaurados

Actualizacion posterior por indicacion del usuario: MENU raiz debe estar a la
derecha. Se cambia solo el lado del lienzo fijo (PPU y layout/mouse); el perfil
raiz y el tratamiento restaurado de los submenus permanecen iguales.
Correccion de idioma: captura real pub-current-desktop.png muestra el MENU en
espanol aun a la izquierda. Se elimina del perfil la anchura inglesa fija96;
la anchura validada se lee de la lista original. Tests cubren anchos96..224
ademas de96, sin incluir el estilo03 de los submenus. Falta validacion manual
del ejecutable actualizado en esa escena en espanol.

El usuario solicita revertir el desplazamiento de LIST/Rumors/Missions y
anclar el MENU raiz completo a la izquierda. El perfil visual solo acepta
el MENU medido: estilo0F, cuatro filas, origen[8,92], limite derecho96;
el estilo03 de LIST conserva la ruta visual anterior.
MENU, retrato y resto del UI raiz comparten un lienzo nativo fijo de240x160,
sin recortes independientes ni anclas calculadas desde pixeles animados.
El mouse usa ese mismo lienzo. No se cambia el motor de sprites ni instalador.
MEDIDO: replay desde frame0, entrada pub-left-hold.input (se omite A de5000
para no entrar en Rumors). Frames4800..5400:601/601 main=true, side=false,
extra=None, sin cambios de capa/lado. Prueba con extra_x306 y extra_y40.
Logs: target/pub-left-stable.log. Tests143 core+18 runner aprobados,4 ignorados.
NO MEDIDO: aceptacion manual de todas las opciones y transiciones del Pub.

Correccion posterior: el usuario rechazo la entrega anterior; el snapshot LIST
no cubria MENU con Rumors/Missions/Quit Mission/Leave. Captura real revisada:
target/pub-live-desktop.png. Snapshot correspondiente mouse-pub-simple-4800:
lista de 4 opciones, origen [8,92], limite derecho96, cursor flags0F, no03.
El perfil admite ambos estilos medidos. MENU se extrae a su propia capa derecha;
Greetings/retrato/resto del UI conservan una capa derecha independiente, para
que el ancho del dialogo no deje el menu separado de su borde. La inversion
del mouse usa la capa del menu. La prueba incluye ahora el MENU de4 opciones.
Snapshot final medido: town=true right=true, UI principal [3,1,240,155],
MENU independiente [0,78,88,157]. Tests completos:143 core+18 runner aprobados,
4 ignorados. Extra=0:600 pares framebuffer/OAM sin diferencias. La ventana
anterior sigue usando su ejecutable renombrado; abrir el nuevo para probar.

MEDIDO en snapshot EUR mouse-pub-direct-5400: BG0/1 4C03/4E02,
BG2/3 0B08/0A08; lista original de 8 filas en 02025F08, flags 03.
El panel se confundia con un dialogo y se anclaba a la izquierda; su flecha
OBJ no entraba en la capa separada. Ahora el perfil de lista activa verifica
callback original y geometria; panel y flecha van juntos a la derecha.
El mouse invierte el mismo layout y conserva la seleccion aunque la flecha
parpadee. Las confirmaciones existentes conservan prioridad frente a la lista.
La deteccion usa los perfiles exactos USA/EUR, sin nombres de opciones.
MEDIDO: snapshot separado town=true right=true bounds=[40,6,192,149].
Pruebas de fila durante parpadeo y coordenadas de panel derecho pasan.
NO MEDIDO: aceptacion manual del Pub completo y todos sus submenus; Pub USA.
No se modifica instalador ni guardado original. Detenerse para prueba del usuario.

- MEDIDO POR EL USUARIO: Do it/Cancel ya funciona. Preservarlo.
- El menu de batalla dejo de responder al mouse.
- Notice y otros avisos se ven completamente incorrectos.
- La parte superior de las orejas largas de Montblanc, Chiko y personajes
  similares aparece desfasada en batalla. No asumir aun que sea una capa concreta.
- Con Lester y otros personajes, una parte del menu de acciones aparece en el
  lado derecho, separada del resto del cartel.
- Los nombres de hechizos se extienden por la pantalla; deben quedar anclados
  a su esquina correspondiente.
- Mouse and Keyboard necesita un switch exclusivo frente a Keyboard y una
  explicacion al lado que incluya (Experimental).
- Prison: fondo azul del menu desfasado y mouse sin funcionar.
- Fast-forward con audio Normal/x1: el usuario quiere solamente musica de fondo,
  sin efectos ni otros sonidos. Pendiente separar musica y efectos; no confundir
  esto con el estado actual, que aun puede reproducir ambos.
- Buscar componentes compartidos para que un arreglo de tienda, menu o piezas
  de orejas beneficie a todos sus usuarios, sin implementar soluciones por nombre
  de tienda/personaje. Verificar los dos perfiles USA/EUR; no dar por probada una
  region basandose solo en la otra.
