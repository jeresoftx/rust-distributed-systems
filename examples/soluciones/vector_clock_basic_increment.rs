use rust_distributed_systems::vector_clock::{Counter, NodeId, VectorClock};

fn main() {
    let mut clock = VectorClock::new();

    assert_eq!(clock.increment(NodeId(1)), Counter(1));
    assert_eq!(clock.increment(NodeId(1)), Counter(2));
    assert_eq!(clock.counter(NodeId(1)), Counter(2));
    assert_eq!(clock.counter(NodeId(2)), Counter(0));

    println!("Nodo 1 avanzó hasta {:?}", clock.counter(NodeId(1)));
}
