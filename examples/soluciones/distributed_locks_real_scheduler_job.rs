use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockManager, LeaseDuration, LogicalTime, ResourceId,
};

fn main() {
    let mut locks = DistributedLockManager::new(LogicalTime(100));
    let job = ResourceId("daily-ledger-close");

    let first_worker = locks.acquire(ClientId(7), job, LeaseDuration(10)).unwrap();
    locks.validate_operation(job, first_worker.token).unwrap();

    locks.advance_to(LogicalTime(105));
    let renewed = locks
        .renew(ClientId(7), job, first_worker.token, LeaseDuration(10))
        .unwrap();
    assert_eq!(renewed.expires_at, LogicalTime(115));

    locks.release(ClientId(7), job, first_worker.token).unwrap();

    let second_worker = locks.acquire(ClientId(8), job, LeaseDuration(10)).unwrap();
    locks.validate_operation(job, second_worker.token).unwrap();

    assert!(second_worker.token > first_worker.token);
    println!(
        "Scheduler reasignó {:?} de {:?} a {:?}",
        job, first_worker.owner, second_worker.owner
    );
}
