use rust_distributed_systems::consensus::{ConsensusError, ConsensusRound, NodeId, ProposalId};

fn main() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");
    round.fail_node(NodeId(2)).unwrap();

    assert_eq!(
        round.accept(NodeId(2), ProposalId(10)),
        Err(ConsensusError::NodeUnavailable(NodeId(2)))
    );

    round.recover_node(NodeId(2)).unwrap();
    round.accept(NodeId(2), ProposalId(10)).unwrap();

    println!("Historial después de recuperación: {:?}", round.history());
}
