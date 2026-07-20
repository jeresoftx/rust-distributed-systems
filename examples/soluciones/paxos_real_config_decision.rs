use rust_distributed_systems::paxos::{NodeId, PaxosRound, ProposalNumber};

fn main() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4), NodeId(5)]);
    let proposal = ProposalNumber(42);
    let value = "configuracion:v3;nodos=1,2,3,4,5";

    round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
    round.prepare(NodeId(1), NodeId(2), proposal).unwrap();
    round.prepare(NodeId(1), NodeId(3), proposal).unwrap();

    round.accept(NodeId(1), proposal, value).unwrap();
    round.accept(NodeId(2), proposal, value).unwrap();
    assert_eq!(round.chosen_value(), None);

    round.accept(NodeId(3), proposal, value).unwrap();

    assert_eq!(round.chosen_value(), Some(value));
    println!("Configuración elegida: {:?}", round.chosen_value());
}
