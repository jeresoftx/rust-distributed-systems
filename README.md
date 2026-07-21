# Rust Distributed Systems

Repositorio del camino troncal de Jeresoft Academy para estudiar sistemas
distribuidos en Rust. Pertenece al Semestre 4 del plan de estudios junto con
`rust-system-design` (RFC-0001 §10).

El objetivo no es construir Kubernetes, Kafka, Spanner ni una base distribuida
de producción. El objetivo es construir modelos educativos pequeños para
entender cómo razona un sistema distribuido cuando no existe un reloj global
confiable, la red puede fallar, los mensajes pueden llegar tarde y los nodos no
siempre observan la misma verdad al mismo tiempo.

## Qué contiene

- Capítulos en Markdown compatibles con mdBook.
- Modelos Rust idiomáticos, un mecanismo por módulo.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks que comparan análisis teórico con mediciones.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar en el camino

Este curso vive en el Semestre 4. Recibe ideas de `rust-networking`,
`rust-operating-systems`, `rust-concurrency` y `rust-database-internals`.
Alimenta `rust-system-design`, `rust-software-architecture`, `rust-cloud`,
`rust-performance` y los proyectos integradores como Kafka, Redis distribuido y
sistemas de reservas.

`rust-distributed-systems` es canónico para consenso, Raft, Paxos, elección de
líder, locks distribuidos, vector clocks, Lamport clocks, CRDTs, teorema CAP,
consistent hashing, protocolo gossip y transacciones distribuidas.

## Alcance del curso

El curso estudia qué cambia cuando un programa deja de vivir en una sola
máquina y empieza a coordinar múltiples nodos. Su unidad principal no es la
función, el proceso ni la tabla: es el sistema observado como conjunto de
nodos, mensajes, relojes parciales y fallas independientes.

Este repositorio sí cubre:

- coordinación entre nodos sin memoria compartida;
- comunicación con retrasos, pérdidas, duplicados y reordenamiento;
- tiempo lógico, causalidad y orden parcial de eventos;
- consenso, elección de líder y replicación como mecanismos educativos;
- consistencia, disponibilidad y tolerancia a particiones como decisiones con
  costo;
- modelos pequeños en Rust para razonar sobre invariantes y modos de falla.

Este repositorio no reemplaza:

- `rust-networking`: aquí no se reexplican TCP, UDP, HTTP, TLS ni DNS;
- `rust-operating-systems`: aquí no se reexplican procesos, memoria, señales ni
  filesystem;
- `rust-concurrency`: aquí no se reexplican mutex, atomics, memory ordering ni
  deadlocks locales;
- `rust-database-internals`: aquí no se reexplican B-Tree, WAL, MVCC,
  recovery ni query optimizer;
- `rust-system-design`: aquí se construyen mecanismos; allá se componen
  sistemas completos inspirados en productos reales.

La frontera práctica es esta: si el aprendizaje depende de una decisión tomada
por varios nodos bajo incertidumbre, pertenece a este curso; si depende de una
abstracción local, un protocolo de red específico o un diseño de producto
completo, pertenece a otro repositorio del camino.

## Capítulos

| # | Capítulo | Módulo | Estado |
|---|----------|--------|--------|
| 01 | Consenso | `src/consensus.rs` | benchmarked |
| 02 | Raft | `src/raft.rs` | benchmarked |
| 03 | Paxos | `src/paxos.rs` | benchmarked |
| 04 | Elección de líder | `src/leader_election.rs` | benchmarked |
| 05 | Locks distribuidos | `src/distributed_locks.rs` | benchmarked |
| 06 | Vector clocks | `src/vector_clock.rs` | benchmarked |
| 07 | Lamport clocks | `src/lamport_clock.rs` | benchmarked |
| 08 | CRDTs | `src/crdt.rs` | benchmarked |
| 09 | Teorema CAP | `src/cap.rs` | benchmarked |
| 10 | Consistent hashing | `src/consistent_hashing.rs` | benchmarked |
| 11 | Protocolo gossip | `src/gossip.rs` | implemented |
| 12 | Transacciones distribuidas | `src/distributed_transactions.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
src/
  lib.rs
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- Los planes de implementación viven en `docs/superpowers/plans/`.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
