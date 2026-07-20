use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId};

fn main() {
    let mut counter = GCounter::new();

    assert_eq!(counter.increment(ReplicaId(1)), Count(1));
    assert_eq!(counter.increment(ReplicaId(1)), Count(2));
    assert_eq!(counter.count(ReplicaId(1)), Count(2));
    assert_eq!(counter.count(ReplicaId(2)), Count(0));
    assert_eq!(counter.value(), Count(2));

    println!("Réplica 1 avanzó hasta {:?}", counter.count(ReplicaId(1)));
}
