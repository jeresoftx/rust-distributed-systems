use rust_distributed_systems::paxos::{NodeId, PaxosError, PaxosRound, ProposalNumber};

fn main() {
    let mut round = PaxosRound::new([NodeId(1), NodeId(2), NodeId(3)]);

    round
        .prepare(NodeId(1), NodeId(2), ProposalNumber(20))
        .unwrap();

    let result = round.prepare(NodeId(3), NodeId(2), ProposalNumber(10));

    assert_eq!(
        result,
        Err(PaxosError::StaleProposal {
            acceptor: NodeId(2),
            promised: ProposalNumber(20),
            attempted: ProposalNumber(10),
        })
    );
    println!("Propuesta vieja rechazada: {:?}", result);
}
