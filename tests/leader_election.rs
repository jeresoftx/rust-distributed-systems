use rust_distributed_systems::leader_election::{
    ElectionEvent, ElectionTerm, LeaderElection, LeaderElectionError, LeadershipRole, NodeId,
};

#[test]
fn candidate_becomes_leader_after_majority_vote() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);

    let term = election
        .start_election(NodeId(1))
        .expect("el candidato pertenece al grupo");
    election
        .grant_vote(NodeId(2), NodeId(1), term)
        .expect("el votante concede voto en el término vigente");
    election
        .finish_election(NodeId(1))
        .expect("dos votos alcanzan mayoría");

    assert_eq!(term, ElectionTerm(1));
    assert_eq!(election.leader(), Some(NodeId(1)));
    assert_eq!(
        election.role(NodeId(1)).expect("nodo conocido"),
        LeadershipRole::Leader
    );
}

#[test]
fn node_grants_only_one_vote_per_term() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = election.start_election(NodeId(1)).unwrap();

    election.grant_vote(NodeId(2), NodeId(1), term).unwrap();

    assert_eq!(
        election.grant_vote(NodeId(2), NodeId(3), term),
        Err(LeaderElectionError::AlreadyVoted {
            voter: NodeId(2),
            term,
            voted_for: NodeId(1),
            attempted: NodeId(3),
        })
    );
}

#[test]
fn unavailable_nodes_cannot_vote_until_they_recover() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = election.start_election(NodeId(1)).unwrap();

    election.fail_node(NodeId(2)).expect("nodo conocido");

    assert_eq!(
        election.grant_vote(NodeId(2), NodeId(1), term),
        Err(LeaderElectionError::NodeUnavailable(NodeId(2)))
    );

    election.recover_node(NodeId(2)).expect("nodo conocido");
    election
        .grant_vote(NodeId(2), NodeId(1), term)
        .expect("un nodo recuperado puede votar");
}

#[test]
fn stale_terms_do_not_change_current_election() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = election.start_election(NodeId(1)).unwrap();

    assert_eq!(
        election.grant_vote(NodeId(2), NodeId(1), ElectionTerm(0)),
        Err(LeaderElectionError::StaleTerm {
            node: NodeId(2),
            current: term,
            attempted: ElectionTerm(0),
        })
    );
}

#[test]
fn history_explains_the_election() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = election.start_election(NodeId(1)).unwrap();
    election.grant_vote(NodeId(2), NodeId(1), term).unwrap();
    election.finish_election(NodeId(1)).unwrap();

    assert_eq!(
        election.history(),
        [
            ElectionEvent::ElectionStarted {
                candidate: NodeId(1),
                term,
            },
            ElectionEvent::VoteGranted {
                voter: NodeId(2),
                candidate: NodeId(1),
                term,
            },
            ElectionEvent::LeaderElected {
                leader: NodeId(1),
                term,
            },
        ]
    );
}
