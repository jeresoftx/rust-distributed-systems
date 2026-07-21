# Ruta de lectura

Este curso puede leerse como libro técnico o usarse como crate educativo. La
ruta recomendada separa fundamentos, coordinación, tiempo lógico, consistencia,
distribución de estado y cierre transaccional.

## Antes de empezar

Lee primero:

- `00-introduccion.md`: propósito del curso;
- `00-glosario.md`: vocabulario común;
- `00-convenciones-de-simulacion.md`: reglas de los modelos Rust.

Estos tres archivos fijan el lenguaje del curso. Evitan que cada capítulo
reexplique qué significa nodo, mensaje, falla, historia, invariante o
simulación.

## Bloque 1: acuerdo y liderazgo

Lee en este orden:

1. `01-consenso.md`
2. `02-raft.md`
3. `03-paxos.md`
4. `04-eleccion-de-lider.md`

Este bloque explica cómo varios nodos llegan a una decisión compartida, por qué
el liderazgo ayuda a ordenar trabajo y qué costos aparecen cuando la red o los
nodos fallan.

## Bloque 2: coordinación y tiempo

Lee en este orden:

1. `05-locks-distribuidos.md`
2. `06-vector-clocks.md`
3. `07-lamport-clocks.md`

Este bloque enseña coordinación por exclusión, tokens de protección y tiempo
lógico. No busca reemplazar cursos de concurrencia ni sistemas operativos; aquí
el foco está en fallas distribuidas y observación parcial.

## Bloque 3: consistencia y convergencia

Lee en este orden:

1. `08-crdts.md`
2. `09-teorema-cap.md`

Este bloque compara convergencia eventual con decisiones bajo partición. Sirve
para entender cuándo el sistema puede aceptar divergencia temporal y cuándo una
operación exige coordinación más fuerte.

## Bloque 4: distribución de estado

Lee en este orden:

1. `10-consistent-hashing.md`
2. `11-protocolo-gossip.md`

Este bloque muestra cómo repartir claves y propagar conocimiento sin asumir una
vista perfecta del cluster. Consistent hashing decide dueños; gossip difunde
hechos y cambios de vista.

## Bloque 5: decisiones de negocio distribuidas

Lee al final:

1. `12-transacciones-distribuidas.md`

Este capítulo combina ideas anteriores: coordinación, fallas parciales,
idempotencia, compensación y límites de exactly-once práctico.

## Uso con Rust

Cada capítulo apunta a un módulo de `src/`, ejemplos en `examples/soluciones/`,
tests de integración en `tests/` y benchmarks educativos en `benches/`.

Comandos útiles:

```bash
cargo test --all-targets
cargo test --doc
cargo bench
```

Los benchmarks son material didáctico. No son promesas de rendimiento de
producción.

## Estado editorial

Todos los capítulos están en estado `benchmarked`. Esto significa cierre técnico
educativo, no publicación. Ningún capítulo debe marcarse como `reviewed` ni
`published` sin revisión humana de Joel.

## Ingestión futura

Esta navegación queda lista para que `academy-web` la consuma cuando se decida
el mecanismo de contenido. Hasta entonces, `docs/SUMMARY.md` es el índice
canónico de lectura y no asume una plataforma de publicación específica.
