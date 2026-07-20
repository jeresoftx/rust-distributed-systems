# Consenso: cierre pedagógico del capítulo

> **Issue:** #7  
> **Milestone:** 01. Consenso  
> **Estado:** benchmarked

## Contexto

Consenso ya contaba con especificación, modelo Rust mínimo, tests de
integración, capítulo inicial, ejemplos progresivos y ejercicios. El cierre
faltante era registrar medición educativa, fuente Mermaid y estado visible.

## Decisión

Se eleva Consenso a `benchmarked` sin marcarlo como `reviewed` ni `published`.
El cierre conserva el alcance educativo actual: una ronda lógica con propuestas,
aceptaciones, quórum mayoritario, fallas explícitas e historial observable.

## Alcance

- Agregar `benches/consensus_bench.rs` para medir operaciones educativas.
- Agregar `diagrams/01-consenso.mmd` como fuente Mermaid.
- Documentar el benchmark en `docs/01-consenso.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa Raft.
- No se implementa Paxos.
- No se agrega persistencia.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench consensus_bench`
- `git diff --check`
