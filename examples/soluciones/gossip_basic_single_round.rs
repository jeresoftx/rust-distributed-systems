use rust_distributed_systems::gossip::{Fanout, GossipCluster, GossipFact, GossipNodeId};

fn main() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    let fact = GossipFact(42);

    assert!(cluster.insert_fact(GossipNodeId(1), fact));

    let report = cluster.run_round(Fanout(2));

    assert_eq!(report.messages_sent, 2);
    assert_eq!(report.facts_delivered, 2);
    assert_eq!(cluster.coverage(fact), 3);

    println!(
        "Gossip propagó {:?} a {} nodos en una ronda.",
        fact,
        cluster.coverage(fact)
    );
}
