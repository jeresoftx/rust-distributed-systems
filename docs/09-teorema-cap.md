# 09. Teorema CAP

> **Estado:** tested.
>
> El capítulo cuenta con especificación inicial, modelo Rust mínimo, tests de
> invariantes, ejemplos progresivos, ejercicios, soluciones ejecutables y
> diagrama Mermaid. Todavía no tiene benchmark, revisión humana ni está marcado
> como `published`.

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

## Diagrama

```mermaid
sequenceDiagram
    participant U as Usuario
    participant A as Réplica A
    participant B as Réplica B

    Note over A,B: Partición de red
    U->>A: solicita escritura
    alt Exigir coordinación
        A-->>U: rechaza para preservar consistencia fuerte
    else Servir réplica local
        A->>A: acepta escritura local
        A-->>U: responde disponible
        Note over A,B: B no observó el cambio; hay divergencia temporal
    end
```

## Modelo educativo esperado

El modelo de este curso empieza con una evaluación explícita de decisiones bajo
partición:

- `ConsistencyLevel`: consistencia fuerte o eventual;
- `AvailabilityPolicy`: responder en réplicas vivas o exigir coordinación;
- `PartitionState`: red saludable o particionada;
- `OperationKind`: lectura o escritura;
- `CapDecision`: aceptar, rechazar para preservar consistencia o aceptar con
  riesgo de divergencia;
- `CapOutcome`: garantías visibles de la decisión;
- explicación textual de cada decisión.

El objetivo no es demostrar CAP formalmente ni clasificar bases de datos reales.
El objetivo es aprender a formular la decisión incómoda: si la red está
particionada, la operación debe elegir entre responder localmente o preservar
una verdad fuerte que requiere comunicación.

## Implementación

El módulo `src/cap.rs` implementa un evaluador determinista de escenarios CAP.
Su API expone una secuencia pequeña:

- crear un `CapScenario`;
- declarar si la red está saludable o particionada;
- declarar si la operación busca consistencia fuerte o eventual;
- declarar si la política exige coordinación o sirve la réplica local;
- declarar si la operación es lectura o escritura;
- evaluar el escenario para obtener un `CapOutcome`.

El resultado no intenta simular una red. Expone señales pedagógicas:

- si el tradeoff CAP es visible;
- si la decisión preserva consistencia fuerte;
- si preserva disponibilidad CAP para la operación;
- si puede producir divergencia temporal;
- qué decisión tomó el modelo.

Uso básico:

```rust
use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind,
    PartitionState,
};

let scenario = CapScenario::new(
    PartitionState::Partitioned,
    ConsistencyLevel::Strong,
    AvailabilityPolicy::RequireCoordination,
    OperationKind::Write,
);

let outcome = scenario.evaluate();

assert_eq!(outcome.decision, CapDecision::RejectToPreserveConsistency);
assert!(outcome.partition_tradeoff_visible);
assert!(outcome.preserves_strong_consistency);
assert!(!outcome.preserves_cap_availability);
```

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

## Ejemplos progresivos

### Básico

`examples/soluciones/cap_basic_healthy_network.rs` muestra una red saludable. La
operación se acepta de forma consistente y el modelo no fabrica un tradeoff CAP.

La lección es que CAP no aparece en cada llamada normal: aparece cuando una
partición impide comunicarse con una parte necesaria.

### Intermedio

`examples/soluciones/cap_intermediate_partition_reject.rs` muestra una escritura
durante partición con política de coordinación. La operación se rechaza para
preservar consistencia fuerte.

La lección es que rechazar puede ser la decisión correcta, pero sacrifica
disponibilidad CAP para esa operación.

### Avanzado

`examples/soluciones/cap_advanced_available_divergence.rs` muestra una operación
eventual que se completa en la réplica local durante una partición.

La lección es que disponibilidad bajo partición viene con costo: divergencia
temporal y reconciliación posterior.

### Caso real

`examples/soluciones/cap_real_reservation_checkout.rs` modela un checkout de
reservas. Confirmar una reserva con inventario fuerte rechaza durante partición;
registrar una intención eventual puede aceptarse localmente y reconciliarse
después.

Este caso conecta CAP con reservas, pagos, inventario, carritos y cualquier
flujo donde una ruta de negocio debe decidir si prefiere rechazar o aceptar
trabajo pendiente de reconciliación.

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

## Ejercicios

### Nivel 1: red saludable

Crea un `CapScenario` con `PartitionState::Healthy`. Evalúalo y verifica que el
resultado sea `CapDecision::AcceptConsistent` y que
`partition_tradeoff_visible` sea `false`.

Solución sugerida:
`examples/soluciones/cap_basic_healthy_network.rs`.

### Nivel 2: rechazo por consistencia

Crea una escritura bajo `PartitionState::Partitioned` con
`ConsistencyLevel::Strong` y `AvailabilityPolicy::RequireCoordination`. Verifica
que el modelo rechace para preservar consistencia fuerte.

Solución sugerida:
`examples/soluciones/cap_intermediate_partition_reject.rs`.

### Nivel 3: disponibilidad con divergencia

Crea una escritura bajo partición con `ConsistencyLevel::Eventual` y
`AvailabilityPolicy::ServeLocalReplica`. Verifica que el modelo preserve
disponibilidad CAP y marque `divergence_possible`.

Solución sugerida:
`examples/soluciones/cap_advanced_available_divergence.rs`.

### Nivel 4: checkout de reservas

Modela dos rutas de checkout: confirmación fuerte de inventario e intención
eventual de reserva. Durante una partición, la primera debe rechazar y la
segunda puede aceptar localmente con riesgo de reconciliación.

Solución sugerida:
`examples/soluciones/cap_real_reservation_checkout.rs`.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/02-raft.md`
- `docs/03-paxos.md`
- `docs/08-crdts.md`
- `docs/superpowers/specs/2026-07-20-cap-theorem-specification.md`

## Siguiente paso

El siguiente paso natural es agregar el benchmark educativo de Teorema CAP y
cerrar el estado del capítulo como `benchmarked`, sin marcarlo como `reviewed`
ni `published` hasta que exista revisión humana.
