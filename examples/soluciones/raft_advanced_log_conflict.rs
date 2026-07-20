use rust_distributed_systems::raft::{NodeId, RaftCluster, RaftError, Term};

fn main() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();

    cluster
        .install_log_for_scenario(NodeId(2), [(Term(9), "valor-anterior")])
        .unwrap();

    let index = cluster.append_entry(NodeId(1), "valor-nuevo").unwrap();
    let result = cluster.replicate_entry(NodeId(1), NodeId(2), index);

    assert_eq!(
        result,
        Err(RaftError::LogConflict {
            node: NodeId(2),
            index,
        })
    );
    println!("Conflicto detectado en {:?}", index);
}
