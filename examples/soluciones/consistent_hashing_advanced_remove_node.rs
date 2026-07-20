use rust_distributed_systems::consistent_hashing::{
    ConsistentHashRing, HashSlot, Key, KeyMovement, NodeId, RingNode,
};

fn main() {
    let before = ConsistentHashRing::from_nodes([
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

    println!("Claves movidas al retirar NodeId(2): {:?}", movements);
}
