# Verificación de publicación candidata

> Estado: candidato técnico para revisión humana. No publicado.

## Issue

- GitHub: #54, "Verificar suite completa y publicación candidata".
- Milestone: "13. Cierre editorial y publicación".
- Alcance: revisar que el curso tenga cierre técnico verificable sin marcar
  capítulos como `reviewed` ni `published`.

## Resultado

El curso `rust-distributed-systems` queda como candidato técnico para revisión
humana diferida. Los 12 capítulos planeados están en estado `benchmarked`, con
capítulo extendido, especificación, modelo Rust educativo, pruebas, ejemplos,
ejercicios, soluciones, diagrama Mermaid y benchmark manual.

Este estado no equivale a publicación. Según RFC-0001 §20, Joel conserva la
decisión editorial final antes de publicar, ingerir en `academy-web` o marcar
capítulos como revisados.

## Verificaciones ejecutadas

- `cargo fmt --check`: correcto.
- `git diff --check`: correcto.
- `cargo test --doc`: correcto.
- `cargo clippy --all-targets --all-features -- -D warnings`: correcto.
- `cargo test --all-targets`: correcto.
- `cargo bench`: correcto.

## Revisión editorial dirigida

Se hizo una revisión dirigida de español es-MX para términos visibles
frecuentes del cierre editorial:

- acentos en palabras como publicación, revisión, capítulo, navegación,
  índice, introducción, técnico, práctico, lógico, canónico, simulación,
  coordinación, transacción, compensación, medición y público;
- uso de `ñ` donde corresponde;
- nombres propios como Joel, Jeresoft Academy, Rust y RFC-0001;
- consistencia entre README, ROADMAP, índice mdBook y capítulos.

Los hallazgos restantes pertenecen a nombres de archivo ASCII, por ejemplo
`00-introduccion.md` o `00-convenciones-de-simulacion.md`. Se conservan así por
estabilidad de rutas, compatibilidad de herramientas y coherencia con la
estructura del repositorio.

## Riesgos pendientes

- Falta revisión humana de Joel antes de marcar cualquier capítulo como
  `reviewed` o `published`.
- Falta decidir el mecanismo de ingestión hacia `academy-web`.
- Los benchmarks son educativos y manuales; no son contratos de rendimiento de
  producción.
- La verificación principal fue local. Si GitHub Actions se agrega después,
  deberá convertirse en requisito antes de publicar.
- La navegación mdBook está preparada, pero la publicación del libro debe
  validarse cuando exista la configuración final de despliegue.

## Decisión

El repositorio queda técnicamente cerrado como curso candidato, listo para
revisión humana y correcciones posteriores mediante nuevos issues y PRs.
