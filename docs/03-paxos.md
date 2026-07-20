# 03. Paxos

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes. Todavía no tiene
> modelo Rust, tests, ejemplos, ejercicios, benchmark ni revisión humana.

## Concepto

Paxos es una forma de lograr consenso mediante propuestas numeradas, promesas de
aceptores y aceptación por quórum. Su núcleo no depende de un líder estable: la
seguridad nace de que los aceptores recuerdan qué prometieron y qué aceptaron.

La frase corta es esta: Paxos permite que varios proponentes compitan sin que el
sistema pueda elegir dos valores incompatibles.

## Problema

En un sistema distribuido, una propuesta puede llegar tarde, duplicada o después
de otra propuesta más nueva. Si los nodos aceptan cualquier valor que llegue con
mayoría momentánea, dos grupos podrían creer que decidieron historias
incompatibles.

Paxos responde preguntas más formales que Raft:

- qué hace que una propuesta vieja deje de ser válida;
- qué memoria mínima debe conservar un aceptor;
- cuándo una mayoría obliga a respetar un valor previo;
- cómo se mantiene seguridad aunque varios proponentes compitan;
- por qué una decisión futura no puede contradecir una decisión ya elegida.

## Modelo educativo esperado

El modelo de este curso debe representar Paxos clásico para una sola decisión,
sin red real, hilos, disco ni timeouts físicos:

- `NodeId`: identidad estable de participante;
- `ProposalNumber`: número ordenable de propuesta;
- `ProposalValue`: valor educativo propuesto;
- `AcceptorState`: promesa más alta y aceptación previa;
- `PrepareRequest`: solicitud de promesa;
- `Promise`: respuesta de un aceptor;
- `AcceptRequest`: solicitud de aceptar un valor;
- `Accepted`: aceptación observable;
- `PaxosRound`: escenario determinista para enseñar quórums.

El objetivo no es esconder Paxos detrás de una API cómoda. El objetivo es que el
alumno vea por qué una promesa existe, qué obliga a adoptar un valor previo y
cómo se decide por intersección de mayorías.

## Invariantes

Paxos protege seguridad antes que comodidad. El capítulo debe hacer visibles
estas reglas:

- todo número de propuesta tiene orden total;
- un aceptor no reduce la mayor propuesta prometida;
- un aceptor rechaza propuestas menores que su promesa vigente;
- una aceptación previa queda disponible para promesas futuras;
- un proponente que observa aceptaciones previas adopta el valor de la propuesta
  aceptada con mayor número;
- un valor solo queda elegido si lo acepta una mayoría;
- dos mayorías dentro del mismo conjunto de aceptores deben intersectarse;
- si un valor queda elegido, otro incompatible no puede quedar elegido después;
- el historial permite explicar promesas, rechazos, aceptaciones y decisión.

## Alternativas

### Mayoría sin promesas

Contar votos enseña quórums, pero no basta. Sin promesas, una propuesta nueva y
una vieja pueden cruzarse de forma que parezcan válidas dos historias
incompatibles.

### Coordinador único

Un coordinador evita competencia en el camino feliz, pero mueve el problema
hacia la confiabilidad de ese coordinador. Es una idea útil para comparar con
Raft, no para explicar Paxos clásico.

### Paxos clásico

Paxos clásico es el modelo elegido para este capítulo porque separa preparación
y aceptación. Esa separación permite estudiar seguridad con precisión: primero
se obtiene una mayoría de promesas, después se propone el valor seguro.

### Multi-Paxos

Multi-Paxos reduce costo cuando hay muchas decisiones y un coordinador estable,
pero conviene llegar a él después de entender una sola decisión clásica.

## Costos

Paxos hace visible el precio de la seguridad:

- una decisión clásica necesita preparación y aceptación;
- propuestas competidoras pueden provocar rechazos y reintentos;
- cada aceptor debe recordar su mayor promesa y su aceptación previa;
- una mayoría preserva seguridad, pero limita disponibilidad durante
  particiones;
- un proponente puede terminar proponiendo un valor distinto al que quería si
  encuentra una aceptación previa.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- `prepare` con número viejo;
- `accept` rechazado por una promesa mayor;
- competencia entre dos proponentes;
- adopción obligatoria de valor previo;
- partición sin quórum;
- mensajes duplicados que no cambian la decisión.

## Límites

Este capítulo no promete:

- Multi-Paxos completo;
- liderazgo estable;
- leases;
- persistencia real;
- recovery de producción;
- red real;
- relojes físicos;
- tolerancia a fallas bizantinas;
- API de producción.

Primero se aprende por qué Paxos es seguro. Después se puede estudiar cómo se
vuelve práctico.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/02-raft.md`
- `docs/superpowers/specs/2026-07-20-paxos-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de Paxos que
represente promesas, aceptaciones, valor seguro y decisión por quórum.
