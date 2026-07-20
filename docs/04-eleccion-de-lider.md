# 04. Elección de líder

> **Estado:** draft.
>
> El capítulo cuenta con especificación inicial e invariantes. Todavía no tiene
> modelo Rust, tests, ejemplos, ejercicios, benchmark ni revisión humana.

## Concepto

Elección de líder es el mecanismo mediante el cual un grupo de nodos decide qué
nodo coordina temporalmente una tarea. El líder no es una verdad absoluta: es
una autoridad acotada por un término, votos e invariantes.

La pregunta central no es "quién manda", sino "bajo qué evidencia podemos
aceptar que este nodo coordina ahora".

## Problema

En sistemas distribuidos no hay una vista global perfecta. Un nodo puede estar
lento, aislado o caído; otro nodo puede observar mensajes viejos; una partición
puede hacer que dos grupos crean tener autoridad.

Elección de líder responde preguntas prácticas:

- quién coordina una ronda;
- qué hace obsoleto a un líder anterior;
- cuántos votos bastan para reconocer liderazgo;
- qué ocurre si un votante ya eligió a otro candidato;
- cómo explicar por qué una elección ganó o falló.

## Modelo educativo esperado

El modelo de este curso debe representar una elección determinista por mayoría:

- `NodeId`: identidad estable de nodo;
- `ElectionTerm`: época lógica monótona;
- `LeadershipRole`: follower, candidate o leader;
- `LeaderElection`: escenario con nodos, términos, votos, disponibilidad e
  historial;
- solicitud de voto;
- concesión o rechazo de voto;
- elección por quórum;
- rechazo de mensajes obsoletos.

El objetivo no es construir Raft otra vez. El objetivo es aislar la idea de
autoridad temporal para entender qué significa "este nodo puede coordinar
ahora" sin esconder las fallas.

## Invariantes

El capítulo debe hacer visibles estas reglas:

- cada nodo tiene identidad única;
- un término no retrocede;
- un nodo concede como máximo un voto por término;
- un candidato necesita mayoría para ser líder;
- no hay dos líderes legítimos del mismo término sobre la misma mayoría;
- un término viejo no cambia liderazgo vigente;
- un nodo no disponible no puede votar;
- el historial explica inicios de elección, votos, rechazos y liderazgo.

## Alternativas

### Líder fijo

Un coordinador permanente simplifica el camino feliz, pero convierte su caída en
un punto único de falla.

### Mayor identificador activo

Elegir al nodo con identificador más alto es simple, pero depende de detectar
actividad correctamente. Una red lenta puede parecer una falla.

### Votación por mayoría

Es el modelo elegido para este capítulo. Conecta con Raft, pero conserva un
alcance menor: términos, votos, disponibilidad y quórum.

### Detector externo

Un servicio externo puede decidir liderazgo, pero oculta el mecanismo que el
curso necesita estudiar.

## Costos

La elección de líder tiene precio:

- cada elección intercambia mensajes;
- mayoría mejora seguridad, pero limita disponibilidad;
- cada nodo debe recordar votos por término;
- fallas y recuperaciones agregan ruido operacional;
- cambios frecuentes de líder pueden frenar el progreso.

## Modos de falla

El capítulo debe cubrir, como mínimo:

- candidato sin mayoría;
- doble voto en el mismo término;
- voto desde nodo no disponible;
- mensaje de término viejo;
- líder obsoleto;
- recuperación de nodo después de una elección.

## Límites

Este capítulo no promete:

- consenso completo;
- leases con relojes reales;
- persistencia en disco;
- detección perfecta de fallas;
- red real;
- tolerancia a fallas bizantinas;
- API de producción.

Primero se aprende la autoridad temporal. Después se estudia cómo se combina
con logs, locks, clocks y transacciones.

## Referencias internas

- `docs/00-glosario.md`
- `docs/00-convenciones-de-simulacion.md`
- `docs/02-raft.md`
- `docs/superpowers/specs/2026-07-20-leader-election-specification.md`

## Siguiente paso

El siguiente paso natural es implementar un modelo Rust mínimo de elección de
líder con términos, votos, disponibilidad y quórum mayoritario.
