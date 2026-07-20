use rust_distributed_systems::leader_election::{ElectionTerm, LeaderElection, NodeId};

fn main() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4), NodeId(5)]);

    let first_term = election.start_election(NodeId(1)).unwrap();
    election
        .grant_vote(NodeId(2), NodeId(1), first_term)
        .unwrap();
    election
        .grant_vote(NodeId(3), NodeId(1), first_term)
        .unwrap();
    election.finish_election(NodeId(1)).unwrap();
    assert_eq!(election.leader(), Some(NodeId(1)));

    election.fail_node(NodeId(1)).unwrap();

    let second_term = election.start_election(NodeId(2)).unwrap();
    election
        .grant_vote(NodeId(3), NodeId(2), second_term)
        .unwrap();
    election
        .grant_vote(NodeId(4), NodeId(2), second_term)
        .unwrap();
    election.finish_election(NodeId(2)).unwrap();

    assert_eq!(first_term, ElectionTerm(1));
    assert_eq!(second_term, ElectionTerm(2));
    assert_eq!(election.leader(), Some(NodeId(2)));
    println!(
        "Nuevo líder tras falla: {:?} en término {:?}",
        election.leader(),
        second_term
    );
}
