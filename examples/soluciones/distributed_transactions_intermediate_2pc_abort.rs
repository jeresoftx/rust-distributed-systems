use rust_distributed_systems::distributed_transactions::{
    ParticipantId, ParticipantVote, TransactionDecision, TransactionId, TwoPhaseCommit,
};

fn main() {
    let mut coordinator =
        TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2), ParticipantId(3)]);
    let transaction = TransactionId(20);

    let first_decision = coordinator
        .decide(
            transaction,
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Abort),
                (ParticipantId(3), ParticipantVote::Prepared),
            ],
        )
        .expect("todos los votos llegaron");

    let retry = coordinator
        .decide(
            transaction,
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Prepared),
                (ParticipantId(3), ParticipantVote::Prepared),
            ],
        )
        .expect("el reintento usa la decisión previa");

    assert_eq!(first_decision, TransactionDecision::Aborted);
    assert_eq!(retry, TransactionDecision::Aborted);

    println!(
        "El rechazo de un participante hizo abortar {:?}",
        transaction
    );
}
