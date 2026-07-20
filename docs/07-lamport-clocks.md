# 07. Lamport clocks

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes. Todavía no tiene
> modelo Rust, tests, ejemplos, ejercicios, benchmark ni revisión humana.

## Concepto

Un Lamport clock representa tiempo lógico con un solo contador por nodo.

La pregunta central no es "qué hora era", sino "qué eventos deben observarse
como posteriores a otros dentro de una historia distribuida".

## Problema

En un sistema distribuido no existe un reloj físico perfecto compartido por
todos los nodos. Un mensaje puede enviarse desde una máquina con reloj atrasado
y recibirse en otra con reloj adelantado. Si el sistema confunde esos relojes
con causalidad, puede producir trazas imposibles de explicar.

Lamport clocks responden preguntas prácticas:

- cómo ordenar eventos sin un reloj global confiable;
- cómo asegurar que recibir un mensaje ocurre después de enviarlo;
- cómo construir trazas distribuidas deterministas;
- cuándo un contador lógico debe avanzar al observar otro nodo;
- por qué un orden escalar no prueba causalidad completa.

## Modelo educativo esperado

El modelo de este curso debe representar relojes lógicos escalares:

- `NodeId`: identidad estable de nodo;
- `LamportTimestamp`: contador lógico escalar;
- `LamportClock`: reloj local de un nodo;
- `EventId`: desempate determinista opcional;
- evento local;
- envío de mensaje;
- recepción de mensaje con `max(local, remoto) + 1`;
- comparación de timestamps;
- orden total educativo cuando se combine timestamp con nodo.

El objetivo no es simular una red ni construir un sistema de trazas completo. El
objetivo es aprender la promesa exacta de Lamport: si A causó B, entonces el
timestamp de A es menor que el timestamp de B. El inverso no se promete.

## Invariantes

El capítulo debe hacer visibles estas reglas:

- el reloj local nunca retrocede;
- un evento local incrementa el contador;
- enviar un mensaje incrementa antes de adjuntar el timestamp;
- recibir un mensaje calcula `max(local, remoto) + 1`;
- la causalidad observada produce timestamps crecientes;
- un timestamp menor no prueba causalidad por sí mismo;
- el desempate para orden total debe ser explícito;
- el modelo no depende de tiempo físico.

## Alternativas

### Timestamp físico

Un timestamp físico es útil para humanos, pero no prueba causalidad si los
relojes de las máquinas no están perfectamente sincronizados.

### Contador local

Un contador local ordena eventos dentro de un nodo, pero no avanza al observar
mensajes de otros nodos.

### Vector clock

Un vector clock conserva más evidencia causal y detecta concurrencia, pero su
metadato crece con los nodos observados.

### Lamport clock

Es el modelo elegido para este capítulo porque es compacto, fácil de transportar
y suficiente para preservar orden lógico compatible con causalidad. Su límite es
igual de importante que su utilidad: no detecta concurrencia con precisión.

## Costos

Lamport clocks tienen precio:

- un contador escalar pierde información causal;
- eventos concurrentes pueden quedar ordenados por desempate;
- el orden total educativo no significa orden causal real;
- los mensajes deben cargar timestamps lógicos;
- reinicios sin persistencia pueden romper monotonía;
- depurar causalidad fina requiere volver a vector clocks u otro metadato.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- eventos concurrentes con timestamps comparables;
- mensaje atrasado que obliga a avanzar por `max + 1`;
- interpretación incorrecta de orden escalar como causalidad probada;
- contador perdido tras reinicio;
- empate de timestamps sin desempate explícito;
- recepción de mensaje sin actualizar el reloj local.

## Límites

Este capítulo no promete:

- detectar concurrencia con precisión;
- representar todo el conocimiento causal;
- resolver conflictos;
- ordenar por tiempo físico;
- sincronización real de relojes;
- red real;
- persistencia real;
- API de producción.

Primero se aprende orden lógico compacto. Después se estudia cuándo ese orden
alcanza, cuándo necesita desempate y cuándo debe reemplazarse por evidencia
causal más rica.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/02-raft.md`
- `docs/03-paxos.md`
- `docs/06-vector-clocks.md`
- `docs/superpowers/specs/2026-07-20-lamport-clocks-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de Lamport
clocks con evento local, envío, recepción, monotonía y comparación ordenable.
