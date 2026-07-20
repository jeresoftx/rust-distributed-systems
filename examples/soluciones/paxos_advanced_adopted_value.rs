use rust_distributed_systems::paxos::{NodeId, PaxosRound, ProposalNumber};

fn main() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);

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

    assert_eq!(safe_value, "valor-previo");

    round
        .accept(NodeId(2), ProposalNumber(2), safe_value.clone())
        .unwrap();
    round
        .accept(NodeId(3), ProposalNumber(2), safe_value)
        .unwrap();

    assert_eq!(round.chosen_value(), Some("valor-previo"));
    println!("Valor seguro adoptado: {:?}", round.chosen_value());
}
