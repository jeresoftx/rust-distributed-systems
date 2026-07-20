use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId, StateRelation};

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegionMetrics {
    name: &'static str,
    confirmations: GCounter,
}

impl RegionMetrics {
    fn new(name: &'static str, replica: ReplicaId, confirmations: Count) -> Self {
        let mut counter = GCounter::new();
        counter.increment_by(replica, confirmations);

        Self {
            name,
            confirmations: counter,
        }
    }
}

fn main() {
    let mexico = RegionMetrics::new("México", ReplicaId(52), Count(12));
    let canada = RegionMetrics::new("Canadá", ReplicaId(1), Count(8));
    let spain = RegionMetrics::new("España", ReplicaId(34), Count(5));

    assert_eq!(
        mexico.confirmations.compare(&canada.confirmations),
        StateRelation::Concurrent
    );

    let global = mexico
        .confirmations
        .merged(&canada.confirmations)
        .merged(&spain.confirmations);

    assert_eq!(global.count(ReplicaId(52)), Count(12));
    assert_eq!(global.count(ReplicaId(1)), Count(8));
    assert_eq!(global.count(ReplicaId(34)), Count(5));
    assert_eq!(global.value(), Count(25));

    println!(
        "Confirmaciones globales: {:?} desde {}, {} y {}",
        global.value(),
        mexico.name,
        canada.name,
        spain.name
    );
}
