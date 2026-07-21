# Cierre de capítulo: Protocolo gossip

> Estado del cierre: `benchmarked`.
>
> Este documento registra el cierre técnico del capítulo 11 sin marcarlo como
> `reviewed` ni `published`.

## Alcance cerrado

El capítulo de Protocolo gossip ya cuenta con:

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

- conocimiento monótono;
- idempotencia ante hechos duplicados;
- fanout acotado por emisor;
- omisión de nodos no disponibles;
- recuperación posterior de nodos;
- convergencia eventual bajo conectividad suficiente;
- reportes observables por ronda.

## Benchmark educativo

El benchmark vive en `benches/gossip_bench.rs` y mide:

- propagación de una ronda;
- omitir nodos no disponibles;
- convergencia eventual;
- recuperación posterior.

La medición es pedagógica. No representa rendimiento de red, serialización,
selección aleatoria, protocolos SWIM ni clusters reales.

## Límites respetados

- No se agregó `unsafe`.
- No se agregaron dependencias externas.
- No se marcó el capítulo como `reviewed`.
- No se marcó el capítulo como `published`.
- No se cambió el currículum ni la gobernanza del curso.

## Siguiente paso natural

El siguiente paso natural del curso es abrir Transacciones distribuidas. Ese
capítulo debe conectar coordinación, atomicidad, fallas parciales y límites de
consenso sin ocultar los costos operativos.
