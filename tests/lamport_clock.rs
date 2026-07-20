use rust_distributed_systems::lamport_clock::{EventId, LamportClock, LamportTimestamp, NodeId};

#[test]
fn local_events_are_monotonic() {
    let mut clock = LamportClock::new(NodeId(1));

    let first = clock.local_event();
    let second = clock.local_event();

    assert_eq!(first, EventId::new(LamportTimestamp(1), NodeId(1)));
    assert_eq!(second, EventId::new(LamportTimestamp(2), NodeId(1)));
    assert_eq!(clock.timestamp(), LamportTimestamp(2));
}

#[test]
fn sending_a_message_increments_before_attaching_timestamp() {
    let mut sender = LamportClock::new(NodeId(1));

    let message = sender.send();

    assert_eq!(message.sender, NodeId(1));
    assert_eq!(message.timestamp, LamportTimestamp(1));
    assert_eq!(sender.timestamp(), LamportTimestamp(1));
}

#[test]
fn receiving_a_message_uses_remote_max_plus_one() {
    let mut sender = LamportClock::new(NodeId(1));
    sender.local_event();
    let message = sender.send();

    let mut receiver = LamportClock::new(NodeId(2));
    let received = receiver.receive(message);

    assert_eq!(message.timestamp, LamportTimestamp(2));
    assert_eq!(received, EventId::new(LamportTimestamp(3), NodeId(2)));
    assert_eq!(receiver.timestamp(), LamportTimestamp(3));
}

#[test]
fn receiving_an_old_message_still_advances_local_clock() {
    let mut receiver = LamportClock::new(NodeId(2));
    receiver.local_event();
    receiver.local_event();
    receiver.local_event();

    let old_message = LamportClock::new(NodeId(1)).send();
    let received = receiver.receive(old_message);

    assert_eq!(received, EventId::new(LamportTimestamp(4), NodeId(2)));
    assert_eq!(receiver.timestamp(), LamportTimestamp(4));
}

#[test]
fn event_ids_sort_by_timestamp_then_node() {
    let same_time_on_node_two = EventId::new(LamportTimestamp(1), NodeId(2));
    let same_time_on_node_one = EventId::new(LamportTimestamp(1), NodeId(1));
    let later_on_node_one = EventId::new(LamportTimestamp(2), NodeId(1));

    let mut events = vec![
        same_time_on_node_two,
        later_on_node_one,
        same_time_on_node_one,
    ];
    events.sort();

    assert_eq!(
        events,
        [
            EventId::new(LamportTimestamp(1), NodeId(1)),
            EventId::new(LamportTimestamp(1), NodeId(2)),
            EventId::new(LamportTimestamp(2), NodeId(1)),
        ]
    );
}
