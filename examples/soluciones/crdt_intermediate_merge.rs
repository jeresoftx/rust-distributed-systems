use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId, StateRelation};

fn main() {
    let mut mexico = GCounter::new();
    mexico.increment(ReplicaId(52));
    mexico.increment(ReplicaId(52));

    let mut canada = GCounter::new();
    canada.increment(ReplicaId(1));
    canada.increment(ReplicaId(1));
    canada.increment(ReplicaId(1));

    assert_eq!(mexico.compare(&canada), StateRelation::Concurrent);

    let merged = mexico.merged(&canada);

    assert_eq!(merged.count(ReplicaId(52)), Count(2));
    assert_eq!(merged.count(ReplicaId(1)), Count(3));
    assert_eq!(merged.value(), Count(5));
    assert_eq!(mexico.compare(&merged), StateRelation::Before);
    assert_eq!(canada.compare(&merged), StateRelation::Before);

    println!("Estados offline fusionados con valor {:?}", merged.value());
}
