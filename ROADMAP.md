# ROADMAP

Estado de avance de `rust-distributed-systems`, repositorio del camino troncal
de Jeresoft Academy para sistemas distribuidos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio acaba de nacer con su estructura inicial, licencias, README,
AGENTS, crate Rust mínimo y tabla de capítulos planeados.

Todavía no hay capítulos implementados ni publicados. El siguiente paso natural
es convertir el alcance del curso en milestones e issues, antes de tocar código
de curso, para que el checklist operativo viva en GitHub.

## Capítulos planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Consenso | planned |
| 02 | Raft | planned |
| 03 | Paxos | planned |
| 04 | Elección de líder | planned |
| 05 | Locks distribuidos | planned |
| 06 | Vector clocks | planned |
| 07 | Lamport clocks | planned |
| 08 | CRDTs | planned |
| 09 | Teorema CAP | planned |
| 10 | Consistent hashing | planned |
| 11 | Protocolo gossip | planned |
| 12 | Transacciones distribuidas | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.

## Fuera de alcance por ahora

- Construir infraestructura distribuida de producción.
- Ocultar los mecanismos detrás de dependencias externas.
- Usar `unsafe` sin justificación escrita y revisión humana explícita.
- Reexplicar redes, sistemas operativos, concurrencia o bases de datos desde
  cero; esos fundamentos viven en sus cursos canónicos.
- Publicar capítulos parciales como si estuvieran completos.

## Siguiente paso natural

Crear milestones e issues para todo el plan del repositorio, asignarlos a
`jeresoftx`, etiquetarlos y usarlos como checklist antes de iniciar la
implementación capítulo por capítulo.
