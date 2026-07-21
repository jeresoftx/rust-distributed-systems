# 11. Protocolo gossip

> **Estado:** implemented.
>
> El capítulo cuenta con especificación inicial, modelo Rust mínimo y tests de
> invariantes. Todavía no tiene ejemplos, ejercicios, benchmark, revisión humana
> ni está marcado como `published`.

## Concepto

Un protocolo gossip propaga información de forma epidémica: cada nodo comparte
lo que sabe con algunos pares y, ronda tras ronda, el conocimiento se extiende
por el sistema.

La pregunta central no es "a quién le mandamos todo", sino "cómo hacemos que el
conocimiento se propague aunque cada nodo hable solo con una parte del cluster".

## Problema

En sistemas distribuidos, muchas señales no necesitan consenso inmediato:
membresía, métricas, hints, versiones, estado de salud y datos eventualmente
consistentes. Pedir coordinación fuerte para cada señal puede ser más caro que
el valor de la señal.

Gossip permite que los nodos propaguen conocimiento parcial de manera repetida,
idempotente y tolerante a fallas temporales.

## Invariantes iniciales

El modelo educativo debe hacer visibles estas reglas:

- recibir gossip solo agrega conocimiento;
- recibir el mismo hecho varias veces no duplica estado;
- un nodo no disponible no envía ni recibe durante una ronda;
- cada nodo contacta como máximo `Fanout` pares por ronda;
- la propagación no depende de un coordinador global;
- si hay conectividad suficiente durante suficientes rondas, los nodos
  disponibles convergen;
- el modelo puede reportar cuántos nodos conocen un hecho.

## Implementación

El módulo `src/gossip.rs` implementa un cluster determinista con `GossipNodeId`,
`GossipFact`, `Fanout`, `GossipCluster`, `GossipContact` y
`GossipRoundReport`. Las rondas usan un snapshot del conocimiento inicial para
que los hechos recibidos se retransmitan hasta la siguiente ronda.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/08-crdts.md`
- `docs/09-teorema-cap.md`
- `docs/10-consistent-hashing.md`
- `docs/superpowers/specs/2026-07-20-gossip-protocol-specification.md`

## Siguiente paso

El siguiente paso natural es escribir el capítulo completo con ejemplos
progresivos, ejercicios, soluciones ejecutables y diagrama Mermaid.
