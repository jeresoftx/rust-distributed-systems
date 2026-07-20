use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockManager, FencingToken, LeaseDuration, LogicalTime, ResourceId,
};

fn main() {
    let mut locks = DistributedLockManager::new(LogicalTime(0));

    let grant = locks
        .acquire(ClientId(1), ResourceId("billing-job"), LeaseDuration(5))
        .unwrap();

    assert_eq!(grant.owner, ClientId(1));
    assert_eq!(grant.token, FencingToken(1));
    assert_eq!(grant.expires_at, LogicalTime(5));
    assert_eq!(
        locks.owner(ResourceId("billing-job")),
        Some((ClientId(1), FencingToken(1)))
    );

    println!("Lock adquirido: {:?}", grant);
}
