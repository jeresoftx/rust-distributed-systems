use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId};

fn main() {
    let mut left = GCounter::new();
    left.increment_by(ReplicaId(10), Count(4));

    let mut right = GCounter::new();
    right.increment_by(ReplicaId(20), Count(7));

    let mut aggregator_a = GCounter::new();
    aggregator_a.merge(&left);
    aggregator_a.merge(&right);
    aggregator_a.merge(&right);

    let mut aggregator_b = GCounter::new();
    aggregator_b.merge(&right);
    aggregator_b.merge(&left);

    assert_eq!(aggregator_a, aggregator_b);
    assert_eq!(aggregator_a.value(), Count(11));

    let duplicate_delivery = aggregator_a.merged(&right);
    assert_eq!(duplicate_delivery, aggregator_a);

    println!("Ambos agregadores convergieron a {:?}", aggregator_a);
}
