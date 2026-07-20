# ROADMAP

Estado de avance de `rust-distributed-systems`, repositorio del camino troncal
de Jeresoft Academy para sistemas distribuidos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio acaba de nacer con su estructura inicial, licencias, README,
AGENTS, crate Rust mínimo y tabla de capítulos planeados.

El plan de trabajo ya vive en GitHub como milestones e issues. Cada paso queda
asignado a `jeresoftx`, asociado al milestone correspondiente y etiquetado para
mantener la regla del repositorio: un issue, un commit y un PR.

Consenso ya cuenta con especificación, modelo Rust mínimo, tests de integración,
capítulo inicial, diagrama Mermaid, ejemplos progresivos, ejercicios, soluciones
ejecutables, fuente Mermaid y benchmark manual. Su estado visible es
`benchmarked`: tiene medición educativa, pero todavía no se marca como
`reviewed` ni `published` sin revisión humana.

Raft ya cuenta con especificación inicial, modelo Rust mínimo y tests de
invariantes, capítulo extendido, diagrama Mermaid, ejemplos progresivos,
ejercicios, soluciones ejecutables y benchmark manual. Su estado visible es
`benchmarked`: tiene medición educativa, pero todavía no se marca como
`reviewed` ni `published` sin revisión humana.

Paxos ya cuenta con especificación inicial, modelo Rust mínimo y tests de
invariantes. Su estado visible es `tested`: todavía faltan ejemplos,
ejercicios, benchmark y revisión humana.

## Progresión del Semestre 4

El curso abre el Semestre 4 con mecanismos distribuidos antes de pasar a
diseños de sistemas completos. La progresión esperada es:

1. **Fundamentos de acuerdo y tiempo:** consenso, Raft, Paxos, elección de
   líder, Lamport clocks y vector clocks.
2. **Coordinación y consistencia:** locks distribuidos, CRDTs, teorema CAP y
   transacciones distribuidas.
3. **Distribución de estado:** consistent hashing y protocolo gossip.
4. **Puente hacia System Design:** usar estos mecanismos como piezas explícitas
   en diseños como Kafka, Redis distribuido, sistemas de reservas y servicios
   replicados.

## Capítulos planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Consenso | benchmarked |
| 02 | Raft | benchmarked |
| 03 | Paxos | tested |
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

Continuar con el issue #14: escribir capítulo extendido, ejemplos progresivos y
ejercicios de Paxos. Ese paso debe convertir el modelo probado en material de
estudio guiado.
