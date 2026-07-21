use rust_distributed_systems::gossip::{Fanout, GossipCluster, GossipFact, GossipNodeId};

fn main() {
    let mut cluster = GossipCluster::from_nodes([
        GossipNodeId(1),
        GossipNodeId(2),
        GossipNodeId(3),
        GossipNodeId(4),
    ]);
    let fact = GossipFact(99);

    cluster.insert_fact(GossipNodeId(1), fact);

    let mut rounds = 0;
    while !cluster.available_nodes_converged() {
        cluster.run_round(Fanout(1));
        rounds += 1;
        assert!(rounds <= 8, "el ejemplo debe converger rápido");
    }

    assert_eq!(cluster.coverage(fact), 4);

    println!(
        "Fanout(1) propagó {:?} a todo el cluster en {rounds} rondas.",
        fact
    );
}
