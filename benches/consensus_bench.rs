use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::consensus::{ConsensusError, ConsensusRound, NodeId, ProposalId};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_majority_decision(),
        benchmark_unavailable_rejections(),
        benchmark_conflict_detection(),
    ];

    println!("\nConsenso benchmark educativo");
    println!("Modelo: una ronda lógica con quórum mayoritario");
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

fn benchmark_majority_decision() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut round = base_round();
        let proposal = ProposalId(round_id as u64);

        round.propose(NodeId(1), proposal, "valor-a");
        round
            .accept(NodeId(1), proposal)
            .expect("el nodo pertenece a la ronda");
        round
            .accept(NodeId(2), proposal)
            .expect("dos votos alcanzan mayoría");

        black_box(round.decided_value());
    }

    BenchmarkResult {
        name: "decisión por mayoría",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_unavailable_rejections() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut round = base_round();
        let proposal = ProposalId(round_id as u64);

        round.propose(NodeId(1), proposal, "valor-a");
        round
            .fail_node(NodeId(2))
            .expect("el nodo pertenece a la ronda");

        let result = round.accept(NodeId(2), proposal);
        assert_eq!(result, Err(ConsensusError::NodeUnavailable(NodeId(2))));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo nodo caído",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_conflict_detection() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut round = base_round();
        let first = ProposalId((round_id * 2) as u64);
        let second = ProposalId((round_id * 2 + 1) as u64);

        round.propose(NodeId(1), first, "valor-a");
        round.propose(NodeId(2), second, "valor-b");
        round.accept(NodeId(1), first).expect("primer voto válido");

        let result = round.accept(NodeId(1), second);
        assert!(matches!(
            result,
            Err(ConsensusError::ConflictingAcceptance { .. })
        ));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "conflicto de aceptación",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn base_round() -> ConsensusRound {
    ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)])
}
