use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind, PartitionState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckoutRoute {
    ConfirmInventory,
    RecordIntent,
}

fn checkout_decision(route: CheckoutRoute) -> CapDecision {
    let scenario = match route {
        CheckoutRoute::ConfirmInventory => CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Strong,
            AvailabilityPolicy::RequireCoordination,
            OperationKind::Write,
        ),
        CheckoutRoute::RecordIntent => CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Eventual,
            AvailabilityPolicy::ServeLocalReplica,
            OperationKind::Write,
        ),
    };

    scenario.evaluate().decision
}

fn main() {
    let confirmation = checkout_decision(CheckoutRoute::ConfirmInventory);
    let intent = checkout_decision(CheckoutRoute::RecordIntent);

    assert_eq!(confirmation, CapDecision::RejectToPreserveConsistency);
    assert_eq!(intent, CapDecision::AcceptWithDivergenceRisk);

    println!(
        "Confirmación de inventario: {:?}; intención de reserva: {:?}",
        confirmation, intent
    );
}
