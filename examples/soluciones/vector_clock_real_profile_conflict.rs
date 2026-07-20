use rust_distributed_systems::vector_clock::{CausalRelation, NodeId, VectorClock};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProfileVersion {
    display_name: &'static str,
    city: &'static str,
    clock: VectorClock,
}

fn main() {
    let mut base_clock = VectorClock::new();
    base_clock.increment(NodeId(1));

    let mut mobile = ProfileVersion {
        display_name: "Joel",
        city: "Mazatlán",
        clock: base_clock.clone(),
    };
    mobile.clock.increment(NodeId(2));
    mobile.city = "Ciudad de México";

    let mut web = ProfileVersion {
        display_name: "Joel Álvarez",
        city: "Mazatlán",
        clock: base_clock,
    };
    web.clock.increment(NodeId(3));

    assert_eq!(mobile.clock.compare(&web.clock), CausalRelation::Concurrent);

    let merged_clock = mobile.clock.merged(&web.clock);
    let resolved = ProfileVersion {
        display_name: web.display_name,
        city: mobile.city,
        clock: merged_clock,
    };

    assert_eq!(
        mobile.clock.compare(&resolved.clock),
        CausalRelation::Before
    );
    assert_eq!(web.clock.compare(&resolved.clock), CausalRelation::Before);

    println!("Perfil resuelto con reloj {:?}", resolved.clock);
}
