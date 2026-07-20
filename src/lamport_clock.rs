//! Modelo educativo mínimo de Lamport clocks.
//!
//! El modelo representa tiempo lógico escalar. No implementa red real,
//! persistencia, sincronización física ni un sistema de trazas de producción.

/// Identificador estable de nodo.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Timestamp lógico escalar.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct LamportTimestamp(pub u64);

/// Identificador ordenable de evento.
///
/// El orden total educativo usa primero el timestamp lógico y después el nodo.
/// Ese desempate es determinista, no una prueba adicional de causalidad.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventId {
    /// Timestamp lógico del evento.
    pub timestamp: LamportTimestamp,
    /// Nodo que produjo el evento.
    pub node: NodeId,
}

impl EventId {
    /// Crea un identificador de evento.
    #[must_use]
    pub fn new(timestamp: LamportTimestamp, node: NodeId) -> Self {
        Self { timestamp, node }
    }
}

/// Mensaje educativo con timestamp lógico adjunto.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LamportMessage {
    /// Nodo emisor.
    pub sender: NodeId,
    /// Timestamp observado al enviar.
    pub timestamp: LamportTimestamp,
}

/// Reloj Lamport local de un nodo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LamportClock {
    node: NodeId,
    timestamp: LamportTimestamp,
}

impl LamportClock {
    /// Crea un reloj Lamport en cero para un nodo.
    #[must_use]
    pub fn new(node: NodeId) -> Self {
        Self {
            node,
            timestamp: LamportTimestamp(0),
        }
    }

    /// Nodo dueño del reloj.
    #[must_use]
    pub fn node(&self) -> NodeId {
        self.node
    }

    /// Timestamp lógico actual.
    #[must_use]
    pub fn timestamp(&self) -> LamportTimestamp {
        self.timestamp
    }

    /// Registra un evento local e incrementa el reloj.
    pub fn local_event(&mut self) -> EventId {
        self.tick();
        self.event_id()
    }

    /// Envía un mensaje e incrementa antes de adjuntar el timestamp.
    pub fn send(&mut self) -> LamportMessage {
        self.tick();
        LamportMessage {
            sender: self.node,
            timestamp: self.timestamp,
        }
    }

    /// Recibe un mensaje usando `max(local, remoto) + 1`.
    pub fn receive(&mut self, message: LamportMessage) -> EventId {
        self.timestamp = self.timestamp.max(message.timestamp);
        self.tick();
        self.event_id()
    }

    fn tick(&mut self) {
        self.timestamp.0 += 1;
    }

    fn event_id(&self) -> EventId {
        EventId::new(self.timestamp, self.node)
    }
}

#[cfg(test)]
mod tests {
    use super::{LamportClock, LamportTimestamp, NodeId};

    #[test]
    fn new_clock_starts_at_zero() {
        let clock = LamportClock::new(NodeId(7));

        assert_eq!(clock.node(), NodeId(7));
        assert_eq!(clock.timestamp(), LamportTimestamp(0));
    }
}
