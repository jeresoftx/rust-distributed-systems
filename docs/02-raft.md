# 02. Raft

> **Estado:** tested.
>
> El capítulo cuenta con especificación inicial, modelo Rust mínimo y tests de
> invariantes. Todavía no tiene ejemplos progresivos, ejercicios, benchmark ni
> revisión humana.

## Concepto

Raft es un protocolo de consenso diseñado para que un grupo de nodos mantenga
un log replicado mediante un líder, términos y reglas de confirmación por
mayoría.

La intuición es sencilla de decir y difícil de sostener correctamente: mientras
un líder legítimo coordina el clúster, las escrituras entran por él, se replican
a seguidores y solo se consideran confirmadas cuando una mayoría las reconoce.

## Problema

Consenso sobre un solo valor enseña quórums, propuestas e incompatibilidad.
Pero un servicio real no decide una sola vez. Decide una secuencia: comando 1,
comando 2, comando 3. Esa secuencia debe conservar orden incluso cuando hay
fallas.

Raft responde preguntas operacionales:

- qué nodo puede coordinar escrituras ahora;
- qué término vuelve obsoleto a un líder anterior;
- qué entradas pertenecen al log aceptado;
- cuándo una entrada pasa de recibida a confirmada;
- cómo se repara un seguidor con historial atrasado o incompatible.

## Modelo educativo esperado

El modelo de este curso debe representar las piezas mínimas de Raft sin ocultar
el mecanismo detrás de red real, hilos ni dependencias externas:

- `NodeId`: identidad estable de nodo;
- `Term`: época lógica monótona;
- `Role`: rol local del nodo: follower, candidate o leader;
- `LogIndex`: posición dentro del log;
- `LogEntry`: comando y término que lo creó;
- `CommitIndex`: última entrada confirmada;
- solicitud de voto;
- respuesta de voto;
- replicación de entradas;
- respuesta de replicación.

El objetivo no es imitar todos los detalles de una implementación industrial. El
objetivo es tener un laboratorio determinista donde se puedan observar términos,
votos, liderazgo, divergencia de logs, reparación y commit por mayoría.

## Implementación

El módulo `src/raft.rs` implementa un clúster educativo determinista. Su API
expone una secuencia de pasos observables:

- iniciar elección;
- solicitar votos;
- finalizar elección si hay mayoría;
- agregar entradas desde el líder;
- replicar entradas hacia seguidores;
- confirmar entradas al alcanzar mayoría;
- preparar escenarios de log divergente.

Uso básico:

```rust
use rust_distributed_systems::raft::{LogIndex, NodeId, RaftCluster};

let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);

let term = cluster.start_election(NodeId(1))?;
cluster.request_vote(NodeId(2), NodeId(1), term)?;
cluster.finish_election(NodeId(1))?;

let index = cluster.append_entry(NodeId(1), "set x=1")?;
cluster.replicate_entry(NodeId(1), NodeId(2), index)?;
cluster.commit_entry(NodeId(1), index)?;

assert_eq!(index, LogIndex(1));
assert_eq!(cluster.committed_command(index), Some("set x=1"));
# Ok::<(), rust_distributed_systems::raft::RaftError>(())
```

## Invariantes

Raft existe para proteger historias. Por eso el capítulo debe hacer visibles
estas reglas:

- un nodo no reduce su término local;
- un nodo concede como máximo un voto por término;
- no puede haber dos líderes legítimos del mismo término con la misma mayoría;
- una entrada confirmada no puede ser reemplazada por otra incompatible;
- una entrada solo se confirma cuando alcanza mayoría;
- mensajes con términos obsoletos no pueden cambiar el liderazgo vigente;
- un seguidor no acepta una continuación de log si no coincide el prefijo
  esperado;
- toda transición relevante queda registrada en un historial explicable.

## Alternativas

### Líder fijo

Un coordinador permanente simplifica el camino feliz, pero convierte su caída en
un punto único de falla. No enseña cómo el sistema se recupera cuando el líder
desaparece.

### Mayoría por entrada

Decidir cada entrada como una ronda aislada ayuda a recordar el capítulo de
Consenso, pero deja difusa la operación de un log largo: elección, continuidad,
reparación y avance de commit.

### Raft

Raft es el modelo elegido para este capítulo porque organiza el problema con un
vocabulario práctico: término, líder, follower, candidato, log, commit y
mayoría. Esa forma permite escribir escenarios pequeños sin perder el vínculo
con sistemas reales.

### Paxos

Paxos aparecerá después. Es una alternativa importante, pero conviene llegar a
ella con el problema de log replicado ya visto desde una forma más operacional.

## Costos

Raft cambia simplicidad conceptual por coordinación explícita:

- una elección requiere intercambio de votos;
- una escritura requiere replicación desde el líder;
- una mayoría mejora seguridad, pero reduce disponibilidad si la partición deja
  pocos nodos juntos;
- reparar logs atrasados exige comparar índices y términos;
- conservar historial ayuda a explicar el sistema, pero crece con los eventos.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- candidato sin mayoría;
- líder aislado de la mayoría;
- mensaje de término viejo;
- seguidor con log atrasado;
- conflicto entre prefijos de log;
- entrada recibida por algunos nodos, pero no confirmada.

## Límites

Este capítulo no promete:

- almacenamiento persistente real;
- snapshots;
- membresía dinámica;
- lecturas linealizables completas;
- red real;
- temporizadores físicos;
- tolerancia a fallas bizantinas;
- API de producción.

Primero se aprende la estructura. Después se puede hablar de optimización,
persistencia, snapshots y operación real.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/superpowers/specs/2026-07-20-raft-specification.md`

## Siguiente paso

El siguiente paso natural es agregar ejemplos progresivos y ejercicios para que
el modelo se pueda estudiar desde escenarios pequeños hasta un caso real.
