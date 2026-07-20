use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::vector_clock::{CausalRelation, NodeId, VectorClock};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_local_increment(),
        benchmark_componentwise_merge(),
        benchmark_causal_comparison(),
        benchmark_concurrent_detection(),
    ];

    println!("\nVector clocks benchmark educativo");
    println!("Modelo: contadores por nodo, fusión por máximo y comparación causal");
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

fn benchmark_local_increment() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut clock = VectorClock::new();
        let counter = clock.increment(NodeId((round_id % 16) as u64));

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
        let mut first = clock_with_nodes(&[1, 2, 3, 4]);
        let second = clock_with_nodes(&[3, 4, 5, 6]);

        first.merge(&second);

        black_box(first);
    }

    BenchmarkResult {
        name: "fusión por máximo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_causal_comparison() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let earlier = clock_with_nodes(&[1, 2, 3]);
        let mut later = earlier.clone();
        later.increment(NodeId(4));

        let relation = earlier.compare(&later);
        assert_eq!(relation, CausalRelation::Before);

        black_box(relation);
    }

    BenchmarkResult {
        name: "comparación causal",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_concurrent_detection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let left = clock_with_nodes(&[1, 2]);
        let right = clock_with_nodes(&[3, 4]);

        let relation = left.compare(&right);
        assert_eq!(relation, CausalRelation::Concurrent);

        black_box(relation);
    }

    BenchmarkResult {
        name: "detección de concurrencia",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn clock_with_nodes(nodes: &[u64]) -> VectorClock {
    let mut clock = VectorClock::new();
    for &node in nodes {
        clock.increment(NodeId(node));
    }
    clock
}
