//! Modelo educativo mínimo de Paxos.
//!
//! El modelo representa una sola decisión con propuestas numeradas, promesas,
//! aceptaciones, quórum mayoritario e historial observable. No implementa
//! Multi-Paxos, red real, persistencia ni timeouts físicos.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de un participante dentro de una ronda Paxos.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Número total y ordenable de propuesta.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProposalNumber(pub u64);

/// Aceptación previa reportable en promesas futuras.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedProposal {
    /// Número de propuesta aceptado.
    pub proposal: ProposalNumber,
    /// Valor aceptado.
    pub value: String,
}

/// Promesa emitida por un aceptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Promise {
    /// Aceptor que promete.
    pub acceptor: NodeId,
    /// Propuesta prometida.
    pub proposal: ProposalNumber,
    /// Aceptación previa, si existe.
    pub accepted: Option<AcceptedProposal>,
}

/// Evento observable del modelo Paxos.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaxosEvent {
    /// Un aceptor prometió no aceptar propuestas menores.
    PromiseGranted {
        /// Proponente que solicitó la promesa.
        proposer: NodeId,
        /// Aceptor que prometió.
        acceptor: NodeId,
        /// Propuesta prometida.
        proposal: ProposalNumber,
    },
    /// Un aceptor aceptó una propuesta.
    Accepted {
        /// Aceptor que aceptó.
        acceptor: NodeId,
        /// Propuesta aceptada.
        proposal: ProposalNumber,
    },
    /// Una propuesta alcanzó mayoría.
    Chosen {
        /// Propuesta elegida.
        proposal: ProposalNumber,
    },
}

/// Error explícito del modelo educativo de Paxos.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaxosError {
    /// El nodo no pertenece al conjunto de aceptores.
    UnknownNode(NodeId),
    /// La propuesta intentada es menor que la promesa vigente del aceptor.
    StaleProposal {
        /// Aceptor que rechaza.
        acceptor: NodeId,
        /// Mayor propuesta prometida.
        promised: ProposalNumber,
        /// Propuesta intentada.
        attempted: ProposalNumber,
    },
    /// El modelo ya eligió otra propuesta incompatible.
    AlreadyChosen {
        /// Propuesta ya elegida.
        chosen: ProposalNumber,
        /// Propuesta intentada después.
        attempted: ProposalNumber,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AcceptorState {
    promised: Option<ProposalNumber>,
    accepted: Option<AcceptedProposal>,
}

impl AcceptorState {
    fn new() -> Self {
        Self {
            promised: None,
            accepted: None,
        }
    }
}

/// Ronda Paxos educativa para una sola decisión.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaxosRound {
    acceptors: BTreeMap<NodeId, AcceptorState>,
    acceptances: BTreeMap<(ProposalNumber, String), BTreeSet<NodeId>>,
    chosen: Option<AcceptedProposal>,
    history: Vec<PaxosEvent>,
}

impl PaxosRound {
    /// Crea una ronda con aceptores conocidos.
    #[must_use]
    pub fn new(acceptors: impl IntoIterator<Item = NodeId>) -> Self {
        Self {
            acceptors: acceptors
                .into_iter()
                .map(|acceptor| (acceptor, AcceptorState::new()))
                .collect(),
            acceptances: BTreeMap::new(),
            chosen: None,
            history: Vec::new(),
        }
    }

    /// Devuelve el tamaño de quórum mayoritario.
    #[must_use]
    pub fn quorum_size(&self) -> usize {
        (self.acceptors.len() / 2) + 1
    }

    /// Solicita una promesa a un aceptor.
    pub fn prepare(
        &mut self,
        proposer: NodeId,
        acceptor: NodeId,
        proposal: ProposalNumber,
    ) -> Result<Promise, PaxosError> {
        self.ensure_known_node(proposer)?;
        let state = self.acceptor_mut(acceptor)?;

        if let Some(promised) = state.promised {
            if proposal < promised {
                return Err(PaxosError::StaleProposal {
                    acceptor,
                    promised,
                    attempted: proposal,
                });
            }
        }

        state.promised = Some(proposal);
        let accepted = state.accepted.clone();

        self.history.push(PaxosEvent::PromiseGranted {
            proposer,
            acceptor,
            proposal,
        });

        Ok(Promise {
            acceptor,
            proposal,
            accepted,
        })
    }

    /// Elige el valor seguro a partir de una colección de promesas.
    #[must_use]
    pub fn safe_value(promises: &[Promise], preferred: impl Into<String>) -> String {
        promises
            .iter()
            .filter_map(|promise| promise.accepted.as_ref())
            .max_by_key(|accepted| accepted.proposal)
            .map_or_else(|| preferred.into(), |accepted| accepted.value.clone())
    }

    /// Solicita a un aceptor aceptar una propuesta y valor.
    pub fn accept(
        &mut self,
        acceptor: NodeId,
        proposal: ProposalNumber,
        value: impl Into<String>,
    ) -> Result<(), PaxosError> {
        if let Some(chosen) = &self.chosen {
            if chosen.proposal != proposal {
                return Err(PaxosError::AlreadyChosen {
                    chosen: chosen.proposal,
                    attempted: proposal,
                });
            }
        }

        let value = value.into();

        {
            let state = self.acceptor_mut(acceptor)?;
            if let Some(promised) = state.promised {
                if proposal < promised {
                    return Err(PaxosError::StaleProposal {
                        acceptor,
                        promised,
                        attempted: proposal,
                    });
                }
            }

            state.promised = Some(proposal);
            state.accepted = Some(AcceptedProposal {
                proposal,
                value: value.clone(),
            });
        }

        self.history
            .push(PaxosEvent::Accepted { acceptor, proposal });

        let quorum = self.quorum_size();
        let votes = self
            .acceptances
            .entry((proposal, value.clone()))
            .or_default();
        votes.insert(acceptor);

        if self.chosen.is_none() && votes.len() >= quorum {
            self.chosen = Some(AcceptedProposal { proposal, value });
            self.history.push(PaxosEvent::Chosen { proposal });
        }

        Ok(())
    }

    /// Valor elegido, si una propuesta alcanzó quórum.
    #[must_use]
    pub fn chosen_value(&self) -> Option<&str> {
        self.chosen.as_ref().map(|chosen| chosen.value.as_str())
    }

    /// Historial observable de la ronda.
    #[must_use]
    pub fn history(&self) -> &[PaxosEvent] {
        &self.history
    }

    fn ensure_known_node(&self, node: NodeId) -> Result<(), PaxosError> {
        if self.acceptors.contains_key(&node) {
            Ok(())
        } else {
            Err(PaxosError::UnknownNode(node))
        }
    }

    fn acceptor_mut(&mut self, node: NodeId) -> Result<&mut AcceptorState, PaxosError> {
        self.acceptors
            .get_mut(&node)
            .ok_or(PaxosError::UnknownNode(node))
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeId, PaxosRound};

    #[test]
    fn majority_quorum_is_half_plus_one() {
        let three_nodes = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);
        let four_nodes = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);

        assert_eq!(three_nodes.quorum_size(), 2);
        assert_eq!(four_nodes.quorum_size(), 3);
    }
}
