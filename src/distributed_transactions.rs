//! Modelo educativo mínimo de transacciones distribuidas.
//!
//! El modelo compara 2PC, sagas e idempotencia por identidad estable. No
//! implementa red real, WAL, timeouts físicos, recuperación durable, locks de
//! base de datos ni garantías absolutas de exactly-once.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de una transacción lógica.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransactionId(pub u64);

/// Identificador estable de participante.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ParticipantId(pub u64);

/// Voto de preparación de un participante en 2PC.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParticipantVote {
    /// El participante puede confirmar si el coordinador decide commit.
    Prepared,
    /// El participante no puede confirmar y fuerza abort.
    Abort,
}

/// Decisión final observable de 2PC.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionDecision {
    /// Todos los participantes prepararon y la transacción confirma.
    Committed,
    /// Al menos un participante rechazó o la transacción debe abortar.
    Aborted,
}

/// Error explícito del modelo educativo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionError {
    /// El voto pertenece a un participante que no forma parte del coordinador.
    UnknownParticipant {
        /// Transacción evaluada.
        transaction: TransactionId,
        /// Participante desconocido.
        participant: ParticipantId,
    },
    /// Falta el voto de un participante conocido.
    MissingVote {
        /// Transacción evaluada.
        transaction: TransactionId,
        /// Participante que no votó.
        participant: ParticipantId,
    },
}

/// Evento observable de 2PC.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionEvent {
    /// Un participante votó preparado.
    ParticipantPrepared {
        /// Transacción evaluada.
        transaction: TransactionId,
        /// Participante que preparó.
        participant: ParticipantId,
    },
    /// Un participante votó abortar.
    ParticipantRejected {
        /// Transacción evaluada.
        transaction: TransactionId,
        /// Participante que rechazó.
        participant: ParticipantId,
    },
    /// La transacción confirmó.
    TransactionCommitted {
        /// Transacción confirmada.
        transaction: TransactionId,
    },
    /// La transacción abortó.
    TransactionAborted {
        /// Transacción abortada.
        transaction: TransactionId,
    },
}

/// Coordinador determinista de two-phase commit.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TwoPhaseCommit {
    participants: BTreeSet<ParticipantId>,
    decisions: BTreeMap<TransactionId, TransactionDecision>,
    history: Vec<TransactionEvent>,
}

impl TwoPhaseCommit {
    /// Crea un coordinador sin participantes.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un coordinador con participantes conocidos.
    #[must_use]
    pub fn from_participants(participants: impl IntoIterator<Item = ParticipantId>) -> Self {
        let mut coordinator = Self::new();
        for participant in participants {
            coordinator.insert_participant(participant);
        }
        coordinator
    }

    /// Inserta un participante conocido.
    pub fn insert_participant(&mut self, participant: ParticipantId) {
        self.participants.insert(participant);
    }

    /// Historial observable de la coordinación.
    #[must_use]
    pub fn history(&self) -> &[TransactionEvent] {
        &self.history
    }

    /// Devuelve la decisión ya tomada para una transacción, si existe.
    #[must_use]
    pub fn decision(&self, transaction: TransactionId) -> Option<TransactionDecision> {
        self.decisions.get(&transaction).copied()
    }

    /// Decide una transacción a partir de votos de preparación.
    ///
    /// Reintentar una transacción ya decidida devuelve la misma decisión sin
    /// duplicar eventos.
    pub fn decide(
        &mut self,
        transaction: TransactionId,
        votes: impl IntoIterator<Item = (ParticipantId, ParticipantVote)>,
    ) -> Result<TransactionDecision, TransactionError> {
        if let Some(decision) = self.decision(transaction) {
            return Ok(decision);
        }

        let mut votes_by_participant = BTreeMap::new();
        for (participant, vote) in votes {
            if !self.participants.contains(&participant) {
                return Err(TransactionError::UnknownParticipant {
                    transaction,
                    participant,
                });
            }

            votes_by_participant.insert(participant, vote);
        }

        for participant in &self.participants {
            if !votes_by_participant.contains_key(participant) {
                return Err(TransactionError::MissingVote {
                    transaction,
                    participant: *participant,
                });
            }
        }

        let mut decision = TransactionDecision::Committed;
        for (&participant, &vote) in &votes_by_participant {
            match vote {
                ParticipantVote::Prepared => {
                    self.history.push(TransactionEvent::ParticipantPrepared {
                        transaction,
                        participant,
                    })
                }
                ParticipantVote::Abort => {
                    decision = TransactionDecision::Aborted;
                    self.history.push(TransactionEvent::ParticipantRejected {
                        transaction,
                        participant,
                    });
                }
            }
        }

        match decision {
            TransactionDecision::Committed => self
                .history
                .push(TransactionEvent::TransactionCommitted { transaction }),
            TransactionDecision::Aborted => self
                .history
                .push(TransactionEvent::TransactionAborted { transaction }),
        }

        self.decisions.insert(transaction, decision);
        Ok(decision)
    }
}

/// Identificador estable de paso de saga.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SagaStepId(pub &'static str);

/// Paso determinista de una saga educativa.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SagaStep {
    id: SagaStepId,
    succeeds: bool,
}

impl SagaStep {
    /// Crea un paso con resultado determinista.
    #[must_use]
    pub fn new(id: SagaStepId, succeeds: bool) -> Self {
        Self { id, succeeds }
    }

    /// Identidad del paso.
    #[must_use]
    pub fn id(&self) -> SagaStepId {
        self.id
    }
}

/// Resultado observable de una saga.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SagaOutcome {
    /// Todos los pasos se aplicaron.
    Applied {
        /// Transacción ejecutada.
        transaction: TransactionId,
        /// Pasos aplicados en orden.
        applied: Vec<SagaStepId>,
    },
    /// Un paso falló y se compensaron los pasos ya aplicados.
    Compensated {
        /// Transacción ejecutada.
        transaction: TransactionId,
        /// Paso que falló.
        failed_step: SagaStepId,
        /// Pasos aplicados antes de la falla.
        applied: Vec<SagaStepId>,
        /// Pasos compensados en orden de compensación.
        compensated: Vec<SagaStepId>,
    },
}

/// Evento observable de una saga.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SagaEvent {
    /// Un paso local se aplicó.
    StepApplied {
        /// Transacción ejecutada.
        transaction: TransactionId,
        /// Paso aplicado.
        step: SagaStepId,
    },
    /// Un paso local falló.
    StepFailed {
        /// Transacción ejecutada.
        transaction: TransactionId,
        /// Paso fallido.
        step: SagaStepId,
    },
    /// Un paso previo fue compensado.
    StepCompensated {
        /// Transacción ejecutada.
        transaction: TransactionId,
        /// Paso compensado.
        step: SagaStepId,
    },
}

/// Ejecutor determinista de sagas educativas.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Saga {
    steps: Vec<SagaStep>,
    outcomes: BTreeMap<TransactionId, SagaOutcome>,
    compensated: BTreeMap<TransactionId, Vec<SagaStepId>>,
    history: Vec<SagaEvent>,
}

impl Saga {
    /// Crea una saga sin pasos.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una saga con pasos iniciales.
    #[must_use]
    pub fn from_steps(steps: impl IntoIterator<Item = SagaStep>) -> Self {
        let mut saga = Self::new();
        for step in steps {
            saga.push_step(step);
        }
        saga
    }

    /// Agrega un paso al final de la saga.
    pub fn push_step(&mut self, step: SagaStep) {
        self.steps.push(step);
    }

    /// Historial observable de la saga.
    #[must_use]
    pub fn history(&self) -> &[SagaEvent] {
        &self.history
    }

    /// Pasos compensados para una transacción.
    #[must_use]
    pub fn compensated_steps(&self, transaction: TransactionId) -> Vec<SagaStepId> {
        self.compensated
            .get(&transaction)
            .cloned()
            .unwrap_or_default()
    }

    /// Ejecuta la saga.
    ///
    /// Reintentar una transacción ya ejecutada devuelve el mismo resultado sin
    /// duplicar efectos ni compensaciones.
    pub fn run(&mut self, transaction: TransactionId) -> SagaOutcome {
        if let Some(outcome) = self.outcomes.get(&transaction) {
            return outcome.clone();
        }

        let mut applied = Vec::new();
        for step in &self.steps {
            if step.succeeds {
                applied.push(step.id());
                self.history.push(SagaEvent::StepApplied {
                    transaction,
                    step: step.id(),
                });
                continue;
            }

            self.history.push(SagaEvent::StepFailed {
                transaction,
                step: step.id(),
            });
            let compensated: Vec<SagaStepId> = applied.iter().rev().copied().collect();
            for step in &compensated {
                self.history.push(SagaEvent::StepCompensated {
                    transaction,
                    step: *step,
                });
            }

            let outcome = SagaOutcome::Compensated {
                transaction,
                failed_step: step.id(),
                applied,
                compensated: compensated.clone(),
            };
            self.compensated.insert(transaction, compensated);
            self.outcomes.insert(transaction, outcome.clone());
            return outcome;
        }

        let outcome = SagaOutcome::Applied {
            transaction,
            applied,
        };
        self.outcomes.insert(transaction, outcome.clone());
        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::{ParticipantId, Saga, TransactionId, TwoPhaseCommit};

    #[test]
    fn empty_coordinator_can_be_created() {
        assert_eq!(TwoPhaseCommit::new().history(), []);
    }

    #[test]
    fn participants_are_registered_once() {
        let mut coordinator = TwoPhaseCommit::new();

        coordinator.insert_participant(ParticipantId(1));
        coordinator.insert_participant(ParticipantId(1));

        assert_eq!(coordinator.decision(TransactionId(1)), None);
    }

    #[test]
    fn empty_saga_succeeds_without_steps() {
        let mut saga = Saga::new();

        assert!(matches!(
            saga.run(TransactionId(1)),
            super::SagaOutcome::Applied { applied, .. } if applied.is_empty()
        ));
    }
}
