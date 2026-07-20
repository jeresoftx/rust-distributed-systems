# Especificación de Paxos

## Issue

Este documento traza el issue #12: `[03] Definir especificación e invariantes de
Paxos`.

## Concepto

Paxos es una familia de protocolos de consenso basada en propuestas numeradas,
promesas, aceptaciones y quórums. Su idea central es separar una decisión en dos
preguntas:

> ¿Qué propuesta puede seguir compitiendo sin romper decisiones anteriores?

> ¿Qué valor puede aceptarse si una mayoría ya prometió no aceptar propuestas
> viejas?

Paxos no necesita un líder estable para explicar la seguridad. Puede tener
proponentes compitiendo, mensajes retrasados y aceptores que recuerdan
promesas. La garantía importante no es que siempre sea cómodo avanzar, sino que
dos mayorías no puedan decidir valores incompatibles.

## Problema

El capítulo de Consenso mostró quórums y aceptación dentro de una ronda mínima.
Raft convirtió el consenso en un log replicado con líder, términos y reglas
operacionales. Paxos regresa al núcleo más formal: cómo preservar seguridad
cuando varias propuestas compiten y los nodos solo tienen memoria local.

Paxos aparece cuando el sistema necesita razonar sobre:

- propuestas que llegan fuera de orden;
- aceptores que prometen ignorar propuestas viejas;
- valores ya aceptados que deben respetarse en rondas posteriores;
- quórums que se intersectan aunque cambie quién participa en cada mayoría;
- diferencia entre progreso práctico y seguridad formal.

Sin esas reglas, una mayoría puede aceptar un valor y una mayoría posterior
puede aceptar otro incompatible. El sistema parecería disponible, pero perdería
la propiedad más valiosa: una decisión no contradice otra decisión válida.

## Alternativas consideradas

### Mayoría sin promesas

Un proponente envía un valor y lo considera elegido si recibe respuestas de la
mayoría.

Esta alternativa enseña quórums, pero no protege contra carreras entre
proponentes. Si los aceptores no prometen ignorar números viejos, una propuesta
posterior puede convivir con una historia incompatible.

### Coordinador único

Un coordinador decide qué propuesta se intenta y evita competencia.

Puede simplificar el camino común, pero desplaza el problema hacia la elección y
confiabilidad del coordinador. Sirve como contraste con Raft, no como modelo
principal de Paxos.

### Paxos clásico

Paxos clásico divide el protocolo en preparación y aceptación. El proponente
primero obtiene promesas de una mayoría; después propone el valor seguro según
lo que esa mayoría ya había aceptado.

Es la elección educativa de este capítulo porque deja ver por qué las promesas
existen y cómo la intersección de quórums preserva seguridad.

### Multi-Paxos

Multi-Paxos optimiza decisiones repetidas con liderazgo estable o una fase
preparada reutilizable.

Es relevante para sistemas reales, pero el primer modelo debe enseñar Paxos
clásico antes de acelerar el camino común. Multi-Paxos queda como extensión o
puente hacia capítulos posteriores.

## Decisión

El capítulo de Paxos debe construir primero un modelo educativo de una sola
decisión con estas piezas mínimas:

- `NodeId`: identidad estable de aceptores y proponentes;
- `ProposalNumber`: número total y monótono de propuesta;
- `ProposalValue`: valor educativo propuesto;
- `AcceptorState`: mayor promesa conocida y aceptación previa;
- `PrepareRequest`: intento de reservar una propuesta;
- `Promise`: respuesta que puede incluir una aceptación previa;
- `AcceptRequest`: solicitud de aceptación de valor;
- `Accepted`: aceptación observable de una propuesta;
- `PaxosRound`: escenario determinista para explorar mensajes y quórums.

El modelo inicial no necesita red real, disco, hilos, timeouts ni elección de
líder. Debe permitir construir historias pequeñas: propuesta baja rechazada,
propuesta alta prometida, valor previo adoptado y decisión por mayoría.

## Invariantes

El modelo educativo de Paxos debe declarar y probar estas invariantes:

- **Propuestas ordenables:** todo número de propuesta tiene un orden total.
- **Promesas monótonas:** un aceptor no reduce la mayor propuesta prometida.
- **Rechazo de propuesta vieja:** un aceptor rechaza `prepare` o `accept` con
  número menor que su promesa vigente.
- **Aceptación recordada:** si un aceptor acepta una propuesta, puede reportar
  esa aceptación en promesas futuras.
- **Valor seguro:** si una mayoría reporta aceptaciones previas, el proponente
  debe usar el valor de la aceptación con mayor número de propuesta.
- **Decisión por quórum:** un valor solo queda elegido cuando una mayoría lo
  acepta para la misma propuesta.
- **Intersección de quórums:** dos decisiones válidas no pueden depender de
  mayorías completamente separadas dentro del mismo conjunto de aceptores.
- **No contradicción:** si un valor queda elegido, no puede elegirse otro valor
  incompatible bajo las reglas del modelo.
- **Historia explicable:** promesas, rechazos, aceptaciones y decisión deben
  quedar visibles en eventos.

## Límites

El capítulo no promete:

- Multi-Paxos completo;
- leases;
- liderazgo estable;
- almacenamiento persistente real;
- recovery de producción;
- red real;
- relojes físicos;
- tolerancia a fallas bizantinas;
- optimizaciones de latencia.

Estos límites protegen el objetivo pedagógico: entender seguridad antes de
optimizar progreso.

## Costos

Paxos debe hacer visibles estos costos:

- una decisión clásica requiere fase de preparación y fase de aceptación;
- competir con varios proponentes puede provocar rechazos y reintentos;
- recordar promesas y aceptaciones aumenta estado local;
- exigir mayoría preserva seguridad, pero reduce disponibilidad durante
  particiones;
- adoptar valores previos puede sorprender a quien esperaba proponer un valor
  nuevo;
- explicar la seguridad exige leer historial, no solo el último estado.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- un `prepare` llega con número menor que una promesa existente;
- un `accept` llega después de que el aceptor prometió una propuesta mayor;
- dos proponentes compiten con números distintos;
- un proponente debe adoptar un valor previamente aceptado;
- una partición deja a un grupo sin quórum;
- mensajes duplicados no deben crear una segunda decisión incompatible.

Cada escenario debe separar seguridad de progreso: Paxos puede mantenerse
seguro aunque temporalmente no avance.

## Relación con capítulos posteriores

- Elección de líder usará algunas intuiciones de coordinación y autoridad
  temporal.
- Locks distribuidos compararán consenso, leases y seguridad ante particiones.
- Transacciones distribuidas retomarán promesas, preparación y commit como
  vocabulario relacionado.
- System Design podrá comparar Raft y Paxos como opciones de replicación y
  acuerdo.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Paxos en estado `draft`, pero no agrega código Rust ni marca el capítulo como
implementado, probado, revisado o publicado.
