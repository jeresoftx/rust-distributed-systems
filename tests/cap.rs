use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind, PartitionState,
};

#[test]
fn healthy_network_does_not_create_a_cap_tradeoff() {
    let scenario = CapScenario::new(
        PartitionState::Healthy,
        ConsistencyLevel::Strong,
        AvailabilityPolicy::RequireCoordination,
        OperationKind::Write,
    );

    let outcome = scenario.evaluate();

    assert_eq!(outcome.decision, CapDecision::AcceptConsistent);
    assert!(!outcome.partition_tradeoff_visible);
    assert!(outcome.preserves_strong_consistency);
    assert!(outcome.preserves_cap_availability);
    assert!(!outcome.divergence_possible);
}

#[test]
fn strong_consistency_rejects_writes_during_partition() {
    let scenario = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Strong,
        AvailabilityPolicy::RequireCoordination,
        OperationKind::Write,
    );

    let outcome = scenario.evaluate();

    assert_eq!(outcome.decision, CapDecision::RejectToPreserveConsistency);
    assert!(outcome.partition_tradeoff_visible);
    assert!(outcome.preserves_strong_consistency);
    assert!(!outcome.preserves_cap_availability);
    assert!(!outcome.divergence_possible);
}

#[test]
fn serving_local_replica_during_partition_accepts_divergence_risk() {
    let scenario = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Eventual,
        AvailabilityPolicy::ServeLocalReplica,
        OperationKind::Write,
    );

    let outcome = scenario.evaluate();

    assert_eq!(outcome.decision, CapDecision::AcceptWithDivergenceRisk);
    assert!(outcome.partition_tradeoff_visible);
    assert!(!outcome.preserves_strong_consistency);
    assert!(outcome.preserves_cap_availability);
    assert!(outcome.divergence_possible);
}

#[test]
fn requiring_coordination_under_partition_sacrifices_cap_availability() {
    let scenario = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Eventual,
        AvailabilityPolicy::RequireCoordination,
        OperationKind::Read,
    );

    let outcome = scenario.evaluate();

    assert_eq!(outcome.decision, CapDecision::RejectToPreserveConsistency);
    assert!(outcome.partition_tradeoff_visible);
    assert!(outcome.preserves_strong_consistency);
    assert!(!outcome.preserves_cap_availability);
    assert!(!outcome.divergence_possible);
}

#[test]
fn operation_kind_is_kept_in_the_outcome() {
    let read = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Eventual,
        AvailabilityPolicy::ServeLocalReplica,
        OperationKind::Read,
    );

    let write = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Eventual,
        AvailabilityPolicy::ServeLocalReplica,
        OperationKind::Write,
    );

    assert_eq!(read.evaluate().operation, OperationKind::Read);
    assert_eq!(write.evaluate().operation, OperationKind::Write);
    assert_eq!(read.evaluate().decision, write.evaluate().decision);
}

#[test]
fn explanations_name_the_tradeoff_without_product_labels() {
    let scenario = CapScenario::new(
        PartitionState::Partitioned,
        ConsistencyLevel::Eventual,
        AvailabilityPolicy::ServeLocalReplica,
        OperationKind::Write,
    );

    let explanation = scenario.evaluate().explanation;

    assert!(explanation.contains("partición"));
    assert!(explanation.contains("divergencia"));
    assert!(!explanation.contains("CP"));
    assert!(!explanation.contains("AP"));
    assert!(!explanation.contains("CA"));
}
