use rust_distributed_systems::vector_clock::{CausalRelation, Counter, NodeId, VectorClock};

fn main() {
    let mut writer = VectorClock::new();
    writer.increment(NodeId(1));

    let mut reader = VectorClock::new();
    reader.increment(NodeId(2));
    reader.merge(&writer);
    reader.increment(NodeId(2));

    assert_eq!(reader.counter(NodeId(1)), Counter(1));
    assert_eq!(reader.counter(NodeId(2)), Counter(2));
    assert_eq!(writer.compare(&reader), CausalRelation::Before);

    println!("El lector observó al escritor y produjo {:?}", reader);
}
