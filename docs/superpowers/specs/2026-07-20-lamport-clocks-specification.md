# Especificación de Lamport clocks

## Issue

Este documento traza el issue #28: `[07] Definir especificación e invariantes de
Lamport clocks`.

## Concepto

Un Lamport clock es un reloj lógico escalar que permite ordenar eventos en un
sistema distribuido sin depender de un reloj físico global.

Cada nodo mantiene un contador local. El contador aumenta antes de registrar un
evento local o enviar un mensaje. Cuando un nodo recibe un mensaje, primero
observa el contador remoto, toma el máximo entre su contador local y el remoto,
y después incrementa.

La idea central no es medir tiempo real. La idea central es preservar una regla
causal mínima: si un evento A causó un evento B, entonces el timestamp lógico de
A debe ser menor que el de B.

## Problema

En sistemas distribuidos, los relojes físicos pueden estar desfasados y los
mensajes pueden llegar tarde. Si el sistema usa únicamente timestamps físicos,
puede registrar un evento como "anterior" aunque en realidad haya observado un
mensaje que venía de otro nodo.

El problema aparece cuando el sistema necesita:

- ordenar eventos para depuración, auditoría o trazas;
- registrar que un mensaje recibido ocurrió después del mensaje enviado;
- evitar que un nodo retroceda su noción de tiempo lógico;
- construir una cola determinista de eventos distribuidos;
- explicar por qué un orden lógico no equivale a causalidad completa.

Lamport clocks resuelven una parte del problema: producen timestamps escalares
compatibles con causalidad. También enseñan un límite importante: si el reloj de
A es menor que el de B, eso no prueba que A haya causado B.

## Alternativas consideradas

### Timestamp físico

Es fácil de entender y útil para humanos, pero depende de sincronización entre
máquinas. Sirve para observabilidad, no como prueba confiable de orden causal.

### Contador local

Ordena eventos dentro de un nodo, pero no incorpora conocimiento recibido desde
otros nodos.

### Vector clock

Representa causalidad parcial con más precisión y detecta concurrencia, pero
carga más metadatos: un contador por nodo observado.

### Lamport clock

Es el modelo elegido para este capítulo porque enseña la forma más compacta de
preservar orden lógico compatible con causalidad. Su precisión es menor que la
de vector clocks, y ese límite debe quedar visible.

## Decisión

El capítulo de Lamport clocks debe construir un modelo educativo con estas
piezas mínimas:

- `NodeId`: identidad estable de nodo;
- `LamportTimestamp`: contador lógico escalar;
- `LamportClock`: reloj local de un nodo;
- `EventId`: desempate determinista opcional para ordenar eventos;
- evento local;
- envío de mensaje;
- recepción de mensaje;
- comparación de timestamps;
- orden total educativo mediante `(timestamp, node_id)` cuando el capítulo lo
  necesite para estabilizar salidas.

El modelo inicial no necesita red real, hilos, relojes físicos, persistencia ni
dependencias externas. Debe permitir escribir historias pequeñas: eventos
locales, mensajes enviados, mensajes recibidos, trazas ordenadas y casos donde
dos eventos tienen orden lógico sin relación causal demostrable.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Monotonía local:** el reloj de un nodo nunca retrocede.
- **Evento local incrementa:** registrar un evento local aumenta el contador en
  uno.
- **Envío incrementa:** enviar un mensaje aumenta el contador antes de adjuntar
  el timestamp al mensaje.
- **Recepción observa máximo:** recibir un mensaje usa
  `max(local, remoto) + 1`.
- **Causalidad preservada:** si A causó B mediante evento local, envío o
  recepción, entonces `timestamp(A) < timestamp(B)`.
- **Orden lógico no prueba causalidad inversa:** `timestamp(A) < timestamp(B)`
  no implica que A causó B.
- **Desempate determinista:** si se necesita orden total educativo, el
  desempate debe ser explícito y estable, por ejemplo `(timestamp, node_id)`.
- **Sin tiempo físico:** ningún resultado debe depender de segundos,
  milisegundos ni reloj del sistema operativo.

## Límites

El capítulo no promete:

- detectar concurrencia con precisión;
- representar todo el conocimiento causal;
- resolver conflictos;
- ordenar eventos por tiempo físico;
- sincronización real de relojes;
- red real;
- persistencia real;
- API de producción.

Estos límites evitan confundir un timestamp lógico compacto con una prueba
completa de causalidad.

## Costos

El capítulo debe hacer visibles estos costos:

- un contador escalar pierde información causal;
- dos eventos concurrentes pueden quedar ordenados por accidente;
- el desempate por `NodeId` crea orden determinista, no causalidad real;
- recibir mensajes obliga a transportar timestamps lógicos;
- reinicios o pérdida de estado pueden romper monotonía si no hay persistencia;
- el modelo es compacto, pero menos expresivo que vector clocks.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- dos nodos producen eventos concurrentes con timestamps comparables;
- un nodo recibe un mensaje atrasado y debe avanzar con `max + 1`;
- un sistema interpreta un orden escalar como causalidad probada;
- un reinicio pierde el contador y genera timestamps repetidos o menores;
- dos eventos tienen el mismo timestamp y requieren desempate explícito;
- una implementación recibe mensajes sin actualizar el reloj local.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Vector clocks detectan concurrencia con más precisión; Lamport clocks
  enseñan la alternativa compacta que solo preserva una implicación causal.
- Raft, Paxos y elección de líder usan términos, rondas y epochs; Lamport
  clocks ayudan a separar esos contadores de un reloj lógico general.
- CRDTs usarán causalidad y concurrencia para explicar convergencia.
- Sistemas de auditoría y trazas distribuidas usarán orden lógico para narrar
  eventos sin prometer tiempo físico exacto.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Lamport clocks en estado `draft`, pero no agrega código Rust ni marca el
capítulo como implementado, probado, revisado o publicado.
