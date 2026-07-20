use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockError, DistributedLockEvent, DistributedLockManager, FencingToken,
    LeaseDuration, LogicalTime, ResourceId,
};

#[test]
fn client_acquires_available_lock_with_first_fencing_token() {
    let mut locks = DistributedLockManager::new(LogicalTime(10));

    let grant = locks
        .acquire(ClientId(1), ResourceId("billing-job"), LeaseDuration(5))
        .expect("el recurso está disponible");

    assert_eq!(grant.owner, ClientId(1));
    assert_eq!(grant.resource, ResourceId("billing-job"));
    assert_eq!(grant.token, FencingToken(1));
    assert_eq!(grant.expires_at, LogicalTime(15));
    assert_eq!(
        locks.owner(ResourceId("billing-job")),
        Some((ClientId(1), FencingToken(1)))
    );
}

#[test]
fn occupied_resource_rejects_another_client_until_expiration() {
    let mut locks = DistributedLockManager::new(LogicalTime(0));
    locks
        .acquire(ClientId(1), ResourceId("indexer"), LeaseDuration(3))
        .unwrap();

    assert_eq!(
        locks.acquire(ClientId(2), ResourceId("indexer"), LeaseDuration(3)),
        Err(DistributedLockError::ResourceBusy {
            resource: ResourceId("indexer"),
            owner: ClientId(1),
            expires_at: LogicalTime(3),
        })
    );

    locks.advance_to(LogicalTime(3));
    let grant = locks
        .acquire(ClientId(2), ResourceId("indexer"), LeaseDuration(3))
        .expect("el lease anterior expiró");

    assert_eq!(grant.token, FencingToken(2));
    assert_eq!(
        locks.owner(ResourceId("indexer")),
        Some((ClientId(2), FencingToken(2)))
    );
}

#[test]
fn only_current_owner_can_renew_or_release() {
    let mut locks = DistributedLockManager::new(LogicalTime(20));
    let grant = locks
        .acquire(ClientId(1), ResourceId("catalog"), LeaseDuration(4))
        .unwrap();

    assert_eq!(
        locks.renew(
            ClientId(2),
            ResourceId("catalog"),
            grant.token,
            LeaseDuration(4),
        ),
        Err(DistributedLockError::NotLockOwner {
            resource: ResourceId("catalog"),
            expected: ClientId(1),
            attempted: ClientId(2),
        })
    );

    let renewed = locks
        .renew(
            ClientId(1),
            ResourceId("catalog"),
            grant.token,
            LeaseDuration(10),
        )
        .expect("el propietario vigente puede renovar");
    assert_eq!(renewed.expires_at, LogicalTime(30));

    assert_eq!(
        locks.release(ClientId(2), ResourceId("catalog"), grant.token),
        Err(DistributedLockError::NotLockOwner {
            resource: ResourceId("catalog"),
            expected: ClientId(1),
            attempted: ClientId(2),
        })
    );

    locks
        .release(ClientId(1), ResourceId("catalog"), grant.token)
        .expect("el propietario vigente puede liberar");
    assert_eq!(locks.owner(ResourceId("catalog")), None);
}

#[test]
fn stale_fencing_token_cannot_validate_operation() {
    let mut locks = DistributedLockManager::new(LogicalTime(0));
    let first = locks
        .acquire(ClientId(1), ResourceId("orders"), LeaseDuration(2))
        .unwrap();

    locks.advance_to(LogicalTime(2));
    let second = locks
        .acquire(ClientId(2), ResourceId("orders"), LeaseDuration(2))
        .unwrap();

    assert_eq!(
        locks.validate_operation(ResourceId("orders"), first.token),
        Err(DistributedLockError::StaleFencingToken {
            resource: ResourceId("orders"),
            current: second.token,
            attempted: first.token,
        })
    );
    locks
        .validate_operation(ResourceId("orders"), second.token)
        .expect("el token vigente protege la operación");
}

#[test]
fn history_explains_lock_lifecycle() {
    let mut locks = DistributedLockManager::new(LogicalTime(0));
    let grant = locks
        .acquire(ClientId(1), ResourceId("scheduler"), LeaseDuration(2))
        .unwrap();
    locks
        .renew(
            ClientId(1),
            ResourceId("scheduler"),
            grant.token,
            LeaseDuration(4),
        )
        .unwrap();
    locks
        .release(ClientId(1), ResourceId("scheduler"), grant.token)
        .unwrap();

    assert_eq!(
        locks.history(),
        [
            DistributedLockEvent::LockAcquired {
                owner: ClientId(1),
                resource: ResourceId("scheduler"),
                token: FencingToken(1),
                expires_at: LogicalTime(2),
            },
            DistributedLockEvent::LockRenewed {
                owner: ClientId(1),
                resource: ResourceId("scheduler"),
                token: FencingToken(1),
                expires_at: LogicalTime(4),
            },
            DistributedLockEvent::LockReleased {
                owner: ClientId(1),
                resource: ResourceId("scheduler"),
                token: FencingToken(1),
            },
        ]
    );
}
