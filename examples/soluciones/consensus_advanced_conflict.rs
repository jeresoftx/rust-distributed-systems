use rust_distributed_systems::consensus::{ConsensusError, ConsensusRound, NodeId, ProposalId};

fn main() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");
    round.propose(NodeId(2), ProposalId(20), "valor-b");
    round.accept(NodeId(1), ProposalId(10)).unwrap();

    assert_eq!(
        round.accept(NodeId(1), ProposalId(20)),
        Err(ConsensusError::ConflictingAcceptance {
            node: NodeId(1),
            accepted: ProposalId(10),
            attempted: ProposalId(20),
        })
    );

    println!("El conflicto fue rechazado de forma explícita.");
}
