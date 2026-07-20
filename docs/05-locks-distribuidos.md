# 05. Locks distribuidos

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes. Todavía no tiene
> modelo Rust, tests, ejemplos, ejercicios, benchmark ni revisión humana.

## Concepto

Un lock distribuido coordina acceso exclusivo a un recurso cuando los clientes y
el recurso pueden vivir en nodos distintos.

La pregunta central no es "cómo bloqueo una variable", sino "bajo qué evidencia
aceptamos que este cliente puede operar sobre este recurso ahora".

## Problema

En una sola máquina, un mutex protege memoria compartida bajo reglas locales.
En un sistema distribuido, la propiedad de un recurso debe sobrevivir mensajes
tardíos, clientes pausados, expiraciones y observaciones parciales.

Locks distribuidos responden preguntas prácticas:

- quién puede operar un recurso compartido ahora;
- cuánto dura esa autorización;
- qué ocurre si el propietario deja de responder;
- cómo impedir que un propietario viejo escriba después de expirar;
- cómo explicar por qué una adquisición, renovación o liberación fue aceptada o
  rechazada.

## Modelo educativo esperado

El modelo de este curso debe representar locks por lease con fencing tokens:

- `ClientId`: identidad estable de cliente;
- `ResourceId`: identidad estable de recurso;
- `LogicalTime`: tiempo lógico controlado por el escenario;
- `LeaseDuration`: duración lógica del permiso;
- `FencingToken`: número monótono asociado a una propiedad;
- `LockGrant`: concesión observable;
- `DistributedLockManager`: coordinador con locks activos, tokens e historial;
- adquisición de lock;
- renovación por propietario;
- liberación por propietario;
- expiración explícita;
- validación de operaciones con fencing token.

El objetivo no es construir un servicio de coordinación real. El objetivo es
aislar la diferencia entre exclusión local y propiedad temporal distribuida.

## Invariantes

El capítulo debe hacer visibles estas reglas:

- un recurso tiene como máximo un propietario activo;
- todo lock tiene expiración;
- cada adquisición exitosa produce un fencing token mayor;
- solo el propietario vigente puede renovar;
- solo el propietario vigente puede liberar;
- un lock vencido deja de bloquear nuevas adquisiciones;
- una operación con token viejo no debe modificar el recurso protegido;
- el historial explica adquisiciones, renovaciones, liberaciones, expiraciones
  y rechazos.

## Alternativas

### Mutex local

Un mutex local es correcto dentro de un proceso, pero no coordina clientes en
nodos distintos.

### Lock en base de datos

Una base de datos puede representar locks con filas, restricciones únicas o
transacciones. Es práctico, pero oculta parte del mecanismo que el curso necesita
estudiar.

### Coordinador por consenso

Un coordinador respaldado por consenso puede ofrecer una autoridad más robusta,
pero conviene estudiarlo después de entender qué significa poseer un lock.

### Lease con fencing token

Es el modelo elegido para este capítulo. El lease limita el tiempo de propiedad;
el fencing token permite que el recurso protegido rechace operaciones viejas.

## Costos

Los locks distribuidos tienen precio:

- cada adquisición depende de una autoridad compartida;
- leases cortos generan más renovaciones;
- leases largos prolongan propiedad obsoleta después de fallas;
- fencing tokens obligan al recurso protegido a validar escrituras;
- particiones fuerzan decisiones entre progreso y seguridad;
- un coordinador simple puede convertirse en punto de falla.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- recurso ocupado;
- lease expirado;
- token obsoleto;
- renovación desde cliente incorrecto;
- liberación desde cliente incorrecto;
- solicitud duplicada;
- coordinador no disponible.

## Límites

Este capítulo no promete:

- reloj físico confiable;
- servicio de locks de producción;
- consenso completo;
- persistencia real;
- red real;
- recuperación automática de coordinador;
- tolerancia a fallas bizantinas;
- API de producción.

Primero se aprende propiedad temporal. Después se estudia cómo se combina con
relojes lógicos, consenso, transacciones y operación real.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/02-raft.md`
- `docs/04-eleccion-de-lider.md`
- `docs/superpowers/specs/2026-07-20-distributed-locks-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de Locks
distribuidos con leases lógicos, fencing tokens, propiedad por recurso,
renovación, liberación, expiración y errores explícitos.
