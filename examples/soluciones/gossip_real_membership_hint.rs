use rust_distributed_systems::gossip::{Fanout, GossipCluster, GossipFact, GossipNodeId};

fn main() {
    let mut cluster = GossipCluster::from_nodes([
        GossipNodeId(10),
        GossipNodeId(20),
        GossipNodeId(30),
        GossipNodeId(40),
    ]);

    let membership_version_12 = GossipFact(12);
    cluster.insert_fact(GossipNodeId(10), membership_version_12);
    cluster.set_available(GossipNodeId(40), false);

    cluster.run_round(Fanout(2));
    assert_eq!(cluster.coverage(membership_version_12), 3);
    assert!(!cluster.knows(GossipNodeId(40), membership_version_12));

    cluster.set_available(GossipNodeId(40), true);
    cluster.run_round(Fanout(2));

    assert_eq!(cluster.coverage(membership_version_12), 4);
    assert!(cluster.available_nodes_converged());

    println!(
        "La versión de membresía {:?} llegó a todos después de recuperar el nodo 40.",
        membership_version_12
    );
}
