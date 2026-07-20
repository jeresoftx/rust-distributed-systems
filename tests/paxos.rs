use rust_distributed_systems::paxos::{
    AcceptedProposal, NodeId, PaxosError, PaxosEvent, PaxosRound, ProposalNumber,
};

#[test]
fn prepare_promises_are_monotonic_and_reject_old_proposals() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    let promise = round
        .prepare(NodeId(1), NodeId(2), ProposalNumber(10))
        .expect("el aceptor promete la propuesta más alta observada");

    assert_eq!(promise.acceptor, NodeId(2));
    assert_eq!(promise.proposal, ProposalNumber(10));
    assert_eq!(promise.accepted, None);
    assert_eq!(
        round.prepare(NodeId(3), NodeId(2), ProposalNumber(5)),
        Err(PaxosError::StaleProposal {
            acceptor: NodeId(2),
            promised: ProposalNumber(10),
            attempted: ProposalNumber(5),
        })
    );
}

#[test]
fn acceptors_reject_accept_requests_below_their_promise() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round
        .prepare(NodeId(1), NodeId(2), ProposalNumber(10))
        .unwrap();

    assert_eq!(
        round.accept(NodeId(2), ProposalNumber(5), "valor-viejo"),
        Err(PaxosError::StaleProposal {
            acceptor: NodeId(2),
            promised: ProposalNumber(10),
            attempted: ProposalNumber(5),
        })
    );
}

#[test]
fn chosen_value_requires_majority_acceptance() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);
    let proposal = ProposalNumber(10);

    round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
    round.prepare(NodeId(1), NodeId(2), proposal).unwrap();

    round.accept(NodeId(1), proposal, "valor-a").unwrap();
    assert_eq!(round.chosen_value(), None);

    round.accept(NodeId(2), proposal, "valor-a").unwrap();
    assert_eq!(round.chosen_value(), Some("valor-a"));
}

#[test]
fn proposer_adopts_highest_previously_accepted_value() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round
        .prepare(NodeId(1), NodeId(1), ProposalNumber(1))
        .unwrap();
    round
        .accept(NodeId(1), ProposalNumber(1), "valor-a")
        .unwrap();

    let promises = [
        round
            .prepare(NodeId(2), NodeId(1), ProposalNumber(2))
            .unwrap(),
        round
            .prepare(NodeId(2), NodeId(2), ProposalNumber(2))
            .unwrap(),
        round
            .prepare(NodeId(2), NodeId(3), ProposalNumber(2))
            .unwrap(),
    ];

    assert_eq!(
        promises[0].accepted,
        Some(AcceptedProposal {
            proposal: ProposalNumber(1),
            value: "valor-a".to_string(),
        })
    );
    assert_eq!(PaxosRound::safe_value(&promises, "valor-b"), "valor-a");

    round
        .accept(NodeId(2), ProposalNumber(2), "valor-a")
        .unwrap();
    round
        .accept(NodeId(3), ProposalNumber(2), "valor-a")
        .unwrap();

    assert_eq!(round.chosen_value(), Some("valor-a"));
}

#[test]
fn history_explains_promises_acceptances_and_decision() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);
    let proposal = ProposalNumber(7);

    round.prepare(NodeId(1), NodeId(1), proposal).unwrap();
    round.prepare(NodeId(1), NodeId(2), proposal).unwrap();
    round.accept(NodeId(1), proposal, "valor-a").unwrap();
    round.accept(NodeId(2), proposal, "valor-a").unwrap();

    assert_eq!(
        round.history(),
        [
            PaxosEvent::PromiseGranted {
                proposer: NodeId(1),
                acceptor: NodeId(1),
                proposal,
            },
            PaxosEvent::PromiseGranted {
                proposer: NodeId(1),
                acceptor: NodeId(2),
                proposal,
            },
            PaxosEvent::Accepted {
                acceptor: NodeId(1),
                proposal,
            },
            PaxosEvent::Accepted {
                acceptor: NodeId(2),
                proposal,
            },
            PaxosEvent::Chosen { proposal },
        ]
    );
}
