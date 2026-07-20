# Especificación de Consenso

## Issue

Este documento traza el issue #4: `[01] Definir especificación e invariantes de
Consenso`.

## Concepto

Consenso es el problema de lograr que varios nodos acuerden una decisión común
aunque algunos participantes fallen o la red entregue mensajes de forma
imperfecta.

La palabra "acuerdo" es deliberadamente estricta: no basta con que un líder
crea haber decidido, ni con que la mayoría haya visto una propuesta por un
momento. Un modelo de consenso debe decir cuándo una decisión queda aceptada,
quién puede observarla y qué evita que dos decisiones incompatibles parezcan
válidas al mismo tiempo.

## Problema

En una sola máquina, un programa puede guardar un valor y leerlo después desde
la misma memoria. En un sistema distribuido no existe esa memoria común. Cada
nodo conserva estado local, recibe mensajes en distinto orden y puede fallar
antes de avisar lo que sabe.

El problema de consenso aparece cuando el sistema necesita una decisión única:

- qué comando ocupa una posición de un log replicado;
- qué nodo actúa como líder de una ronda;
- qué configuración de clúster queda activa;
- qué valor se considera confirmado después de varias propuestas.

Sin consenso, dos nodos pueden avanzar con historias incompatibles. Eso rompe
invariantes de sistemas que dependen de un orden común, como logs replicados,
locks distribuidos, transacciones coordinadas o servicios configurados por
mayoría.

## Alternativas consideradas

### Decisión centralizada

Un nodo decide y los demás obedecen.

Es simple y útil como punto de partida, pero no resuelve consenso cuando ese
nodo falla, queda aislado o pierde mensajes. El curso puede usarlo para
contrastar con protocolos reales, no como solución suficiente.

### Mayoría sin historial

Un nodo propone un valor y lo considera aceptado cuando recibe respuestas de la
mayoría.

Esta alternativa enseña quórums, pero es incompleta: si no se conserva
historial, rondas, términos o promesas, una mayoría posterior puede aceptar un
valor incompatible. Sirve como modelo mínimo para mostrar el peligro de contar
votos sin memoria.

### Consenso por log replicado

Los nodos no acuerdan un valor aislado, sino una secuencia de entradas.

Es la forma que conecta con Raft y con sistemas reales como logs replicados,
pero introduce más piezas: índice, término, commit index, líder y followers. Se
deja para el capítulo de Raft.

### Consenso por propuesta y promesa

Los nodos coordinan rondas con propuestas, promesas y aceptaciones.

Es la puerta natural hacia Paxos. Enseña la esencia teórica del consenso, pero
puede resultar menos intuitiva como primer modelo del curso. Se deja para el
capítulo de Paxos.

## Decisión

El capítulo de Consenso será conceptual y preparatorio. No intentará implementar
Raft ni Paxos completos. Su modelo educativo inicial debe representar:

- un conjunto finito de nodos;
- propuestas identificables;
- votos o aceptaciones por nodo;
- regla de decisión por quórum mayoritario;
- historial observable de eventos;
- fallas explícitas de nodo o mensaje;
- una frontera clara entre "propuesto", "aceptado por algunos" y "decidido".

El objetivo es que el alumno entienda por qué consenso es difícil antes de
estudiar protocolos concretos. Raft y Paxos aparecen después como respuestas
estructuradas a las debilidades de los modelos simples.

## Invariantes

El modelo educativo de consenso debe declarar y probar estas invariantes:

- **Identidad única:** cada nodo tiene un identificador único dentro de la
  simulación.
- **Una aceptación por nodo y ronda:** un nodo no puede aceptar dos valores
  incompatibles dentro de la misma ronda lógica.
- **Decisión por quórum:** un valor solo se considera decidido cuando alcanza el
  quórum configurado.
- **No contradicción visible:** si el modelo declara un valor como decidido, no
  debe declarar otro valor incompatible como decidido bajo las mismas reglas.
- **Historial explicable:** toda decisión debe poder reconstruirse desde los
  eventos registrados.
- **Fallas explícitas:** una pérdida de mensaje, caída de nodo o partición debe
  aparecer en el escenario de prueba, no esconderse dentro de la implementación.

## Límites

El capítulo no promete:

- tolerar fallas bizantinas;
- resolver consenso en redes asincrónicas sin supuestos adicionales;
- modelar almacenamiento persistente;
- implementar elección de líder completa;
- implementar Raft o Paxos;
- ofrecer una API de producción.

Estos límites no son debilidades del curso. Son la frontera que permite enseñar
el problema antes de construir protocolos más fuertes.

## Costos

Incluso el modelo mínimo debe hacer visibles algunos costos:

- más nodos aumentan el número de mensajes necesarios para decidir;
- exigir quórum mejora seguridad, pero reduce disponibilidad durante fallas;
- guardar historial facilita explicar decisiones, pero aumenta memoria;
- permitir reintentos exige distinguir propuestas repetidas de propuestas
  nuevas;
- detectar fallas por timeout introduce ambigüedad entre nodo caído, nodo lento
  y red lenta.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- un nodo cae antes de votar;
- un nodo cae después de votar;
- un mensaje de propuesta se pierde;
- un mensaje de aceptación se duplica;
- una partición deja a un grupo sin quórum;
- dos propuestas compiten por el mismo espacio de decisión.

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos posteriores

- Raft toma el problema de consenso y lo vuelve más operacional mediante líder,
  términos y log replicado.
- Paxos toma el problema de consenso y lo expresa con propuestas, promesas,
  aceptaciones y quórums.
- Elección de líder usa consenso o mecanismos emparentados para decidir quién
  coordina temporalmente.
- Locks distribuidos dependen de acuerdos o leases para evitar doble posesión.
- Transacciones distribuidas usan coordinación para decidir commit o abort.

## Estado

Este issue define especificación e invariantes. No agrega código Rust ni marca
el capítulo como implementado, probado, revisado o publicado.
