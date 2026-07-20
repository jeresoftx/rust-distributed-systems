use rust_distributed_systems::lamport_clock::{EventId, LamportClock, LamportTimestamp, NodeId};

fn main() {
    let mut api = LamportClock::new(NodeId(1));
    let mut worker = LamportClock::new(NodeId(2));

    let api_started = api.local_event();
    let worker_started = worker.local_event();
    let message = api.send();
    let worker_received = worker.receive(message);

    let mut trace = vec![worker_received, worker_started, api_started];
    trace.sort();

    assert_eq!(
        trace,
        [
            EventId::new(LamportTimestamp(1), NodeId(1)),
            EventId::new(LamportTimestamp(1), NodeId(2)),
            EventId::new(LamportTimestamp(3), NodeId(2)),
        ]
    );

    println!("Traza ordenada: {:?}", trace);
}
