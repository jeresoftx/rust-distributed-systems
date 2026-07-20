use rust_distributed_systems::leader_election::{LeaderElection, LeaderElectionError, NodeId};

fn main() {
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

    election.finish_election(NodeId(1)).unwrap();
    assert_eq!(election.leader(), Some(NodeId(1)));
    println!(
        "Doble voto rechazado; líder vigente: {:?}",
        election.leader()
    );
}
