# Elección de líder: cierre pedagógico del capítulo

> **Issue:** #19  
> **Milestone:** 04. Elección de líder  
> **Estado:** benchmarked

## Contexto

Elección de líder ya contaba con especificación, modelo Rust mínimo, tests de
invariantes, capítulo extendido, ejemplos progresivos, ejercicios, soluciones
ejecutables y diagrama Mermaid. El cierre faltante era registrar medición
educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Elección de líder a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: términos, roles,
votos por término, disponibilidad, quórum mayoritario, rechazo de términos
obsoletos e historial observable.

## Alcance

- Agregar `benches/leader_election_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/04-eleccion-de-lider.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa elección de líder de producción.
- No se agregan leases ni relojes físicos.
- No se agrega red real.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench leader_election_bench`
- `git diff --check`
