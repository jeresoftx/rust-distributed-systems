use rust_distributed_systems::paxos::{NodeId, PaxosRound, ProposalNumber};

fn main() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);
    let proposal = ProposalNumber(10);

    round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
    round.prepare(NodeId(1), NodeId(2), proposal).unwrap();
    round.accept(NodeId(1), proposal, "valor-a").unwrap();
    assert_eq!(round.chosen_value(), None);

    round.accept(NodeId(2), proposal, "valor-a").unwrap();

    assert_eq!(round.chosen_value(), Some("valor-a"));
    println!("Valor elegido: {:?}", round.chosen_value());
}
