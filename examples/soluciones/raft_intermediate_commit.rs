use rust_distributed_systems::raft::{LogIndex, NodeId, RaftCluster};

fn main() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();

    let index = cluster.append_entry(NodeId(1), "set limite=100").unwrap();
    assert_eq!(index, LogIndex(1));
    assert_eq!(cluster.committed_command(index), None);

    cluster
        .replicate_entry(NodeId(1), NodeId(2), index)
        .unwrap();
    cluster.commit_entry(NodeId(1), index).unwrap();

    assert_eq!(cluster.committed_command(index), Some("set limite=100"));
    println!("Entrada confirmada: {:?}", cluster.committed_command(index));
}
