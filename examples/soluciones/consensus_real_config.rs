use rust_distributed_systems::consensus::{ConsensusRound, NodeId, ProposalId};

fn main() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4), NodeId(5)]);

    round.propose(
        NodeId(1),
        ProposalId(42),
        "configuracion:v2;nodos=1,2,3,4,5",
    );

    round.accept(NodeId(1), ProposalId(42)).unwrap();
    round.accept(NodeId(3), ProposalId(42)).unwrap();
    assert_eq!(round.decided_value(), None);

    round.accept(NodeId(5), ProposalId(42)).unwrap();
    assert_eq!(
        round.decided_value(),
        Some("configuracion:v2;nodos=1,2,3,4,5")
    );

    println!("Configuración activa: {:?}", round.decided_value());
}
