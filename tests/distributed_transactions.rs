use rust_distributed_systems::distributed_transactions::{
    ParticipantId, ParticipantVote, Saga, SagaEvent, SagaOutcome, SagaStep, SagaStepId,
    TransactionDecision, TransactionError, TransactionEvent, TransactionId, TwoPhaseCommit,
};

#[test]
fn two_phase_commit_commits_only_when_all_participants_prepare() {
    let mut coordinator =
        TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2), ParticipantId(3)]);

    let decision = coordinator
        .decide(
            TransactionId(100),
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Prepared),
                (ParticipantId(3), ParticipantVote::Prepared),
            ],
        )
        .expect("todos los participantes votaron");

    assert_eq!(decision, TransactionDecision::Committed);
    assert_eq!(
        coordinator.decision(TransactionId(100)),
        Some(TransactionDecision::Committed)
    );
}

#[test]
fn two_phase_commit_aborts_when_any_participant_rejects() {
    let mut coordinator =
        TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2), ParticipantId(3)]);

    let decision = coordinator
        .decide(
            TransactionId(200),
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Abort),
                (ParticipantId(3), ParticipantVote::Prepared),
            ],
        )
        .expect("todos los participantes votaron");

    assert_eq!(decision, TransactionDecision::Aborted);
    assert_eq!(
        coordinator.history().last(),
        Some(&TransactionEvent::TransactionAborted {
            transaction: TransactionId(200),
        })
    );
}

#[test]
fn decided_transactions_are_idempotent() {
    let mut coordinator = TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2)]);
    let transaction = TransactionId(300);

    let first = coordinator
        .decide(
            transaction,
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Prepared),
            ],
        )
        .unwrap();
    let events_after_first_decision = coordinator.history().len();

    let retry = coordinator
        .decide(
            transaction,
            [
                (ParticipantId(1), ParticipantVote::Abort),
                (ParticipantId(2), ParticipantVote::Abort),
            ],
        )
        .unwrap();

    assert_eq!(first, TransactionDecision::Committed);
    assert_eq!(retry, TransactionDecision::Committed);
    assert_eq!(coordinator.history().len(), events_after_first_decision);
}

#[test]
fn missing_vote_is_an_explicit_error() {
    let mut coordinator = TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2)]);

    assert_eq!(
        coordinator.decide(
            TransactionId(400),
            [(ParticipantId(1), ParticipantVote::Prepared)]
        ),
        Err(TransactionError::MissingVote {
            transaction: TransactionId(400),
            participant: ParticipantId(2),
        })
    );
}

#[test]
fn successful_saga_applies_each_step_without_compensation() {
    let mut saga = Saga::from_steps([
        SagaStep::new(SagaStepId("reservar-asiento"), true),
        SagaStep::new(SagaStepId("cobrar-tarjeta"), true),
        SagaStep::new(SagaStepId("emitir-boleto"), true),
    ]);

    let outcome = saga.run(TransactionId(500));

    assert_eq!(
        outcome,
        SagaOutcome::Applied {
            transaction: TransactionId(500),
            applied: vec![
                SagaStepId("reservar-asiento"),
                SagaStepId("cobrar-tarjeta"),
                SagaStepId("emitir-boleto"),
            ],
        }
    );
    assert!(saga
        .history()
        .iter()
        .all(|event| !matches!(event, SagaEvent::StepCompensated { .. })));
}

#[test]
fn failed_saga_compensates_applied_steps_in_reverse_order() {
    let mut saga = Saga::from_steps([
        SagaStep::new(SagaStepId("reservar-asiento"), true),
        SagaStep::new(SagaStepId("cobrar-tarjeta"), true),
        SagaStep::new(SagaStepId("emitir-boleto"), false),
    ]);

    let outcome = saga.run(TransactionId(600));

    assert_eq!(
        outcome,
        SagaOutcome::Compensated {
            transaction: TransactionId(600),
            failed_step: SagaStepId("emitir-boleto"),
            applied: vec![SagaStepId("reservar-asiento"), SagaStepId("cobrar-tarjeta"),],
            compensated: vec![SagaStepId("cobrar-tarjeta"), SagaStepId("reservar-asiento"),],
        }
    );
    assert_eq!(
        saga.compensated_steps(TransactionId(600)),
        [SagaStepId("cobrar-tarjeta"), SagaStepId("reservar-asiento")]
    );
}
