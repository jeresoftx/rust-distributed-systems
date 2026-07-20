# Introducción

Un sistema distribuido es un sistema que debe comportarse como una unidad útil,
aunque sus partes vivan en nodos distintos y se comuniquen por una red que no
garantiza tiempo, orden ni entrega perfecta.

Este curso estudia esa tensión con Rust: primero con modelos pequeños, después
con protocolos y finalmente con decisiones de diseño que conectan con sistemas
reales.

## Ideas centrales

- No existe un reloj global confiable.
- Un mensaje puede perderse, duplicarse, retrasarse o llegar fuera de orden.
- Un nodo puede fallar mientras otro nodo sigue operando correctamente.
- La consistencia es una decisión de diseño, no una palabra decorativa.
- La disponibilidad, la latencia y la tolerancia a particiones tienen costos
  observables.

## Enfoque del curso

Cada capítulo empieza por el problema, declara las invariantes del modelo,
compara alternativas y termina con código Rust, ejemplos, pruebas, ejercicios,
soluciones y mediciones cuando aporten aprendizaje.
