# Especificación de CRDTs

## Issue

Este documento traza el issue #32: `[08] Definir especificación e invariantes de
CRDTs`.

## Concepto

Un CRDT, Conflict-free Replicated Data Type, es una estructura de datos
replicada que puede aceptar cambios en varias réplicas y converger al mismo
estado sin coordinación central en cada operación.

La idea central no es "evitar conflictos". La idea central es diseñar
operaciones y fusiones para que el conflicto tenga una forma matemática segura:
si las réplicas intercambian su estado suficientes veces, todas llegan al mismo
resultado.

## Problema

En sistemas distribuidos, exigir coordinación fuerte para cada escritura puede
ser demasiado caro o imposible durante particiones de red. Aun así, muchas
aplicaciones necesitan permitir escritura local y sincronización posterior.

El problema aparece cuando el sistema necesita:

- aceptar actualizaciones en réplicas desconectadas;
- fusionar estados sin perder incrementos legítimos;
- tolerar mensajes duplicados o fuera de orden;
- converger sin elegir un "último ganador" arbitrario;
- explicar qué operaciones son seguras sin consenso por escritura.

CRDTs resuelven una parte del problema: hacen que ciertas estructuras converjan
por construcción. También enseñan un límite importante: no toda regla de negocio
puede convertirse en CRDT sin costo, metadatos o restricciones adicionales.

## Alternativas consideradas

### Coordinación fuerte

Consenso, transacciones distribuidas o un líder único pueden serializar
escrituras. Es correcto cuando se necesita una única verdad inmediata, pero
puede sacrificar disponibilidad durante fallas o particiones.

### Last write wins

Elegir la escritura con timestamp mayor es simple, pero puede perder trabajo
legítimo y depende de relojes físicos o desempates artificiales.

### Merge manual

Guardar versiones concurrentes y pedir resolución humana puede ser honesto,
pero no escala para contadores, sets o estados que podrían fusionarse de forma
automática.

### CRDT

Es el modelo elegido para este capítulo porque enseña convergencia por diseño:
estado monótono, fusión idempotente, conmutativa y asociativa, y operaciones que
no dependen de coordinación central para cada actualización.

## Decisión

El capítulo de CRDTs debe construir primero un modelo educativo mínimo basado en
un **G-Counter** state-based:

- `ReplicaId`: identidad estable de réplica;
- `Count`: contador lógico no negativo;
- `GCounter`: mapa de réplica a conteo;
- incremento local por réplica;
- consulta de conteo por réplica;
- valor total como suma de componentes;
- fusión por máximo componente a componente;
- comparación parcial opcional entre estados;
- pruebas de convergencia.

El modelo inicial no necesita deletes, decrements, tombstones, compaction,
persistencia, red real ni dependencias externas. Debe permitir escribir historias
pequeñas: incrementos offline, intercambio de estados, mensajes duplicados,
fusión fuera de orden y convergencia eventual.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Monotonía local:** una réplica solo incrementa su propio componente.
- **Conteos no negativos:** el modelo no representa decrementos en un
  G-Counter.
- **Fusión por máximo:** al fusionar dos estados, cada componente queda en el
  máximo observado.
- **Idempotencia:** fusionar un estado consigo mismo no cambia el resultado.
- **Conmutatividad:** fusionar A con B produce el mismo estado que fusionar B
  con A.
- **Asociatividad:** el agrupamiento de fusiones no cambia el resultado final.
- **Convergencia:** si dos réplicas reciben los mismos estados, aunque sea en
  orden distinto, terminan iguales.
- **No se pierde incremento observado:** después de fusionar, ningún componente
  queda por debajo de lo que cualquiera de los estados de entrada conocía.

## Límites

El capítulo no promete:

- decrementos;
- borrados;
- sets observados con tombstones;
- resolución de conflictos arbitrarios;
- compaction de metadatos;
- causalidad completa;
- red real;
- persistencia real;
- API de producción.

Estos límites evitan confundir CRDTs con magia general para consistencia.

## Costos

El capítulo debe hacer visibles estos costos:

- cada réplica necesita identidad estable;
- el estado puede crecer con el número de réplicas;
- borrar o decrementar exige CRDTs más complejos;
- convergencia eventual no significa lectura inmediatamente consistente;
- reglas de negocio no monotónicas pueden requerir coordinación;
- compactar metadatos sin romper convergencia es difícil.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- una réplica incrementa offline y sincroniza tarde;
- un mensaje de estado se entrega dos veces;
- dos réplicas fusionan en orden distinto;
- una implementación suma componentes en vez de tomar máximo y duplica
  incrementos;
- una réplica pierde su identidad estable;
- un sistema promete decrementos usando un G-Counter y rompe invariantes.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Vector clocks y Lamport clocks explican orden y causalidad; CRDTs usan
  estructura algebraica para converger incluso con concurrencia.
- Locks distribuidos y consenso coordinan antes de aceptar cambios; CRDTs
  aceptan cambios locales y coordinan después mediante merge.
- CAP aparece de forma natural: CRDTs suelen favorecer disponibilidad y
  tolerancia a particiones para ciertos datos.
- Transacciones distribuidas mostrarán cuándo la convergencia eventual no basta
  y se necesita coordinación fuerte.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
CRDTs en estado `draft`, pero no agrega código Rust ni marca el capítulo como
implementado, probado, revisado o publicado.
