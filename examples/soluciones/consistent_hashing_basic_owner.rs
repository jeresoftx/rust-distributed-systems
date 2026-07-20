use rust_distributed_systems::consistent_hashing::{
    ConsistentHashRing, HashSlot, Key, NodeId, RingNode,
};

fn main() {
    let ring = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);

    assert_eq!(ring.owner(Key(5)), Some(NodeId(1)));
    assert_eq!(ring.owner(Key(39)), Some(NodeId(2)));
    assert_eq!(ring.owner(Key(81)), Some(NodeId(1)));

    println!("Key(39) pertenece a {:?}", ring.owner(Key(39)));
}
