//! Modelo educativo mínimo de CRDTs.
//!
//! El modelo implementa un G-Counter state-based: cada réplica incrementa su
//! propio componente y la fusión conserva el máximo observado por réplica. No
//! implementa decrementos, borrados, compactación de metadatos, red real ni
//! persistencia.

use std::collections::{BTreeMap, BTreeSet};

/// Identificador estable de réplica.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReplicaId(pub u64);

/// Conteo lógico no negativo.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Count(pub u64);

/// Relación parcial observable entre dos estados de un CRDT.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateRelation {
    /// Ambos estados contienen el mismo conocimiento.
    Equal,
    /// El estado izquierdo está contenido estrictamente en el derecho.
    Before,
    /// El estado izquierdo contiene estrictamente al derecho.
    After,
    /// Ningún estado contiene al otro.
    Concurrent,
}

/// G-Counter determinista para escenarios educativos.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GCounter {
    counts: BTreeMap<ReplicaId, Count>,
}

impl GCounter {
    /// Crea un contador vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Devuelve el conteo observado para una réplica.
    ///
    /// Una réplica ausente equivale a `Count(0)`.
    #[must_use]
    pub fn count(&self, replica: ReplicaId) -> Count {
        self.counts.get(&replica).copied().unwrap_or_default()
    }

    /// Incrementa en uno el componente local de una réplica.
    pub fn increment(&mut self, replica: ReplicaId) -> Count {
        self.increment_by(replica, Count(1))
    }

    /// Incrementa el componente local de una réplica por una cantidad dada.
    ///
    /// `Count(0)` es una operación vacía: no crea componentes artificiales.
    /// El modelo no expone decrementos.
    pub fn increment_by(&mut self, replica: ReplicaId, amount: Count) -> Count {
        if amount == Count(0) {
            return self.count(replica);
        }

        let count = self.counts.entry(replica).or_default();
        count.0 = count.0.saturating_add(amount.0);
        *count
    }

    /// Devuelve el valor total como suma de todos los componentes.
    #[must_use]
    pub fn value(&self) -> Count {
        Count(
            self.counts
                .values()
                .fold(0_u64, |total, count| total.saturating_add(count.0)),
        )
    }

    /// Fusiona otro estado conservando el máximo por componente.
    pub fn merge(&mut self, other: &Self) {
        for (&replica, &other_count) in &other.counts {
            let count = self.counts.entry(replica).or_default();
            *count = (*count).max(other_count);
        }
    }

    /// Devuelve un nuevo contador con el resultado de fusionar ambos estados.
    #[must_use]
    pub fn merged(&self, other: &Self) -> Self {
        let mut merged = self.clone();
        merged.merge(other);
        merged
    }

    /// Compara este estado contra otro y devuelve su relación parcial.
    #[must_use]
    pub fn compare(&self, other: &Self) -> StateRelation {
        let mut has_lower_component = false;
        let mut has_higher_component = false;

        for replica in self.replicas_seen_with(other) {
            match self.count(replica).cmp(&other.count(replica)) {
                std::cmp::Ordering::Less => has_lower_component = true,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => has_higher_component = true,
            }
        }

        match (has_lower_component, has_higher_component) {
            (false, false) => StateRelation::Equal,
            (true, false) => StateRelation::Before,
            (false, true) => StateRelation::After,
            (true, true) => StateRelation::Concurrent,
        }
    }

    fn replicas_seen_with(&self, other: &Self) -> BTreeSet<ReplicaId> {
        self.counts
            .keys()
            .chain(other.counts.keys())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Count, GCounter, ReplicaId, StateRelation};

    #[test]
    fn empty_counters_are_equal() {
        assert_eq!(
            GCounter::new().compare(&GCounter::new()),
            StateRelation::Equal
        );
    }

    #[test]
    fn increment_returns_next_count() {
        let mut counter = GCounter::new();

        assert_eq!(counter.increment(ReplicaId(9)), Count(1));
    }
}
