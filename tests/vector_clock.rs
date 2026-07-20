use rust_distributed_systems::vector_clock::{CausalRelation, Counter, NodeId, VectorClock};

#[test]
fn local_increment_only_changes_that_node() {
    let mut clock = VectorClock::new();

    assert_eq!(clock.increment(NodeId(1)), Counter(1));
    assert_eq!(clock.increment(NodeId(1)), Counter(2));

    assert_eq!(clock.counter(NodeId(1)), Counter(2));
    assert_eq!(clock.counter(NodeId(2)), Counter(0));
}

#[test]
fn merge_keeps_componentwise_max() {
    let mut first = VectorClock::new();
    first.increment(NodeId(1));
    first.increment(NodeId(1));
    first.increment(NodeId(2));

    let mut second = VectorClock::new();
    second.increment(NodeId(1));
    second.increment(NodeId(3));

    first.merge(&second);

    assert_eq!(first.counter(NodeId(1)), Counter(2));
    assert_eq!(first.counter(NodeId(2)), Counter(1));
    assert_eq!(first.counter(NodeId(3)), Counter(1));
}

#[test]
fn causal_comparison_detects_equal_before_and_after() {
    let mut earlier = VectorClock::new();
    earlier.increment(NodeId(1));

    let mut later = earlier.clone();
    later.increment(NodeId(2));

    assert_eq!(earlier.compare(&earlier), CausalRelation::Equal);
    assert_eq!(earlier.compare(&later), CausalRelation::Before);
    assert_eq!(later.compare(&earlier), CausalRelation::After);
}

#[test]
fn concurrent_clocks_are_explicit() {
    let mut left = VectorClock::new();
    left.increment(NodeId(1));

    let mut right = VectorClock::new();
    right.increment(NodeId(2));

    assert_eq!(left.compare(&right), CausalRelation::Concurrent);
    assert_eq!(right.compare(&left), CausalRelation::Concurrent);
}

#[test]
fn missing_nodes_compare_as_zero() {
    let empty = VectorClock::new();

    let mut observed = VectorClock::new();
    observed.increment(NodeId(3));

    assert_eq!(empty.counter(NodeId(3)), Counter(0));
    assert_eq!(empty.compare(&observed), CausalRelation::Before);
    assert_eq!(observed.compare(&empty), CausalRelation::After);
}

#[test]
fn merged_returns_a_new_clock_without_mutating_inputs() {
    let mut first = VectorClock::new();
    first.increment(NodeId(1));

    let mut second = VectorClock::new();
    second.increment(NodeId(2));

    let merged = first.merged(&second);

    assert_eq!(merged.counter(NodeId(1)), Counter(1));
    assert_eq!(merged.counter(NodeId(2)), Counter(1));
    assert_eq!(first.counter(NodeId(2)), Counter(0));
    assert_eq!(second.counter(NodeId(1)), Counter(0));
}
