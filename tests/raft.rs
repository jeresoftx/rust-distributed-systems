use rust_distributed_systems::raft::{
    LogIndex, NodeId, RaftCluster, RaftError, RaftEvent, Role, Term,
};

#[test]
fn candidate_becomes_leader_only_after_majority() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);

    let term = cluster
        .start_election(NodeId(1))
        .expect("el candidato pertenece al clúster");

    assert_eq!(term, Term(1));
    assert_eq!(cluster.leader(), None);

    cluster
        .request_vote(NodeId(2), NodeId(1), term)
        .expect("el votante concede voto en el término vigente");
    cluster
        .finish_election(NodeId(1))
        .expect("dos votos alcanzan mayoría");

    assert_eq!(cluster.leader(), Some(NodeId(1)));
    assert_eq!(
        cluster.node_role(NodeId(1)).expect("nodo conocido"),
        Role::Leader
    );
}

#[test]
fn node_grants_one_vote_per_term_and_rejects_stale_terms() {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = cluster.start_election(NodeId(1)).unwrap();

    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();

    assert_eq!(
        cluster.request_vote(NodeId(2), NodeId(3), term),
        Err(RaftError::AlreadyVoted {
            voter: NodeId(2),
            term,
            voted_for: NodeId(1),
            attempted: NodeId(3),
        })
    );
    assert_eq!(
        cluster.request_vote(NodeId(2), NodeId(3), Term(0)),
        Err(RaftError::StaleTerm {
            node: NodeId(2),
            current: term,
            attempted: Term(0),
        })
    );
}

#[test]
fn leader_commits_entry_only_after_majority_replication() {
    let mut cluster = elected_cluster();

    let index = cluster
        .append_entry(NodeId(1), "set x=1")
        .expect("el líder puede agregar entradas");

    assert_eq!(index, LogIndex(1));
    assert_eq!(cluster.committed_command(index), None);

    cluster
        .replicate_entry(NodeId(1), NodeId(2), index)
        .expect("el seguidor acepta la entrada del líder");
    cluster
        .commit_entry(NodeId(1), index)
        .expect("líder y un seguidor forman mayoría");

    assert_eq!(cluster.committed_command(index), Some("set x=1"));
}

#[test]
fn follower_rejects_replication_when_log_prefix_conflicts() {
    let mut cluster = elected_cluster();

    cluster
        .install_log_for_scenario(NodeId(2), [(Term(9), "valor-anterior")])
        .expect("el escenario puede preparar un log divergente");

    let index = cluster.append_entry(NodeId(1), "valor-nuevo").unwrap();

    assert_eq!(
        cluster.replicate_entry(NodeId(1), NodeId(2), index),
        Err(RaftError::LogConflict {
            node: NodeId(2),
            index,
        })
    );
}

#[test]
fn history_explains_election_replication_and_commit() {
    let mut cluster = elected_cluster();
    let index = cluster.append_entry(NodeId(1), "activar").unwrap();
    cluster
        .replicate_entry(NodeId(1), NodeId(2), index)
        .unwrap();
    cluster.commit_entry(NodeId(1), index).unwrap();

    assert_eq!(
        cluster.history(),
        [
            RaftEvent::ElectionStarted {
                candidate: NodeId(1),
                term: Term(1),
            },
            RaftEvent::VoteGranted {
                voter: NodeId(2),
                candidate: NodeId(1),
                term: Term(1),
            },
            RaftEvent::LeaderElected {
                leader: NodeId(1),
                term: Term(1),
            },
            RaftEvent::EntryAppended {
                leader: NodeId(1),
                index,
                term: Term(1),
            },
            RaftEvent::EntryReplicated {
                leader: NodeId(1),
                follower: NodeId(2),
                index,
            },
            RaftEvent::EntryCommitted { index },
        ]
    );
}

fn elected_cluster() -> RaftCluster {
    let mut cluster = RaftCluster::new([NodeId(1), NodeId(2), NodeId(3)]);
    let term = cluster.start_election(NodeId(1)).unwrap();
    cluster.request_vote(NodeId(2), NodeId(1), term).unwrap();
    cluster.finish_election(NodeId(1)).unwrap();
    cluster
}
