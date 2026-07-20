//! Modelo educativo mínimo de elección de líder.
//!
//! El modelo representa una elección determinista por mayoría con términos,
//! roles, disponibilidad, votos e historial observable. No implementa Raft
//! completo, leases, red real ni detección automática de fallas.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de un nodo dentro de una elección.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Término lógico de una elección.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ElectionTerm(pub u64);

/// Rol local de un nodo dentro del modelo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeadershipRole {
    /// Nodo sin candidatura vigente.
    Follower,
    /// Nodo que está solicitando votos.
    Candidate,
    /// Nodo que alcanzó mayoría dentro de un término.
    Leader,
}

/// Evento observable del modelo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElectionEvent {
    /// Un candidato inició elección.
    ElectionStarted {
        /// Nodo candidato.
        candidate: NodeId,
        /// Término de la elección.
        term: ElectionTerm,
    },
    /// Un nodo concedió voto.
    VoteGranted {
        /// Nodo que vota.
        voter: NodeId,
        /// Candidato que recibe el voto.
        candidate: NodeId,
        /// Término del voto.
        term: ElectionTerm,
    },
    /// Un candidato alcanzó mayoría.
    LeaderElected {
        /// Líder elegido.
        leader: NodeId,
        /// Término del liderazgo.
        term: ElectionTerm,
    },
    /// Un nodo quedó no disponible.
    NodeFailed {
        /// Nodo afectado.
        node: NodeId,
    },
    /// Un nodo volvió a estar disponible.
    NodeRecovered {
        /// Nodo afectado.
        node: NodeId,
    },
}

/// Error explícito del modelo educativo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeaderElectionError {
    /// El nodo no pertenece al grupo.
    UnknownNode(NodeId),
    /// El nodo está caído o particionado en el escenario.
    NodeUnavailable(NodeId),
    /// El término intentado es menor que el término local vigente.
    StaleTerm {
        /// Nodo que rechaza el término.
        node: NodeId,
        /// Término vigente.
        current: ElectionTerm,
        /// Término intentado.
        attempted: ElectionTerm,
    },
    /// Un nodo ya votó por otro candidato en el mismo término.
    AlreadyVoted {
        /// Nodo que ya votó.
        voter: NodeId,
        /// Término del voto.
        term: ElectionTerm,
        /// Candidato votado previamente.
        voted_for: NodeId,
        /// Candidato intentado después.
        attempted: NodeId,
    },
    /// El candidato no alcanzó mayoría.
    ElectionWithoutMajority {
        /// Candidato evaluado.
        candidate: NodeId,
        /// Término de la elección.
        term: ElectionTerm,
        /// Votos observados.
        votes: usize,
        /// Votos requeridos.
        quorum: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ElectionNode {
    current_term: ElectionTerm,
    role: LeadershipRole,
    votes_by_term: BTreeMap<ElectionTerm, NodeId>,
}

impl ElectionNode {
    fn new() -> Self {
        Self {
            current_term: ElectionTerm(0),
            role: LeadershipRole::Follower,
            votes_by_term: BTreeMap::new(),
        }
    }
}

/// Elección determinista de líder por mayoría.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeaderElection {
    nodes: BTreeMap<NodeId, ElectionNode>,
    unavailable_nodes: BTreeSet<NodeId>,
    leader: Option<NodeId>,
    history: Vec<ElectionEvent>,
}

impl LeaderElection {
    /// Crea una elección con un conjunto finito de nodos.
    #[must_use]
    pub fn new(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self {
            nodes: nodes
                .into_iter()
                .map(|node| (node, ElectionNode::new()))
                .collect(),
            unavailable_nodes: BTreeSet::new(),
            leader: None,
            history: Vec::new(),
        }
    }

    /// Devuelve el tamaño de quórum mayoritario.
    #[must_use]
    pub fn quorum_size(&self) -> usize {
        (self.nodes.len() / 2) + 1
    }

    /// Líder vigente, si ya fue elegido.
    #[must_use]
    pub fn leader(&self) -> Option<NodeId> {
        self.leader
    }

    /// Rol local de un nodo conocido.
    pub fn role(&self, node: NodeId) -> Result<LeadershipRole, LeaderElectionError> {
        Ok(self.node(node)?.role)
    }

    /// Historial observable de la elección.
    #[must_use]
    pub fn history(&self) -> &[ElectionEvent] {
        &self.history
    }

    /// Inicia una elección desde un candidato conocido y disponible.
    pub fn start_election(
        &mut self,
        candidate: NodeId,
    ) -> Result<ElectionTerm, LeaderElectionError> {
        self.ensure_available_node(candidate)?;
        let term = {
            let node = self.node_mut(candidate)?;
            let term = ElectionTerm(node.current_term.0 + 1);
            node.current_term = term;
            node.role = LeadershipRole::Candidate;
            node.votes_by_term.insert(term, candidate);
            term
        };

        self.leader = None;
        self.history
            .push(ElectionEvent::ElectionStarted { candidate, term });
        Ok(term)
    }

    /// Concede un voto desde un votante hacia un candidato.
    pub fn grant_vote(
        &mut self,
        voter: NodeId,
        candidate: NodeId,
        term: ElectionTerm,
    ) -> Result<(), LeaderElectionError> {
        self.ensure_known_node(candidate)?;
        self.ensure_available_node(voter)?;
        let current_term = self.highest_term();

        if term < current_term {
            return Err(LeaderElectionError::StaleTerm {
                node: voter,
                current: current_term,
                attempted: term,
            });
        }

        {
            let voter_node = self.node_mut(voter)?;
            if term < voter_node.current_term {
                return Err(LeaderElectionError::StaleTerm {
                    node: voter,
                    current: voter_node.current_term,
                    attempted: term,
                });
            }

            if term > voter_node.current_term {
                voter_node.current_term = term;
                voter_node.role = LeadershipRole::Follower;
            }

            if let Some(voted_for) = voter_node.votes_by_term.get(&term).copied() {
                if voted_for != candidate {
                    return Err(LeaderElectionError::AlreadyVoted {
                        voter,
                        term,
                        voted_for,
                        attempted: candidate,
                    });
                }

                return Ok(());
            }

            voter_node.votes_by_term.insert(term, candidate);
        }

        self.history.push(ElectionEvent::VoteGranted {
            voter,
            candidate,
            term,
        });
        Ok(())
    }

    /// Finaliza una elección si el candidato alcanzó mayoría.
    pub fn finish_election(&mut self, candidate: NodeId) -> Result<(), LeaderElectionError> {
        let term = self.node(candidate)?.current_term;
        let votes = self
            .nodes
            .values()
            .filter(|node| node.votes_by_term.get(&term) == Some(&candidate))
            .count();
        let quorum = self.quorum_size();

        if votes < quorum {
            return Err(LeaderElectionError::ElectionWithoutMajority {
                candidate,
                term,
                votes,
                quorum,
            });
        }

        for (node_id, node) in &mut self.nodes {
            if *node_id == candidate {
                node.role = LeadershipRole::Leader;
                node.current_term = term;
            } else if node.current_term <= term {
                node.role = LeadershipRole::Follower;
                node.current_term = term;
            }
        }

        self.leader = Some(candidate);
        self.history.push(ElectionEvent::LeaderElected {
            leader: candidate,
            term,
        });
        Ok(())
    }

    /// Marca un nodo como no disponible dentro del escenario.
    pub fn fail_node(&mut self, node: NodeId) -> Result<(), LeaderElectionError> {
        self.ensure_known_node(node)?;
        self.unavailable_nodes.insert(node);
        self.history.push(ElectionEvent::NodeFailed { node });
        Ok(())
    }

    /// Recupera un nodo previamente marcado como no disponible.
    pub fn recover_node(&mut self, node: NodeId) -> Result<(), LeaderElectionError> {
        self.ensure_known_node(node)?;
        self.unavailable_nodes.remove(&node);
        self.history.push(ElectionEvent::NodeRecovered { node });
        Ok(())
    }

    fn ensure_known_node(&self, node: NodeId) -> Result<(), LeaderElectionError> {
        if self.nodes.contains_key(&node) {
            Ok(())
        } else {
            Err(LeaderElectionError::UnknownNode(node))
        }
    }

    fn ensure_available_node(&self, node: NodeId) -> Result<(), LeaderElectionError> {
        self.ensure_known_node(node)?;
        if self.unavailable_nodes.contains(&node) {
            Err(LeaderElectionError::NodeUnavailable(node))
        } else {
            Ok(())
        }
    }

    fn node(&self, node: NodeId) -> Result<&ElectionNode, LeaderElectionError> {
        self.nodes
            .get(&node)
            .ok_or(LeaderElectionError::UnknownNode(node))
    }

    fn node_mut(&mut self, node: NodeId) -> Result<&mut ElectionNode, LeaderElectionError> {
        self.nodes
            .get_mut(&node)
            .ok_or(LeaderElectionError::UnknownNode(node))
    }

    fn highest_term(&self) -> ElectionTerm {
        self.nodes
            .values()
            .map(|node| node.current_term)
            .max()
            .unwrap_or(ElectionTerm(0))
    }
}

#[cfg(test)]
mod tests {
    use super::{LeaderElection, NodeId};

    #[test]
    fn majority_quorum_is_half_plus_one() {
        let three_nodes = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
        let four_nodes = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);

        assert_eq!(three_nodes.quorum_size(), 2);
        assert_eq!(four_nodes.quorum_size(), 3);
    }
}
