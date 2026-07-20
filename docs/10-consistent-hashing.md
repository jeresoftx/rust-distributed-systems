# 10. Consistent hashing

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes documentados.
> Todavía no tiene modelo Rust, tests, ejemplos, ejercicios, benchmark, revisión
> humana ni está marcado como `published`.

## Concepto

Consistent hashing distribuye claves entre nodos intentando que un cambio de
membresía mueva solo una parte acotada del espacio de claves.

La pregunta central no es "qué función hash usamos", sino "cuántas claves deben
cambiar de dueño cuando agregamos o quitamos un nodo".

## Problema

En un sistema distribuido, agregar capacidad no debería obligar a mover todo el
estado. Un reparto ingenuo con `hash(clave) % N` parece natural, pero cambiar
`N` cambia el resultado para muchas claves.

Consistent hashing aparece cuando queremos que la distribución sea determinista,
que todos puedan calcular el dueño de una clave y que los cambios de membresía
tengan un costo localizado.

## Invariantes iniciales

El modelo educativo debe hacer visibles estas reglas:

- un anillo vacío no tiene dueño para ninguna clave;
- una misma clave debe producir la misma posición dentro del modelo;
- los nodos deben ordenarse de forma determinista;
- una clave pertenece al primer nodo sucesor en el anillo;
- si no existe sucesor hacia adelante, la búsqueda vuelve al inicio;
- agregar un nodo solo mueve las claves del rango que ese nodo toma;
- quitar un nodo solo mueve las claves que pertenecían al nodo retirado.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/09-teorema-cap.md`
- `docs/superpowers/specs/2026-07-20-consistent-hashing-specification.md`

## Siguiente paso

El siguiente paso natural es implementar el modelo Rust mínimo de Consistent
hashing y sus tests de invariantes.
