use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::distributed_transactions::{
    ParticipantId, ParticipantVote, Saga, SagaOutcome, SagaStep, SagaStepId, TransactionDecision,
    TransactionId, TwoPhaseCommit,
};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_2pc_commit(),
        benchmark_2pc_abort(),
        benchmark_idempotent_retry(),
        benchmark_saga_compensation(),
    ];

    println!("\nTransacciones distribuidas benchmark educativo");
    println!("Modelo: 2PC, sagas, compensación e idempotencia por identidad");
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

fn benchmark_2pc_commit() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut coordinator = sample_coordinator();
        let decision = coordinator
            .decide(
                TransactionId(round as u64),
                [
                    (ParticipantId(1), ParticipantVote::Prepared),
                    (ParticipantId(2), ParticipantVote::Prepared),
                    (ParticipantId(3), ParticipantVote::Prepared),
                ],
            )
            .expect("todos los votos están presentes");

        assert_eq!(decision, TransactionDecision::Committed);
        black_box(coordinator);
    }

    BenchmarkResult {
        name: "commit 2PC",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_2pc_abort() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut coordinator = sample_coordinator();
        let decision = coordinator
            .decide(
                TransactionId(round as u64),
                [
                    (ParticipantId(1), ParticipantVote::Prepared),
                    (ParticipantId(2), ParticipantVote::Abort),
                    (ParticipantId(3), ParticipantVote::Prepared),
                ],
            )
            .expect("todos los votos están presentes");

        assert_eq!(decision, TransactionDecision::Aborted);
        black_box(coordinator);
    }

    BenchmarkResult {
        name: "abort 2PC",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_idempotent_retry() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut coordinator = sample_coordinator();
        let transaction = TransactionId(round as u64);
        let first = coordinator
            .decide(
                transaction,
                [
                    (ParticipantId(1), ParticipantVote::Prepared),
                    (ParticipantId(2), ParticipantVote::Prepared),
                    (ParticipantId(3), ParticipantVote::Prepared),
                ],
            )
            .expect("primer intento válido");
        let retry = coordinator
            .decide(
                transaction,
                [
                    (ParticipantId(1), ParticipantVote::Abort),
                    (ParticipantId(2), ParticipantVote::Abort),
                    (ParticipantId(3), ParticipantVote::Abort),
                ],
            )
            .expect("reintento usa decisión previa");

        assert_eq!(first, retry);
        black_box(coordinator);
    }

    BenchmarkResult {
        name: "reintento idempotente",
        operations: ROUNDS * 2,
        elapsed: start.elapsed(),
    }
}

fn benchmark_saga_compensation() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut saga = sample_saga();
        let outcome = saga.run(TransactionId(round as u64));

        assert!(matches!(
            outcome,
            SagaOutcome::Compensated {
                failed_step: SagaStepId("emitir-boleto"),
                ..
            }
        ));
        assert_eq!(
            saga.compensated_steps(TransactionId(round as u64)),
            [SagaStepId("cobrar-tarjeta"), SagaStepId("reservar-asiento")]
        );
        black_box(saga);
    }

    BenchmarkResult {
        name: "compensación de saga",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn sample_coordinator() -> TwoPhaseCommit {
    TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2), ParticipantId(3)])
}

fn sample_saga() -> Saga {
    Saga::from_steps([
        SagaStep::new(SagaStepId("reservar-asiento"), true),
        SagaStep::new(SagaStepId("cobrar-tarjeta"), true),
        SagaStep::new(SagaStepId("emitir-boleto"), false),
    ])
}
