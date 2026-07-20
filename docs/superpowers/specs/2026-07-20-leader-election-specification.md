# Especificación de Elección de líder

## Issue

Este documento traza el issue #16: `[04] Definir especificación e invariantes de
Elección de líder`.

## Concepto

Elección de líder es el problema de lograr que un grupo de nodos acuerde quién
coordina temporalmente una tarea sin depender de una autoridad central perfecta.

El líder no es un rey permanente ni una fuente absoluta de verdad. Es una
responsabilidad acotada: durante una época o término, un nodo puede coordinar
trabajo mientras las reglas del sistema permitan reconocerlo como vigente.

## Problema

En una sola máquina, elegir un coordinador puede ser una variable local. En un
sistema distribuido, cada nodo observa mensajes en distinto orden, puede quedar
aislado y puede creer que otro nodo sigue vivo cuando ya no responde.

El problema aparece cuando el sistema necesita:

- decidir quién coordina una ronda de replicación;
- evitar dos coordinadores válidos dentro de la misma época;
- detectar liderazgo obsoleto;
- recuperar coordinación después de una falla;
- explicar por qué un nodo ganó o perdió una elección.

Sin una regla clara, dos particiones pueden actuar como si tuvieran autoridad.
Eso rompe capas superiores: logs replicados, locks, cambios de configuración o
coordinación de transacciones.

## Alternativas consideradas

### Líder fijo

Un nodo predeterminado coordina siempre.

Es simple y útil para laboratorios pequeños, pero convierte su caída en un
punto único de falla. No enseña recuperación ni autoridad temporal.

### Mayor identificador activo

El nodo activo con identificador más alto se vuelve líder.

Es fácil de simular y conecta con algoritmos tipo bully. Su debilidad es que
necesita una noción observable de "activo" y puede producir cambios frecuentes
si la detección de fallas es ruidosa.

### Votación por mayoría

Un candidato solicita votos y gana si alcanza quórum.

Es la opción elegida para el modelo educativo porque se conecta con Raft sin
reimplementar todo Raft: términos, votos, quórum, liderazgo vigente y rechazo de
mensajes obsoletos.

### Detector externo de fallas

Un servicio separado decide qué nodo está vivo y quién coordina.

Puede ser práctico en algunas arquitecturas, pero oculta el mecanismo que este
capítulo debe enseñar. Se deja como comparación, no como base del modelo.

## Decisión

El capítulo de Elección de líder debe construir un modelo educativo de votación
por mayoría con estas piezas mínimas:

- `NodeId`: identidad estable de nodo;
- `ElectionTerm`: época lógica monótona;
- `LeadershipRole`: `Follower`, `Candidate` o `Leader`;
- `Election`: escenario determinista con nodos, términos, votos e historial;
- eventos observables para inicio de elección, voto concedido, líder elegido y
  rechazo;
- errores explícitos para nodo desconocido, término obsoleto y doble voto.

El modelo inicial no necesita red real, hilos, timeouts físicos, leases ni
persistencia. Debe permitir escribir historias pequeñas: candidato sin mayoría,
candidato que alcanza quórum, votante que rechaza doble voto y líder obsoleto
que ya no puede considerarse vigente.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Identidad única:** cada nodo tiene un identificador único en la elección.
- **Términos monótonos:** ningún nodo reduce su término local.
- **Un voto por término:** un nodo no concede dos votos distintos en el mismo
  término.
- **Liderazgo por mayoría:** un candidato solo se vuelve líder si alcanza
  quórum.
- **Un líder vigente por término observable:** no puede haber dos líderes
  legítimos en el mismo término con la misma mayoría.
- **Rechazo de término viejo:** mensajes de términos menores no cambian el
  liderazgo vigente.
- **Pérdida explícita de disponibilidad:** un nodo marcado como no disponible no
  puede votar hasta recuperarse.
- **Historia explicable:** toda elección debe reconstruirse desde eventos
  observables.

## Límites

El capítulo no promete:

- detección perfecta de fallas;
- leases con relojes físicos;
- consenso completo;
- Raft completo;
- persistencia real;
- tolerancia a fallas bizantinas;
- red real;
- API de producción.

Estos límites evitan confundir elección de coordinador con consenso completo.

## Costos

El capítulo debe hacer visibles estos costos:

- una elección requiere mensajes de solicitud y respuesta de voto;
- exigir mayoría mejora seguridad, pero reduce disponibilidad durante
  particiones;
- mantener términos y votos consume estado local;
- detectar nodos caídos implica distinguir falla real de lentitud;
- cambios frecuentes de líder pueden degradar progreso aunque preserven
  seguridad.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- candidato no alcanza mayoría;
- votante intenta votar dos veces en el mismo término;
- nodo caído no puede votar;
- líder viejo intenta operar después de un término mayor;
- partición deja a un grupo sin quórum;
- recuperación de nodo después de una elección.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos posteriores

- Locks distribuidos usarán liderazgo o leases para hablar de propiedad
  temporal.
- Lamport clocks y vector clocks separarán liderazgo de orden causal.
- Transacciones distribuidas usarán coordinadores que pueden fallar.
- System Design usará elección de líder como pieza explícita en servicios
  replicados.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Elección de líder en estado `draft`, pero no agrega código Rust ni marca el
capítulo como implementado, probado, revisado o publicado.
