# Raft: cierre pedagógico del capítulo

> **Issue:** #11  
> **Milestone:** 02. Raft  
> **Estado:** benchmarked

## Contexto

Raft ya contaba con especificación, modelo Rust mínimo, tests de invariantes,
capítulo extendido, ejemplos progresivos, ejercicios, soluciones ejecutables y
diagrama Mermaid. El cierre faltante era registrar medición educativa y elevar
el estado visible del capítulo.

## Decisión

Se eleva Raft a `benchmarked` sin marcarlo como `reviewed` ni `published`. El
cierre conserva el alcance educativo actual: términos, roles, votos, liderazgo,
log replicado, commit por mayoría, conflicto de log e historial observable.

## Alcance

- Agregar `benches/raft_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/02-raft.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa Raft de producción.
- No se agrega persistencia real.
- No se agregan snapshots.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench raft_bench`
- `git diff --check`
