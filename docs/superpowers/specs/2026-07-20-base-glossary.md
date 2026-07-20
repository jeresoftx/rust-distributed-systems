# Glosario base de sistemas distribuidos

## Issue

Este documento traza el issue #2: definir glosario base de sistemas
distribuidos.

## Concepto

Un glosario de curso no es un diccionario exhaustivo. Es una frontera de
lenguaje compartido: evita que un capítulo use "nodo", "réplica", "quórum" o
"consistencia" con significados distintos sin avisar.

## Problema

Los sistemas distribuidos están llenos de palabras que parecen obvias hasta que
fallan: nodo, mensaje, disponibilidad, consenso o partición. Si esas palabras
no se fijan antes de escribir modelos, los tests y capítulos posteriores pueden
mezclar supuestos incompatibles.

## Decisión

El glosario base vive en `docs/00-glosario.md` y queda enlazado desde
`docs/SUMMARY.md`.

El glosario inicial cubre:

- nodo;
- mensaje;
- réplica;
- quórum;
- consenso;
- partición;
- reloj lógico;
- consistencia;
- disponibilidad.

Cada término incluye definición breve, ejemplo y límite. Los capítulos futuros
pueden extender el glosario, pero deben conservar estas definiciones como
vocabulario base o documentar explícitamente por qué necesitan una acepción más
específica.

## Estado

Este issue es documental. No agrega módulos Rust ni cambia estados de capítulos.
