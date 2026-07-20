# Glosario base

Este glosario fija el vocabulario mínimo del curso antes de entrar a los
capítulos. No busca reemplazar definiciones académicas completas; busca que cada
modelo Rust use las mismas palabras para hablar de nodos, mensajes, fallas,
tiempo y consistencia.

## Nodo

**Definición breve:** participante independiente del sistema distribuido. Puede
ejecutar código, conservar estado local, enviar mensajes y recibir mensajes.

**Ejemplo:** en un clúster de tres réplicas, cada proceso que mantiene una copia
del estado es un nodo.

**Límite:** un nodo no es necesariamente una máquina física. Puede ser un
proceso, una tarea, un contenedor o una simulación dentro de una prueba.

## Mensaje

**Definición breve:** unidad de comunicación entre nodos. Representa una
intención observable: pedir una lectura, proponer un valor, confirmar una
escritura o propagar estado.

**Ejemplo:** un líder envía un mensaje `AppendEntries` a una réplica para copiar
entradas de log.

**Límite:** un mensaje no garantiza entrega, orden ni unicidad por sí mismo. El
modelo debe declarar si puede perderse, duplicarse, retrasarse o llegar fuera de
orden.

## Réplica

**Definición breve:** nodo que conserva una copia parcial o completa de cierto
estado para mejorar disponibilidad, tolerar fallas o repartir carga.

**Ejemplo:** un primary acepta una escritura y después replica el cambio a dos
followers.

**Límite:** tener réplicas no significa tener consistencia automática. Si dos
réplicas no han visto los mismos mensajes, pueden exponer estados distintos.

## Quórum

**Definición breve:** subconjunto mínimo de nodos requerido para aceptar una
decisión, lectura o escritura.

**Ejemplo:** en un grupo de cinco nodos, exigir tres respuestas crea un quórum
mayoritario.

**Límite:** un quórum no elimina fallas. Solo establece una regla para que las
decisiones tengan intersección suficiente entre operaciones.

## Consenso

**Definición breve:** mecanismo por el cual varios nodos acuerdan un valor o
una decisión, incluso si algunos nodos fallan dentro de los límites del modelo.

**Ejemplo:** un grupo acuerda qué entrada debe ocupar la posición 42 de un log
replicado.

**Límite:** consenso no significa que el sistema nunca falle. Significa que,
bajo supuestos explícitos, los nodos correctos no deciden valores
contradictorios.

## Partición

**Definición breve:** falla de red donde algunos nodos no pueden comunicarse con
otros, aunque sigan vivos y ejecutando código.

**Ejemplo:** tres nodos quedan comunicados entre sí, pero aislados de otros dos
nodos del mismo clúster.

**Límite:** una partición no siempre se distingue de un nodo lento o caído. Esa
ambigüedad es una de las tensiones centrales del diseño distribuido.

## Reloj lógico

**Definición breve:** mecanismo para ordenar eventos sin depender de un reloj
físico global confiable.

**Ejemplo:** un Lamport clock incrementa un contador local y lo propaga en
mensajes para construir un orden causal parcial.

**Límite:** un reloj lógico ayuda a razonar sobre causalidad, pero no mide
tiempo real. Dos eventos con marcas comparables no necesariamente ocurrieron
con una separación física conocida.

## Consistencia

**Definición breve:** contrato que describe qué valores puede observar un
cliente después de lecturas, escrituras, réplicas y fallas.

**Ejemplo:** una lectura linealizable debe observar la escritura confirmada más
reciente según un único orden global.

**Límite:** consistencia no es una propiedad única. Existen modelos más fuertes
o más débiles, y cada uno cambia latencia, disponibilidad y complejidad.

## Disponibilidad

**Definición breve:** capacidad del sistema para responder a una operación,
aunque parte de sus nodos o enlaces fallen.

**Ejemplo:** durante una partición, un sistema puede elegir aceptar escrituras
en ambos lados para seguir disponible.

**Límite:** disponibilidad no dice que la respuesta sea la más fresca ni que
todas las réplicas estén de acuerdo. Solo dice que el sistema responde dentro de
los supuestos definidos.

## Uso en el curso

Cada capítulo puede ampliar estas definiciones, pero no debe contradecirlas sin
explicar la razón. Si un término cambia de significado por contexto, el capítulo
debe declararlo antes de usarlo.
