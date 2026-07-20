use rust_distributed_systems::lamport_clock::{EventId, LamportClock, NodeId};

#[derive(Clone, Debug, Eq, PartialEq)]
struct AuditEvent {
    id: EventId,
    description: &'static str,
}

fn main() {
    let mut checkout = LamportClock::new(NodeId(1));
    let mut payment = LamportClock::new(NodeId(2));
    let mut reservations = LamportClock::new(NodeId(3));

    let checkout_started = AuditEvent {
        id: checkout.local_event(),
        description: "checkout iniciado",
    };

    let payment_request = checkout.send();
    let payment_received = AuditEvent {
        id: payment.receive(payment_request),
        description: "pago recibido",
    };

    let reservation_request = payment.send();
    let reservation_confirmed = AuditEvent {
        id: reservations.receive(reservation_request),
        description: "reserva confirmada",
    };

    let mut audit = vec![reservation_confirmed, checkout_started, payment_received];
    audit.sort_by_key(|event| event.id);

    assert_eq!(audit[0].description, "checkout iniciado");
    assert_eq!(audit[1].description, "pago recibido");
    assert_eq!(audit[2].description, "reserva confirmada");

    println!("Auditoría lógica: {:?}", audit);
}
