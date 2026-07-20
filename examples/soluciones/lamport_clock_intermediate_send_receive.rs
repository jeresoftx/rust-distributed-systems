use rust_distributed_systems::lamport_clock::{EventId, LamportClock, LamportTimestamp, NodeId};

fn main() {
    let mut sender = LamportClock::new(NodeId(1));
    sender.local_event();

    let message = sender.send();
    assert_eq!(message.timestamp, LamportTimestamp(2));

    let mut receiver = LamportClock::new(NodeId(2));
    let received = receiver.receive(message);

    assert_eq!(received, EventId::new(LamportTimestamp(3), NodeId(2)));
    assert!(message.timestamp < received.timestamp);

    println!("Mensaje {:?} recibido como {:?}", message, received);
}
