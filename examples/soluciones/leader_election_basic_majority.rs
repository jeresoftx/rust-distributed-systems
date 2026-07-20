use rust_distributed_systems::leader_election::{LeaderElection, LeadershipRole, NodeId};

fn main() {
    let mut election = LeaderElection::new([NodeId(1), NodeId(2), NodeId(3)]);

    let term = election.start_election(NodeId(1)).unwrap();
    assert_eq!(election.leader(), None);
    assert_eq!(election.role(NodeId(1)).unwrap(), LeadershipRole::Candidate);

    election.grant_vote(NodeId(2), NodeId(1), term).unwrap();
    election.finish_election(NodeId(1)).unwrap();

    assert_eq!(election.leader(), Some(NodeId(1)));
    assert_eq!(election.role(NodeId(1)).unwrap(), LeadershipRole::Leader);
    println!("Líder elegido por mayoría: {:?}", election.leader());
}
