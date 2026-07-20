use rust_distributed_systems::consensus::{ConsensusRound, NodeId, ProposalId};

fn main() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "activar-configuracion-a");
    round.accept(NodeId(1), ProposalId(10)).unwrap();
    round.accept(NodeId(2), ProposalId(10)).unwrap();

    assert_eq!(round.decided_value(), Some("activar-configuracion-a"));
    println!("Decisión alcanzada: {:?}", round.decided_value());
}
