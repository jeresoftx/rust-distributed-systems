# Consistent hashing: cierre pedagógico del capítulo

> **Issue:** #43  
> **Milestone:** 10. Consistent hashing  
> **Estado:** benchmarked

## Contexto

Consistent hashing ya contaba con especificación, modelo Rust mínimo, tests de
invariantes, capítulo extendido, ejemplos progresivos, ejercicios, soluciones
ejecutables y diagrama Mermaid. El cierre faltante era registrar una medición
educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Consistent hashing a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: anillo ordenado,
asignación por sucesor, wrap-around, inserción, retiro y comparación de
movimientos.

## Alcance

- Agregar `benches/consistent_hashing_bench.rs` para medir operaciones
  educativas.
- Documentar el benchmark en `docs/10-consistent-hashing.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementan réplicas virtuales.
- No se implementan pesos por nodo.
- No se implementa gossip de membresía.
- No se implementa migración real de datos.
- No se agrega una función hash criptográfica.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench consistent_hashing_bench`
- `git diff --check`
