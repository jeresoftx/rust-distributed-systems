use rust_distributed_systems::raft::{NodeId, RaftCluster};

fn main() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3), NodeId(4), NodeId(5)]);

    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.request_vote(NodeId(3), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();

    let index = cluster
        .append_entry(NodeId(1), "configuracion:v2;nodos=1,2,3,4,5")
        .unwrap();

    cluster
        .replicate_entry(NodeId(1), NodeId(2), index)
        .unwrap();
    assert_eq!(cluster.committed_command(index), None);

    cluster
        .replicate_entry(NodeId(1), NodeId(4), index)
        .unwrap();
    cluster.commit_entry(NodeId(1), index).unwrap();

    assert_eq!(
        cluster.committed_command(index),
        Some("configuracion:v2;nodos=1,2,3,4,5")
    );
    println!(
        "Configuración replicada y confirmada: {:?}",
        cluster.committed_command(index)
    );
}
