//! Modelo educativo mínimo de consenso.
//!
//! El modelo representa una sola ronda lógica con propuestas identificables,
//! aceptación por nodos conocidos, quórum mayoritario e historial de eventos.
//! No implementa Raft ni Paxos; solo deja visible el problema de acordar un
//! valor sin memoria compartida.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de un nodo dentro de una simulación.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Identificador estable de una propuesta dentro de una ronda lógica.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProposalId(pub u64);

/// Evento observable del modelo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsensusEvent {
    /// Un nodo creó una propuesta.
    Proposed {
        /// Nodo que propone el valor.
        proposer: NodeId,
        /// Propuesta creada.
        proposal: ProposalId,
    },
    /// Un nodo aceptó una propuesta.
    Accepted {
        /// Nodo que acepta.
        node: NodeId,
        /// Propuesta aceptada.
        proposal: ProposalId,
    },
    /// Una propuesta alcanzó quórum.
    Decided {
        /// Propuesta decidida.
        proposal: ProposalId,
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
pub enum ConsensusError {
    /// El nodo no pertenece a la ronda.
    UnknownNode(NodeId),
    /// La propuesta no existe.
    UnknownProposal(ProposalId),
    /// El nodo está caído o particionado dentro del escenario.
    NodeUnavailable(NodeId),
    /// El nodo intentó aceptar dos propuestas incompatibles en la misma ronda.
    ConflictingAcceptance {
        /// Nodo que ya había aceptado otra propuesta.
        node: NodeId,
        /// Propuesta aceptada previamente.
        accepted: ProposalId,
        /// Propuesta incompatible intentada después.
        attempted: ProposalId,
    },
    /// La ronda ya decidió una propuesta distinta.
    AlreadyDecided {
        /// Propuesta ya decidida.
        decided: ProposalId,
        /// Propuesta incompatible intentada después.
        attempted: ProposalId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Proposal {
    value: String,
}

/// Ronda lógica de consenso con quórum mayoritario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsensusRound {
    nodes: BTreeSet<NodeId>,
    unavailable_nodes: BTreeSet<NodeId>,
    proposals: BTreeMap<ProposalId, Proposal>,
    accepted_by_node: BTreeMap<NodeId, ProposalId>,
    votes_by_proposal: BTreeMap<ProposalId, BTreeSet<NodeId>>,
    decided: Option<ProposalId>,
    history: Vec<ConsensusEvent>,
}

impl ConsensusRound {
    /// Crea una ronda con el conjunto finito de nodos participantes.
    #[must_use]
    pub fn new(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
            unavailable_nodes: BTreeSet::new(),
            proposals: BTreeMap::new(),
            accepted_by_node: BTreeMap::new(),
            votes_by_proposal: BTreeMap::new(),
            decided: None,
            history: Vec::new(),
        }
    }

    /// Devuelve el tamaño de quórum mayoritario.
    #[must_use]
    pub fn quorum_size(&self) -> usize {
        (self.nodes.len() / 2) + 1
    }

    /// Registra una propuesta con un valor educativo.
    pub fn propose(&mut self, proposer: NodeId, proposal: ProposalId, value: impl Into<String>) {
        self.proposals.insert(
            proposal,
            Proposal {
                value: value.into(),
            },
        );
        self.history
            .push(ConsensusEvent::Proposed { proposer, proposal });
    }

    /// Acepta una propuesta desde un nodo conocido y disponible.
    pub fn accept(&mut self, node: NodeId, proposal: ProposalId) -> Result<(), ConsensusError> {
        self.ensure_known_node(node)?;
        self.ensure_available_node(node)?;
        self.ensure_known_proposal(proposal)?;

        if let Some(decided) = self.decided {
            if decided != proposal {
                return Err(ConsensusError::AlreadyDecided {
                    decided,
                    attempted: proposal,
                });
            }
        }

        if let Some(accepted) = self.accepted_by_node.get(&node).copied() {
            if accepted != proposal {
                return Err(ConsensusError::ConflictingAcceptance {
                    node,
                    accepted,
                    attempted: proposal,
                });
            }

            return Ok(());
        }

        self.accepted_by_node.insert(node, proposal);
        let quorum_size = self.quorum_size();
        let votes = self.votes_by_proposal.entry(proposal).or_default();
        votes.insert(node);

        self.history
            .push(ConsensusEvent::Accepted { node, proposal });

        if self.decided.is_none() && votes.len() >= quorum_size {
            self.decided = Some(proposal);
            self.history.push(ConsensusEvent::Decided { proposal });
        }

        Ok(())
    }

    /// Marca un nodo como no disponible dentro del escenario.
    pub fn fail_node(&mut self, node: NodeId) -> Result<(), ConsensusError> {
        self.ensure_known_node(node)?;
        self.unavailable_nodes.insert(node);
        self.history.push(ConsensusEvent::NodeFailed { node });
        Ok(())
    }

    /// Recupera un nodo previamente marcado como no disponible.
    pub fn recover_node(&mut self, node: NodeId) -> Result<(), ConsensusError> {
        self.ensure_known_node(node)?;
        self.unavailable_nodes.remove(&node);
        self.history.push(ConsensusEvent::NodeRecovered { node });
        Ok(())
    }

    /// Valor decidido, si alguna propuesta alcanzó quórum.
    #[must_use]
    pub fn decided_value(&self) -> Option<&str> {
        self.decided
            .and_then(|proposal| self.proposals.get(&proposal))
            .map(|proposal| proposal.value.as_str())
    }

    /// Historial observable de la ronda.
    #[must_use]
    pub fn history(&self) -> &[ConsensusEvent] {
        &self.history
    }

    fn ensure_known_node(&self, node: NodeId) -> Result<(), ConsensusError> {
        if self.nodes.contains(&node) {
            Ok(())
        } else {
            Err(ConsensusError::UnknownNode(node))
        }
    }

    fn ensure_available_node(&self, node: NodeId) -> Result<(), ConsensusError> {
        if self.unavailable_nodes.contains(&node) {
            Err(ConsensusError::NodeUnavailable(node))
        } else {
            Ok(())
        }
    }

    fn ensure_known_proposal(&self, proposal: ProposalId) -> Result<(), ConsensusError> {
        if self.proposals.contains_key(&proposal) {
            Ok(())
        } else {
            Err(ConsensusError::UnknownProposal(proposal))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsensusRound, NodeId};

    #[test]
    fn majority_quorum_is_half_plus_one() {
        let three_nodes = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);
        let four_nodes = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);

        assert_eq!(three_nodes.quorum_size(), 2);
        assert_eq!(four_nodes.quorum_size(), 3);
    }
}
