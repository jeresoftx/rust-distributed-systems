use rust_distributed_systems::raft::{NodeId, RaftCluster, Role, Term};

fn main() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);

    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();

    assert_eq!(term, Term(1));
    assert_eq!(cluster.leader(), Some(NodeId(1)));
    assert_eq!(cluster.node_role(NodeId(1)).unwrap(), Role::Leader);

    println!(
        "Líder elegido: {:?} en término {:?}",
        cluster.leader(),
        term
    );
}
