use rust_distributed_systems::lamport_clock::{EventId, LamportClock, LamportTimestamp, NodeId};

fn main() {
    let mut clock = LamportClock::new(NodeId(1));

    let first = clock.local_event();
    let second = clock.local_event();

    assert_eq!(first, EventId::new(LamportTimestamp(1), NodeId(1)));
    assert_eq!(second, EventId::new(LamportTimestamp(2), NodeId(1)));
    assert_eq!(clock.timestamp(), LamportTimestamp(2));

    println!("Eventos locales: {:?}, {:?}", first, second);
}
