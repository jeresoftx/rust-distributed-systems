//! Modelos educativos de sistemas distribuidos para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-distributed-systems`. Su propósito es
//! representar con código pequeño y verificable los mecanismos centrales de un
//! sistema distribuido: nodos, mensajes, tiempo, fallas, replicación, consenso
//! y consistencia observable.

pub mod cap;
pub mod consensus;
pub mod consistent_hashing;
pub mod crdt;
pub mod distributed_locks;
pub mod lamport_clock;
pub mod leader_election;
pub mod paxos;
pub mod raft;
pub mod vector_clock;

/// Nombre canónico del curso dentro de Jeresoft Academy.
#[must_use]
pub fn course_name() -> &'static str {
    "rust-distributed-systems"
}

#[cfg(test)]
mod tests {
    use super::course_name;

    #[test]
    fn exposes_course_name() {
        assert_eq!(course_name(), "rust-distributed-systems");
    }
}
