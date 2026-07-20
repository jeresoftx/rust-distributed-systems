use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockError, DistributedLockManager, FencingToken, LeaseDuration,
    LogicalTime, ResourceId,
};

fn main() {
    let mut locks = DistributedLockManager::new(LogicalTime(10));

    locks
        .acquire(ClientId(1), ResourceId("indexer"), LeaseDuration(3))
        .unwrap();

    assert_eq!(
        locks.acquire(ClientId(2), ResourceId("indexer"), LeaseDuration(3)),
        Err(DistributedLockError::ResourceBusy {
            resource: ResourceId("indexer"),
            owner: ClientId(1),
            expires_at: LogicalTime(13),
        })
    );

    locks.advance_to(LogicalTime(13));
    let grant = locks
        .acquire(ClientId(2), ResourceId("indexer"), LeaseDuration(3))
        .unwrap();

    assert_eq!(grant.token, FencingToken(2));
    assert_eq!(
        locks.owner(ResourceId("indexer")),
        Some((ClientId(2), grant.token))
    );
    println!("Lock reasignado después de expirar: {:?}", grant);
}
