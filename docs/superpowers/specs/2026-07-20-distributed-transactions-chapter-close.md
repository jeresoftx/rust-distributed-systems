# Cierre de capítulo: Transacciones distribuidas

> Estado del cierre: `benchmarked`.
>
> Este documento registra el cierre técnico del capítulo 12 sin marcarlo como
> `reviewed` ni `published`.

## Alcance cerrado

El capítulo de Transacciones distribuidas ya cuenta con:

- especificación inicial;
- capítulo extendido;
- diagrama Mermaid;
- modelo Rust mínimo;
- tests unitarios y de integración;
- ejemplos progresivos ejecutables;
- ejercicios con soluciones sugeridas;
- benchmark educativo manual.

## Invariantes cubiertas

El material actual cubre:

- identidad estable por `TransactionId`;
- commit 2PC solo con todos los participantes preparados;
- abort 2PC ante rechazo;
- error explícito cuando falta un voto;
- idempotencia de transacciones ya decididas;
- compensación de saga en orden inverso;
- registro observable de pasos y compensaciones;
- exactly-once explicado como diseño compuesto.

## Benchmark educativo

El benchmark vive en `benches/distributed_transactions_bench.rs` y mide:

- commit 2PC;
- abort 2PC;
- reintento idempotente;
- compensación de saga.

La medición es pedagógica. No representa rendimiento de red, almacenamiento
durable, locks reales, colas transaccionales ni recovery de producción.

## Límites respetados

- No se agregó `unsafe`.
- No se agregaron dependencias externas.
- No se marcó el capítulo como `reviewed`.
- No se marcó el capítulo como `published`.
- No se cambió el currículum ni la gobernanza del curso.

## Siguiente paso natural

El siguiente paso natural del curso es el cierre editorial: alinear README,
ROADMAP, navegación mdBook y verificación completa sin marcar contenido como
publicado antes de la revisión humana.
