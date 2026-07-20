use rust_distributed_systems::consistent_hashing::{
    ConsistentHashRing, HashSlot, Key, KeyMovement, NodeId, RingNode,
};

#[test]
fn empty_ring_has_no_owner_for_keys() {
    let ring = ConsistentHashRing::new();

    assert_eq!(ring.owner(Key(42)), None);
    assert!(ring.nodes().is_empty());
}

#[test]
fn key_is_assigned_to_first_successor_with_wrap_around() {
    let ring = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);

    assert_eq!(ring.owner(Key(5)), Some(NodeId(1)));
    assert_eq!(ring.owner(Key(10)), Some(NodeId(1)));
    assert_eq!(ring.owner(Key(39)), Some(NodeId(2)));
    assert_eq!(ring.owner(Key(81)), Some(NodeId(1)));
}

#[test]
fn rings_with_same_nodes_are_deterministic_regardless_of_insertion_order() {
    let left = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(3), HashSlot(80)),
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
    ]);
    let right = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(2), HashSlot(40)),
        RingNode::new(NodeId(3), HashSlot(80)),
        RingNode::new(NodeId(1), HashSlot(10)),
    ]);

    assert_eq!(left.nodes(), right.nodes());

    for key in [Key(0), Key(10), Key(20), Key(40), Key(79), Key(99)] {
        assert_eq!(left.owner(key), right.owner(key));
    }
}

#[test]
fn adding_a_node_only_moves_keys_in_the_new_node_range() {
    let before = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);
    let mut after = before.clone();
    after.insert_node(RingNode::new(NodeId(2), HashSlot(40)));

    let movements = ConsistentHashRing::movements_between(
        &before,
        &after,
        &[Key(5), Key(20), Key(39), Key(40), Key(79), Key(81)],
    );

    assert_eq!(
        movements,
        vec![
            KeyMovement::new(Key(20), Some(NodeId(3)), Some(NodeId(2))),
            KeyMovement::new(Key(39), Some(NodeId(3)), Some(NodeId(2))),
            KeyMovement::new(Key(40), Some(NodeId(3)), Some(NodeId(2))),
        ]
    );
}

#[test]
fn removing_a_node_only_moves_keys_owned_by_that_node() {
    let mut before = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);
    let mut after = before.clone();
    assert_eq!(
        after.remove_node(NodeId(2)),
        Some(RingNode::new(NodeId(2), HashSlot(40)))
    );

    let movements = ConsistentHashRing::movements_between(
        &before,
        &after,
        &[Key(5), Key(20), Key(39), Key(40), Key(79), Key(81)],
    );

    assert_eq!(
        movements,
        vec![
            KeyMovement::new(Key(20), Some(NodeId(2)), Some(NodeId(3))),
            KeyMovement::new(Key(39), Some(NodeId(2)), Some(NodeId(3))),
            KeyMovement::new(Key(40), Some(NodeId(2)), Some(NodeId(3))),
        ]
    );

    assert_eq!(before.remove_node(NodeId(99)), None);
}

#[test]
fn updating_a_node_keeps_identity_unique_and_repositions_it() {
    let mut ring = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
    ]);

    ring.insert_node(RingNode::new(NodeId(2), HashSlot(80)));

    assert_eq!(
        ring.nodes(),
        vec![
            RingNode::new(NodeId(1), HashSlot(10)),
            RingNode::new(NodeId(2), HashSlot(80)),
        ]
    );
    assert_eq!(ring.owner(Key(40)), Some(NodeId(2)));
}
