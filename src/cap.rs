//! Modelo educativo mínimo del Teorema CAP.
//!
//! El modelo no demuestra formalmente CAP ni simula red real. Solo representa
//! una decisión bajo partición: rechazar para preservar una verdad fuerte o
//! aceptar localmente con riesgo de divergencia temporal.

/// Nivel de consistencia que la operación intenta preservar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsistencyLevel {
    /// La operación necesita observar una única verdad fuerte.
    Strong,
    /// La operación tolera divergencia temporal si existe reconciliación.
    Eventual,
}

/// Política de respuesta cuando la réplica local está viva.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvailabilityPolicy {
    /// La operación exige coordinación con otras réplicas antes de completarse.
    RequireCoordination,
    /// La operación se completa en la réplica local aunque haya partición.
    ServeLocalReplica,
}

/// Estado observable de la red para el escenario educativo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartitionState {
    /// Las réplicas necesarias pueden comunicarse.
    Healthy,
    /// Al menos una réplica necesaria está separada por la red.
    Partitioned,
}

/// Tipo de operación que se está evaluando.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationKind {
    /// Lectura de estado.
    Read,
    /// Escritura de estado.
    Write,
}

/// Decisión CAP observable para una operación.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapDecision {
    /// La operación puede completarse sin sacrificar consistencia fuerte.
    AcceptConsistent,
    /// La operación se rechaza para no crear una verdad divergente.
    RejectToPreserveConsistency,
    /// La operación se acepta localmente con riesgo de divergencia temporal.
    AcceptWithDivergenceRisk,
}

/// Escenario mínimo para razonar una decisión CAP.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapScenario {
    partition: PartitionState,
    consistency: ConsistencyLevel,
    availability: AvailabilityPolicy,
    operation: OperationKind,
}

impl CapScenario {
    /// Crea un escenario educativo CAP.
    #[must_use]
    pub fn new(
        partition: PartitionState,
        consistency: ConsistencyLevel,
        availability: AvailabilityPolicy,
        operation: OperationKind,
    ) -> Self {
        Self {
            partition,
            consistency,
            availability,
            operation,
        }
    }

    /// Evalúa el escenario y devuelve la decisión con sus garantías visibles.
    #[must_use]
    pub fn evaluate(&self) -> CapOutcome {
        match (self.partition, self.consistency, self.availability) {
            (PartitionState::Healthy, _, _) => CapOutcome {
                operation: self.operation,
                decision: CapDecision::AcceptConsistent,
                partition_tradeoff_visible: false,
                preserves_strong_consistency: true,
                preserves_cap_availability: true,
                divergence_possible: false,
                explanation: "Sin partición, el modelo no fabrica un tradeoff CAP.",
            },
            (PartitionState::Partitioned, _, AvailabilityPolicy::RequireCoordination) => {
                CapOutcome {
                    operation: self.operation,
                    decision: CapDecision::RejectToPreserveConsistency,
                    partition_tradeoff_visible: true,
                    preserves_strong_consistency: true,
                    preserves_cap_availability: false,
                    divergence_possible: false,
                    explanation: "Durante una partición, exigir coordinación rechaza la operación para conservar una verdad fuerte.",
                }
            }
            (
                PartitionState::Partitioned,
                ConsistencyLevel::Strong,
                AvailabilityPolicy::ServeLocalReplica,
            ) => CapOutcome {
                operation: self.operation,
                decision: CapDecision::RejectToPreserveConsistency,
                partition_tradeoff_visible: true,
                preserves_strong_consistency: true,
                preserves_cap_availability: false,
                divergence_possible: false,
                explanation: "Durante una partición, consistencia fuerte rechaza completar localmente para evitar divergencia.",
            },
            (
                PartitionState::Partitioned,
                ConsistencyLevel::Eventual,
                AvailabilityPolicy::ServeLocalReplica,
            ) => CapOutcome {
                operation: self.operation,
                decision: CapDecision::AcceptWithDivergenceRisk,
                partition_tradeoff_visible: true,
                preserves_strong_consistency: false,
                preserves_cap_availability: true,
                divergence_possible: true,
                explanation: "Durante una partición, responder localmente preserva disponibilidad y acepta divergencia temporal.",
            },
        }
    }
}

/// Resultado de evaluar una operación bajo el modelo CAP educativo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapOutcome {
    /// Operación evaluada.
    pub operation: OperationKind,
    /// Decisión tomada por el modelo.
    pub decision: CapDecision,
    /// Indica si la partición hizo visible el tradeoff CAP.
    pub partition_tradeoff_visible: bool,
    /// Indica si el resultado conserva consistencia fuerte.
    pub preserves_strong_consistency: bool,
    /// Indica si el resultado conserva disponibilidad CAP para la operación.
    pub preserves_cap_availability: bool,
    /// Indica si el resultado puede producir estados divergentes.
    pub divergence_possible: bool,
    /// Explicación breve de la decisión.
    pub explanation: &'static str,
}

#[cfg(test)]
mod tests {
    use super::{
        AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind,
        PartitionState,
    };

    #[test]
    fn healthy_network_accepts_consistently() {
        let scenario = CapScenario::new(
            PartitionState::Healthy,
            ConsistencyLevel::Eventual,
            AvailabilityPolicy::ServeLocalReplica,
            OperationKind::Read,
        );

        assert_eq!(scenario.evaluate().decision, CapDecision::AcceptConsistent);
    }
}
