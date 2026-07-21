# Especificación de Transacciones distribuidas

## Issue

Este documento traza el issue #48: `[12] Definir especificación e invariantes
de Transacciones distribuidas`.

## Concepto

Una transacción distribuida coordina una decisión que afecta a más de un
participante. La dificultad no está en llamar a dos servicios. La dificultad
está en decidir qué significa "terminar" cuando un participante puede aceptar,
otro puede fallar, la red puede partirse y los mensajes pueden repetirse.

La pregunta central del capítulo no es "cómo hacemos exactly-once". La pregunta
central es qué invariantes necesita proteger el sistema y qué costo estamos
dispuestos a pagar: bloqueo, compensación, idempotencia, reintentos,
observabilidad o consistencia fuerte.

## Problema

En sistemas reales, una operación de negocio suele tocar varias fronteras:
reservar inventario, cobrar, emitir boleto, registrar auditoría, enviar una
notificación o actualizar una proyección. Si una parte confirma y otra falla,
el sistema queda ante decisiones incómodas.

El problema aparece cuando el sistema necesita:

- coordinar una decisión entre varios participantes;
- evitar doble confirmación por mensajes repetidos;
- distinguir preparación, confirmación, aborto y compensación;
- tolerar fallas parciales sin ocultar estado incierto;
- comparar bloqueo coordinado con compensación por saga;
- explicar por qué exactly-once suele construirse con idempotencia,
  deduplicación y registros durables.

## Alternativas consideradas

### Commit en una sola base local

Una transacción local es clara y fuerte dentro de un solo motor. El límite
aparece cuando la operación cruza procesos, bases de datos, colas o servicios.

### Two-phase commit

2PC separa preparación y decisión. Enseña atomicidad coordinada: si todos
preparan, el coordinador puede confirmar; si alguien rechaza, aborta. Su costo
principal es el bloqueo cuando participantes preparados esperan una decisión.

### Saga

Una saga divide el trabajo en pasos confirmados localmente y compensaciones.
Acepta que no todo será atómico en sentido fuerte. Su costo principal es que la
compensación no siempre equivale a deshacer el mundo real.

### Idempotencia y deduplicación

La idempotencia permite reintentar sin duplicar efectos. No reemplaza atomicidad,
pero hace practicables los reintentos, consumidores de colas, webhooks y flujos
de pago.

### Exactly-once como diseño compuesto

Exactly-once no debe enseñarse como propiedad mágica de red. En la práctica se
aproxima mediante identificadores estables, registros de procesamiento,
idempotencia, deduplicación, commits transaccionales cuando existen y
observabilidad de casos inciertos.

## Decisión

El capítulo debe construir primero un modelo educativo pequeño con dos familias
de decisiones:

- **2PC educativo:** participantes que preparan, confirman o abortan una
  transacción coordinada.
- **Saga educativa:** pasos que aplican efectos locales y ejecutan
  compensaciones en orden inverso cuando un paso falla.

El modelo inicial debe exponer:

- `TransactionId`: identidad estable de operación;
- `ParticipantId`: identidad estable de participante;
- `ParticipantVote`: voto de preparación;
- `TwoPhaseCommit`: coordinador educativo;
- `TransactionDecision`: decisión final observable;
- `SagaStep`: paso con efecto y compensación;
- `Saga`: ejecutor determinista de pasos;
- `SagaOutcome`: resultado aplicado, compensado o fallido;
- registro de eventos para explicar la historia.

El modelo inicial no necesita red real, WAL, locks de base de datos, colas,
timeouts físicos, recuperación durable ni dependencias externas. Debe permitir
historias pequeñas: todos preparan y confirman, un participante rechaza y se
aborta, una saga falla a mitad y compensa lo ya aplicado, un comando repetido
se deduplica por identidad.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Identidad estable:** la misma `TransactionId` representa la misma operación
  lógica para reintentos y deduplicación.
- **Atomicidad de decisión en 2PC:** una transacción 2PC termina en commit solo
  si todos los participantes preparan.
- **Aborto por rechazo:** si un participante rechaza preparar, la decisión final
  es abort.
- **Participante preparado espera decisión:** un participante preparado no debe
  inventar commit local sin decisión del coordinador.
- **Idempotencia por transacción:** procesar de nuevo una transacción ya
  decidida devuelve la misma decisión y no duplica efectos.
- **Compensación inversa en saga:** cuando una saga falla, compensa los pasos
  ya aplicados en orden inverso.
- **Compensación explícita:** compensar no borra la historia; registra una
  acción correctiva observable.
- **Exactly-once pragmático:** el modelo debe enseñar deduplicación e
  idempotencia, no prometer entrega exactamente una vez por la red.

## Límites

El capítulo no promete:

- red real;
- logs durables;
- recuperación después de reinicio;
- locks reales de base de datos;
- consenso de coordinador;
- integración con colas;
- aislamiento serializable;
- dinero real ni efectos irreversibles;
- exactly-once absoluto;
- seguridad contra participantes maliciosos.

Estos límites evitan enseñar transacciones distribuidas como una receta
universal. El capítulo enseña decisiones, costos e invariantes; los motores,
colas y plataformas reales pertenecen a cursos posteriores.

## Costos

El capítulo debe hacer visibles estos costos:

- 2PC puede bloquear si el coordinador falla después de preparar participantes;
- las sagas pueden dejar efectos visibles mientras compensan;
- una compensación puede fallar o no ser equivalente a deshacer;
- la idempotencia exige guardar identificadores procesados;
- deduplicar consume memoria o almacenamiento;
- exactly-once práctico exige diseño de extremo a extremo;
- mayor consistencia suele reducir disponibilidad durante fallas parciales;
- el estado incierto debe observarse y operarse.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- un participante rechaza preparar;
- un participante queda preparado esperando decisión;
- el coordinador decide abortar;
- una saga falla en un paso intermedio;
- una compensación corre en orden incorrecto;
- un mensaje de commit se repite;
- un comando de pago se reintenta con la misma identidad;
- el sistema confunde "mensaje entregado una vez" con "efecto aplicado una vez".

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Consenso y Raft explican cómo decidir de forma replicada; 2PC no reemplaza
  consenso del coordinador.
- Teorema CAP explica por qué una partición obliga a elegir entre avanzar o
  preservar una verdad fuerte.
- Locks distribuidos muestran otra forma de coordinar acceso, con sus propios
  riesgos de expiración y fencing.
- CRDTs y gossip sirven cuando la convergencia eventual es aceptable.
- `rust-database-internals` profundiza en WAL, recovery, MVCC y atomicidad local.
- `rust-system-design` compondrá estas piezas en sistemas como pagos, reservas
  y procesamiento de eventos.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Transacciones distribuidas en estado `draft`, pero no agrega código Rust ni
marca el capítulo como implementado, probado, revisado o publicado.
