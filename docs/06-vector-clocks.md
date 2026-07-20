# 06. Vector clocks

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes. Todavía no tiene
> modelo Rust, tests, ejemplos, ejercicios, benchmark ni revisión humana.

## Concepto

Un vector clock representa evidencia causal entre eventos distribuidos mediante
un contador por nodo observado.

La pregunta central no es "qué hora era", sino "qué sabía este evento sobre los
eventos anteriores del sistema".

## Problema

En un sistema distribuido no todos los eventos tienen un orden demostrable. Dos
nodos pueden modificar datos al mismo tiempo sin observarse mutuamente. Si el
sistema ordena esas versiones solo por timestamp físico, puede borrar
concurrencia real.

Vector clocks responden preguntas prácticas:

- si una versión deriva causalmente de otra;
- si dos versiones son concurrentes;
- qué conocimiento debe fusionarse al recibir un mensaje;
- cuándo una actualización es vieja;
- cuándo una actualización necesita resolución de conflicto.

## Modelo educativo esperado

El modelo de este curso debe representar causalidad parcial con relojes vector:

- `NodeId`: identidad estable de nodo;
- `Counter`: contador lógico por nodo;
- `CausalRelation`: relación entre dos relojes;
- `VectorClock`: mapa de nodo a contador;
- incremento local;
- fusión por máximo componente a componente;
- comparación causal;
- consulta de contador por nodo.

El objetivo no es simular una red ni construir un CRDT completo. El objetivo es
aprender a ver cuándo el sistema puede probar causalidad y cuándo solo puede
decir "estos eventos son concurrentes".

## Invariantes

El capítulo debe hacer visibles estas reglas:

- incrementar un nodo solo aumenta su propio contador;
- fusionar relojes conserva el máximo observado por cada nodo;
- después de fusionar no se pierde conocimiento;
- un reloj comparado consigo mismo es igual;
- `Before` significa inclusión causal estricta;
- `After` significa inclusión causal estricta en sentido contrario;
- `Concurrent` significa que ningún reloj contiene al otro;
- un nodo ausente cuenta como `Counter(0)` durante la comparación.

## Alternativas

### Timestamp físico

Un timestamp físico es cómodo para humanos, pero no prueba causalidad si los
relojes no están perfectamente sincronizados.

### Contador local

Un contador local ordena eventos dentro de un nodo, pero no entre nodos.

### Lamport clock

Lamport clocks son compactos y preservan una implicación útil: si A causó B,
entonces A tiene reloj menor que B. Pero no detectan concurrencia con precisión.

### Vector clock

Es el modelo elegido para este capítulo porque representa causalidad parcial de
forma explícita: antes, después, igual o concurrente.

## Costos

Vector clocks tienen precio:

- el tamaño del reloj crece con el número de nodos;
- cada mensaje debe cargar metadatos causales;
- comparar relojes exige revisar componentes;
- nodos dinámicos complican poda y compactación;
- detectar concurrencia no resuelve el conflicto automáticamente.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- actualización concurrente;
- mensaje atrasado;
- fusión incompleta;
- interpretación incorrecta de `Concurrent`;
- crecimiento excesivo del metadato;
- pérdida de componentes del reloj.

## Límites

Este capítulo no promete:

- orden total global;
- tiempo físico;
- resolución automática de conflictos;
- CRDT completo;
- persistencia real;
- red real;
- API de producción.

Primero se aprende causalidad parcial. Después se estudia cómo se usa para
replicación eventual, CRDTs, resolución de conflictos y diseños de sistemas.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/04-eleccion-de-lider.md`
- `docs/05-locks-distribuidos.md`
- `docs/superpowers/specs/2026-07-20-vector-clocks-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de Vector clocks
con incremento local, fusión por máximo, comparación causal y tratamiento de
nodos ausentes como cero.
