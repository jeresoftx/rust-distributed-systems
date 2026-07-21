use rust_distributed_systems::distributed_transactions::{
    ParticipantId, ParticipantVote, TransactionDecision, TransactionId, TwoPhaseCommit,
};

fn main() {
    let mut coordinator = TwoPhaseCommit::from_participants([ParticipantId(1), ParticipantId(2)]);

    let decision = coordinator
        .decide(
            TransactionId(10),
            [
                (ParticipantId(1), ParticipantVote::Prepared),
                (ParticipantId(2), ParticipantVote::Prepared),
            ],
        )
        .expect("ambos participantes votaron");

    assert_eq!(decision, TransactionDecision::Committed);

    println!("La transacción {:?} terminó en commit.", TransactionId(10));
}
