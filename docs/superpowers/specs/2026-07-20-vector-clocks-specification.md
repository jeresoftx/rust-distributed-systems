# Especificación de Vector clocks

## Issue

Este documento traza el issue #24: `[06] Definir especificación e invariantes de
Vector clocks`.

## Concepto

Un vector clock es una estructura de tiempo lógico que permite comparar eventos
en un sistema distribuido sin inventar un reloj global perfecto.

Cada nodo mantiene un contador propio y conserva la mejor versión conocida de
los contadores de otros nodos. Esa colección de contadores permite responder
una pregunta esencial: ¿un evento causó a otro, ocurrió después de otro o ambos
son concurrentes?

La idea central no es medir segundos. La idea central es representar evidencia
causal.

## Problema

En sistemas distribuidos, dos eventos pueden ocurrir sin que exista una relación
causal entre ellos. Si el sistema fuerza un orden total artificial, puede
ocultar conflictos reales o sobrescribir trabajo legítimo.

El problema aparece cuando el sistema necesita:

- detectar si una versión deriva de otra;
- distinguir actualización vieja de actualización concurrente;
- fusionar conocimiento recibido desde otro nodo;
- explicar conflictos sin depender de timestamps físicos;
- construir CRDTs, replicación eventual o resolución de versiones.

Un timestamp físico puede decir "esto ocurrió a las 10:00:01", pero no prueba
causalidad. Dos máquinas pueden tener relojes desfasados, y un evento con hora
menor puede haber observado causalmente a otro con hora mayor.

## Alternativas consideradas

### Timestamp físico

Es fácil de guardar y ordenar, pero depende de relojes sincronizados. Sirve para
telemetría y experiencia humana, no como prueba confiable de causalidad.

### Contador local

Un contador local ordena eventos dentro de un nodo, pero no puede comparar
eventos entre nodos sin información adicional.

### Lamport clock

Un Lamport clock da un orden lógico compatible con causalidad: si A causó B,
entonces el reloj de A es menor que el de B. Pero el inverso no se cumple; dos
valores distintos no demuestran causalidad. Es útil para ordenar, no para
detectar concurrencia con precisión.

### Vector clock

Un vector clock conserva un contador por nodo observado. Permite distinguir:

- `Before`: el reloj A está incluido en B y B sabe algo más;
- `After`: A sabe todo lo de B y algo más;
- `Equal`: ambos relojes tienen la misma evidencia;
- `Concurrent`: ninguno contiene al otro.

Es la opción elegida porque enseña causalidad parcial de forma explícita.

## Decisión

El capítulo de Vector clocks debe construir un modelo educativo con estas piezas
mínimas:

- `NodeId`: identidad estable de nodo;
- `Counter`: valor lógico por nodo;
- `CausalRelation`: `Before`, `After`, `Equal` o `Concurrent`;
- `VectorClock`: mapa ordenado de nodo a contador;
- incremento local;
- fusión por máximo componente a componente;
- comparación causal;
- consulta de contador por nodo;
- historial opcional cuando el capítulo lo necesite para explicar eventos.

El modelo inicial no necesita red real, hilos, relojes físicos, persistencia ni
dependencias externas. Debe permitir escribir historias pequeñas: eventos
locales, recepción de mensajes, fusión de conocimiento, versiones causales y
versiones concurrentes.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Monotonía local:** incrementar un nodo solo aumenta su propio contador.
- **Fusión por máximo:** al fusionar dos relojes, cada componente queda en el
  máximo observado.
- **No se pierde conocimiento:** después de fusionar, el reloj resultante es
  mayor o igual que ambos relojes de entrada.
- **Comparación reflexiva:** un reloj comparado consigo mismo es `Equal`.
- **Antes/después por inclusión:** A es `Before` B si todos sus componentes son
  menores o iguales y al menos uno es estrictamente menor.
- **Concurrencia explícita:** A y B son `Concurrent` si cada uno tiene al menos
  un componente mayor que el otro.
- **Nodos ausentes equivalen a cero:** comparar relojes con conjuntos distintos
  de nodos trata componentes ausentes como `Counter(0)`.
- **Historia causal observable:** un mensaje recibido debe fusionar el reloj
  remoto antes de registrar el evento local de recepción, si el modelo incluye
  eventos.

## Límites

El capítulo no promete:

- orden total de todos los eventos;
- medición de tiempo físico;
- sincronización de relojes reales;
- resolución automática de conflictos;
- CRDT completo;
- persistencia real;
- red real;
- API de producción.

Estos límites evitan confundir causalidad parcial con orden global.

## Costos

El capítulo debe hacer visibles estos costos:

- el tamaño del reloj crece con el número de nodos observados;
- cada mensaje puede cargar metadatos adicionales;
- comparar relojes exige revisar componentes;
- nodos que entran y salen complican poda y compactación;
- detectar concurrencia no resuelve el conflicto por sí mismo;
- conservar precisión causal puede costar más que usar un timestamp simple.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- dos nodos editan el mismo valor de forma concurrente;
- un nodo recibe un mensaje con conocimiento atrasado;
- un reloj pierde componentes y no puede probar causalidad;
- un sistema interpreta `Concurrent` como si fuera `Before`;
- una fusión omite el máximo por componente;
- la cantidad de nodos hace crecer demasiado el metadato.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Elección de líder y locks distribuidos hablan de autoridad temporal; vector
  clocks hablan de evidencia causal entre eventos.
- Lamport clocks aparecerán como una alternativa más compacta para ordenar, pero
  menos precisa para detectar concurrencia.
- CRDTs usarán causalidad para explicar convergencia y conflictos.
- Transacciones distribuidas usarán relaciones causales para razonar sobre
  visibilidad, orden y versiones.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Vector clocks en estado `draft`, pero no agrega código Rust ni marca el capítulo
como implementado, probado, revisado o publicado.
