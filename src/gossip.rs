//! Modelo educativo mínimo de un protocolo gossip.
//!
//! El modelo propaga hechos por rondas deterministas. Cada nodo disponible
//! comparte su conocimiento con un número acotado de pares disponibles. No
//! implementa red real, aleatoriedad de producción, sospecha de fallas ni
//! garantías probabilísticas formales.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de nodo dentro del cluster.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GossipNodeId(pub u64);

/// Hecho propagable por gossip.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GossipFact(pub u64);

/// Cantidad máxima de pares contactados por nodo en una ronda.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fanout(pub usize);

/// Contacto realizado durante una ronda.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GossipContact {
    /// Nodo que envía su conocimiento.
    pub from: GossipNodeId,
    /// Nodo que recibe el mensaje.
    pub to: GossipNodeId,
    /// Cantidad de hechos incluidos en el mensaje.
    pub facts_sent: usize,
}

impl GossipContact {
    /// Crea un contacto observable de gossip.
    #[must_use]
    pub fn new(from: GossipNodeId, to: GossipNodeId, facts_sent: usize) -> Self {
        Self {
            from,
            to,
            facts_sent,
        }
    }
}

/// Resumen de una ronda de propagación.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GossipRoundReport {
    /// Mensajes enviados durante la ronda.
    pub messages_sent: usize,
    /// Hechos nuevos incorporados por receptores.
    pub facts_delivered: usize,
    /// Contactos realizados.
    pub contacts: Vec<GossipContact>,
}

/// Nodo educativo de gossip.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GossipNode {
    available: bool,
    facts: BTreeSet<GossipFact>,
}

impl Default for GossipNode {
    fn default() -> Self {
        Self {
            available: true,
            facts: BTreeSet::new(),
        }
    }
}

/// Cluster determinista para simular rondas de gossip.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GossipCluster {
    nodes: BTreeMap<GossipNodeId, GossipNode>,
}

impl GossipCluster {
    /// Crea un cluster vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un cluster con nodos iniciales disponibles.
    #[must_use]
    pub fn from_nodes(nodes: impl IntoIterator<Item = GossipNodeId>) -> Self {
        let mut cluster = Self::new();
        for node in nodes {
            cluster.insert_node(node);
        }
        cluster
    }

    /// Inserta un nodo disponible si no existe.
    pub fn insert_node(&mut self, id: GossipNodeId) {
        self.nodes.entry(id).or_default();
    }

    /// Marca un nodo como disponible o no disponible.
    pub fn set_available(&mut self, id: GossipNodeId, available: bool) -> bool {
        let Some(node) = self.nodes.get_mut(&id) else {
            return false;
        };

        node.available = available;
        true
    }

    /// Indica si un nodo existe y está disponible.
    #[must_use]
    pub fn is_available(&self, id: GossipNodeId) -> bool {
        self.nodes.get(&id).is_some_and(|node| node.available)
    }

    /// Inserta un hecho directamente en un nodo.
    ///
    /// Devuelve `true` si el hecho era nuevo para ese nodo.
    pub fn insert_fact(&mut self, id: GossipNodeId, fact: GossipFact) -> bool {
        let Some(node) = self.nodes.get_mut(&id) else {
            return false;
        };

        node.facts.insert(fact)
    }

    /// Indica si un nodo conoce un hecho.
    #[must_use]
    pub fn knows(&self, id: GossipNodeId, fact: GossipFact) -> bool {
        self.nodes
            .get(&id)
            .is_some_and(|node| node.facts.contains(&fact))
    }

    /// Devuelve cuántos nodos conocen un hecho.
    #[must_use]
    pub fn coverage(&self, fact: GossipFact) -> usize {
        self.nodes
            .values()
            .filter(|node| node.facts.contains(&fact))
            .count()
    }

    /// Devuelve los nodos en orden estable.
    #[must_use]
    pub fn node_ids(&self) -> Vec<GossipNodeId> {
        self.nodes.keys().copied().collect()
    }

    /// Ejecuta una ronda push de gossip.
    ///
    /// Cada emisor usa un snapshot del conocimiento al inicio de la ronda. Los
    /// hechos recibidos durante esta ronda se podrán retransmitir hasta la
    /// siguiente.
    pub fn run_round(&mut self, fanout: Fanout) -> GossipRoundReport {
        if fanout.0 == 0 {
            return GossipRoundReport::default();
        }

        let snapshot: BTreeMap<GossipNodeId, BTreeSet<GossipFact>> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.available)
            .map(|(&id, node)| (id, node.facts.clone()))
            .collect();
        let available_ids: Vec<GossipNodeId> = snapshot.keys().copied().collect();

        let mut report = GossipRoundReport::default();
        let mut deliveries: Vec<(GossipNodeId, BTreeSet<GossipFact>)> = Vec::new();

        for (&from, facts) in &snapshot {
            if facts.is_empty() {
                continue;
            }

            for to in peers_after(from, &available_ids).into_iter().take(fanout.0) {
                report
                    .contacts
                    .push(GossipContact::new(from, to, facts.len()));
                deliveries.push((to, facts.clone()));
            }
        }

        report.messages_sent = report.contacts.len();

        for (to, facts) in deliveries {
            let Some(receiver) = self.nodes.get_mut(&to) else {
                continue;
            };

            for fact in facts {
                if receiver.facts.insert(fact) {
                    report.facts_delivered += 1;
                }
            }
        }

        report
    }

    /// Indica si todos los nodos disponibles conocen exactamente los mismos hechos.
    #[must_use]
    pub fn available_nodes_converged(&self) -> bool {
        let mut available = self.nodes.values().filter(|node| node.available);
        let Some(first) = available.next() else {
            return true;
        };

        available.all(|node| node.facts == first.facts)
    }
}

fn peers_after(from: GossipNodeId, available_ids: &[GossipNodeId]) -> Vec<GossipNodeId> {
    if available_ids.len() <= 1 {
        return Vec::new();
    }

    let start = available_ids
        .iter()
        .position(|&id| id == from)
        .map_or(0, |index| index + 1);

    (0..available_ids.len() - 1)
        .map(|offset| available_ids[(start + offset) % available_ids.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Fanout, GossipCluster, GossipFact, GossipNodeId};

    #[test]
    fn empty_cluster_is_converged() {
        assert!(GossipCluster::new().available_nodes_converged());
    }

    #[test]
    fn zero_fanout_sends_no_messages() {
        let mut cluster = GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2)]);
        cluster.insert_fact(GossipNodeId(1), GossipFact(1));

        let report = cluster.run_round(Fanout(0));

        assert_eq!(report.messages_sent, 0);
        assert_eq!(cluster.coverage(GossipFact(1)), 1);
    }
}
