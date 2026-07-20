# Teorema CAP: cierre pedagógico del capítulo

> **Issue:** #39  
> **Milestone:** 09. Teorema CAP  
> **Estado:** benchmarked

## Contexto

Teorema CAP ya contaba con especificación, modelo Rust mínimo, tests de
decisiones bajo partición, capítulo extendido, ejemplos progresivos, ejercicios,
soluciones ejecutables y diagrama Mermaid. El cierre faltante era registrar una
medición educativa y elevar el estado visible del capítulo.

## Decisión

Se eleva Teorema CAP a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: decisiones
explícitas bajo red saludable, rechazo por consistencia, disponibilidad local
con divergencia y rutas de checkout durante partición.

## Alcance

- Agregar `benches/cap_bench.rs` para medir operaciones educativas.
- Documentar el benchmark en `docs/09-teorema-cap.md`.
- Actualizar README y ROADMAP para reflejar el estado `benchmarked`.

## Fuera de alcance

- No se implementa una red real.
- No se implementan quórums reales.
- No se implementa consenso.
- No se clasifican productos reales como CP, AP o CA.
- No se agrega reconciliación automática.
- No se agregan dependencias externas.
- No se marca el capítulo como revisado o publicado.

## Verificación esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `cargo bench --bench cap_bench`
- `git diff --check`
