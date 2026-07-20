# Especificación de Teorema CAP

## Issue

Este documento traza el issue #36: `[09] Definir especificación e invariantes
de Teorema CAP`.

## Concepto

El teorema CAP enseña una restricción de diseño en sistemas distribuidos: cuando
existe una partición de red, el sistema no puede garantizar al mismo tiempo
consistencia fuerte y disponibilidad para todas las operaciones.

La idea central no es "elige dos letras". La idea central es preguntar qué debe
hacer una réplica cuando no puede comunicarse con otra parte necesaria del
sistema y, aun así, recibe una solicitud.

## Problema

En una sola máquina, una escritura puede observar una memoria compartida y
responder con una verdad local. En un sistema distribuido, dos réplicas pueden
quedar separadas por la red. Si ambas aceptan escrituras, pueden producir
estados divergentes. Si alguna deja de aceptar operaciones, conserva una verdad
más estricta pero sacrifica disponibilidad.

El problema aparece cuando el sistema necesita:

- responder durante una partición de red;
- evitar lecturas o escrituras que contradigan una única verdad fuerte;
- explicar por qué algunas operaciones deben rechazarse;
- distinguir disponibilidad de latencia baja;
- distinguir consistencia fuerte de convergencia eventual;
- no convertir CAP en una regla superficial de arquitectura.

## Alternativas consideradas

### Ignorar particiones

Asumir que la red siempre funciona simplifica el diseño, pero oculta el caso que
define al sistema distribuido: nodos vivos que no pueden comunicarse.

### Prometer consistencia y disponibilidad absolutas

Es una promesa atractiva para usuarios, pero falsa bajo partición si el sistema
debe preservar una única verdad fuerte.

### Last write wins

Aceptar escrituras en ambos lados y elegir luego una por timestamp mantiene
disponibilidad, pero puede perder trabajo legítimo y no conserva consistencia
fuerte durante la partición.

### Modelo CAP explícito

Es el modelo elegido para este capítulo porque obliga a declarar la decisión:
ante una partición, una operación puede preservar consistencia rechazando
la operación o puede preservar disponibilidad aceptando divergencia temporal. Esa
elección debe explicarse por operación, no por marca comercial de base de datos.

## Decisión

El capítulo debe construir primero un modelo educativo mínimo basado en una
decisión explícita bajo partición:

- `ConsistencyLevel`: consistencia fuerte o eventual;
- `AvailabilityPolicy`: responder siempre que el nodo local esté vivo o
  rechazar cuando no exista quórum;
- `PartitionState`: red saludable o red particionada;
- `OperationKind`: lectura o escritura;
- `CapDecision`: aceptar, rechazar para preservar consistencia o aceptar con
  riesgo de divergencia;
- evaluación determinista de una operación bajo estado de red y política;
- explicación textual de la decisión.

El modelo inicial no necesita red real, almacenamiento, consenso, relojes,
quórums dinámicos ni dependencias externas. Debe permitir historias pequeñas:
red saludable, partición, lectura local, escritura local, rechazo por
consistencia fuerte y aceptación eventual con reconciliación posterior.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Partición explícita:** CAP solo se vuelve una restricción visible cuando la
  red está particionada.
- **Consistencia fuerte bajo partición:** si una operación necesita observar una
  única verdad fuerte y no puede comunicarse con el resto requerido, debe
  rechazar o bloquear.
- **Disponibilidad bajo partición:** si el sistema responde en cada réplica viva
  durante una partición, acepta la posibilidad de divergencia temporal.
- **Red saludable:** sin partición, el modelo no debe fabricar un tradeoff CAP.
- **Decisión por operación:** una lectura, una escritura y una reconciliación
  pueden tener políticas distintas.
- **No confundir eventual con fuerte:** converger después no significa haber
  mantenido consistencia fuerte durante la partición.
- **No confundir respuesta técnica con disponibilidad CAP:** devolver un error
  controlado puede ser correcto para la API, pero sacrifica disponibilidad CAP
  para esa operación si el contrato esperaba completarla en una réplica viva.
- **No hay magia de producto:** ninguna etiqueta de tecnología elimina el
  tradeoff cuando existe partición.

## Límites

El capítulo no promete:

- demostrar formalmente CAP;
- implementar consenso;
- implementar quórums reales;
- simular una red real;
- modelar latencia, timeouts o retries;
- decidir por una base de datos específica;
- clasificar productos como CP, AP o CA de forma absoluta;
- reemplazar diseño de dominio por una tabla de letras.

Estos límites evitan enseñar CAP como trivia y lo devuelven a su lugar útil:
razonamiento sobre decisiones bajo falla.

## Costos

El capítulo debe hacer visibles estos costos:

- preservar consistencia fuerte puede rechazar operaciones durante una
  partición;
- preservar disponibilidad puede crear estados divergentes;
- reconciliar después exige reglas de dominio;
- una operación puede necesitar una política distinta a otra;
- timeouts pueden confundirse con particiones reales;
- clasificar todo un sistema como CP o AP oculta decisiones por ruta crítica.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- el sistema acepta escrituras en ambos lados de una partición y luego detecta
  conflicto;
- el sistema rechaza una escritura para preservar consistencia fuerte;
- una lectura local responde rápido pero devuelve una versión vieja;
- una operación se anuncia como "siempre disponible" aunque depende de un líder
  inaccesible;
- un equipo llama "eventual" a un comportamiento que nunca reconcilia;
- se usa CAP para justificar malas decisiones de producto sin explicar
  invariantes.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Consenso, Raft y Paxos muestran cómo coordinar una verdad fuerte cuando hay
  quórum suficiente.
- Locks distribuidos muestran el costo de coordinar antes de operar.
- Vector clocks y Lamport clocks ayudan a observar orden y causalidad, pero no
  eliminan el tradeoff CAP.
- CRDTs muestran una ruta disponible y convergente para datos monotónicos.
- Transacciones distribuidas mostrarán cuándo la convergencia eventual no basta
  y se necesita coordinación fuerte.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Teorema CAP en estado `draft`, pero no agrega código Rust ni marca el capítulo
como implementado, probado, revisado o publicado.
