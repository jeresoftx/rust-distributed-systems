use rust_distributed_systems::gossip::{
    Fanout, GossipCluster, GossipContact, GossipFact, GossipNodeId,
};

#[test]
fn facts_are_monotonic_and_idempotent() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    let fact = GossipFact(99);

    assert!(cluster.insert_fact(GossipNodeId(1), fact));
    assert!(!cluster.insert_fact(GossipNodeId(1), fact));
    assert_eq!(cluster.coverage(fact), 1);

    let first_round = cluster.run_round(Fanout(2));
    assert_eq!(first_round.facts_delivered, 2);
    assert_eq!(cluster.coverage(fact), 3);

    let duplicate_round = cluster.run_round(Fanout(2));
    assert_eq!(duplicate_round.facts_delivered, 0);
    assert_eq!(cluster.coverage(fact), 3);
}

#[test]
fn unavailable_nodes_neither_send_nor_receive() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    let fact = GossipFact(7);

    cluster.insert_fact(GossipNodeId(1), fact);
    cluster.set_available(GossipNodeId(2), false);

    let report = cluster.run_round(Fanout(2));

    assert!(cluster.knows(GossipNodeId(1), fact));
    assert!(!cluster.knows(GossipNodeId(2), fact));
    assert!(cluster.knows(GossipNodeId(3), fact));
    assert!(report
        .contacts
        .iter()
        .all(|contact| contact.from != GossipNodeId(2) && contact.to != GossipNodeId(2)));
}

#[test]
fn fanout_limits_contacts_per_sender() {
    let mut cluster = GossipCluster::from_nodes([
        GossipNodeId(1),
        GossipNodeId(2),
        GossipNodeId(3),
        GossipNodeId(4),
    ]);

    cluster.insert_fact(GossipNodeId(1), GossipFact(1));
    cluster.insert_fact(GossipNodeId(2), GossipFact(2));

    let report = cluster.run_round(Fanout(1));

    for sender in [
        GossipNodeId(1),
        GossipNodeId(2),
        GossipNodeId(3),
        GossipNodeId(4),
    ] {
        let sent = report
            .contacts
            .iter()
            .filter(|contact| contact.from == sender)
            .count();
        assert!(sent <= 1, "{sender:?} sent {sent} contacts");
    }
}

#[test]
fn available_nodes_eventually_converge_with_repeated_rounds() {
    let mut cluster = GossipCluster::from_nodes([
        GossipNodeId(1),
        GossipNodeId(2),
        GossipNodeId(3),
        GossipNodeId(4),
    ]);
    let fact = GossipFact(42);

    cluster.insert_fact(GossipNodeId(1), fact);

    for _ in 0..6 {
        cluster.run_round(Fanout(1));
    }

    assert_eq!(cluster.coverage(fact), 4);
    assert!(cluster.available_nodes_converged());
}

#[test]
fn recovered_nodes_can_catch_up_in_later_rounds() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    let fact = GossipFact(5);

    cluster.insert_fact(GossipNodeId(1), fact);
    cluster.set_available(GossipNodeId(3), false);
    cluster.run_round(Fanout(2));

    assert_eq!(cluster.coverage(fact), 2);

    cluster.set_available(GossipNodeId(3), true);
    cluster.run_round(Fanout(2));

    assert_eq!(cluster.coverage(fact), 3);
    assert!(cluster.available_nodes_converged());
}

#[test]
fn round_report_records_contacts_and_message_count() {
    let mut cluster =
        GossipCluster::from_nodes([GossipNodeId(1), GossipNodeId(2), GossipNodeId(3)]);
    cluster.insert_fact(GossipNodeId(1), GossipFact(1));

    let report = cluster.run_round(Fanout(1));

    assert_eq!(report.messages_sent, report.contacts.len());
    assert_eq!(
        report.contacts.first(),
        Some(&GossipContact::new(GossipNodeId(1), GossipNodeId(2), 1))
    );
}
