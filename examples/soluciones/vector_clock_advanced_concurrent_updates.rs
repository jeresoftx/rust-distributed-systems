use rust_distributed_systems::vector_clock::{CausalRelation, Counter, NodeId, VectorClock};

fn main() {
    let mut left = VectorClock::new();
    left.increment(NodeId(1));

    let mut right = VectorClock::new();
    right.increment(NodeId(2));

    assert_eq!(left.compare(&right), CausalRelation::Concurrent);

    let resolved = left.merged(&right);
    assert_eq!(resolved.counter(NodeId(1)), Counter(1));
    assert_eq!(resolved.counter(NodeId(2)), Counter(1));
    assert_eq!(left.compare(&resolved), CausalRelation::Before);
    assert_eq!(right.compare(&resolved), CausalRelation::Before);

    println!(
        "Actualizaciones concurrentes fusionadas como {:?}",
        resolved
    );
}
