use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::paxos::{NodeId, PaxosError, PaxosRound, ProposalNumber};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_prepare_majority(),
        benchmark_accept_majority(),
        benchmark_stale_rejection(),
        benchmark_safe_value_adoption(),
    ];

    println!("\nPaxos benchmark educativo");
    println!("Modelo: propuestas, promesas, aceptaciones y quórum mayoritario");
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

fn benchmark_prepare_majority() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut round = base_round();
        let proposal = ProposalNumber(round_id as u64 + 1);

        let first = round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
        let second = round.prepare(NodeId(1), NodeId(2), proposal).unwrap();

        black_box([first, second]);
    }

    BenchmarkResult {
        name: "promesas por mayoría",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_accept_majority() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut round = base_round();
        let proposal = ProposalNumber(round_id as u64 + 1);

        round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
        round.prepare(NodeId(1), NodeId(2), proposal).unwrap();
        round.accept(NodeId(1), proposal, "valor-a").unwrap();
        round.accept(NodeId(2), proposal, "valor-a").unwrap();

        black_box(round.chosen_value());
    }

    BenchmarkResult {
        name: "aceptación y decisión",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_stale_rejection() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut round = base_round();
        round
            .prepare(NodeId(1), NodeId(2), ProposalNumber(20))
            .unwrap();

        let result = round.prepare(NodeId(3), NodeId(2), ProposalNumber(10));
        assert!(matches!(result, Err(PaxosError::StaleProposal { .. })));
        let _ = black_box(result);
    }

    BenchmarkResult {
        name: "rechazo propuesta vieja",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_safe_value_adoption() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut round = base_round();
        round
            .prepare(NodeId(1), NodeId(1), ProposalNumber(1))
            .unwrap();
        round
            .accept(NodeId(1), ProposalNumber(1), "valor-previo")
            .unwrap();

        let promises = [
            round
                .prepare(NodeId(2), NodeId(1), ProposalNumber(2))
                .unwrap(),
            round
                .prepare(NodeId(2), NodeId(2), ProposalNumber(2))
                .unwrap(),
        ];
        let safe_value = PaxosRound::safe_value(&promises, "valor-nuevo");

        black_box(safe_value);
    }

    BenchmarkResult {
        name: "adopción valor previo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn base_round() -> PaxosRound {
    PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)])
}
