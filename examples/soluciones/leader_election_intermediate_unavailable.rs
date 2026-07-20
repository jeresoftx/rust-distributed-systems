use rust_distributed_systems::leader_election::{LeaderElection, LeaderElectionError, NodeId};

fn main() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = election.start_election(NodeId(1)).unwrap();

    election.fail_node(NodeId(2)).unwrap();
    assert_eq!(
        election.grant_vote(NodeId(2), NodeId(1), term),
        Err(LeaderElectionError::NodeUnavailable(NodeId(2)))
    );

    election.recover_node(NodeId(2)).unwrap();
    election.grant_vote(NodeId(2), NodeId(1), term).unwrap();
    election.finish_election(NodeId(1)).unwrap();

    assert_eq!(election.leader(), Some(NodeId(1)));
    println!("Nodo recuperado votó por: {:?}", election.leader());
}
