use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind, PartitionState,
};

fn main() {
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

    println!(
        "Partición con disponibilidad local: {}",
        outcome.explanation
    );
}
