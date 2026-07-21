use rust_distributed_systems::gossip::{Fanout, GossipCluster, GossipFact, GossipNodeId};

fn main() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    let fact = GossipFact(7);

    cluster.insert_fact(GossipNodeId(1), fact);
    assert!(cluster.set_available(GossipNodeId(3), false));

    let report = cluster.run_round(Fanout(2));

    assert!(cluster.knows(GossipNodeId(1), fact));
    assert!(cluster.knows(GossipNodeId(2), fact));
    assert!(!cluster.knows(GossipNodeId(3), fact));
    assert_eq!(report.facts_delivered, 1);

    println!(
        "Nodo 3 no recibió {:?}; los contactos fueron {:?}.",
        fact, report.contacts
    );
}
