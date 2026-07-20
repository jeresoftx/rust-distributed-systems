use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::raft::{NodeId, RaftCluster, RaftError, Term};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_majority_election(),
        benchmark_replicated_commit(),
        benchmark_vote_rejection(),
        benchmark_log_conflict(),
    ];

    println!("\nRaft benchmark educativo");
    println!("Modelo: términos, líder, log replicado y commit por mayoría");
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

fn benchmark_majority_election() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = base_cluster();
        let term = cluster.start_election(NodeId(1)).unwrap();
        cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
        cluster.finish_election(NodeId(1)).unwrap();

        black_box(cluster.leader());
    }

    BenchmarkResult {
        name: "elección por mayoría",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_replicated_commit() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut cluster = elected_cluster();
        let index = cluster
            .append_entry(NodeId(1), format!("set x={round_id}"))
            .unwrap();

        cluster
            .replicate_entry(NodeId(1), NodeId(2), index)
            .unwrap();
        cluster.commit_entry(NodeId(1), index).unwrap();

        black_box(cluster.committed_command(index));
    }

    BenchmarkResult {
        name: "replicación y commit",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_vote_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = base_cluster();
        let term = cluster.start_election(NodeId(1)).unwrap();
        cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();

        let result = cluster.request_vote(NodeId(2), NodeId(3), term);
        assert!(matches!(result, Err(RaftError::AlreadyVoted { .. })));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo voto duplicado",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_log_conflict() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut cluster = elected_cluster();
        cluster
            .install_log_for_scenario(NodeId(2), [(Term(9), "valor-anterior")])
            .unwrap();

        let index = cluster
            .append_entry(NodeId(1), format!("valor-nuevo-{round_id}"))
            .unwrap();
        let result = cluster.replicate_entry(NodeId(1), NodeId(2), index);

        assert!(matches!(result, Err(RaftError::LogConflict { .. })));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "conflicto de log",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn elected_cluster() -> RaftCluster {
    let mut cluster = base_cluster();
    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();
    cluster
}

fn base_cluster() -> RaftCluster {
    RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)])
}
