# 12. Transacciones distribuidas

> **Estado:** implemented.
>
> El capítulo cuenta con especificación inicial e invariantes documentados.
> También cuenta con modelo Rust mínimo y tests de invariantes. Todavía no
> tiene ejemplos, ejercicios, benchmark, revisión humana ni está marcado como
> `published`.

## Concepto

Una transacción distribuida coordina una decisión que afecta a más de un
participante. El problema no es enviar dos solicitudes. El problema es preservar
invariantes cuando una parte acepta, otra falla, la red se retrasa y los
mensajes pueden repetirse.

La pregunta central no es "cómo garantizamos exactly-once". La pregunta central
es qué significa aplicar un efecto una sola vez cuando el sistema necesita
reintentar, deduplicar, compensar y registrar decisiones bajo incertidumbre.

## Problema

Operaciones como reservas, pagos, facturación, inventario y emisión de boletos
cruzan varias fronteras. Si todo sucede dentro de una sola base local, una
transacción local puede bastar. Cuando la operación cruza servicios, bases,
colas o regiones, el sistema debe elegir entre coordinación fuerte,
compensación, idempotencia y aceptación explícita de estados intermedios.

Este capítulo compara:

- two-phase commit como coordinación bloqueante;
- sagas como pasos locales con compensación;
- idempotencia como herramienta para reintentos seguros;
- exactly-once como diseño compuesto, no como magia de red.

## Invariantes iniciales

El modelo educativo debe hacer visibles estas reglas:

- una transacción tiene identidad estable;
- 2PC confirma solo si todos los participantes preparan;
- si un participante rechaza preparar, la transacción aborta;
- un participante preparado no inventa commit sin decisión;
- reintentar la misma transacción no duplica efectos;
- una saga compensa pasos aplicados en orden inverso;
- una compensación queda registrada como acción observable;
- exactly-once práctico se construye con idempotencia y deduplicación.

## Implementación

El módulo `src/distributed_transactions.rs` implementa un coordinador 2PC
determinista y un ejecutor de sagas educativas. La API expone
`TransactionId`, `ParticipantId`, `ParticipantVote`, `TransactionDecision`,
`TwoPhaseCommit`, `SagaStep`, `Saga`, `SagaOutcome` y eventos observables.

2PC usa una identidad estable para hacer idempotentes los reintentos de una
transacción ya decidida. Saga aplica pasos en orden y, si uno falla, compensa
los pasos ya aplicados en orden inverso.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/01-consenso.md`
- `docs/02-raft.md`
- `docs/05-locks-distribuidos.md`
- `docs/08-crdts.md`
- `docs/09-teorema-cap.md`
- `docs/11-protocolo-gossip.md`
- `docs/superpowers/specs/2026-07-20-distributed-transactions-specification.md`

## Siguiente paso

El siguiente paso natural es escribir el capítulo completo con ejemplos
progresivos, ejercicios, soluciones ejecutables y diagrama Mermaid.
