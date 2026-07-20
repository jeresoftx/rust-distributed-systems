use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind, PartitionState,
};

fn main() {
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

    println!("Red saludable: {}", outcome.explanation);
}
