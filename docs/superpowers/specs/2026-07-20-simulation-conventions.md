# Convenciones de simulación distribuida

## Issue

Este documento traza el issue #3: definir convenciones de simulación
distribuida.

## Concepto

Una simulación educativa es un laboratorio controlado. No intenta reproducir
todos los detalles de producción; intenta aislar una pregunta para que el lector
pueda observar el mecanismo sin ruido.

En sistemas distribuidos, ese control es especialmente importante porque el
comportamiento depende de tiempo, orden de mensajes y fallas. Si esos elementos
no son deterministas, los tests se vuelven frágiles y la explicación pierde
claridad.

## Problema

Los capítulos futuros necesitan representar nodos, mensajes, eventos, relojes y
fallas. Si cada capítulo inventa su propia manera sin una frontera común, el
curso se vuelve inconsistente: un "mensaje perdido" podría significar cosas
distintas en Raft, gossip o transacciones distribuidas.

## Decisión

El curso usará simulaciones deterministas con:

- nodos identificables;
- mensajes explícitos;
- eventos observables;
- relojes lógicos controlados por el modelo;
- red simulada con política declarada;
- fallas programadas desde el escenario de prueba;
- tests sin esperas reales ni dependencia del planificador del sistema
  operativo.

La documentación pública vive en `docs/00-convenciones-de-simulacion.md` y queda
enlazada desde `docs/SUMMARY.md`.

## Invariantes

- El orden de eventos debe poder reproducirse.
- Cada falla debe ser explícita en el escenario.
- Todo resultado importante debe explicarse desde el historial de eventos.
- Si un capítulo ignora una falla posible, debe declararlo como límite.
- Las dependencias externas no deben ocultar el mecanismo central.

## Estado

Este issue es documental. No agrega módulos Rust ni cambia estados de capítulos.
