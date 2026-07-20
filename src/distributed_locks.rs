//! Modelo educativo mínimo de locks distribuidos.
//!
//! El modelo representa locks por lease con tiempo lógico y fencing tokens.
//! No implementa red real, relojes físicos, persistencia, consenso ni un
//! servicio de coordinación de producción.

use std::collections::BTreeMap;

/// Identificador estable de cliente.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClientId(pub u64);

/// Identificador estable de recurso protegido.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(pub &'static str);

/// Tiempo lógico controlado por el escenario.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LogicalTime(pub u64);

/// Duración lógica de un lease.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LeaseDuration(pub u64);

/// Token monótono usado para rechazar operaciones obsoletas.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FencingToken(pub u64);

/// Concesión observable de un lock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LockGrant {
    /// Cliente propietario.
    pub owner: ClientId,
    /// Recurso protegido.
    pub resource: ResourceId,
    /// Token monótono de la concesión.
    pub token: FencingToken,
    /// Tiempo lógico en el que termina el lease.
    pub expires_at: LogicalTime,
}

/// Evento observable del modelo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DistributedLockEvent {
    /// Un cliente adquirió un lock.
    LockAcquired {
        /// Cliente propietario.
        owner: ClientId,
        /// Recurso protegido.
        resource: ResourceId,
        /// Token de fencing concedido.
        token: FencingToken,
        /// Tiempo lógico de expiración.
        expires_at: LogicalTime,
    },
    /// El propietario renovó un lock vigente.
    LockRenewed {
        /// Cliente propietario.
        owner: ClientId,
        /// Recurso protegido.
        resource: ResourceId,
        /// Token de fencing vigente.
        token: FencingToken,
        /// Nueva expiración lógica.
        expires_at: LogicalTime,
    },
    /// El propietario liberó un lock vigente.
    LockReleased {
        /// Cliente propietario.
        owner: ClientId,
        /// Recurso protegido.
        resource: ResourceId,
        /// Token liberado.
        token: FencingToken,
    },
    /// Un lock expiró al avanzar el tiempo lógico.
    LockExpired {
        /// Cliente que era propietario.
        owner: ClientId,
        /// Recurso protegido.
        resource: ResourceId,
        /// Token que dejó de estar activo.
        token: FencingToken,
        /// Tiempo lógico de expiración.
        expired_at: LogicalTime,
    },
}

/// Error explícito del modelo educativo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DistributedLockError {
    /// La duración del lease debe ser mayor que cero.
    InvalidLeaseDuration(LeaseDuration),
    /// El recurso ya tiene propietario activo.
    ResourceBusy {
        /// Recurso solicitado.
        resource: ResourceId,
        /// Propietario vigente.
        owner: ClientId,
        /// Expiración vigente.
        expires_at: LogicalTime,
    },
    /// No existe lock activo para el recurso.
    LockNotHeld(ResourceId),
    /// El cliente que intenta operar no es el propietario vigente.
    NotLockOwner {
        /// Recurso protegido.
        resource: ResourceId,
        /// Propietario vigente.
        expected: ClientId,
        /// Cliente que intentó operar.
        attempted: ClientId,
    },
    /// El token intentado no coincide con el token activo.
    StaleFencingToken {
        /// Recurso protegido.
        resource: ResourceId,
        /// Token vigente.
        current: FencingToken,
        /// Token intentado.
        attempted: FencingToken,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ActiveLock {
    grant: LockGrant,
}

impl ActiveLock {
    fn is_active_at(&self, now: LogicalTime) -> bool {
        self.grant.expires_at > now
    }
}

/// Coordinador determinista de locks por lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedLockManager {
    now: LogicalTime,
    locks: BTreeMap<ResourceId, ActiveLock>,
    last_tokens: BTreeMap<ResourceId, FencingToken>,
    history: Vec<DistributedLockEvent>,
}

impl DistributedLockManager {
    /// Crea un administrador con tiempo lógico inicial.
    #[must_use]
    pub fn new(now: LogicalTime) -> Self {
        Self {
            now,
            locks: BTreeMap::new(),
            last_tokens: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    /// Tiempo lógico actual del escenario.
    #[must_use]
    pub fn now(&self) -> LogicalTime {
        self.now
    }

    /// Propietario activo de un recurso, si su lease no ha expirado.
    #[must_use]
    pub fn owner(&self, resource: ResourceId) -> Option<(ClientId, FencingToken)> {
        self.locks
            .get(&resource)
            .filter(|active_lock| active_lock.is_active_at(self.now))
            .map(|active_lock| (active_lock.grant.owner, active_lock.grant.token))
    }

    /// Historial observable de decisiones.
    #[must_use]
    pub fn history(&self) -> &[DistributedLockEvent] {
        &self.history
    }

    /// Avanza el tiempo lógico y expira locks vencidos.
    pub fn advance_to(&mut self, now: LogicalTime) {
        if now < self.now {
            return;
        }

        self.now = now;
        self.expire_due_locks();
    }

    /// Intenta adquirir un lock para un recurso.
    pub fn acquire(
        &mut self,
        owner: ClientId,
        resource: ResourceId,
        duration: LeaseDuration,
    ) -> Result<LockGrant, DistributedLockError> {
        self.ensure_valid_duration(duration)?;
        self.expire_resource_if_due(resource);

        if let Some(active_lock) = self.locks.get(&resource) {
            return Err(DistributedLockError::ResourceBusy {
                resource,
                owner: active_lock.grant.owner,
                expires_at: active_lock.grant.expires_at,
            });
        }

        let token = self.next_token(resource);
        let grant = LockGrant {
            owner,
            resource,
            token,
            expires_at: LogicalTime(self.now.0 + duration.0),
        };

        self.locks.insert(resource, ActiveLock { grant });
        self.history.push(DistributedLockEvent::LockAcquired {
            owner,
            resource,
            token,
            expires_at: grant.expires_at,
        });
        Ok(grant)
    }

    /// Renueva un lock vigente si el cliente y el token coinciden.
    pub fn renew(
        &mut self,
        owner: ClientId,
        resource: ResourceId,
        token: FencingToken,
        duration: LeaseDuration,
    ) -> Result<LockGrant, DistributedLockError> {
        self.ensure_valid_duration(duration)?;
        self.expire_resource_if_due(resource);
        self.ensure_current_owner(resource, owner)?;
        self.ensure_current_token(resource, token)?;

        let expires_at = LogicalTime(self.now.0 + duration.0);
        let grant = {
            let active_lock = self
                .locks
                .get_mut(&resource)
                .ok_or(DistributedLockError::LockNotHeld(resource))?;
            active_lock.grant.expires_at = expires_at;
            active_lock.grant
        };

        self.history.push(DistributedLockEvent::LockRenewed {
            owner,
            resource,
            token,
            expires_at,
        });
        Ok(grant)
    }

    /// Libera un lock vigente si el cliente y el token coinciden.
    pub fn release(
        &mut self,
        owner: ClientId,
        resource: ResourceId,
        token: FencingToken,
    ) -> Result<(), DistributedLockError> {
        self.expire_resource_if_due(resource);
        self.ensure_current_owner(resource, owner)?;
        self.ensure_current_token(resource, token)?;

        self.locks.remove(&resource);
        self.history.push(DistributedLockEvent::LockReleased {
            owner,
            resource,
            token,
        });
        Ok(())
    }

    /// Valida que una operación use el fencing token vigente del recurso.
    pub fn validate_operation(
        &mut self,
        resource: ResourceId,
        token: FencingToken,
    ) -> Result<(), DistributedLockError> {
        self.expire_resource_if_due(resource);
        self.ensure_current_token(resource, token)
    }

    fn ensure_valid_duration(&self, duration: LeaseDuration) -> Result<(), DistributedLockError> {
        if duration.0 == 0 {
            Err(DistributedLockError::InvalidLeaseDuration(duration))
        } else {
            Ok(())
        }
    }

    fn ensure_current_owner(
        &self,
        resource: ResourceId,
        owner: ClientId,
    ) -> Result<(), DistributedLockError> {
        let active_lock = self
            .locks
            .get(&resource)
            .ok_or(DistributedLockError::LockNotHeld(resource))?;

        if active_lock.grant.owner == owner {
            Ok(())
        } else {
            Err(DistributedLockError::NotLockOwner {
                resource,
                expected: active_lock.grant.owner,
                attempted: owner,
            })
        }
    }

    fn ensure_current_token(
        &self,
        resource: ResourceId,
        token: FencingToken,
    ) -> Result<(), DistributedLockError> {
        let active_lock = self
            .locks
            .get(&resource)
            .ok_or(DistributedLockError::LockNotHeld(resource))?;

        if active_lock.grant.token == token {
            Ok(())
        } else {
            Err(DistributedLockError::StaleFencingToken {
                resource,
                current: active_lock.grant.token,
                attempted: token,
            })
        }
    }

    fn expire_due_locks(&mut self) {
        let expired_resources: Vec<ResourceId> = self
            .locks
            .iter()
            .filter_map(|(resource, active_lock)| {
                (!active_lock.is_active_at(self.now)).then_some(*resource)
            })
            .collect();

        for resource in expired_resources {
            self.expire_resource(resource);
        }
    }

    fn expire_resource_if_due(&mut self, resource: ResourceId) {
        let should_expire = self
            .locks
            .get(&resource)
            .is_some_and(|active_lock| !active_lock.is_active_at(self.now));

        if should_expire {
            self.expire_resource(resource);
        }
    }

    fn expire_resource(&mut self, resource: ResourceId) {
        if let Some(active_lock) = self.locks.remove(&resource) {
            self.history.push(DistributedLockEvent::LockExpired {
                owner: active_lock.grant.owner,
                resource,
                token: active_lock.grant.token,
                expired_at: active_lock.grant.expires_at,
            });
        }
    }

    fn next_token(&mut self, resource: ResourceId) -> FencingToken {
        let next = self
            .last_tokens
            .get(&resource)
            .map_or(FencingToken(1), |token| FencingToken(token.0 + 1));
        self.last_tokens.insert(resource, next);
        next
    }
}

#[cfg(test)]
mod tests {
    use super::{DistributedLockManager, LeaseDuration, LogicalTime};

    #[test]
    fn manager_starts_at_given_logical_time() {
        let locks = DistributedLockManager::new(LogicalTime(42));

        assert_eq!(locks.now(), LogicalTime(42));
    }

    #[test]
    fn zero_duration_is_invalid() {
        let mut locks = DistributedLockManager::new(LogicalTime(0));

        assert!(locks
            .acquire(
                super::ClientId(1),
                super::ResourceId("resource"),
                LeaseDuration(0)
            )
            .is_err());
    }
}
