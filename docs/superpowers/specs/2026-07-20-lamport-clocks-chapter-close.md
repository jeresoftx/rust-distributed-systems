# Lamport clocks: cierre pedagógico del capítulo

> **Issue:** #31  
> **Milestone:** 07. Lamport clocks  
> **Estado:** benchmarked

## Contexto

Lamport clocks ya contaba con especificación, modelo Rust mínimo, tests de
invariantes, capítulo extendido, ejemplos progresivos, ejercicios, soluciones
ejecutables y diagrama Mermaid. El cierre faltante era registrar medición
educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Lamport clocks a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: evento local,
envío, recepción con `max(local, remoto) + 1`, timestamp escalar y desempate
determinista por `EventId`.

## Alcance

- Agregar `benches/lamport_clock_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/07-lamport-clocks.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa un sistema de trazas distribuido de producción.
- No se agrega sincronización de relojes físicos.
- No se agrega persistencia.
- No se agrega red real.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench lamport_clock_bench`
- `git diff --check`
