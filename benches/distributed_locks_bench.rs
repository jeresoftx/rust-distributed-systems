use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::distributed_locks::{
    ClientId, DistributedLockError, DistributedLockManager, LeaseDuration, LogicalTime, ResourceId,
};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_acquire_available_lock(),
        benchmark_busy_resource_rejection(),
        benchmark_expire_and_reacquire(),
        benchmark_stale_fencing_token_rejection(),
    ];

    println!("\nLocks distribuidos benchmark educativo");
    println!("Modelo: leases lógicos, propietario activo y fencing tokens");
    println!("| Operación | Ops | Total | ns/op |");
    println!("|-----------|-----|-------|-------|");

    for result in results {
        println!(
            "| {} | {} | {:?} | {} |",
            result.name,
            result.operations,
            result.elapsed,
            result.nanoseconds_per_operation()
        );
    }
}

struct BenchmarkResult {
    name: &'static str,
    operations: usize,
    elapsed: Duration,
}

impl BenchmarkResult {
    fn nanoseconds_per_operation(&self) -> u128 {
        self.elapsed.as_nanos() / self.operations as u128
    }
}

fn benchmark_acquire_available_lock() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut locks = base_manager();
        let resource = ResourceId(if round_id % 2 == 0 { "job-a" } else { "job-b" });
        let grant = locks
            .acquire(ClientId(1), resource, LeaseDuration(5))
            .unwrap();

        black_box(grant);
    }

    BenchmarkResult {
        name: "adquisición disponible",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_busy_resource_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut locks = base_manager();
        locks
            .acquire(ClientId(1), ResourceId("indexer"), LeaseDuration(5))
            .unwrap();

        let result = locks.acquire(ClientId(2), ResourceId("indexer"), LeaseDuration(5));
        assert!(matches!(
            result,
            Err(DistributedLockError::ResourceBusy { .. })
        ));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo recurso ocupado",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_expire_and_reacquire() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut locks = base_manager();
        locks
            .acquire(ClientId(1), ResourceId("scheduler"), LeaseDuration(5))
            .unwrap();
        locks.advance_to(LogicalTime(5));
        let grant = locks
            .acquire(ClientId(2), ResourceId("scheduler"), LeaseDuration(5))
            .unwrap();

        black_box(grant);
    }

    BenchmarkResult {
        name: "expirar y readquirir",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_stale_fencing_token_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut locks = base_manager();
        let first = locks
            .acquire(ClientId(1), ResourceId("orders"), LeaseDuration(2))
            .unwrap();
        locks.advance_to(LogicalTime(2));
        locks
            .acquire(ClientId(2), ResourceId("orders"), LeaseDuration(2))
            .unwrap();

        let result = locks.validate_operation(ResourceId("orders"), first.token);
        assert!(matches!(
            result,
            Err(DistributedLockError::StaleFencingToken { .. })
        ));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo token obsoleto",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn base_manager() -> DistributedLockManager {
    DistributedLockManager::new(LogicalTime(0))
}
