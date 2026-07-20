# Locks distribuidos: cierre pedagógico del capítulo

> **Issue:** #23  
> **Milestone:** 05. Locks distribuidos  
> **Estado:** benchmarked

## Contexto

Locks distribuidos ya contaba con especificación, modelo Rust mínimo, tests de
invariantes, capítulo extendido, ejemplos progresivos, ejercicios, soluciones
ejecutables y diagrama Mermaid. El cierre faltante era registrar medición
educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Locks distribuidos a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: leases lógicos,
fencing tokens, propietario activo, renovación, liberación, expiración,
readquisición y rechazo de operaciones obsoletas.

## Alcance

- Agregar `benches/distributed_locks_bench.rs` para medir operaciones
  educativas.
- Documentar el benchmark en `docs/05-locks-distribuidos.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa un servicio de locks de producción.
- No se agregan relojes físicos ni red real.
- No se agrega persistencia.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench distributed_locks_bench`
- `git diff --check`
