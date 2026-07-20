//! Modelo educativo mínimo de consistent hashing.
//!
//! El modelo representa un anillo lógico ordenado. Cada clave se asigna al
//! primer nodo sucesor de su posición y vuelve al inicio cuando cae después del
//! último nodo. No implementa red real, réplicas virtuales, pesos ni migración
//! de datos.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de un nodo del anillo.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Clave lógica que debe asignarse a un nodo.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Key(pub u64);

impl Key {
    fn slot(self) -> HashSlot {
        HashSlot(self.0)
    }
}

/// Posición determinista dentro del anillo lógico.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HashSlot(pub u64);

/// Nodo ubicado en una posición del anillo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RingNode {
    /// Identidad estable del nodo.
    pub id: NodeId,
    /// Posición del nodo en el anillo.
    pub slot: HashSlot,
}

impl RingNode {
    /// Crea un nodo de anillo.
    #[must_use]
    pub fn new(id: NodeId, slot: HashSlot) -> Self {
        Self { id, slot }
    }
}

/// Cambio observable de dueño para una clave.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyMovement {
    /// Clave evaluada.
    pub key: Key,
    /// Dueño antes del cambio.
    pub from: Option<NodeId>,
    /// Dueño después del cambio.
    pub to: Option<NodeId>,
}

impl KeyMovement {
    /// Crea un movimiento observable de clave.
    #[must_use]
    pub fn new(key: Key, from: Option<NodeId>, to: Option<NodeId>) -> Self {
        Self { key, from, to }
    }
}

/// Anillo determinista para asignar claves a nodos.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConsistentHashRing {
    ids: BTreeMap<NodeId, HashSlot>,
    slots: BTreeMap<HashSlot, BTreeSet<NodeId>>,
}

impl ConsistentHashRing {
    /// Crea un anillo vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un anillo a partir de nodos iniciales.
    #[must_use]
    pub fn from_nodes(nodes: impl IntoIterator<Item = RingNode>) -> Self {
        let mut ring = Self::new();
        for node in nodes {
            ring.insert_node(node);
        }
        ring
    }

    /// Inserta o reposiciona un nodo.
    ///
    /// Si el `NodeId` ya existía, se retira de su posición anterior antes de
    /// insertar la nueva posición. Esto mantiene identidad única.
    pub fn insert_node(&mut self, node: RingNode) {
        self.remove_node(node.id);

        self.ids.insert(node.id, node.slot);
        self.slots.entry(node.slot).or_default().insert(node.id);
    }

    /// Retira un nodo por identidad y devuelve su posición previa si existía.
    pub fn remove_node(&mut self, id: NodeId) -> Option<RingNode> {
        let slot = self.ids.remove(&id)?;

        let should_remove_slot = if let Some(nodes) = self.slots.get_mut(&slot) {
            nodes.remove(&id);
            nodes.is_empty()
        } else {
            false
        };

        if should_remove_slot {
            self.slots.remove(&slot);
        }

        Some(RingNode::new(id, slot))
    }

    /// Devuelve el dueño de una clave.
    #[must_use]
    pub fn owner(&self, key: Key) -> Option<NodeId> {
        let slot = key.slot();

        self.slots
            .range(slot..)
            .next()
            .or_else(|| self.slots.iter().next())
            .and_then(|(_, nodes)| nodes.iter().next().copied())
    }

    /// Devuelve los nodos ordenados por posición y luego por identidad.
    #[must_use]
    pub fn nodes(&self) -> Vec<RingNode> {
        self.slots
            .iter()
            .flat_map(|(&slot, nodes)| nodes.iter().map(move |&id| RingNode::new(id, slot)))
            .collect()
    }

    /// Compara dos anillos para observar qué claves cambiaron de dueño.
    #[must_use]
    pub fn movements_between(before: &Self, after: &Self, keys: &[Key]) -> Vec<KeyMovement> {
        keys.iter()
            .copied()
            .filter_map(|key| {
                let from = before.owner(key);
                let to = after.owner(key);

                (from != to).then_some(KeyMovement::new(key, from, to))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsistentHashRing, HashSlot, Key, NodeId, RingNode};

    #[test]
    fn empty_ring_starts_without_nodes() {
        let ring = ConsistentHashRing::new();

        assert_eq!(ring.owner(Key(1)), None);
        assert!(ring.nodes().is_empty());
    }

    #[test]
    fn insertion_returns_nodes_in_ring_order() {
        let ring = ConsistentHashRing::from_nodes([
            RingNode::new(NodeId(2), HashSlot(40)),
            RingNode::new(NodeId(1), HashSlot(10)),
        ]);

        assert_eq!(
            ring.nodes(),
            vec![
                RingNode::new(NodeId(1), HashSlot(10)),
                RingNode::new(NodeId(2), HashSlot(40)),
            ]
        );
    }
}
