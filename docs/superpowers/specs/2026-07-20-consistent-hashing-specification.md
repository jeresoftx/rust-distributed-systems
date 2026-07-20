# Especificación de Consistent hashing

## Issue

Este documento traza el issue #40: `[10] Definir especificación e invariantes
de Consistent hashing`.

## Concepto

Consistent hashing es una técnica para distribuir claves entre nodos de forma
estable cuando el conjunto de nodos cambia. La idea central no es "hacer hash",
sino reducir el movimiento de claves cuando un nodo entra o sale del sistema.

En un hash tradicional, si se calcula `hash(clave) % numero_de_nodos`, agregar
o quitar un nodo puede mover casi todas las claves. En consistent hashing, las
claves y los nodos se ubican en un anillo lógico. Cada clave pertenece al primer
nodo que aparece al avanzar en el anillo desde la posición de la clave.

## Problema

Los sistemas distribuidos necesitan repartir trabajo o datos entre nodos:
cachés, particiones de almacenamiento, colas, índices y servicios con estado.
Ese reparto debe sobrevivir cambios de membresía sin obligar a reubicar todo.

El problema aparece cuando el sistema necesita:

- decidir qué nodo atiende una clave;
- agregar nodos sin mover claves innecesarias;
- quitar nodos sin perder responsabilidad sobre claves;
- explicar por qué un reparto por módulo es frágil;
- razonar sobre balance, movimiento y puntos calientes;
- mantener una regla determinista que todos los clientes puedan reproducir.

## Alternativas consideradas

### Módulo por cantidad de nodos

`hash(clave) % N` es simple y barato, pero sufre movimiento masivo cuando `N`
cambia. Sirve para enseñar el problema, no como modelo principal del capítulo.

### Tabla central de asignación

Una tabla explícita de clave a nodo ofrece control fino, pero exige almacenar,
replicar y actualizar mucho metadato. También puede convertirse en punto central
de coordinación.

### Rango fijo por nodo

Dividir el espacio en rangos manuales es razonable en algunos sistemas, pero
requiere rebalanceos explícitos y decisiones administrativas frecuentes.

### Consistent hashing

Es el modelo elegido porque enseña movimiento acotado con una estructura
pequeña: anillo ordenado, posiciones hash, búsqueda del sucesor y transferencia
local de responsabilidad ante cambios de membresía.

## Decisión

El capítulo debe construir primero un modelo educativo mínimo basado en un
anillo determinista:

- `NodeId`: identidad estable de nodo;
- `Key`: clave lógica de usuario;
- `HashSlot`: posición en el anillo;
- `RingNode`: nodo ubicado en una posición del anillo;
- `ConsistentHashRing`: colección ordenada de nodos;
- inserción y eliminación de nodos;
- asignación de claves al primer nodo sucesor en el anillo;
- vuelta al inicio cuando la clave cae después del último nodo;
- cálculo de claves que cambian de dueño al agregar o quitar un nodo.

El modelo inicial no necesita red real, réplicas virtuales, pesos, persistencia,
gossip de membresía ni dependencias externas. Debe permitir historias pequeñas:
anillo vacío, un nodo, varios nodos, wrap-around, agregar nodo, quitar nodo y
comparar movimiento contra un reparto por módulo.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Identidad estable:** dos nodos con el mismo `NodeId` representan el mismo
  participante lógico.
- **Posición estable:** una misma clave o nodo siempre produce el mismo
  `HashSlot` dentro del modelo.
- **Orden total del anillo:** los nodos se mantienen ordenados por `HashSlot` y
  por `NodeId` como desempate determinista.
- **Asignación por sucesor:** una clave pertenece al primer nodo con posición
  mayor o igual a la posición de la clave.
- **Wrap-around:** si no existe sucesor hacia adelante, la clave pertenece al
  primer nodo del anillo.
- **Anillo vacío:** no existe dueño para una clave cuando no hay nodos.
- **Movimiento acotado al agregar:** al agregar un nodo, solo deben moverse las
  claves que caen en el rango que ese nuevo nodo toma.
- **Movimiento acotado al quitar:** al quitar un nodo, solo deben moverse las
  claves que pertenecían al nodo retirado.
- **Determinismo:** dos anillos con los mismos nodos deben asignar las mismas
  claves al mismo dueño.

## Límites

El capítulo no promete:

- balance perfecto;
- pesos por nodo;
- réplicas virtuales;
- reparación automática;
- detección real de nodos caídos;
- consenso de membresía;
- migración real de datos;
- protección contra claves calientes;
- función hash criptográfica.

Estos límites separan la idea esencial del anillo de los problemas de un
sistema de almacenamiento real.

## Costos

El capítulo debe hacer visibles estos costos:

- el balance depende de la distribución de posiciones;
- pocos nodos pueden producir rangos muy desiguales;
- agregar réplicas virtuales mejora balance, pero aumenta metadatos;
- todos los participantes deben conocer la misma membresía;
- un cambio de membresía todavía mueve datos, aunque menos que módulo por `N`;
- puntos calientes pueden sobrevivir si una clave concentra demasiado tráfico.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- anillo vacío que no puede asignar claves;
- dos nodos con posiciones cercanas que producen rangos desbalanceados;
- clientes con vistas distintas de membresía que enrutan claves a nodos
  distintos;
- retiro de un nodo sin migrar sus claves;
- uso de módulo por `N` que remapea casi todo al cambiar la cantidad de nodos;
- uso de una función hash pobre que concentra claves.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Teorema CAP ayuda a explicar qué ocurre si los clientes no comparten la misma
  vista de membresía durante una partición.
- Vector clocks y Lamport clocks ayudan a razonar sobre eventos de cambio de
  membresía, pero no eligen el dueño de una clave.
- Gossip puede propagar cambios de membresía de forma eventual.
- Transacciones distribuidas aparecen cuando mover claves exige coordinar datos
  entre nodos.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Consistent hashing en estado `draft`, pero no agrega código Rust ni marca el
capítulo como implementado, probado, revisado o publicado.
