# Paxos: cierre pedagógico del capítulo

> **Issue:** #15  
> **Milestone:** 03. Paxos  
> **Estado:** benchmarked

## Contexto

Paxos ya contaba con especificación, modelo Rust mínimo, tests de invariantes,
capítulo extendido, ejemplos progresivos, ejercicios, soluciones ejecutables y
diagrama Mermaid. El cierre faltante era registrar medición educativa y elevar
el estado visible del capítulo.

## Decisión

Se eleva Paxos a `benchmarked` sin marcarlo como `reviewed` ni `published`. El
cierre conserva el alcance educativo actual: una sola decisión con propuestas,
promesas, aceptaciones, valor seguro, quórum mayoritario e historial observable.

## Alcance

- Agregar `benches/paxos_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/03-paxos.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa Multi-Paxos.
- No se agrega persistencia real.
- No se agregan líderes estables ni leases.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench paxos_bench`
- `git diff --check`
