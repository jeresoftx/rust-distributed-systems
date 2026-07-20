use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::cap::{
    AvailabilityPolicy, CapDecision, CapScenario, ConsistencyLevel, OperationKind, PartitionState,
};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_healthy_network(),
        benchmark_consistency_rejection(),
        benchmark_available_divergence(),
        benchmark_checkout_routes(),
    ];

    println!("\nTeorema CAP benchmark educativo");
    println!("Modelo: decisiones explícitas bajo partición y políticas por operación");
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

fn benchmark_healthy_network() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let outcome = CapScenario::new(
            PartitionState::Healthy,
            ConsistencyLevel::Strong,
            AvailabilityPolicy::RequireCoordination,
            OperationKind::Write,
        )
        .evaluate();

        assert_eq!(outcome.decision, CapDecision::AcceptConsistent);
        black_box(outcome);
    }

    BenchmarkResult {
        name: "red saludable",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_consistency_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let outcome = CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Strong,
            AvailabilityPolicy::RequireCoordination,
            OperationKind::Write,
        )
        .evaluate();

        assert_eq!(outcome.decision, CapDecision::RejectToPreserveConsistency);
        assert!(outcome.preserves_strong_consistency);
        black_box(outcome);
    }

    BenchmarkResult {
        name: "rechazo consistente",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_available_divergence() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let outcome = CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Eventual,
            AvailabilityPolicy::ServeLocalReplica,
            OperationKind::Write,
        )
        .evaluate();

        assert_eq!(outcome.decision, CapDecision::AcceptWithDivergenceRisk);
        assert!(outcome.preserves_cap_availability);
        assert!(outcome.divergence_possible);
        black_box(outcome);
    }

    BenchmarkResult {
        name: "disponibilidad local",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_checkout_routes() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let confirmation = CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Strong,
            AvailabilityPolicy::RequireCoordination,
            OperationKind::Write,
        )
        .evaluate();
        let intent = CapScenario::new(
            PartitionState::Partitioned,
            ConsistencyLevel::Eventual,
            AvailabilityPolicy::ServeLocalReplica,
            OperationKind::Write,
        )
        .evaluate();

        assert_eq!(
            confirmation.decision,
            CapDecision::RejectToPreserveConsistency
        );
        assert_eq!(intent.decision, CapDecision::AcceptWithDivergenceRisk);
        black_box((confirmation, intent));
    }

    BenchmarkResult {
        name: "checkout particionado",
        operations: ROUNDS * 2,
        elapsed: start.elapsed(),
    }
}
