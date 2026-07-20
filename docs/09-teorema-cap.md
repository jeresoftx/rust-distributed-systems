# 09. Teorema CAP

> **Estado:** tested.
>
> El capítulo cuenta con especificación inicial, modelo Rust mínimo e
> invariantes probadas. Todavía no tiene ejemplos progresivos, ejercicios,
> benchmark ni revisión humana.

## Concepto

El teorema CAP enseña una restricción de diseño: cuando existe una partición de
red, un sistema distribuido no puede garantizar al mismo tiempo consistencia
fuerte y disponibilidad para todas las operaciones.

La pregunta central no es "qué dos letras elegimos", sino "qué debe hacer una
réplica cuando no puede comunicarse con otra parte necesaria del sistema".

## Problema

En un sistema distribuido, una réplica puede estar viva y aun así no poder
hablar con otras réplicas. Si acepta una escritura local, el sistema sigue
respondiendo, pero puede crear una verdad temporalmente distinta. Si rechaza la
operación, protege una única verdad fuerte, pero sacrifica disponibilidad para
esa ruta.

CAP responde preguntas prácticas:

- qué ocurre cuando la red se parte;
- cuándo conviene rechazar una operación;
- cuándo conviene aceptar divergencia temporal;
- por qué converger después no es lo mismo que ser consistente durante la
  partición;
- por qué una tecnología no se clasifica honestamente con solo dos letras.

## Modelo educativo esperado

El modelo de este curso debe empezar con una evaluación explícita de decisiones
bajo partición:

- `ConsistencyLevel`: consistencia fuerte o eventual;
- `AvailabilityPolicy`: responder en réplicas vivas o exigir coordinación;
- `PartitionState`: red saludable o particionada;
- `OperationKind`: lectura o escritura;
- `CapDecision`: aceptar, rechazar para preservar consistencia o aceptar con
  riesgo de divergencia;
- explicación textual de cada decisión.

El objetivo no es demostrar CAP formalmente ni clasificar bases de datos
reales. El objetivo es aprender a formular la decisión incómoda: si la red está
particionada, la operación debe elegir entre responder localmente o preservar
una verdad fuerte que requiere comunicación.

## Invariantes

El capítulo debe hacer visibles estas reglas:

- CAP solo se vuelve visible bajo partición de red;
- sin partición, el modelo no debe fabricar un tradeoff CAP;
- preservar consistencia fuerte bajo partición puede requerir rechazar o
  bloquear operaciones;
- preservar disponibilidad bajo partición puede crear divergencia temporal;
- convergencia posterior no equivale a consistencia fuerte durante la
  partición;
- una lectura puede tener una política distinta a una escritura;
- una respuesta técnica no siempre equivale a disponibilidad CAP;
- ninguna etiqueta de producto elimina el tradeoff.

## Alternativas

### Ignorar particiones

Asumir que la red siempre funciona simplifica el diseño, pero oculta el caso que
define al sistema distribuido: nodos vivos que no pueden comunicarse.

### Prometer consistencia y disponibilidad absolutas

Es una promesa atractiva, pero falsa bajo partición si el sistema debe preservar
una única verdad fuerte.

### Last write wins

Aceptar escrituras en ambos lados y elegir luego una por timestamp mantiene
disponibilidad, pero puede perder trabajo legítimo y no conserva consistencia
fuerte durante la partición.

### CAP explícito

Es el modelo elegido para este capítulo porque obliga a declarar la decisión por
operación. CAP deja de ser eslogan cuando una ruta concreta debe decidir si
responde localmente o rechaza para proteger consistencia.

## Costos

CAP tiene precio:

- preservar consistencia fuerte puede rechazar operaciones durante una
  partición;
- preservar disponibilidad puede crear estados divergentes;
- reconciliar después exige reglas de dominio;
- una operación puede necesitar una política distinta a otra;
- timeouts pueden confundirse con particiones reales;
- clasificar todo un sistema como CP o AP oculta decisiones por ruta crítica.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- aceptar escrituras en ambos lados de una partición;
- rechazar una escritura para preservar consistencia fuerte;
- lectura local que responde rápido pero ve una versión vieja;
- dependencia de un líder inaccesible;
- prometer convergencia eventual sin reconciliación real;
- usar CAP como excusa sin explicar invariantes.

## Límites

Este capítulo no promete:

- demostración formal del teorema;
- consenso real;
- quórums reales;
- red real;
- latencia, timeouts o retries;
- clasificar productos reales como CP, AP o CA de forma absoluta;
- reemplazar diseño de dominio por una tabla de letras.

Primero se aprende a nombrar la decisión bajo partición. Después se estudia cómo
esa decisión aparece en consenso, CRDTs, transacciones distribuidas y diseño de
sistemas completos.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/02-raft.md`
- `docs/03-paxos.md`
- `docs/08-crdts.md`
- `docs/superpowers/specs/2026-07-20-cap-theorem-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de Teorema CAP
que evalúe decisiones bajo partición para consistencia fuerte y disponibilidad.
