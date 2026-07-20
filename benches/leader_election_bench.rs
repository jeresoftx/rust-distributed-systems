use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::leader_election::{
    ElectionTerm, LeaderElection, LeaderElectionError, NodeId,
};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_majority_election(),
        benchmark_double_vote_rejection(),
        benchmark_stale_term_rejection(),
        benchmark_recovered_node_vote(),
    ];

    println!("\nElección de líder benchmark educativo");
    println!("Modelo: términos, votos, disponibilidad y quórum mayoritario");
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
        let mut election = base_election();
        let term = election.start_election(NodeId(1)).unwrap();
        election.grant_vote(NodeId(2), NodeId(1), term).unwrap();
        election.finish_election(NodeId(1)).unwrap();

        black_box(election.leader());
    }

    BenchmarkResult {
        name: "elección por mayoría",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_double_vote_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut election = base_election();
        let term = election.start_election(NodeId(1)).unwrap();
        election.grant_vote(NodeId(2), NodeId(1), term).unwrap();

        let result = election.grant_vote(NodeId(2), NodeId(3), term);
        assert!(matches!(
            result,
            Err(LeaderElectionError::AlreadyVoted { .. })
        ));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo doble voto",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_stale_term_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut election = base_election();
        election.start_election(NodeId(1)).unwrap();

        let result = election.grant_vote(NodeId(2), NodeId(1), ElectionTerm(0));
        assert!(matches!(result, Err(LeaderElectionError::StaleTerm { .. })));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo término viejo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_recovered_node_vote() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut election = base_election();
        let term = election.start_election(NodeId(1)).unwrap();
        election.fail_node(NodeId(2)).unwrap();
        election.recover_node(NodeId(2)).unwrap();
        election.grant_vote(NodeId(2), NodeId(1), term).unwrap();
        election.finish_election(NodeId(1)).unwrap();

        black_box(election.history());
    }

    BenchmarkResult {
        name: "voto tras recuperación",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn base_election() -> LeaderElection {
    LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)])
}
