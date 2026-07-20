# CRDTs: cierre pedagógico del capítulo

> **Issue:** #35  
> **Milestone:** 08. CRDTs  
> **Estado:** benchmarked

## Contexto

CRDTs ya contaba con especificación, modelo Rust mínimo, tests de invariantes,
capítulo extendido, ejemplos progresivos, ejercicios, soluciones ejecutables y
diagrama Mermaid. El cierre faltante era registrar medición educativa y elevar
el estado visible del capítulo.

## Decisión

Se eleva CRDTs a `benchmarked` sin marcarlo como `reviewed` ni `published`. El
cierre conserva el alcance educativo actual: G-Counter state-based, incremento
local, fusión por máximo, comparación parcial y convergencia eventual.

## Alcance

- Agregar `benches/crdt_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/08-crdts.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementan decrementos.
- No se implementan borrados ni tombstones.
- No se agrega compactación de metadatos.
- No se agrega persistencia.
- No se agrega red real.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench crdt_bench`
- `git diff --check`
