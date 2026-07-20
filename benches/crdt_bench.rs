use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::crdt::{Count, GCounter, ReplicaId, StateRelation};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_local_increment(),
        benchmark_componentwise_merge(),
        benchmark_state_comparison(),
        benchmark_eventual_convergence(),
    ];

    println!("\nCRDTs benchmark educativo");
    println!("Modelo: G-Counter state-based, fusión por máximo y convergencia eventual");
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
        self.elapsed.as_nanos().div_ceil(self.operations as u128)
    }
}

fn benchmark_local_increment() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut counter = GCounter::new();
        let count = counter.increment(black_box(ReplicaId((round_id % 16) as u64)));

        black_box(count);
        black_box(counter);
    }

    BenchmarkResult {
        name: "incremento local",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_componentwise_merge() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut first = counter_with_replicas(&[(1, 2), (2, 3), (3, 5), (4, 8)]);
        let second = counter_with_replicas(&[(3, 13), (4, 21), (5, 34), (6, 55)]);

        first.merge(&second);

        black_box(first);
    }

    BenchmarkResult {
        name: "fusión por máximo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_state_comparison() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let earlier = counter_with_replicas(&[(1, 2), (2, 3), (3, 5)]);
        let mut later = earlier.clone();
        later.increment_by(ReplicaId(4), Count(8));

        let relation = earlier.compare(&later);
        assert_eq!(relation, StateRelation::Before);

        black_box(relation);
    }

    BenchmarkResult {
        name: "comparación parcial",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_eventual_convergence() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mexico = counter_with_replicas(&[(52, 12)]);
        let canada = counter_with_replicas(&[(1, 8)]);
        let spain = counter_with_replicas(&[(34, 5)]);

        let left_order = mexico.merged(&canada).merged(&spain);
        let right_order = spain.merged(&mexico).merged(&canada);

        assert_eq!(left_order, right_order);
        assert_eq!(left_order.value(), Count(25));

        black_box(left_order);
        black_box(right_order);
    }

    BenchmarkResult {
        name: "convergencia eventual",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn counter_with_replicas(entries: &[(u64, u64)]) -> GCounter {
    let mut counter = GCounter::new();
    for &(replica, count) in entries {
        counter.increment_by(ReplicaId(replica), Count(count));
    }
    counter
}
