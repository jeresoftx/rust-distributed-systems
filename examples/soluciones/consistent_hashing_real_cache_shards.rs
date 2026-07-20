use rust_distributed_systems::consistent_hashing::{
    ConsistentHashRing, HashSlot, Key, KeyMovement, NodeId, RingNode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProfileCacheKey {
    user_id: u64,
    slot: Key,
}

impl ProfileCacheKey {
    fn new(user_id: u64, slot: u64) -> Self {
        Self {
            user_id,
            slot: Key(slot),
        }
    }
}

fn main() {
    let profiles = [
        ProfileCacheKey::new(1001, 8),
        ProfileCacheKey::new(1002, 21),
        ProfileCacheKey::new(1003, 37),
        ProfileCacheKey::new(1004, 59),
        ProfileCacheKey::new(1005, 82),
    ];

    let before = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(10), HashSlot(10)),
        RingNode::new(NodeId(80), HashSlot(80)),
    ]);
    let mut after = before.clone();
    after.insert_node(RingNode::new(NodeId(40), HashSlot(40)));

    let profile_slots: Vec<Key> = profiles.iter().map(|profile| profile.slot).collect();
    let movements = ConsistentHashRing::movements_between(&before, &after, &profile_slots);

    assert_eq!(
        movements,
        vec![
            KeyMovement::new(Key(21), Some(NodeId(80)), Some(NodeId(40))),
            KeyMovement::new(Key(37), Some(NodeId(80)), Some(NodeId(40))),
        ]
    );
    assert_eq!(profiles[1].user_id, 1002);
    assert_eq!(profiles[2].user_id, 1003);

    println!("Perfiles que cambiarían de shard: {:?}", movements);
}
