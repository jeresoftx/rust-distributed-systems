# Vector clocks: cierre pedagógico del capítulo

> **Issue:** #27  
> **Milestone:** 06. Vector clocks  
> **Estado:** benchmarked

## Contexto

Vector clocks ya contaba con especificación, modelo Rust mínimo, tests de
invariantes, capítulo extendido, ejemplos progresivos, ejercicios, soluciones
ejecutables y diagrama Mermaid. El cierre faltante era registrar medición
educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Vector clocks a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: incremento local,
fusión por máximo, comparación causal, detección de concurrencia y tratamiento
de nodos ausentes como cero.

## Alcance

- Agregar `benches/vector_clock_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/06-vector-clocks.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa un CRDT completo.
- No se agrega resolución automática de conflictos.
- No se agrega persistencia.
- No se agrega red real.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench vector_clock_bench`
- `git diff --check`
