use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId, StateRelation};

#[test]
fn local_increment_only_changes_that_replica() {
    let mut counter = GCounter::new();

    assert_eq!(counter.increment(ReplicaId(1)), Count(1));
    assert_eq!(counter.increment(ReplicaId(1)), Count(2));

    assert_eq!(counter.count(ReplicaId(1)), Count(2));
    assert_eq!(counter.count(ReplicaId(2)), Count(0));
    assert_eq!(counter.value(), Count(2));
}

#[test]
fn increment_by_zero_is_a_noop() {
    let mut counter = GCounter::new();

    assert_eq!(counter.increment_by(ReplicaId(1), Count(0)), Count(0));

    assert_eq!(counter.count(ReplicaId(1)), Count(0));
    assert_eq!(counter.value(), Count(0));
}

#[test]
fn merge_keeps_componentwise_max_and_total_value() {
    let mut first = GCounter::new();
    first.increment(ReplicaId(1));
    first.increment(ReplicaId(1));
    first.increment(ReplicaId(2));

    let mut second = GCounter::new();
    second.increment(ReplicaId(1));
    second.increment(ReplicaId(3));
    second.increment(ReplicaId(3));
    second.increment(ReplicaId(3));

    first.merge(&second);

    assert_eq!(first.count(ReplicaId(1)), Count(2));
    assert_eq!(first.count(ReplicaId(2)), Count(1));
    assert_eq!(first.count(ReplicaId(3)), Count(3));
    assert_eq!(first.value(), Count(6));
}

#[test]
fn merge_is_idempotent() {
    let mut counter = GCounter::new();
    counter.increment(ReplicaId(1));
    counter.increment(ReplicaId(1));

    let once = counter.merged(&counter);
    let twice = once.merged(&counter);

    assert_eq!(once, counter);
    assert_eq!(twice, counter);
}

#[test]
fn merge_is_commutative() {
    let mut left = GCounter::new();
    left.increment(ReplicaId(1));

    let mut right = GCounter::new();
    right.increment(ReplicaId(2));
    right.increment(ReplicaId(2));

    assert_eq!(left.merged(&right), right.merged(&left));
}

#[test]
fn merge_is_associative() {
    let mut a = GCounter::new();
    a.increment(ReplicaId(1));

    let mut b = GCounter::new();
    b.increment(ReplicaId(2));
    b.increment(ReplicaId(2));

    let mut c = GCounter::new();
    c.increment(ReplicaId(3));
    c.increment(ReplicaId(3));
    c.increment(ReplicaId(3));

    let left_grouped = a.merged(&b).merged(&c);
    let right_grouped = a.merged(&b.merged(&c));

    assert_eq!(left_grouped, right_grouped);
    assert_eq!(left_grouped.value(), Count(6));
}

#[test]
fn replicas_converge_after_exchanging_states() {
    let mut mexico = GCounter::new();
    mexico.increment(ReplicaId(52));
    mexico.increment(ReplicaId(52));

    let mut canada = GCounter::new();
    canada.increment(ReplicaId(1));
    canada.increment(ReplicaId(1));
    canada.increment(ReplicaId(1));

    let first_delivery = mexico.merged(&canada);
    let duplicate_delivery = first_delivery.merged(&canada);
    let reversed_delivery = canada.merged(&mexico);

    assert_eq!(first_delivery, duplicate_delivery);
    assert_eq!(first_delivery, reversed_delivery);
    assert_eq!(first_delivery.value(), Count(5));
}

#[test]
fn comparison_detects_equal_before_after_and_concurrent_states() {
    let empty = GCounter::new();

    let mut first = GCounter::new();
    first.increment(ReplicaId(1));

    let mut second = first.clone();
    second.increment(ReplicaId(2));

    let mut concurrent = GCounter::new();
    concurrent.increment(ReplicaId(3));

    assert_eq!(empty.compare(&GCounter::new()), StateRelation::Equal);
    assert_eq!(first.compare(&second), StateRelation::Before);
    assert_eq!(second.compare(&first), StateRelation::After);
    assert_eq!(first.compare(&concurrent), StateRelation::Concurrent);
}
