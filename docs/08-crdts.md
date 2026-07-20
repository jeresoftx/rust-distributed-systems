# 08. CRDTs

> **Estado:** tested.
>
> El capítulo cuenta con especificación inicial, modelo Rust mínimo e
> invariantes probadas. Todavía no tiene ejemplos progresivos, ejercicios,
> benchmark ni revisión humana.

## Concepto

Un CRDT es una estructura de datos replicada que puede modificarse en varias
réplicas y converger sin coordinación central en cada operación.

La pregunta central no es "quién ganó", sino "cómo diseñamos el estado para que
fusionarlo no pierda información legítima".

## Problema

En un sistema distribuido, pedir consenso para cada escritura puede ser correcto
pero caro. Durante una partición de red, también puede hacer que una parte del
sistema deje de aceptar cambios.

CRDTs responden preguntas prácticas:

- cómo aceptar cambios locales y sincronizar después;
- cómo tolerar mensajes duplicados o fuera de orden;
- cómo fusionar estados sin duplicar incrementos;
- cuándo la convergencia eventual es suficiente;
- qué reglas de negocio todavía necesitan coordinación fuerte.

## Modelo educativo esperado

El modelo de este curso debe empezar con un G-Counter state-based:

- `ReplicaId`: identidad estable de réplica;
- `Count`: contador no negativo por réplica;
- `GCounter`: mapa de réplica a conteo;
- incremento local;
- consulta de conteo por réplica;
- valor total como suma de componentes;
- fusión por máximo componente a componente;
- pruebas de idempotencia, conmutatividad, asociatividad y convergencia.

El objetivo no es cubrir toda la familia CRDT de una vez. El objetivo es
aprender la primera idea estable: si el estado solo crece y el merge conserva el
máximo observado por componente, las réplicas pueden converger aunque se
sincronicen tarde.

## Invariantes

El capítulo debe hacer visibles estas reglas:

- una réplica solo incrementa su propio componente;
- un G-Counter no representa decrementos;
- fusionar estados conserva el máximo por réplica;
- fusionar el mismo estado dos veces no duplica incrementos;
- el orden de fusión no cambia el estado final;
- el agrupamiento de fusiones no cambia el estado final;
- dos réplicas que reciben los mismos estados convergen;
- después de fusionar no se pierde ningún incremento observado.

## Alternativas

### Coordinación fuerte

Consenso, transacciones distribuidas o un líder único pueden serializar
escrituras. Esa opción es necesaria para algunas reglas, pero reduce
disponibilidad cuando hay fallas o particiones.

### Last write wins

Elegir la versión con timestamp mayor es fácil de implementar, pero puede borrar
trabajo legítimo y depender de relojes físicos.

### Resolución manual

Guardar conflictos para que una persona o capa de aplicación decida puede ser
honesto, pero resulta excesivo para estructuras que tienen una fusión segura.

### CRDT

Es el modelo elegido para este capítulo porque enseña convergencia por diseño.
En vez de resolver conflictos después como casos especiales, se define una
operación de merge que conserva invariantes matemáticas.

## Costos

CRDTs tienen precio:

- cada réplica necesita identidad estable;
- el estado puede crecer con el número de réplicas;
- deletes y decrementos requieren modelos más complejos;
- convergencia eventual no garantiza lectura inmediata de la última escritura;
- reglas no monotónicas pueden requerir coordinación;
- compactar metadatos sin romper invariantes es difícil.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- incremento offline;
- mensaje duplicado;
- fusión fuera de orden;
- merge incorrecto por suma en vez de máximo;
- pérdida de identidad de réplica;
- prometer decrementos usando un G-Counter;
- confundir convergencia eventual con consistencia fuerte.

## Límites

Este capítulo no promete:

- decrementos;
- borrados;
- sets observados con tombstones;
- resolución de conflictos arbitrarios;
- compaction de metadatos;
- causalidad completa;
- red real;
- persistencia real;
- API de producción.

Primero se aprende convergencia monotónica. Después se estudian CRDTs más ricos,
sus costos de metadatos y los casos donde todavía conviene coordinar.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/05-locks-distribuidos.md`
- `docs/06-vector-clocks.md`
- `docs/07-lamport-clocks.md`
- `docs/superpowers/specs/2026-07-20-crdts-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de CRDTs con un
G-Counter state-based, incrementos locales y fusión por máximo.
