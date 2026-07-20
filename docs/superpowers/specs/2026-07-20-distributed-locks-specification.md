# Especificación de Locks distribuidos

## Issue

Este documento traza el issue #20: `[05] Definir especificación e invariantes de
Locks distribuidos`.

## Concepto

Un lock distribuido es un mecanismo para coordinar acceso exclusivo a un recurso
cuando los clientes, el coordinador y el recurso no necesariamente viven en la
misma máquina.

La palabra "lock" puede engañar. En una sola máquina, un mutex suele proteger
memoria compartida dentro de un proceso. En un sistema distribuido, un lock
protege una decisión observable: qué cliente tiene permiso temporal para operar
sobre un recurso mientras la red puede fallar, los mensajes pueden llegar tarde
y un cliente puede creer que todavía posee algo que ya expiró.

El concepto central no es bloquear por bloquear. El concepto central es
propiedad temporal verificable.

## Problema

En sistemas reales aparece una pregunta recurrente: "¿quién puede tocar este
recurso ahora?". Puede ser una tarea programada, un archivo compartido, un
índice de búsqueda, una migración, una cuenta, una orden, una partición de cola
o una fila lógica en un sistema externo.

Si dos clientes creen tener el mismo lock al mismo tiempo, el sistema puede:

- ejecutar dos veces una tarea que debía ser única;
- escribir estados incompatibles;
- procesar pagos duplicados;
- romper orden de mensajes;
- dejar una migración a medias;
- sobrescribir trabajo de un propietario más reciente.

La dificultad no está en guardar un booleano `locked = true`. La dificultad está
en responder qué ocurre cuando:

- el propietario se pausa y regresa tarde;
- el lease expira mientras un mensaje viejo sigue en vuelo;
- el coordinador recibe solicitudes duplicadas;
- un cliente intenta liberar un lock que ya no le pertenece;
- el recurso protegido necesita rechazar operaciones obsoletas.

## Alternativas consideradas

### Mutex local

Un mutex local protege memoria dentro de un proceso. Es correcto para
concurrencia local, pero no sirve cuando los participantes están en procesos o
nodos distintos.

### Lock en una base de datos

Una fila con restricción única, `SELECT ... FOR UPDATE` o una transacción puede
servir como lock práctico. Es una opción útil en producción cuando la base de
datos ya es la fuente de verdad, pero este capítulo no debe depender de un motor
externo para enseñar la invariante.

### Lock por consenso

Un coordinador respaldado por consenso puede ofrecer exclusión más fuerte si la
mayoría está disponible. Es una base sólida para sistemas como servicios de
coordinación, pero conviene estudiarla después de entender qué protege el lock.

### Lease con fencing token

Un coordinador concede propiedad temporal y entrega un token monótono. El
recurso protegido debe rechazar operaciones con tokens viejos.

Es la opción elegida para el modelo educativo porque separa dos ideas que suelen
confundirse:

- el lease dice "puedes intentar operar durante esta ventana";
- el fencing token dice "esta operación pertenece a una propiedad más reciente
  que las anteriores".

### Algoritmos multi-coordinador

Algoritmos como Redlock intentan coordinar locks sobre varias instancias. Son
útiles para discusión comparativa, pero quedan fuera del modelo inicial porque
agregan supuestos de tiempo, quórum y operación que pueden ocultar la lección
principal.

## Decisión

El capítulo de Locks distribuidos debe construir un modelo educativo de lease
con fencing tokens sobre tiempo lógico determinista. Sus piezas mínimas son:

- `ClientId`: identidad estable de cliente;
- `ResourceId`: identidad estable de recurso protegido;
- `LogicalTime`: reloj lógico del escenario, controlado por el test;
- `LeaseDuration`: duración lógica del permiso;
- `FencingToken`: token monótono entregado al adquirir el lock;
- `LockGrant`: concesión con propietario, recurso, token y expiración;
- `DistributedLockManager`: coordinador determinista de locks;
- eventos observables para adquisición, renovación, liberación, expiración y
  rechazo;
- errores explícitos para recurso ocupado, propietario incorrecto, token
  obsoleto, lease vencido y duración inválida.

El modelo inicial no necesita red real, hilos, temporizadores físicos,
persistencia, base de datos ni dependencia externa. Debe permitir escribir
historias pequeñas: cliente adquiere lock, otro cliente es rechazado, el
propietario renueva, el lease expira, otro cliente adquiere con token mayor y
una operación vieja queda rechazada por fencing.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Exclusión por recurso:** un recurso tiene como máximo un propietario activo
  en un instante lógico.
- **Propiedad acotada:** todo lock tiene expiración explícita.
- **Token monótono:** cada adquisición exitosa produce un fencing token mayor
  que cualquier token anterior del mismo recurso.
- **Renovación autorizada:** solo el propietario vigente puede renovar el lock.
- **Liberación autorizada:** solo el propietario vigente puede liberar el lock.
- **Expiración libera propiedad:** un lock vencido deja de bloquear nuevas
  adquisiciones.
- **Token viejo no protege escritura:** una operación con fencing token menor al
  token vigente del recurso debe rechazarse, aunque venga de un cliente que
  alguna vez fue propietario.
- **Idempotencia controlada:** solicitudes duplicadas no deben crear tokens
  incompatibles ni dos propietarios activos.
- **Historia explicable:** toda decisión debe reconstruirse desde eventos
  observables.

## Límites

El capítulo no promete:

- relojes físicos confiables;
- precisión de tiempo real;
- consenso completo;
- leases linealizables de producción;
- persistencia real;
- tolerancia a fallas bizantinas;
- recuperación automática de coordinador;
- red real;
- API de producción.

Estos límites evitan vender un lock distribuido como si fuera un mutex mágico.

## Costos

El capítulo debe hacer visibles estos costos:

- cada adquisición requiere coordinar con una autoridad compartida;
- leases cortos reducen tiempo de bloqueo después de fallas, pero aumentan
  renovaciones;
- leases largos reducen ruido, pero alargan ventanas de propiedad obsoleta;
- fencing tokens obligan al recurso protegido a validar escrituras;
- un coordinador único simplifica enseñanza, pero puede ser cuello de botella o
  punto de falla;
- durante particiones, el sistema debe elegir entre progreso y seguridad.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- cliente intenta adquirir un recurso ocupado;
- propietario se pausa y su lease expira;
- cliente viejo intenta escribir con token obsoleto;
- cliente que no es propietario intenta renovar;
- cliente que no es propietario intenta liberar;
- solicitud duplicada de adquisición;
- duración de lease inválida;
- coordinador no disponible.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Consenso explica por qué una decisión compartida necesita quórum cuando hay
  fallas.
- Raft y elección de líder explican cómo puede existir una autoridad temporal
  para coordinar operaciones.
- Locks distribuidos usan esa autoridad temporal para proteger recursos
  concretos.
- Lamport clocks y vector clocks ayudarán a separar propiedad temporal de orden
  causal.
- Transacciones distribuidas usarán coordinadores, bloqueos y recuperación como
  piezas de atomicidad entre nodos.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Locks distribuidos en estado `draft`, pero no agrega código Rust ni marca el
capítulo como implementado, probado, revisado o publicado.
