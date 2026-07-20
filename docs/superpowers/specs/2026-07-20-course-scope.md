# Alcance operativo de rust-distributed-systems

## Issue

Este documento traza el issue #1: definir alcance completo de
`rust-distributed-systems`.

## Concepto

Un sistema distribuido es un sistema que conserva una intención común aunque sus
partes vivan en nodos distintos, se comuniquen por una red incierta y puedan
fallar de forma independiente.

La idea central del curso es que la incertidumbre no es una anomalía: es el
medio natural donde el sistema opera. Por eso los capítulos no deben presentar
consenso, relojes lógicos, quórums o replicación como técnicas aisladas, sino
como respuestas a preguntas concretas:

- ¿Quién decide cuando no hay una autoridad perfecta?
- ¿Qué significa "antes" y "después" si no existe reloj global confiable?
- ¿Cuántas réplicas deben responder para aceptar una lectura o una escritura?
- ¿Qué se sacrifica cuando la red se parte?
- ¿Cómo se diseña una operación para tolerar reintentos, duplicados y mensajes
  tardíos?

## Problema

Sin una frontera clara, este repositorio puede invadir cursos vecinos:
protocolos de red, concurrencia local, internals de bases de datos o diseños
completos de producto. Esa mezcla vuelve opaco el aprendizaje porque el alumno
no sabe si está estudiando un mecanismo distribuido, una primitiva local o una
decisión de arquitectura.

El curso necesita un alcance explícito antes de escribir capítulos o código
para que cada issue posterior tenga una pregunta técnica concreta.

## Alternativas consideradas

### Convertir el curso en system design

Esta opción permitiría trabajar directamente con casos como Kafka, Dropbox,
Uber o sistemas de reservas. Se descarta porque RFC-0001 §10 ya asigna esos
capítulos-proyecto a `rust-system-design`. Este repositorio debe construir las
piezas conceptuales que luego se usan allá.

### Convertir el curso en redes avanzadas

Esta opción profundizaría en TCP, QUIC, gRPC, TLS, DNS y detalles de transporte.
Se descarta porque `rust-networking` es el repositorio canónico para protocolos
de red. Aquí la red se modela como canal incierto para estudiar decisiones entre
nodos.

### Convertir el curso en concurrencia distribuida

Esta opción empezaría por locks, atomics y deadlocks locales para luego
extenderlos. Se descarta porque `rust-concurrency` ya cubre concurrencia en una
máquina. Este curso toma esos fundamentos como prerequisito y pregunta qué
cambia cuando no hay memoria compartida.

### Mantener un curso de mecanismos distribuidos

Esta opción conserva la frontera más limpia: cada capítulo implementa un modelo
pequeño en Rust para explicar coordinación, tiempo, fallas, consistencia y
replicación. Es la opción elegida porque conecta directamente con RFC-0001 §10
y prepara al alumno para `rust-system-design`.

## Decisión

`rust-distributed-systems` será el curso canónico de mecanismos distribuidos
del camino troncal. Su foco es razonar con nodos, mensajes, relojes parciales,
fallas independientes, quórums, consenso, replicación y convergencia.

El curso sí incluye:

- consenso y sus protocolos educativos principales;
- elección de líder y coordinación entre participantes;
- relojes lógicos y causalidad;
- locks distribuidos y sus límites;
- CRDTs y convergencia sin coordinación central;
- CAP como herramienta de análisis, no como eslogan;
- consistent hashing y distribución de claves;
- gossip como propagación tolerante a fallas parciales;
- transacciones distribuidas, 2PC, sagas, idempotencia y exactly-once.

El curso no incluye como canon:

- protocolos de red detallados;
- primitivas locales de concurrencia;
- internals de motores de bases de datos;
- despliegue en cloud, Kubernetes u operación de producción;
- diseños completos de productos o plataformas.

## Invariantes del curso

- Cada capítulo declara qué garantiza y qué no garantiza el modelo.
- Cada modelo tiene fallas explícitas; si una falla se ignora, se documenta.
- Los ejemplos deben ser deterministas para que las pruebas enseñen, no solo
  validen.
- Las dependencias externas no deben esconder el mecanismo central.
- Ningún capítulo se marca como `reviewed` o `published` sin revisión humana.

## Relación con repositorios vecinos

| Repositorio | Qué aporta como prerequisito o continuación |
|-------------|---------------------------------------------|
| `rust-networking` | Transporte y protocolos sobre los que viajan mensajes. |
| `rust-operating-systems` | Procesos, memoria, archivos y señales como base local. |
| `rust-concurrency` | Coordinación dentro de una máquina. |
| `rust-database-internals` | Replicación, WAL, recovery y consistencia desde motores locales. |
| `rust-system-design` | Composición de estos mecanismos en diseños completos. |
| `rust-cloud` | Plataformas donde estos sistemas se operan. |
| `rust-performance` | Medición rigurosa de costos, latencia y throughput. |

## Estado

Este issue es documental. No agrega módulos Rust ni cambia estados de capítulos.
Todos los capítulos permanecen en `planned`.
