//! Modelo educativo mínimo de Vector clocks.
//!
//! El modelo representa causalidad parcial entre eventos distribuidos. No
//! implementa red real, persistencia, resolución automática de conflictos ni
//! un CRDT completo.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de nodo.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Contador lógico observado para un nodo.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Counter(pub u64);

/// Relación causal observable entre dos relojes vectoriales.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalRelation {
    /// Ambos relojes representan el mismo conocimiento causal.
    Equal,
    /// El reloj izquierdo está contenido estrictamente en el derecho.
    Before,
    /// El reloj izquierdo contiene estrictamente al derecho.
    After,
    /// Ningún reloj contiene al otro.
    Concurrent,
}

/// Reloj vectorial determinista para escenarios educativos.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VectorClock {
    counters: BTreeMap<NodeId, Counter>,
}

impl VectorClock {
    /// Crea un reloj vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Devuelve el contador observado para un nodo.
    ///
    /// Un nodo ausente equivale a `Counter(0)`.
    #[must_use]
    pub fn counter(&self, node: NodeId) -> Counter {
        self.counters.get(&node).copied().unwrap_or_default()
    }

    /// Incrementa el contador local de un nodo y devuelve el nuevo valor.
    pub fn increment(&mut self, node: NodeId) -> Counter {
        let counter = self.counters.entry(node).or_default();
        counter.0 += 1;
        *counter
    }

    /// Fusiona otro reloj conservando el máximo por componente.
    pub fn merge(&mut self, other: &Self) {
        for (&node, &other_counter) in &other.counters {
            let counter = self.counters.entry(node).or_default();
            *counter = (*counter).max(other_counter);
        }
    }

    /// Devuelve un reloj nuevo con el resultado de fusionar ambos relojes.
    #[must_use]
    pub fn merged(&self, other: &Self) -> Self {
        let mut merged = self.clone();
        merged.merge(other);
        merged
    }

    /// Compara este reloj contra otro y devuelve su relación causal.
    #[must_use]
    pub fn compare(&self, other: &Self) -> CausalRelation {
        let mut has_lower_component = false;
        let mut has_higher_component = false;

        for node in self.nodes_seen_with(other) {
            match self.counter(node).cmp(&other.counter(node)) {
                std::cmp::Ordering::Less => has_lower_component = true,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => has_higher_component = true,
            }
        }

        match (has_lower_component, has_higher_component) {
            (false, false) => CausalRelation::Equal,
            (true, false) => CausalRelation::Before,
            (false, true) => CausalRelation::After,
            (true, true) => CausalRelation::Concurrent,
        }
    }

    fn nodes_seen_with(&self, other: &Self) -> BTreeSet<NodeId> {
        self.counters
            .keys()
            .chain(other.counters.keys())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{CausalRelation, Counter, NodeId, VectorClock};

    #[test]
    fn empty_clocks_are_equal() {
        assert_eq!(
            VectorClock::new().compare(&VectorClock::new()),
            CausalRelation::Equal
        );
    }

    #[test]
    fn increment_returns_next_counter() {
        let mut clock = VectorClock::new();

        assert_eq!(clock.increment(NodeId(9)), Counter(1));
    }
}
