use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockError, DistributedLockManager, LeaseDuration, LogicalTime, ResourceId,
};

fn main() {
    let mut locks = DistributedLockManager::new(LogicalTime(0));

    let old_grant = locks
        .acquire(ClientId(1), ResourceId("orders"), LeaseDuration(2))
        .unwrap();

    locks.advance_to(LogicalTime(2));
    let current_grant = locks
        .acquire(ClientId(2), ResourceId("orders"), LeaseDuration(4))
        .unwrap();

    assert_eq!(
        locks.validate_operation(ResourceId("orders"), old_grant.token),
        Err(DistributedLockError::StaleFencingToken {
            resource: ResourceId("orders"),
            current: current_grant.token,
            attempted: old_grant.token,
        })
    );
    locks
        .validate_operation(ResourceId("orders"), current_grant.token)
        .unwrap();

    println!(
        "Token viejo {:?} rechazado; token vigente {:?}",
        old_grant.token, current_grant.token
    );
}
