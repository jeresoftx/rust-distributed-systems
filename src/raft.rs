//! Modelo educativo mínimo de Raft.
//!
//! El modelo representa términos, roles, votos, log replicado y commit por
//! mayoría. No implementa red real, almacenamiento persistente ni timeouts
//! físicos; deja visibles las invariantes centrales del protocolo.

use std::collections::BTreeMap;

/// Identificador estable de un nodo dentro de una simulación Raft.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u64);

/// Término lógico monótono de Raft.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Term(pub u64);

/// Posición de una entrada dentro del log replicado.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LogIndex(pub u64);

/// Rol local de un nodo Raft.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    /// Nodo que sigue al líder vigente o espera una elección.
    Follower,
    /// Nodo que solicita votos para convertirse en líder.
    Candidate,
    /// Nodo que coordina replicación durante un término.
    Leader,
}

/// Entrada educativa del log replicado.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogEntry {
    /// Índice estable de la entrada.
    pub index: LogIndex,
    /// Término en el que el líder creó la entrada.
    pub term: Term,
    /// Comando representado como texto para mantener el modelo simple.
    pub command: String,
}

/// Evento observable del modelo Raft.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RaftEvent {
    /// Un candidato inició elección.
    ElectionStarted {
        /// Nodo candidato.
        candidate: NodeId,
        /// Término de la elección.
        term: Term,
    },
    /// Un nodo concedió voto.
    VoteGranted {
        /// Nodo que vota.
        voter: NodeId,
        /// Nodo que recibe el voto.
        candidate: NodeId,
        /// Término del voto.
        term: Term,
    },
    /// Un candidato alcanzó mayoría y se volvió líder.
    LeaderElected {
        /// Nuevo líder.
        leader: NodeId,
        /// Término del liderazgo.
        term: Term,
    },
    /// El líder agregó una entrada a su log.
    EntryAppended {
        /// Líder que crea la entrada.
        leader: NodeId,
        /// Índice de la entrada.
        index: LogIndex,
        /// Término de la entrada.
        term: Term,
    },
    /// Un seguidor replicó una entrada del líder.
    EntryReplicated {
        /// Líder que replica.
        leader: NodeId,
        /// Seguidor que acepta la entrada.
        follower: NodeId,
        /// Índice replicado.
        index: LogIndex,
    },
    /// Una entrada alcanzó mayoría y quedó confirmada.
    EntryCommitted {
        /// Índice confirmado.
        index: LogIndex,
    },
}

/// Error explícito del modelo educativo de Raft.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RaftError {
    /// El nodo no pertenece al clúster.
    UnknownNode(NodeId),
    /// El mensaje pertenece a un término viejo.
    StaleTerm {
        /// Nodo que rechaza el término.
        node: NodeId,
        /// Término local vigente.
        current: Term,
        /// Término intentado por el mensaje.
        attempted: Term,
    },
    /// Un nodo ya votó por otro candidato en el mismo término.
    AlreadyVoted {
        /// Nodo que ya votó.
        voter: NodeId,
        /// Término del voto.
        term: Term,
        /// Candidato votado previamente.
        voted_for: NodeId,
        /// Candidato intentado después.
        attempted: NodeId,
    },
    /// El candidato no alcanzó quórum.
    ElectionWithoutMajority {
        /// Candidato evaluado.
        candidate: NodeId,
        /// Término de la elección.
        term: Term,
        /// Votos observados.
        votes: usize,
        /// Votos requeridos.
        quorum: usize,
    },
    /// Una operación que requiere líder fue llamada desde otro rol.
    NotLeader {
        /// Nodo que intentó actuar como líder.
        node: NodeId,
        /// Rol actual del nodo.
        role: Role,
    },
    /// El índice solicitado no existe en el log del líder.
    UnknownLogEntry(LogIndex),
    /// El log del seguidor no coincide con el prefijo esperado.
    LogConflict {
        /// Nodo que contiene el conflicto.
        node: NodeId,
        /// Índice conflictivo.
        index: LogIndex,
    },
    /// La entrada todavía no fue replicada en una mayoría.
    EntryWithoutMajority {
        /// Índice evaluado.
        index: LogIndex,
        /// Réplicas observadas.
        replicas: usize,
        /// Réplicas requeridas.
        quorum: usize,
    },
    /// Una entrada confirmada intentó cambiarse.
    CommittedEntryCannotChange {
        /// Nodo que protege la entrada confirmada.
        node: NodeId,
        /// Índice protegido.
        index: LogIndex,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RaftNode {
    current_term: Term,
    role: Role,
    voted_for: BTreeMap<Term, NodeId>,
    log: Vec<LogEntry>,
    commit_index: Option<LogIndex>,
}

impl RaftNode {
    fn new() -> Self {
        Self {
            current_term: Term(0),
            role: Role::Follower,
            voted_for: BTreeMap::new(),
            log: Vec::new(),
            commit_index: None,
        }
    }
}

/// Clúster Raft educativo con ejecución determinista.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaftCluster {
    nodes: BTreeMap<NodeId, RaftNode>,
    leader: Option<NodeId>,
    history: Vec<RaftEvent>,
}

impl RaftCluster {
    /// Crea un clúster con nodos conocidos.
    #[must_use]
    pub fn new(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self {
            nodes: nodes
                .into_iter()
                .map(|node| (node, RaftNode::new()))
                .collect(),
            leader: None,
            history: Vec::new(),
        }
    }

    /// Devuelve el tamaño de quórum mayoritario.
    #[must_use]
    pub fn quorum_size(&self) -> usize {
        (self.nodes.len() / 2) + 1
    }

    /// Líder vigente, si ya existe uno en el modelo.
    #[must_use]
    pub fn leader(&self) -> Option<NodeId> {
        self.leader
    }

    /// Rol local de un nodo conocido.
    pub fn node_role(&self, node: NodeId) -> Result<Role, RaftError> {
        Ok(self.node(node)?.role)
    }

    /// Término local de un nodo conocido.
    pub fn node_term(&self, node: NodeId) -> Result<Term, RaftError> {
        Ok(self.node(node)?.current_term)
    }

    /// Historial observable del clúster.
    #[must_use]
    pub fn history(&self) -> &[RaftEvent] {
        &self.history
    }

    /// Inicia una elección desde un candidato conocido.
    pub fn start_election(&mut self, candidate: NodeId) -> Result<Term, RaftError> {
        let term = {
            let node = self.node_mut(candidate)?;
            let next = Term(node.current_term.0 + 1);
            node.current_term = next;
            node.role = Role::Candidate;
            node.voted_for.insert(next, candidate);
            next
        };

        self.leader = None;
        self.history
            .push(RaftEvent::ElectionStarted { candidate, term });
        Ok(term)
    }

    /// Solicita un voto a un nodo para un candidato y término concretos.
    pub fn request_vote(
        &mut self,
        voter: NodeId,
        candidate: NodeId,
        term: Term,
    ) -> Result<(), RaftError> {
        self.node(candidate)?;

        {
            let voter_node = self.node_mut(voter)?;
            if term < voter_node.current_term {
                return Err(RaftError::StaleTerm {
                    node: voter,
                    current: voter_node.current_term,
                    attempted: term,
                });
            }

            if term > voter_node.current_term {
                voter_node.current_term = term;
                voter_node.role = Role::Follower;
            }

            if let Some(voted_for) = voter_node.voted_for.get(&term).copied() {
                if voted_for != candidate {
                    return Err(RaftError::AlreadyVoted {
                        voter,
                        term,
                        voted_for,
                        attempted: candidate,
                    });
                }

                return Ok(());
            }

            voter_node.voted_for.insert(term, candidate);
        }

        self.history.push(RaftEvent::VoteGranted {
            voter,
            candidate,
            term,
        });
        Ok(())
    }

    /// Convierte a un candidato en líder si alcanzó quórum.
    pub fn finish_election(&mut self, candidate: NodeId) -> Result<(), RaftError> {
        let term = self.node(candidate)?.current_term;
        let votes = self
            .nodes
            .values()
            .filter(|node| node.voted_for.get(&term) == Some(&candidate))
            .count();
        let quorum = self.quorum_size();

        if votes < quorum {
            return Err(RaftError::ElectionWithoutMajority {
                candidate,
                term,
                votes,
                quorum,
            });
        }

        for (node_id, node) in &mut self.nodes {
            if *node_id == candidate {
                node.role = Role::Leader;
                node.current_term = term;
            } else if node.current_term <= term {
                node.role = Role::Follower;
                node.current_term = term;
            }
        }

        self.leader = Some(candidate);
        self.history.push(RaftEvent::LeaderElected {
            leader: candidate,
            term,
        });
        Ok(())
    }

    /// Agrega una entrada al log del líder.
    pub fn append_entry(
        &mut self,
        leader: NodeId,
        command: impl Into<String>,
    ) -> Result<LogIndex, RaftError> {
        self.ensure_leader(leader)?;

        let (index, term) = {
            let leader_node = self.node_mut(leader)?;
            let index = LogIndex(leader_node.log.len() as u64 + 1);
            let term = leader_node.current_term;
            leader_node.log.push(LogEntry {
                index,
                term,
                command: command.into(),
            });
            (index, term)
        };

        self.history.push(RaftEvent::EntryAppended {
            leader,
            index,
            term,
        });
        Ok(index)
    }

    /// Replica una entrada del líder hacia un seguidor.
    pub fn replicate_entry(
        &mut self,
        leader: NodeId,
        follower: NodeId,
        index: LogIndex,
    ) -> Result<(), RaftError> {
        self.ensure_leader(leader)?;
        let entry = self
            .entry_at(leader, index)?
            .cloned()
            .ok_or(RaftError::UnknownLogEntry(index))?;

        {
            let follower_node = self.node_mut(follower)?;
            ensure_log_can_receive(follower, follower_node, &entry)?;

            if follower_node.log_entry(index) == Some(&entry) {
                return Ok(());
            }

            follower_node.current_term = follower_node.current_term.max(entry.term);
            follower_node.log.push(entry);
        }

        self.history.push(RaftEvent::EntryReplicated {
            leader,
            follower,
            index,
        });
        Ok(())
    }

    /// Confirma una entrada cuando está replicada en una mayoría.
    pub fn commit_entry(&mut self, leader: NodeId, index: LogIndex) -> Result<(), RaftError> {
        self.ensure_leader(leader)?;
        let entry = self
            .entry_at(leader, index)?
            .cloned()
            .ok_or(RaftError::UnknownLogEntry(index))?;
        let replicas = self
            .nodes
            .values()
            .filter(|node| node.log_entry(index) == Some(&entry))
            .count();
        let quorum = self.quorum_size();

        if replicas < quorum {
            return Err(RaftError::EntryWithoutMajority {
                index,
                replicas,
                quorum,
            });
        }

        for node in self.nodes.values_mut() {
            if node.log_entry(index) == Some(&entry) {
                node.commit_index = Some(index);
            }
        }

        self.history.push(RaftEvent::EntryCommitted { index });
        Ok(())
    }

    /// Comando confirmado en un índice, si el líder vigente lo conoce como commit.
    #[must_use]
    pub fn committed_command(&self, index: LogIndex) -> Option<&str> {
        self.leader
            .and_then(|leader| self.nodes.get(&leader))
            .filter(|node| node.commit_index >= Some(index))
            .and_then(|node| node.log_entry(index))
            .map(|entry| entry.command.as_str())
    }

    /// Prepara un log local para escenarios educativos de divergencia.
    pub fn install_log_for_scenario(
        &mut self,
        node: NodeId,
        entries: impl IntoIterator<Item = (Term, impl Into<String>)>,
    ) -> Result<(), RaftError> {
        let node = self.node_mut(node)?;
        node.log = entries
            .into_iter()
            .enumerate()
            .map(|(position, (term, command))| LogEntry {
                index: LogIndex(position as u64 + 1),
                term,
                command: command.into(),
            })
            .collect();
        Ok(())
    }

    fn ensure_leader(&self, node: NodeId) -> Result<(), RaftError> {
        let role = self.node(node)?.role;
        if role == Role::Leader {
            Ok(())
        } else {
            Err(RaftError::NotLeader { node, role })
        }
    }

    fn entry_at(&self, node: NodeId, index: LogIndex) -> Result<Option<&LogEntry>, RaftError> {
        Ok(self.node(node)?.log_entry(index))
    }

    fn node(&self, node: NodeId) -> Result<&RaftNode, RaftError> {
        self.nodes.get(&node).ok_or(RaftError::UnknownNode(node))
    }

    fn node_mut(&mut self, node: NodeId) -> Result<&mut RaftNode, RaftError> {
        self.nodes
            .get_mut(&node)
            .ok_or(RaftError::UnknownNode(node))
    }
}

impl RaftNode {
    fn log_entry(&self, index: LogIndex) -> Option<&LogEntry> {
        log_position(index).and_then(|position| self.log.get(position))
    }
}

fn ensure_log_can_receive(
    follower: NodeId,
    follower_node: &RaftNode,
    entry: &LogEntry,
) -> Result<(), RaftError> {
    if let Some(existing) = follower_node.log_entry(entry.index) {
        if existing == entry {
            return Ok(());
        }

        if follower_node.commit_index >= Some(entry.index) {
            return Err(RaftError::CommittedEntryCannotChange {
                node: follower,
                index: entry.index,
            });
        }

        return Err(RaftError::LogConflict {
            node: follower,
            index: entry.index,
        });
    }

    let expected_next = LogIndex(follower_node.log.len() as u64 + 1);
    if entry.index != expected_next {
        return Err(RaftError::LogConflict {
            node: follower,
            index: entry.index,
        });
    }

    if entry.index.0 > 1 {
        let previous = LogIndex(entry.index.0 - 1);
        if follower_node.log_entry(previous).is_none() {
            return Err(RaftError::LogConflict {
                node: follower,
                index: previous,
            });
        }
    }

    Ok(())
}

fn log_position(index: LogIndex) -> Option<usize> {
    if index.0 == 0 {
        None
    } else {
        Some((index.0 - 1) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeId, RaftCluster};

    #[test]
    fn majority_quorum_is_half_plus_one() {
        let three_nodes = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);
        let four_nodes = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);

        assert_eq!(three_nodes.quorum_size(), 2);
        assert_eq!(four_nodes.quorum_size(), 3);
    }
}
