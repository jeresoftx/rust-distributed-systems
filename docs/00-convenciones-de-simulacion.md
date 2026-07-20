# Convenciones de simulación

Los capítulos de este curso usan modelos pequeños y deterministas. La meta no
es simular internet con fidelidad física, sino tener un laboratorio donde las
ideas distribuidas se puedan observar, probar y discutir sin ruido accidental.

## Principio rector

Una simulación distribuida educativa debe responder una pregunta técnica a la
vez. Si el capítulo estudia consenso, la simulación no debe esconder el acuerdo
detrás de una biblioteca. Si estudia relojes lógicos, la simulación no debe
depender de tiempo real. Si estudia particiones, la falla debe ser visible en el
modelo y en los tests.

## Representación mínima

Todo modelo puede extender estas piezas, pero debe partir de ellas o explicar
por qué necesita otra forma:

- **Nodo:** entidad con identificador estable, estado local y capacidad de
  emitir o recibir mensajes.
- **Mensaje:** dato explícito con emisor, receptor, contenido y metadatos
  necesarios para el capítulo.
- **Evento:** hecho observable dentro de la simulación: enviar, entregar,
  descartar, duplicar, retrasar, fallar o recuperar.
- **Reloj lógico:** valor controlado por la simulación, no por el reloj físico
  del sistema operativo.
- **Red simulada:** cola o agenda determinista que decide qué mensajes se
  entregan y en qué orden.
- **Falla:** condición declarada en el modelo: nodo caído, mensaje perdido,
  duplicado, retraso, partición o recuperación.

## Invariantes comunes

Estas invariantes aplican a los modelos del curso salvo que un capítulo declare
una excepción:

- Cada nodo tiene identidad única dentro de una simulación.
- Un mensaje conserva emisor y receptor desde que se crea hasta que se entrega o
  se descarta.
- Una falla no se inventa dentro de la implementación; se programa desde el
  escenario de prueba.
- Un test no depende de dormir hilos, esperar tiempo real ni competir contra el
  planificador del sistema operativo.
- El orden de entrega de mensajes es reproducible.
- Todo resultado importante puede explicarse a partir del historial de eventos.
- Si un modelo ignora una falla posible, debe declararlo como límite.

## Tiempo lógico

El curso usa tiempo lógico como herramienta de razonamiento. El tiempo físico
solo aparece cuando el capítulo lo necesita para hablar de timeouts o latencia,
y aun así debe abstraerse para que las pruebas sigan siendo deterministas.

Reglas:

- No usar `std::time::Instant::now()` como fuente directa de verdad en modelos
  educativos.
- Preferir contadores, ticks o marcas lógicas controladas por la simulación.
- Documentar si una marca de tiempo significa orden local, orden causal,
  intento, término, versión o vencimiento.
- No prometer orden total cuando el mecanismo solo entrega orden parcial.

## Entrega de mensajes

La red simulada puede ofrecer distintas políticas según el capítulo:

- entrega inmediata;
- entrega en orden de inserción;
- entrega elegida manualmente por el test;
- pérdida explícita;
- duplicación explícita;
- retraso por ticks;
- partición entre subconjuntos de nodos.

La política debe ser parte del escenario, no una sorpresa oculta. Un test bueno
de sistemas distribuidos se lee como una historia: se crea el mundo, se envían
mensajes, se introducen fallas, se entregan eventos y se observan invariantes.

## Fallas

Las fallas se modelan de forma explícita y acotada. El curso empieza con fallas
simples porque son más pedagógicas:

- nodo detenido;
- nodo recuperado;
- mensaje perdido;
- mensaje duplicado;
- mensaje retrasado;
- partición de red;
- recuperación de partición.

Antes de agregar una falla más compleja, el capítulo debe explicar qué pregunta
nueva permite responder.

## Tests deterministas

Todo comportamiento importante debe poder probarse sin azar no controlado. Si
un capítulo usa aleatoriedad para ilustrar gossip, backoff o elección, debe
inyectar una semilla o una fuente determinista.

Un test determinista debe:

- construir el estado inicial;
- ejecutar pasos explícitos;
- observar estado final e invariantes;
- evitar esperas reales;
- fallar por una razón reproducible.

## Frontera de dependencias

La biblioteca estándar de Rust es suficiente para los primeros modelos. Antes
de agregar una dependencia externa, el capítulo debe justificar qué enseña mejor
esa dependencia y qué mecanismo deja de implementar el curso.

No se agregan runtimes, motores de simulación, crates de consenso ni librerías
de red para esconder el mecanismo central. Si se usan en un capítulo avanzado,
deben aparecer después de una versión pequeña construida desde cero.

## Uso en capítulos

Cada capítulo puede especializar estas convenciones. Por ejemplo:

- Raft necesita términos, líder, followers y log replicado.
- Paxos necesita propuestas, promesas, aceptaciones y quórums.
- CRDTs necesita merges conmutativos, asociativos e idempotentes.
- Gossip necesita rondas, peers seleccionados y propagación gradual.
- Transacciones distribuidas necesita coordinador, participantes, preparación,
  confirmación, cancelación e idempotencia.

La convención común evita que cada capítulo reinvente el mundo antes de enseñar
su idea central.
