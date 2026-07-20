# Especificación de Raft

## Issue

Este documento traza el issue #8: `[02] Definir especificación e invariantes de
Raft`.

## Concepto

Raft es una forma de construir consenso mediante un líder explícito, términos
monótonos y un log replicado. En lugar de pedir que los nodos acuerden un valor
aislado, Raft organiza el sistema alrededor de una pregunta más operable:

> ¿Qué comandos forman la historia aceptada del clúster y en qué orden?

La idea central es separar el problema en piezas visibles: elegir un líder,
replicar entradas, confirmar entradas por mayoría y hacer que los seguidores
converjan hacia el log del líder legítimo.

## Problema

El capítulo de Consenso mostró una ronda mínima: propuestas, aceptaciones,
quórum y decisión. Esa forma ayuda a entender el problema, pero todavía no dice
cómo operar un servicio replicado durante varias decisiones consecutivas.

Raft aparece cuando el sistema necesita:

- mantener una secuencia de comandos, no solo una decisión aislada;
- saber quién coordina temporalmente la escritura;
- rechazar líderes viejos después de una nueva elección;
- recuperar seguidores atrasados sin aceptar historias incompatibles;
- distinguir una entrada recibida de una entrada confirmada.

Sin estas reglas, un clúster puede tener dos líderes aparentes, logs que divergen
sin reparación clara, confirmaciones prematuras o lecturas que no se pueden
explicar desde una historia común.

## Alternativas consideradas

### Líder fijo

Un nodo predeterminado recibe todas las escrituras y replica a los demás.

Es fácil de entender, pero falla como modelo de consenso cuando ese líder cae o
queda aislado. Sirve para explicar por qué un coordinador no basta si el sistema
debe sobrevivir fallas.

### Consenso por entrada independiente

Cada posición del log se decide como una ronda de consenso aislada.

La idea es correcta como intuición, pero vuelve difícil enseñar operación real:
la elección de líder, la continuidad del log y la recuperación de seguidores
quedan dispersas.

### Raft

Raft hace explícito el rol del líder, divide el tiempo en términos y usa reglas
de coincidencia de log para mantener una historia coherente. Es una buena
elección educativa porque convierte el consenso en estados, mensajes e
invariantes que se pueden simular paso a paso.

### Paxos

Paxos también resuelve consenso, pero su forma clásica es más abstracta para una
primera implementación de log replicado. En este curso se estudia después de
Raft para contrastar claridad operacional contra formalismo.

## Decisión

El capítulo de Raft debe construir un modelo educativo de log replicado con
estas piezas mínimas:

- `NodeId`: identidad estable de cada nodo;
- `Term`: época lógica monótona que invalida liderazgo viejo;
- `Role`: `Follower`, `Candidate` o `Leader`;
- `LogIndex`: posición de una entrada en el log;
- `LogEntry`: comando educativo junto con el término que lo creó;
- `CommitIndex`: última posición conocida como confirmada;
- mensajes conceptuales de solicitud de voto y replicación.

El primer modelo no necesita red real, hilos, sockets, disco ni temporizadores
físicos. Debe preferir pasos deterministas que permitan escribir tests como
historias: elección, envío de entradas, aceptación por seguidores, avance de
commit y rechazo de mensajes obsoletos.

## Invariantes

El modelo educativo de Raft debe declarar y probar estas invariantes:

- **Términos monótonos:** un nodo nunca reduce su término local.
- **Un voto por término:** un nodo no concede dos votos distintos dentro del
  mismo término.
- **Un líder por término observable:** el modelo no debe permitir dos líderes
  legítimos en el mismo término dentro de una misma mayoría.
- **Coincidencia de log:** si dos logs contienen una entrada con el mismo índice
  y término, las entradas anteriores deben ser compatibles.
- **Commit por mayoría:** una entrada solo avanza a `committed` cuando fue
  replicada en una mayoría del clúster bajo las reglas del término vigente.
- **No sobrescribir entradas confirmadas:** una entrada confirmada no puede ser
  reemplazada por otra incompatible.
- **Rechazo de mensajes obsoletos:** mensajes con términos viejos no pueden
  modificar el liderazgo ni el log vigente.
- **Historia explicable:** cada cambio relevante debe poder reconstruirse desde
  eventos observables.

## Límites

El capítulo no promete:

- implementación de producción;
- persistencia real en disco;
- snapshots;
- cambios dinámicos de membresía;
- lecturas linealizables completas;
- transporte de red;
- timeouts basados en reloj físico;
- tolerancia a fallas bizantinas.

Estos límites son intencionales. El objetivo inicial es entender la forma del
protocolo antes de optimizarlo o acercarlo a producción.

## Costos

Raft debe hacer visibles estos costos:

- elegir líder requiere mensajes de solicitud de voto y respuestas;
- replicar una escritura requiere al menos una ronda líder-seguidores;
- confirmar por mayoría aumenta seguridad, pero reduce disponibilidad durante
  particiones;
- reparar seguidores atrasados implica comparar índices y términos;
- guardar historial facilita depurar, pero consume memoria;
- usar líder simplifica el camino común, pero concentra presión operacional.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- candidato pierde una elección porque no alcanza mayoría;
- líder viejo intenta replicar en un término obsoleto;
- seguidor rechaza una entrada por conflicto de log;
- líder confirma una entrada solo después de mayoría;
- nodo atrasado se recupera y converge hacia el log aceptado;
- partición separa al líder de la mayoría.

Cada escenario debe separar lo que el modelo garantiza de lo que deja para
capítulos o cursos posteriores.

## Relación con capítulos posteriores

- Paxos permitirá comparar otra familia de consenso.
- Elección de líder profundizará mecanismos de coordinación temporal.
- Locks distribuidos usarán liderazgo, leases o consenso como piezas de
  seguridad.
- Transacciones distribuidas dependerán de logs, coordinadores y recuperación.
- System Design tomará Raft como una pieza para servicios replicados.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Raft en estado `draft`, pero no agrega código Rust ni marca el capítulo como
implementado, probado, revisado o publicado.
