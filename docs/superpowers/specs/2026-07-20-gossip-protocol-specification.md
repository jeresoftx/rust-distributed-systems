# Especificación de Protocolo gossip

## Issue

Este documento traza el issue #44: `[11] Definir especificación e invariantes
de Protocolo gossip`.

## Concepto

Un protocolo gossip propaga información de forma epidémica: cada nodo comparte
lo que sabe con algunos vecinos en rondas sucesivas, y el conocimiento se
extiende sin exigir un coordinador central para cada actualización.

La idea central no es "enviar mensajes al azar". La idea central es que una
noticia pequeña puede llegar a todo el sistema mediante intercambios parciales,
redundantes y tolerantes a fallas temporales.

## Problema

En sistemas distribuidos, no siempre conviene coordinar cada cambio con todos
los nodos. La membresía, métricas, hints, versiones, estado de salud o datos
eventuales pueden propagarse mejor con intercambios repetidos y baratos.

El problema aparece cuando el sistema necesita:

- difundir conocimiento sin líder único;
- tolerar nodos temporalmente caídos;
- aceptar mensajes duplicados sin duplicar estado;
- converger cuando se restauran rutas de comunicación;
- explicar la relación entre fanout, rondas y velocidad de propagación;
- distinguir convergencia eventual de consistencia inmediata.

## Alternativas consideradas

### Broadcast total

Enviar cada cambio a todos los nodos es fácil de razonar, pero caro y frágil
cuando el cluster crece. También hace que cada actualización dependa de muchos
destinos.

### Coordinador central

Un coordinador puede simplificar membresía y distribución, pero concentra carga
y crea una dependencia fuerte. Es útil en algunos diseños, aunque no enseña la
propagación descentralizada.

### Pull periódico

Cada nodo puede consultar a una fuente central o a vecinos conocidos. Reduce
emisiones innecesarias, pero puede tardar más en propagar eventos urgentes y
sigue dependiendo de cómo se eligen fuentes.

### Gossip epidémico

Es el modelo elegido porque enseña propagación eventual con intercambio parcial:
nodos vivos comparten conocimiento con algunos pares, los mensajes duplicados
son seguros y la convergencia aparece por repetición.

## Decisión

El capítulo debe construir primero un modelo educativo mínimo basado en rondas
deterministas:

- `GossipNodeId`: identidad estable de nodo;
- `GossipFact`: hecho propagable;
- `GossipNode`: estado local de conocimiento y disponibilidad;
- `GossipCluster`: conjunto de nodos y rondas de intercambio;
- `Fanout`: cantidad máxima de pares contactados por nodo y ronda;
- intercambio push de hechos conocidos;
- nodos no disponibles que no envían ni reciben;
- mensajes duplicados idempotentes;
- observación de cobertura y convergencia.

El modelo inicial no necesita red real, aleatoriedad real, anti-entropy pull,
probabilidades, relojes físicos, sospecha de fallas ni dependencias externas.
Debe permitir historias pequeñas: un hecho nace en un nodo, se propaga por
rondas, un nodo caído se salta intercambios, el nodo se recupera y converge.

## Invariantes

El modelo educativo debe declarar y probar estas invariantes:

- **Identidad estable:** dos nodos con el mismo `GossipNodeId` representan el
  mismo participante lógico.
- **Conocimiento monótono:** recibir gossip solo agrega hechos, no borra lo ya
  conocido.
- **Idempotencia:** recibir el mismo hecho varias veces no cambia el resultado.
- **Disponibilidad explícita:** un nodo no disponible no envía ni recibe durante
  una ronda.
- **Fanout acotado:** un nodo contacta como máximo `Fanout` pares disponibles
  por ronda.
- **Sin coordinador global:** la propagación ocurre por intercambios locales.
- **Convergencia eventual:** si hay conectividad suficiente durante suficientes
  rondas, los nodos disponibles terminan observando los mismos hechos.
- **Observabilidad:** el modelo debe poder reportar cuántos nodos conocen un
  hecho.

## Límites

El capítulo no promete:

- red real;
- aleatoriedad de producción;
- detección de fallas;
- seguridad contra mensajes maliciosos;
- orden total de eventos;
- consistencia fuerte;
- garantías probabilísticas formales;
- reconciliación de conflictos de negocio;
- control de versiones complejo.

Estos límites evitan enseñar gossip como una solución universal. Gossip propaga
conocimiento eventual; no reemplaza consenso ni transacciones.

## Costos

El capítulo debe hacer visibles estos costos:

- fanout bajo puede tardar más en converger;
- fanout alto aumenta tráfico;
- duplicar mensajes es normal y debe ser seguro;
- nodos con vistas distintas pueden tomar decisiones distintas temporalmente;
- hechos obsoletos necesitan expiración o versionado en sistemas reales;
- la convergencia depende de conectividad suficiente a lo largo del tiempo.

## Modos de falla

El capítulo debe poder describir estos escenarios:

- un nodo no disponible no recibe actualizaciones;
- un hecho se propaga lentamente por fanout bajo;
- mensajes duplicados llegan en rondas distintas;
- particiones temporales retrasan la convergencia;
- nodos recuperados necesitan ponerse al día;
- un sistema confunde "eventualmente visible" con "confirmado por todos".

Cada escenario debe explicar qué garantiza el modelo y qué queda fuera de su
alcance.

## Relación con capítulos anteriores y posteriores

- Consistent hashing puede necesitar gossip para distribuir cambios de
  membresía.
- Teorema CAP explica por qué durante particiones algunos nodos tienen vistas
  distintas.
- CRDTs combinan bien con gossip porque su merge tolera duplicados y desorden.
- Vector clocks y Lamport clocks ayudan a observar causalidad, pero no
  propagan por sí mismos.
- Transacciones distribuidas mostrarán casos donde la propagación eventual no
  basta.

## Estado

Este issue define especificación e invariantes. Abre el capítulo documental de
Protocolo gossip en estado `draft`, pero no agrega código Rust ni marca el
capítulo como implementado, probado, revisado o publicado.
