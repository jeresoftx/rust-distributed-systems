use rust_distributed_systems::consensus::{
    ConsensusError, ConsensusEvent, ConsensusRound, NodeId, ProposalId,
};

#[test]
fn decides_value_when_majority_accepts_it() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "activar-configuracion-a");
    assert_eq!(round.decided_value(), None);

    round
        .accept(NodeId(1), ProposalId(10))
        .expect("el proponente es parte del grupo");
    assert_eq!(round.decided_value(), None);

    round
        .accept(NodeId(2), ProposalId(10))
        .expect("dos votos alcanzan mayoría en tres nodos");

    assert_eq!(round.decided_value(), Some("activar-configuracion-a"));
}

#[test]
fn rejects_unknown_nodes() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");

    assert_eq!(
        round.accept(NodeId(99), ProposalId(10)),
        Err(ConsensusError::UnknownNode(NodeId(99)))
    );
}

#[test]
fn a_node_cannot_accept_two_incompatible_values_in_the_same_round() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");
    round.propose(NodeId(2), ProposalId(20), "valor-b");

    round
        .accept(NodeId(1), ProposalId(10))
        .expect("primer voto válido");

    assert_eq!(
        round.accept(NodeId(1), ProposalId(20)),
        Err(ConsensusError::ConflictingAcceptance {
            node: NodeId(1),
            accepted: ProposalId(10),
            attempted: ProposalId(20),
        })
    );
}

#[test]
fn failed_nodes_cannot_accept_messages_until_they_recover() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");
    round.fail_node(NodeId(2)).expect("nodo conocido");

    assert_eq!(
        round.accept(NodeId(2), ProposalId(10)),
        Err(ConsensusError::NodeUnavailable(NodeId(2)))
    );

    round.recover_node(NodeId(2)).expect("nodo conocido");
    round
        .accept(NodeId(2), ProposalId(10))
        .expect("el nodo recuperado vuelve a votar");
}

#[test]
fn history_explains_the_decision() {
    let mut round = ConsensusRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round.propose(NodeId(1), ProposalId(10), "valor-a");
    round.accept(NodeId(1), ProposalId(10)).unwrap();
    round.accept(NodeId(2), ProposalId(10)).unwrap();

    assert_eq!(
        round.history(),
        [
            ConsensusEvent::Proposed {
                proposer: NodeId(1),
                proposal: ProposalId(10),
            },
            ConsensusEvent::Accepted {
                node: NodeId(1),
                proposal: ProposalId(10),
            },
            ConsensusEvent::Accepted {
                node: NodeId(2),
                proposal: ProposalId(10),
            },
            ConsensusEvent::Decided {
                proposal: ProposalId(10),
            },
        ]
    );
}
